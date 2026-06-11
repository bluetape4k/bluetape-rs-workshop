# bluetape-rs-workshop

Runnable Rust backend examples for
[`bluetape-rs`](https://github.com/bluetape4k/bluetape-rs).

This repository uses `main` as the only long-lived branch. Feature work should
open pull requests against `main`.

## Languages

- [English](README.md)
- [Korean](README.ko.md)

## Milestone 0.1.0: Foundation Examples

Milestone 0.1.0 introduces small, runnable examples for the first workshop
building blocks. Each example is intentionally focused, but they share a single
scenario: ingest a partner order, preserve request context, and test filesystem
side effects without leaking temporary state.

| Example | Focus | Run |
|---|---|---|
| [`foundation-order-cleanup`](examples/foundation-order-cleanup/README.md) | Validate and normalize partner order rows with typed errors and captured logs | `cargo test -p foundation-order-cleanup` |
| [`request-tracing-log-capture`](examples/request-tracing-log-capture/README.md) | Emit and assert correlation-aware request logs | `cargo test -p request-tracing-log-capture` |
| [`temp-resource-test-harness`](examples/temp-resource-test-harness/README.md) | Use temporary workspaces for isolated file tests | `cargo test -p temp-resource-test-harness` |

## Learning Path

1. Start with `foundation-order-cleanup` to learn how validation helpers turn
   raw partner input into typed domain output.
2. Move to `request-tracing-log-capture` to see how correlation IDs are carried
   into structured logs and asserted in tests.
3. Finish with `temp-resource-test-harness` to isolate file-producing tests with
   deterministic cleanup.

## Architecture

![Milestone 0.1.0 architecture](docs/images/readme-diagrams/workshop-foundation-architecture.png)

The workshop keeps each foundation example as an independent crate so learners
can inspect one `bluetape-rs` capability at a time. Later milestones will reuse
these crates as a larger service-style flow.

## Sequence

![Milestone 0.1.0 sequence](docs/images/readme-diagrams/workshop-foundation-sequence.png)

The 0.1.0 walkthrough starts with raw partner input, validates it with
`bluetape-rs-core`, captures request logs with `bluetape-rs-logging`, and writes
test artifacts through `bluetape-rs-test`.

## Repository Layout

```text
examples/
  foundation-order-cleanup/       # validation + normalization
  request-tracing-log-capture/    # correlation-aware log capture
  temp-resource-test-harness/     # temporary filesystem test harness
docs/images/readme-diagrams/      # README diagram sources and rendered PNGs
```

## Verify

```bash
make ci
```

`make ci` runs formatting, Clippy, and all workspace tests with all features
enabled.
