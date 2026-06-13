# batched-order-windowing

[English](README.md) | [한국어](README.ko.md)

This example groups partner order events by tenant and channel, chunks each
group into deterministic batches, and returns a materialized `Page`.

## Scenario

Partner events arrive as flat rows. The example validates required fields,
preserves a correlation ID for logs, sorts grouped output after `HashMap`
grouping, and pages the resulting batches.

![batched-order-windowing flow](../../docs/images/readme-diagrams/example-batched-order-windowing.png)

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

- `require_not_blank` and `require_positive` keep caller input failures typed.
- `iter::group_by` uses a `HashMap`, so the example sorts groups before paging.
- `iter::chunks` and `Page::with_meta` make batch size and page metadata
  explicit.

## Run

```bash
cargo test -p batched-order-windowing
```
