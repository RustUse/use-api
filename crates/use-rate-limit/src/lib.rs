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

text_newtype!(BucketName);
text_newtype!(ResetTimeLabel);

/// Rate limit policy labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LimitPolicy {
    /// A stable label variant.
    FixedWindow,
    /// A stable label variant.
    SlidingWindow,
    /// A stable label variant.
    TokenBucket,
    /// A stable label variant.
    LeakyBucket,
}

impl LimitPolicy {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixedWindow => "fixed-window",
            Self::SlidingWindow => "sliding-window",
            Self::TokenBucket => "token-bucket",
            Self::LeakyBucket => "leaky-bucket",
        }
    }
}

impl Default for LimitPolicy {
    fn default() -> Self {
        Self::FixedWindow
    }
}

impl fmt::Display for LimitPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LimitPolicy {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "fixed-window" => Ok(Self::FixedWindow),
            "sliding-window" => Ok(Self::SlidingWindow),
            "token-bucket" => Ok(Self::TokenBucket),
            "leaky-bucket" => Ok(Self::LeakyBucket),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: BucketName,
    kind: LimitPolicy,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: BucketName, kind: LimitPolicy) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &BucketName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> LimitPolicy {
        self.kind
    }
}

/// Rate limit counters.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RateLimit {
    quota: u64,
    remaining: u64,
}

impl RateLimit {
    /// Creates rate limit counters.
    #[must_use]
    pub const fn new(quota: u64, remaining: u64) -> Self {
        Self { quota, remaining }
    }

    /// Returns true when no quota remains.
    #[must_use]
    pub const fn is_exhausted(self) -> bool {
        self.remaining == 0
    }

    /// Returns true when quota remains.
    #[must_use]
    pub const fn has_remaining(self) -> bool {
        self.remaining > 0
    }

    /// Returns the configured quota.
    #[must_use]
    pub const fn quota(self) -> u64 {
        self.quota
    }
}

/// Retry-after seconds.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RetryAfterSeconds(u64);

impl RetryAfterSeconds {
    /// Creates retry-after seconds.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the seconds value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = BucketName::new("global")?;

        assert_eq!(value.as_str(), "global");
        assert_eq!(value.to_string(), "global");
        assert_eq!("global".parse::<BucketName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(BucketName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "fixed-window".parse::<LimitPolicy>()?;

        assert_eq!(kind, LimitPolicy::FixedWindow);
        assert_eq!(kind.to_string(), "fixed-window");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(BucketName::new("global")?, LimitPolicy::default());

        assert_eq!(metadata.name().as_str(), "global");
        assert_eq!(metadata.kind(), LimitPolicy::default());
        Ok(())
    }
}
