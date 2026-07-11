# AGENTS.md - bluetape-rs-workshop

This repository inherits the workspace guidance from `../AGENTS.md`.
Read and follow the workspace root guide first. This file only adds
Rust-workshop layout, commands, domain rules, and local exceptions.

Runnable Rust backend examples for `bluetape-rs`.

## Skills

- Use `bluetape-workflow` for task classification and issue/PR discipline.
- Use `bluetape-rs-patterns` for Rust implementation, tests, async work, Cargo
  metadata, and review.

## Commands

```bash
make fmt
make clippy
make test
make ci
cargo test --workspace --all-features
```

## Rules

- Keep examples small and runnable as independent crates.
- Prefer Rust-native teaching examples over Kotlin/Go-style API ports.
- Public example APIs should use Rustdoc where it helps readers understand the
  contract.
- Keep `Cargo.toml`, example READMEs, and top-level README tables synchronized
  when examples are added, renamed, or removed.
