# use-api-operation

API operation primitives for RustUse.

Models operation identifiers, names, summaries, kinds, status labels, and lifecycle labels shared across `OpenAPI`, RPC, `GraphQL`, and REST-style APIs.

## Example

```rust
use use_api_operation::{OperationKind, OperationId};

let value = OperationId::new("listUsers").expect("valid API primitive");

assert_eq!(value.as_str(), "listUsers");
assert_eq!(OperationKind::Query.to_string(), "query");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No operation execution.
- No code generation.
- No API gateway behavior.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license