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

text_newtype!(ErrorCode);
text_newtype!(ErrorMessage);
text_newtype!(ErrorDetail);
text_newtype!(FieldName);

/// API error category labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorCategory {
    /// A stable label variant.
    Validation,
    /// A stable label variant.
    Authentication,
    /// A stable label variant.
    Authorization,
    /// A stable label variant.
    Conflict,
    /// A stable label variant.
    RateLimit,
    /// A stable label variant.
    NotFound,
    /// A stable label variant.
    Server,
    /// A stable label variant.
    Unknown,
}

impl ErrorCategory {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::Conflict => "conflict",
            Self::RateLimit => "rate-limit",
            Self::NotFound => "not-found",
            Self::Server => "server",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for ErrorCategory {
    fn default() -> Self {
        Self::Validation
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ErrorCategory {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "validation" => Ok(Self::Validation),
            "authentication" => Ok(Self::Authentication),
            "authorization" => Ok(Self::Authorization),
            "conflict" => Ok(Self::Conflict),
            "rate-limit" => Ok(Self::RateLimit),
            "not-found" => Ok(Self::NotFound),
            "server" => Ok(Self::Server),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ErrorCode,
    kind: ErrorCategory,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ErrorCode, kind: ErrorCategory) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ErrorCode {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> ErrorCategory {
        self.kind
    }
}

/// Protocol-neutral API error envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiError {
    code: ErrorCode,
    message: ErrorMessage,
    category: ErrorCategory,
    retryable: bool,
}

impl ApiError {
    /// Creates an API error envelope.
    #[must_use]
    pub const fn new(code: ErrorCode, message: ErrorMessage) -> Self {
        Self {
            code,
            message,
            category: ErrorCategory::Unknown,
            retryable: false,
        }
    }

    /// Sets the error category.
    #[must_use]
    pub const fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    /// Marks the error as retryable or not retryable.
    #[must_use]
    pub const fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// Returns the error code.
    #[must_use]
    pub const fn code(&self) -> &ErrorCode {
        &self.code
    }

    /// Returns true when retrying may be useful.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        self.retryable
    }
}

/// A field-specific validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldError {
    field: FieldName,
    message: ErrorMessage,
}

impl FieldError {
    /// Creates a field error.
    #[must_use]
    pub const fn new(field: FieldName, message: ErrorMessage) -> Self {
        Self { field, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ErrorCode::new("invalid-request")?;

        assert_eq!(value.as_str(), "invalid-request");
        assert_eq!(value.to_string(), "invalid-request");
        assert_eq!("invalid-request".parse::<ErrorCode>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ErrorCode::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "validation".parse::<ErrorCategory>()?;

        assert_eq!(kind, ErrorCategory::Validation);
        assert_eq!(kind.to_string(), "validation");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(ErrorCode::new("invalid-request")?, ErrorCategory::default());

        assert_eq!(metadata.name().as_str(), "invalid-request");
        assert_eq!(metadata.kind(), ErrorCategory::default());
        Ok(())
    }
}
