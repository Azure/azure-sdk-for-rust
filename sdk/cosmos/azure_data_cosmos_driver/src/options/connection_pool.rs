// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{net::IpAddr, time::Duration};

use super::env_parsing::{
    parse_duration_millis_from_env, parse_from_env, parse_optional_duration_millis_from_env,
    parse_optional_from_env, ValidationBounds,
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

    max_http2_streams_per_client: u32,
    max_http2_connections_per_endpoint: usize,
    min_http2_connections_per_endpoint: usize,
    idle_http2_client_timeout: Duration,
    http2_health_check_interval: Duration,
    http2_consecutive_failure_threshold: u32,
    http2_eviction_grace_period: Duration,
    http2_keep_alive_interval: Duration,
    http2_keep_alive_timeout: Duration,
    tcp_keepalive_time: Option<Duration>,
    tcp_keepalive_interval: Option<Duration>,
    tcp_keepalive_retries: Option<u32>,

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

    /// Returns the per-shard HTTP/2 stream budget before another shard is used.
    pub fn max_http2_streams_per_client(&self) -> u32 {
        self.max_http2_streams_per_client
    }

    /// Returns the maximum number of HTTP/2 shard clients per endpoint.
    pub fn max_http2_connections_per_endpoint(&self) -> usize {
        self.max_http2_connections_per_endpoint
    }

    /// Returns the minimum number of HTTP/2 shard clients per endpoint.
    pub fn min_http2_connections_per_endpoint(&self) -> usize {
        self.min_http2_connections_per_endpoint
    }

    /// Returns how long an overflow HTTP/2 shard must stay idle before reclaim.
    pub fn idle_http2_client_timeout(&self) -> Duration {
        self.idle_http2_client_timeout
    }

    /// Returns how often the HTTP/2 background health sweep runs.
    pub fn http2_health_check_interval(&self) -> Duration {
        self.http2_health_check_interval
    }

    /// Returns the consecutive failure threshold that marks a shard unhealthy.
    pub fn http2_consecutive_failure_threshold(&self) -> u32 {
        self.http2_consecutive_failure_threshold
    }

    /// Returns the grace period after the last success before an unhealthy shard can be evicted.
    pub fn http2_eviction_grace_period(&self) -> Duration {
        self.http2_eviction_grace_period
    }

    /// Returns the HTTP/2 keep-alive ping interval.
    pub fn http2_keep_alive_interval(&self) -> Duration {
        self.http2_keep_alive_interval
    }

    /// Returns the HTTP/2 keep-alive ping timeout.
    pub fn http2_keep_alive_timeout(&self) -> Duration {
        self.http2_keep_alive_timeout
    }

    /// Returns the TCP keepalive time. Defaults to 1 second.
    pub fn tcp_keepalive_time(&self) -> Option<Duration> {
        self.tcp_keepalive_time
    }

    /// Returns the TCP keepalive probe interval. Defaults to 1 second.
    pub fn tcp_keepalive_interval(&self) -> Option<Duration> {
        self.tcp_keepalive_interval
    }

    /// Returns the TCP keepalive probe retry count, if enabled.
    pub fn tcp_keepalive_retries(&self) -> Option<u32> {
        self.tcp_keepalive_retries
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
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_HTTP2_STREAMS_PER_CLIENT`: Maximum concurrent streams per HTTP/2 shard client (default: `16`, min: `1`, max: `20`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MAX_HTTP2_CONNECTIONS_PER_ENDPOINT`: Maximum number of HTTP/2 shard clients per endpoint (default: `available_parallelism * 2`, fallback: `32`, min: `1`, max: `256`)
/// - `AZURE_COSMOS_CONNECTION_POOL_MIN_HTTP2_CONNECTIONS_PER_ENDPOINT`: Minimum number of HTTP/2 shard clients per endpoint (default: `1`, min: `1`, max: `256`)
/// - `AZURE_COSMOS_CONNECTION_POOL_IDLE_HTTP2_CLIENT_TIMEOUT_MS`: Idle timeout for overflow HTTP/2 shard clients in milliseconds (default: `60_000`, min: `1_000`)
/// - `AZURE_COSMOS_CONNECTION_POOL_HTTP2_HEALTH_CHECK_INTERVAL_MS`: Background HTTP/2 health-sweep interval in milliseconds (default: `10_000`, min: `100`)
/// - `AZURE_COSMOS_CONNECTION_POOL_HTTP2_CONSECUTIVE_FAILURE_THRESHOLD`: Consecutive failure count before a shard becomes unhealthy (default: `5`, min: `1`, max: `255`)
/// - `AZURE_COSMOS_CONNECTION_POOL_HTTP2_EVICTION_GRACE_PERIOD_MS`: Minimum time since the last successful request before an unhealthy shard can be evicted (default: `2_000`, min: `100`)
/// - `AZURE_COSMOS_CONNECTION_POOL_HTTP2_KEEP_ALIVE_INTERVAL_MS`: HTTP/2 keep-alive ping interval in milliseconds (default: `1_000`, min: `100`)
/// - `AZURE_COSMOS_CONNECTION_POOL_HTTP2_KEEP_ALIVE_TIMEOUT_MS`: HTTP/2 keep-alive ping timeout in milliseconds (default: `2_000`, min: `100`)
/// - `AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_TIME_MS`: TCP keepalive time in milliseconds (default: `1_000`, min: `1_000` when set)
/// - `AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_INTERVAL_MS`: TCP keepalive probe interval in milliseconds (default: `1_000`, min: `1_000` when set)
/// - `AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_RETRIES`: TCP keepalive retry count (default: none, min: `1`, max: `255`)
/// - `AZURE_COSMOS_CONNECTION_POOL_IS_HTTP2_ALLOWED`: Whether HTTP/2 is allowed for gateway mode connections (default: `true`)
/// - `AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED`: Whether Gateway 2.0 feature is allowed (default: `false`)
/// - `AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED`: Whether server certificate validation is disabled for emulator; `true` maps to [`EmulatorServerCertValidation::DangerousDisabled`], `false` to [`EmulatorServerCertValidation::Enabled`] (default: `false`)
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
    max_http2_streams_per_client: Option<u32>,
    max_http2_connections_per_endpoint: Option<usize>,
    min_http2_connections_per_endpoint: Option<usize>,
    idle_http2_client_timeout: Option<Duration>,
    http2_health_check_interval: Option<Duration>,
    http2_consecutive_failure_threshold: Option<u32>,
    http2_eviction_grace_period: Option<Duration>,
    http2_keep_alive_interval: Option<Duration>,
    http2_keep_alive_timeout: Option<Duration>,
    tcp_keepalive_time: Option<Duration>,
    tcp_keepalive_interval: Option<Duration>,
    tcp_keepalive_retries: Option<u32>,
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

    /// Sets the maximum concurrent streams per HTTP/2 shard client.
    ///
    /// Must be between 1 and 20 inclusive.
    /// Default: 16.
    pub fn with_max_http2_streams_per_client(mut self, value: u32) -> Self {
        self.max_http2_streams_per_client = Some(value);
        self
    }

    /// Sets the maximum number of HTTP/2 shard clients per endpoint.
    ///
    /// Must be between 1 and 256 inclusive.
    pub fn with_max_http2_connections_per_endpoint(mut self, value: usize) -> Self {
        self.max_http2_connections_per_endpoint = Some(value);
        self
    }

    /// Sets the minimum number of HTTP/2 shard clients per endpoint.
    ///
    /// Must be between 1 and 256 inclusive and less than or equal to the maximum.
    pub fn with_min_http2_connections_per_endpoint(mut self, value: usize) -> Self {
        self.min_http2_connections_per_endpoint = Some(value);
        self
    }

    /// Sets the idle timeout for overflow HTTP/2 shard clients.
    ///
    /// Must be at least 1 second.
    /// Default: 60 seconds.
    pub fn with_idle_http2_client_timeout(mut self, timeout: Duration) -> Self {
        self.idle_http2_client_timeout = Some(timeout);
        self
    }

    /// Sets the background HTTP/2 health-sweep interval.
    ///
    /// Must be at least 100 milliseconds.
    /// Default: 10 seconds.
    pub fn with_http2_health_check_interval(mut self, timeout: Duration) -> Self {
        self.http2_health_check_interval = Some(timeout);
        self
    }

    /// Sets the consecutive failure threshold for unhealthy HTTP/2 shards.
    ///
    /// Must be between 1 and 255 inclusive.
    /// Default: 5.
    pub fn with_http2_consecutive_failure_threshold(mut self, value: u32) -> Self {
        self.http2_consecutive_failure_threshold = Some(value);
        self
    }

    /// Sets the grace period before an unhealthy HTTP/2 shard can be evicted.
    ///
    /// Must be at least 100 milliseconds.
    /// Default: 2 seconds.
    pub fn with_http2_eviction_grace_period(mut self, timeout: Duration) -> Self {
        self.http2_eviction_grace_period = Some(timeout);
        self
    }

    /// Sets the HTTP/2 keep-alive ping interval.
    ///
    /// Must be at least 100 milliseconds.
    /// Default: 1 second.
    pub fn with_http2_keep_alive_interval(mut self, timeout: Duration) -> Self {
        self.http2_keep_alive_interval = Some(timeout);
        self
    }

    /// Sets the HTTP/2 keep-alive ping timeout.
    ///
    /// Must be at least 100 milliseconds.
    /// Default: 2 seconds.
    pub fn with_http2_keep_alive_timeout(mut self, timeout: Duration) -> Self {
        self.http2_keep_alive_timeout = Some(timeout);
        self
    }

    /// Enables TCP keepalive with the given initial probe delay.
    pub fn with_tcp_keepalive_time(mut self, timeout: Duration) -> Self {
        self.tcp_keepalive_time = Some(timeout);
        self
    }

    /// Sets the TCP keepalive probe interval.
    pub fn with_tcp_keepalive_interval(mut self, timeout: Duration) -> Self {
        self.tcp_keepalive_interval = Some(timeout);
        self
    }

    /// Sets the TCP keepalive retry count.
    pub fn with_tcp_keepalive_retries(mut self, value: u32) -> Self {
        self.tcp_keepalive_retries = Some(value);
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
    /// Use [`EmulatorServerCertValidation::DangerousDisabled`] to skip TLS certificate
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

        let max_http2_streams_per_client = parse_from_env(
            self.max_http2_streams_per_client,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_HTTP2_STREAMS_PER_CLIENT",
            16_u32,
            ValidationBounds::range(1, 20),
        )?;

        // Default: available_parallelism * 2 (fallback 32).
        // NOTE: In containerized environments, `available_parallelism()` may
        // report the container's CPU quota or the host's CPU count depending
        // on the runtime. This is a known limitation of `std`; the env-var
        // override can be used to tune when the heuristic is wrong.
        let cpu_based_http2_max = std::thread::available_parallelism()
            .map(|count| count.get().saturating_mul(2))
            .unwrap_or(32)
            .clamp(1, 256);

        let max_http2_connections_per_endpoint = parse_from_env(
            self.max_http2_connections_per_endpoint,
            "AZURE_COSMOS_CONNECTION_POOL_MAX_HTTP2_CONNECTIONS_PER_ENDPOINT",
            cpu_based_http2_max,
            ValidationBounds::range(1, 256),
        )?;

        let min_http2_connections_per_endpoint = parse_from_env(
            self.min_http2_connections_per_endpoint,
            "AZURE_COSMOS_CONNECTION_POOL_MIN_HTTP2_CONNECTIONS_PER_ENDPOINT",
            1_usize,
            ValidationBounds::range(1, 256),
        )?;

        if min_http2_connections_per_endpoint > max_http2_connections_per_endpoint {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "min_http2_connections_per_endpoint must be less than or equal to max_http2_connections_per_endpoint, got {} > {}",
                    min_http2_connections_per_endpoint,
                    max_http2_connections_per_endpoint
                ),
            ));
        }

        let idle_http2_client_timeout = parse_duration_millis_from_env(
            self.idle_http2_client_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_IDLE_HTTP2_CLIENT_TIMEOUT_MS",
            60_000,
            1_000,
            u64::MAX,
        )?;

        let http2_health_check_interval = parse_duration_millis_from_env(
            self.http2_health_check_interval,
            "AZURE_COSMOS_CONNECTION_POOL_HTTP2_HEALTH_CHECK_INTERVAL_MS",
            10_000,
            100,
            u64::MAX,
        )?;

        let http2_consecutive_failure_threshold = parse_from_env(
            self.http2_consecutive_failure_threshold,
            "AZURE_COSMOS_CONNECTION_POOL_HTTP2_CONSECUTIVE_FAILURE_THRESHOLD",
            5_u32,
            ValidationBounds::range(1_u32, 255_u32),
        )?;

        let http2_eviction_grace_period = parse_duration_millis_from_env(
            self.http2_eviction_grace_period,
            "AZURE_COSMOS_CONNECTION_POOL_HTTP2_EVICTION_GRACE_PERIOD_MS",
            2_000,
            100,
            u64::MAX,
        )?;

        let http2_keep_alive_interval = parse_duration_millis_from_env(
            self.http2_keep_alive_interval,
            "AZURE_COSMOS_CONNECTION_POOL_HTTP2_KEEP_ALIVE_INTERVAL_MS",
            1_000,
            100,
            u64::MAX,
        )?;

        let http2_keep_alive_timeout = parse_duration_millis_from_env(
            self.http2_keep_alive_timeout,
            "AZURE_COSMOS_CONNECTION_POOL_HTTP2_KEEP_ALIVE_TIMEOUT_MS",
            2_000,
            100,
            u64::MAX,
        )?;

        let tcp_keepalive_time = parse_optional_duration_millis_from_env(
            self.tcp_keepalive_time,
            "AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_TIME_MS",
            1_000,
            u64::MAX,
        )?
        // Default to 1 s to match the HTTP/2 keep-alive interval.
        // This ensures idle HTTP/1.1 connections are detected as stale
        // before NAT/firewall timeouts silently close them.
        .or(Some(Duration::from_secs(1)));

        let tcp_keepalive_interval = parse_optional_duration_millis_from_env(
            self.tcp_keepalive_interval,
            "AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_INTERVAL_MS",
            1_000,
            u64::MAX,
        )?
        // Default to 1 s, matching the HTTP/2 keep-alive probe cadence.
        .or(Some(Duration::from_secs(1)));

        let tcp_keepalive_retries = parse_optional_from_env(
            self.tcp_keepalive_retries,
            "AZURE_COSMOS_CONNECTION_POOL_TCP_KEEPALIVE_RETRIES",
            ValidationBounds::range(1_u32, 255_u32),
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
            max_http2_streams_per_client,
            max_http2_connections_per_endpoint,
            min_http2_connections_per_endpoint,
            idle_http2_client_timeout,
            http2_health_check_interval,
            http2_consecutive_failure_threshold,
            http2_eviction_grace_period,
            http2_keep_alive_interval,
            http2_keep_alive_timeout,
            tcp_keepalive_time,
            tcp_keepalive_interval,
            tcp_keepalive_retries,
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
            EmulatorServerCertValidation::Enabled
        );
        assert_eq!(options.idle_connection_timeout(), None);
        assert_eq!(options.max_http2_streams_per_client(), 16);
        assert!(options.max_http2_connections_per_endpoint() >= 1);
        assert_eq!(options.min_http2_connections_per_endpoint(), 1);
        assert_eq!(options.idle_http2_client_timeout(), Duration::from_secs(60));
        assert_eq!(
            options.http2_health_check_interval(),
            Duration::from_secs(10)
        );
        assert_eq!(options.http2_consecutive_failure_threshold(), 5);
        assert_eq!(
            options.http2_eviction_grace_period(),
            Duration::from_secs(2)
        );
        assert_eq!(options.http2_keep_alive_interval(), Duration::from_secs(1));
        assert_eq!(options.http2_keep_alive_timeout(), Duration::from_secs(2));
        assert_eq!(options.tcp_keepalive_time(), Some(Duration::from_secs(1)));
        assert_eq!(
            options.tcp_keepalive_interval(),
            Some(Duration::from_secs(1))
        );
        assert_eq!(options.tcp_keepalive_retries(), None);
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
            .with_max_http2_streams_per_client(12)
            .with_max_http2_connections_per_endpoint(24)
            .with_min_http2_connections_per_endpoint(3)
            .with_idle_http2_client_timeout(Duration::from_millis(90_000))
            .with_http2_health_check_interval(Duration::from_millis(15_000))
            .with_http2_consecutive_failure_threshold(8)
            .with_http2_eviction_grace_period(Duration::from_millis(4_000))
            .with_http2_keep_alive_interval(Duration::from_millis(1_500))
            .with_http2_keep_alive_timeout(Duration::from_millis(2_500))
            .with_tcp_keepalive_time(Duration::from_millis(30_000))
            .with_tcp_keepalive_interval(Duration::from_millis(5_000))
            .with_tcp_keepalive_retries(4)
            .with_is_http2_allowed(false)
            .with_is_gateway20_allowed(true)
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DangerousDisabled)
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
        assert_eq!(options.max_http2_streams_per_client(), 12);
        assert_eq!(options.max_http2_connections_per_endpoint(), 24);
        assert_eq!(options.min_http2_connections_per_endpoint(), 3);
        assert_eq!(
            options.idle_http2_client_timeout(),
            Duration::from_millis(90_000)
        );
        assert_eq!(
            options.http2_health_check_interval(),
            Duration::from_millis(15_000)
        );
        assert_eq!(options.http2_consecutive_failure_threshold(), 8);
        assert_eq!(
            options.http2_eviction_grace_period(),
            Duration::from_millis(4_000)
        );
        assert_eq!(
            options.http2_keep_alive_interval(),
            Duration::from_millis(1_500)
        );
        assert_eq!(
            options.http2_keep_alive_timeout(),
            Duration::from_millis(2_500)
        );
        assert_eq!(
            options.tcp_keepalive_time(),
            Some(Duration::from_millis(30_000))
        );
        assert_eq!(
            options.tcp_keepalive_interval(),
            Some(Duration::from_millis(5_000))
        );
        assert_eq!(options.tcp_keepalive_retries(), Some(4));
        assert!(!options.is_http2_allowed());
        // gateway20 is set to true but HTTP/2 is false, so it should be false
        assert!(!options.is_gateway20_allowed());
        assert_eq!(
            options.emulator_server_cert_validation(),
            EmulatorServerCertValidation::DangerousDisabled
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
    fn max_http2_streams_per_client_too_large() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_max_http2_streams_per_client(21)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_http2_streams_per_client must be at most 20"));
    }

    #[test]
    fn min_http2_connections_cannot_exceed_max() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_min_http2_connections_per_endpoint(4)
            .with_max_http2_connections_per_endpoint(3)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("min_http2_connections_per_endpoint must be less than or equal to max_http2_connections_per_endpoint"));
    }

    #[test]
    fn idle_http2_client_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_idle_http2_client_timeout(Duration::from_millis(500))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("idle_http2_client_timeout_ms must be at least 1000ms"));
    }

    #[test]
    fn http2_health_check_interval_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_http2_health_check_interval(Duration::from_millis(50))
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("http2_health_check_interval_ms must be at least 100ms"));
    }

    #[test]
    fn http2_consecutive_failure_threshold_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_http2_consecutive_failure_threshold(0)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("http2_consecutive_failure_threshold must be at least 1"));
    }

    #[test]
    fn tcp_keepalive_retries_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .with_tcp_keepalive_retries(0)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("tcp_keepalive_retries must be at least 1"));
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
