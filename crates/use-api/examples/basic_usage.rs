use use_api::{error, graphql, grpc, key, openapi, pagination, route, version, webhook};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = version::ApiVersion::new("v1")?;
    let route = route::RouteTemplate::parse("/users/:id")?;
    let page = pagination::PageInfo::new(
        pagination::PageNumber::new(1)?,
        pagination::PageSize::new(25)?,
    )
    .with_has_more(true);
    let key = key::ApiKey::new("sk_live_example_123456")?;
    let error = error::ApiError::new(
        error::ErrorCode::new("invalid-request")?,
        error::ErrorMessage::new("Invalid request")?,
    );
    let event = webhook::WebhookEventName::new("user.created")?;
    let operation = openapi::OperationId::new("listUsers")?;
    let field = graphql::FieldName::new("viewer")?;
    let method = grpc::GrpcMethodPath::new("/users.UserService/GetUser")?;

    assert_eq!(version.kind(), version::VersionKind::Simple);
    assert_eq!(route.segments().len(), 2);
    assert!(page.has_more());
    assert!(key.redacted().contains("..."));
    assert_eq!(error.code().as_str(), "invalid-request");
    assert_eq!(event.as_str(), "user.created");
    assert_eq!(operation.as_str(), "listUsers");
    assert_eq!(field.as_str(), "viewer");
    assert_eq!(method.as_str(), "/users.UserService/GetUser");

    Ok(())
}
