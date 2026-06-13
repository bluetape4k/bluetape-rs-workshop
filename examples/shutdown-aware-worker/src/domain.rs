use std::time::Duration;

/// Unit of work accepted by the worker example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerItem {
    pub queue: String,
    pub key: String,
    pub cost: Duration,
}

impl WorkerItem {
    #[must_use]
    pub fn new(queue: &str, key: &str, cost: Duration) -> Self {
        Self {
            queue: queue.to_owned(),
            key: key.to_owned(),
            cost,
        }
    }
}

/// Worker run configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerConfig {
    pub correlation_id: String,
    pub max_batch_size: usize,
    pub timeout: Duration,
    pub items: Vec<WorkerItem>,
}

/// Final worker lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    Completed,
    TimedOut,
    Cancelled,
}

/// Worker run result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerReport {
    pub status: WorkerStatus,
    pub processed: usize,
    pub queues: Vec<String>,
}
