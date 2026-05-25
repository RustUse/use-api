# use-api-route

API route template primitives for RustUse.

Models route templates and route segment metadata for static, dynamic, wildcard, and optional segments without implementing a full router.

## Example

```rust
use use_api_route::{RouteSegmentKind, RouteTemplate};

let value = RouteTemplate::new("/users/:id").expect("valid API primitive");

assert_eq!(value.as_str(), "/users/:id");
assert_eq!(RouteSegmentKind::Static.to_string(), "static");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No full router.
- No regex matching engine.
- No middleware or server integration.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license