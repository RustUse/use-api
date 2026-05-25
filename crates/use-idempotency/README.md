# use-idempotency

Idempotency primitives for RustUse.

Models idempotency keys, scopes, replay status, request fingerprint labels, and conflict status metadata.

## Example

```rust
use use_idempotency::{ReplayStatus, IdempotencyKey};

let value = IdempotencyKey::new("checkout:abc123").expect("valid API primitive");

assert_eq!(value.as_str(), "checkout:abc123");
assert_eq!(ReplayStatus::Original.to_string(), "original");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No storage backend.
- No request hashing.
- No distributed lock implementation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license