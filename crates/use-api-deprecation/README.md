# use-api-deprecation

API deprecation primitives for RustUse.

Models deprecation status, sunset date strings, replacement endpoints, migration notes, and warning labels with simple state helpers.

## Example

```rust
use use_api_deprecation::{DeprecationStatus, SunsetDate};

let value = SunsetDate::new("2026-05-25").expect("valid API primitive");

assert_eq!(value.as_str(), "2026-05-25");
assert_eq!(DeprecationStatus::Active.to_string(), "active");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No calendar math.
- No automatic migration.
- No policy enforcement.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license