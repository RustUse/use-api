# use-endpoint

Endpoint primitives for RustUse.

Models endpoint names, paths, identifiers, groups, and status labels with validation and display helpers.

## Example

```rust
use use_endpoint::{EndpointStatus, EndpointName};

let value = EndpointName::new("/v1/users").expect("valid API primitive");

assert_eq!(value.as_str(), "/v1/users");
assert_eq!(EndpointStatus::Active.to_string(), "active");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No HTTP route registration.
- No request handling.
- No service discovery.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license