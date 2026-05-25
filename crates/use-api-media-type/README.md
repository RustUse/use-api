# use-api-media-type

API media type primitives for RustUse.

Models media types, subtypes, suffixes, charsets, structured syntax suffixes, and common API media type labels with conservative parsing.

## Example

```rust
use use_api_media_type::{CommonApiMediaType, MediaType};

let value = MediaType::new("application/json").expect("valid API primitive");

assert_eq!(value.as_str(), "application/json");
assert_eq!(CommonApiMediaType::Json.to_string(), "json");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No full MIME registry.
- No content negotiation engine.
- No payload parser.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license