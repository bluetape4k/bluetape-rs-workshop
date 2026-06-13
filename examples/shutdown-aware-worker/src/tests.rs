use std::time::Duration;

use bluetape_rs_async::{AsyncControlError, shutdown_signal};

use super::*;

#[tokio::test]
async fn worker_completes_grouped_work() {
    let (_trigger, signal) = shutdown_signal();
    let report = run_worker(config(Duration::from_millis(50)), signal)
        .await
        .expect("worker should complete");

    assert_eq!(report.status, WorkerStatus::Completed);
    assert_eq!(report.processed, 3);
    assert_eq!(report.queues, vec!["email", "sms"]);
}

#[tokio::test]
async fn invalid_config_fails_before_work() {
    let (_trigger, signal) = shutdown_signal();
    let mut config = config(Duration::from_millis(50));
    config.max_batch_size = 0;

    let err = run_worker(config, signal)
        .await
        .expect_err("invalid batch size should fail");

    assert!(matches!(err, WorkerError::Validation(_)));
}

#[tokio::test]
async fn timeout_returns_typed_error() {
    let (_trigger, signal) = shutdown_signal();
    let err = run_worker(config(Duration::from_millis(1)), signal)
        .await
        .expect_err("slow work should time out");

    assert_eq!(err, WorkerError::Control(AsyncControlError::TimedOut));
}

#[tokio::test]
async fn shutdown_returns_typed_cancelled_error() {
    let (trigger, signal) = shutdown_signal();
    trigger.shutdown();

    let err = run_worker(config(Duration::from_millis(50)), signal)
        .await
        .expect_err("shutdown should cancel work");

    assert_eq!(err, WorkerError::Control(AsyncControlError::Cancelled));
}

fn config(timeout: Duration) -> WorkerConfig {
    WorkerConfig {
        correlation_id: "corr-worker-001".to_owned(),
        max_batch_size: 2,
        timeout,
        items: vec![
            WorkerItem::new("email", "msg-1", Duration::from_millis(5)),
            WorkerItem::new("email", "msg-2", Duration::from_millis(5)),
            WorkerItem::new("sms", "msg-3", Duration::from_millis(5)),
        ],
    }
}
