# Milestone 0.2.0 Examples Lessons

Date: 2026-06-13
Branch: `feat/milestone-0.2.0-examples`

## What Changed

- Added `batched-order-windowing` for deterministic grouping, chunking, and
  `Page` metadata.
- Added `catalog-enrichment-fanout` for bounded async provider work with
  required-provider failures, optional-provider warnings, and timeout control.
- Added `shutdown-aware-worker` for grouped worker processing with typed timeout
  and shutdown cancellation errors.
- Registered `bluetape-rs-collections = "0.3.1"` and
  `bluetape-rs-async = "0.3.1"` without upgrading the existing `0.3.1` workshop
  dependency lane.

## What Was Corrected

The first implementation plan draft implied each new example could live mostly
inside `src/lib.rs`. That was corrected before implementation. The final crates
keep `lib.rs` as a short Rustdoc/export surface and split code into domain,
error, workflow, and test modules.

## Validation Evidence

- RED targeted tests failed at the intentional `todo!()` implementation points.
- `cargo test -p batched-order-windowing`: PASS, 4 tests.
- `cargo test -p catalog-enrichment-fanout`: PASS, 5 tests.
- `cargo test -p shutdown-aware-worker`: PASS, 4 tests.
- `cargo fmt --all --check`: PASS.
- `cargo test --workspace --all-features`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `make ci`: PASS.
- `git diff --check`: PASS.
- Step 6-R code review: PASS, `P0=0 P1=0`.

## Carry Forward

- For future Rust workshop examples, start the plan with module boundaries
  before writing tests.
- Treat `HashMap`-backed grouping as nondeterministic until explicitly sorted.
- Keep external provider/network behavior out of workshop examples unless an
  issue explicitly asks for integration tests and service fixtures.
