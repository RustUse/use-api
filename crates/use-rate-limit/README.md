# use-rate-limit

Rate limit primitives for RustUse.

Models quotas, remaining counts, reset timing labels, retry-after values, buckets, and limit policy labels.

## Example

```rust
use use_rate_limit::{LimitPolicy, BucketName};

let value = BucketName::new("global").expect("valid API primitive");

assert_eq!(value.as_str(), "global");
assert_eq!(LimitPolicy::FixedWindow.to_string(), "fixed-window");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No throttling middleware.
- No clocks or timers.
- No distributed rate-limit store.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license