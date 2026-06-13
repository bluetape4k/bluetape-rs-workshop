# Milestone 0.2.0 Examples Design

## Context

`bluetape-rs-workshop` teaches `bluetape-rs` through runnable Rust examples. The
repository uses `main` as its only long-lived branch, and feature work targets
`main` through pull requests.

Milestone `0.1.0` delivered three independent foundation examples:

- `foundation-order-cleanup`: validation, normalization, typed errors, captured
  logs.
- `request-tracing-log-capture`: correlation-aware request logs.
- `temp-resource-test-harness`: isolated temporary filesystem tests.

Milestone `0.2.0` is tracked by epic #5 and child issues #6, #7, and #8 in
`bluetape4k/bluetape-rs-workshop`. This spec uses the workshop repository
issues as the source of truth. Older similarly named `bluetape-rs` repository
issues are not the target for this implementation.

## Goal

Add the `0.2.0` collections and async lane as three runnable example crates that
compose the `0.1.0` validation, logging, and test baseline into small backend
workflows.

## Current Evidence

- Current branch baseline: `make ci` passed on
  `feat/milestone-0.2.0-examples` before implementation.
- Open milestone issues:
  - #5: `[Epic] 0.2.0 collections and async examples`
  - #6: `feat: add catalog enrichment fanout workshop example`
  - #7: `feat: add batched order windowing workshop example`
  - #8: `feat: add shutdown aware worker workshop example`
- Current workspace dependencies include `bluetape-rs-core`,
  `bluetape-rs-logging`, `bluetape-rs-test`, `tokio`, `thiserror`, and
  `tracing`.
- Local Cargo source confirms `bluetape-rs-collections 0.3.1` exports
  `iter::{chunks, group_by, partition_results}`, `Page`, and `PageError`.
- Local Cargo source confirms `bluetape-rs-async 0.3.1` exports
  `try_map_bounded`, `map_bounded_collect`, `with_timeout`,
  `with_timeout_or_cancel`, `run_until_cancelled`, `CancellationSource`, and
  `shutdown_signal`.

## Selected Approach

Use three independent example crates in one PR:

1. `examples/batched-order-windowing`
   - Collections-first workflow.
   - Validates partner order events, groups by tenant/channel, chunks into
     deterministic batches, and returns page metadata.
2. `examples/catalog-enrichment-fanout`
   - Collections plus bounded async fan-out workflow.
   - Validates catalog rows and request metadata, groups rows, pages output, and
     enriches catalog items through required and optional providers.
3. `examples/shutdown-aware-worker`
   - Async lifecycle workflow.
   - Validates worker configuration, performs a small collection transform, logs
     progress, and respects shutdown, timeout, and cancellation.

The root README and Korean README will add a `0.2.0` section with run commands
and a learning path. `WIP.md` will add a detailed `0.2.0 Delivery Scope` table.
The existing diagram generator may be extended only if it can preserve the
current Graphviz and geometry evidence pattern without broad rewrite.

## Alternatives Considered

### Alternative A: One integrated `collections-async-workflow` crate

This would show integration strongly, but it would create a larger learning
surface and make it harder for readers to isolate collection behavior from
async lifecycle behavior. Rejected because the workshop pattern is crate-per
example.

### Alternative B: Three independent PRs

This would reduce review size per PR, but it would repeat root README and
milestone diagram work and make the `0.2.0` lane harder to validate as a single
coherent step. Rejected because the user requested milestone-level example work.

### Alternative C: Three independent crates in one PR

Selected. It preserves small examples while giving one milestone-level README,
review, CI, and DoD gate.

## Example Contracts

### `batched-order-windowing`

Inputs:

- `BatchRun` with `correlation_id`, `page_number`, `page_size`, and raw
  `OrderEvent` rows.
- `OrderEvent` includes `tenant`, `channel`, `order_id`, `sku`, and `quantity`.

Behavior:

- Reject blank required fields and non-positive quantities.
- Group validated rows by `(tenant, channel)` using `bluetape-rs-collections`.
- Chunk each group by requested page size.
- Return a deterministic list of `OrderBatch` values sorted by tenant, channel,
  and chunk index.
- Return `Page<OrderBatch>` with the supplied page metadata.
- Emit a correlation-aware summary log.

Tests:

- Normal grouping/chunking/paging.
- Empty input returns an empty page.
- Invalid page size fails.
- Invalid row fails with typed validation error.

### `catalog-enrichment-fanout`

Inputs:

- `CatalogRequest` with `correlation_id`, `page_number`, `page_size`,
  `max_concurrency`, and `CatalogRow` items.
- Provider fixtures represented as in-memory provider responses, not network
  calls.

Behavior:

- Validate request metadata and product rows.
- Group products by category and page the product view.
- Use `try_map_bounded` or `map_bounded_collect` for provider fan-out with an
  explicit concurrency bound.
- Required provider failure fails the request.
- Optional provider failure is captured as a warning and does not fail the
  request.
- Timeout or cancellation returns a caller-visible typed error.
- Emit correlation-aware logs for the enrichment run.

Tests:

- Success with grouped, paged, enriched output.
- Blank request metadata or product fields fail.
- Required provider failure fails.
- Optional provider failure is recorded but output remains usable.
- Timeout or cancellation behavior is explicit and deterministic.

### `shutdown-aware-worker`

Inputs:

- `WorkerConfig` with `correlation_id`, `max_concurrency`, `deadline`, and
  work items.
- `WorkerItem` with queue, key, and cost fields.

Behavior:

- Validate config and work items.
- Group or chunk work before dispatch.
- Use `shutdown_signal`, `with_timeout_or_cancel`, or `run_until_cancelled`
  to model shutdown-aware lifecycle.
- Keep work async and non-blocking. No blocking work runs on core async tasks.
- Return `WorkerReport` with processed count and shutdown/timeout state.
- Emit correlation-aware progress logs.

Tests:

- Worker completes normal work.
- Invalid config fails.
- Timeout fails with typed timeout state.
- Shutdown/cancellation returns a typed cancelled state.

## Risks and Failure Modes

1. Async tasks can leak or ignore caller shutdown.
   - Mitigation: use `bluetape-rs-async` helpers and deterministic Tokio tests
     for timeout/cancellation.
2. Provider fan-out can hide required provider failures.
   - Mitigation: model required and optional providers separately and assert both
     failure modes.
3. Collection helpers can produce nondeterministic ordering through `HashMap`.
   - Mitigation: sort grouped output before returning it.
4. Examples can become generic snippets instead of backend workflows.
   - Mitigation: every example starts with validation, uses realistic order or
     catalog data, and ends in a caller-visible report.
5. README claims can drift from source names.
   - Mitigation: verify root and example README commands against `Cargo.toml`
     package names and run targeted tests.

## Acceptance Criteria

- `Cargo.toml` registers the three new example crates.
- Root `README.md` and `README.ko.md` link all `0.2.0` examples and include
  commands for each.
- `WIP.md` includes a `0.2.0 Delivery Scope` table.
- Each example has `README.md`, `README.ko.md`, `Cargo.toml`, and `src/lib.rs`.
- Each example uses typed Rust errors with `thiserror`.
- Public structs/functions have English Rustdoc where they are part of the
  example's caller-facing API.
- `batched-order-windowing` covers validation, grouping, chunking, paging, and
  boundary tests.
- `catalog-enrichment-fanout` covers validation, grouping, paging, bounded
  fan-out, required failure, optional failure, and timeout or cancellation.
- `shutdown-aware-worker` covers success, input validation, timeout, and
  shutdown/cancellation.
- Local validation passes:
  - `cargo fmt --all --check`
  - `cargo test -p batched-order-windowing`
  - `cargo test -p catalog-enrichment-fanout`
  - `cargo test -p shutdown-aware-worker`
  - `cargo test --workspace --all-features`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `git diff --check`

## Out of Scope

- Upgrading `bluetape-rs-*` dependencies from `0.3.1` to `0.4.0`.
- Real network, database, or filesystem integration beyond existing test helpers.
- Releasing or closing milestone `0.2.0`; this PR only delivers the examples.
- Adding benchmark harnesses.
- Changing GitHub Actions unless the new workspace members are not covered by
  existing `make ci` behavior.

## DoD

- Spec and implementation plan are committed before implementation.
- Tests are written first for new behavior and observed failing before the
  corresponding implementation is added.
- Step 2-R, Step 3-R, Step 6-R, and PR review gates converge with P0=0 and P1=0.
- Lessons are committed before PR creation.
- PR body ends with `## DoD Status`.
