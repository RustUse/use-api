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

text_newtype!(ServiceName);
text_newtype!(MethodName);
text_newtype!(GrpcMethodPath);
text_newtype!(MetadataKey);

/// `gRPC` status code labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GrpcStatusCode {
    /// A stable label variant.
    Ok,
    /// A stable label variant.
    Cancelled,
    /// A stable label variant.
    Unknown,
    /// A stable label variant.
    InvalidArgument,
    /// A stable label variant.
    DeadlineExceeded,
    /// A stable label variant.
    NotFound,
    /// A stable label variant.
    AlreadyExists,
    /// A stable label variant.
    PermissionDenied,
    /// A stable label variant.
    ResourceExhausted,
    /// A stable label variant.
    FailedPrecondition,
    /// A stable label variant.
    Aborted,
    /// A stable label variant.
    OutOfRange,
    /// A stable label variant.
    Unimplemented,
    /// A stable label variant.
    Internal,
    /// A stable label variant.
    Unavailable,
    /// A stable label variant.
    DataLoss,
    /// A stable label variant.
    Unauthenticated,
}

impl GrpcStatusCode {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
            Self::InvalidArgument => "invalid-argument",
            Self::DeadlineExceeded => "deadline-exceeded",
            Self::NotFound => "not-found",
            Self::AlreadyExists => "already-exists",
            Self::PermissionDenied => "permission-denied",
            Self::ResourceExhausted => "resource-exhausted",
            Self::FailedPrecondition => "failed-precondition",
            Self::Aborted => "aborted",
            Self::OutOfRange => "out-of-range",
            Self::Unimplemented => "unimplemented",
            Self::Internal => "internal",
            Self::Unavailable => "unavailable",
            Self::DataLoss => "data-loss",
            Self::Unauthenticated => "unauthenticated",
        }
    }
}

impl Default for GrpcStatusCode {
    fn default() -> Self {
        Self::Ok
    }
}

impl fmt::Display for GrpcStatusCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GrpcStatusCode {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "ok" => Ok(Self::Ok),
            "cancelled" => Ok(Self::Cancelled),
            "unknown" => Ok(Self::Unknown),
            "invalid-argument" => Ok(Self::InvalidArgument),
            "deadline-exceeded" => Ok(Self::DeadlineExceeded),
            "not-found" => Ok(Self::NotFound),
            "already-exists" => Ok(Self::AlreadyExists),
            "permission-denied" => Ok(Self::PermissionDenied),
            "resource-exhausted" => Ok(Self::ResourceExhausted),
            "failed-precondition" => Ok(Self::FailedPrecondition),
            "aborted" => Ok(Self::Aborted),
            "out-of-range" => Ok(Self::OutOfRange),
            "unimplemented" => Ok(Self::Unimplemented),
            "internal" => Ok(Self::Internal),
            "unavailable" => Ok(Self::Unavailable),
            "data-loss" => Ok(Self::DataLoss),
            "unauthenticated" => Ok(Self::Unauthenticated),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// `gRPC` streaming mode labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StreamingMode {
    /// A stable label variant.
    Unary,
    /// A stable label variant.
    ClientStreaming,
    /// A stable label variant.
    ServerStreaming,
    /// A stable label variant.
    BidirectionalStreaming,
}

impl StreamingMode {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unary => "unary",
            Self::ClientStreaming => "client-streaming",
            Self::ServerStreaming => "server-streaming",
            Self::BidirectionalStreaming => "bidirectional-streaming",
        }
    }
}

impl Default for StreamingMode {
    fn default() -> Self {
        Self::Unary
    }
}

impl fmt::Display for StreamingMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for StreamingMode {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "unary" => Ok(Self::Unary),
            "client-streaming" => Ok(Self::ClientStreaming),
            "server-streaming" => Ok(Self::ServerStreaming),
            "bidirectional-streaming" => Ok(Self::BidirectionalStreaming),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ServiceName,
    kind: GrpcStatusCode,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ServiceName, kind: GrpcStatusCode) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ServiceName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> GrpcStatusCode {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ServiceName::new("/users.UserService/GetUser")?;

        assert_eq!(value.as_str(), "/users.UserService/GetUser");
        assert_eq!(value.to_string(), "/users.UserService/GetUser");
        assert_eq!("/users.UserService/GetUser".parse::<ServiceName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ServiceName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "ok".parse::<GrpcStatusCode>()?;

        assert_eq!(kind, GrpcStatusCode::Ok);
        assert_eq!(kind.to_string(), "ok");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            ServiceName::new("/users.UserService/GetUser")?,
            GrpcStatusCode::default(),
        );

        assert_eq!(metadata.name().as_str(), "/users.UserService/GetUser");
        assert_eq!(metadata.kind(), GrpcStatusCode::default());
        Ok(())
    }
}
