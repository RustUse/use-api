# Contributing

Thanks for helping improve RustUse/use-api.

Keep contributions focused on small API primitive crates. Avoid adding server/client implementations, network access, code generation, or framework-specific behavior to this workspace.

Before opening a pull request, run:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```