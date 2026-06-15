//! Statistics New Zealand (Stats NZ) OData API provider.
//!
//! Fetches entity set listings and dataset records from the Stats NZ
//! OData v1 endpoint. Supports conditional requests via ETag/Last-Modified
//! headers.

use async_trait::async_trait;
use polars::prelude::*;
use reqwest::StatusCode;
use reqwest::header::{ACCEPT, ETAG, HeaderMap, HeaderValue, LAST_MODIFIED, USER_AGENT};
use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_BASE_URL: &str = "https://api.stats.govt.nz/opendata/v1";
const USER_AGENT_VALUE: &str = "open-social-data/0.1";

#[derive(Clone)]
pub struct StatsNzProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for StatsNzProvider {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL)
    }
}

impl StatsNzProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = build_http_client().unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub fn service_document_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn dataset_url(&self, dataset_id: &str) -> String {
        format!("{}/{}", self.base_url, dataset_id.trim_start_matches('/'))
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers
    }

    async fn get_json<T>(&self, url: String) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .get(&url)
            .headers(Self::headers())
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(CoreError::HttpStatus {
                status: status.as_u16(),
                url,
            });
        }
        Ok(response.json::<T>().await?)
    }
}

#[async_trait]
impl DatasetProvider for StatsNzProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "stats_nz".to_string(),
            name: "Stats NZ".to_string(),
            description: Some("Stats NZ OData API".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        let _: ODataServiceDocument = self.get_json(self.service_document_url()).await?;
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        let response: ODataServiceDocument = self.get_json(self.service_document_url()).await?;
        Ok(Catalog {
            datasets: response
                .value
                .into_iter()
                .map(|item| {
                    let source_url = self.dataset_url(&item.name);
                    DatasetMetadata {
                        id: item.name.clone(),
                        name: item.title.unwrap_or(item.name),
                        description: None,
                        version: None,
                        source_url: Some(source_url),
                    }
                })
                .collect(),
        })
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        let url = self.dataset_url(dataset_id);
        let mut headers = Self::headers();
        headers.extend(options.conditional.to_headers()?);
        let response = self.client.get(&url).headers(headers).send().await?;
        let status = response.status();
        let response_headers = response.headers().clone();
        let etag = response_headers
            .get(ETAG)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let last_modified = response_headers
            .get(LAST_MODIFIED)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        if status == StatusCode::NOT_MODIFIED {
            return Ok(FetchResult::not_modified(etag, last_modified));
        }
        if !status.is_success() {
            return Err(CoreError::HttpStatus {
                status: status.as_u16(),
                url,
            });
        }
        let payload = response.bytes().await?;
        Ok(FetchResult::fetched(
            provider_payload_frame("stats_nz", dataset_id, payload.len())?,
            etag,
            last_modified,
        ))
    }
}

#[derive(Debug, Deserialize)]
struct ODataServiceDocument {
    value: Vec<ODataEntitySet>,
}

#[derive(Debug, Deserialize)]
struct ODataEntitySet {
    name: String,
    #[serde(default)]
    title: Option<String>,
}

fn provider_payload_frame(
    provider: &str,
    dataset_id: &str,
    payload_bytes: usize,
) -> Result<DataFrame> {
    let provider = Series::new("provider".into(), &[provider]);
    let dataset_id = Series::new("dataset_id".into(), &[dataset_id]);
    let payload_bytes = Series::new("payload_bytes".into(), &[payload_bytes as u64]);
    DataFrame::new(vec![
        provider.into(),
        dataset_id.into(),
        payload_bytes.into(),
    ])
    .map_err(|error| CoreError::TransformationError(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardening::ConditionalRequestMetadata;
    use crate::providers::test_support::complete_request;

    #[test]
    fn builds_stats_nz_urls() {
        let provider = StatsNzProvider::new("https://example.test/opendata/v1/");
        assert_eq!(
            provider.service_document_url(),
            "https://example.test/opendata/v1"
        );
        assert_eq!(
            provider.dataset_url("/Population"),
            "https://example.test/opendata/v1/Population"
        );
    }

    #[tokio::test]
    async fn fetch_returns_not_modified_and_sends_conditional_headers() {
        let response = concat!(
            "HTTP/1.1 304 Not Modified\r\n",
            "ETag: \"stats-etag\"\r\n",
            "Last-Modified: Wed, 21 Oct 2015 07:28:00 GMT\r\n",
            "Content-Length: 0\r\n",
            "\r\n",
        );
        let completed = complete_request(response, |base_url| async move {
            let provider = StatsNzProvider::new(base_url);
            provider
                .fetch_dataset_with_options(
                    "/Population",
                    FetchOptions::new(ConditionalRequestMetadata::new(
                        Some("\"cached-etag\"".to_string()),
                        Some("Tue, 20 Oct 2015 07:28:00 GMT".to_string()),
                    )),
                )
                .await
        })
        .await;

        let result = completed.output.unwrap();

        assert!(result.is_not_modified());
        assert_eq!(result.etag(), Some("\"stats-etag\""));
        assert_eq!(
            result.last_modified(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT")
        );
        assert!(completed.request.starts_with("GET /Population "));
        assert!(completed.request.contains("if-none-match: \"cached-etag\""));
        assert!(
            completed
                .request
                .contains("if-modified-since: Tue, 20 Oct 2015 07:28:00 GMT")
        );
    }
}
