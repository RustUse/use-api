use use_api::{
    auth, content_negotiation, cursor, deprecation, endpoint, error, graphql, grpc, header,
    idempotency, key, media_type, openapi, operation, pagination, param, rate_limit, request,
    resource, response, rest, route, rpc, schema, version, webhook,
};

#[test]
fn facade_aliases_are_available() -> Result<(), Box<dyn std::error::Error>> {
    let _ = rest::ResourceName::new("users")?;
    let _ = openapi::OperationId::new("listUsers")?;
    let _ = graphql::FieldName::new("viewer")?;
    let _ = grpc::GrpcMethodPath::new("/users.UserService/GetUser")?;
    let _ = rpc::RpcMethodName::new("users.get")?;
    let _ = endpoint::EndpointPath::new("/v1/users")?;
    let _ = route::RouteTemplate::parse("/users/:id")?;
    let _ = version::ApiVersion::new("v1")?;
    let _ = pagination::PageInfo::new(
        pagination::PageNumber::new(1)?,
        pagination::PageSize::new(25)?,
    );
    let _ = cursor::OpaqueCursor::new("opaque")?;
    let _ = rate_limit::RateLimit::new(100, 99);
    let _ = error::ApiError::new(
        error::ErrorCode::new("invalid-request")?,
        error::ErrorMessage::new("Invalid request")?,
    );
    let _ = key::ApiKey::new("sk_live_example_123456")?.redacted();
    let _ = webhook::WebhookEventName::new("user.created")?;
    let _ = idempotency::IdempotencyKey::new("checkout:abc")?;
    let _ = content_negotiation::QualityValue::from_millis(900)?;
    let _ = auth::AuthSchemeName::new("Bearer")?;
    let _ = request::RequestId::new("req_123")?;
    let _ = response::ApiResponse::new(response::ResponseStatusCategory::Success, ());
    let _ = schema::SchemaName::new("User")?;
    let _ = param::PathParamName::new("user_id")?;
    let _ = header::ApiHeaderName::new("X-Request-Id")?;
    let _ = media_type::MediaType::new("application/json")?;
    assert!(deprecation::DeprecationStatus::Active.is_active());
    let _ = resource::ApiResourceName::new("users")?;
    let _ = operation::OperationId::new("listUsers")?;

    Ok(())
}
