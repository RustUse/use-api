# use-api-key

API key primitives for RustUse.

Models key identifiers, prefixes, safe redaction, key metadata, and key status labels without generating cryptographic secrets.

## Example

```rust
use use_api_key::{ApiKeyStatus, ApiKeyId};

let value = ApiKeyId::new("sk_live_example").expect("valid API primitive");

assert_eq!(value.as_str(), "sk_live_example");
assert_eq!(ApiKeyStatus::Active.to_string(), "active");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No key generation.
- No cryptographic storage.
- No authentication checks.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license