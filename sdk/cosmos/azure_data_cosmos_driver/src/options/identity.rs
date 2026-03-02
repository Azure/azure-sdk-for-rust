// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Identity and telemetry configuration types.
//!
//! This module contains configuration options for identifying requests:
//! - [`WorkloadId`] - Workload identifier for resource governance
//! - [`CorrelationId`] - Client-side metrics correlation
//! - [`UserAgentSuffix`] - Suffix appended to the user agent string
//!
//! The computed [`UserAgent`](crate::models::UserAgent) type is in the models module.

use std::fmt;

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
///
/// If [`CorrelationId`] is not set, this suffix is used as the correlation
/// dimension for client-side metrics.
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

    /// Creates a new user agent suffix.
    ///
    /// # Panics
    ///
    /// Panics if the value exceeds 25 characters or contains invalid characters.
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(
            value.len() <= Self::MAX_LENGTH,
            "UserAgentSuffix must be at most {} characters, got {}",
            Self::MAX_LENGTH,
            value.len()
        );
        assert!(
            is_http_header_safe(&value),
            "UserAgentSuffix must contain only HTTP header-safe characters (alphanumeric, hyphen, underscore, dot, tilde)"
        );
        Self(value)
    }

    /// Creates a new user agent suffix, returning `None` if validation fails.
    pub fn try_new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.len() <= Self::MAX_LENGTH && is_http_header_safe(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the user agent suffix string.
    pub fn as_str(&self) -> &str {
        &self.0
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
        let suffix = UserAgentSuffix::new("myapp-westus2");
        assert_eq!(suffix.as_str(), "myapp-westus2");
    }

    #[test]
    fn user_agent_suffix_max_length() {
        let long_suffix = "a".repeat(25);
        assert!(UserAgentSuffix::try_new(&long_suffix).is_some());

        let too_long = "a".repeat(26);
        assert!(UserAgentSuffix::try_new(&too_long).is_none());
    }

    #[test]
    fn user_agent_suffix_invalid_chars() {
        assert!(UserAgentSuffix::try_new("valid-suffix").is_some());
        assert!(UserAgentSuffix::try_new("invalid suffix").is_none()); // space
    }
}
