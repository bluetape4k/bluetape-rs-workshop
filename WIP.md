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
| 0.3.0 | [#9](https://github.com/bluetape4k/bluetape-rs-workshop/issues/9) | [#10](https://github.com/bluetape4k/bluetape-rs-workshop/issues/10), [#11](https://github.com/bluetape4k/bluetape-rs-workshop/issues/11), [#12](https://github.com/bluetape4k/bluetape-rs-workshop/issues/12) | Add persistence-adjacent and async boundary scenarios |
| 0.4.0 | [#13](https://github.com/bluetape4k/bluetape-rs-workshop/issues/13) | [#14](https://github.com/bluetape4k/bluetape-rs-workshop/issues/14), [#15](https://github.com/bluetape4k/bluetape-rs-workshop/issues/15), [#16](https://github.com/bluetape4k/bluetape-rs-workshop/issues/16) | Integrated service-style walkthroughs that combine previous milestones |

## 0.1.0 Delivery Scope

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| [#2](https://github.com/bluetape4k/bluetape-rs-workshop/issues/2) | `foundation-order-cleanup` | `bluetape-rs-core`, `bluetape-rs-logging` | Partner order rows normalize into typed orders with validation and captured logs |
| [#3](https://github.com/bluetape4k/bluetape-rs-workshop/issues/3) | `request-tracing-log-capture` | `bluetape-rs-core`, `bluetape-rs-logging` | Request handling emits correlation-aware logs that tests can assert |
| [#4](https://github.com/bluetape4k/bluetape-rs-workshop/issues/4) | `temp-resource-test-harness` | `bluetape-rs-core`, `bluetape-rs-test` | Temporary workspaces isolate file tests and clean up deterministically |

