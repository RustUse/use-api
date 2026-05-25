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

text_newtype!(PathParamName);
text_newtype!(QueryParamName);
text_newtype!(HeaderParamName);
text_newtype!(BodyParamName);

/// API parameter location labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParamLocation {
    /// A stable label variant.
    Path,
    /// A stable label variant.
    Query,
    /// A stable label variant.
    Header,
    /// A stable label variant.
    Body,
}

impl ParamLocation {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Query => "query",
            Self::Header => "header",
            Self::Body => "body",
        }
    }
}

impl Default for ParamLocation {
    fn default() -> Self {
        Self::Path
    }
}

impl fmt::Display for ParamLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ParamLocation {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "path" => Ok(Self::Path),
            "query" => Ok(Self::Query),
            "header" => Ok(Self::Header),
            "body" => Ok(Self::Body),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// API parameter requirement labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParamRequirement {
    /// A stable label variant.
    Required,
    /// A stable label variant.
    Optional,
}

impl ParamRequirement {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

impl Default for ParamRequirement {
    fn default() -> Self {
        Self::Required
    }
}

impl fmt::Display for ParamRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ParamRequirement {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "required" => Ok(Self::Required),
            "optional" => Ok(Self::Optional),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// API parameter style labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParamStyle {
    /// A stable label variant.
    Simple,
    /// A stable label variant.
    Form,
    /// A stable label variant.
    Matrix,
    /// A stable label variant.
    Label,
    /// A stable label variant.
    DeepObject,
}

impl ParamStyle {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Form => "form",
            Self::Matrix => "matrix",
            Self::Label => "label",
            Self::DeepObject => "deep-object",
        }
    }
}

impl Default for ParamStyle {
    fn default() -> Self {
        Self::Simple
    }
}

impl fmt::Display for ParamStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ParamStyle {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "simple" => Ok(Self::Simple),
            "form" => Ok(Self::Form),
            "matrix" => Ok(Self::Matrix),
            "label" => Ok(Self::Label),
            "deep-object" => Ok(Self::DeepObject),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: PathParamName,
    kind: ParamLocation,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: PathParamName, kind: ParamLocation) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &PathParamName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> ParamLocation {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = PathParamName::new("user_id")?;

        assert_eq!(value.as_str(), "user_id");
        assert_eq!(value.to_string(), "user_id");
        assert_eq!("user_id".parse::<PathParamName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(PathParamName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "path".parse::<ParamLocation>()?;

        assert_eq!(kind, ParamLocation::Path);
        assert_eq!(kind.to_string(), "path");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(PathParamName::new("user_id")?, ParamLocation::default());

        assert_eq!(metadata.name().as_str(), "user_id");
        assert_eq!(metadata.kind(), ParamLocation::default());
        Ok(())
    }
}
