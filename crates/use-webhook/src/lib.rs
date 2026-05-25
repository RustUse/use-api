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

text_newtype!(WebhookEventName);
text_newtype!(WebhookEndpointUrl);
text_newtype!(DeliveryId);
text_newtype!(SignatureHeaderName);

/// Webhook delivery status labels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeliveryStatus {
    /// A stable label variant.
    Pending,
    /// A stable label variant.
    Delivered,
    /// A stable label variant.
    Failed,
    /// A stable label variant.
    Retrying,
    /// A stable label variant.
    Abandoned,
}

impl DeliveryStatus {
    /// Returns the stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
            Self::Abandoned => "abandoned",
        }
    }
}

impl Default for DeliveryStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl fmt::Display for DeliveryStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DeliveryStatus {
    type Err = ApiPrimitiveError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ApiPrimitiveError::Empty);
        }
        let normalized = trimmed.to_ascii_lowercase().replace('_', "-");
        match normalized.as_str() {
            "pending" => Ok(Self::Pending),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "retrying" => Ok(Self::Retrying),
            "abandoned" => Ok(Self::Abandoned),
            _ => Err(ApiPrimitiveError::Unknown),
        }
    }
}

/// Lightweight metadata tying this crate's primary text and label together.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveMetadata {
    name: WebhookEventName,
    kind: DeliveryStatus,
}

impl PrimitiveMetadata {
    /// Creates primitive metadata.
    #[must_use]
    pub const fn new(name: WebhookEventName, kind: DeliveryStatus) -> Self {
        Self { name, kind }
    }

    /// Returns the primary text value.
    #[must_use]
    pub const fn name(&self) -> &WebhookEventName {
        &self.name
    }

    /// Returns the primary label.
    #[must_use]
    pub const fn kind(&self) -> DeliveryStatus {
        self.kind
    }
}

/// Webhook event envelope metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookEvent {
    event: WebhookEventName,
    delivery_id: DeliveryId,
    status: DeliveryStatus,
}

impl WebhookEvent {
    /// Creates webhook event metadata.
    #[must_use]
    pub const fn new(event: WebhookEventName, delivery_id: DeliveryId) -> Self {
        Self {
            event,
            delivery_id,
            status: DeliveryStatus::Pending,
        }
    }

    /// Sets delivery status metadata.
    #[must_use]
    pub const fn with_status(mut self, status: DeliveryStatus) -> Self {
        self.status = status;
        self
    }
}

/// Delivery attempt count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeliveryAttempt(u32);

impl DeliveryAttempt {
    /// Creates a one-based delivery attempt count.
    pub const fn new(value: u32) -> Result<Self, ApiPrimitiveError> {
        if value == 0 {
            Err(ApiPrimitiveError::Invalid)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_displays_text() -> Result<(), ApiPrimitiveError> {
        let value = WebhookEventName::new("user.created")?;

        assert_eq!(value.as_str(), "user.created");
        assert_eq!(value.to_string(), "user.created");
        assert_eq!("user.created".parse::<WebhookEventName>()?, value);
        Ok(())
    }

    #[test]
    fn rejects_empty_text() {
        assert_eq!(WebhookEventName::new(""), Err(ApiPrimitiveError::Empty));
    }

    #[test]
    fn parses_and_displays_labels() -> Result<(), ApiPrimitiveError> {
        let kind = "pending".parse::<DeliveryStatus>()?;

        assert_eq!(kind, DeliveryStatus::Pending);
        assert_eq!(kind.to_string(), "pending");
        Ok(())
    }

    #[test]
    fn creates_metadata() -> Result<(), ApiPrimitiveError> {
        let metadata = PrimitiveMetadata::new(
            WebhookEventName::new("user.created")?,
            DeliveryStatus::default(),
        );

        assert_eq!(metadata.name().as_str(), "user.created");
        assert_eq!(metadata.kind(), DeliveryStatus::default());
        Ok(())
    }
}
