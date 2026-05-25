# use-pagination

Pagination primitives for RustUse.

Provides page number, page size, limit, offset, total count, marker, and page metadata helpers.

## Example

```rust
use use_pagination::{PaginationDirection, PageMarker};

let value = PageMarker::new("page-1").expect("valid API primitive");

assert_eq!(value.as_str(), "page-1");
assert_eq!(PaginationDirection::Next.to_string(), "next");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No database integration.
- No query execution.
- No cursor encoding.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license