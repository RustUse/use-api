#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when API primitive text or labels are invalid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiPrimitiveError {
    /// The supplied value was empty after trimming.
    Empty,
    /// The supplied value used syntax this crate rejects.
    Invalid,
    /// The supplied label was not recognized.
    Unknown,
}

impl fmt::Display for ApiPrimitiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("API primitive value cannot be empty"),
            Self::Invalid => formatter.write_str("invalid API primitive value"),
            Self::Unknown => formatter.write_str("unknown API primitive label"),
        }
    }
}

impl Error for ApiPrimitiveError {}

fn validate_api_text(value: &str) -> Result<&str, ApiPrimitiveError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiPrimitiveError::Empty);
    }
    if trimmed.chars().any(char::is_control) {
        return Err(ApiPrimitiveError::Invalid);
    }
    Ok(trimmed)
}

macro_rules! text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates validated text metadata.
            ///
            /// # Errors
            ///
            /// Returns [ApiPrimitiveError] when the value is empty or contains control characters.
            pub fn new(value: impl AsRef<str>) -> Result<Self, ApiPrimitiveError> {
                validate_api_text(value.as_ref()).map(|value| Self(value.to_owned()))
            }

            /// Parses validated text metadata.
            ///
            /// # Errors
            ///
            /// Returns [ApiPrimitiveError] when validation fails.
            pub fn parse(value: impl AsRef<str>) -> Result<Self, ApiPrimitiveError> {
                Self::new(value)
            }

            /// Returns the stored text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the value and returns the stored text.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = ApiPrimitiveError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ApiPrimitiveError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

text_newtype!(RpcMethodName);
text_newtype!(RpcRequestId);
text_newtype!(RpcErrorCode);
text_newtype!(RpcErrorMessage);

/// Protocol-neutral procedure kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProcedureKind {
    /// A stable label variant.
    Method,
    /// A stable label variant.
    Notification,
    /// A stable label variant.
    Query,
    /// A stable label variant.
    Command,
}

impl ProcedureKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Method => "method",
            Self::Notification => "notification",
            Self::Query => "query",
            Self::Command => "command",
        }
    }
}

impl Default for ProcedureKind {
    fn default() -> Self {
        Self::Method
    }
}

impl fmt::Display for ProcedureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ProcedureKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "method" => Ok(Self::Method),
            "notification" => Ok(Self::Notification),
            "query" => Ok(Self::Query),
            "command" => Ok(Self::Command),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// Protocol-neutral response status labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RpcResponseStatus {
    /// A stable label variant.
    Success,
    /// A stable label variant.
    Error,
}

impl RpcResponseStatus {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}

impl Default for RpcResponseStatus {
    fn default() -> Self {
        Self::Success
    }
}

impl fmt::Display for RpcResponseStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RpcResponseStatus {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: RpcMethodName,
    kind: ProcedureKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: RpcMethodName, kind: ProcedureKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &RpcMethodName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> ProcedureKind {
        self.kind
    }
}

/// Protocol-neutral RPC request envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestEnvelope<T> {
    id: RpcRequestId,
    method: RpcMethodName,
    body: T,
}

impl<T> RequestEnvelope<T> {
    /// Creates a request envelope.
    #[must_use]
    pub const fn new(id: RpcRequestId, method: RpcMethodName, body: T) -> Self {
        Self { id, method, body }
    }

    /// Returns the request ID.
    #[must_use]
    pub const fn id(&self) -> &RpcRequestId {
        &self.id
    }

    /// Returns the method name.
    #[must_use]
    pub const fn method(&self) -> &RpcMethodName {
        &self.method
    }

    /// Returns the request body.
    #[must_use]
    pub const fn body(&self) -> &T {
        &self.body
    }
}

/// Protocol-neutral RPC response envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseEnvelope<T> {
    id: RpcRequestId,
    status: RpcResponseStatus,
    body: T,
}

impl<T> ResponseEnvelope<T> {
    /// Creates a response envelope.
    #[must_use]
    pub const fn new(id: RpcRequestId, status: RpcResponseStatus, body: T) -> Self {
        Self { id, status, body }
    }

    /// Returns the response status.
    #[must_use]
    pub const fn status(&self) -> RpcResponseStatus {
        self.status
    }
}

/// Protocol-neutral RPC error envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorEnvelope {
    code: RpcErrorCode,
    message: RpcErrorMessage,
}

impl ErrorEnvelope {
    /// Creates an error envelope.
    #[must_use]
    pub const fn new(code: RpcErrorCode, message: RpcErrorMessage) -> Self {
        Self { code, message }
    }

    /// Returns the error code.
    #[must_use]
    pub const fn code(&self) -> &RpcErrorCode {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = RpcMethodName::new("users.get")?;

        assert_eq!(value.as_str(), "users.get");
        assert_eq!(value.to_string(), "users.get");
        assert_eq!("users.get".parse::<RpcMethodName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(RpcMethodName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "method".parse::<ProcedureKind>()?;

        assert_eq!(kind, ProcedureKind::Method);
        assert_eq!(kind.to_string(), "method");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(RpcMethodName::new("users.get")?, ProcedureKind::default());

        assert_eq!(metadata.name().as_str(), "users.get");
        assert_eq!(metadata.kind(), ProcedureKind::default());
        Ok(())
    }
}
