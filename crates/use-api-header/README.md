# use-api-header

API header primitives for RustUse.

Models common API header names, custom header names, correlation headers, auth headers, idempotency headers, and rate-limit headers without duplicating all HTTP behavior.

## Example

```rust
use use_api_header::{CommonApiHeader, ApiHeaderName};

let value = ApiHeaderName::new("X-Request-Id").expect("valid API primitive");

assert_eq!(value.as_str(), "X-Request-Id");
assert_eq!(CommonApiHeader::Authorization.to_string(), "authorization");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No full HTTP header registry.
- No header value parser.
- No transport integration.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license