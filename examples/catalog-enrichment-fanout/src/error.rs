use bluetape_rs_async::{AsyncControlError, TaskGroupError};
use bluetape_rs_collections::{CollectionError, PageError};
use bluetape_rs_core::ValidationError;
use bluetape_rs_logging::CorrelationIdError;
use thiserror::Error;

/// Provider failure captured while enriching a product.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("provider `{name}` failed for product `{product_id}`: {message}")]
pub struct ProviderError {
    pub name: String,
    pub product_id: String,
    pub required: bool,
    pub message: String,
}

/// Errors returned by catalog enrichment.
#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("correlation id rejected: {0}")]
    Correlation(#[from] CorrelationIdError),
    #[error("collection operation failed: {0}")]
    Collection(#[from] CollectionError),
    #[error("page metadata rejected: {0}")]
    Page(#[from] PageError),
    #[error("async control failed: {0}")]
    Control(#[from] AsyncControlError),
    #[error("provider fan-out failed: {0}")]
    Fanout(#[from] TaskGroupError<ProviderError>),
    #[error("required provider failed: {0}")]
    RequiredProvider(ProviderError),
}
