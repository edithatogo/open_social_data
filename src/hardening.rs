//! Connection hardening, retry logic, and panic safety.
//!
//! Provides [`RetryPolicy`] for exponential backoff, [`CircuitBreaker`] for
//! fault isolation, [`build_http_client`] for pre-configured HTTP clients,
//! and [`run_provider_safely`] for panic-safe provider execution.

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, IF_MODIFIED_SINCE, IF_NONE_MATCH};

use crate::error::{CoreError, Result};

const DEFAULT_TIMEOUT_SECONDS: u64 = 60;
const DEFAULT_POOL_IDLE_SECONDS: u64 = 90;
const DEFAULT_USER_AGENT: &str = "open-social-data/0.1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: usize,
    pub base_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(250),
        }
    }
}

impl RetryPolicy {
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        let multiplier = 2_u32.saturating_pow(attempt.saturating_sub(1) as u32);
        self.base_delay.saturating_mul(multiplier)
    }
}

pub async fn retry_async<T, F, Fut>(policy: RetryPolicy, mut operation: F) -> Result<T>
where
    F: FnMut(usize) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let attempts = policy.max_attempts.max(1);
    let mut last_error = None;

    for attempt in 1..=attempts {
        match operation(attempt).await {
            Ok(value) => return Ok(value),
            Err(error) => {
                last_error = Some(error);
                if attempt < attempts {
                    tokio::time::sleep(policy.delay_for_attempt(attempt)).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| CoreError::Internal("retry operation failed".to_string())))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    failure_threshold: usize,
    state: Arc<Mutex<CircuitBreakerState>>,
}

#[derive(Debug, Clone)]
struct CircuitBreakerState {
    failures: usize,
    state: CircuitState,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize) -> Self {
        Self {
            failure_threshold: failure_threshold.max(1),
            state: Arc::new(Mutex::new(CircuitBreakerState {
                failures: 0,
                state: CircuitState::Closed,
            })),
        }
    }

    pub fn allow_request(&self) -> bool {
        self.state
            .lock()
            .map(|state| state.state == CircuitState::Closed)
            .unwrap_or(false)
    }

    pub fn record_success(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.failures = 0;
            state.state = CircuitState::Closed;
        }
    }

    pub fn record_failure(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.failures += 1;
            if state.failures >= self.failure_threshold {
                state.state = CircuitState::Open;
            }
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
            .lock()
            .map(|state| state.state)
            .unwrap_or(CircuitState::Open)
    }
}

pub fn build_http_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
        .pool_idle_timeout(Duration::from_secs(DEFAULT_POOL_IDLE_SECONDS))
        .build()?)
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConditionalRequestMetadata {
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

impl ConditionalRequestMetadata {
    pub fn new(etag: Option<String>, last_modified: Option<String>) -> Self {
        Self {
            etag,
            last_modified,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.etag.is_none() && self.last_modified.is_none()
    }

    pub fn to_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        if let Some(etag) = self.etag.as_deref() {
            headers.insert(
                IF_NONE_MATCH,
                HeaderValue::from_str(etag)
                    .map_err(|error| CoreError::Internal(error.to_string()))?,
            );
        }
        if let Some(last_modified) = self.last_modified.as_deref() {
            headers.insert(
                IF_MODIFIED_SINCE,
                HeaderValue::from_str(last_modified)
                    .map_err(|error| CoreError::Internal(error.to_string()))?,
            );
        }
        Ok(headers)
    }
}

use std::panic::AssertUnwindSafe;

/// Runs a provider future with panic isolation.
pub async fn run_provider_safely<F, T>(future: F) -> Result<T>
where
    F: Future<Output = Result<T>> + Send + 'static,
    T: Send + 'static,
{
    let handle = tokio::runtime::Handle::current();
    let join = tokio::task::spawn_blocking(move || {
        std::panic::catch_unwind(AssertUnwindSafe(|| handle.block_on(future)))
    });

    match join.await {
        Ok(Ok(result)) => result,
        Ok(Err(panic)) => {
            let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic.downcast_ref::<String>() {
                s.clone()
            } else {
                "provider panicked".to_string()
            };
            Err(CoreError::Internal(msg))
        }
        Err(error) => Err(CoreError::Internal(format!(
            "provider task cancelled: {error}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_policy_uses_exponential_delay() {
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(10),
        };

        assert_eq!(policy.delay_for_attempt(1), Duration::from_millis(10));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_millis(20));
        assert_eq!(policy.delay_for_attempt(3), Duration::from_millis(40));
    }

    #[test]
    fn circuit_opens_after_threshold() {
        let breaker = CircuitBreaker::new(2);
        assert!(breaker.allow_request());
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);
        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.allow_request());
        breaker.record_success();
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn conditional_metadata_builds_headers() {
        let metadata = ConditionalRequestMetadata::new(
            Some("\"abc\"".to_string()),
            Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
        );

        let headers = metadata.to_headers().unwrap();

        assert_eq!(headers.get(IF_NONE_MATCH).unwrap(), "\"abc\"");
        assert_eq!(
            headers.get(IF_MODIFIED_SINCE).unwrap(),
            "Wed, 21 Oct 2015 07:28:00 GMT"
        );
    }

    #[tokio::test]
    async fn run_provider_safely_catches_panic() {
        // Panic with &str
        let result = run_provider_safely::<_, ()>(async {
            panic!("crash and burn");
        })
        .await;
        assert!(result.is_err());

        // Non-panicking future returns Ok
        let result = run_provider_safely(async { Ok::<_, crate::error::CoreError>(42_i32) }).await;
        assert_eq!(result.unwrap(), 42);
    }
}
