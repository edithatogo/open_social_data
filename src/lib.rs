//! # Open Social Data Core
//!
//! A high-performance Rust library for downloading, processing, and exporting
//! public social datasets from national statistics agencies such as the
//! Australian Bureau of Statistics (ABS) and Stats New Zealand (Stats NZ).
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - **`traits`**: Core abstractions including the [`DatasetProvider`] trait and
//!   [`FetchResult`]/[`FetchOptions`] types.
//! - **`models`**: Standard data structures for provider/dataset metadata.
//! - **`error`**: Domain-specific error types via [`thiserror`].
//! - **`providers`**: Concrete provider implementations (ABS, Stats NZ).
//! - **`registry`**: Dynamic provider registry for plugin-style loading.
//! - **`catalog`**: JSON-backed local catalog for tracking fetched datasets.
//! - **`sqlite_catalog`**: SQLite-backed local catalog storage.
//! - **`catalog_sync`**: Syncing remote provider catalogs into the local catalog.
//! - **`pipeline`**: DataFrame ingestion, schema validation, and atomic Parquet export.
//! - **`quality`**: Data quality assertions and validation reports.
//! - **`hardening`**: Retry policies, circuit breakers, and panic-safe provider execution.
//! - **`mock`**: Mock provider for testing.

pub mod catalog;
pub mod catalog_sync;
pub mod error;
pub mod hardening;
pub mod mock;
pub mod models;
pub mod pipeline;
pub mod providers;
pub mod quality;
pub mod registry;
pub mod sqlite_catalog;
pub mod traits;

// Re-export core types for convenience
pub use catalog::*;
pub use catalog_sync::*;
pub use error::*;
pub use hardening::*;
pub use mock::*;
pub use models::*;
pub use pipeline::*;
pub use providers::*;
pub use quality::*;
pub use registry::*;
pub use sqlite_catalog::*;
pub use traits::*;
