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

text_newtype!(ResourceName);
text_newtype!(ResourceIdentifier);
text_newtype!(CollectionName);
text_newtype!(RepresentationName);

/// REST-style action vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RestAction {
    /// A stable label variant.
    List,
    /// A stable label variant.
    Retrieve,
    /// A stable label variant.
    Create,
    /// A stable label variant.
    Update,
    /// A stable label variant.
    Delete,
    /// A stable label variant.
    Patch,
    /// A stable label variant.
    Search,
}

impl RestAction {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Retrieve => "retrieve",
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Patch => "patch",
            Self::Search => "search",
        }
    }
}

impl Default for RestAction {
    fn default() -> Self {
        Self::List
    }
}

impl fmt::Display for RestAction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RestAction {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "list" => Ok(Self::List),
            "retrieve" => Ok(Self::Retrieve),
            "create" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "patch" => Ok(Self::Patch),
            "search" => Ok(Self::Search),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// REST-style constraint labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RestConstraint {
    /// A stable label variant.
    Stateless,
    /// A stable label variant.
    Cacheable,
    /// A stable label variant.
    UniformInterface,
    /// A stable label variant.
    LayeredSystem,
}

impl RestConstraint {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stateless => "stateless",
            Self::Cacheable => "cacheable",
            Self::UniformInterface => "uniform-interface",
            Self::LayeredSystem => "layered-system",
        }
    }
}

impl Default for RestConstraint {
    fn default() -> Self {
        Self::Stateless
    }
}

impl fmt::Display for RestConstraint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RestConstraint {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "stateless" => Ok(Self::Stateless),
            "cacheable" => Ok(Self::Cacheable),
            "uniform-interface" => Ok(Self::UniformInterface),
            "layered-system" => Ok(Self::LayeredSystem),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// Representation kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepresentationKind {
    /// A stable label variant.
    Json,
    /// A stable label variant.
    Xml,
    /// A stable label variant.
    Form,
    /// A stable label variant.
    Binary,
    /// A stable label variant.
    Text,
}

impl RepresentationKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Xml => "xml",
            Self::Form => "form",
            Self::Binary => "binary",
            Self::Text => "text",
        }
    }
}

impl Default for RepresentationKind {
    fn default() -> Self {
        Self::Json
    }
}

impl fmt::Display for RepresentationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RepresentationKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "json" => Ok(Self::Json),
            "xml" => Ok(Self::Xml),
            "form" => Ok(Self::Form),
            "binary" => Ok(Self::Binary),
            "text" => Ok(Self::Text),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ResourceName,
    kind: RestAction,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ResourceName, kind: RestAction) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ResourceName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> RestAction {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ResourceName::new("users")?;

        assert_eq!(value.as_str(), "users");
        assert_eq!(value.to_string(), "users");
        assert_eq!("users".parse::<ResourceName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ResourceName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "list".parse::<RestAction>()?;

        assert_eq!(kind, RestAction::List);
        assert_eq!(kind.to_string(), "list");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(ResourceName::new("users")?, RestAction::default());

        assert_eq!(metadata.name().as_str(), "users");
        assert_eq!(metadata.kind(), RestAction::default());
        Ok(())
    }
}
