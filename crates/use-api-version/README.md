# use-api-version

API version primitives for RustUse.

Supports simple labels such as `v1`, date-based labels such as `2026-05-25`, and semantic-looking labels without imposing a release policy.

## Example

```rust
use use_api_version::{VersionKind, ApiVersion};

let value = ApiVersion::new("v1").expect("valid API primitive");

assert_eq!(value.as_str(), "v1");
assert_eq!(VersionKind::Simple.to_string(), "simple");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No release automation.
- No semver compatibility guarantees.
- No date validation beyond shape checks.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license