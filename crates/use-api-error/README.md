# use-api-error

API error envelope primitives for RustUse.

Models error codes, messages, details, field errors, validation errors, retryability, categories, and protocol-neutral error envelopes.

## Example

```rust
use use_api_error::{ErrorCategory, ErrorCode};

let value = ErrorCode::new("invalid-request").expect("valid API primitive");

assert_eq!(value.as_str(), "invalid-request");
assert_eq!(ErrorCategory::Validation.to_string(), "validation");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No framework response conversion.
- No localization system.
- No exception handling runtime.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license