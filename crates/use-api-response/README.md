# use-api-response

API response metadata primitives for RustUse.

Models response envelopes, status categories, metadata labels, timing labels, page info, and response links without protocol-specific transport behavior.

## Example

```rust
use use_api_response::{ResponseStatusCategory, ResponseMetadata};

let value = ResponseMetadata::new("response-link").expect("valid API primitive");

assert_eq!(value.as_str(), "response-link");
assert_eq!(ResponseStatusCategory::Informational.to_string(), "informational");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No HTTP response serialization.
- No template rendering.
- No streaming body implementation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license