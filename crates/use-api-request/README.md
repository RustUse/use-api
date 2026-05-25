# use-api-request

API request metadata primitives for RustUse.

Models request IDs, correlation IDs, trace IDs, timestamp labels, sources, contexts, and protocol-neutral request envelopes.

## Example

```rust
use use_api_request::{RequestContextKind, RequestId};

let value = RequestId::new("req_123").expect("valid API primitive");

assert_eq!(value.as_str(), "req_123");
assert_eq!(RequestContextKind::User.to_string(), "user");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No HTTP parsing.
- No clock access.
- No tracing backend integration.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license