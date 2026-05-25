#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for primitive API vocabulary crates.

#[cfg(feature = "rest")]
pub use use_rest as rest;

#[cfg(feature = "openapi")]
pub use use_openapi as openapi;

#[cfg(feature = "graphql")]
pub use use_graphql as graphql;

#[cfg(feature = "grpc")]
pub use use_grpc as grpc;

#[cfg(feature = "rpc")]
pub use use_rpc as rpc;

#[cfg(feature = "endpoint")]
pub use use_endpoint as endpoint;

#[cfg(feature = "route")]
pub use use_api_route as route;

#[cfg(feature = "version")]
pub use use_api_version as version;

#[cfg(feature = "pagination")]
pub use use_pagination as pagination;

#[cfg(feature = "cursor")]
pub use use_cursor as cursor;

#[cfg(feature = "rate-limit")]
pub use use_rate_limit as rate_limit;

#[cfg(feature = "error")]
pub use use_api_error as error;

#[cfg(feature = "key")]
pub use use_api_key as key;

#[cfg(feature = "webhook")]
pub use use_webhook as webhook;

#[cfg(feature = "idempotency")]
pub use use_idempotency as idempotency;

#[cfg(feature = "content-negotiation")]
pub use use_content_negotiation as content_negotiation;

#[cfg(feature = "auth")]
pub use use_api_auth as auth;

#[cfg(feature = "request")]
pub use use_api_request as request;

#[cfg(feature = "response")]
pub use use_api_response as response;

#[cfg(feature = "schema")]
pub use use_api_schema as schema;

#[cfg(feature = "param")]
pub use use_api_param as param;

#[cfg(feature = "header")]
pub use use_api_header as header;

#[cfg(feature = "media-type")]
pub use use_api_media_type as media_type;

#[cfg(feature = "deprecation")]
pub use use_api_deprecation as deprecation;

#[cfg(feature = "resource")]
pub use use_api_resource as resource;

#[cfg(feature = "operation")]
pub use use_api_operation as operation;
