use bluetape_rs_async::{AsyncControlError, ShutdownSignal, with_timeout};
use bluetape_rs_collections::iter;
use bluetape_rs_core::{require_not_blank, require_positive};
use bluetape_rs_logging::CorrelationId;
use tokio::time::sleep;
use tracing::info;

use crate::{WorkerConfig, WorkerError, WorkerItem, WorkerReport, WorkerStatus};

/// Runs work until it completes, times out, or receives shutdown.
pub async fn run_worker(
    config: WorkerConfig,
    shutdown: ShutdownSignal,
) -> Result<WorkerReport, WorkerError> {
    let correlation_id =
        CorrelationId::new(require_not_blank("correlation_id", &config.correlation_id)?)?;
    let max_batch_size = require_positive("max_batch_size", config.max_batch_size)?;
    let items = validate_items(config.items)?;
    let timeout = config.timeout;

    match with_timeout(timeout, process_items(items, max_batch_size, shutdown)).await {
        Ok(report) => {
            let report = report?;
            info!(
                correlation.id = correlation_id.as_str(),
                processed = report.processed,
                status = ?report.status,
                "worker completed"
            );
            Ok(report)
        }
        Err(error) => Err(WorkerError::Control(error)),
    }
}

fn validate_items(items: Vec<WorkerItem>) -> Result<Vec<WorkerItem>, WorkerError> {
    items
        .into_iter()
        .map(|item| {
            Ok(WorkerItem {
                queue: require_not_blank("queue", &item.queue)?.to_owned(),
                key: require_not_blank("key", &item.key)?.to_owned(),
                cost: item.cost,
            })
        })
        .collect()
}

async fn process_items(
    items: Vec<WorkerItem>,
    max_batch_size: usize,
    mut shutdown: ShutdownSignal,
) -> Result<WorkerReport, WorkerError> {
    if shutdown.is_shutdown_requested() {
        return Err(WorkerError::Control(AsyncControlError::Cancelled));
    }

    let grouped = iter::group_by(items, |item| item.queue.clone());
    let mut grouped: Vec<_> = grouped.into_iter().collect();
    grouped.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut processed = 0;
    let mut queues = Vec::with_capacity(grouped.len());
    for (queue, items) in grouped {
        queues.push(queue);
        for chunk in iter::chunks(items, max_batch_size)? {
            for item in chunk {
                tokio::select! {
                    biased;
                    _ = shutdown.wait() => {
                        return Err(WorkerError::Control(AsyncControlError::Cancelled));
                    }
                    _ = sleep(item.cost) => {
                        processed += 1;
                    }
                }
            }
        }
    }

    Ok(WorkerReport {
        status: WorkerStatus::Completed,
        processed,
        queues,
    })
}
