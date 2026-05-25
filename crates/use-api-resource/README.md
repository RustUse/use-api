# use-api-resource

API resource primitives for RustUse.

Models resource names, identifiers, collections, paths, relationships, and actions useful across REST, `GraphQL`, RPC, and `OpenAPI` surfaces.

## Example

```rust
use use_api_resource::{ApiResourceAction, ApiResourceName};

let value = ApiResourceName::new("users").expect("valid API primitive");

assert_eq!(value.as_str(), "users");
assert_eq!(ApiResourceAction::Read.to_string(), "read");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No persistence model.
- No ORM behavior.
- No authorization checks.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license