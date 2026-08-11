//! Auckland Museum Linked Open Data provider.
//!
//! This is a deliberately metadata-only, bounded redundancy provider.  It does
//! not download Cenotaph biographies, contributor material, documents, or
//! media.  Those require a current rights review at the individual-resource
//! level.

use async_trait::async_trait;
use reqwest::header::{
    ACCEPT, CONTENT_TYPE, ETAG, HeaderMap, HeaderValue, LAST_MODIFIED, USER_AGENT,
};
use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::pipeline::{RawRecord, RecordBatchBuilder};
use crate::quality::QualityAssertion;
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_SPARQL_URL: &str = "https://api.aucklandmuseum.com/sparql";
const DATASET_ID: &str = "cenotaph_metadata_sample";
const USER_AGENT_VALUE: &str = "open-social-data/0.1";
const MAX_SAMPLE_ROWS: usize = 100;
const CENOTAPH_QUERY: &str = "PREFIX am: <http://collections.aucklandmuseum.com/ontology/core/>\nSELECT ?subject WHERE { ?subject a am:MilitaryPerson } LIMIT 100";

#[derive(Clone)]
pub struct AucklandMuseumProvider {
    client: reqwest::Client,
    sparql_url: String,
}

impl Default for AucklandMuseumProvider {
    fn default() -> Self {
        Self::new(DEFAULT_SPARQL_URL)
    }
}

impl AucklandMuseumProvider {
    pub fn new(sparql_url: impl Into<String>) -> Self {
        Self {
            client: build_http_client().unwrap_or_else(|_| reqwest::Client::new()),
            sparql_url: sparql_url.into(),
        }
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/sparql-results+json"),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers
    }

    async fn fetch_bindings(&self, options: FetchOptions) -> Result<FetchResult> {
        let mut headers = Self::headers();
        headers.extend(options.conditional.to_headers()?);
        let response = self
            .client
            .post(&self.sparql_url)
            .headers(headers)
            .form(&[("query", CENOTAPH_QUERY)])
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
        if status == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(FetchResult::not_modified(etag, last_modified));
        }
        if !status.is_success() {
            return Err(CoreError::HttpStatus {
                status: status.as_u16(),
                url: self.sparql_url.clone(),
            });
        }
        let payload: SparqlResponse = response.json().await?;
        let mut builder = RecordBatchBuilder::new();
        for binding in payload.results.bindings.into_iter().take(MAX_SAMPLE_ROWS) {
            if let Some(subject) = binding.subject {
                builder.push(
                    RawRecord::new()
                        .with("provider", "auckland_museum")
                        .with("dataset_id", DATASET_ID)
                        .with("record_id", subject.value.clone())
                        .with("source_url", subject.value)
                        .with("record_type", "am:MilitaryPerson")
                        .with("capture_scope", "metadata_only"),
                );
            }
        }
        Ok(FetchResult::fetched(builder.build()?, etag, last_modified))
    }
}

#[async_trait]
impl DatasetProvider for AucklandMuseumProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "auckland_museum".to_string(),
            name: "Auckland Museum Linked Open Data".to_string(),
            description: Some("Bounded, metadata-only Cenotaph redundancy source".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        self.fetch_bindings(FetchOptions::default())
            .await
            .map(|_| ())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        Ok(Catalog { datasets: vec![DatasetMetadata {
            id: DATASET_ID.to_string(),
            name: "Online Cenotaph metadata sample".to_string(),
            description: Some("At most 100 Cenotaph subject URIs from the public SPARQL endpoint; excludes biographies, documents, and media pending rights review.".to_string()),
            version: Some("metadata-sample-1".to_string()),
            source_url: Some(self.sparql_url.clone()),
        }] })
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        if dataset_id != DATASET_ID {
            return Err(CoreError::NotFound(dataset_id.to_string()));
        }
        self.fetch_bindings(options).await
    }

    fn quality_assertions(&self) -> Vec<QualityAssertion> {
        vec![
            QualityAssertion::non_null("record_id"),
            QualityAssertion::non_null("source_url"),
        ]
    }
}

#[derive(Debug, Deserialize)]
struct SparqlResponse {
    results: SparqlResults,
}
#[derive(Debug, Deserialize)]
struct SparqlResults {
    bindings: Vec<SparqlBinding>,
}
#[derive(Debug, Deserialize)]
struct SparqlBinding {
    subject: Option<SparqlValue>,
}
#[derive(Debug, Deserialize)]
struct SparqlValue {
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::DatasetProvider;

    #[tokio::test]
    async fn lists_bounded_metadata_dataset() {
        let catalog = AucklandMuseumProvider::default()
            .list_datasets()
            .await
            .unwrap();
        assert_eq!(catalog.datasets[0].id, DATASET_ID);
        assert!(
            catalog.datasets[0]
                .description
                .as_deref()
                .unwrap()
                .contains("excludes")
        );
    }
}
