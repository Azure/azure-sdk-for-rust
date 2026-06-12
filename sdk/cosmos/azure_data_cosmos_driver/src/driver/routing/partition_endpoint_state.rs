// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition-level endpoint routing state for PPAF and PPCB.

use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use crate::options::{OperationOptionsView, Region};

use super::{partition_key_range_id::PartitionKeyRangeId, CosmosEndpoint};

/// Immutable partition-level endpoint routing state.
///
/// Managed via CAS in `LocationStateStore` alongside `AccountEndpointState`.
/// Mutations create a new instance and swap it atomically via `crossbeam_epoch`.
#[derive(Clone, Debug)]
pub(crate) struct PartitionEndpointState {
    /// PPAF map: writes on single-master accounts.
    /// Key: partition key range ID.
    pub failover_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,

    /// PPCB map: reads (any account) + writes on multi-master.
    /// Key: partition key range ID.
    pub circuit_breaker_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,

    /// PPAF enabled (from `AccountProperties.enable_per_partition_failover_behavior`).
    pub per_partition_automatic_failover_enabled: bool,

    /// PPCB enabled (from env var + account property).
    pub per_partition_circuit_breaker_enabled: bool,

    /// Per-`(partition, primary_region)` count of consecutive alternate-region
    /// hedge wins: incremented when the alternate
    /// hedge attempt finishes before the primary, reset on a direct primary-region
    /// win. When the count reaches [`PartitionFailoverConfig::consecutive_hedge_win_threshold`]
    /// the partition is tripped by installing an [`HealthStatus::Unhealthy`]
    /// entry in [`Self::circuit_breaker_overrides`] (same shape PPCB uses for hard
    /// failures), so subsequent reads route away from the degraded primary region.
    /// The trip is recovered by the existing PPCB failback sweep — primary wins
    /// only reset the counter, never the trip itself.
    ///
    /// `Option<Region>` accommodates default-endpoint accounts whose snapshots
    /// do not carry a named region (the counter key is
    /// `(partition, primary_region)` with `primary_region` allowed to be
    /// absent).
    pub consecutive_hedge_wins: HashMap<(PartitionKeyRangeId, Option<Region>), u32>,

    /// Configuration read from env vars at construction time.
    pub config: PartitionFailoverConfig,

    /// Test-only liveness canary. Set by tests that want to observe whether a
    /// particular `PartitionEndpointState` instance is dropped (e.g., the
    /// `apply_partition` use-after-free regression test). Production code never
    /// reads or writes this field; it is `None` in all non-test constructions.
    #[cfg(test)]
    pub(crate) _test_canary: Option<std::sync::Arc<()>>,
}

impl PartitionEndpointState {
    /// Creates a new `PartitionEndpointState` from the given partition failover config.
    pub fn new(config: PartitionFailoverConfig) -> Self {
        Self {
            per_partition_circuit_breaker_enabled: config.circuit_breaker_option_enabled,
            failover_overrides: HashMap::new(),
            circuit_breaker_overrides: HashMap::new(),
            consecutive_hedge_wins: HashMap::new(),
            per_partition_automatic_failover_enabled: false,
            config,
            #[cfg(test)]
            _test_canary: None,
        }
    }
}

impl Default for PartitionEndpointState {
    fn default() -> Self {
        Self::new(PartitionFailoverConfig::default())
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

    /// Per-entry random delay added to `partition_unavailability_duration` before
    /// this entry becomes a `ProbeCandidate`. Spreads simultaneously-failed
    /// partitions across the failback window so they don't all stampede the
    /// recovering region on the same sweep tick (thundering-herd mitigation).
    /// Sampled once when the entry is created (and re-sampled on probe failure)
    /// from `[0, partition_unavailability_duration / 2]`. PPAF entries always
    /// use `Duration::ZERO` since they don't participate in background failback.
    pub failback_jitter: Duration,
}

/// Configuration for partition-level failover, read once at construction.
#[derive(Clone, Debug)]
pub(crate) struct PartitionFailoverConfig {
    /// Read failures before circuit trips (default: 10).
    pub read_failure_threshold: i32,

    /// Write failures before circuit trips (default: 5).
    pub write_failure_threshold: i32,

    /// Window after which failure counters reset (default: 5 minutes).
    pub counter_reset_window: Duration,

    /// Duration a partition must remain unavailable before failback (default: 5s).
    pub partition_unavailability_duration: Duration,

    /// Interval for the background failback sweep (default: 300s).
    pub failback_sweep_interval: Duration,

    /// Whether PPCB is enabled via options (default: false).
    pub circuit_breaker_option_enabled: bool,

    /// Consecutive alternate-region hedge wins on the same
    /// `(partition, primary_region)` pair before PPCB trips the partition.
    /// Default `5` matches the .NET v3 SDK convention; override via
    /// [`OperationOptions::consecutive_hedge_win_threshold`] or
    /// `AZURE_COSMOS_CONSECUTIVE_HEDGE_WIN_THRESHOLD`.
    ///
    /// Cross-region hedging surfaces a steady
    /// signal of primary-region degradation when the alternate consistently
    /// beats the primary. Tripping the partition routes subsequent requests
    /// away from the degraded region until the failback loop allows a probe.
    pub consecutive_hedge_win_threshold: u32,
}

impl Default for PartitionFailoverConfig {
    fn default() -> Self {
        Self {
            read_failure_threshold: 10,
            write_failure_threshold: 5,
            counter_reset_window: Duration::from_secs(5 * 60),
            partition_unavailability_duration: Duration::from_secs(5),
            failback_sweep_interval: Duration::from_secs(300),
            circuit_breaker_option_enabled: false,
            consecutive_hedge_win_threshold: 5,
        }
    }
}

impl PartitionFailoverConfig {
    /// Creates a `PartitionFailoverConfig` by resolving values from the
    /// layered [`OperationOptionsView`], falling back to compile-time defaults.
    ///
    /// Called once at driver construction time.
    pub fn from_options(view: &OperationOptionsView<'_>) -> Self {
        let defaults = Self::default();

        let read_failure_threshold = view
            .circuit_breaker_failure_count_for_reads()
            .map(|v| i32::try_from(*v).unwrap_or(i32::MAX))
            .unwrap_or(defaults.read_failure_threshold);

        let write_failure_threshold = view
            .circuit_breaker_failure_count_for_writes()
            .map(|v| i32::try_from(*v).unwrap_or(i32::MAX))
            .unwrap_or(defaults.write_failure_threshold);

        let counter_reset_window_minutes = view
            .circuit_breaker_timeout_counter_reset_window_in_minutes()
            .map(|v| u64::from(*v))
            .unwrap_or(5);

        let partition_unavailability_secs = view
            .allowed_partition_unavailability_duration_in_seconds()
            .map(|v| u64::from(*v))
            .unwrap_or(5);

        let failback_sweep_secs = view
            .ppcb_stale_partition_unavailability_refresh_interval_in_seconds()
            .map(|v| u64::from(*v))
            .unwrap_or(300);

        let circuit_breaker_option_enabled = view
            .per_partition_circuit_breaker_enabled()
            .copied()
            .unwrap_or(false);

        let consecutive_hedge_win_threshold = view
            .consecutive_hedge_win_threshold()
            .copied()
            .unwrap_or(defaults.consecutive_hedge_win_threshold);

        Self {
            read_failure_threshold,
            write_failure_threshold,
            counter_reset_window: Duration::from_secs(counter_reset_window_minutes.max(1) * 60),
            partition_unavailability_duration: Duration::from_secs(
                partition_unavailability_secs.max(1),
            ),
            failback_sweep_interval: Duration::from_secs(failback_sweep_secs.max(1)),
            circuit_breaker_option_enabled,
            consecutive_hedge_win_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let config = PartitionFailoverConfig::default();
        assert_eq!(config.read_failure_threshold, 10);
        assert_eq!(config.write_failure_threshold, 5);
        assert_eq!(config.counter_reset_window, Duration::from_secs(300));
        assert_eq!(
            config.partition_unavailability_duration,
            Duration::from_secs(5)
        );
        assert_eq!(config.failback_sweep_interval, Duration::from_secs(300));
        assert!(!config.circuit_breaker_option_enabled);
        assert_eq!(config.consecutive_hedge_win_threshold, 5);
    }

    #[test]
    fn default_partition_state() {
        let state = PartitionEndpointState::default();
        assert!(state.failover_overrides.is_empty());
        assert!(state.circuit_breaker_overrides.is_empty());
        assert!(state.consecutive_hedge_wins.is_empty());
        assert!(!state.per_partition_automatic_failover_enabled);
        assert!(!state.per_partition_circuit_breaker_enabled);
        assert!(!state.config.circuit_breaker_option_enabled);
    }

    #[test]
    fn from_options_uses_user_supplied_consecutive_hedge_win_threshold() {
        use crate::options::{OperationOptions, OperationOptionsView};

        // User opts into a more aggressive trip threshold (2 wins instead
        // of the default 5).
        let op = OperationOptions {
            consecutive_hedge_win_threshold: Some(2),
            ..Default::default()
        };
        let view = OperationOptionsView::new(None, None, None, Some(&op));
        let config = PartitionFailoverConfig::from_options(&view);

        assert_eq!(
            config.consecutive_hedge_win_threshold, 2,
            "consecutive_hedge_win_threshold must be honored from OperationOptions",
        );
    }

    #[test]
    fn from_options_falls_back_to_default_consecutive_hedge_win_threshold() {
        use crate::options::{OperationOptions, OperationOptionsView};

        let op = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op));
        let config = PartitionFailoverConfig::from_options(&view);

        assert_eq!(
            config.consecutive_hedge_win_threshold,
            PartitionFailoverConfig::default().consecutive_hedge_win_threshold,
            "absent option must fall back to the compile-time default",
        );
    }

    #[test]
    fn from_options_env_override_enables_ppcb_over_operation() {
        use crate::options::{OperationOptions, OperationOptionsView};

        // The `env_override` kill-switch layer (sourced from
        // `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED_OVERRIDE`) must
        // reach `PartitionFailoverConfig::from_options` and beat a per-operation
        // value — here forcing PPCB on even though the operation set it off.
        let env_override = OperationOptions {
            per_partition_circuit_breaker_enabled: Some(true),
            ..Default::default()
        };
        let op = OperationOptions {
            per_partition_circuit_breaker_enabled: Some(false),
            ..Default::default()
        };
        let view = OperationOptionsView::new_with_override(
            Some(std::sync::Arc::new(env_override)),
            None,
            None,
            None,
            Some(&op),
        );
        let config = PartitionFailoverConfig::from_options(&view);

        assert!(
            config.circuit_breaker_option_enabled,
            "env_override PPCB=true must enable the circuit breaker over the operation layer",
        );
    }

    #[test]
    fn from_options_env_override_disables_ppcb_over_operation() {
        use crate::options::{OperationOptions, OperationOptionsView};

        // Parity in the other direction: an override of `false` disables PPCB
        // even when the operation layer turned it on.
        let env_override = OperationOptions {
            per_partition_circuit_breaker_enabled: Some(false),
            ..Default::default()
        };
        let op = OperationOptions {
            per_partition_circuit_breaker_enabled: Some(true),
            ..Default::default()
        };
        let view = OperationOptionsView::new_with_override(
            Some(std::sync::Arc::new(env_override)),
            None,
            None,
            None,
            Some(&op),
        );
        let config = PartitionFailoverConfig::from_options(&view);

        assert!(
            !config.circuit_breaker_option_enabled,
            "env_override PPCB=false must disable the circuit breaker over the operation layer",
        );
    }
}
