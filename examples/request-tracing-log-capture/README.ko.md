# request-tracing-log-capture

`CorrelationId`가 포함된 요청 처리 로그를 기록하고 테스트에서 캡처된 로그를
검증하는 예제입니다.

## Scenario

요청 로그는 요청 경계에서 route, status, correlation ID를 먼저 검증하고
구조화 필드로 남길 때 가치가 있습니다. 이 예제는 캡처 subscriber를 사용해
테스트에서 이벤트 payload를 직접 검증합니다.

![request-tracing-log-capture flow](../../docs/images/readme-diagrams/example-request-tracing-log-capture.png)

## Representative Code

```rust
let captured = CapturedLogs::new();
let subscriber = capture_subscriber(captured.clone(), "info")?;

let summary = with_default(subscriber, || {
    record_request("corr-042", "/orders", 202)
})?;

assert_eq!(summary.status, 202);
assert!(captured.to_lossy_string().contains("corr-042"));
```

## What To Notice

- `CorrelationId::new`은 공백, 과도하게 긴 값, unsafe 문자를 거부합니다.
- `tracing::info!`는 `correlation.id`, `http.status` 같은 구조화 필드를
  기록합니다.
- `with_default`로 캡처 subscriber 범위를 테스트 본문에 한정합니다.

## Run

```bash
cargo test -p request-tracing-log-capture
```
