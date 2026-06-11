# foundation-order-cleanup

This example normalizes raw partner order rows into a typed internal order model.
It demonstrates `bluetape-rs-core` validation helpers and `bluetape-rs-logging`
log capture.

## Scenario

A partner feed can contain blank optional fields and invalid line items. The
example keeps caller errors explicit, drops invalid line items, and emits a
captured cleanup log with the correlation ID.

![foundation-order-cleanup flow](../../docs/images/readme-diagrams/example-foundation-order-cleanup.png)

## Representative Code

```rust
let report = normalize_partner_rows(
    "corr-001",
    &[PartnerRow {
        tenant: "north".to_owned(),
        order_id: "ord-100".to_owned(),
        customer: "alice".to_owned(),
        channel: " ".to_owned(),
        items: vec![PartnerItem {
            sku: "sku-1".to_owned(),
            quantity: 2,
            note: "fragile".to_owned(),
        }],
    }],
)?;

assert_eq!(report.orders[0].channel, "unknown");
assert_eq!(report.correlation_id.as_str(), "corr-001");
```

## What To Notice

- `require_not_blank("field", value)` rejects required blank fields.
- `blank_to_default(value, "unknown")` keeps optional partner input tolerant.
- `CapturedLogs` and `capture_subscriber` make the emitted cleanup event
  testable.

## Run

```bash
cargo test -p foundation-order-cleanup
```
