# use-content-negotiation

Content negotiation primitives for RustUse.

Models accepted media types, quality values, content preferences, language preferences, and encoding preferences with conservative parsing.

## Example

```rust
use use_content_negotiation::{PreferenceKind, AcceptedMediaType};

let value = AcceptedMediaType::new("application/json").expect("valid API primitive");

assert_eq!(value.as_str(), "application/json");
assert_eq!(PreferenceKind::MediaType.to_string(), "media-type");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No full `Accept` parser.
- No locale matching engine.
- No content transformation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license