// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request policy types.

use crate::{driver::transport::is_emulator_host, models::AccountEndpoint, options::Region};
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
    type Err = crate::error::CosmosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "true" | "enabled" => Ok(Self::Enabled),
            "false" | "disabled" => Ok(Self::Disabled),
            _ => Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message(format!(
                    "Unknown content response on write value: '{s}'. Expected 'true'/'false' or 'enabled'/'disabled'"
                )).build()),
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

/// Controls whether TLS server certificate validation is performed for Cosmos DB connections.
///
/// By default, certificate validation is enabled. Disabling it is **dangerous** and should only be
/// used when connecting to the local Cosmos DB emulator, which uses a self-signed certificate.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ServerCertificateValidation {
    /// Certificate validation is required
    #[default]
    Required,
    /// Certificate validation is required, unless connecting to the emulator.
    ///
    /// The Cosmos DB SDK will detect an emulator connection by comparing the hostname or IP address
    /// to a known value. If you are connecting to an emulator on a different IP address, you can use
    /// the `AZURE_COSMOS_EMULATOR_HOST` environment variable to allow insecure connections to an emulator
    /// at a non-standard host name or IP address.
    RequiredUnlessEmulator,
}

impl ServerCertificateValidation {
    pub(crate) fn allows_insecure_connection(self, endpoint: &AccountEndpoint) -> bool {
        self == ServerCertificateValidation::RequiredUnlessEmulator && is_emulator_host(endpoint)
    }
}

/// Selects the TLS backend used by the officially-supported `reqwest` transport.
///
/// The driver does not expose direct access to the underlying HTTP transport, so
/// this option is the supported mechanism for asserting a specific TLS backend.
/// The default is [`TlsBackend::Rustls`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TlsBackend {
    /// Use the `rustls` TLS backend.
    #[default]
    Rustls,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{EndToEndOperationLatencyPolicy, ServerCertificateValidation};
    use crate::models::AccountEndpoint;

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

    // ServerCertificateValidation tests

    #[test]
    fn required_validation_does_not_allow_insecure_with_production_endpoint() {
        let endpoint = AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/")
            .expect("Failed to create production endpoint");
        let validation = ServerCertificateValidation::Required;

        assert!(!validation.allows_insecure_connection(&endpoint));
    }

    #[test]
    fn required_validation_does_not_allow_insecure_with_emulator_endpoint() {
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/")
            .expect("Failed to create emulator endpoint");
        let validation = ServerCertificateValidation::Required;

        assert!(!validation.allows_insecure_connection(&endpoint));
    }

    #[test]
    fn required_unless_emulator_does_not_allow_insecure_with_production_endpoint() {
        let endpoint = AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/")
            .expect("Failed to create production endpoint");
        let validation = ServerCertificateValidation::RequiredUnlessEmulator;

        assert!(!validation.allows_insecure_connection(&endpoint));
    }

    #[test]
    fn required_unless_emulator_allows_insecure_with_known_emulator_endpoints() {
        const EMULATOR_ENDPOINTS: &[&str] = &[
            "https://localhost:8081/",
            "https://127.0.0.1:8081/",
            "https://[::1]:8081/",
            "https://[0:0:0:0:0:0:0:1]:8081/",
        ];

        let validation = ServerCertificateValidation::RequiredUnlessEmulator;

        for endpoint_url in EMULATOR_ENDPOINTS {
            let endpoint = AccountEndpoint::try_from(*endpoint_url)
                .expect("Failed to create emulator endpoint");
            assert!(
                validation.allows_insecure_connection(&endpoint),
                "Expected insecure connection to be allowed for endpoint: {}",
                endpoint_url
            );
        }
    }
}
