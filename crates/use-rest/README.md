# use-rest

REST-style API primitives for RustUse.

Models resource names, collection labels, representation metadata, and REST-style constraint vocabulary without claiming full REST compliance.

## Example

```rust
use use_rest::{RestAction, ResourceName};

let value = ResourceName::new("users").expect("valid API primitive");

assert_eq!(value.as_str(), "users");
assert_eq!(RestAction::List.to_string(), "list");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No HTTP server or client.
- No resource persistence.
- No full REST compliance framework.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license