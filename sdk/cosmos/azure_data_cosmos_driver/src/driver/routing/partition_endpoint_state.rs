// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition-level endpoint routing state for PPAF and PPCB.

use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use crate::options::{PartitionFailoverOptions, Region};

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
    /// win. When the count reaches
    /// [`PartitionFailoverOptions::consecutive_hedge_win_threshold`]
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

    /// Driver-level partition-failover / PPCB tuning, captured once at
    /// construction time from [`DriverOptions`](crate::options::DriverOptions).
    pub config: PartitionFailoverOptions,

    /// Test-only liveness canary. Set by tests that want to observe whether a
    /// particular `PartitionEndpointState` instance is dropped (e.g., the
    /// `apply_partition` use-after-free regression test). Production code never
    /// reads or writes this field; it is `None` in all non-test constructions.
    #[cfg(test)]
    pub(crate) _test_canary: Option<std::sync::Arc<()>>,
}

impl PartitionEndpointState {
    /// Creates a new `PartitionEndpointState` from the given partition failover options.
    pub fn new(config: PartitionFailoverOptions) -> Self {
        // The incident kill switch (`AZURE_COSMOS_PPCB_ENABLED_OVERRIDE`), when
        // set, is authoritative over the base option here. It also wins over the
        // account property later in `LocationStateStore` when properties refresh.
        let per_partition_circuit_breaker_enabled = config
            .circuit_breaker_enabled_override()
            .unwrap_or_else(|| config.circuit_breaker_enabled());
        Self {
            per_partition_circuit_breaker_enabled,
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
        Self::new(PartitionFailoverOptions::default())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_partition_state() {
        let state = PartitionEndpointState::default();
        assert!(state.failover_overrides.is_empty());
        assert!(state.circuit_breaker_overrides.is_empty());
        assert!(state.consecutive_hedge_wins.is_empty());
        assert!(!state.per_partition_automatic_failover_enabled);
        assert!(state.per_partition_circuit_breaker_enabled);
        assert!(state.config.circuit_breaker_enabled());
    }

    #[test]
    fn new_propagates_circuit_breaker_enabled_to_state_flag() {
        // Resolve against an empty environment so unrelated `AZURE_COSMOS_PPCB_*`
        // values from a concurrently-running env-mutating test can't fail the
        // build's bounds validation.
        let opts = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(true)
            .build_from_env(&|_| None)
            .unwrap();
        let state = PartitionEndpointState::new(opts);
        assert!(state.per_partition_circuit_breaker_enabled);
        assert!(state.config.circuit_breaker_enabled());
    }

    #[test]
    fn new_override_off_wins_over_enabled_base_option() {
        // The PPCB incident kill switch (`AZURE_COSMOS_PPCB_ENABLED_OVERRIDE`)
        // is authoritative: an override of `false` forces PPCB off at
        // construction even though the base option is enabled.
        let opts = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(true)
            .with_circuit_breaker_enabled_override(false)
            .build_from_env(&|_| None)
            .unwrap();
        let state = PartitionEndpointState::new(opts);
        assert!(!state.per_partition_circuit_breaker_enabled);
    }

    #[test]
    fn new_override_on_wins_over_disabled_base_option() {
        // Parity in the other direction: an override of `true` forces PPCB on
        // even though the base option is disabled.
        let opts = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(false)
            .with_circuit_breaker_enabled_override(true)
            .build_from_env(&|_| None)
            .unwrap();
        let state = PartitionEndpointState::new(opts);
        assert!(state.per_partition_circuit_breaker_enabled);
    }
}
