/// Partner order event accepted by the windowing example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderEvent {
    pub tenant: String,
    pub channel: String,
    pub order_id: String,
    pub sku: String,
    pub quantity: i32,
}

impl OrderEvent {
    #[must_use]
    pub fn new(tenant: &str, channel: &str, order_id: &str, sku: &str, quantity: i32) -> Self {
        Self {
            tenant: tenant.to_owned(),
            channel: channel.to_owned(),
            order_id: order_id.to_owned(),
            sku: sku.to_owned(),
            quantity,
        }
    }
}

/// Input for building deterministic order batches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchRun {
    pub correlation_id: String,
    pub page_number: u64,
    pub page_size: u64,
    pub events: Vec<OrderEvent>,
}

/// Validated order line inside an output batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderLine {
    pub order_id: String,
    pub sku: String,
    pub quantity: u32,
}

/// Grouped and chunked order batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBatch {
    pub tenant: String,
    pub channel: String,
    pub batch_index: usize,
    pub orders: Vec<OrderLine>,
}
