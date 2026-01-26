//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a three-level hierarchy: Environment → Driver → Operation.

use azure_core::http::ClientOptions;
use std::time::Duration;

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

    /// Default consistency level for read operations (can be overridden per-operation).
    pub default_consistency: Option<ConsistencyLevel>,
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            connection_pool: ConnectionPoolOptions::default(),
            default_timeout: Duration::from_secs(60),
            default_consistency: None,
        }
    }
}

/// Configuration for connection pooling behavior.
///
/// Controls how the driver manages connections to Cosmos DB endpoints.
#[derive(Clone, Debug)]
pub struct ConnectionPoolOptions {

    /// Whether to allow using HTTP/2 for gateway mode connections.
    pub is_http2_allowed: bool,

    /// Whether to allow the Gateway 2.0 feature for gateway mode connections.
    pub is_gateway20_allowed: bool,
}

impl Default for ConnectionPoolOptions {
    fn default() -> Self {
        Self {
            is_http2_allowed: true,
            is_gateway20_allowed: false,
        }
    }
}

/// Consistency level for Cosmos DB operations.
///
/// See [Cosmos DB consistency levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
/// for detailed semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsistencyLevel {
    /// Guarantees linearizability - reads return the most recent committed version.
    Strong,
    /// Bounded staleness with configurable lag.
    BoundedStaleness,
    /// Session consistency within a client session.
    Session,
    /// Monotonic read consistency with eventual convergence.
    ConsistentPrefix,
    /// No ordering guarantees, lowest latency.
    Eventual,
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
            azure_core::Error::message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Unknown read consistency strategy: {}", s),
            )
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
}
