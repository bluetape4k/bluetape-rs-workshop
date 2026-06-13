//! Deterministic order-event grouping, chunking, and paging example.

mod domain;
mod error;
mod windowing;

pub use domain::{BatchRun, OrderBatch, OrderEvent, OrderLine};
pub use error::BatchError;
pub use windowing::build_order_batches;

#[cfg(test)]
mod tests;
