//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a three-level hierarchy: Environment → Driver → Operation.

use azure_core::http::ClientOptions;
use std::{net::IpAddr, time::Duration};

/// Configuration options for a Cosmos DB driver instance.
///
/// These options control driver-wide behavior including connection pooling,
/// default consistency levels, and HTTP pipeline configuration.
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// Core HTTP client options from azure_core.
    pub client_options: ClientOptions,

    /// Connection pool configuration for managing TCP connections.
    pub connection_pool: ConnectionPoolOptions,

    /// Default request timeout for operations (can be overridden per-operation).
    pub default_timeout: Duration,

    /// Default read consistency strategy for read operations (can be overridden per-operation).
    pub default_read_consistency_strategy: Option<ReadConsistencyStrategy>,
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            connection_pool: ConnectionPoolOptions::default(),
            default_timeout: Duration::from_secs(60),
            default_read_consistency_strategy: None,
        }
    }
}

/// Configuration for connection pooling behavior.
///
/// Controls how the driver manages connections to Cosmos DB endpoints.
///
/// Use [`ConnectionPoolOptionsBuilder`] to construct instances with custom values.
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

    /// Whether to allow using HTTP/2 for gateway mode connections.
    is_http2_allowed: bool,

    /// Whether to allow the Gateway 2.0 feature for gateway mode connections.
    /// If true the driver will use Gateway 2.0 features when communicating with the Cosmos DB service if
    /// the Gateway 2.0 feature is enabled for the account.
    is_gateway20_allowed: bool,

    emulator_server_cert_validation_disabled: bool,

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
}

/// Read consistency strategies supported by Azure Cosmos DB.
///
/// The requested read consistency strategy can be chosen independent of the consistency level
/// provisioned for the database account.
///
/// The `ReadConsistencyStrategy` setting will override whatever `ConsistencyLevel` is chosen
/// in request options, client options, or the default consistency level for an account unless
/// `ReadConsistencyStrategy::Default` is used.
///
/// **NOTE**: `ReadConsistencyStrategy` is currently only supported when using direct mode.
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for detailed semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReadConsistencyStrategy {
    /// Use the default read behavior for the consistency level applied to the operation,
    /// the client, or the account.
    Default,

    /// Eventual consistency guarantees that reads will return a subset of writes.
    /// All writes will eventually be available for reads.
    Eventual,

    /// Session consistency guarantees monotonic reads (you never read old data, then new,
    /// then old again), monotonic writes (writes are ordered), and read your writes
    /// (your writes are immediately visible to your reads) within any single session.
    Session,

    /// Reads the latest version across all regions.
    ///
    /// Since replication with global strong consistency is synchronous, this read
    /// consistency strategy ensures that the latest successfully written version
    /// across regions is returned.
    ///
    /// **NOTE**: Only supported for single-master accounts with Strong consistency
    /// enabled as default consistency.
    GlobalStrong,
}

impl ReadConsistencyStrategy {
    /// Parses a read consistency strategy from its wire format representation.
    ///
    /// Parsing is case-sensitive for exact matches, with case-insensitive fallback.
    ///
    /// Returns `None` if the string does not match any known strategy.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Default" => Some(Self::Default),
            "Eventual" => Some(Self::Eventual),
            "Session" => Some(Self::Session),
            "GlobalStrong" => Some(Self::GlobalStrong),
            _ => {
                // Case-insensitive fallback
                if s.eq_ignore_ascii_case("Default") {
                    Some(Self::Default)
                } else if s.eq_ignore_ascii_case("Eventual") {
                    Some(Self::Eventual)
                } else if s.eq_ignore_ascii_case("Session") {
                    Some(Self::Session)
                } else if s.eq_ignore_ascii_case("GlobalStrong") {
                    Some(Self::GlobalStrong)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the wire format representation of this read consistency strategy.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Eventual => "Eventual",
            Self::Session => "Session",
            Self::GlobalStrong => "GlobalStrong",
        }
    }
}

impl std::fmt::Display for ReadConsistencyStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ReadConsistencyStrategy {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Unknown read consistency strategy: {}", s),
            )
        })
    }
}

/// Helper function to parse a duration from an environment variable value in milliseconds.
///
/// Expected format: integer number of milliseconds.
fn parse_duration_from_env_millis(value: &str) -> Option<Duration> {
    value.parse::<u64>().ok().map(Duration::from_millis)
}

/// Helper function to parse a duration from an environment variable value in seconds.
///
/// Expected format: integer number of seconds.
fn parse_duration_from_env_secs(value: &str) -> Option<Duration> {
    value.parse::<u64>().ok().map(Duration::from_secs)
}

/// Parses a value from an environment variable with proper error handling.
///
/// Returns the value from the builder if present, otherwise attempts to parse from the environment variable.
/// Falls back to the default value if the environment variable is not set.
fn parse_from_env<T>(
    builder_value: Option<T>,
    env_var_name: &str,
    default: T,
) -> azure_core::Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match builder_value {
        Some(value) => Ok(value),
        None => match std::env::var(env_var_name) {
            Ok(v) => v.parse().map_err(|e| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!(
                        "Failed to parse {} as {}: {} ({})",
                        env_var_name,
                        std::any::type_name::<T>(),
                        v,
                        e
                    ),
                )
            }),
            Err(_) => Ok(default),
        },
    }
}

/// Parses a duration from an environment variable (in milliseconds) with validation.
///
/// Returns an error if the parsed value is less than the minimum duration.
fn parse_duration_millis_from_env(
    builder_value: Option<Duration>,
    env_var_name: &str,
    default_millis: u64,
    min_millis: u64,
    max_millis: u64,
) -> azure_core::Result<Duration> {
    let duration = match builder_value {
        Some(timeout) => timeout,
        None => match std::env::var(env_var_name) {
            Ok(v) => parse_duration_from_env_millis(&v).ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("Failed to parse {} as milliseconds: {}", env_var_name, v),
                )
            })?,
            Err(_) => Duration::from_millis(default_millis),
        },
    };

    if duration < Duration::from_millis(min_millis) {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "{} must be at least {}ms, got {:?}",
                env_var_name
                    .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                    .unwrap_or(env_var_name)
                    .to_lowercase(),
                min_millis,
                duration
            ),
        ));
    }

    if duration > Duration::from_millis(max_millis) {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "{} must be at most {}ms, got {:?}",
                env_var_name
                    .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                    .unwrap_or(env_var_name)
                    .to_lowercase(),
                max_millis,
                duration
            ),
        ));
    }

    Ok(duration)
}

/// Parses an optional duration from an environment variable (in milliseconds) with validation.
fn parse_optional_duration_millis_from_env(
    builder_value: Option<Duration>,
    env_var_name: &str,
    min_millis: u64,
    max_millis: u64,
) -> azure_core::Result<Option<Duration>> {
    match builder_value {
        Some(timeout) => {
            if timeout < Duration::from_millis(min_millis) {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!(
                        "{} must be at least {}ms, got {:?}",
                        env_var_name
                            .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                            .unwrap_or(env_var_name)
                            .to_lowercase(),
                        min_millis,
                        timeout
                    ),
                ));
            }
            Ok(Some(timeout))
        }
        None => match std::env::var(env_var_name) {
            Ok(v) => {
                let timeout = parse_duration_from_env_millis(&v).ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!("Failed to parse {} as milliseconds: {}", env_var_name, v),
                    )
                })?;
                if timeout < Duration::from_millis(min_millis) {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!(
                            "{} must be at least {}ms, got {:?}",
                            env_var_name
                                .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                                .unwrap_or(env_var_name)
                                .to_lowercase(),
                            min_millis,
                            timeout
                        ),
                    ));
                }
                if timeout > Duration::from_millis(max_millis) {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!(
                            "{} must be at most {}ms, got {:?}",
                            env_var_name
                                .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                                .unwrap_or(env_var_name)
                                .to_lowercase(),
                            max_millis,
                            timeout
                        ),
                    ));
                }
                Ok(Some(timeout))
            }
            Err(_) => Ok(None),
        },
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
/// - `AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED`: Whether server certificate validation is disabled for emulator (default: `false`)
/// - `AZURE_COSMOS_LOCAL_ADDRESS`: Local IP address to bind to (default: none)
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::ConnectionPoolOptions;
///
/// let options = ConnectionPoolOptions::builder()
///     .max_idle_connections_per_endpoint(5_000)
///     .build()
///     .expect("valid options");
/// ```
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
    emulator_server_cert_validation_disabled: Option<bool>,
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
    pub fn dangerous_is_proxy_allowed(mut self, value: bool) -> Self {
        self.is_proxy_allowed = Some(value);
        self
    }

    /// Sets the minimum connection timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 100ms.
    pub fn min_connect_timeout(mut self, timeout: Duration) -> Self {
        self.min_connect_timeout = Some(timeout);
        self
    }

    /// Sets the maximum connection timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 5_000ms (5 seconds).
    pub fn max_connect_timeout(mut self, timeout: Duration) -> Self {
        self.max_connect_timeout = Some(timeout);
        self
    }

    /// Sets the minimum data plane request timeout.
    ///
    /// Must be between 100ms and 65_000ms (65 seconds).
    /// Default: 100ms.
    pub fn min_dataplane_request_timeout(mut self, timeout: Duration) -> Self {
        self.min_dataplane_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum data plane request timeout.
    ///
    /// Must be at least 100ms.
    /// Default: 6 seconds.
    pub fn max_dataplane_request_timeout(mut self, timeout: Duration) -> Self {
        self.max_dataplane_request_timeout = Some(timeout);
        self
    }

    /// Sets the minimum metadata request timeout.
    ///
    /// Must be between 100ms and 6_000ms (6 seconds).
    /// Default: 100ms.
    pub fn min_metadata_request_timeout(mut self, timeout: Duration) -> Self {
        self.min_metadata_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum metadata request timeout.
    ///
    /// Must be between 100ms and 65_000ms (65 seconds).
    /// Default: 65_000ms (65 seconds).
    pub fn max_metadata_request_timeout(mut self, timeout: Duration) -> Self {
        self.max_metadata_request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum number of idle connections per endpoint.
    ///
    /// Must be between 10 and 64_000.
    /// Default: 1_000 if HTTP/2 is allowed, 10_000 otherwise.
    pub fn max_idle_connections_per_endpoint(mut self, count: usize) -> Self {
        self.max_idle_connections_per_endpoint = Some(count);
        self
    }

    /// Sets the idle connection timeout.
    ///
    /// Must be at least 300_000ms (5 minutes) when set.
    /// Default: none (connections are never closed due to idleness).
    pub fn idle_connection_timeout(mut self, timeout: Duration) -> Self {
        self.idle_connection_timeout = Some(timeout);
        self
    }

    /// Sets whether HTTP/2 is allowed for gateway mode connections.
    pub fn is_http2_allowed(mut self, value: bool) -> Self {
        self.is_http2_allowed = Some(value);
        self
    }

    /// Sets whether Gateway 2.0 feature is allowed.
    pub fn is_gateway20_allowed(mut self, value: bool) -> Self {
        self.is_gateway20_allowed = Some(value);
        self
    }

    /// Sets whether server certificate validation is disabled when the service endpoint is targeting local emulator installations.
    pub fn dangerous_emulator_server_cert_validation_disabled(mut self, value: bool) -> Self {
        self.emulator_server_cert_validation_disabled = Some(value);
        self
    }

    /// Sets the local IP address to bind to.
    pub fn local_address(mut self, addr: IpAddr) -> Self {
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

        let max_idle_connections_per_endpoint = match self.max_idle_connections_per_endpoint {
            Some(count) => count,
            None => match std::env::var("AZURE_COSMOS_CONNECTION_POOL_MAX_IDLE_CONNECTIONS_PER_ENDPOINT") {
                Ok(v) => v.parse().map_err(|e| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!("Failed to parse AZURE_COSMOS_CONNECTION_POOL_MAX_IDLE_CONNECTIONS_PER_ENDPOINT as integer: {} ({})", v, e),
                    )
                })?,
                Err(_) => max_connection_pool_size_default,
            },
        };

        if max_idle_connections_per_endpoint < 10 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "max_idle_connections_per_endpoint must be at least 10, got {}",
                    max_idle_connections_per_endpoint
                ),
            ));
        }

        if max_idle_connections_per_endpoint > 64_000 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "max_idle_connections_per_endpoint must be at most 64_000, got {}",
                    max_idle_connections_per_endpoint
                ),
            ));
        }

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
            emulator_server_cert_validation_disabled: parse_from_env(
                self.emulator_server_cert_validation_disabled,
                "AZURE_COSMOS_EMULATOR_SERVER_CERT_VALIDATION_DISABLED",
                false,
            )?,
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
    fn parse_all_strategies() {
        assert_eq!(
            ReadConsistencyStrategy::from_str("Default"),
            Some(ReadConsistencyStrategy::Default)
        );
        assert_eq!(
            ReadConsistencyStrategy::from_str("Eventual"),
            Some(ReadConsistencyStrategy::Eventual)
        );
        assert_eq!(
            ReadConsistencyStrategy::from_str("Session"),
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(
            ReadConsistencyStrategy::from_str("GlobalStrong"),
            Some(ReadConsistencyStrategy::GlobalStrong)
        );
    }

    #[test]
    fn parse_unknown_returns_none() {
        assert_eq!(ReadConsistencyStrategy::from_str("Unknown"), None);
    }

    #[test]
    fn parse_case_insensitive_fallback() {
        // Case-insensitive fallback works
        assert_eq!(
            ReadConsistencyStrategy::from_str("eventual"),
            Some(ReadConsistencyStrategy::Eventual)
        );
    }

    #[test]
    fn to_string_roundtrip() {
        for strategy in &[
            ReadConsistencyStrategy::Default,
            ReadConsistencyStrategy::Eventual,
            ReadConsistencyStrategy::Session,
            ReadConsistencyStrategy::GlobalStrong,
        ] {
            let s = strategy.to_string();
            assert_eq!(ReadConsistencyStrategy::from_str(&s), Some(*strategy));
        }
    }

    #[test]
    fn connection_pool_options_builder_defaults() {
        let options = ConnectionPoolOptionsBuilder::new().build().unwrap();

        assert_eq!(options.is_proxy_allowed, false);
        assert_eq!(options.min_connect_timeout, Duration::from_millis(100));
        assert_eq!(options.max_connect_timeout, Duration::from_millis(5_000));
        assert_eq!(
            options.min_dataplane_request_timeout,
            Duration::from_millis(100)
        );
        assert_eq!(
            options.max_dataplane_request_timeout,
            Duration::from_millis(6_000)
        );
        assert_eq!(
            options.min_metadata_request_timeout,
            Duration::from_millis(100)
        );
        assert_eq!(
            options.max_metadata_request_timeout,
            Duration::from_millis(65_000)
        );
        assert_eq!(options.is_http2_allowed, true);
        assert_eq!(options.is_gateway20_allowed, false);
        assert_eq!(options.emulator_server_cert_validation_disabled, false);
        assert_eq!(options.idle_connection_timeout, None);
        assert_eq!(options.local_address, None);
        // Default is 1_000 when HTTP/2 is allowed (which is true by default)
        assert_eq!(options.max_idle_connections_per_endpoint, 1_000);
    }

    #[test]
    fn connection_pool_options_builder_custom_values() {
        let options = ConnectionPoolOptionsBuilder::new()
            .dangerous_is_proxy_allowed(true)
            .min_connect_timeout(Duration::from_millis(200))
            .max_connect_timeout(Duration::from_millis(3_000))
            .min_dataplane_request_timeout(Duration::from_millis(500))
            .max_dataplane_request_timeout(Duration::from_millis(10_000))
            .min_metadata_request_timeout(Duration::from_millis(150))
            .max_metadata_request_timeout(Duration::from_millis(30_000))
            .max_idle_connections_per_endpoint(5_000)
            .idle_connection_timeout(Duration::from_millis(600_000))
            .is_http2_allowed(false)
            .is_gateway20_allowed(true)
            .dangerous_emulator_server_cert_validation_disabled(true)
            .build()
            .unwrap();

        assert_eq!(options.is_proxy_allowed, true);
        assert_eq!(options.min_connect_timeout, Duration::from_millis(200));
        assert_eq!(options.max_connect_timeout, Duration::from_millis(3_000));
        assert_eq!(
            options.min_dataplane_request_timeout,
            Duration::from_millis(500)
        );
        assert_eq!(
            options.max_dataplane_request_timeout,
            Duration::from_millis(10_000)
        );
        assert_eq!(
            options.min_metadata_request_timeout,
            Duration::from_millis(150)
        );
        assert_eq!(
            options.max_metadata_request_timeout,
            Duration::from_millis(30_000)
        );
        assert_eq!(options.max_idle_connections_per_endpoint, 5_000);
        assert_eq!(
            options.idle_connection_timeout,
            Some(Duration::from_millis(600_000))
        );
        assert_eq!(options.is_http2_allowed, false);
        // gateway20 is set to true but HTTP/2 is false, so it should be false
        assert_eq!(options.is_gateway20_allowed, false);
        assert_eq!(options.emulator_server_cert_validation_disabled, true);
    }

    #[test]
    fn min_connect_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .min_connect_timeout(Duration::from_millis(50))
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
            .min_connect_timeout(Duration::from_millis(7_000))
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
            .max_connect_timeout(Duration::from_millis(50))
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
            .max_connect_timeout(Duration::from_millis(7_000))
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
            .min_dataplane_request_timeout(Duration::from_millis(50))
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
            .min_dataplane_request_timeout(Duration::from_millis(70_000))
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
            .max_dataplane_request_timeout(Duration::from_millis(50))
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
            .min_metadata_request_timeout(Duration::from_millis(50))
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
            .min_metadata_request_timeout(Duration::from_millis(7_000))
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
            .max_metadata_request_timeout(Duration::from_millis(50))
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
            .max_metadata_request_timeout(Duration::from_millis(70_000))
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
            .max_idle_connections_per_endpoint(5)
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
            .max_idle_connections_per_endpoint(65_000)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("max_idle_connections_per_endpoint must be at most 64_000"));
    }

    #[test]
    fn idle_connection_timeout_too_small() {
        let result = ConnectionPoolOptionsBuilder::new()
            .idle_connection_timeout(Duration::from_millis(100_000))
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
            .is_http2_allowed(false)
            .is_gateway20_allowed(true)
            .build()
            .unwrap();

        // Gateway 2.0 should be disabled if HTTP/2 is not allowed
        assert_eq!(options.is_gateway20_allowed, false);
    }

    #[test]
    fn http2_disabled_changes_max_connection_pool_default() {
        let options = ConnectionPoolOptionsBuilder::new()
            .is_http2_allowed(false)
            .build()
            .unwrap();

        // When HTTP/2 is disabled, default should be 10_000
        assert_eq!(options.max_idle_connections_per_endpoint, 10_000);
    }
}
