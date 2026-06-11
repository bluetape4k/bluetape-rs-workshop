# temp-resource-test-harness

This example uses `bluetape-rs-test` temporary directories to verify filesystem
side effects without leaking state between tests.

## Scenario

File-producing code should be testable without writing into a shared repository
path. The example validates the scratch file name, writes rows into a temporary
workspace, and closes the workspace to prove cleanup.

![temp-resource-test-harness flow](../../docs/images/readme-diagrams/example-temp-resource-test-harness.png)

## Representative Code

```rust
let workspace = write_scratch_rows(
    ScratchConfig {
        prefix: "order-import".to_owned(),
        file_name: "orders.csv".to_owned(),
    },
    &["id,sku", "ord-1,sku-1"],
)?;

assert!(workspace.file_path().exists());
assert_eq!(workspace.row_count(), 2);
workspace.close()?;
```

## What To Notice

- `TempDir::new(prefix)` creates an isolated workspace for one test.
- File names are validated to prevent absolute paths or `..` traversal.
- `close()` removes the temporary workspace deterministically.

## Run

```bash
cargo test -p temp-resource-test-harness
```
