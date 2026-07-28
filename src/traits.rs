//! Core abstractions for dataset providers.
//!
//! This module defines the [`DatasetProvider`] trait that all data source
//! providers must implement, along with [`FetchOptions`] for conditional
//! requests and [`FetchResult`] for representing fetch outcomes.

use crate::error::Result;
use crate::hardening::ConditionalRequestMetadata;
use crate::models::{Catalog, ProviderMetadata};
use crate::quality::QualityAssertion;
use async_trait::async_trait;
use polars::prelude::DataFrame;

/// Options for dataset fetch requests, including conditional request headers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FetchOptions {
    pub conditional: ConditionalRequestMetadata,
}

impl FetchOptions {
    /// Creates a new `FetchOptions` with the given conditional request metadata.
    pub fn new(conditional: ConditionalRequestMetadata) -> Self {
        Self { conditional }
    }
}

/// The result of a dataset fetch operation.
///
/// Indicates whether the dataset was actually fetched (modified) or has not
/// been modified since the last conditional request.
#[derive(Debug, Clone)]
pub enum FetchResult {
    /// The dataset was modified and new data was fetched.
    Fetched {
        frame: DataFrame,
        etag: Option<String>,
        last_modified: Option<String>,
    },
    /// The dataset has not been modified since the last fetch.
    NotModified {
        etag: Option<String>,
        last_modified: Option<String>,
    },
}

impl FetchResult {
    /// Creates a `Fetched` variant with no ETag or last-modified metadata.
    pub fn from_frame(frame: DataFrame) -> Self {
        Self::Fetched {
            frame,
            etag: None,
            last_modified: None,
        }
    }

    /// Creates a `Fetched` variant with the given frame and response metadata.
    pub fn fetched(frame: DataFrame, etag: Option<String>, last_modified: Option<String>) -> Self {
        Self::Fetched {
            frame,
            etag,
            last_modified,
        }
    }

    /// Creates a `NotModified` variant with the given response metadata.
    pub fn not_modified(etag: Option<String>, last_modified: Option<String>) -> Self {
        Self::NotModified {
            etag,
            last_modified,
        }
    }

    /// Returns `true` if the result is `NotModified`.
    pub fn is_not_modified(&self) -> bool {
        matches!(self, Self::NotModified { .. })
    }

    /// Returns the ETag header value, if any.
    pub fn etag(&self) -> Option<&str> {
        match self {
            Self::Fetched { etag, .. } | Self::NotModified { etag, .. } => etag.as_deref(),
        }
    }

    /// Returns the `Last-Modified` header value, if any.
    pub fn last_modified(&self) -> Option<&str> {
        match self {
            Self::Fetched { last_modified, .. } | Self::NotModified { last_modified, .. } => {
                last_modified.as_deref()
            }
        }
    }

    /// Consumes the result and returns the inner `DataFrame`, or `None` if not modified.
    pub fn into_frame(self) -> Option<DataFrame> {
        match self {
            Self::Fetched { frame, .. } => Some(frame),
            Self::NotModified { .. } => None,
        }
    }
}

/// Core abstraction for a data source provider.
///
/// Implementations connect to a specific national statistics agency's API
/// (e.g., ABS SDMX, Stats NZ OData) and provide dataset listing and fetching.
#[async_trait]
pub trait DatasetProvider: Send + Sync {
    /// Returns metadata about the provider.
    fn metadata(&self) -> ProviderMetadata;

    /// Pings the provider to check connection health.
    async fn ping(&self) -> Result<()>;

    /// Lists available datasets from this provider.
    async fn list_datasets(&self) -> Result<Catalog>;

    /// Fetches a specific dataset by ID and returns it as a DataFrame.
    ///
    /// In a more advanced implementation, this could return a stream of DataFrames.
    async fn fetch_dataset(&self, dataset_id: &str) -> Result<DataFrame> {
        let result = self
            .fetch_dataset_with_options(dataset_id, FetchOptions::default())
            .await?;
        match result {
            FetchResult::Fetched { frame, .. } => Ok(frame),
            FetchResult::NotModified { .. } => Err(crate::error::CoreError::NotModified(format!(
                "{dataset_id} returned not modified without conditional request"
            ))),
        }
    }

    /// Fetches a dataset with conditional request options (ETag / Last-Modified).
    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        _options: FetchOptions,
    ) -> Result<FetchResult>;

    /// Returns provider-specific quality assertions for normalized payloads.
    fn quality_assertions(&self) -> Vec<QualityAssertion> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    fn sample_dataframe() -> DataFrame {
        DataFrame::new(3, vec![Series::new("a".into(), &[1, 2, 3]).into()]).unwrap()
    }

    #[test]
    fn test_fetch_result_from_frame() {
        let df = sample_dataframe();
        let result = FetchResult::from_frame(df.clone());

        match result {
            FetchResult::Fetched {
                frame,
                etag,
                last_modified,
            } => {
                assert_eq!(frame, df);
                assert_eq!(etag, None);
                assert_eq!(last_modified, None);
            }
            _ => panic!("Expected Fetched variant"),
        }
    }

    #[test]
    fn test_fetch_result_fetched() {
        let df = sample_dataframe();
        let result = FetchResult::fetched(
            df.clone(),
            Some("etag123".to_string()),
            Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
        );

        match result {
            FetchResult::Fetched {
                frame,
                etag,
                last_modified,
            } => {
                assert_eq!(frame, df);
                assert_eq!(etag.as_deref(), Some("etag123"));
                assert_eq!(
                    last_modified.as_deref(),
                    Some("Wed, 21 Oct 2015 07:28:00 GMT")
                );
            }
            _ => panic!("Expected Fetched variant"),
        }
    }

    #[test]
    fn test_fetch_result_not_modified() {
        let result = FetchResult::not_modified(
            Some("etag123".to_string()),
            Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
        );

        assert!(result.is_not_modified());

        match result {
            FetchResult::NotModified {
                etag,
                last_modified,
            } => {
                assert_eq!(etag.as_deref(), Some("etag123"));
                assert_eq!(
                    last_modified.as_deref(),
                    Some("Wed, 21 Oct 2015 07:28:00 GMT")
                );
            }
            _ => panic!("Expected NotModified variant"),
        }
    }

    #[test]
    fn test_fetch_result_is_not_modified() {
        let df = sample_dataframe();
        let fetched = FetchResult::from_frame(df);
        assert!(!fetched.is_not_modified());

        let not_modified = FetchResult::not_modified(None, None);
        assert!(not_modified.is_not_modified());
    }

    #[test]
    fn test_fetch_result_etag() {
        let df = sample_dataframe();

        let fetched_with_etag = FetchResult::fetched(df.clone(), Some("etag1".to_string()), None);
        assert_eq!(fetched_with_etag.etag(), Some("etag1"));

        let fetched_no_etag = FetchResult::from_frame(df);
        assert_eq!(fetched_no_etag.etag(), None);

        let not_modified_with_etag = FetchResult::not_modified(Some("etag2".to_string()), None);
        assert_eq!(not_modified_with_etag.etag(), Some("etag2"));

        let not_modified_no_etag = FetchResult::not_modified(None, None);
        assert_eq!(not_modified_no_etag.etag(), None);
    }

    #[test]
    fn test_fetch_result_last_modified() {
        let df = sample_dataframe();
        let date_str = "Wed, 21 Oct 2015 07:28:00 GMT";

        let fetched_with_date = FetchResult::fetched(df.clone(), None, Some(date_str.to_string()));
        assert_eq!(fetched_with_date.last_modified(), Some(date_str));

        let fetched_no_date = FetchResult::from_frame(df);
        assert_eq!(fetched_no_date.last_modified(), None);

        let not_modified_with_date = FetchResult::not_modified(None, Some(date_str.to_string()));
        assert_eq!(not_modified_with_date.last_modified(), Some(date_str));

        let not_modified_no_date = FetchResult::not_modified(None, None);
        assert_eq!(not_modified_no_date.last_modified(), None);
    }

    #[test]
    fn test_fetch_result_into_frame() {
        let df = sample_dataframe();
        let expected_df = df.clone();

        let fetched = FetchResult::from_frame(df);
        assert_eq!(fetched.into_frame(), Some(expected_df));

        let not_modified = FetchResult::not_modified(None, None);
        assert_eq!(not_modified.into_frame(), None);
    }
}
