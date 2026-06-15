use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::hardening::ConditionalRequestMetadata;
use crate::models::DatasetMetadata;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CachedDataset {
    pub provider: String,
    pub dataset_id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub catalog_synced_at: Option<String>,
    pub last_fetched_at: Option<String>,
    pub last_not_modified_at: Option<String>,
    pub source_url: Option<String>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub output_path: Option<PathBuf>,
    pub quality_status: Option<QualityStatus>,
    pub quality_report_path: Option<PathBuf>,
    pub row_count: Option<usize>,
    pub not_modified_count: usize,
}

impl CachedDataset {
    pub fn key(&self) -> String {
        dataset_key(&self.provider, &self.dataset_id)
    }

    pub fn mark_not_modified(&mut self, timestamp: impl Into<String>) {
        self.last_not_modified_at = Some(timestamp.into());
        self.not_modified_count = self.not_modified_count.saturating_add(1);
    }

    pub fn with_quality(mut self, status: QualityStatus, report_path: Option<PathBuf>) -> Self {
        self.quality_status = Some(status);
        self.quality_report_path = report_path;
        self
    }

    pub fn conditional_request_metadata(&self) -> ConditionalRequestMetadata {
        ConditionalRequestMetadata::new(self.etag.clone(), self.last_modified.clone())
    }

    pub fn from_dataset_metadata(
        provider: impl Into<String>,
        dataset: DatasetMetadata,
        catalog_synced_at: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            dataset_id: dataset.id,
            name: Some(dataset.name),
            version: dataset.version,
            catalog_synced_at: Some(catalog_synced_at.into()),
            last_fetched_at: None,
            last_not_modified_at: None,
            source_url: dataset.source_url,
            etag: None,
            last_modified: None,
            output_path: None,
            quality_status: Some(QualityStatus::NotRun),
            quality_report_path: None,
            row_count: None,
            not_modified_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityStatus {
    Passed,
    Failed,
    NotRun,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct LocalCatalog {
    pub datasets: BTreeMap<String, CachedDataset>,
}

impl LocalCatalog {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }

        let reader = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn save_atomic(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp_path = tmp_path_for(path);
        if tmp_path.exists() {
            fs::remove_file(&tmp_path)?;
        }

        let writer = BufWriter::new(File::create(&tmp_path)?);
        serde_json::to_writer_pretty(writer, self)?;

        if path.exists() {
            fs::remove_file(path)?;
        }
        fs::rename(tmp_path, path)?;
        Ok(())
    }

    pub fn upsert(&mut self, dataset: CachedDataset) {
        self.datasets.insert(dataset.key(), dataset);
    }

    pub fn upsert_metadata(&mut self, dataset: CachedDataset) {
        let key = dataset.key();
        if let Some(existing) = self.datasets.get_mut(&key) {
            existing.name = dataset.name;
            existing.version = dataset.version;
            existing.catalog_synced_at = dataset.catalog_synced_at;
            if dataset.source_url.is_some() {
                existing.source_url = dataset.source_url;
            }
            if existing.quality_status.is_none() {
                existing.quality_status = dataset.quality_status;
            }
        } else {
            self.datasets.insert(key, dataset);
        }
    }

    pub fn upsert_fetch_result(&mut self, dataset: CachedDataset) {
        let key = dataset.key();
        if let Some(existing) = self.datasets.get_mut(&key) {
            existing.last_fetched_at = dataset.last_fetched_at;
            existing.last_not_modified_at = None;
            existing.output_path = dataset.output_path;
            existing.quality_status = dataset.quality_status;
            existing.quality_report_path = dataset.quality_report_path;
            existing.row_count = dataset.row_count;
            if dataset.source_url.is_some() {
                existing.source_url = dataset.source_url;
            }
            if dataset.etag.is_some() {
                existing.etag = dataset.etag;
            }
            if dataset.last_modified.is_some() {
                existing.last_modified = dataset.last_modified;
            }
        } else {
            self.datasets.insert(key, dataset);
        }
    }

    pub fn mark_not_modified(
        &mut self,
        provider: impl Into<String>,
        dataset_id: impl Into<String>,
        timestamp: impl Into<String>,
        etag: Option<String>,
        last_modified: Option<String>,
    ) {
        let provider = provider.into();
        let dataset_id = dataset_id.into();
        let key = dataset_key(&provider, &dataset_id);
        if let Some(existing) = self.datasets.get_mut(&key) {
            existing.mark_not_modified(timestamp);
            if etag.is_some() {
                existing.etag = etag;
            }
            if last_modified.is_some() {
                existing.last_modified = last_modified;
            }
        } else {
            let mut dataset = CachedDataset {
                provider,
                dataset_id,
                name: None,
                version: None,
                catalog_synced_at: None,
                last_fetched_at: None,
                last_not_modified_at: None,
                source_url: None,
                etag,
                last_modified,
                output_path: None,
                quality_status: Some(QualityStatus::NotRun),
                quality_report_path: None,
                row_count: None,
                not_modified_count: 0,
            };
            dataset.mark_not_modified(timestamp);
            self.datasets.insert(dataset.key(), dataset);
        }
    }

    pub fn get(&self, provider: &str, dataset_id: &str) -> Option<&CachedDataset> {
        self.datasets.get(&dataset_key(provider, dataset_id))
    }

    pub fn len(&self) -> usize {
        self.datasets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.datasets.is_empty()
    }

    pub fn list(&self, provider: Option<&str>) -> Vec<&CachedDataset> {
        self.datasets
            .values()
            .filter(|dataset| provider.is_none_or(|provider| dataset.provider == provider))
            .collect()
    }

    pub fn search(&self, query: &str, provider: Option<&str>) -> Vec<&CachedDataset> {
        let query = query.to_ascii_lowercase();
        self.datasets
            .values()
            .filter(|dataset| provider.is_none_or(|provider| dataset.provider == provider))
            .filter(|dataset| dataset.matches_query(&query))
            .collect()
    }
}

impl CachedDataset {
    fn matches_query(&self, query: &str) -> bool {
        self.provider.to_ascii_lowercase().contains(query)
            || self.dataset_id.to_ascii_lowercase().contains(query)
            || self
                .name
                .as_ref()
                .map(|name| name.to_ascii_lowercase().contains(query))
                .unwrap_or(false)
    }
}

pub fn dataset_key(provider: &str, dataset_id: &str) -> String {
    format!("{}:{}", provider.trim(), dataset_id.trim())
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|file_name| file_name.to_os_string())
        .unwrap_or_else(|| "catalog.json".into());
    name.push(".tmp");
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upserts_by_provider_dataset_key() {
        let mut catalog = LocalCatalog::default();
        catalog.upsert(CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: Some("Business indicators".to_string()),
            version: Some("1.0".to_string()),
            catalog_synced_at: None,
            last_fetched_at: None,
            last_not_modified_at: None,
            source_url: None,
            etag: None,
            last_modified: None,
            output_path: None,
            quality_status: None,
            quality_report_path: None,
            row_count: Some(3),
            not_modified_count: 0,
        });

        assert_eq!(catalog.len(), 1);
        assert_eq!(
            catalog.get("abs", "QBIS").and_then(|item| item.row_count),
            Some(3)
        );
    }

    #[test]
    fn searches_by_dataset_id_and_provider() {
        let mut catalog = LocalCatalog::default();
        catalog.upsert(CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: Some("Business indicators".to_string()),
            version: None,
            catalog_synced_at: None,
            last_fetched_at: None,
            last_not_modified_at: None,
            source_url: None,
            etag: None,
            last_modified: None,
            output_path: None,
            quality_status: None,
            quality_report_path: None,
            row_count: None,
            not_modified_count: 0,
        });

        assert_eq!(catalog.search("qbis", Some("abs")).len(), 1);
        assert_eq!(catalog.search("qbis", Some("stats_nz")).len(), 0);
    }

    #[test]
    fn tracks_not_modified_events() {
        let mut dataset = CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: None,
            version: None,
            catalog_synced_at: None,
            last_fetched_at: None,
            last_not_modified_at: None,
            source_url: None,
            etag: Some("abc".to_string()),
            last_modified: None,
            output_path: None,
            quality_status: Some(QualityStatus::NotRun),
            quality_report_path: None,
            row_count: None,
            not_modified_count: 0,
        };

        dataset.mark_not_modified("123");

        assert_eq!(dataset.last_not_modified_at.as_deref(), Some("123"));
        assert_eq!(dataset.not_modified_count, 1);
    }

    #[test]
    fn builds_conditional_request_metadata() {
        let dataset = CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: None,
            version: None,
            catalog_synced_at: None,
            last_fetched_at: None,
            last_not_modified_at: None,
            source_url: None,
            etag: Some("\"abc\"".to_string()),
            last_modified: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
            output_path: None,
            quality_status: Some(QualityStatus::NotRun),
            quality_report_path: None,
            row_count: None,
            not_modified_count: 0,
        };

        let metadata = dataset.conditional_request_metadata();

        assert_eq!(metadata.etag.as_deref(), Some("\"abc\""));
        assert_eq!(
            metadata.last_modified.as_deref(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT")
        );
    }

    #[test]
    fn metadata_upsert_preserves_fetch_outputs() {
        let mut catalog = LocalCatalog::default();
        catalog.upsert(CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: None,
            version: None,
            catalog_synced_at: None,
            last_fetched_at: Some("100".to_string()),
            last_not_modified_at: None,
            source_url: Some("https://example.test/qbis".to_string()),
            etag: None,
            last_modified: None,
            output_path: Some(PathBuf::from("qbis.parquet")),
            quality_status: Some(QualityStatus::Passed),
            quality_report_path: Some(PathBuf::from("qbis-quality.json")),
            row_count: Some(1),
            not_modified_count: 0,
        });

        catalog.upsert_metadata(CachedDataset::from_dataset_metadata(
            "abs",
            DatasetMetadata {
                id: "QBIS".to_string(),
                name: "Business indicators".to_string(),
                description: None,
                version: Some("2.0".to_string()),
                source_url: None,
            },
            "200",
        ));

        let dataset = catalog.get("abs", "QBIS").unwrap();
        assert_eq!(dataset.name.as_deref(), Some("Business indicators"));
        assert_eq!(
            dataset.output_path.as_deref(),
            Some(Path::new("qbis.parquet"))
        );
        assert_eq!(
            dataset.source_url.as_deref(),
            Some("https://example.test/qbis")
        );
        assert_eq!(dataset.quality_status, Some(QualityStatus::Passed));
    }

    #[test]
    fn fetch_result_upsert_preserves_catalog_metadata() {
        let mut catalog = LocalCatalog::default();
        catalog.upsert_metadata(CachedDataset::from_dataset_metadata(
            "abs",
            DatasetMetadata {
                id: "QBIS".to_string(),
                name: "Business indicators".to_string(),
                description: None,
                version: Some("2.0".to_string()),
                source_url: Some("https://example.test/qbis".to_string()),
            },
            "200",
        ));

        catalog.upsert_fetch_result(CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: None,
            version: None,
            catalog_synced_at: None,
            last_fetched_at: Some("300".to_string()),
            last_not_modified_at: None,
            source_url: None,
            etag: None,
            last_modified: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
            output_path: Some(PathBuf::from("qbis.parquet")),
            quality_status: Some(QualityStatus::Passed),
            quality_report_path: None,
            row_count: Some(1),
            not_modified_count: 0,
        });

        let dataset = catalog.get("abs", "QBIS").unwrap();
        assert_eq!(
            dataset.source_url.as_deref(),
            Some("https://example.test/qbis")
        );
        assert_eq!(dataset.catalog_synced_at.as_deref(), Some("200"));
        assert_eq!(dataset.last_fetched_at.as_deref(), Some("300"));
        assert_eq!(dataset.last_not_modified_at, None);
        assert_eq!(
            dataset.last_modified.as_deref(),
            Some("Wed, 21 Oct 2015 07:28:00 GMT")
        );
    }

    #[test]
    fn mark_not_modified_preserves_existing_fetch_metadata() {
        let mut catalog = LocalCatalog::default();
        catalog.upsert(CachedDataset {
            provider: "abs".to_string(),
            dataset_id: "QBIS".to_string(),
            name: None,
            version: None,
            catalog_synced_at: None,
            last_fetched_at: Some("100".to_string()),
            last_not_modified_at: None,
            source_url: Some("https://example.test/qbis".to_string()),
            etag: Some("\"old\"".to_string()),
            last_modified: None,
            output_path: Some(PathBuf::from("qbis.parquet")),
            quality_status: Some(QualityStatus::Passed),
            quality_report_path: None,
            row_count: Some(1),
            not_modified_count: 0,
        });

        catalog.mark_not_modified("abs", "QBIS", "400", Some("\"new\"".to_string()), None);

        let dataset = catalog.get("abs", "QBIS").unwrap();
        assert_eq!(dataset.last_not_modified_at.as_deref(), Some("400"));
        assert_eq!(dataset.etag.as_deref(), Some("\"new\""));
        assert_eq!(
            dataset.output_path.as_deref(),
            Some(Path::new("qbis.parquet"))
        );
        assert_eq!(
            dataset.source_url.as_deref(),
            Some("https://example.test/qbis")
        );
        assert_eq!(dataset.not_modified_count, 1);
    }
}
