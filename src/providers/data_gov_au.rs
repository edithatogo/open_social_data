//! Data.gov.au CKAN provider for official Australian FOI statistics.
//!
//! This provider consumes the public CKAN API and the CSV resource published by
//! the Office of the Australian Information Commissioner. It intentionally
//! exposes aggregate government statistics, not Right to Know request records.

use std::io::Cursor;

use async_trait::async_trait;
use polars::prelude::*;
use reqwest::StatusCode;
use reqwest::header::{ACCEPT, ETAG, HeaderMap, HeaderValue, LAST_MODIFIED, USER_AGENT};
use serde::Deserialize;

use crate::error::{CoreError, Result};
use crate::hardening::build_http_client;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_BASE_URL: &str = "https://data.gov.au/data";
const DATASET_ID: &str = "freedom_of_information_statistics";
const PACKAGE_ID: &str = "freedom-of-information-statistics";
const USER_AGENT_VALUE: &str = "open-social-data/0.1";

#[derive(Clone)]
pub struct DataGovAuProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for DataGovAuProvider {
    fn default() -> Self {
        Self::new(DEFAULT_BASE_URL)
    }
}

impl DataGovAuProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = build_http_client().unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/csv"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers
    }

    fn package_url(&self) -> String {
        format!(
            "{}/api/3/action/package_show?id={PACKAGE_ID}",
            self.base_url
        )
    }

    async fn package(&self) -> Result<PackageResponse> {
        let url = self.package_url();
        let response = self
            .client
            .get(&url)
            .headers(Self::headers())
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(CoreError::HttpStatus {
                status: response.status().as_u16(),
                url,
            });
        }
        Ok(response.json().await?)
    }

    async fn csv_resource(&self) -> Result<Resource> {
        let package = self.package().await?;
        package
            .result
            .resources
            .into_iter()
            .find(|resource| {
                resource.format.eq_ignore_ascii_case("csv")
                    && resource.name.starts_with("FOI requests, costs and charges")
            })
            .ok_or_else(|| {
                CoreError::NotFound("current Data.gov.au FOI statistics CSV".to_string())
            })
    }

    async fn download_csv(
        &self,
        resource: &Resource,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        let mut headers = Self::headers();
        headers.extend(options.conditional.to_headers()?);
        let response = self
            .client
            .get(&resource.url)
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
                url: resource.url.clone(),
            });
        }
        let bytes = response.bytes().await?;
        let frame = csv_to_frame(&bytes)?;
        Ok(FetchResult::fetched(frame, etag, last_modified))
    }
}

#[async_trait]
impl DatasetProvider for DataGovAuProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "data_gov_au".to_string(),
            name: "Data.gov.au".to_string(),
            description: Some("Australian Government CKAN open-data API".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        let _ = self.package().await?;
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        let package = self.package().await?;
        let resource = package.result.resources.iter().find(|resource| {
            resource.format.eq_ignore_ascii_case("csv")
                && resource.name.starts_with("FOI requests, costs and charges")
        });
        Ok(Catalog {
            datasets: vec![DatasetMetadata {
                id: DATASET_ID.to_string(),
                name: package.result.title,
                description: Some("OAIC aggregate FOI requests, costs and charges".to_string()),
                version: resource.map(|item| item.name.clone()),
                source_url: resource.map(|item| item.url.clone()),
            }],
        })
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        if dataset_id != DATASET_ID {
            return Err(CoreError::NotFound(dataset_id.to_string()));
        }
        let resource = self.csv_resource().await?;
        self.download_csv(&resource, options).await
    }
}

#[derive(Debug, Deserialize)]
struct PackageResponse {
    result: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    title: String,
    #[serde(default)]
    resources: Vec<Resource>,
}

#[derive(Debug, Clone, Deserialize)]
struct Resource {
    name: String,
    format: String,
    url: String,
}

fn csv_to_frame(bytes: &[u8]) -> Result<DataFrame> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(Cursor::new(bytes));
    let headers = reader
        .headers()
        .map_err(|error| CoreError::TransformationError(error.to_string()))?
        .iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut columns = vec![Vec::<String>::new(); headers.len()];
    for row in reader.records() {
        let row = row.map_err(|error| CoreError::TransformationError(error.to_string()))?;
        for (index, value) in row.iter().enumerate().take(headers.len()) {
            columns[index].push(value.to_string());
        }
        for values in columns.iter_mut().skip(row.len()) {
            values.push(String::new());
        }
    }
    let row_count = columns.first().map_or(0, Vec::len);
    let series = headers
        .iter()
        .zip(columns)
        .map(|(name, values)| Series::new(name.into(), values).into())
        .collect();
    DataFrame::new(row_count, series)
        .map_err(|error| CoreError::TransformationError(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_official_csv_rows() {
        let frame = csv_to_frame(b"year,total\n2024,12\n2025,15\n").unwrap();
        assert_eq!(frame.height(), 2);
        assert_eq!(frame.width(), 2);
        assert_eq!(
            frame.column("year").unwrap().str().unwrap().get(1),
            Some("2025")
        );
    }

    #[test]
    fn metadata_uses_stable_provider_id() {
        assert_eq!(DataGovAuProvider::default().metadata().id, "data_gov_au");
    }
}
