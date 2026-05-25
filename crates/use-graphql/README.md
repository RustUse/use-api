# use-graphql

`GraphQL` naming and schema primitives for RustUse.

Models `GraphQL` names, operation labels, directives, arguments, and variables without parsing queries or executing schemas.

## Example

```rust
use use_graphql::{GraphqlOperationKind, GraphqlName};

let value = GraphqlName::new("viewer").expect("valid API primitive");

assert_eq!(value.as_str(), "viewer");
assert_eq!(GraphqlOperationKind::Query.to_string(), "query");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No `GraphQL` parser.
- No execution engine.
- No transport integration.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license