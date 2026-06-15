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

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockProvider;

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
        assert!(catalog.is_empty());
    }
}
