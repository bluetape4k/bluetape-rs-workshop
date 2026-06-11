# Milestone 0.1.0 Review Evidence

## Scope

- Added the initial `bluetape-rs-workshop` Rust workspace.
- Implemented milestone 0.1.0 examples:
  - `foundation-order-cleanup`
  - `request-tracing-log-capture`
  - `temp-resource-test-harness`
- Added `WIP.md`, bilingual README files, and README diagram assets.

## 7-Tier Review

| Tier | Result | Evidence |
|---|---|---|
| Functional correctness | Pass | 9 unit tests cover success, failure, and boundary cases across the three examples. |
| API compatibility | Pass | Examples compile against published `bluetape-rs-*` `0.3.1` crates from crates.io. |
| Rust idioms | Pass | Uses typed errors, `Result`, owned output models, no `unsafe`, no panics in library code. |
| Async/concurrency | Pass | No async or shared mutable concurrency surface in 0.1.0 examples. |
| Testing discipline | Pass | `cargo test --workspace --all-features` passes, including doctest harnesses. |
| Documentation | Pass | `README.md`, `README.ko.md`, and per-example README files describe run commands and scenario intent. |
| Diagram gate | Pass | `scripts/generate-foundation-diagrams.py` regenerates final SVG/PNG pairs, Graphviz `.dot/.plain/*-graphviz.svg/*-graphviz.png` evidence, and `docs/images/readme-diagrams/geometry-summary.txt`. |
| CI/release readiness | Pass | GitHub CI runs fmt, clippy, and all-features tests for PRs targeting `main`. |

## Validation

```text
make ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Result: passed locally on 2026-06-11.

Diagram validation:

```text
python3 scripts/generate-foundation-diagrams.py
xmllint --noout docs/images/readme-diagrams/*.svg
rg 'Inter|Arial|Helvetica' docs/images/readme-diagrams/*.svg
rg 'docs/images/readme-diagrams/.*\.svg' README.md README.ko.md examples/*/README*.md
```

Result: generator completed, SVG parsing passed, forbidden UI fonts absent, and README files embed PNG only.

## Follow-up Defect

The first generated example flow diagrams kept cards too close together, which
made connector stems hard to see at README scale. The fix was not only to
increase canvas height and card spacing, but also to promote the defect into
`scripts/generate-foundation-diagrams.py`:

- vertical example-flow card spacing uses a larger step
- `geometry-summary.txt` records `shortConnectors` and `minConnectorStem`
- the generator fails when a direct vertical connector stem is shorter than 28px

## Risk

- P0: 0
- P1: 0
- Residual risk: diagram rendering depends on committed PNG assets for GitHub README display; SVG and Graphviz sources are committed for regeneration.
