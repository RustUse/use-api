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

text_newtype!(RouteTemplate);
text_newtype!(StaticSegment);
text_newtype!(DynamicParam);
text_newtype!(WildcardParam);
text_newtype!(OptionalSegment);

/// Route segment kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RouteSegmentKind {
    /// A stable label variant.
    Static,
    /// A stable label variant.
    Dynamic,
    /// A stable label variant.
    Wildcard,
    /// A stable label variant.
    Optional,
}

impl RouteSegmentKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Dynamic => "dynamic",
            Self::Wildcard => "wildcard",
            Self::Optional => "optional",
        }
    }
}

impl Default for RouteSegmentKind {
    fn default() -> Self {
        Self::Static
    }
}

impl fmt::Display for RouteSegmentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RouteSegmentKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "static" => Ok(Self::Static),
            "dynamic" => Ok(Self::Dynamic),
            "wildcard" => Ok(Self::Wildcard),
            "optional" => Ok(Self::Optional),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// Route matching metadata labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RouteMatchKind {
    /// A stable label variant.
    Exact,
    /// A stable label variant.
    Pattern,
    /// A stable label variant.
    Prefix,
}

impl RouteMatchKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Pattern => "pattern",
            Self::Prefix => "prefix",
        }
    }
}

impl Default for RouteMatchKind {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for RouteMatchKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RouteMatchKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "exact" => Ok(Self::Exact),
            "pattern" => Ok(Self::Pattern),
            "prefix" => Ok(Self::Prefix),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: RouteTemplate,
    kind: RouteSegmentKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: RouteTemplate, kind: RouteSegmentKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &RouteTemplate {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> RouteSegmentKind {
        self.kind
    }
}

/// Lightweight route segment metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteSegment {
    value: String,
    kind: RouteSegmentKind,
}

impl RouteSegment {
    /// Classifies a route segment.
    #[must_use]
    pub fn classify(value: impl AsRef<str>) -> Self {
        let value = value.as_ref().trim().to_owned();
        let kind = if value.starts_with(':') {
            RouteSegmentKind::Dynamic
        } else if value.starts_with('*') {
            RouteSegmentKind::Wildcard
        } else if value.starts_with('[') && value.ends_with(']') {
            RouteSegmentKind::Optional
        } else {
            RouteSegmentKind::Static
        };
        Self { value, kind }
    }

    /// Returns the segment text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the segment kind.
    #[must_use]
    pub const fn kind(&self) -> RouteSegmentKind {
        self.kind
    }
}

impl RouteTemplate {
    /// Returns non-empty classified route segments.
    #[must_use]
    pub fn segments(&self) -> Vec<RouteSegment> {
        self.as_str()
            .split('/')
            .filter(|segment| !segment.is_empty())
            .map(RouteSegment::classify)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = RouteTemplate::new("/users/:id")?;

        assert_eq!(value.as_str(), "/users/:id");
        assert_eq!(value.to_string(), "/users/:id");
        assert_eq!("/users/:id".parse::<RouteTemplate>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(RouteTemplate::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "static".parse::<RouteSegmentKind>()?;

        assert_eq!(kind, RouteSegmentKind::Static);
        assert_eq!(kind.to_string(), "static");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            RouteTemplate::new("/users/:id")?,
            RouteSegmentKind::default(),
        );

        assert_eq!(metadata.name().as_str(), "/users/:id");
        assert_eq!(metadata.kind(), RouteSegmentKind::default());
        Ok(())
    }
}
