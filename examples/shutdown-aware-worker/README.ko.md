# shutdown-aware-worker

이 예제는 작업이 끝나거나 timeout이 발생하거나 shutdown signal이 요청될 때까지
그룹화된 worker item을 실행합니다.

## Scenario

worker는 비용이 다른 queue item을 받습니다. 예제는 실행 설정을 검증하고,
queue 기준으로 작업을 그룹화하며, 각 queue를 chunk로 나눈 뒤 async sleep 기반
작업 단위 사이에서 shutdown signal을 확인합니다.

## Representative Code

```rust
let (_trigger, signal) = shutdown_signal();
let report = run_worker(config, signal).await?;

assert_eq!(report.status, WorkerStatus::Completed);
```

## What To Notice

- `shutdown_signal`은 trigger/listener 소유권을 명시합니다.
- `with_timeout`은 deadline 만료를 `AsyncControlError::TimedOut`으로
  매핑합니다.
- shutdown은 `ShutdownSignal::wait`로 관찰하고
  `AsyncControlError::Cancelled`로 반환합니다.

## Run

```bash
cargo test -p shutdown-aware-worker
```
