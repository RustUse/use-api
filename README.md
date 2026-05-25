# RustUse/use-api

`use-api` is a RustUse facade workspace for small, focused API primitive crates. It models common API concepts, styles, contracts, identifiers, labels, and metadata without becoming an application framework.

It is not a web framework, server framework, HTTP client, client library, OpenAPI generator, GraphQL engine, or gRPC implementation. It avoids network calls, transport behavior, code generation, and heavy protocol dependencies.

## Sibling relationships

- `use-rest` models REST-style primitives without claiming full REST compliance.
- `use-openapi` models API description and specification primitives without parsing or generating full OpenAPI documents.
- `use-graphql` models GraphQL naming, schema, and query vocabulary without parsing or executing GraphQL.
- `use-grpc` models gRPC service, method, status, and streaming metadata without transport or Protobuf behavior.
- `use-rpc` models generic RPC primitives in a protocol-neutral way.

`use-http` remains under the RustUse `use-web` set. `use-api` may interoperate with HTTP primitives later, but it should not duplicate all HTTP behavior. `use-protobuf` should remain separate from `use-grpc`.

## Workspace crates

| Crate                     | Path                              | Purpose                                                                               |
| ------------------------- | --------------------------------- | ------------------------------------------------------------------------------------- |
| `use-api`                 | `crates/use-api/`                 | Feature-gated facade over the focused API primitive crates                            |
| `use-rest`                | `crates/use-rest/`                | REST-style resources, collections, representations, actions, and constraint labels    |
| `use-openapi`             | `crates/use-openapi/`             | OpenAPI path, operation, parameter, response, schema, tag, and component primitives   |
| `use-graphql`             | `crates/use-graphql/`             | GraphQL names, operation kinds, directives, arguments, and variables                  |
| `use-grpc`                | `crates/use-grpc/`                | gRPC service names, method paths, metadata keys, status codes, and streaming modes    |
| `use-rpc`                 | `crates/use-rpc/`                 | Protocol-neutral RPC method names, request IDs, envelopes, and procedure kinds        |
| `use-endpoint`            | `crates/use-endpoint/`            | Endpoint names, paths, identifiers, groups, and status labels                         |
| `use-api-route`           | `crates/use-api-route/`           | API route templates and segment metadata; re-exported by the facade as `route`        |
| `use-api-version`         | `crates/use-api-version/`         | Simple, date-shaped, semantic-looking, and custom API version labels                  |
| `use-pagination`          | `crates/use-pagination/`          | Page numbers, page sizes, markers, and page metadata                                  |
| `use-cursor`              | `crates/use-cursor/`              | Opaque cursor and before/after/next/previous cursor wrappers                          |
| `use-rate-limit`          | `crates/use-rate-limit/`          | Quotas, remaining counts, retry-after values, buckets, and policy labels              |
| `use-api-error`           | `crates/use-api-error/`           | Error codes, messages, field errors, categories, retryability, and envelopes          |
| `use-api-key`             | `crates/use-api-key/`             | API key identifiers, prefixes, safe redaction helpers, metadata, and status labels    |
| `use-webhook`             | `crates/use-webhook/`             | Webhook event names, endpoint URL wrappers, delivery IDs, attempts, and statuses      |
| `use-idempotency`         | `crates/use-idempotency/`         | Idempotency keys, scopes, replay status, fingerprints, and conflict labels            |
| `use-content-negotiation` | `crates/use-content-negotiation/` | Accepted media types, quality values, content, language, and encoding preferences     |
| `use-api-auth`            | `crates/use-api-auth/`            | Auth scheme names, bearer metadata, API key locations, OAuth scopes, and permissions  |
| `use-api-request`         | `crates/use-api-request/`         | Request IDs, correlation IDs, trace IDs, timestamps, sources, contexts, and envelopes |
| `use-api-response`        | `crates/use-api-response/`        | Response envelopes, status categories, metadata, timing labels, page info, and links  |
| `use-api-schema`          | `crates/use-api-schema/`          | Schema names, fields, field kinds, requirement markers, nullability, and shape labels |
| `use-api-param`           | `crates/use-api-param/`           | Path, query, header, and body parameter names, locations, requirements, and styles    |
| `use-api-header`          | `crates/use-api-header/`          | Common API, custom, correlation, auth, idempotency, and rate-limit header names       |
| `use-api-media-type`      | `crates/use-api-media-type/`      | API media types, subtypes, suffixes, charsets, and common media type labels           |
| `use-api-deprecation`     | `crates/use-api-deprecation/`     | Deprecation status, sunset date strings, replacement endpoints, notes, and warnings   |
| `use-api-resource`        | `crates/use-api-resource/`        | Resource names, IDs, collections, paths, relationships, and actions                   |
| `use-api-operation`       | `crates/use-api-operation/`       | Operation IDs, names, summaries, kinds, statuses, and lifecycle labels                |

## Installation

Use the workspace directly or depend on a Git revision until the first crates.io release is published.

```toml
[dependencies]
use-api = { git = "https://github.com/RustUse/use-api", rev = "<commit>" }
```

After publication, choose the narrowest focused crate that matches your use case or use the facade when one dependency is more convenient.

```toml
[dependencies]
use-api = "0.0.1"
```

## Basic usage

```rust
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
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

`use-api` keeps focused crates small, dependency-light, deterministic, and framework-free. The workspace favors validated newtypes, explicit error enums, stable display labels, small builders, and conservative parsing helpers.

## Non-goals

- No web framework or server framework.
- No HTTP client or server implementation.
- No OpenAPI generator or full parser.
- No GraphQL parser or execution engine.
- No gRPC transport or Protobuf implementation.
- No code generation.
- No network access.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
