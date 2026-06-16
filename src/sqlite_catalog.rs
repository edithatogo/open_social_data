//! SQLite-backed local dataset catalog.
//!
//! Stores the same cached metadata as [`crate::catalog::LocalCatalog`] in an
//! embedded SQLite database. The JSON catalog remains useful for simple file
//! exchange, while this backend supports local cataloging workflows that need
//! a queryable database file.

use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension, params};

use crate::catalog::{CachedDataset, LocalCatalog, QualityStatus};
use crate::error::{CoreError, Result};

pub struct SqliteCatalog {
    conn: Connection,
}

impl SqliteCatalog {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        if let Some(parent) = path
            .as_ref()
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        let catalog = Self { conn };
        catalog.init_schema()?;
        Ok(catalog)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let catalog = Self { conn };
        catalog.init_schema()?;
        Ok(catalog)
    }

    pub fn load(&self) -> Result<LocalCatalog> {
        let mut stmt = self.conn.prepare(
            "SELECT provider,
                    dataset_id,
                    name,
                    version,
                    catalog_synced_at,
                    last_fetched_at,
                    last_not_modified_at,
                    source_url,
                    etag,
                    last_modified,
                    output_path,
                    quality_status,
                    quality_report_path,
                    row_count,
                    not_modified_count
             FROM datasets
             ORDER BY provider, dataset_id",
        )?;
        let rows = stmt.query_map([], row_to_cached_dataset)?;
        let mut catalog = LocalCatalog::default();
        for row in rows {
            catalog.upsert(row?);
        }
        Ok(catalog)
    }

    pub fn save_catalog(&mut self, catalog: &LocalCatalog) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM datasets", [])?;
        {
            let mut stmt = tx.prepare(UPSERT_DATASET_SQL)?;
            for dataset in catalog.datasets.values() {
                execute_upsert(&mut stmt, dataset)?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn upsert(&mut self, dataset: CachedDataset) -> Result<()> {
        let mut catalog = self.load()?;
        catalog.upsert(dataset);
        self.save_catalog(&catalog)
    }

    pub fn upsert_metadata(&mut self, dataset: CachedDataset) -> Result<()> {
        let mut catalog = self.load()?;
        catalog.upsert_metadata(dataset);
        self.save_catalog(&catalog)
    }

    pub fn upsert_fetch_result(&mut self, dataset: CachedDataset) -> Result<()> {
        let mut catalog = self.load()?;
        catalog.upsert_fetch_result(dataset);
        self.save_catalog(&catalog)
    }

    pub fn mark_not_modified(
        &mut self,
        provider: impl Into<String>,
        dataset_id: impl Into<String>,
        timestamp: impl Into<String>,
        etag: Option<String>,
        last_modified: Option<String>,
    ) -> Result<()> {
        let mut catalog = self.load()?;
        catalog.mark_not_modified(provider, dataset_id, timestamp, etag, last_modified);
        self.save_catalog(&catalog)
    }

    pub fn get(&self, provider: &str, dataset_id: &str) -> Result<Option<CachedDataset>> {
        self.load()
            .map(|catalog| catalog.get(provider, dataset_id).cloned())
    }

    pub fn list(&self, provider: Option<&str>) -> Result<Vec<CachedDataset>> {
        let catalog = self.load()?;
        Ok(catalog.list(provider).into_iter().cloned().collect())
    }

    pub fn search(&self, query: &str, provider: Option<&str>) -> Result<Vec<CachedDataset>> {
        let catalog = self.load()?;
        Ok(catalog
            .search(query, provider)
            .into_iter()
            .cloned()
            .collect())
    }

    pub fn len(&self) -> Result<usize> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM datasets", [], |row| {
                row.get::<_, i64>(0)
            })? as usize)
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS datasets (
                provider TEXT NOT NULL,
                dataset_id TEXT NOT NULL,
                name TEXT,
                version TEXT,
                catalog_synced_at TEXT,
                last_fetched_at TEXT,
                last_not_modified_at TEXT,
                source_url TEXT,
                etag TEXT,
                last_modified TEXT,
                output_path TEXT,
                quality_status TEXT,
                quality_report_path TEXT,
                row_count INTEGER,
                not_modified_count INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (provider, dataset_id)
            );
            CREATE INDEX IF NOT EXISTS idx_datasets_provider ON datasets(provider);
            CREATE INDEX IF NOT EXISTS idx_datasets_name ON datasets(name);
            CREATE INDEX IF NOT EXISTS idx_datasets_dataset_id ON datasets(dataset_id);",
        )?;
        Ok(())
    }
}

const UPSERT_DATASET_SQL: &str = "INSERT INTO datasets (
        provider,
        dataset_id,
        name,
        version,
        catalog_synced_at,
        last_fetched_at,
        last_not_modified_at,
        source_url,
        etag,
        last_modified,
        output_path,
        quality_status,
        quality_report_path,
        row_count,
        not_modified_count
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
    ON CONFLICT(provider, dataset_id) DO UPDATE SET
        name = excluded.name,
        version = excluded.version,
        catalog_synced_at = excluded.catalog_synced_at,
        last_fetched_at = excluded.last_fetched_at,
        last_not_modified_at = excluded.last_not_modified_at,
        source_url = excluded.source_url,
        etag = excluded.etag,
        last_modified = excluded.last_modified,
        output_path = excluded.output_path,
        quality_status = excluded.quality_status,
        quality_report_path = excluded.quality_report_path,
        row_count = excluded.row_count,
        not_modified_count = excluded.not_modified_count";

fn execute_upsert(stmt: &mut rusqlite::Statement<'_>, dataset: &CachedDataset) -> Result<()> {
    let output_path = path_to_string(dataset.output_path.as_deref());
    let quality_status = dataset.quality_status.map(quality_status_to_str);
    let quality_report_path = path_to_string(dataset.quality_report_path.as_deref());
    let row_count = dataset.row_count.map(|count| count as i64);
    let not_modified_count = dataset.not_modified_count as i64;

    stmt.execute(params![
        dataset.provider,
        dataset.dataset_id,
        dataset.name,
        dataset.version,
        dataset.catalog_synced_at,
        dataset.last_fetched_at,
        dataset.last_not_modified_at,
        dataset.source_url,
        dataset.etag,
        dataset.last_modified,
        output_path,
        quality_status,
        quality_report_path,
        row_count,
        not_modified_count,
    ])?;
    Ok(())
}

fn row_to_cached_dataset(row: &rusqlite::Row<'_>) -> rusqlite::Result<CachedDataset> {
    let quality_status: Option<String> = row.get(11)?;
    let row_count: Option<i64> = row.get(13)?;
    let not_modified_count: i64 = row.get(14)?;

    Ok(CachedDataset {
        provider: row.get(0)?,
        dataset_id: row.get(1)?,
        name: row.get(2)?,
        version: row.get(3)?,
        catalog_synced_at: row.get(4)?,
        last_fetched_at: row.get(5)?,
        last_not_modified_at: row.get(6)?,
        source_url: row.get(7)?,
        etag: row.get(8)?,
        last_modified: row.get(9)?,
        output_path: optional_path(row.get::<_, Option<String>>(10)?),
        quality_status: quality_status.as_deref().and_then(str_to_quality_status),
        quality_report_path: optional_path(row.get::<_, Option<String>>(12)?),
        row_count: row_count.map(|count| count.max(0) as usize),
        not_modified_count: not_modified_count.max(0) as usize,
    })
}

fn path_to_string(path: Option<&Path>) -> Option<String> {
    path.map(|path| path.to_string_lossy().into_owned())
}

fn optional_path(path: Option<String>) -> Option<PathBuf> {
    path.map(PathBuf::from)
}

fn quality_status_to_str(status: QualityStatus) -> &'static str {
    match status {
        QualityStatus::Passed => "passed",
        QualityStatus::Failed => "failed",
        QualityStatus::NotRun => "not_run",
    }
}

fn str_to_quality_status(status: &str) -> Option<QualityStatus> {
    match status {
        "passed" => Some(QualityStatus::Passed),
        "failed" => Some(QualityStatus::Failed),
        "not_run" => Some(QualityStatus::NotRun),
        _ => None,
    }
}

pub fn load_sqlite_catalog(path: impl AsRef<Path>) -> Result<LocalCatalog> {
    SqliteCatalog::open(path)?.load()
}

pub fn save_sqlite_catalog(path: impl AsRef<Path>, catalog: &LocalCatalog) -> Result<()> {
    let mut sqlite = SqliteCatalog::open(path)?;
    sqlite.save_catalog(catalog)
}

pub fn sqlite_catalog_contains(
    path: impl AsRef<Path>,
    provider: &str,
    dataset_id: &str,
) -> Result<bool> {
    let sqlite = SqliteCatalog::open(path)?;
    sqlite
        .conn
        .query_row(
            "SELECT 1 FROM datasets WHERE provider = ?1 AND dataset_id = ?2",
            params![provider, dataset_id],
            |_| Ok(()),
        )
        .optional()
        .map(|row| row.is_some())
        .map_err(CoreError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DatasetMetadata;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn persists_and_loads_cached_dataset_metadata() {
        let path = unique_catalog_path("persist");
        let mut sqlite = SqliteCatalog::open(&path).unwrap();
        sqlite
            .upsert(CachedDataset {
                provider: "abs".to_string(),
                dataset_id: "QBIS".to_string(),
                name: Some("Business indicators".to_string()),
                version: Some("1.0".to_string()),
                catalog_synced_at: Some("100".to_string()),
                last_fetched_at: Some("200".to_string()),
                last_not_modified_at: None,
                source_url: Some("https://example.test/qbis".to_string()),
                etag: Some("\"abc\"".to_string()),
                last_modified: Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string()),
                output_path: Some(PathBuf::from("qbis.parquet")),
                quality_status: Some(QualityStatus::Passed),
                quality_report_path: Some(PathBuf::from("qbis-quality.json")),
                row_count: Some(42),
                not_modified_count: 2,
            })
            .unwrap();
        drop(sqlite);

        let loaded = SqliteCatalog::open(&path).unwrap().load().unwrap();
        let _ = std::fs::remove_file(&path);

        let dataset = loaded.get("abs", "QBIS").unwrap();
        assert_eq!(dataset.name.as_deref(), Some("Business indicators"));
        assert_eq!(dataset.row_count, Some(42));
        assert_eq!(dataset.quality_status, Some(QualityStatus::Passed));
        assert_eq!(dataset.not_modified_count, 2);
    }

    #[test]
    fn metadata_upsert_preserves_fetch_outputs() {
        let mut sqlite = SqliteCatalog::open_in_memory().unwrap();
        sqlite
            .upsert(CachedDataset {
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
                quality_report_path: None,
                row_count: Some(1),
                not_modified_count: 0,
            })
            .unwrap();

        sqlite
            .upsert_metadata(CachedDataset::from_dataset_metadata(
                "abs",
                DatasetMetadata {
                    id: "QBIS".to_string(),
                    name: "Business indicators".to_string(),
                    description: None,
                    version: Some("2.0".to_string()),
                    source_url: None,
                },
                "200",
            ))
            .unwrap();

        let dataset = sqlite.get("abs", "QBIS").unwrap().unwrap();
        assert_eq!(dataset.name.as_deref(), Some("Business indicators"));
        assert_eq!(
            dataset.output_path.as_deref(),
            Some(Path::new("qbis.parquet"))
        );
        assert_eq!(dataset.quality_status, Some(QualityStatus::Passed));
    }

    #[test]
    fn sqlite_search_matches_local_catalog_semantics() {
        let mut sqlite = SqliteCatalog::open_in_memory().unwrap();
        sqlite
            .upsert(CachedDataset::from_dataset_metadata(
                "stats_nz",
                DatasetMetadata {
                    id: "CPI".to_string(),
                    name: "Consumer price index".to_string(),
                    description: None,
                    version: None,
                    source_url: Some("https://example.test/cpi".to_string()),
                },
                "100",
            ))
            .unwrap();

        assert_eq!(
            sqlite.search("consumer", Some("stats_nz")).unwrap().len(),
            1
        );
        assert_eq!(sqlite.search("consumer", Some("abs")).unwrap().len(), 0);
    }

    #[test]
    fn tracks_not_modified_events() {
        let mut sqlite = SqliteCatalog::open_in_memory().unwrap();
        sqlite
            .mark_not_modified("abs", "QBIS", "400", Some("\"new\"".to_string()), None)
            .unwrap();

        let dataset = sqlite.get("abs", "QBIS").unwrap().unwrap();
        assert_eq!(dataset.last_not_modified_at.as_deref(), Some("400"));
        assert_eq!(dataset.etag.as_deref(), Some("\"new\""));
        assert_eq!(dataset.not_modified_count, 1);
    }

    fn unique_catalog_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        std::env::temp_dir().join(format!(
            "open-social-data-{name}-{}-{unique}.sqlite",
            std::process::id()
        ))
    }
}
