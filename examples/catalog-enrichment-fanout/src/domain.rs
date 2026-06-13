use std::collections::HashMap;
use std::time::Duration;

/// Catalog row accepted by the enrichment example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogRow {
    pub product_id: String,
    pub category: String,
    pub title: String,
}

impl CatalogRow {
    #[must_use]
    pub fn new(product_id: &str, category: &str, title: &str) -> Self {
        Self {
            product_id: product_id.to_owned(),
            category: category.to_owned(),
            title: title.to_owned(),
        }
    }
}

/// In-memory provider fixture used by the fan-out example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderFixture {
    pub name: String,
    pub required: bool,
    pub latency: Duration,
    pub attributes: HashMap<String, String>,
    pub failure: Option<String>,
}

/// Input for catalog enrichment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogRequest {
    pub correlation_id: String,
    pub page_number: u64,
    pub page_size: u64,
    pub max_concurrency: usize,
    pub timeout: Duration,
    pub rows: Vec<CatalogRow>,
    pub providers: Vec<ProviderFixture>,
}

/// Enriched product returned in the output page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrichedProduct {
    pub product_id: String,
    pub category: String,
    pub title: String,
    pub attributes: Vec<String>,
    pub warnings: Vec<String>,
}
