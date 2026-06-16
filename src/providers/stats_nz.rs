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
use serde_json::{Map, Value};

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::pipeline::{RawRecord, RecordBatchBuilder};
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
        let payload: Value = response.json().await?;
        let mut frame = odata_json_to_frame(dataset_id, &payload)?;
        let mut next_link = odata_next_link(&payload);
        while let Some(url) = next_link {
            let page = self.get_json::<Value>(url).await?;
            let page_frame = odata_json_to_frame(dataset_id, &page)?;
            frame = frame.vstack(&page_frame).map_err(|error| {
                CoreError::TransformationError(format!(
                    "failed to append Stats NZ OData page: {error}"
                ))
            })?;
            next_link = odata_next_link(&page);
        }

        Ok(FetchResult::fetched(frame, etag, last_modified))
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

fn odata_json_to_frame(dataset_id: &str, payload: &Value) -> Result<DataFrame> {
    let rows = payload
        .get("value")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CoreError::TransformationError("Stats NZ OData JSON missing value rows".to_string())
        })?;
    let mut builder = RecordBatchBuilder::new();
    for row in rows {
        let object = row.as_object().ok_or_else(|| {
            CoreError::TransformationError("Stats NZ OData row is not an object".to_string())
        })?;
        builder.push(odata_row_to_record(dataset_id, object));
    }
    builder.build()
}

fn odata_row_to_record(dataset_id: &str, object: &Map<String, Value>) -> RawRecord {
    let mut record = RawRecord::new()
        .with("provider", "stats_nz")
        .with("dataset_id", dataset_id);
    for (key, value) in object {
        if key.starts_with('@') {
            continue;
        }
        record = record.with(key, value_to_cell(value));
    }
    record
}

fn odata_next_link(payload: &Value) -> Option<String> {
    payload
        .get("@odata.nextLink")
        .or_else(|| payload.get("odata.nextLink"))
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn value_to_cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(_) | Value::Object(_) => value.to_string(),
    }
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

    #[tokio::test]
    async fn fetch_parses_odata_value_rows() {
        let body = serde_json::json!({
            "value": [
                {
                    "Period": "2024",
                    "Region": "Auckland",
                    "Value": 123.4,
                    "@odata.etag": "row-etag"
                },
                {
                    "Period": "2025",
                    "Region": "Wellington",
                    "Value": 130.0
                }
            ]
        });
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \"stats-data\"\r\nContent-Length: {}\r\n\r\n{}",
            body.to_string().len(),
            body
        );
        let completed = complete_request(
            Box::leak(response.into_boxed_str()),
            |base_url| async move {
                let provider = StatsNzProvider::new(base_url);
                provider
                    .fetch_dataset_with_options("/Population", FetchOptions::default())
                    .await
            },
        )
        .await;

        let frame = completed.output.unwrap().into_frame().unwrap();

        assert_eq!(frame.height(), 2);
        assert_eq!(
            frame.column("provider").unwrap().str().unwrap().get(0),
            Some("stats_nz")
        );
        assert_eq!(
            frame.column("dataset_id").unwrap().str().unwrap().get(0),
            Some("/Population")
        );
        assert_eq!(
            frame.column("Region").unwrap().str().unwrap().get(1),
            Some("Wellington")
        );
        assert_eq!(
            frame.column("Value").unwrap().str().unwrap().get(0),
            Some("123.4")
        );
        assert!(frame.column("@odata.etag").is_err());
    }
}
