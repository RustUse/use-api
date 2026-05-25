# use-rpc

Generic RPC primitives for RustUse.

Provides protocol-neutral method names, request identifiers, request and response envelopes, error envelopes, and procedure labels.

## Example

```rust
use use_rpc::{ProcedureKind, RpcMethodName};

let value = RpcMethodName::new("users.get").expect("valid API primitive");

assert_eq!(value.as_str(), "users.get");
assert_eq!(ProcedureKind::Method.to_string(), "method");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No JSON-RPC implementation.
- No transport binding.
- No server or client runtime.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license