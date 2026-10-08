// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Identity and telemetry configuration types.
//!
//! This module contains configuration options for identifying requests:
//! - [`WorkloadId`] - Workload identifier for resource governance
//! - [`CorrelationId`] - Client-side metrics correlation
//! - [`UserAgentSuffix`] - Suffix appended to the user agent string
//! - [`UserAgentProperty`] - Custom `key=value` entry in the user agent metadata
//!
//! The computed [`UserAgent`](crate::models::UserAgent) type is in the models module.

use std::fmt;

use crate::{
    error::CosmosError,
    models::{DRIVER_VERSION_KEY, FEATURE_FLAGS_KEY},
};

/// Workload identifier for resource governance.
///
/// Must be a value between 1 and 50 (inclusive) if set.
/// Used for workload-based resource allocation and tracking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkloadId(u8);

impl WorkloadId {
    /// The minimum allowed workload ID value.
    pub const MIN: u8 = 1;
    /// The maximum allowed workload ID value.
    pub const MAX: u8 = 50;

    /// Creates a new workload ID.
    ///
    /// # Panics
    ///
    /// Panics if the value is not between 1 and 50 (inclusive).
    pub fn new(value: u8) -> Self {
        assert!(
            (Self::MIN..=Self::MAX).contains(&value),
            "WorkloadId must be between {} and {} (inclusive), got {}",
            Self::MIN,
            Self::MAX,
            value
        );
        Self(value)
    }

    /// Creates a new workload ID, returning `None` if the value is out of range.
    pub fn try_new(value: u8) -> Option<Self> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the workload ID value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for WorkloadId {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value).ok_or("WorkloadId must be between 1 and 50 (inclusive)")
    }
}

impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validates that a string contains only HTTP header-safe characters.
///
/// Allowed characters: alphanumeric, hyphen, underscore, dot, and tilde.
fn is_http_header_safe(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~'))
}

/// Correlation ID for client-side metrics.
///
/// Used as a dimension for client-side metrics to correlate requests.
/// Limited to 50 characters and must contain only HTTP header-safe characters
/// (alphanumeric, hyphen, underscore, dot, tilde).
///
/// # Cardinality Warning
///
/// If the cardinality of correlation IDs is too high, metrics aggregation may
/// ignore or truncate this dimension. Choose values that provide meaningful
/// grouping without excessive uniqueness (e.g., cluster names, environment IDs,
/// deployment identifiers).
///
/// # Examples
///
/// Good values (low to moderate cardinality):
/// - AKS cluster name: `"aks-prod-eastus-001"`
/// - Environment: `"production"`, `"staging"`
/// - Deployment ID: `"deploy-2024-01-15"`
///
/// Avoid (high cardinality):
/// - Request IDs
/// - Timestamps
/// - User IDs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Maximum length for a correlation ID.
    pub const MAX_LENGTH: usize = 50;

    /// Creates a new correlation ID.
    ///
    /// # Panics
    ///
    /// Panics if the value exceeds 50 characters or contains invalid characters.
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(
            value.len() <= Self::MAX_LENGTH,
            "CorrelationId must be at most {} characters, got {}",
            Self::MAX_LENGTH,
            value.len()
        );
        assert!(
            is_http_header_safe(&value),
            "CorrelationId must contain only HTTP header-safe characters (alphanumeric, hyphen, underscore, dot, tilde)"
        );
        Self(value)
    }

    /// Creates a new correlation ID, returning `None` if validation fails.
    pub fn try_new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.len() <= Self::MAX_LENGTH && is_http_header_safe(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the correlation ID string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CorrelationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// User agent suffix for request identification.
///
/// Appended to the user agent string to identify the source of requests.
/// Limited to 25 characters and must contain only HTTP header-safe characters
/// (alphanumeric, hyphen, underscore, dot, tilde).
/// Convert from a `String` or `&str` with `TryFrom`; invalid values return
/// a [`CosmosError`] with HTTP 400 status.
///
/// # Server-Side Enforcement
///
/// The Cosmos DB service enforces cardinality limits on user agent suffixes
/// more strictly than client-side correlation IDs. High-cardinality suffixes
/// may be rejected or normalized by the service.
///
/// # Examples
///
/// Good values:
/// - AKS cluster name: `"aks-prod-eastus"`
/// - Azure VM ID (if node count is limited): `"vm-worker-01"`
/// - App identifier with region: `"myapp-westus2"`
/// - Service name: `"order-service"`
///
/// Avoid:
/// - Instance-specific IDs with high cardinality
/// - Timestamps or request IDs
/// - Values that change frequently
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAgentSuffix(String);

impl UserAgentSuffix {
    /// Maximum length for a user agent suffix.
    pub const MAX_LENGTH: usize = 25;

    /// Creates a new user agent suffix, returning `None` if validation fails.
    pub fn try_new(value: impl Into<String>) -> Option<Self> {
        Self::try_from(value.into()).ok()
    }

    /// Returns the user agent suffix string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for UserAgentSuffix {
    type Error = CosmosError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let message = if value.len() > Self::MAX_LENGTH {
            format!(
                "UserAgentSuffix must be at most {} characters, got {}",
                Self::MAX_LENGTH,
                value.len()
            )
        } else if !is_http_header_safe(&value) {
            "UserAgentSuffix must contain only HTTP header-safe characters (alphanumeric, hyphen, underscore, dot, tilde)".to_owned()
        } else {
            return Ok(Self(value));
        };

        Err(CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_USER_AGENT_SUFFIX_INVALID)
            .with_message(message)
            .build())
    }
}

impl TryFrom<&str> for UserAgentSuffix {
    type Error = CosmosError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl AsRef<str> for UserAgentSuffix {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for UserAgentSuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validates a user agent property value: printable ASCII, no leading or
/// trailing space, and none of the metadata-segment delimiters `;`, `(`, `)`.
fn is_valid_property_value(s: &str) -> bool {
    !s.starts_with(' ')
        && !s.ends_with(' ')
        && s.chars()
            .all(|c| matches!(c, ' '..='~') && !matches!(c, ';' | '(' | ')'))
}

/// A custom `key=value` entry added to the metadata segment of the
/// `User-Agent` header.
///
/// Wrapping SDKs use this to report their own metadata, such as a runtime
/// version (`dotnet=8.0.1`). Entries appear inside the parentheses after the
/// driver-owned entries, in the order they were added.
///
/// Keys start with a lowercase ASCII letter and continue with lowercase ASCII
/// letters, digits, hyphens, or underscores, up to
/// [`MAX_KEY_LENGTH`](Self::MAX_KEY_LENGTH) characters. The keys `drv` and `ft`
/// are reserved for the driver. Values are between 1 and
/// [`MAX_VALUE_LENGTH`](Self::MAX_VALUE_LENGTH) printable ASCII characters
/// (including spaces, so `.NET 8.0.1` is fine), with no leading or trailing
/// space. Values can never contain `;`, `(`, or `)`, which delimit the
/// metadata segment. Invalid input returns a [`CosmosError`] with HTTP 400
/// status.
///
/// The header is limited in size: if a property does not fit, it (and any
/// properties after it) is omitted from the `User-Agent`.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::options::UserAgentProperty;
///
/// let property = UserAgentProperty::try_new("rt", ".NET 8.0.1").unwrap();
/// assert_eq!(property.to_string(), "rt=.NET 8.0.1");
/// assert!(UserAgentProperty::try_new("rt", "a;b").is_err());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAgentProperty {
    key: String,
    value: String,
}

impl UserAgentProperty {
    /// Maximum length for a property key.
    pub const MAX_KEY_LENGTH: usize = 16;

    /// Maximum length for a property value.
    pub const MAX_VALUE_LENGTH: usize = 32;

    /// Creates a new property, validating the key and value.
    ///
    /// # Errors
    ///
    /// Returns a [`CosmosError`] with HTTP 400 and substatus 20129
    /// ([`CLIENT_USER_AGENT_PROPERTY_INVALID`](crate::error::status_codes::CLIENT_USER_AGENT_PROPERTY_INVALID))
    /// if the key is invalid or reserved, or the value is empty, too long,
    /// contains non-printable ASCII or a metadata delimiter (`;`, `(`, or `)`),
    /// or has leading or trailing spaces.
    pub fn try_new(key: impl Into<String>, value: impl Into<String>) -> Result<Self, CosmosError> {
        let key = key.into();
        let value = value.into();

        let valid_key = key.len() <= Self::MAX_KEY_LENGTH
            && key.chars().next().is_some_and(|c| c.is_ascii_lowercase())
            && key
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '-' | '_'));
        let message = if !valid_key {
            format!(
                "UserAgentProperty key must be 1-{} characters, start with a lowercase letter, and contain only lowercase letters, digits, hyphen, or underscore",
                Self::MAX_KEY_LENGTH
            )
        } else if key == DRIVER_VERSION_KEY || key == FEATURE_FLAGS_KEY {
            format!("UserAgentProperty key '{key}' is reserved for the driver")
        } else if value.is_empty() || value.len() > Self::MAX_VALUE_LENGTH {
            format!(
                "UserAgentProperty value must be 1-{} characters, got {}",
                Self::MAX_VALUE_LENGTH,
                value.len()
            )
        } else if !is_valid_property_value(&value) {
            "UserAgentProperty value must be printable ASCII without leading or trailing spaces and must not contain ';', '(' or ')'".to_owned()
        } else {
            return Ok(Self { key, value });
        };

        Err(CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_USER_AGENT_PROPERTY_INVALID)
            .with_message(message)
            .build())
    }

    /// Returns the property key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the property value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for UserAgentProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workload_id_valid_range() {
        assert!(WorkloadId::try_new(1).is_some());
        assert!(WorkloadId::try_new(25).is_some());
        assert!(WorkloadId::try_new(50).is_some());
    }

    #[test]
    fn workload_id_invalid_range() {
        assert!(WorkloadId::try_new(0).is_none());
        assert!(WorkloadId::try_new(51).is_none());
        assert!(WorkloadId::try_new(255).is_none());
    }

    #[test]
    #[should_panic(expected = "WorkloadId must be between 1 and 50")]
    fn workload_id_panics_on_zero() {
        WorkloadId::new(0);
    }

    #[test]
    fn correlation_id_valid() {
        let id = CorrelationId::new("aks-prod-eastus-001");
        assert_eq!(id.as_str(), "aks-prod-eastus-001");
    }

    #[test]
    fn correlation_id_max_length() {
        let long_id = "a".repeat(50);
        assert!(CorrelationId::try_new(&long_id).is_some());

        let too_long = "a".repeat(51);
        assert!(CorrelationId::try_new(&too_long).is_none());
    }

    #[test]
    fn correlation_id_invalid_chars() {
        assert!(CorrelationId::try_new("valid-id_123").is_some());
        assert!(CorrelationId::try_new("invalid id").is_none()); // space
        assert!(CorrelationId::try_new("invalid/id").is_none()); // slash
        assert!(CorrelationId::try_new("invalid:id").is_none()); // colon
    }

    #[test]
    fn user_agent_suffix_valid() {
        let suffix = UserAgentSuffix::try_from("myapp-westus2").unwrap();
        assert_eq!(suffix.as_str(), "myapp-westus2");
    }

    #[test]
    fn user_agent_suffix_max_length() {
        let at_limit = "a".repeat(UserAgentSuffix::MAX_LENGTH);
        assert_eq!(
            UserAgentSuffix::try_from(at_limit.clone())
                .unwrap()
                .as_str(),
            at_limit
        );
        assert_eq!(
            UserAgentSuffix::try_from(at_limit.as_str())
                .unwrap()
                .as_str(),
            at_limit
        );

        let too_long = "a".repeat(UserAgentSuffix::MAX_LENGTH + 1);
        for error in [
            UserAgentSuffix::try_from(too_long.as_str()).unwrap_err(),
            UserAgentSuffix::try_from(too_long).unwrap_err(),
        ] {
            assert_eq!(
                error.status(),
                crate::error::status_codes::CLIENT_USER_AGENT_SUFFIX_INVALID
            );
            assert!(error.to_string().contains("at most 25 characters"));
        }
    }

    #[test]
    fn user_agent_suffix_invalid_chars() {
        for valid in ["", "a-Z_09.~"] {
            assert_eq!(UserAgentSuffix::try_from(valid).unwrap().as_str(), valid);
            assert_eq!(
                UserAgentSuffix::try_from(valid.to_owned())
                    .unwrap()
                    .as_str(),
                valid
            );
        }
        for invalid in ["invalid suffix", "invalid/suffix", "é", "bad\nheader"] {
            for error in [
                UserAgentSuffix::try_from(invalid).unwrap_err(),
                UserAgentSuffix::try_from(invalid.to_owned()).unwrap_err(),
            ] {
                assert_eq!(
                    error.status(),
                    crate::error::status_codes::CLIENT_USER_AGENT_SUFFIX_INVALID
                );
                assert!(error.to_string().contains("HTTP header-safe"));
                assert!(!error.to_string().contains(invalid));
            }
        }
    }

    #[test]
    fn user_agent_property_valid() {
        let p = UserAgentProperty::try_new("dotnet", "8.0.1").unwrap();
        assert_eq!(p.key(), "dotnet");
        assert_eq!(p.value(), "8.0.1");
        assert_eq!(p.to_string(), "dotnet=8.0.1");
    }

    #[test]
    fn user_agent_property_allows_spaces_and_punctuation() {
        let p = UserAgentProperty::try_new("rt", ".NET 8.0.1").unwrap();
        assert_eq!(p.to_string(), "rt=.NET 8.0.1");
        assert!(UserAgentProperty::try_new("os", "Ubuntu 24.04+lts/x=y,z").is_ok());
    }

    #[test]
    fn user_agent_property_rejects_invalid_input() {
        for (key, value) in [
            ("", "1"),
            ("Dotnet", "1"),
            ("1abc", "1"),
            ("a b", "1"),
            ("a=b", "1"),
            ("a;b", "1"),
            ("drv", "1"),
            ("ft", "1"),
            (&"k".repeat(UserAgentProperty::MAX_KEY_LENGTH + 1), "1"),
            ("k", ""),
            ("k", " leading"),
            ("k", "trailing "),
            ("k", "tab\there"),
            ("k", "caf\u{e9}"),
            ("k", "a;b"),
            ("k", "(x)"),
            ("k", &"v".repeat(UserAgentProperty::MAX_VALUE_LENGTH + 1)),
        ] {
            let err = UserAgentProperty::try_new(key, value).unwrap_err();
            assert_eq!(
                err.status(),
                crate::error::status_codes::CLIENT_USER_AGENT_PROPERTY_INVALID,
                "{key:?}={value:?}"
            );
        }
    }
}
