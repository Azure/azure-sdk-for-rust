// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request policy types.

use crate::options::Region;
use std::time::Duration;

const MIN_END_TO_END_OPERATION_TIMEOUT: Duration = Duration::from_secs(1);

/// Controls whether the response body is returned for write operations.
///
/// When disabled, reduces networking and CPU load by not sending the payload
/// back over the network. Does not impact RU usage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ContentResponseOnWrite {
    /// Content response is enabled (response body returned).
    Enabled,
    /// Content response is disabled (no response body).
    #[default]
    Disabled,
}

impl From<bool> for ContentResponseOnWrite {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<ContentResponseOnWrite> for bool {
    fn from(value: ContentResponseOnWrite) -> Self {
        matches!(value, ContentResponseOnWrite::Enabled)
    }
}

impl std::str::FromStr for ContentResponseOnWrite {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "true" | "enabled" => Ok(Self::Enabled),
            "false" | "disabled" => Ok(Self::Disabled),
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!(
                    "Unknown content response on write value: '{}'. Expected 'true'/'false' or 'enabled'/'disabled'",
                    s
                ),
            )),
        }
    }
}

impl std::fmt::Display for ContentResponseOnWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enabled => f.write_str("Enabled"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}

/// Configuration for end-to-end operation latency policy.
///
/// Specifies the maximum time an operation can take, including all retries.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndToEndOperationLatencyPolicy {
    /// Maximum end-to-end timeout for the operation.
    timeout: Duration,
}

impl EndToEndOperationLatencyPolicy {
    /// Creates a new end-to-end operation latency policy with the given timeout.
    ///
    /// Timeouts below 1 second are clamped to 1 second at the public API boundary
    /// to avoid unrealistic operation-level timeout settings.
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout: timeout.max(MIN_END_TO_END_OPERATION_TIMEOUT),
        }
    }

    /// Returns the maximum end-to-end timeout for the operation.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

impl From<Duration> for EndToEndOperationLatencyPolicy {
    fn from(timeout: Duration) -> Self {
        Self::new(timeout)
    }
}

/// List of regions to exclude from request routing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExcludedRegions(pub Vec<Region>);

impl ExcludedRegions {
    /// Creates a new empty excluded regions list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a region to exclude.
    pub fn with_region(mut self, region: impl Into<Region>) -> Self {
        self.0.push(region.into());
        self
    }

    /// Returns true if no regions are excluded.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of excluded regions.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the excluded regions.
    pub fn iter(&self) -> impl Iterator<Item = &Region> {
        self.0.iter()
    }
}

impl<T: Into<Region>> FromIterator<T> for ExcludedRegions {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

/// Controls whether JavaScript stored procedure logging is enabled.
///
/// When enabled, script logs from stored procedures are included in the response.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScriptLoggingEnabled {
    /// Script logging is enabled.
    Enabled,
    /// Script logging is disabled.
    #[default]
    Disabled,
}

impl From<bool> for ScriptLoggingEnabled {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<ScriptLoggingEnabled> for bool {
    fn from(value: ScriptLoggingEnabled) -> Self {
        matches!(value, ScriptLoggingEnabled::Enabled)
    }
}

/// Controls whether quota information is included in responses.
///
/// When enabled, container quota stats are returned in the response headers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum QuotaInfoEnabled {
    /// Quota info is enabled.
    Enabled,
    /// Quota info is disabled.
    #[default]
    Disabled,
}

impl From<bool> for QuotaInfoEnabled {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<QuotaInfoEnabled> for bool {
    fn from(value: QuotaInfoEnabled) -> Self {
        matches!(value, QuotaInfoEnabled::Enabled)
    }
}

/// Controls whether TLS server certificate validation is performed for Cosmos DB emulator connections.
///
/// By default, certificate validation is enabled. Disabling it is **dangerous** and should only be
/// used when connecting to the local Cosmos DB emulator, which uses a self-signed certificate.
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::EmulatorServerCertValidation;
///
/// // Safe default: validation is enabled.
/// let validation = EmulatorServerCertValidation::Enabled;
///
/// // Dangerous: disables certificate validation for emulator use.
/// let validation = EmulatorServerCertValidation::DangerousDisabled;
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EmulatorServerCertValidation {
    /// Certificate validation is enabled (default, safe).
    #[default]
    Enabled,
    /// Certificate validation is disabled (**dangerous** — only for local emulator connections).
    DangerousDisabled,
}

impl From<bool> for EmulatorServerCertValidation {
    /// Converts a `bool` where `true` means validation is disabled (dangerous).
    fn from(disabled: bool) -> Self {
        if disabled {
            Self::DangerousDisabled
        } else {
            Self::Enabled
        }
    }
}

impl From<EmulatorServerCertValidation> for bool {
    fn from(value: EmulatorServerCertValidation) -> Self {
        matches!(value, EmulatorServerCertValidation::DangerousDisabled)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::EndToEndOperationLatencyPolicy;

    #[test]
    fn end_to_end_latency_policy_clamps_timeout_below_one_second() {
        let policy = EndToEndOperationLatencyPolicy::new(Duration::from_millis(250));
        assert_eq!(policy.timeout(), Duration::from_secs(1));
    }

    #[test]
    fn end_to_end_latency_policy_keeps_timeout_at_or_above_one_second() {
        let policy = EndToEndOperationLatencyPolicy::new(Duration::from_secs(2));
        assert_eq!(policy.timeout(), Duration::from_secs(2));
    }
}
