//! Timeout and shutdown aware worker-loop example.

mod domain;
mod error;
mod worker;

pub use domain::{WorkerConfig, WorkerItem, WorkerReport, WorkerStatus};
pub use error::WorkerError;
pub use worker::run_worker;

#[cfg(test)]
mod tests;
