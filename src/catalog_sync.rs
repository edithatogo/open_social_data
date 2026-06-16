use std::path::Path;

use crate::catalog::{CachedDataset, LocalCatalog};
use crate::error::Result;
use crate::registry::ProviderRegistry;
use crate::sqlite_catalog::SqliteCatalog;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSyncError {
    pub provider: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogSyncReport {
    pub synced_records: usize,
    pub synced_providers: Vec<String>,
    pub errors: Vec<CatalogSyncError>,
    pub partial_success: bool,
}

impl CatalogSyncReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_summary(&self) -> String {
        self.errors
            .iter()
            .map(|error| format!("{}: {}", error.provider, error.message))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

pub async fn sync_catalog_from_registry(
    catalog: &mut LocalCatalog,
    registry: &ProviderRegistry,
    provider_filter: Option<&str>,
    timestamp: impl Into<String>,
) -> CatalogSyncReport {
    let timestamp = timestamp.into();
    let provider_names = match provider_filter {
        Some(provider) => vec![provider.to_string()],
        None => registry
            .names()
            .into_iter()
            .map(ToString::to_string)
            .collect(),
    };

    let mut report = CatalogSyncReport::default();

    for provider_name in provider_names {
        let provider = match registry.get(&provider_name) {
            Ok(provider) => provider,
            Err(error) => {
                report.errors.push(CatalogSyncError {
                    provider: provider_name,
                    message: error.to_string(),
                });
                continue;
            }
        };

        match provider.list_datasets().await {
            Ok(remote_catalog) => {
                for dataset in remote_catalog.datasets {
                    catalog.upsert_metadata(CachedDataset::from_dataset_metadata(
                        provider_name.clone(),
                        dataset,
                        timestamp.clone(),
                    ));
                    report.synced_records += 1;
                }
                report.synced_providers.push(provider_name);
            }
            Err(error) => {
                report.errors.push(CatalogSyncError {
                    provider: provider_name,
                    message: error.to_string(),
                });
            }
        }
    }

    report.partial_success = report.synced_records > 0 && report.has_errors();
    report
}

pub async fn sync_catalog_path_from_registry(
    path: impl AsRef<Path>,
    registry: &ProviderRegistry,
    provider_filter: Option<&str>,
    timestamp: impl Into<String>,
) -> Result<CatalogSyncReport> {
    let path = path.as_ref();
    let mut catalog = LocalCatalog::load(path)?;
    let report =
        sync_catalog_from_registry(&mut catalog, registry, provider_filter, timestamp).await;
    catalog.save_atomic(path)?;
    Ok(report)
}

pub async fn sync_sqlite_catalog_path_from_registry(
    path: impl AsRef<Path>,
    registry: &ProviderRegistry,
    provider_filter: Option<&str>,
    timestamp: impl Into<String>,
) -> Result<CatalogSyncReport> {
    let mut sqlite_catalog = SqliteCatalog::open(path)?;
    let mut catalog = sqlite_catalog.load()?;
    let report =
        sync_catalog_from_registry(&mut catalog, registry, provider_filter, timestamp).await;
    sqlite_catalog.save_catalog(&catalog)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CoreError;
    use crate::mock::MockProvider;
    use crate::models::{Catalog, ProviderMetadata};
    use crate::traits::{DatasetProvider, FetchOptions, FetchResult};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct FailingProvider;

    #[async_trait::async_trait]
    impl DatasetProvider for FailingProvider {
        fn metadata(&self) -> ProviderMetadata {
            ProviderMetadata {
                id: "z-failing".to_string(),
                name: "Failing provider".to_string(),
                description: Some("Provider fixture that fails dataset listing".to_string()),
            }
        }

        async fn ping(&self) -> Result<()> {
            Ok(())
        }

        async fn list_datasets(&self) -> Result<Catalog> {
            Err(CoreError::Internal("listing failed".to_string()))
        }

        async fn fetch_dataset_with_options(
            &self,
            _dataset_id: &str,
            _options: FetchOptions,
        ) -> Result<FetchResult> {
            Err(CoreError::Internal("fetch failed".to_string()))
        }
    }

    #[tokio::test]
    async fn syncs_mock_provider_metadata() {
        let mut registry = ProviderRegistry::new();
        registry.register(MockProvider);
        let mut catalog = LocalCatalog::default();

        let report = sync_catalog_from_registry(&mut catalog, &registry, Some("mock"), "123").await;

        assert_eq!(report.synced_records, 2);
        assert_eq!(report.synced_providers, vec!["mock"]);
        assert!(report.errors.is_empty());
        assert_eq!(catalog.len(), 2);
        assert_eq!(
            catalog
                .get("mock", "test-dataset-1")
                .and_then(|dataset| dataset.source_url.as_deref()),
            Some("mock://test-dataset-1")
        );
    }

    #[tokio::test]
    async fn reports_missing_provider_without_mutating_catalog() {
        let registry = ProviderRegistry::new();
        let mut catalog = LocalCatalog::default();

        let report =
            sync_catalog_from_registry(&mut catalog, &registry, Some("missing"), "123").await;

        assert_eq!(report.synced_records, 0);
        assert_eq!(report.errors.len(), 1);
        assert!(!report.partial_success);
        assert!(catalog.is_empty());
    }

    #[tokio::test]
    async fn reports_partial_success_when_some_metadata_persisted_before_errors() {
        let mut registry = ProviderRegistry::new();
        registry.register(MockProvider);
        registry.register(FailingProvider);
        let mut catalog = LocalCatalog::default();

        let report = sync_catalog_from_registry(&mut catalog, &registry, None, "123").await;

        assert_eq!(report.synced_records, 2);
        assert_eq!(report.synced_providers, vec!["mock"]);
        assert_eq!(report.errors.len(), 1);
        assert!(report.partial_success);
        assert_eq!(catalog.len(), 2);
    }

    #[tokio::test]
    async fn path_sync_saves_partial_success_before_returning_report() {
        let mut registry = ProviderRegistry::new();
        registry.register(MockProvider);
        registry.register(FailingProvider);
        let path = unique_catalog_path("partial-success");

        let report = sync_catalog_path_from_registry(&path, &registry, None, "123")
            .await
            .unwrap();
        let saved = LocalCatalog::load(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(report.partial_success);
        assert_eq!(saved.len(), 2);
        assert!(saved.get("mock", "test-dataset-1").is_some());
    }

    #[tokio::test]
    async fn sqlite_path_sync_saves_partial_success_before_returning_report() {
        let mut registry = ProviderRegistry::new();
        registry.register(MockProvider);
        registry.register(FailingProvider);
        let path = unique_sqlite_catalog_path("partial-success");

        let report = sync_sqlite_catalog_path_from_registry(&path, &registry, None, "123")
            .await
            .unwrap();
        let saved = SqliteCatalog::open(&path).unwrap().load().unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(report.partial_success);
        assert_eq!(saved.len(), 2);
        assert!(saved.get("mock", "test-dataset-1").is_some());
    }

    fn unique_catalog_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        std::env::temp_dir().join(format!(
            "open-social-data-{name}-{}-{unique}.json",
            std::process::id()
        ))
    }

    fn unique_sqlite_catalog_path(name: &str) -> PathBuf {
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
