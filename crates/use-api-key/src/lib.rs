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

text_newtype!(ApiKeyId);
text_newtype!(ApiKeyPrefix);

/// An API key value that redacts its debug representation.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApiKey(String);

impl ApiKey {
    /// Creates validated API key text metadata.
    ///
    /// # Errors
    ///
    /// Returns [`ApiPrimitiveError`] when the value is empty or contains control characters.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ApiPrimitiveError> {
        validate_api_text(value.as_ref()).map(|value| Self(value.to_owned()))
    }

    /// Parses validated API key text metadata.
    ///
    /// # Errors
    ///
    /// Returns [`ApiPrimitiveError`] when validation fails.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ApiPrimitiveError> {
        Self::new(value)
    }

    /// Returns the stored API key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns the stored text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ApiKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ApiKey {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for ApiKey {
    type Error = ApiPrimitiveError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

text_newtype!(ApiKeyLabel);

/// API key status labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ApiKeyStatus {
    /// A stable label variant.
    Active,
    /// A stable label variant.
    Revoked,
    /// A stable label variant.
    Expired,
    /// A stable label variant.
    Suspended,
}

impl ApiKeyStatus {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Suspended => "suspended",
        }
    }
}

impl Default for ApiKeyStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl fmt::Display for ApiKeyStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ApiKeyStatus {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "active" => Ok(Self::Active),
            "revoked" => Ok(Self::Revoked),
            "expired" => Ok(Self::Expired),
            "suspended" => Ok(Self::Suspended),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ApiKeyId,
    kind: ApiKeyStatus,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ApiKeyId, kind: ApiKeyStatus) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ApiKeyId {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> ApiKeyStatus {
        self.kind
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ApiKey(\"<redacted>\")")
    }
}

impl ApiKey {
    /// Returns a safely redacted key string.
    #[must_use]
    pub fn redacted(&self) -> String {
        redact_api_key(self.as_str())
    }
}

/// Redacts API key text while preserving short prefix context.
#[must_use]
pub fn redact_api_key(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= 8 {
        return String::from("<redacted>");
    }
    let prefix = &trimmed[..4];
    let suffix = &trimmed[trimmed.len() - 4..];
    format!("{prefix}...{suffix}")
}

/// API key metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiKeyMetadata {
    id: ApiKeyId,
    prefix: ApiKeyPrefix,
    status: ApiKeyStatus,
}

impl ApiKeyMetadata {
    /// Creates API key metadata.
    #[must_use]
    pub const fn new(id: ApiKeyId, prefix: ApiKeyPrefix, status: ApiKeyStatus) -> Self {
        Self { id, prefix, status }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ApiKeyId::new("sk_live_example")?;

        assert_eq!(value.as_str(), "sk_live_example");
        assert_eq!(value.to_string(), "sk_live_example");
        assert_eq!("sk_live_example".parse::<ApiKeyId>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ApiKeyId::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "active".parse::<ApiKeyStatus>()?;

        assert_eq!(kind, ApiKeyStatus::Active);
        assert_eq!(kind.to_string(), "active");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(ApiKeyId::new("sk_live_example")?, ApiKeyStatus::default());

        assert_eq!(metadata.name().as_str(), "sk_live_example");
        assert_eq!(metadata.kind(), ApiKeyStatus::default());
        Ok(())
    }
}
