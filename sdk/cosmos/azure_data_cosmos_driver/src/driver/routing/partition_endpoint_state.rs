// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition-level endpoint routing state for PPAF and PPCB.

use std::{
    collections::{HashMap, HashSet},
    env,
    time::{Duration, Instant},
};

use super::CosmosEndpoint;

/// Immutable partition-level endpoint routing state.
///
/// Managed via CAS in `LocationStateStore` alongside `AccountEndpointState`.
/// Mutations create a new instance and swap it atomically via `crossbeam_epoch`.
#[derive(Clone, Debug)]
pub(crate) struct PartitionEndpointState {
    /// PPAF map: writes on single-master accounts.
    /// Key: partition key range ID.
    pub failover_overrides: HashMap<String, PartitionFailoverEntry>,

    /// PPCB map: reads (any account) + writes on multi-master.
    /// Key: partition key range ID.
    pub circuit_breaker_overrides: HashMap<String, PartitionFailoverEntry>,

    /// PPAF enabled (from `AccountProperties.enable_per_partition_failover_behavior`).
    pub per_partition_automatic_failover_enabled: bool,

    /// PPCB enabled (from env var + account property).
    pub per_partition_circuit_breaker_enabled: bool,

    /// Retained option value for recomputation on account refresh.
    pub circuit_breaker_option_enabled: bool,

    /// Configuration read from env vars at construction time.
    pub config: PartitionFailoverConfig,
}

impl Default for PartitionEndpointState {
    fn default() -> Self {
        let config = PartitionFailoverConfig::from_env();
        let circuit_breaker_option_enabled = config.circuit_breaker_option_enabled;
        Self {
            failover_overrides: HashMap::new(),
            circuit_breaker_overrides: HashMap::new(),
            per_partition_automatic_failover_enabled: false,
            per_partition_circuit_breaker_enabled: circuit_breaker_option_enabled,
            circuit_breaker_option_enabled,
            config,
        }
    }
}

/// Health status of a partition failover entry.
///
/// Tracks the lifecycle of a failed-over partition through recovery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum HealthStatus {
    /// Partition is failed-over to an alternate region.
    /// All requests route to the override endpoint.
    Unhealthy,
    /// Unavailability window has elapsed. The next single request for this
    /// partition is tentatively routed back to the original region as a probe.
    ProbeCandidate,
}

/// Per-partition failover entry.
///
/// Immutable value — mutations produce a new instance via CAS.
#[derive(Clone, Debug)]
pub(crate) struct PartitionFailoverEntry {
    /// Current endpoint this partition is routed to.
    pub current_endpoint: CosmosEndpoint,
    /// Original endpoint that first failed (used for failback probing).
    pub first_failed_endpoint: CosmosEndpoint,
    /// Set of endpoints already tried.
    pub failed_endpoints: HashSet<CosmosEndpoint>,

    /// Read failure count (not necessarily consecutive — see §13.2).
    pub read_failure_count: i32,
    /// Write failure count (not necessarily consecutive — see §13.2).
    pub write_failure_count: i32,

    /// When the first failure occurred (for failback eligibility).
    pub first_failure_time: Instant,
    /// When the most recent failure occurred (for counter reset).
    pub last_failure_time: Instant,

    /// Health status for gradual failback (probe-based recovery).
    pub health_status: HealthStatus,
}

/// Configuration for partition-level failover, read once at construction.
#[derive(Clone, Debug)]
pub(crate) struct PartitionFailoverConfig {
    /// Read failures before circuit trips (default: 2).
    pub read_failure_threshold: i32,

    /// Write failures before circuit trips (default: 5).
    pub write_failure_threshold: i32,

    /// Window after which failure counters reset (default: 5 minutes).
    pub counter_reset_window: Duration,

    /// Duration a partition must remain unavailable before failback (default: 5s).
    pub partition_unavailability_duration: Duration,

    /// Interval for the background failback sweep (default: 300s).
    pub failback_sweep_interval: Duration,

    /// Whether PPCB is enabled via options (default: true).
    pub circuit_breaker_option_enabled: bool,
}

impl Default for PartitionFailoverConfig {
    fn default() -> Self {
        Self {
            read_failure_threshold: 2,
            write_failure_threshold: 5,
            counter_reset_window: Duration::from_secs(5 * 60),
            partition_unavailability_duration: Duration::from_secs(5),
            failback_sweep_interval: Duration::from_secs(300),
            circuit_breaker_option_enabled: true,
        }
    }
}

impl PartitionFailoverConfig {
    /// Reads configuration from environment variables, falling back to defaults.
    pub fn from_env() -> Self {
        let defaults = Self::default();

        let read_failure_threshold =
            env::var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(defaults.read_failure_threshold);

        let write_failure_threshold =
            env::var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(defaults.write_failure_threshold);

        let counter_reset_window_minutes =
            env::var("AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(5);

        let partition_unavailability_secs =
            env::var("AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(5);

        let failback_sweep_secs = env::var(
            "AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS",
        )
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(300);

        let circuit_breaker_option_enabled =
            env::var("AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true);

        Self {
            read_failure_threshold,
            write_failure_threshold,
            counter_reset_window: Duration::from_secs(counter_reset_window_minutes * 60),
            partition_unavailability_duration: Duration::from_secs(partition_unavailability_secs),
            failback_sweep_interval: Duration::from_secs(failback_sweep_secs),
            circuit_breaker_option_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let config = PartitionFailoverConfig::default();
        assert_eq!(config.read_failure_threshold, 2);
        assert_eq!(config.write_failure_threshold, 5);
        assert_eq!(config.counter_reset_window, Duration::from_secs(300));
        assert_eq!(
            config.partition_unavailability_duration,
            Duration::from_secs(5)
        );
        assert_eq!(config.failback_sweep_interval, Duration::from_secs(300));
        assert!(config.circuit_breaker_option_enabled);
    }

    #[test]
    fn default_partition_state() {
        let state = PartitionEndpointState::default();
        assert!(state.failover_overrides.is_empty());
        assert!(state.circuit_breaker_overrides.is_empty());
        assert!(!state.per_partition_automatic_failover_enabled);
        assert!(state.per_partition_circuit_breaker_enabled);
        assert!(state.circuit_breaker_option_enabled);
    }
}
