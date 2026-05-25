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

text_newtype!(PageMarker);
text_newtype!(NextMarker);
text_newtype!(PreviousMarker);

/// Pagination direction labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaginationDirection {
    /// A stable label variant.
    Next,
    /// A stable label variant.
    Previous,
    /// A stable label variant.
    Current,
}

impl PaginationDirection {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Next => "next",
            Self::Previous => "previous",
            Self::Current => "current",
        }
    }
}

impl Default for PaginationDirection {
    fn default() -> Self {
        Self::Next
    }
}

impl fmt::Display for PaginationDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PaginationDirection {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "next" => Ok(Self::Next),
            "previous" => Ok(Self::Previous),
            "current" => Ok(Self::Current),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: PageMarker,
    kind: PaginationDirection,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: PageMarker, kind: PaginationDirection) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &PageMarker {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> PaginationDirection {
        self.kind
    }
}

/// A one-based page number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PageNumber(u64);

impl PageNumber {
    /// Creates a one-based page number.
    pub const fn new(value: u64) -> Result<Self, ApiPrimitiveError> {
        if value == 0 {
            Err(ApiPrimitiveError::Invalid)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric page number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// A bounded page size.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PageSize(u64);

impl PageSize {
    /// Creates a non-zero page size.
    pub const fn new(value: u64) -> Result<Self, ApiPrimitiveError> {
        if value == 0 {
            Err(ApiPrimitiveError::Invalid)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric page size.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Pagination metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageInfo {
    page: PageNumber,
    size: PageSize,
    total_count: Option<u64>,
    has_more: bool,
}

impl PageInfo {
    /// Creates pagination metadata.
    #[must_use]
    pub const fn new(page: PageNumber, size: PageSize) -> Self {
        Self {
            page,
            size,
            total_count: None,
            has_more: false,
        }
    }

    /// Adds a total count.
    #[must_use]
    pub const fn with_total_count(mut self, total_count: u64) -> Self {
        self.total_count = Some(total_count);
        self
    }

    /// Sets whether a next page is available.
    #[must_use]
    pub const fn with_has_more(mut self, has_more: bool) -> Self {
        self.has_more = has_more;
        self
    }

    /// Returns true when another page is available.
    #[must_use]
    pub const fn has_more(&self) -> bool {
        self.has_more
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = PageMarker::new("page-1")?;

        assert_eq!(value.as_str(), "page-1");
        assert_eq!(value.to_string(), "page-1");
        assert_eq!("page-1".parse::<PageMarker>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(PageMarker::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "next".parse::<PaginationDirection>()?;

        assert_eq!(kind, PaginationDirection::Next);
        assert_eq!(kind.to_string(), "next");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata =
            PrimitiveMetadata::new(PageMarker::new("page-1")?, PaginationDirection::default());

        assert_eq!(metadata.name().as_str(), "page-1");
        assert_eq!(metadata.kind(), PaginationDirection::default());
        Ok(())
    }
}
