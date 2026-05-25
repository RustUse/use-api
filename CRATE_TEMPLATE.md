# RustUse API Crate Template

Use this checklist when adding a focused crate to the `use-api` workspace.

## Checklist

- Keep package metadata inherited from the workspace wherever possible.
- Prefer no dependencies for primitive vocabulary crates.
- Use README-driven crate docs with `#![doc = include_str!("../README.md")]`.
- Use explicit error enums instead of stringly errors.
- Add unit tests for accepted and rejected values.
- Keep server/client behavior, network access, code generation, and framework integration out of scope.

## Validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```