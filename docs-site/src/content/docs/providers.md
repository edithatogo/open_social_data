---
title: Provider Authoring Guide
description: How the DatasetProvider trait works, existing provider implementations, and how to add a new data source provider.
---

# Provider Authoring Guide

Providers are the pluggable data source adapters that connect the Open Social Data engine to national statistics agency APIs. Each provider implements the `DatasetProvider` trait.

## The `DatasetProvider` Trait

Every provider must implement this trait, defined in `src/traits.rs`:

```rust
#[async_trait]
pub trait DatasetProvider: Send + Sync {
    /// Returns metadata about the provider.
    fn metadata(&self) -> ProviderMetadata;

    /// Pings the provider to check connection health.
    async fn ping(&self) -> Result<()>;

    /// Lists available datasets from this provider.
    async fn list_datasets(&self) -> Result<Catalog>;

    /// Fetches a specific dataset by ID.
    async fn fetch_dataset(&self, dataset_id: &str) -> Result<DataFrame>;

    /// Fetches a dataset with conditional request options (ETag / Last-Modified).
    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult>;
}
```

### Key data types

| Type | Description |
|------|-------------|
| `ProviderMetadata` | Provider ID, name, description |
| `DatasetMetadata` | Dataset ID, name, description, version, source URL |
| `Catalog` | A collection of `DatasetMetadata` entries |
| `FetchOptions` | Carries conditional request metadata (ETag, `Last-Modified`) |
| `FetchResult` | Either `Fetched { frame, etag, last_modified }` or `NotModified { etag, last_modified }` |

## Existing Providers

### AbsProvider (SDMX-JSON)

**Module:** `src/providers/abs.rs`

Connects to the [ABS API](https://api.abs.gov.au) using SDMX-JSON 1.0.0 and SDMX-Structure 2.0.0.

### MockProvider (testing)

**Module:** `src/mock.rs`

A zero-dependency provider that returns synthetic data without network access.

- **Provider ID:** `mock`
- **Datasets:** `test-dataset-1` (3 rows, 2 columns) and `test-dataset-2`
- **Source URLs:** Uses `mock://` scheme for clear test resource identification
- **Uses:** Integration tests, catalog sync tests, and development without live endpoints

## ProviderRegistry

Providers are registered dynamically in the `ProviderRegistry`, defined in `src/registry.rs`:

```rust
let mut registry = ProviderRegistry::new();
registry.register(AbsProvider::default());
registry.register(StatsNzProvider::default());
```

The `with_defaults()` constructor registers both ABS and Stats NZ providers. Providers are stored as `Arc<dyn DatasetProvider>`, allowing shared access across threads.

```rust
// Look up a provider by name
let provider = registry.get("abs")?;

// List all registered provider names
let names: Vec<&str> = registry.names();
```

## Adding a New Provider

### Step 1: Create the module

Add a new file in `src/providers/`, e.g., `src/providers/eurostat.rs`:

```rust
use async_trait::async_trait;
use polars::prelude::*;
use crate::error::Result;
use crate::models::{Catalog, DatasetMetadata, ProviderMetadata};
use crate::traits::{DatasetProvider, FetchOptions, FetchResult};

pub struct EurostatProvider {
    client: reqwest::Client,
    base_url: String,
}

impl Default for EurostatProvider {
    fn default() -> Self {
        Self::new("https://ec.europa.eu/eurostat/api/statistics")
    }
}

impl EurostatProvider {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = /* build HTTP client */;
        Self { client, base_url: base_url.into() }
    }
}

#[async_trait]
impl DatasetProvider for EurostatProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "eurostat".to_string(),
            name: "Eurostat".to_string(),
            description: Some("European statistics API".to_string()),
        }
    }

    async fn ping(&self) -> Result<()> {
        Ok(())
    }

    async fn list_datasets(&self) -> Result<Catalog> {
        // Fetch dataset listing and return Catalog
    }

    async fn fetch_dataset_with_options(
        &self,
        dataset_id: &str,
        options: FetchOptions,
    ) -> Result<FetchResult> {
        // Fetch data, parse into DataFrame, return FetchResult
    }
}
```

### Step 2: Register in the module

Update `src/providers/mod.rs`:

```rust
pub mod abs;
pub mod stats_nz;
pub mod eurostat;       // <-- add

pub use abs::AbsProvider;
pub use stats_nz::StatsNzProvider;
pub use eurostat::EurostatProvider;  // <-- add
```

### Step 3: Register in the registry

Update `src/registry.rs`:

```rust
pub fn with_defaults() -> Self {
    let mut registry = Self::new();
    registry.register(AbsProvider::default());
    registry.register(StatsNzProvider::default());
    registry.register(EurostatProvider::default());  // <-- add
    registry
}
```

### Step 4: Write tests

Use the test HTTP server infrastructure in `src/providers/test_support.rs`:

```rust
#[tokio::test]
async fn eurostat_fetch_returns_dataframe() {
    let response = "HTTP/1.1 200 OK\r\n...";
    let completed = complete_request(response, |base_url| async move {
        let provider = EurostatProvider::new(base_url);
        provider.fetch_dataset("test").await
    }).await;
}
```

### Step 5: Add dataset packs

Create dataset directories under `datasets/eurostat/` following the [dataset pack structure](./dataset-packs).

### Guidelines

- **Conditional requests:** Always implement `fetch_dataset_with_options` for ETag/`Last-Modified` caching.
- **Error handling:** Use `CoreError` variants (`HttpStatus`, `NotFound`, `TransformationError`).
- **Ping:** Keep lightweight — a HEAD or GET to the API base endpoint.
- **Column naming:** For SDMX, flatten dimension columns and use `OBS_VALUE` for observation values. For OData, flatten `value` array entries and strip `@odata.*` fields.
