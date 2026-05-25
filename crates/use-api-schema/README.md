# use-api-schema

API schema primitives for RustUse.

Models schema names, schema fields, field kinds, required or optional markers, nullable markers, and broad schema shape labels without becoming a full schema system.

## Example

```rust
use use_api_schema::{FieldKind, SchemaName};

let value = SchemaName::new("User").expect("valid API primitive");

assert_eq!(value.as_str(), "User");
assert_eq!(FieldKind::String.to_string(), "string");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No JSON Schema implementation.
- No `OpenAPI` schema generator.
- No `GraphQL` schema engine.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license