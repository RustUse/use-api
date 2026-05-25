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

text_newtype!(ApiVersion);
text_newtype!(VersionLabel);

/// API version label kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VersionKind {
    /// A stable label variant.
    Simple,
    /// A stable label variant.
    Date,
    /// A stable label variant.
    Semantic,
    /// A stable label variant.
    Custom,
}

impl VersionKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Date => "date",
            Self::Semantic => "semantic",
            Self::Custom => "custom",
        }
    }
}

impl Default for VersionKind {
    fn default() -> Self {
        Self::Simple
    }
}

impl fmt::Display for VersionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for VersionKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "simple" => Ok(Self::Simple),
            "date" => Ok(Self::Date),
            "semantic" => Ok(Self::Semantic),
            "custom" => Ok(Self::Custom),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// API compatibility labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Compatibility {
    /// A stable label variant.
    Compatible,
    /// A stable label variant.
    Breaking,
    /// A stable label variant.
    Deprecated,
    /// A stable label variant.
    Unknown,
}

impl Compatibility {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Breaking => "breaking",
            Self::Deprecated => "deprecated",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for Compatibility {
    fn default() -> Self {
        Self::Compatible
    }
}

impl fmt::Display for Compatibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Compatibility {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "compatible" => Ok(Self::Compatible),
            "breaking" => Ok(Self::Breaking),
            "deprecated" => Ok(Self::Deprecated),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: ApiVersion,
    kind: VersionKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: ApiVersion, kind: VersionKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &ApiVersion {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> VersionKind {
        self.kind
    }
}

impl ApiVersion {
    /// Returns the broad shape of the version label.
    #[must_use]
    pub fn kind(&self) -> VersionKind {
        let value = self.as_str();
        if value.starts_with('v')
            && value[1..]
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            VersionKind::Simple
        } else if is_date_like(value) {
            VersionKind::Date
        } else if is_semantic_like(value) {
            VersionKind::Semantic
        } else {
            VersionKind::Custom
        }
    }
}

fn is_date_like(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts
            .iter()
            .all(|part| part.chars().all(|character| character.is_ascii_digit()))
}

fn is_semantic_like(value: &str) -> bool {
    let parts = value.split('.').collect::<Vec<_>>();
    (2..=3).contains(&parts.len())
        && parts.iter().all(|part| {
            !part.is_empty() && part.chars().all(|character| character.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = ApiVersion::new("v1")?;

        assert_eq!(value.as_str(), "v1");
        assert_eq!(value.to_string(), "v1");
        assert_eq!("v1".parse::<ApiVersion>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(ApiVersion::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "simple".parse::<VersionKind>()?;

        assert_eq!(kind, VersionKind::Simple);
        assert_eq!(kind.to_string(), "simple");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(ApiVersion::new("v1")?, VersionKind::default());

        assert_eq!(metadata.name().as_str(), "v1");
        assert_eq!(metadata.kind(), VersionKind::default());
        Ok(())
    }
}
