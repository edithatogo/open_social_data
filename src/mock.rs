use crate::error::{CoreError, Result};
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};
use async_trait::async_trait;
use polars::prelude::*;

pub struct MockProvider;

#[async_trait]
impl DatasetProvider for MockProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "mock".to_string(),
            name: "Mock Provider".to_string(),
            description: Some("A mock provider for testing purposes".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        Ok(Catalog {
            datasets: vec![
                DatasetMetadata {
                    id: "test-dataset-1".to_string(),
                    name: "Test Dataset 1".to_string(),
                    description: Some("First test dataset".to_string()),
                    version: Some("1.0.0".to_string()),
                    source_url: Some("mock://test-dataset-1".to_string()),
                },
                DatasetMetadata {
                    id: "test-dataset-2".to_string(),
                    name: "Test Dataset 2".to_string(),
                    description: Some("Second test dataset".to_string()),
                    version: Some("2.1.0".to_string()),
                    source_url: Some("mock://test-dataset-2".to_string()),
                },
            ],
        })
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        _options: FetchOptions,
    ) -> Result<FetchResult> {
        match dataset_id {
            "test-dataset-1" => {
                let s0 = Series::new("id".into(), &[1, 2, 3]);
                let s1 = Series::new("name".into(), &["A", "B", "C"]);
                let df = DataFrame::new(vec![s0.into(), s1.into()])
                    .map_err(|e| CoreError::Internal(e.to_string()))?;
                Ok(FetchResult::from_frame(df))
            }
            _ => Err(CoreError::NotFound(dataset_id.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider_metadata() {
        let provider = MockProvider;
        let meta = provider.metadata();
        assert_eq!(meta.name, "Mock Provider");
    }

    #[tokio::test]
    async fn test_mock_provider_list() {
        let provider = MockProvider;
        let catalog = provider.list_datasets().await.unwrap();
        assert_eq!(catalog.datasets.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_provider_fetch() {
        let provider = MockProvider;
        let df = provider.fetch_dataset("test-dataset-1").await.unwrap();
        assert_eq!(df.width(), 2);
        assert_eq!(df.height(), 3);
    }
}
