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

text_newtype!(ApiHeaderName);
text_newtype!(CustomHeaderName);
text_newtype!(CorrelationHeaderName);
text_newtype!(AuthHeaderName);
text_newtype!(IdempotencyHeaderName);
text_newtype!(RateLimitHeaderName);

/// Common API header labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommonApiHeader {
    /// A stable label variant.
    Authorization,
    /// A stable label variant.
    ContentType,
    /// A stable label variant.
    Accept,
    /// A stable label variant.
    CorrelationId,
    /// A stable label variant.
    IdempotencyKey,
    /// A stable label variant.
    RateLimitLimit,
    /// A stable label variant.
    RateLimitRemaining,
    /// A stable label variant.
    RateLimitReset,
}

impl CommonApiHeader {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authorization => "authorization",
            Self::ContentType => "content-type",
            Self::Accept => "accept",
            Self::CorrelationId => "correlation-id",
            Self::IdempotencyKey => "idempotency-key",
            Self::RateLimitLimit => "rate-limit-limit",
            Self::RateLimitRemaining => "rate-limit-remaining",
            Self::RateLimitReset => "rate-limit-reset",
        }
    }
}

impl Default for CommonApiHeader {
    fn default() -> Self {
        Self::Authorization
    }
}

impl fmt::Display for CommonApiHeader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CommonApiHeader {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "authorization" => Ok(Self::Authorization),
            "content-type" => Ok(Self::ContentType),
            "accept" => Ok(Self::Accept),
            "correlation-id" => Ok(Self::CorrelationId),
            "idempotency-key" => Ok(Self::IdempotencyKey),
            "rate-limit-limit" => Ok(Self::RateLimitLimit),
            "rate-limit-remaining" => Ok(Self::RateLimitRemaining),
            "rate-limit-reset" => Ok(Self::RateLimitReset),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ApiHeaderName,
    kind: CommonApiHeader,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ApiHeaderName, kind: CommonApiHeader) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ApiHeaderName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> CommonApiHeader {
        self.kind
    }
}

impl CommonApiHeader {
    /// Returns the conventional HTTP header spelling.
    #[must_use]
    pub const fn header_name(self) -> &'static str {
        match self {
            Self::Authorization => "Authorization",
            Self::ContentType => "Content-Type",
            Self::Accept => "Accept",
            Self::CorrelationId => "X-Correlation-Id",
            Self::IdempotencyKey => "Idempotency-Key",
            Self::RateLimitLimit => "RateLimit-Limit",
            Self::RateLimitRemaining => "RateLimit-Remaining",
            Self::RateLimitReset => "RateLimit-Reset",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ApiHeaderName::new("X-Request-Id")?;

        assert_eq!(value.as_str(), "X-Request-Id");
        assert_eq!(value.to_string(), "X-Request-Id");
        assert_eq!("X-Request-Id".parse::<ApiHeaderName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ApiHeaderName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "authorization".parse::<CommonApiHeader>()?;

        assert_eq!(kind, CommonApiHeader::Authorization);
        assert_eq!(kind.to_string(), "authorization");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            ApiHeaderName::new("X-Request-Id")?,
            CommonApiHeader::default(),
        );

        assert_eq!(metadata.name().as_str(), "X-Request-Id");
        assert_eq!(metadata.kind(), CommonApiHeader::default());
        Ok(())
    }
}
