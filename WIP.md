# bluetape-rs-workshop WIP

This repository teaches `bluetape-rs` through runnable examples. Milestones grow
from isolated foundation examples into integrated backend scenarios.

## Branch Policy

- `main` is the only long-lived branch.
- Feature work opens pull requests into `main`.
- Each PR should reference the relevant milestone issues and keep examples
  runnable with `make ci`.

## Milestone Roadmap

| Milestone | Epic | Examples | Integration Level |
|---|---:|---|---|
| 0.1.0 | [#1](https://github.com/bluetape4k/bluetape-rs-workshop/issues/1) | [#2](https://github.com/bluetape4k/bluetape-rs-workshop/issues/2), [#3](https://github.com/bluetape4k/bluetape-rs-workshop/issues/3), [#4](https://github.com/bluetape4k/bluetape-rs-workshop/issues/4) | Foundation examples for validation, logging, and test helpers |
| 0.2.0 | [#5](https://github.com/bluetape4k/bluetape-rs-workshop/issues/5) | [#6](https://github.com/bluetape4k/bluetape-rs-workshop/issues/6), [#7](https://github.com/bluetape4k/bluetape-rs-workshop/issues/7), [#8](https://github.com/bluetape4k/bluetape-rs-workshop/issues/8) | Compose 0.1.0 helpers around request and worker flows |
| 0.3.0 | [#9](https://github.com/bluetape4k/bluetape-rs-workshop/issues/9) | [#10](https://github.com/bluetape4k/bluetape-rs-workshop/issues/10), [#11](https://github.com/bluetape4k/bluetape-rs-workshop/issues/11), [#12](https://github.com/bluetape4k/bluetape-rs-workshop/issues/12) | Add codec boundary scenarios for tokens, support references, and text ingest |
| 0.4.0 | [#13](https://github.com/bluetape4k/bluetape-rs-workshop/issues/13) | [#14](https://github.com/bluetape4k/bluetape-rs-workshop/issues/14), [#15](https://github.com/bluetape4k/bluetape-rs-workshop/issues/15), [#16](https://github.com/bluetape4k/bluetape-rs-workshop/issues/16) | Integrated service-style walkthroughs that combine previous milestones |

## 0.1.0 Delivery Scope

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| [#2](https://github.com/bluetape4k/bluetape-rs-workshop/issues/2) | `foundation-order-cleanup` | `bluetape-rs-core`, `bluetape-rs-logging` | Partner order rows normalize into typed orders with validation and captured logs |
| [#3](https://github.com/bluetape4k/bluetape-rs-workshop/issues/3) | `request-tracing-log-capture` | `bluetape-rs-core`, `bluetape-rs-logging` | Request handling emits correlation-aware logs that tests can assert |
| [#4](https://github.com/bluetape4k/bluetape-rs-workshop/issues/4) | `temp-resource-test-harness` | `bluetape-rs-core`, `bluetape-rs-test` | Temporary workspaces isolate file tests and clean up deterministically |

## 0.2.0 Delivery Scope

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| [#6](https://github.com/bluetape4k/bluetape-rs-workshop/issues/6) | `catalog-enrichment-fanout` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections`, `bluetape-rs-async` | Bounded provider fan-out handles required provider failures, optional warnings, timeout control, and deterministic paging |
| [#7](https://github.com/bluetape4k/bluetape-rs-workshop/issues/7) | `batched-order-windowing` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections` | Partner order events are validated, grouped, chunked, sorted, and returned through explicit page metadata |
| [#8](https://github.com/bluetape4k/bluetape-rs-workshop/issues/8) | `shutdown-aware-worker` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections`, `bluetape-rs-async` | Worker runs grouped work with typed timeout and shutdown cancellation behavior |

## 0.3.0 Delivery Scope

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| [#10](https://github.com/bluetape4k/bluetape-rs-workshop/issues/10) | `invitation-codecs` | `bluetape-rs-core`, `bluetape-rs-codec` | Invitation artifacts validate inputs, round-trip URL-safe token and callback state, reject malformed input, and expose Base58 support references |
| [#11](https://github.com/bluetape4k/bluetape-rs-workshop/issues/11) | `support-reference-encoding` | `bluetape-rs-core`, `bluetape-rs-codec`, `bluetape-rs-collections` | Support references batch and decode with explicit malformed-reference handling |
| [#12](https://github.com/bluetape4k/bluetape-rs-workshop/issues/12) | `text-boundary-ingest` | `bluetape-rs-core`, `bluetape-rs-codec`, `bluetape-rs-async` | Text ingest separates valid UTF-8, lossy display, and downstream rejection behavior |
