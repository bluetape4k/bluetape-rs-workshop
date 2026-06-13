# Milestone 0.2.0 Examples Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the `0.2.0` workshop lane with three runnable Rust example crates for collections, bounded async fan-out, and shutdown-aware workers.

**Architecture:** Keep one crate per example. Each crate exposes only short Rustdoc and
public re-exports in `src/lib.rs`; domain types, typed errors, workflow logic,
and tests live in focused modules. Each crate validates caller input with
`bluetape-rs-core`, uses typed errors with `thiserror`, and keeps detailed
learning prose in bilingual README files. Root docs describe the milestone lane
and link the examples.

**Tech Stack:** Rust 2024, `bluetape-rs-core 0.3.1`, `bluetape-rs-logging 0.3.1`, `bluetape-rs-collections 0.3.1`, `bluetape-rs-async 0.3.1`, `tokio`, `thiserror`, `tracing`.

---

## File Structure

- Modify `Cargo.toml`: add three workspace members and workspace dependencies for `bluetape-rs-collections` and `bluetape-rs-async`; add missing Tokio features `rt-multi-thread` and `sync` if async tests require them.
- Modify `README.md` and `README.ko.md`: add milestone `0.2.0` examples, learning path, architecture/sequence prose, and layout entries.
- Modify `WIP.md`: add `0.2.0 Delivery Scope`.
- Create `examples/batched-order-windowing/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,windowing.rs,tests.rs}}`.
- Create `examples/catalog-enrichment-fanout/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,enrichment.rs,tests.rs}}`.
- Create `examples/shutdown-aware-worker/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,worker.rs,tests.rs}}`.
- Create `docs/lessons/2026-06-13-milestone-0-2-0-examples.md`.
- Create `docs/review/2026-06-13-milestone-0-2-0-code-review.md`.

## Task 1: Workspace Registration

complexity: low

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Add workspace members and dependencies**

Add these workspace members:

```toml
    "examples/batched-order-windowing",
    "examples/catalog-enrichment-fanout",
    "examples/shutdown-aware-worker",
```

Add these workspace dependencies:

```toml
bluetape-rs-async = "0.3.1"
bluetape-rs-collections = "0.3.1"
```

If compilation requires it, change Tokio features to:

```toml
tokio = { version = "1.48.0", default-features = false, features = ["macros", "rt", "rt-multi-thread", "sync", "time", "test-util"] }
```

- [ ] **Step 2: Verify existing examples still resolve**

Run:

```bash
cargo metadata --no-deps --format-version 1
```

Expected: all six package names appear after Task 2-4 crates exist; before that, run after crate skeletons are added.

## Task 2: `batched-order-windowing`

complexity: medium

Apply `$bluetape-rs-patterns`: Rust-native typed errors, deterministic ordering after `HashMap` grouping, no panics on caller input, English Rustdoc for public API.

**Files:**
- Create: `examples/batched-order-windowing/Cargo.toml`
- Create: `examples/batched-order-windowing/README.md`
- Create: `examples/batched-order-windowing/README.ko.md`
- Create: `examples/batched-order-windowing/src/lib.rs`
- Create: `examples/batched-order-windowing/src/domain.rs`
- Create: `examples/batched-order-windowing/src/error.rs`
- Create: `examples/batched-order-windowing/src/windowing.rs`
- Create: `examples/batched-order-windowing/src/tests.rs`

- [ ] **Step 1: Write tests first**

Create `src/lib.rs` with short crate Rustdoc and public exports only. Put public
types in `domain.rs`, public errors in `error.rs`, workflow logic in
`windowing.rs`, and tests in `tests.rs`. The first test must fail because
implementation is not present yet:

```rust
#[test]
fn groups_chunks_and_pages_partner_events() {
    let page = build_order_batches(BatchRun {
        correlation_id: "corr-batch-001".to_owned(),
        page_number: 0,
        page_size: 2,
        events: vec![
            OrderEvent::new("north", "web", "ord-1", "sku-1", 2),
            OrderEvent::new("north", "web", "ord-2", "sku-2", 1),
            OrderEvent::new("north", "store", "ord-3", "sku-3", 4),
        ],
    })
    .expect("batch page should build");

    assert_eq!(page.total_items(), 2);
    assert_eq!(page.items()[0].tenant, "north");
    assert_eq!(page.items()[0].channel, "store");
    assert_eq!(page.items()[0].orders.len(), 1);
    assert_eq!(page.items()[1].channel, "web");
    assert_eq!(page.items()[1].orders.len(), 2);
}
```

Also add tests for empty input, invalid page size, and blank tenant.

- [ ] **Step 2: Run RED test**

Run:

```bash
cargo test -p batched-order-windowing
```

Expected: fails because the crate or implementation is incomplete.

- [ ] **Step 3: Implement minimal behavior**

Use:

- `require_not_blank`
- `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::iter::group_by`
- `bluetape_rs_collections::iter::chunks`
- `bluetape_rs_collections::Page`
- `tracing::info!`

Implementation must sort grouped batches by `(tenant, channel)` before paging.

- [ ] **Step 4: Run GREEN test**

Run:

```bash
cargo test -p batched-order-windowing
```

Expected: all `batched-order-windowing` tests pass.

- [ ] **Step 5: Add bilingual README**

README files must include scenario, representative code, things to notice, and:

```bash
cargo test -p batched-order-windowing
```

## Task 3: `catalog-enrichment-fanout`

complexity: high

Apply `$bluetape-rs-patterns`: bounded Tokio tasks, typed provider failures, cancellation/timeout evidence, no hidden required-provider failures.

**Files:**
- Create: `examples/catalog-enrichment-fanout/Cargo.toml`
- Create: `examples/catalog-enrichment-fanout/README.md`
- Create: `examples/catalog-enrichment-fanout/README.ko.md`
- Create: `examples/catalog-enrichment-fanout/src/lib.rs`
- Create: `examples/catalog-enrichment-fanout/src/domain.rs`
- Create: `examples/catalog-enrichment-fanout/src/error.rs`
- Create: `examples/catalog-enrichment-fanout/src/enrichment.rs`
- Create: `examples/catalog-enrichment-fanout/src/tests.rs`

- [ ] **Step 1: Write tests first**

Tests must cover:

- success with grouped, paged, enriched output
- blank request metadata
- required provider failure
- optional provider failure recorded as warning
- timeout or cancellation

Use short deterministic sleeps for timeout coverage. Avoid paused-time tests unless the
workspace Tokio features make them necessary.
Keep `lib.rs` to crate Rustdoc, module declarations, and re-exports.

- [ ] **Step 2: Run RED test**

Run:

```bash
cargo test -p catalog-enrichment-fanout
```

Expected: fails before implementation is complete.

- [ ] **Step 3: Implement bounded fan-out**

Use:

- `require_not_blank`, `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::{iter, Page}`
- `bluetape_rs_async::{map_bounded_collect, with_timeout_or_cancel, CancellationSource}`
- `tracing::info!` and `tracing::warn!`

Provider fixtures stay in memory. Required provider errors fail the request.
Optional provider errors append warning strings to the returned view.

- [ ] **Step 4: Run GREEN test**

Run:

```bash
cargo test -p catalog-enrichment-fanout
```

Expected: all tests pass.

- [ ] **Step 5: Add bilingual README**

README files must document the required-vs-optional provider contract and:

```bash
cargo test -p catalog-enrichment-fanout
```

## Task 4: `shutdown-aware-worker`

complexity: high

Apply `$bluetape-rs-patterns`: explicit async lifecycle, timeout and shutdown tests, no blocking work on async tasks.

**Files:**
- Create: `examples/shutdown-aware-worker/Cargo.toml`
- Create: `examples/shutdown-aware-worker/README.md`
- Create: `examples/shutdown-aware-worker/README.ko.md`
- Create: `examples/shutdown-aware-worker/src/lib.rs`
- Create: `examples/shutdown-aware-worker/src/domain.rs`
- Create: `examples/shutdown-aware-worker/src/error.rs`
- Create: `examples/shutdown-aware-worker/src/worker.rs`
- Create: `examples/shutdown-aware-worker/src/tests.rs`

- [ ] **Step 1: Write tests first**

Tests must cover:

- worker completes normal work
- invalid config fails
- timeout returns typed timeout
- shutdown/cancellation returns typed cancelled state

Keep `lib.rs` to crate Rustdoc, module declarations, and re-exports.

- [ ] **Step 2: Run RED test**

Run:

```bash
cargo test -p shutdown-aware-worker
```

Expected: fails before implementation is complete.

- [ ] **Step 3: Implement worker**

Use:

- `require_not_blank`, `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::iter::group_by`
- `bluetape_rs_async::{shutdown_signal, with_timeout, AsyncControlError}`
- `tokio::time::sleep` for deterministic simulated work
- `tracing::info!`

Return a `WorkerReport` with processed count and final status.

- [ ] **Step 4: Run GREEN test**

Run:

```bash
cargo test -p shutdown-aware-worker
```

Expected: all tests pass.

- [ ] **Step 5: Add bilingual README**

README files must explain the cancellation contract and:

```bash
cargo test -p shutdown-aware-worker
```

## Task 5: Root Documentation

complexity: medium

**Files:**
- Modify: `README.md`
- Modify: `README.ko.md`
- Modify: `WIP.md`

- [ ] **Step 1: Update root README example table**

Add a `Milestone 0.2.0: Collections and Async Examples` section with rows for
all three new examples and exact package test commands.

- [ ] **Step 2: Update Korean README**

Mirror the English content with Korean descriptions and exact same commands.

- [ ] **Step 3: Update `WIP.md`**

Add a `0.2.0 Delivery Scope` table:

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| #6 | `catalog-enrichment-fanout` | `core`, `logging`, `collections`, `async` | bounded provider fan-out with required/optional failure behavior |
| #7 | `batched-order-windowing` | `core`, `logging`, `collections` | deterministic grouping, chunking, paging |
| #8 | `shutdown-aware-worker` | `core`, `logging`, `collections`, `async` | bounded shutdown, timeout, cancellation |

- [ ] **Step 4: Verify docs against source**

Run:

```bash
rg -n "batched-order-windowing|catalog-enrichment-fanout|shutdown-aware-worker" README.md README.ko.md WIP.md Cargo.toml examples
```

Expected: package names and commands appear consistently.

## Task 6: Validation

complexity: medium

**Files:** all changed files.

- [ ] **Step 1: Run formatting**

Run:

```bash
cargo fmt --all --check
```

Expected: exit 0.

- [ ] **Step 2: Run targeted tests**

Run:

```bash
cargo test -p batched-order-windowing
cargo test -p catalog-enrichment-fanout
cargo test -p shutdown-aware-worker
```

Expected: exit 0 for all.

- [ ] **Step 3: Run workspace tests and clippy**

Run:

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected: exit 0 for both.

- [ ] **Step 4: Run repository gate**

Run:

```bash
make ci
git diff --check
```

Expected: exit 0 for both.

## Task 7: Review, Lessons, PR

complexity: medium

**Files:**
- Create: `docs/review/2026-06-13-milestone-0-2-0-code-review.md`
- Create: `docs/lessons/2026-06-13-milestone-0-2-0-examples.md`

- [ ] **Step 1: Step 6-R code review**

Run local/native six-lane review against the final diff. Record P0/P1/P2/P3 in
`docs/review/2026-06-13-milestone-0-2-0-code-review.md`.

- [ ] **Step 2: Commit lessons before PR**

Create `docs/lessons/2026-06-13-milestone-0-2-0-examples.md` with what changed,
what was surprising, and validation evidence. Commit it before PR creation.

- [ ] **Step 3: Create PR**

Push branch and create PR with title:

```text
feat: add milestone 0.2.0 collections and async examples
```

PR body must include `Closes #6`, `Closes #7`, `Closes #8`, reference #5, and end
with `## DoD Status`.

## Self-Review

- Spec coverage: every acceptance criterion maps to Tasks 1-7.
- Open-item scan: no deferred or unspecified implementation tasks.
- Type consistency: package names are kebab-case; Rust module crates resolve to
  snake_case by Cargo; `lib.rs` files stay as export surfaces and do not hold all
  implementation code.
- Risk coverage: async timeout/cancellation, provider failure separation,
  deterministic ordering, and README source-drift checks are assigned.
