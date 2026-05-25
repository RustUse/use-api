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

text_newtype!(SchemaName);
text_newtype!(SchemaFieldName);

/// Schema field kind labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FieldKind {
    /// A stable label variant.
    String,
    /// A stable label variant.
    Number,
    /// A stable label variant.
    Integer,
    /// A stable label variant.
    Boolean,
    /// A stable label variant.
    Array,
    /// A stable label variant.
    Object,
    /// A stable label variant.
    Null,
    /// A stable label variant.
    Custom,
}

impl FieldKind {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Number => "number",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::Array => "array",
            Self::Object => "object",
            Self::Null => "null",
            Self::Custom => "custom",
        }
    }
}

impl Default for FieldKind {
    fn default() -> Self {
        Self::String
    }
}

impl fmt::Display for FieldKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FieldKind {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "string" => Ok(Self::String),
            "number" => Ok(Self::Number),
            "integer" => Ok(Self::Integer),
            "boolean" => Ok(Self::Boolean),
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            "null" => Ok(Self::Null),
            "custom" => Ok(Self::Custom),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// Schema field requirement labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FieldRequirement {
    /// A stable label variant.
    Required,
    /// A stable label variant.
    Optional,
}

impl FieldRequirement {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

impl Default for FieldRequirement {
    fn default() -> Self {
        Self::Required
    }
}

impl fmt::Display for FieldRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FieldRequirement {
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
/// Schema nullability labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Nullability {
    /// A stable label variant.
    Nullable,
    /// A stable label variant.
    NonNullable,
}

impl Nullability {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nullable => "nullable",
            Self::NonNullable => "non-nullable",
        }
    }
}

impl Default for Nullability {
    fn default() -> Self {
        Self::Nullable
    }
}

impl fmt::Display for Nullability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Nullability {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "nullable" => Ok(Self::Nullable),
            "non-nullable" => Ok(Self::NonNullable),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}
/// Broad schema shape labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchemaShape {
    /// A stable label variant.
    Scalar,
    /// A stable label variant.
    Array,
    /// A stable label variant.
    Object,
}

impl SchemaShape {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Array => "array",
            Self::Object => "object",
        }
    }
}

impl Default for SchemaShape {
    fn default() -> Self {
        Self::Scalar
    }
}

impl fmt::Display for SchemaShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SchemaShape {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "scalar" => Ok(Self::Scalar),
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: SchemaName,
    kind: FieldKind,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: SchemaName, kind: FieldKind) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &SchemaName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> FieldKind {
        self.kind
    }
}

/// API schema field metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaField {
    name: SchemaFieldName,
    kind: FieldKind,
    requirement: FieldRequirement,
    nullability: Nullability,
}

impl SchemaField {
    /// Creates schema field metadata.
    #[must_use]
    pub const fn new(name: SchemaFieldName, kind: FieldKind) -> Self {
        Self {
            name,
            kind,
            requirement: FieldRequirement::Optional,
            nullability: Nullability::NonNullable,
        }
    }

    /// Marks the field as required.
    #[must_use]
    pub const fn required(mut self) -> Self {
        self.requirement = FieldRequirement::Required;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = SchemaName::new("User")?;

        assert_eq!(value.as_str(), "User");
        assert_eq!(value.to_string(), "User");
        assert_eq!("User".parse::<SchemaName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(SchemaName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "string".parse::<FieldKind>()?;

        assert_eq!(kind, FieldKind::String);
        assert_eq!(kind.to_string(), "string");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(SchemaName::new("User")?, FieldKind::default());

        assert_eq!(metadata.name().as_str(), "User");
        assert_eq!(metadata.kind(), FieldKind::default());
        Ok(())
    }
}
