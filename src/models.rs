//! Standard data models for providers, datasets, and catalogs.
//!
//! These types are shared across all provider implementations and
//! serialized/deserialized for JSON-based local catalog persistence.

use serde::{Deserialize, Serialize};

/// Metadata describing a data source provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// Metadata describing a single dataset within a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub source_url: Option<String>,
}

/// A collection of datasets, typically returned by a provider's listing endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub datasets: Vec<DatasetMetadata>,
}