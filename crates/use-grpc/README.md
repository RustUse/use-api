# use-grpc

`gRPC` service and method primitives for RustUse.

Models service names, method names, fully qualified method paths, metadata keys, status codes, and streaming mode labels without `gRPC` transport or `Protobuf` handling.

## Example

```rust
use use_grpc::{GrpcStatusCode, ServiceName};

let value = ServiceName::new("/users.UserService/GetUser").expect("valid API primitive");

assert_eq!(value.as_str(), "/users.UserService/GetUser");
assert_eq!(GrpcStatusCode::Ok.to_string(), "ok");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No `gRPC` transport.
- No `Protobuf` implementation.
- No client or server runtime.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license