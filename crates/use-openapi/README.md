# use-openapi

`OpenAPI` document primitives for RustUse.

Provides lightweight typed labels for paths, operations, parameters, responses, schemas, tags, and components without parsing or generating full `OpenAPI` documents.

## Example

```rust
use use_openapi::{OpenApiOperationKind, OpenApiPath};

let value = OpenApiPath::new("listUsers").expect("valid API primitive");

assert_eq!(value.as_str(), "listUsers");
assert_eq!(OpenApiOperationKind::Get.to_string(), "get");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No full `OpenAPI` parser.
- No document generation.
- No schema validation engine.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license