# batched-order-windowing

이 예제는 파트너 주문 이벤트를 tenant/channel 기준으로 묶고, 각 그룹을
결정적인 배치로 자른 뒤 `Page`로 반환합니다.

## Scenario

파트너 이벤트는 납작한 행으로 도착합니다. 예제는 필수 필드를 검증하고,
로그용 correlation ID를 보존하며, `HashMap` 기반 그룹 결과를 정렬한 뒤
배치 페이지를 만듭니다.

## Representative Code

```rust
let page = build_order_batches(BatchRun {
    correlation_id: "corr-batch-001".to_owned(),
    page_number: 0,
    page_size: 2,
    events: vec![
        OrderEvent::new("north", "web", "ord-1", "sku-1", 2),
        OrderEvent::new("north", "web", "ord-2", "sku-2", 1),
    ],
})?;

assert_eq!(page.total_items(), 1);
assert_eq!(page.items()[0].orders.len(), 2);
```

## What To Notice

- `require_not_blank`와 `require_positive`로 호출자 입력 오류를 타입으로
  반환합니다.
- `iter::group_by`는 `HashMap`을 사용하므로 페이지를 만들기 전에 그룹을
  정렬합니다.
- `iter::chunks`와 `Page::with_meta`로 배치 크기와 페이지 메타데이터를
  명시합니다.

## Run

```bash
cargo test -p batched-order-windowing
```
