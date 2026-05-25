# use-cursor

Cursor pagination primitives for RustUse.

Models opaque cursors and before, after, next, and previous cursor slots without assuming serialization or secrecy.

## Example

```rust
use use_cursor::{CursorDirection, OpaqueCursor};

let value = OpaqueCursor::new("opaque-cursor").expect("valid API primitive");

assert_eq!(value.as_str(), "opaque-cursor");
assert_eq!(CursorDirection::Forward.to_string(), "forward");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No secret encoding.
- No cursor signing.
- No storage backend assumptions.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license