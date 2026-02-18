// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{net::IpAddr, time::Duration};

use super::env_parsing::{
    parse_duration_millis_from_env, parse_from_env, parse_optional_duration_millis_from_env,
    ValidationBounds,
};
use crate::options::EmulatorServerCertValidation;

/// Configuration for connection pooling behavior.
///
/// Controls how the driver manages connections to Cosmos DB endpoints.
/// This type is immutable after construction - use [`ConnectionPoolOptionsBuilder`]
/// to create instances with custom values.
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::ConnectionPoolOptions;
/// use std::time::Duration;
///
/// let options = ConnectionPoolOptions::builder()
///     .with_max_idle_connections_per_endpoint(5_000)
///     .with_max_connect_timeout(Duration::from_secs(3))
///     .build()
///     .expect("valid options");
///
/// // Access via getters
/// assert_eq!(options.max_idle_connections_per_endpoint(), 5_000);
/// assert_eq!(options.max_connect_timeout(), Duration::from_secs(3));
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ConnectionPoolOptions {
    is_proxy_allowed: bool,

    min_connect_timeout: Duration,
    max_connect_timeout: Duration,

    min_dataplane_request_timeout: Duration,
    max_dataplane_request_timeout: Duration,
    min_metadata_request_timeout: Duration,
    max_metadata_request_timeout: Duration,

    max_idle_connections_per_endpoint: usize,

    idle_connection_timeout: Option<Duration>,

    is_http2_allowed: bool,

    is_gateway20_allowed: bool,

    emulator_server_cert_validation: EmulatorServerCertValidation,

    local_address: Option<IpAddr>,
}

impl Default for ConnectionPoolOptions {
    fn default() -> Self {
        ConnectionPoolOptionsBuilder::new()
            .build()
            .expect("Default ConnectionPoolOptions should always be valid")
    }
}

impl ConnectionPoolOptions {
    /// Creates a new builder for `ConnectionPoolOptions`.
    pub fn builder() -> ConnectionPoolOptionsBuilder {
        ConnectionPoolOptionsBuilder::new()
    }

    /// Returns whether proxy usage is allowed.
    ///
    /// When `true`, the `HTTPS_PROXY` environment variable will be respected.
    /// When using a proxy, no end-to-end SLAs are guaranteed by Azure Cosmos DB.
    pub fn is_proxy_allowed(&self) -> bool {
        self.is_proxy_allowed
    }

    /// Returns the minimum connection timeout.
    pub fn min_connect_timeout(&self) -> Duration {
        self.min_connect_timeout
    }

    /// Returns the maximum connection timeout.
    pub fn max_connect_timeout(&self) -> Duration {
        self.max_connect_timeout
    }

    /// Returns the minimum data plane request timeout.
    pub fn min_dataplane_request_timeout(&self) -> Duration {
        self.min_dataplane_request_timeout
    }

    /// Returns the maximum data plane request timeout.
    pub fn max_dataplane_request_timeout(&self) -> Duration {
        self.max_dataplane_request_timeout
    }

    /// Returns the minimum metadata request timeout.
    pub fn min_metadata_request_timeout(&self) -> Duration {
        self.min_metadata_request_timeout
    }

    /// Returns the maximum metadata request timeout.
    pub fn max_metadata_request_timeout(&self) -> Duration {
        self.max_metadata_request_timeout
    }

    /// Returns the maximum number of idle connections per endpoint.
    pub fn max_idle_connections_per_endpoint(&self) -> usize {
        self.max_idle_connections_per_endpoint
    }

    /// Returns the idle connection timeout, if set.
    pub fn idle_connection_timeout(&self) -> Option<Duration> {
        self.idle_connection_timeout
    }

    /// Returns whether HTTP/2 is allowed for gateway mode connections.
    pub fn is_http2_allowed(&self) -> bool {
        self.is_http2_allowed
    }

    /// Returns whether Gateway 2.0 feature is allowed.
    ///
    /// If `true`, the driver will use Gateway 2.0 features when communicating
    /// with the Cosmos DB service (if the account supports it). Gateway 2.0
    /// requires HTTP/2, so this returns `false` if HTTP/2 is disabled.
    pub fn is_gateway20_allowed(&self) -> bool {
        self.is_gateway20_allowed
    }

    /// Returns the emulator server certificate validation setting.
    ///
    /// This only takes effect when connecting to localhost/emulator endpoints.
    pub fn emulator_server_cert_validation(&self) -> EmulatorServerCertValidation {
        self.emulator_server_cert_validation
    }

    /// Returns the local IP address to bind to, if set.
    pub fn local_address(&self) -> Option<IpAddr> {
        self.local_address
    }
}

/// Builder for [`ConnectionPoolOptions`].
///
/// Default values are read from environment variables when available,
/// and can be overridden using builder methods.
///
/// # Environment Variables
///
/// - `AZURE_COSMOS_CONNECTION_POOL_IS_PROXY_ALLOWED`: Whether proxy usage is allowed (default: `false`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MIN_CONNECT_TIMEOUT_MS`: Minimum connection timeout in milliseconds (default: `100`, min: `100`, max: `6_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_CONNECT_TIMEOUT_MS`: Maximum connection timeout in milliseconds (default: `5_000`, min: `100`, max: `6_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MIN_DATAPLANE_REQUEST_TIMEOUT_MS`: Minimum data plane request timeout in milliseconds (default: `100`, min: `100`, max: `65_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_DATAPLANE_REQUEST_TIMEOUT_MS`: Maximum data plane request timeout in milliseconds (default: `6_000`, min: `100`, max: `65_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MIN_METADATA_REQUEST_TIMEOUT_MS`: Minimum metadata request timeout in milliseconds (default: `100`, min: `100`, max: `65_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_METADATA_REQUEST_TIMEOUT_MS`: Maximum metadata request timeout in milliseconds (default: `65_000`, min: `100`, max: `65_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_IDLE_CONNECTIONS_PER_ENDPOINT`: Maximum idle connections per endpoint (default: `1_000` if HTTP/2 is allowed, `10_000` otherwise, min: `10`, max: `64_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_IDLE_CONNECTION_TIMEOUT_MS`: Idle connection timeout in milliseconds (default: none, min: `300_000` when set)
/// - `AZURE_COSMOS_CONNECTION_POOL_IS_HTTP2_ALLOWED`: Whether HTTP/2 is allowed for gateway mode connections (default: `true`)
/// - `AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED`: Whether Gateway 2.0 feature is allowed (default: `false`)
/// - `AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED`: Whether server certificate validation is disabled for emulator; `true` maps to [`EmulatorServerCertValidation::DANGEROUS_DISABLED`], `false` to [`EmulatorServerCertValidation::ENABLED`] (default: `false`)
/// - `AZURE_COSMOS_LOCAL_ADDRESS`: Local IP address to bind to (default: none)
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::ConnectionPoolOptions;
///
/// let options = ConnectionPoolOptions::builder()
///     .with_max_idle_connections_per_endpoint(5_000)
///     .build()
///     .expect("valid options");
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct ConnectionPoolOptionsBuilder {
    is_proxy_allowed: Option<bool>,
    min_connect_timeout: Option<Duration>,
    max_connect_timeout: Option<Duration>,
    min_dataplane_request_timeout: Option<Duration>,
    max_dataplane_request_timeout: Option<Duration>,
    min_metadata_request_timeout: Option<Duration>,
    max_metadata_request_timeout: Option<Duration>,
    max_idle_connections_per_endpoint: Option<usize>,
    idle_connection_timeout: Option<Duration>,
    is_http2_allowed: Option<bool>,
    is_gateway20_allowed: Option<bool>,
    emulator_server_cert_validation: Option<EmulatorServerCertValidation>,
    local_address: Option<IpAddr>,
}

impl ConnectionPoolOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether proxy usage is allowed. If true, the HTTPS_PROXY environment variable will be respected.
    /// When using a proxy, no e2e SLAs are guaranteed by Azure Cosmos DB.
    /// Defaults to `false`.
    pub fn with_dangerous_is_proxy_allowed(mut self, value: bool) -> Self {
        self.is_proxy_allowed = Some(value);
        self
    }

    /// Sets the minimum connection timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 100ms.
    pub fn with_min_connect_timeout(mut self, timeout: Duration) -> Self {
        self.min_connect_timeout = Some(timeout);
        self
    }

    /// Sets the maximum connection timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 5_000ms (5 seconds).
    pub fn with_max_connect_timeout(mut self, timeout: Duration) -> Self {
        self.max_connect_timeout = Some(timeout);
        self
    }

    /// Sets the minimum data plane request timeout.
    ///
    /// Must be between 100ms and 65_000ms (65 seconds).
    /// Default: 100ms.
    pub fn with_min_dataplane_request_timeout(mut self, timeout: Duration) -> Self {
        self.min_dataplane_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum data plane request timeout.
    ///
    /// Must be at least 100ms.
    /// Default: 6 seconds.
    pub fn with_max_dataplane_request_timeout(mut self, timeout: Duration) -> Self {
        self.max_dataplane_request_timeout = Some(timeout);
        self
    }

    /// Sets the minimum metadata request timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 100ms.
    pub fn with_min_metadata_request_timeout(mut self, timeout: Duration) -> Self {
        self.min_metadata_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum metadata request timeout.
    ///
    /// Must be between 100ms and 65_000ms (65 seconds).
    /// Default: 65_000ms (65 seconds).
    pub fn with_max_metadata_request_timeout(mut self, timeout: Duration) -> Self {
        self.max_metadata_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum number of idle connections per endpoint.
    ///
    /// Must be between 10 and 64_000.
    /// Default: 1_000 if HTTP/2 is allowed, 10_000 otherwise.
    pub fn with_max_idle_connections_per_endpoint(mut self, count: usize) -> Self {
        self.max_idle_connections_per_endpoint = Some(count);
        self
    }

    /// Sets the idle connection timeout.
    ///
    /// Must be at least 300_000ms (5 minutes) when set.
    /// Default: none (connections are never closed due to idleness).
    pub fn with_idle_connection_timeout(mut self, timeout: Duration) -> Self {
        self.idle_connection_timeout = Some(timeout);
        self
    }

    /// Sets whether HTTP/2 is allowed for gateway mode connections.
    pub fn with_is_http2_allowed(mut self, value: bool) -> Self {
        self.is_http2_allowed = Some(value);
        self
    }

    /// Sets whether Gateway 2.0 feature is allowed.
    pub fn with_is_gateway20_allowed(mut self, value: bool) -> Self {
        self.is_gateway20_allowed = Some(value);
        self
    }

    /// Sets the emulator server certificate validation behavior.
    ///
    /// Use [`EmulatorServerCertValidation::DANGEROUS_DISABLED`] to skip TLS certificate
    /// validation when connecting to a local Cosmos DB emulator with a self-signed certificate.
    pub fn with_emulator_server_cert_validation(
        mut self,
        value: EmulatorServerCertValidation,
    ) -> Self {
        self.emulator_server_cert_validation = Some(value);
        self
    }

    /// Sets the local IP address to bind to.
    pub fn with_local_address(mut self, addr: IpAddr) -> Self {
        self.local_address = Some(addr);
        self
    }

    /// Builds the `ConnectionPoolOptions` with configured values.
    ///
    /// Unset values are populated from environment variables or use sensible defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any duration is less than 100 milliseconds
    /// - `max_idle_connections_per_endpoint` is zero
    /// - Environment variable parsing fails
    pub fn build(self) -> azure_core::Result<ConnectionPoolOptions> {
        let effective_is_http2_allowed = parse_from_env(
            self.is_http2_allowed,
            "AZURE_COSMOS_CONNECTION_POOL_IS_HTTP2_ALLOWED",
            true,
            ValidationBounds::none(),
        )?;

        let effective_is_gateway20_allowed = if let Some(gateway20) = self.is_gateway20_allowed {
            gateway20 && effective_is_http2_allowed
        } else {
            match std::env::var("AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED") {
                Ok(v) => {
                    let gateway20: bool = v.parse().map_err(|e| {
                        azure_core::Error::with_message(
                            azure_core::error::ErrorKind::DataConversion,
                            format!(
                                "Failed to parse AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED as boolean: {} ({})",
                                v, e
                            ),
                        )
                    })?;
                    gateway20 && effective_is_http2_allowed
                }
                Err(_) => false, // TODO: Change to true before GA
            }
        };

        let max_connection_pool_size_default = if effective_is_http2_allowed {
            1_000
        } else {
            10_000
        };

        let min_connect_timeout = parse_duration_millis_from_env(
            self.min_connect_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MIN_CONNECT_TIMEOUT_MS",
            100,
            100,
            6_000,
        )?;

        let max_connect_timeout = parse_duration_millis_from_env(
            self.max_connect_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_CONNECT_TIMEOUT_MS",
            5_000,
            100,
            6_000,
        )?;

        let min_dataplane_request_timeout = parse_duration_millis_from_env(
            self.min_dataplane_request_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MIN_DATAPLANE_REQUEST_TIMEOUT_MS",
            100,
            100,
            65_000,
        )?;

        let max_dataplane_request_timeout = parse_duration_millis_from_env(
            self.max_dataplane_request_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_DATAPLANE_REQUEST_TIMEOUT_MS",
            6_000,
            100,
            u64::MAX,
        )?;

        let min_metadata_request_timeout = parse_duration_millis_from_env(
            self.min_metadata_request_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MIN_METADATA_REQUEST_TIMEOUT_MS",
            100,
            100,
            6_000,
        )?;

        let max_metadata_request_timeout = parse_duration_millis_from_env(
            self.max_metadata_request_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_METADATA_REQUEST_TIMEOUT_MS",
            65_000,
            100,
            65_000,
        )?;

        let max_idle_connections_per_endpoint = parse_from_env(
            self.max_idle_connections_per_endpoint,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_IDLE_CONNECTIONS_PER_ENDPOINT",
            max_connection_pool_size_default,
            ValidationBounds::range(10, 64_000),
        )?;

        let idle_connection_timeout = parse_optional_duration_millis_from_env(
            self.idle_connection_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_IDLE_CONNECTION_TIMEOUT_MS",
            300_000,
            u64::MAX,
        )?;

        Ok(ConnectionPoolOptions {
            is_proxy_allowed: parse_from_env(
                self.is_proxy_allowed,
                "AZURE_COSMOS_CONNECTION_POOL_IS_PROXY_ALLOWED",
                false,
                ValidationBounds::none(),
            )?,
            min_connect_timeout,
            max_connect_timeout,
            min_dataplane_request_timeout,
            max_dataplane_request_timeout,
            min_metadata_request_timeout,
            max_metadata_request_timeout,
            max_idle_connections_per_endpoint,
            idle_connection_timeout,
            is_http2_allowed: effective_is_http2_allowed,
            is_gateway20_allowed: effective_is_gateway20_allowed,
            emulator_server_cert_validation: match self.emulator_server_cert_validation {
                Some(v) => v,
                None => EmulatorServerCertValidation::from(parse_from_env(
                    None::<bool>,
                    "AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED",
                    false,
                    ValidationBounds::none(),
                )?),
            },
            local_address: match self.local_address {
                Some(addr) => Some(addr),
                None => match std::env::var("AZURE_COSMOS_LOCAL_ADDRESS") {
                    Ok(v) => Some(v.parse().map_err(|e| {
                        azure_core::Error::with_message(
                            azure_core::error::ErrorKind::DataConversion,
                            format!(
                                "Failed to parse AZURE_COSMOS_LOCAL_ADDRESS as IP address: {} ({})",
                                v, e
                            ),
                        )
                    })?),
                    Err(_) => None,
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_pool_options_builder_defaults() {
        let options = ConnectionPoolOptionsBuilder::new().build().unwrap();

        assert!(!options.is_proxy_allowed());
        assert_eq!(options.min_connect_timeout(), Duration::from_millis(100));
        assert_eq!(options.max_connect_timeout(), Duration::from_millis(5_000));
        assert_eq!(
            options.min_dataplane_request_timeout(),
            Duration::from_millis(100)
        );
        assert_eq!(
            options.max_dataplane_request_timeout(),
            Duration::from_millis(6_000)
        );
        assert_eq!(
            options.min_metadata_request_timeout(),
            Duration::from_millis(100)
        );
        assert_eq!(
            options.max_metadata_request_timeout(),
            Duration::from_millis(65_000)
        );
        assert!(options.is_http2_allowed());
        assert!(!options.is_gateway20_allowed());
        assert_eq!(
            options.emulator_server_cert_validation(),
            EmulatorServerCertValidation::ENABLED
        );
        assert_eq!(options.idle_connection_timeout(), None);
        assert_eq!(options.local_address(), None);
        // Default is 1_000 when HTTP/2 is allowed (which is true by default)
        assert_eq!(options.max_idle_connections_per_endpoint(), 1_000);
    }

    #[test]
    fn connection_pool_options_builder_custom_values() {
        let options = ConnectionPoolOptionsBuilder::new()
            .with_dangerous_is_proxy_allowed(true)
            .with_min_connect_timeout(Duration::from_millis(200))
            .with_max_connect_timeout(Duration::from_millis(3_000))
            .with_min_dataplane_request_timeout(Duration::from_millis(500))
            .with_max_dataplane_request_timeout(Duration::from_millis(10_000))
            .with_min_metadata_request_timeout(Duration::from_millis(150))
            .with_max_metadata_request_timeout(Duration::from_millis(30_000))
            .with_max_idle_connections_per_endpoint(5_000)
            .with_idle_connection_timeout(Duration::from_millis(600_000))
            .with_is_http2_allowed(false)
            .with_is_gateway20_allowed(true)
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DANGEROUS_DISABLED)
            .build()
            .unwrap();

        assert!(options.is_proxy_allowed());
        assert_eq!(options.min_connect_timeout(), Duration::from_millis(200));
        assert_eq!(options.max_connect_timeout(), Duration::from_millis(3_000));
        assert_eq!(
            options.min_dataplane_request_timeout(),
            Duration::from_millis(500)
        );
        assert_eq!(
            options.max_dataplane_request_timeout(),
            Duration::from_millis(10_000)
        );
        assert_eq!(
            options.min_metadata_request_timeout(),
            Duration::from_millis(150)
        );
        assert_eq!(
            options.max_metadata_request_timeout(),
            Duration::from_millis(30_000)
        );
        assert_eq!(options.max_idle_connections_per_endpoint(), 5_000);
        assert_eq!(
            options.idle_connection_timeout(),
            Some(Duration::from_millis(600_000))
        );
        assert!(!options.is_http2_allowed());
        // gateway20 is set to true but HTTP/2 is false, so it should be false
        assert!(!options.is_gateway20_allowed());
        assert_eq!(
            options.emulator_server_cert_validation(),
            EmulatorServerCertValidation::DANGEROUS_DISABLED
        );
    }

    #[test]
    fn min_connect_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_connect_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_connect_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn min_connect_timeout_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_connect_timeout(Duration::from_millis(7_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_connect_timeout_ms must be at most 6000ms"));
    }

    #[test]
    fn max_connect_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_connect_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_connect_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn max_connect_timeout_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_connect_timeout(Duration::from_millis(7_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_connect_timeout_ms must be at most 6000ms"));
    }

    #[test]
    fn min_dataplane_request_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_dataplane_request_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_dataplane_request_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn min_dataplane_request_timeout_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_dataplane_request_timeout(Duration::from_millis(70_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_dataplane_request_timeout_ms must be at most 65000ms"));
    }

    #[test]
    fn max_dataplane_request_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_dataplane_request_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_dataplane_request_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn min_metadata_request_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_metadata_request_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_metadata_request_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn min_metadata_request_timeout_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_metadata_request_timeout(Duration::from_millis(7_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_metadata_request_timeout_ms must be at most 6000ms"));
    }

    #[test]
    fn max_metadata_request_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_metadata_request_timeout(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_metadata_request_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn max_metadata_request_timeout_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_metadata_request_timeout(Duration::from_millis(70_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_metadata_request_timeout_ms must be at most 65000ms"));
    }

    #[test]
    fn max_idle_connections_per_endpoint_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_idle_connections_per_endpoint(5)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_idle_connections_per_endpoint must be at least 10"));
    }

    #[test]
    fn max_idle_connections_per_endpoint_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_idle_connections_per_endpoint(65_000)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_idle_connections_per_endpoint must be at most 64000"));
    }

    #[test]
    fn idle_connection_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_idle_connection_timeout(Duration::from_millis(100_000))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("idle_connection_timeout_ms must be at least 300000ms"));
    }

    #[test]
    fn gateway20_requires_http2() {
        let options = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(false)
            .with_is_gateway20_allowed(true)
            .build()
            .unwrap();

        // Gateway 2.0 should be disabled if HTTP/2 is not allowed
        assert!(!options.is_gateway20_allowed());
    }

    #[test]
    fn http2_disabled_changes_max_connection_pool_default() {
        let options = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(false)
            .build()
            .unwrap();

        // When HTTP/2 is disabled, default should be 10_000
        assert_eq!(options.max_idle_connections_per_endpoint(), 10_000);
    }
}
