# use-api-param

API parameter primitives for RustUse.

Models path, query, header, and body parameter names plus location, requirement, and style labels.

## Example

```rust
use use_api_param::{ParamLocation, PathParamName};

let value = PathParamName::new("user_id").expect("valid API primitive");

assert_eq!(value.as_str(), "user_id");
assert_eq!(ParamLocation::Path.to_string(), "path");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No request parser.
- No framework binding.
- No schema validation.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license