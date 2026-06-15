use crate::catalog::{CachedDataset, LocalCatalog};
use crate::registry::ProviderRegistry;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CoreError, Result};
    use crate::mock::MockProvider;
    use crate::models::{Catalog, ProviderMetadata};
    use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

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
            Err(CoreError::ApiError("listing failed".to_string()))
        }

        async fn fetch_dataset_with_options(
            &self,
            _dataset_id: &str,
            _options: FetchOptions,
        ) -> Result<FetchResult> {
            Err(CoreError::ApiError("fetch failed".to_string()))
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
}
