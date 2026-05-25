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

text_newtype!(OperationId);
text_newtype!(OperationName);
text_newtype!(OperationSummary);

/// API operation kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationKind {
    /// A stable label variant.
    Query,
    /// A stable label variant.
    Command,
    /// A stable label variant.
    Mutation,
    /// A stable label variant.
    Subscription,
    /// A stable label variant.
    Action,
}

impl OperationKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Command => "command",
            Self::Mutation => "mutation",
            Self::Subscription => "subscription",
            Self::Action => "action",
        }
    }
}

impl Default for OperationKind {
    fn default() -> Self {
        Self::Query
    }
}

impl fmt::Display for OperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OperationKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "query" => Ok(Self::Query),
            "command" => Ok(Self::Command),
            "mutation" => Ok(Self::Mutation),
            "subscription" => Ok(Self::Subscription),
            "action" => Ok(Self::Action),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// API operation status labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationStatus {
    /// A stable label variant.
    Active,
    /// A stable label variant.
    Deprecated,
    /// A stable label variant.
    Experimental,
    /// A stable label variant.
    Disabled,
}

impl OperationStatus {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::Experimental => "experimental",
            Self::Disabled => "disabled",
        }
    }
}

impl Default for OperationStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl fmt::Display for OperationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OperationStatus {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "active" => Ok(Self::Active),
            "deprecated" => Ok(Self::Deprecated),
            "experimental" => Ok(Self::Experimental),
            "disabled" => Ok(Self::Disabled),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// API operation lifecycle labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationLifecycle {
    /// A stable label variant.
    Design,
    /// A stable label variant.
    Preview,
    /// A stable label variant.
    Stable,
    /// A stable label variant.
    Deprecated,
    /// A stable label variant.
    Retired,
}

impl OperationLifecycle {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Design => "design",
            Self::Preview => "preview",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Retired => "retired",
        }
    }
}

impl Default for OperationLifecycle {
    fn default() -> Self {
        Self::Design
    }
}

impl fmt::Display for OperationLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OperationLifecycle {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "design" => Ok(Self::Design),
            "preview" => Ok(Self::Preview),
            "stable" => Ok(Self::Stable),
            "deprecated" => Ok(Self::Deprecated),
            "retired" => Ok(Self::Retired),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: OperationId,
    kind: OperationKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: OperationId, kind: OperationKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &OperationId {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> OperationKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = OperationId::new("listUsers")?;

        assert_eq!(value.as_str(), "listUsers");
        assert_eq!(value.to_string(), "listUsers");
        assert_eq!("listUsers".parse::<OperationId>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(OperationId::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "query".parse::<OperationKind>()?;

        assert_eq!(kind, OperationKind::Query);
        assert_eq!(kind.to_string(), "query");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(OperationId::new("listUsers")?, OperationKind::default());

        assert_eq!(metadata.name().as_str(), "listUsers");
        assert_eq!(metadata.kind(), OperationKind::default());
        Ok(())
    }
}
