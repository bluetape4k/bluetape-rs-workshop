# catalog-enrichment-fanout

[English](README.md) | [한국어](README.ko.md)

This example enriches catalog rows through bounded async provider fan-out.
Required provider failures fail the request; optional provider failures are
returned as warnings on each product.

## Scenario

A catalog page needs inventory and recommendation data. The example validates
request metadata, groups rows before deterministic paging, runs provider work
with `map_bounded_collect`, and wraps the fan-out in an explicit timeout and
cancellation boundary.

![catalog-enrichment-fanout flow](../../docs/images/readme-diagrams/example-catalog-enrichment-fanout.png)

## Representative Code

```rust
let page = enrich_catalog(request).await?;

assert_eq!(page.items()[0].category, "books");
assert!(page.items()[0].warnings.is_empty());
```

## What To Notice

- `map_bounded_collect` records every provider task result without losing input
  order.
- `with_timeout_or_cancel` keeps timeout and cancellation behavior explicit.
- Required provider failures return `CatalogError::RequiredProvider`; optional
  failures stay visible as product warnings.

## Run

```bash
cargo test -p catalog-enrichment-fanout
```
