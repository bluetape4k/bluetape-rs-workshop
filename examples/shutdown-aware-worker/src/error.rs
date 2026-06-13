use bluetape_rs_async::AsyncControlError;
use bluetape_rs_collections::CollectionError;
use bluetape_rs_core::ValidationError;
use bluetape_rs_logging::CorrelationIdError;
use thiserror::Error;

/// Errors returned by the shutdown-aware worker.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkerError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("correlation id rejected: {0}")]
    Correlation(#[from] CorrelationIdError),
    #[error("collection operation failed: {0}")]
    Collection(#[from] CollectionError),
    #[error("async control failed: {0}")]
    Control(#[from] AsyncControlError),
}
