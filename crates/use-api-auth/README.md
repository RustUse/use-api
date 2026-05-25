# use-api-auth

API authentication metadata primitives for RustUse.

Models auth scheme names, bearer token metadata labels, basic auth markers, API key locations, OAuth scope labels, and permission scopes without authenticating anything.

## Example

```rust
use use_api_auth::{ApiKeyLocation, AuthSchemeName};

let value = AuthSchemeName::new("Bearer").expect("valid API primitive");

assert_eq!(value.as_str(), "Bearer");
assert_eq!(ApiKeyLocation::Header.to_string(), "header");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No token validation.
- No password handling.
- No authorization engine.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license