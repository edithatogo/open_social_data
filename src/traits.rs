use crate::error::Result;
use crate::hardening::ConditionalRequestMetadata;
use crate::models::{Catalog, ProviderMetadata};
use async_trait::async_trait;
use polars::prelude::DataFrame;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FetchOptions {
    pub conditional: ConditionalRequestMetadata,
}

impl FetchOptions {
    pub fn new(conditional: ConditionalRequestMetadata) -> Self {
        Self { conditional }
    }
}

#[derive(Debug, Clone)]
pub enum FetchResult {
    Fetched {
        frame: DataFrame,
        etag: Option<String>,
        last_modified: Option<String>,
    },
    NotModified {
        etag: Option<String>,
        last_modified: Option<String>,
    },
}

impl FetchResult {
    pub fn from_frame(frame: DataFrame) -> Self {
        Self::Fetched {
            frame,
            etag: None,
            last_modified: None,
        }
    }

    pub fn fetched(frame: DataFrame, etag: Option<String>, last_modified: Option<String>) -> Self {
        Self::Fetched {
            frame,
            etag,
            last_modified,
        }
    }

    pub fn not_modified(etag: Option<String>, last_modified: Option<String>) -> Self {
        Self::NotModified {
            etag,
            last_modified,
        }
    }

    pub fn is_not_modified(&self) -> bool {
        matches!(self, Self::NotModified { .. })
    }

    pub fn etag(&self) -> Option<&str> {
        match self {
            Self::Fetched { etag, .. } | Self::NotModified { etag, .. } => etag.as_deref(),
        }
    }

    pub fn last_modified(&self) -> Option<&str> {
        match self {
            Self::Fetched { last_modified, .. } | Self::NotModified { last_modified, .. } => {
                last_modified.as_deref()
            }
        }
    }

    pub fn into_frame(self) -> Option<DataFrame> {
        match self {
            Self::Fetched { frame, .. } => Some(frame),
            Self::NotModified { .. } => None,
        }
    }
}

#[async_trait]
pub trait DatasetProvider: Send + Sync {
    /// Returns metadata about the provider.
    fn metadata(&self) -> ProviderMetadata;

    /// Pings the provider to check connection health.
    async fn ping(&self) -> Result<()>;

    /// Lists available datasets from this provider.
    async fn list_datasets(&self) -> Result<Catalog>;

    /// Fetches a specific dataset by ID and returns it as a DataFrame.
    /// In a more advanced implementation, this could return a stream of DataFrames.
    async fn fetch_dataset(&self, dataset_id: &str) -> Result<DataFrame> {
        let result = self
            .fetch_dataset_with_options(dataset_id, FetchOptions::default())
            .await?;
        match result {
            FetchResult::Fetched { frame, .. } => Ok(frame),
            FetchResult::NotModified { .. } => Err(crate::error::CoreError::NotModified(format!(
                "{dataset_id} returned not modified without conditional request"
            ))),
        }
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        _options: FetchOptions,
    ) -> Result<FetchResult>;
}
