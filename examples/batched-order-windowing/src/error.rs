use bluetape_rs_collections::{CollectionError, PageError};
use bluetape_rs_core::ValidationError;
use bluetape_rs_logging::CorrelationIdError;
use thiserror::Error;

/// Errors returned while building order batches.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BatchError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("correlation id rejected: {0}")]
    Correlation(#[from] CorrelationIdError),
    #[error("collection operation failed: {0}")]
    Collection(#[from] CollectionError),
    #[error("page metadata rejected: {0}")]
    Page(#[from] PageError),
}
