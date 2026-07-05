//! DigitalNZ provider for curated open social data datasets.
//!
//! This provider keeps `dnz` as the ad hoc search and cache surface while
//! exposing curated DigitalNZ-backed datasets through the existing provider
//! and Parquet export workflow.

use async_trait::async_trait;
use polars::prelude::*;
use reqwest::StatusCode;
use reqwest::header::{ACCEPT, ETAG, HeaderMap, HeaderValue, LAST_MODIFIED, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::pipeline::{RawRecord, RecordBatchBuilder};
use crate::quality::QualityAssertion;
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_BASE_URL: &str = "https://api.digitalnz.org/v3/records.json";
const USER_AGENT_VALUE: &str = "open-social-data/0.1";
const GAZETTE_COLLECTION: &str = "New Zealand Gazette";

#[derive(Clone)]
pub struct DigitalNzProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl Default for DigitalNzProvider {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL, std::env::var("DIGITALNZ_API_KEY").ok())
    }
}

impl DigitalNzProvider {
    pub fn new(base_url: impl Into<String>, api_key: Option<String>) -> Self {
        let client = build_http_client().unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers
    }

    fn require_api_key(&self) -> Result<&str> {
        self.api_key
            .as_deref()
            .ok_or_else(|| CoreError::Internal("DIGITALNZ_API_KEY is not set".to_string()))
    }

    fn record_query_url(&self) -> String {
        self.base_url.clone()
    }

    fn request_url(&self, params: &[(String, String)]) -> Result<reqwest::Url> {
        reqwest::Url::parse_with_params(&self.base_url, params)
            .map_err(|error| CoreError::Internal(error.to_string()))
    }

    fn gazette_params(&self) -> Vec<(String, String)> {
        vec![
            (
                "api_key".to_string(),
                self.api_key.clone().unwrap_or_default(),
            ),
            ("text".to_string(), String::new()),
            ("page".to_string(), "1".to_string()),
            ("per_page".to_string(), "100".to_string()),
            ("sort".to_string(), "date".to_string()),
            ("direction".to_string(), "asc".to_string()),
            (
                "and[primary_collection][]".to_string(),
                GAZETTE_COLLECTION.to_string(),
            ),
        ]
    }

    fn search_fixture_params(&self) -> Vec<(String, String)> {
        vec![
            (
                "api_key".to_string(),
                self.api_key.clone().unwrap_or_default(),
            ),
            ("text".to_string(), "kauri".to_string()),
            ("page".to_string(), "1".to_string()),
            ("per_page".to_string(), "25".to_string()),
            ("sort".to_string(), "date".to_string()),
            ("direction".to_string(), "desc".to_string()),
        ]
    }

    async fn get_json<T>(
        &self,
        params: Vec<(String, String)>,
    ) -> Result<(T, Option<String>, Option<String>)>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self
            .client
            .get(self.request_url(&params)?)
            .headers(Self::headers())
            .send()
            .await?;
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
            return Err(CoreError::NotModified(
                "DigitalNZ returned not modified unexpectedly".to_string(),
            ));
        }
        if !status.is_success() {
            return Err(CoreError::HttpStatus {
                status: status.as_u16(),
                url: self.record_query_url(),
            });
        }
        Ok((response.json::<T>().await?, etag, last_modified))
    }

    fn normalize_records(dataset_id: &str, records: Vec<DigitalNzRecord>) -> Result<DataFrame> {
        let mut builder = RecordBatchBuilder::new();
        for record in records {
            builder.push(record.into_raw_record(dataset_id));
        }
        builder.build()
    }
}

#[async_trait]
impl DatasetProvider for DigitalNzProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "digitalnz".to_string(),
            name: "DigitalNZ".to_string(),
            description: Some(
                "Curated DigitalNZ and New Zealand Gazette dataset provider".to_string(),
            ),
        }
    }

    async fn ping(&self) -> Result<()> {
        let api_key = self.require_api_key()?;
        let params = vec![
            ("api_key".to_string(), api_key.to_string()),
            ("text".to_string(), String::new()),
            ("page".to_string(), "1".to_string()),
            ("per_page".to_string(), "1".to_string()),
        ];
        let _: (DigitalNzSearchResponse, Option<String>, Option<String>) =
            self.get_json(params).await?;
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        Ok(Catalog {
            datasets: vec![
                DatasetMetadata {
                    id: "nz_gazette".to_string(),
                    name: "New Zealand Gazette".to_string(),
                    description: Some(
                        "Curated DigitalNZ Gazette search results normalized for Parquet export"
                            .to_string(),
                    ),
                    version: Some("curated-1".to_string()),
                    source_url: Some(format!(
                        "{}?text=&sort=date&direction=asc&and[primary_collection][]={}",
                        self.base_url, GAZETTE_COLLECTION
                    )),
                },
                DatasetMetadata {
                    id: "search_fixture".to_string(),
                    name: "DigitalNZ Search Fixture".to_string(),
                    description: Some(
                        "Hermetic DigitalNZ search fixture for provider validation".to_string(),
                    ),
                    version: Some("fixture-1".to_string()),
                    source_url: Some(format!(
                        "{}?text=kauri&page=1&per_page=25&sort=date&direction=desc",
                        self.base_url
                    )),
                },
            ],
        })
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        let api_key = self.require_api_key()?.to_string();
        let mut params = match dataset_id {
            "nz_gazette" => self.gazette_params(),
            "search_fixture" => self.search_fixture_params(),
            _ => return Err(CoreError::NotFound(dataset_id.to_string())),
        };
        if let Some(value) = params.iter_mut().find(|(name, _)| name == "api_key") {
            value.1 = api_key;
        }

        let mut headers = Self::headers();
        headers.extend(options.conditional.to_headers()?);

        let response = self
            .client
            .get(self.request_url(&params)?)
            .headers(headers)
            .send()
            .await?;
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
                url: self.record_query_url(),
            });
        }

        let payload: DigitalNzSearchResponse = response.json().await?;
        let frame = Self::normalize_records(dataset_id, payload.search.results)?;
        Ok(FetchResult::fetched(frame, etag, last_modified))
    }

    fn quality_assertions(&self) -> Vec<QualityAssertion> {
        vec![
            QualityAssertion::non_null("record_id"),
            QualityAssertion::non_null("title"),
        ]
    }
}

#[derive(Debug, Deserialize)]
struct DigitalNzSearchResponse {
    search: DigitalNzSearchMetadata,
}

#[derive(Debug, Deserialize)]
struct DigitalNzSearchMetadata {
    #[serde(default)]
    results: Vec<DigitalNzRecord>,
}

#[derive(Debug, Deserialize)]
struct DigitalNzRecord {
    id: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    collection: Option<Vec<String>>,
    #[serde(default)]
    content_partner: Option<Vec<String>>,
    #[serde(default)]
    creator: Option<Vec<String>>,
    #[serde(default)]
    display_url: Option<String>,
    #[serde(default)]
    source_url: Option<String>,
    #[serde(default)]
    category: Option<Vec<String>>,
    #[serde(default)]
    date: Option<Vec<String>>,
    #[serde(default)]
    syndication_date: Option<String>,
    #[serde(flatten, default)]
    extra_fields: std::collections::BTreeMap<String, Value>,
}

impl DigitalNzRecord {
    fn into_raw_record(self, dataset_id: &str) -> RawRecord {
        RawRecord::new()
            .with("provider", "digitalnz")
            .with("dataset_id", dataset_id)
            .with("record_id", self.id)
            .with("title", self.title)
            .with("description", self.description.unwrap_or_default())
            .with("display_url", self.display_url.unwrap_or_default())
            .with("source_url", self.source_url.unwrap_or_default())
            .with("collection", join_values(self.collection.as_deref()))
            .with(
                "content_partner",
                join_values(self.content_partner.as_deref()),
            )
            .with("category", join_values(self.category.as_deref()))
            .with("creator", join_values(self.creator.as_deref()))
            .with("date", join_values(self.date.as_deref()))
            .with(
                "syndication_date",
                self.syndication_date.unwrap_or_default(),
            )
            .with(
                "extra_fields_json",
                serde_json::to_string(&self.extra_fields).unwrap_or_else(|_| "{}".to_string()),
            )
    }
}

fn join_values(values: Option<&[String]>) -> String {
    values.map(|items| items.join(", ")).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardening::ConditionalRequestMetadata;
    use crate::providers::test_support::complete_request;

    #[test]
    fn metadata_reports_digitalnz_provider() {
        let provider = DigitalNzProvider::new(
            "https://example.test/v3/records.json",
            Some("key".to_string()),
        );
        let metadata = provider.metadata();
        assert_eq!(metadata.id, "digitalnz");
        assert_eq!(metadata.name, "DigitalNZ");
    }

    #[tokio::test]
    async fn list_datasets_exposes_curated_digitalnz_entries() {
        let provider = DigitalNzProvider::new(
            "https://example.test/v3/records.json",
            Some("key".to_string()),
        );
        let catalog = provider.list_datasets().await.unwrap();
        assert_eq!(catalog.datasets.len(), 2);
        assert_eq!(catalog.datasets[0].id, "nz_gazette");
        assert_eq!(catalog.datasets[1].id, "search_fixture");
    }

    #[tokio::test]
    async fn ping_uses_authenticated_search_request() {
        let body = r#"{"search":{"results":[]}}"#;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let completed = complete_request(
            Box::leak(response.into_boxed_str()),
            |base_url| async move {
                let provider = DigitalNzProvider::new(base_url, Some("test-key".to_string()));
                provider.ping().await
            },
        )
        .await;

        completed.output.unwrap();
        assert!(completed.request.contains("api_key=test-key"));
        assert!(completed.request.contains("per_page=1"));
    }

    #[tokio::test]
    async fn fetch_normalizes_gazette_records() {
        let payload = include_str!("../../tests/fixtures/digitalnz_nz_gazette_response.json");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \"gazette-etag\"\r\nLast-Modified: Wed, 21 Oct 2015 07:28:00 GMT\r\nContent-Length: {}\r\n\r\n{}",
            payload.len(),
            payload
        );
        let completed = complete_request(
            Box::leak(response.into_boxed_str()),
            |base_url| async move {
                let provider = DigitalNzProvider::new(base_url, Some("test-key".to_string()));
                provider
                    .fetch_dataset_with_options(
                        "nz_gazette",
                        FetchOptions::new(ConditionalRequestMetadata::default()),
                    )
                    .await
            },
        )
        .await;

        let frame = completed.output.unwrap().into_frame().unwrap();
        assert_eq!(frame.height(), 2);
        assert_eq!(
            frame.column("provider").unwrap().str().unwrap().get(0),
            Some("digitalnz")
        );
        assert_eq!(
            frame.column("dataset_id").unwrap().str().unwrap().get(0),
            Some("nz_gazette")
        );
        assert_eq!(
            frame.column("record_id").unwrap().str().unwrap().get(1),
            Some("gaz-2")
        );
        assert_eq!(
            frame
                .column("extra_fields_json")
                .unwrap()
                .str()
                .unwrap()
                .get(0),
            Some("{\"license\":\"CC-BY\"}")
        );
        assert!(
            completed
                .request
                .contains("and%5Bprimary_collection%5D%5B%5D=New+Zealand+Gazette")
                || completed.request.contains("and[primary_collection][]")
        );
    }

    #[tokio::test]
    async fn fetch_normalizes_search_fixture_records() {
        let payload = include_str!("../../tests/fixtures/digitalnz_search_fixture_response.json");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            payload.len(),
            payload
        );
        let completed = complete_request(
            Box::leak(response.into_boxed_str()),
            |base_url| async move {
                let provider = DigitalNzProvider::new(base_url, Some("test-key".to_string()));
                provider
                    .fetch_dataset_with_options("search_fixture", FetchOptions::default())
                    .await
            },
        )
        .await;

        let frame = completed.output.unwrap().into_frame().unwrap();
        assert_eq!(frame.height(), 2);
        assert_eq!(
            frame.column("record_id").unwrap().str().unwrap().get(0),
            Some("search-1")
        );
        assert_eq!(
            frame
                .column("content_partner")
                .unwrap()
                .str()
                .unwrap()
                .get(0),
            Some("Auckland Libraries")
        );
    }
}
