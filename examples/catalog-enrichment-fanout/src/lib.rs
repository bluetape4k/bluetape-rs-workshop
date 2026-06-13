//! Bounded async catalog enrichment fan-out example.

mod domain;
mod enrichment;
mod error;

pub use domain::{CatalogRequest, CatalogRow, EnrichedProduct, ProviderFixture};
pub use enrichment::enrich_catalog;
pub use error::{CatalogError, ProviderError};

#[cfg(test)]
mod tests;
