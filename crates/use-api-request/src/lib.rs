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

text_newtype!(RequestId);
text_newtype!(CorrelationId);
text_newtype!(TraceId);
text_newtype!(RequestTimestampLabel);
text_newtype!(RequestSource);

/// Request context kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestContextKind {
    /// A stable label variant.
    User,
    /// A stable label variant.
    Service,
    /// A stable label variant.
    Job,
    /// A stable label variant.
    Test,
    /// A stable label variant.
    Unknown,
}

impl RequestContextKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Service => "service",
            Self::Job => "job",
            Self::Test => "test",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for RequestContextKind {
    fn default() -> Self {
        Self::User
    }
}

impl fmt::Display for RequestContextKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RequestContextKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "user" => Ok(Self::User),
            "service" => Ok(Self::Service),
            "job" => Ok(Self::Job),
            "test" => Ok(Self::Test),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: RequestId,
    kind: RequestContextKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: RequestId, kind: RequestContextKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &RequestId {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> RequestContextKind {
        self.kind
    }
}

/// Protocol-neutral request envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiRequest<T> {
    id: RequestId,
    context: RequestContextKind,
    body: T,
}

impl<T> ApiRequest<T> {
    /// Creates a request envelope.
    #[must_use]
    pub const fn new(id: RequestId, body: T) -> Self {
        Self {
            id,
            context: RequestContextKind::Unknown,
            body,
        }
    }

    /// Sets request context metadata.
    #[must_use]
    pub const fn with_context(mut self, context: RequestContextKind) -> Self {
        self.context = context;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = RequestId::new("req_123")?;

        assert_eq!(value.as_str(), "req_123");
        assert_eq!(value.to_string(), "req_123");
        assert_eq!("req_123".parse::<RequestId>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(RequestId::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "user".parse::<RequestContextKind>()?;

        assert_eq!(kind, RequestContextKind::User);
        assert_eq!(kind.to_string(), "user");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(RequestId::new("req_123")?, RequestContextKind::default());

        assert_eq!(metadata.name().as_str(), "req_123");
        assert_eq!(metadata.kind(), RequestContextKind::default());
        Ok(())
    }
}
