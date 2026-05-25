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

text_newtype!(ResponseMetadata);
text_newtype!(ResponseTiming);
text_newtype!(ResponseLink);

/// Response status category labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResponseStatusCategory {
    /// A stable label variant.
    Informational,
    /// A stable label variant.
    Success,
    /// A stable label variant.
    Redirection,
    /// A stable label variant.
    ClientError,
    /// A stable label variant.
    ServerError,
    /// A stable label variant.
    Unknown,
}

impl ResponseStatusCategory {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Success => "success",
            Self::Redirection => "redirection",
            Self::ClientError => "client-error",
            Self::ServerError => "server-error",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for ResponseStatusCategory {
    fn default() -> Self {
        Self::Informational
    }
}

impl fmt::Display for ResponseStatusCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ResponseStatusCategory {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "informational" => Ok(Self::Informational),
            "success" => Ok(Self::Success),
            "redirection" => Ok(Self::Redirection),
            "client-error" => Ok(Self::ClientError),
            "server-error" => Ok(Self::ServerError),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ResponseMetadata,
    kind: ResponseStatusCategory,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ResponseMetadata, kind: ResponseStatusCategory) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ResponseMetadata {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> ResponseStatusCategory {
        self.kind
    }
}

/// Protocol-neutral response envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiResponse<T> {
    category: ResponseStatusCategory,
    body: T,
}

impl<T> ApiResponse<T> {
    /// Creates a response envelope.
    #[must_use]
    pub const fn new(category: ResponseStatusCategory, body: T) -> Self {
        Self { category, body }
    }

    /// Returns the status category.
    #[must_use]
    pub const fn category(&self) -> ResponseStatusCategory {
        self.category
    }
}

/// Lightweight response page metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponsePageInfo {
    has_more: bool,
}

impl ResponsePageInfo {
    /// Creates response page metadata.
    #[must_use]
    pub const fn new(has_more: bool) -> Self {
        Self { has_more }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ResponseMetadata::new("response-link")?;

        assert_eq!(value.as_str(), "response-link");
        assert_eq!(value.to_string(), "response-link");
        assert_eq!("response-link".parse::<ResponseMetadata>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ResponseMetadata::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "informational".parse::<ResponseStatusCategory>()?;

        assert_eq!(kind, ResponseStatusCategory::Informational);
        assert_eq!(kind.to_string(), "informational");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            ResponseMetadata::new("response-link")?,
            ResponseStatusCategory::default(),
        );

        assert_eq!(metadata.name().as_str(), "response-link");
        assert_eq!(metadata.kind(), ResponseStatusCategory::default());
        Ok(())
    }
}
