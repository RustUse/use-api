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

text_newtype!(OpenApiPath);
text_newtype!(OperationId);
text_newtype!(ParameterName);
text_newtype!(RequestBodyName);
text_newtype!(ResponseName);
text_newtype!(SchemaName);
text_newtype!(TagName);
text_newtype!(ComponentName);

/// `OpenAPI` operation kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OpenApiOperationKind {
    /// A stable label variant.
    Get,
    /// A stable label variant.
    Post,
    /// A stable label variant.
    Put,
    /// A stable label variant.
    Patch,
    /// A stable label variant.
    Delete,
    /// A stable label variant.
    Options,
    /// A stable label variant.
    Head,
    /// A stable label variant.
    Trace,
}

impl OpenApiOperationKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Put => "put",
            Self::Patch => "patch",
            Self::Delete => "delete",
            Self::Options => "options",
            Self::Head => "head",
            Self::Trace => "trace",
        }
    }
}

impl Default for OpenApiOperationKind {
    fn default() -> Self {
        Self::Get
    }
}

impl fmt::Display for OpenApiOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OpenApiOperationKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            "patch" => Ok(Self::Patch),
            "delete" => Ok(Self::Delete),
            "options" => Ok(Self::Options),
            "head" => Ok(Self::Head),
            "trace" => Ok(Self::Trace),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// `OpenAPI` parameter location labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OpenApiParameterLocation {
    /// A stable label variant.
    Path,
    /// A stable label variant.
    Query,
    /// A stable label variant.
    Header,
    /// A stable label variant.
    Cookie,
}

impl OpenApiParameterLocation {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Query => "query",
            Self::Header => "header",
            Self::Cookie => "cookie",
        }
    }
}

impl Default for OpenApiParameterLocation {
    fn default() -> Self {
        Self::Path
    }
}

impl fmt::Display for OpenApiParameterLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for OpenApiParameterLocation {
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
            "cookie" => Ok(Self::Cookie),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: OpenApiPath,
    kind: OpenApiOperationKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: OpenApiPath, kind: OpenApiOperationKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &OpenApiPath {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> OpenApiOperationKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = OpenApiPath::new("listUsers")?;

        assert_eq!(value.as_str(), "listUsers");
        assert_eq!(value.to_string(), "listUsers");
        assert_eq!("listUsers".parse::<OpenApiPath>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(OpenApiPath::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "get".parse::<OpenApiOperationKind>()?;

        assert_eq!(kind, OpenApiOperationKind::Get);
        assert_eq!(kind.to_string(), "get");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            OpenApiPath::new("listUsers")?,
            OpenApiOperationKind::default(),
        );

        assert_eq!(metadata.name().as_str(), "listUsers");
        assert_eq!(metadata.kind(), OpenApiOperationKind::default());
        Ok(())
    }
}
