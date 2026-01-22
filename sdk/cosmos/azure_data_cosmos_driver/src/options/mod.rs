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
/// Controls how the driver manages HTTP/2 and TCP connections to Cosmos DB endpoints.
#[derive(Clone, Debug)]
pub struct ConnectionPoolOptions {
    /// Minimum number of connections to maintain in the pool.
    pub min_connections: usize,

    /// Maximum number of connections allowed in the pool.
    pub max_connections: usize,

    /// Connection idle timeout before closing unused connections.
    pub idle_timeout: Duration,

    /// Whether to enable HTTP/2 for gateway mode connections.
    pub enable_http2: bool,
}

impl Default for ConnectionPoolOptions {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 50,
            idle_timeout: Duration::from_secs(300),
            enable_http2: true,
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
