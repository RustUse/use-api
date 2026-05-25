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

text_newtype!(GraphqlName);
text_newtype!(TypeName);
text_newtype!(FieldName);
text_newtype!(OperationName);
text_newtype!(DirectiveName);
text_newtype!(ArgumentName);
text_newtype!(VariableName);

/// `GraphQL` operation kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphqlOperationKind {
    /// A stable label variant.
    Query,
    /// A stable label variant.
    Mutation,
    /// A stable label variant.
    Subscription,
}

impl GraphqlOperationKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Mutation => "mutation",
            Self::Subscription => "subscription",
        }
    }
}

impl Default for GraphqlOperationKind {
    fn default() -> Self {
        Self::Query
    }
}

impl fmt::Display for GraphqlOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GraphqlOperationKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "query" => Ok(Self::Query),
            "mutation" => Ok(Self::Mutation),
            "subscription" => Ok(Self::Subscription),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// `GraphQL` type kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphqlTypeKind {
    /// A stable label variant.
    Scalar,
    /// A stable label variant.
    Object,
    /// A stable label variant.
    Interface,
    /// A stable label variant.
    Union,
    /// A stable label variant.
    Enum,
    /// A stable label variant.
    InputObject,
    /// A stable label variant.
    List,
    /// A stable label variant.
    NonNull,
}

impl GraphqlTypeKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Object => "object",
            Self::Interface => "interface",
            Self::Union => "union",
            Self::Enum => "enum",
            Self::InputObject => "input-object",
            Self::List => "list",
            Self::NonNull => "non-null",
        }
    }
}

impl Default for GraphqlTypeKind {
    fn default() -> Self {
        Self::Scalar
    }
}

impl fmt::Display for GraphqlTypeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for GraphqlTypeKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "scalar" => Ok(Self::Scalar),
            "object" => Ok(Self::Object),
            "interface" => Ok(Self::Interface),
            "union" => Ok(Self::Union),
            "enum" => Ok(Self::Enum),
            "input-object" => Ok(Self::InputObject),
            "list" => Ok(Self::List),
            "non-null" => Ok(Self::NonNull),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: GraphqlName,
    kind: GraphqlOperationKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: GraphqlName, kind: GraphqlOperationKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &GraphqlName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> GraphqlOperationKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = GraphqlName::new("viewer")?;

        assert_eq!(value.as_str(), "viewer");
        assert_eq!(value.to_string(), "viewer");
        assert_eq!("viewer".parse::<GraphqlName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(GraphqlName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "query".parse::<GraphqlOperationKind>()?;

        assert_eq!(kind, GraphqlOperationKind::Query);
        assert_eq!(kind.to_string(), "query");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(GraphqlName::new("viewer")?, GraphqlOperationKind::default());

        assert_eq!(metadata.name().as_str(), "viewer");
        assert_eq!(metadata.kind(), GraphqlOperationKind::default());
        Ok(())
    }
}
