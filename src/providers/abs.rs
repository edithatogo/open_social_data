//! Australian Bureau of Statistics (ABS) SDMX API provider.
//!
//! Fetches dataflow listings and dataset records from the ABS SDMX-JSON
//! API. Supports conditional requests via ETag/Last-Modified headers.

use async_trait::async_trait;
use polars::prelude::*;
use reqwest::StatusCode;
use reqwest::header::{ACCEPT, ETAG, HeaderMap, HeaderValue, LAST_MODIFIED, USER_AGENT};
use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_BASE_URL: &str = "https://api.abs.gov.au";
const SDMX_STRUCTURE_ACCEPT: &str = "application/vnd.sdmx.structure+json;version=2.0.0";
const USER_AGENT_VALUE: &str = "open-social-data/0.1";

#[derive(Clone)]
pub struct AbsProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for AbsProvider {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL)
    }
}

impl AbsProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = build_http_client().unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub fn dataflows_url(&self) -> String {
        format!("{}/dataflow/ABS/all?detail=referencepartial", self.base_url)
    }

    pub fn datastructure_url(&self, dataflow_id: &str) -> String {
        format!(
            "{}/datastructure/ABS/{}/latest?detail=full&references=children",
            self.base_url, dataflow_id
        )
    }

    pub fn dataset_url(&self, dataset_id: &str) -> String {
        format!("{}/data/{}", self.base_url, dataset_id)
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static(SDMX_STRUCTURE_ACCEPT));
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
impl DatasetProvider for AbsProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "abs".to_string(),
            name: "Australian Bureau of Statistics".to_string(),
            description: Some("Australian Bureau of Statistics SDMX API".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        let _: AbsDataflowResponse = self.get_json(self.dataflows_url()).await?;
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        let response: AbsDataflowResponse = self.get_json(self.dataflows_url()).await?;
        Ok(Catalog {
            datasets: response
                .data
                .dataflows
                .into_iter()
                .map(|flow| {
                    let name = localized_name(&flow.name)
                        .unwrap_or_else(|| "Unnamed ABS dataflow".to_string());
                    let source_url = self.dataset_url(&flow.id);
                    DatasetMetadata {
                        id: flow.id,
                        name,
                        description: None,
                        version: flow.version,
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
            provider_payload_frame("abs", dataset_id, payload.len())?,
            etag,
            last_modified,
        ))
    }
}

#[derive(Debug, Deserialize)]
struct AbsDataflowResponse {
    data: AbsDataflowData,
}

#[derive(Debug, Deserialize)]
struct AbsDataflowData {
    dataflows: Vec<AbsDataflow>,
}

#[derive(Debug, Deserialize)]
struct AbsDataflow {
    id: String,
    version: Option<String>,
    #[serde(default)]
    name: Vec<LocalizedName>,
}

#[derive(Debug, Deserialize)]
struct LocalizedName {
    locale: Option<String>,
    value: String,
}

fn localized_name(names: &[LocalizedName]) -> Option<String> {
    names
        .iter()
        .find(|name| name.locale.as_deref() == Some("en"))
        .or_else(|| names.first())
        .map(|name| name.value.clone())
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
    fn builds_abs_urls() {
        let provider = AbsProvider::new("https://example.test/");
        assert_eq!(
            provider.dataflows_url(),
            "https://example.test/dataflow/ABS/all?detail=referencepartial"
        );
        assert_eq!(
            provider.datastructure_url("QBIS"),
            "https://example.test/datastructure/ABS/QBIS/latest?detail=full&references=children"
        );
    }

    #[test]
    fn selects_english_name_when_available() {
        let names = vec![
            LocalizedName {
                locale: Some("mi".to_string()),
                value: "Ingoa".to_string(),
            },
            LocalizedName {
                locale: Some("en".to_string()),
                value: "Business indicators".to_string(),
            },
        ];
        assert_eq!(
            localized_name(&names),
            Some("Business indicators".to_string())
        );
    }

    #[tokio::test]
    async fn fetch_returns_not_modified_and_sends_conditional_headers() {
        let response = concat!(
            "HTTP/1.1 304 Not Modified\r\n",
            "ETag: \"abs-etag\"\r\n",
            "Last-Modified: Wed, 21 Oct 2015 07:28:00 GMT\r\n",
            "Content-Length: 0\r\n",
            "\r\n",
        );
        let completed = complete_request(response, |base_url| async move {
            let provider = AbsProvider::new(base_url);
            provider
                .fetch_dataset_with_options(
                    "QBIS",
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
        assert_eq!(result.etag(), Some("\"abs-etag\""));
        assert_eq!(
            result.last_modified(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT")
        );
        assert!(completed.request.starts_with("GET /data/QBIS "));
        assert!(completed.request.contains("if-none-match: \"cached-etag\""));
        assert!(
            completed
                .request
                .contains("if-modified-since: Tue, 20 Oct 2015 07:28:00 GMT")
        );
    }
}
