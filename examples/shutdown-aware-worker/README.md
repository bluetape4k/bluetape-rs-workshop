# shutdown-aware-worker

[English](README.md) | [한국어](README.ko.md)

This example runs grouped worker items until work completes, a timeout fires, or
a shutdown signal is requested.

## Scenario

A worker receives queue items with different simulated costs. The example
validates the run configuration, groups work by queue, chunks each queue, and
checks the shutdown signal between async sleep-based work units.

![shutdown-aware-worker flow](../../docs/images/readme-diagrams/example-shutdown-aware-worker.png)

## Representative Code

```rust
let (_trigger, signal) = shutdown_signal();
let report = run_worker(config, signal).await?;

assert_eq!(report.status, WorkerStatus::Completed);
```

## What To Notice

- `shutdown_signal` provides explicit trigger/listener ownership.
- `with_timeout` maps deadline expiry to `AsyncControlError::TimedOut`.
- Shutdown is observed through `ShutdownSignal::wait` and returned as
  `AsyncControlError::Cancelled`.

## Run

```bash
cargo test -p shutdown-aware-worker
```
