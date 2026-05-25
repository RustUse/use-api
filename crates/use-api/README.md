# use-api

`use-api` is a RustUse facade crate for small, focused API primitive crates. It models common API concepts, styles, contracts, identifiers, labels, and metadata without becoming an application framework.

It is not a web framework, server framework, HTTP client, client library, OpenAPI generator, GraphQL engine, or gRPC implementation. It avoids network calls, transport behavior, code generation, and heavy protocol dependencies.

## Sibling relationships

- `use-rest` models REST-style primitives without claiming full REST compliance.
- `use-openapi` models API description and specification primitives without parsing or generating full OpenAPI documents.
- `use-graphql` models GraphQL naming, schema, and query vocabulary without parsing or executing GraphQL.
- `use-grpc` models gRPC service, method, status, and streaming metadata without transport or Protobuf behavior.
- `use-rpc` models generic RPC primitives in a protocol-neutral way.

`use-http` remains under the RustUse `use-web` set. `use-api` may interoperate with HTTP primitives later, but it should not duplicate all HTTP behavior. `use-protobuf` should remain separate from `use-grpc`.

## Basic usage

```rust
# #[cfg(feature = "full")]
# {
use use_api::{deprecation, error, graphql, grpc, key, openapi, pagination, route, version, webhook};

let api_version = version::ApiVersion::new("v1")?;
let route = route::RouteTemplate::parse("/users/:id")?;
let page = pagination::PageInfo::new(
    pagination::PageNumber::new(1)?,
    pagination::PageSize::new(25)?,
)
.with_has_more(true);
let api_key = key::ApiKey::new("sk_live_example_123456")?;
let api_error = error::ApiError::new(
    error::ErrorCode::new("invalid-request")?,
    error::ErrorMessage::new("Invalid request")?,
)
.with_category(error::ErrorCategory::Validation);
let event = webhook::WebhookEventName::new("user.created")?;
let operation_id = openapi::OperationId::new("listUsers")?;
let field = graphql::FieldName::new("viewer")?;
let method = grpc::GrpcMethodPath::new("/users.UserService/GetUser")?;
let status = deprecation::DeprecationStatus::Active;

assert_eq!(api_version.kind(), version::VersionKind::Simple);
assert_eq!(route.segments().len(), 2);
assert!(page.has_more());
assert!(api_key.redacted().contains("..."));
assert!(api_error.code().as_str().contains("invalid"));
assert_eq!(event.as_str(), "user.created");
assert_eq!(operation_id.as_str(), "listUsers");
assert_eq!(field.as_str(), "viewer");
assert_eq!(method.as_str(), "/users.UserService/GetUser");
assert!(status.is_active());
# }
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Facade aliases

The facade re-exports child crates with explicit aliases such as `rest`, `openapi`, `graphql`, `grpc`, `rpc`, `endpoint`, `route`, `version`, `pagination`, `cursor`, `rate_limit`, `error`, `key`, `webhook`, `idempotency`, `content_negotiation`, `auth`, `request`, `response`, `schema`, `param`, `header`, `media_type`, `deprecation`, `resource`, and `operation`.

## Non-goals

- No web framework or server framework.
- No HTTP client or server implementation.
- No OpenAPI generator or full parser.
- No GraphQL parser or execution engine.
- No gRPC transport or Protobuf implementation.
- No code generation.
- No network access.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
