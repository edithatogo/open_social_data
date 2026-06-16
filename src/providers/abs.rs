//! Australian Bureau of Statistics (ABS) SDMX API provider.
//!
//! Fetches dataflow listings and dataset records from the ABS SDMX-JSON
//! API. Supports conditional requests via ETag/Last-Modified headers.

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
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

const DEFAULT_BASE_URL: &str = "https://api.abs.gov.au";
const SDMX_DATA_ACCEPT: &str = "application/vnd.sdmx.data+json;version=1.0.0";
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
        format!(
            "{}/data/{}?detail=full&dimensionAtObservation=AllDimensions",
            self.base_url, dataset_id
        )
    }

    fn structure_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static(SDMX_STRUCTURE_ACCEPT));
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        headers
    }

    fn data_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static(SDMX_DATA_ACCEPT));
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
            .headers(Self::structure_headers())
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
        let mut headers = Self::data_headers();
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
        Ok(FetchResult::fetched(
            sdmx_json_to_frame(dataset_id, &payload)?,
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

fn sdmx_json_to_frame(dataset_id: &str, payload: &Value) -> Result<DataFrame> {
    let data = payload.get("data").unwrap_or(payload);
    let data_sets = data_sets(data)?;
    let dimensions = data
        .get("structure")
        .and_then(|structure| structure.get("dimensions"))
        .ok_or_else(|| {
            CoreError::TransformationError("ABS SDMX JSON missing structure.dimensions".to_string())
        })?;
    let series_dimensions = dimension_defs(dimensions, "series");
    let observation_dimensions = dimension_defs(dimensions, "observation");

    let mut builder = RecordBatchBuilder::new();
    for data_set in data_sets {
        if let Some(series_map) = data_set.get("series").and_then(Value::as_object) {
            for (series_key, series) in series_map {
                let series_values = decode_dimension_key(series_key, &series_dimensions)?;
                let observations = series
                    .get("observations")
                    .and_then(Value::as_object)
                    .ok_or_else(|| {
                        CoreError::TransformationError(format!(
                            "ABS SDMX series {series_key} missing observations"
                        ))
                    })?;

                for (observation_key, observation) in observations {
                    let mut record = base_abs_record(dataset_id);
                    for (name, value) in &series_values {
                        record = record.with(name, value);
                    }
                    for (name, value) in
                        decode_dimension_key(observation_key, &observation_dimensions)?
                    {
                        record = record.with(name, value);
                    }
                    record = record.with("OBS_VALUE", observation_value(observation));
                    builder.push(record);
                }
            }
        } else if let Some(observations) = data_set.get("observations").and_then(Value::as_object) {
            for (observation_key, observation) in observations {
                let mut record = base_abs_record(dataset_id);
                for (name, value) in decode_dimension_key(observation_key, &observation_dimensions)?
                {
                    record = record.with(name, value);
                }
                record = record.with("OBS_VALUE", observation_value(observation));
                builder.push(record);
            }
        }
    }

    builder.build()
}

fn data_sets(data: &Value) -> Result<&[Value]> {
    data.get("dataSets")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| CoreError::TransformationError("ABS SDMX JSON missing dataSets".to_string()))
}

fn dimension_defs<'a>(dimensions: &'a Value, group: &str) -> Vec<DimensionDef<'a>> {
    dimensions
        .get(group)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|dimension| DimensionDef {
            id: dimension
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("dimension"),
            values: dimension
                .get("values")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        })
        .collect()
}

fn decode_dimension_key(
    key: &str,
    dimensions: &[DimensionDef<'_>],
) -> Result<Vec<(String, String)>> {
    if dimensions.is_empty() {
        return Ok(Vec::new());
    }

    let indexes = key
        .split(':')
        .map(|part| {
            part.parse::<usize>().map_err(|error| {
                CoreError::TransformationError(format!(
                    "invalid ABS SDMX dimension key {key}: {error}"
                ))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut values = Vec::new();
    for (dimension, index) in dimensions.iter().zip(indexes) {
        let value = dimension.values.get(index).ok_or_else(|| {
            CoreError::TransformationError(format!(
                "ABS SDMX dimension {} index {} out of range",
                dimension.id, index
            ))
        })?;
        values.push((
            dimension.id.to_string(),
            value
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
        ));
    }
    Ok(values)
}

fn observation_value(observation: &Value) -> String {
    observation
        .as_array()
        .and_then(|values| values.first())
        .map(value_to_cell)
        .unwrap_or_else(|| value_to_cell(observation))
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

fn base_abs_record(dataset_id: &str) -> RawRecord {
    RawRecord::new()
        .with("provider", "abs")
        .with("dataset_id", dataset_id)
}

struct DimensionDef<'a> {
    id: &'a str,
    values: &'a [Value],
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
        assert!(completed.request.starts_with("GET /data/QBIS?"));
        assert!(completed.request.contains("if-none-match: \"cached-etag\""));
        assert!(
            completed
                .request
                .contains("if-modified-since: Tue, 20 Oct 2015 07:28:00 GMT")
        );
    }

    #[tokio::test]
    async fn fetch_parses_sdmx_json_observations_into_rows() {
        let body = serde_json::json!({
            "data": {
                "dataSets": [{
                    "series": {
                        "0:0": {
                            "observations": {
                                "0": [12.5],
                                "1": [13.0]
                            }
                        },
                        "1:0": {
                            "observations": {
                                "0": [20.0]
                            }
                        }
                    }
                }],
                "structure": {
                    "dimensions": {
                        "series": [
                            {
                                "id": "MEASURE",
                                "values": [
                                    {"id": "SALES"},
                                    {"id": "PROFIT"}
                                ]
                            },
                            {
                                "id": "REGION",
                                "values": [
                                    {"id": "AUS"}
                                ]
                            }
                        ],
                        "observation": [
                            {
                                "id": "TIME_PERIOD",
                                "values": [
                                    {"id": "2024-Q1"},
                                    {"id": "2024-Q2"}
                                ]
                            }
                        ]
                    }
                }
            }
        });
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nETag: \"abs-data\"\r\nContent-Length: {}\r\n\r\n{}",
            body.to_string().len(),
            body
        );
        let completed = complete_request(
            Box::leak(response.into_boxed_str()),
            |base_url| async move {
                let provider = AbsProvider::new(base_url);
                provider
                    .fetch_dataset_with_options("QBIS", FetchOptions::default())
                    .await
            },
        )
        .await;

        let frame = completed.output.unwrap().into_frame().unwrap();

        assert_eq!(frame.height(), 3);
        assert_eq!(
            frame.column("provider").unwrap().str().unwrap().get(0),
            Some("abs")
        );
        assert_eq!(
            frame.column("dataset_id").unwrap().str().unwrap().get(0),
            Some("QBIS")
        );
        assert_eq!(
            frame.column("MEASURE").unwrap().str().unwrap().get(2),
            Some("PROFIT")
        );
        assert_eq!(
            frame.column("TIME_PERIOD").unwrap().str().unwrap().get(1),
            Some("2024-Q2")
        );
        assert_eq!(
            frame.column("OBS_VALUE").unwrap().str().unwrap().get(0),
            Some("12.5")
        );
        assert!(
            completed
                .request
                .contains("accept: application/vnd.sdmx.data+json")
        );
    }
}
