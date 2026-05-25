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

text_newtype!(MediaType);
text_newtype!(MediaSubtype);
text_newtype!(MediaTypeSuffix);
text_newtype!(Charset);
text_newtype!(StructuredSyntaxSuffix);

/// Common API media type labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommonApiMediaType {
    /// A stable label variant.
    Json,
    /// A stable label variant.
    ProblemJson,
    /// A stable label variant.
    MergePatchJson,
    /// A stable label variant.
    Ndjson,
    /// A stable label variant.
    Xml,
    /// A stable label variant.
    TextPlain,
    /// A stable label variant.
    OctetStream,
}

impl CommonApiMediaType {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::ProblemJson => "problem-json",
            Self::MergePatchJson => "merge-patch-json",
            Self::Ndjson => "ndjson",
            Self::Xml => "xml",
            Self::TextPlain => "text-plain",
            Self::OctetStream => "octet-stream",
        }
    }
}

impl Default for CommonApiMediaType {
    fn default() -> Self {
        Self::Json
    }
}

impl fmt::Display for CommonApiMediaType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CommonApiMediaType {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "json" => Ok(Self::Json),
            "problem-json" => Ok(Self::ProblemJson),
            "merge-patch-json" => Ok(Self::MergePatchJson),
            "ndjson" => Ok(Self::Ndjson),
            "xml" => Ok(Self::Xml),
            "text-plain" => Ok(Self::TextPlain),
            "octet-stream" => Ok(Self::OctetStream),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: MediaType,
    kind: CommonApiMediaType,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: MediaType, kind: CommonApiMediaType) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &MediaType {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> CommonApiMediaType {
        self.kind
    }
}

impl CommonApiMediaType {
    /// Returns the conventional media type string.
    #[must_use]
    pub const fn media_type(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::ProblemJson => "application/problem+json",
            Self::MergePatchJson => "application/merge-patch+json",
            Self::Ndjson => "application/x-ndjson",
            Self::Xml => "application/xml",
            Self::TextPlain => "text/plain",
            Self::OctetStream => "application/octet-stream",
        }
    }
}

impl MediaType {
    /// Returns the structured syntax suffix when one is present.
    #[must_use]
    pub fn suffix(&self) -> Option<&str> {
        self.as_str()
            .split(';')
            .next()?
            .rsplit_once('+')
            .map(|(_, suffix)| suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = MediaType::new("application/json")?;

        assert_eq!(value.as_str(), "application/json");
        assert_eq!(value.to_string(), "application/json");
        assert_eq!("application/json".parse::<MediaType>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(MediaType::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "json".parse::<CommonApiMediaType>()?;

        assert_eq!(kind, CommonApiMediaType::Json);
        assert_eq!(kind.to_string(), "json");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            MediaType::new("application/json")?,
            CommonApiMediaType::default(),
        );

        assert_eq!(metadata.name().as_str(), "application/json");
        assert_eq!(metadata.kind(), CommonApiMediaType::default());
        Ok(())
    }
}
