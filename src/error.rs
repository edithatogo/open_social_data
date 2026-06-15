//! Domain-specific error types.
//!
//! Uses [`thiserror`] to derive [`std::error::Error`]
//! implementations for all error variants returned by library operations.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("API returned HTTP {status} for {url}")]
    HttpStatus { status: u16, url: String },

    #[error("Serialization/Deserialization failed: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Data transformation failed: {0}")]
    TransformationError(String),

    #[error("Data quality assertion failed: {0}")]
    QualityError(String),

    #[error("IO operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("provider is not registered: {0}")]
    ProviderNotRegistered(String),

    #[error("Dataset not found: {0}")]
    NotFound(String),

    #[error("Dataset was not modified: {0}")]
    NotModified(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
