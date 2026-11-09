// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Publishes routing feedback at sequential and hedge observation checkpoints.

use crate::{
    models::{CosmosOperation, CosmosStatus},
    options::Region,
    pipeline::{
        operation::{
            recovery::state::OperationRetryState,
            routing,
            routing::{
                partition_endpoint_state::{HealthStatus, PartitionStateUpdate},
                partition_key_range_id::PartitionKeyRangeId,
                CosmosEndpoint, LocationEffect, LocationEffects, LocationSnapshot,
                LocationStateStore, UnavailablePartition,
            },
            AttemptContext,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Suppresses partition and endpoint failure feedback from write-region verification reads.
pub(crate) fn location_effects_for_operation(
    operation: &CosmosOperation,
    effects: LocationEffects,
) -> LocationEffects {
    if !operation.prefers_write_endpoints_for_read() {
        return effects;
    }
    effects
        .into_iter()
        .filter(|effect| {
            !matches!(
                effect,
                LocationEffect::MarkPartitionUnavailable(_)
                    | LocationEffect::MarkEndpointUnavailable { .. }
            )
        })
        .collect()
}

/// Applies confirmed write failures that are not already represented by current routing state.
///
/// The caller must first observe success or a region-confirming abort. Draining
/// before the update prevents the same deferred evidence from being committed twice.
pub(crate) async fn flush_pending_write_effects(
    retry_state: &mut OperationRetryState,
    location_state_store: &LocationStateStore,
) {
    if retry_state.pending_write_effects.is_empty() {
        return;
    }
    let pending = std::mem::take(&mut retry_state.pending_write_effects);
    let snapshot = location_state_store.capture();
    let effects: LocationEffects = pending
        .into_iter()
        .filter(|effect| !self::is_effect_already_applied(effect, &snapshot))
        .collect();
    if effects.is_empty() {
        return;
    }
    location_state_store.apply(&effects).await;
}

/// Returns `true` when the given location effect would be a no-op against the
/// current snapshot — i.e., the routing state has already converged on it.
///
/// This is a conservative check: false negatives (returning `false` when the
/// effect is in fact a no-op) are safe (we just pay the redundant CAS once
/// more). False positives (skipping an effect that would actually mutate
/// state) would be a correctness bug.
pub(crate) fn is_effect_already_applied(
    effect: &LocationEffect,
    snapshot: &LocationSnapshot,
) -> bool {
    match effect {
        LocationEffect::MarkEndpointUnavailable { endpoint, .. } => snapshot
            .account
            .unavailable_endpoints
            .contains_key(endpoint.url()),
        LocationEffect::MarkPartitionUnavailable(partition) => {
            // Without a partition_key_range_id we cannot register an override
            // anyway, so apply() would skip it — treat as already-applied.
            let Some(pk_range_id) = partition.partition_key_range_id.as_ref() else {
                return true;
            };
            let Some(failed_region) = partition.region.as_ref() else {
                return false;
            };
            let partitions = snapshot.partitions.as_ref();
            // Check both override maps. If an entry exists for this partition
            // and its current_endpoint is already a different region than the
            // failed one, the failover has already moved past — re-applying
            // would just bump last_failure_time without changing routing.
            let already_moved = |entry: &crate::pipeline::operation::routing::partition_endpoint_state::PartitionFailoverEntry| -> bool {
                entry
                    .current_endpoint
                    .region()
                    .is_some_and(|r| r != failed_region)
            };
            partitions
                .failover_overrides
                .get(pk_range_id.as_str())
                .is_some_and(already_moved)
                || partitions
                    .circuit_breaker_overrides
                    .get(pk_range_id.as_str())
                    .is_some_and(already_moved)
        }
        // RefreshAccountProperties is internally rate-limited by
        // `refresh_account_properties_if_due` — let the store decide.
        LocationEffect::RefreshAccountProperties => false,
        // Hub-region cache mutations are emitted at most once per operation
        // (CacheHubRegion at Complete, AdvanceHubRegionDiscovery per 403/3
        // attempt). Never treat them as already-applied — letting them
        // run is idempotent at the routing-systems level (cache_hub_region
        // does an upsert, advance_hub_region_discovery rotates the entry).
        LocationEffect::CacheHubRegion { .. }
        | LocationEffect::AdvanceHubRegionDiscovery { .. } => false,
    }
}

/// On a successful PPCB probe request, removes the `ProbeCandidate` entry
/// for this partition so subsequent requests resume default routing.
///
/// Only PPCB circuit-breaker overrides use probe-based failback. PPAF
/// overrides persist until the backend signals a change organically, so
/// this function is a no-op for them.
///
/// **Fast path**: skips the CAS loop entirely when no `ProbeCandidate`
/// entry exists for this partition, avoiding an expensive full-state
/// clone on every successful request (the overwhelmingly common case).
///
/// **TOCTOU guard**: re-checks the `ProbeCandidate` status inside the CAS
/// closure. If another thread transitions the entry to `Unhealthy` between
/// the snapshot check and the CAS, the unhealthy entry must not be removed.
pub(crate) fn try_cleanup_probe_candidate(
    retry_state: &OperationRetryState,
    location_state_store: &LocationStateStore,
) {
    let Some(pk_range_id) = &retry_state.partition_key_range_id else {
        return;
    };

    let snapshot = location_state_store.capture();
    let needs_cleanup = snapshot
        .partitions
        .circuit_breaker_overrides
        .get(pk_range_id.as_str())
        .is_some_and(|e| e.health_status == HealthStatus::ProbeCandidate);
    if !needs_cleanup {
        return;
    }

    #[cfg(test)]
    probe_cleanup_tests::before_update(location_state_store);

    location_state_store.apply_partition(|current| {
        let is_probe = current
            .circuit_breaker_overrides
            .get(pk_range_id.as_str())
            .is_some_and(|e| e.health_status == HealthStatus::ProbeCandidate);
        if is_probe {
            PartitionStateUpdate::Updated(routing::remove_probe_succeeded_entry(
                current,
                pk_range_id,
            ))
        } else {
            PartitionStateUpdate::Unchanged
        }
    });
}

/// Returns the partition key range ID to cache as the hub region if the
/// completed operation satisfies the populate gate; otherwise `None`.
///
/// The gate requires all three: the hub-region-processing-only latch was
/// active for this attempt, a partition key range ID is known, and the
/// operation is a read. The caller additionally skips the apply when PPAF is
/// disabled or the entry already points at the hub endpoint.
pub(crate) fn hub_region_cache_populate_target(
    retry_state: &OperationRetryState,
    operation: &CosmosOperation,
) -> Option<PartitionKeyRangeId> {
    if retry_state.hub_region_processing_only && operation.is_read_only() {
        retry_state.partition_key_range_id.clone()
    } else {
        None
    }
}

/// Caches `hub_endpoint` as the partition's hub region.
///
/// Skips the apply when it would not change state: PPAF off (the cache effect
/// is a no-op) or the entry already points at this endpoint.
pub(crate) async fn populate_hub_region_cache(
    location_state_store: &LocationStateStore,
    pk_range_id: &PartitionKeyRangeId,
    hub_endpoint: &CosmosEndpoint,
) {
    let partitions = location_state_store.capture().partitions;
    let already_cached = partitions
        .failover_overrides
        .get(pk_range_id.as_str())
        .is_some_and(|entry| entry.current_endpoint == *hub_endpoint);
    if partitions.per_partition_automatic_failover_enabled && !already_cached {
        location_state_store
            .apply(&[LocationEffect::CacheHubRegion {
                partition_key_range_id: pk_range_id.clone(),
                hub_endpoint: hub_endpoint.clone(),
            }])
            .await;
    }
}

/// Hedged counterpart of the `Complete` arm's hub-region cache populate: a
/// hub-only read leg that succeeded was served by the partition's hub.
pub(crate) async fn populate_hub_region_cache_for_hedge_winner(
    ctx: &AttemptContext<'_>,
    endpoint: &CosmosEndpoint,
    hub_only_read: bool,
    result: &TransportResult,
    pk_range_id: Option<&PartitionKeyRangeId>,
) {
    let Some(pk_range_id) = pk_range_id else {
        return;
    };
    if hub_only_read && matches!(result.outcome, TransportOutcome::Success { .. }) {
        self::populate_hub_region_cache(ctx.services.location_state_store, pk_range_id, endpoint)
            .await;
    }
}

/// Dispatches a hedge-race outcome to the [`LocationStateStore`]'s
/// PPCB-feedback counters.
///
/// Feedback is keyed by `(partition, primary_region)`. When the
/// operation never resolved a partition key range ID (first attempt
/// against an unresolved item) the call is a no-op: there is no key to
/// attribute feedback against. The same gating applies to both
/// alternate-wins and primary-wins so the reset only fires on pairs
/// that could ever accumulate a count in the first place.
pub(crate) fn record_hedge_outcome(
    location_state_store: &LocationStateStore,
    outcome: HedgeOutcome,
    partition: Option<&PartitionKeyRangeId>,
    primary_region: Option<&Region>,
) {
    let Some(partition) = partition else {
        return;
    };
    match outcome {
        HedgeOutcome::AlternateWin => {
            location_state_store.record_consecutive_hedge_win(partition, primary_region)
        }
        HedgeOutcome::PrimaryWin => {
            location_state_store.record_primary_win(partition, primary_region)
        }
    }
}

/// Hedge race outcome reported to [`record_hedge_outcome`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HedgeOutcome {
    /// The primary attempt produced the operation result (whether
    /// pre-threshold in the zero-overhead happy path or post-threshold
    /// after racing the secondary).
    PrimaryWin,
    /// The alternate (secondary) attempt produced the operation result.
    AlternateWin,
}

/// Publishes a completed hedge leg's feedback before the coordinator polls its partner.
///
/// Unlike sequential write recovery, race observations are not deferred or applied
/// to the invocation's retry progress.
pub(crate) async fn apply_hedge_effects(ctx: &AttemptContext<'_>, effects: &[LocationEffect]) {
    if !effects.is_empty() {
        ctx.services
            .location_state_store
            .apply_with_options(effects, Some(ctx.config.options))
            .await;
    }
}

/// Applies attempt feedback, retaining unconfirmed single-master write failures locally.
pub(crate) async fn apply_location_effects(
    operation: &CosmosOperation,
    effects: LocationEffects,
    retry_state: &mut OperationRetryState,
    location_state_store: &LocationStateStore,
    options: &crate::options::OperationOptionsView<'_>,
) {
    let effects = self::location_effects_for_operation(operation, effects);
    let (immediate_effects, deferred_effects) = routing::partition_effects_for_deferral(
        operation.is_read_only(),
        retry_state.can_use_multiple_write_locations,
        retry_state.ppaf_write_retry_allowed,
        effects,
    );
    retry_state.pending_write_effects.extend(deferred_effects);
    location_state_store
        .apply_with_options(&immediate_effects, Some(options))
        .await;
}

/// Commits confirmed write failures and successful probe/hub feedback before response construction.
pub(crate) async fn complete_attempt(
    operation: &CosmosOperation,
    retry_state: &mut OperationRetryState,
    location_state_store: &LocationStateStore,
    endpoint: &CosmosEndpoint,
) {
    // Flush any deferred write-path effects now that the write
    // has definitively succeeded. The current region is proven
    // healthy, so the previously-failed regions can be safely
    // marked unavailable for this partition (and endpoint, when
    // PPAF is active).
    self::flush_pending_write_effects(retry_state, location_state_store).await;

    // If a PPCB probe request succeeded, remove the ProbeCandidate entry.
    self::try_cleanup_probe_candidate(retry_state, location_state_store);

    // Hub-region cache populate: when the hub-region-processing-only
    // latch was active and we have a partition key range ID, the
    // region that produced this 2xx is — by definition — the
    // partition's current hub. Cache it so subsequent operations
    // that latch the header skip the 403/3 discovery chain.
    if let Some(pk_range_id) = self::hub_region_cache_populate_target(retry_state, operation) {
        self::populate_hub_region_cache(location_state_store, &pk_range_id, endpoint).await;
    }
}

/// Commits deferred failures only when an abort proves that a region processed the request.
pub(crate) async fn abort_attempt(
    retry_state: &mut OperationRetryState,
    location_state_store: &LocationStateStore,
    cosmos_status: &crate::models::CosmosStatus,
) {
    // Flush deferred write-path effects if the abort status
    // confirms the region processed the request (e.g., 409
    // Conflict, 412 Precondition Failed). On non-confirming
    // aborts (503/429-3092/410/408/403-3/transport error/deadline)
    // the buffered effects are discarded — we never proved any
    // region was actually healthy, so polluting routing state
    // would be wrong.
    let confirming = routing::is_region_confirming_status(cosmos_status);
    if confirming {
        self::flush_pending_write_effects(retry_state, location_state_store).await;
    } else {
        retry_state.pending_write_effects.clear();
    }
}

#[cfg(test)]
#[path = "probe_cleanup_tests.rs"]
mod probe_cleanup_tests;

/// Whether the current request is handled by the PPCB threshold mechanism.
///
/// When `true`, `MarkEndpointUnavailable` should be suppressed — failover
/// is driven by the partition-level failure counter instead.
pub(crate) fn is_ppcb_managed(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
) -> bool {
    retry_state.ppcb_active
        && operation
            .resource_type()
            .is_partitioned(operation.operation_type())
        && (operation.is_read_only() || retry_state.can_use_multiple_write_locations)
}

/// Builds an [`UnavailablePartition`] from the current operation context.
///
/// `is_read` is passed explicitly because the WriteForbidden handler hardcodes
/// it to `false` (the request was a write, even though the gateway redirected
/// us to read from a different region). All other call sites derive it from
/// `operation.is_read_only()` so that PPCB increments the correct
/// per-partition counter (read vs write) and gates failover by the matching
/// threshold.
pub(crate) fn make_partition_unavailable(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    is_read: bool,
) -> UnavailablePartition {
    UnavailablePartition {
        partition_key_range_id: retry_state.partition_key_range_id.clone(),
        region: endpoint.region().cloned(),
        is_read,
        is_partitioned_resource: operation
            .resource_type()
            .is_partitioned(operation.operation_type()),
    }
}

/// Returns `true` when a status proves the request reached a server and was
/// processed by it.
///
/// Used by the operation pipeline to decide whether to flush deferred
/// write-path `MarkPartitionUnavailable` (and, for PPAF on single-master,
/// `MarkEndpointUnavailable`) effects when the operation aborts rather than
/// completing successfully. The intuition is: any response outside the
/// retry-trigger set means the *current* region accepted the request, which
/// retroactively confirms that the *earlier* failed regions were the
/// unhealthy ones — their pending marks should be applied so future requests
/// route around them.
///
/// The decision is **deny-listed**, not allow-listed: every status is
/// treated as region-confirming **except** the explicit retry-trigger set
/// and client-synthesized statuses. This means uncommon-but-deterministic
/// service responses (202 Accepted, 207 MultiStatus, 404/0 NotFound, 413
/// Payload Too Large, 451 Unavailable For Legal Reasons, etc.) all flush
/// deferred marks just like the more familiar 200/409/412. **449 RetryWith
/// is deliberately excluded** from the confirming set — see the note on
/// `try_handle_retry_with`; the request hasn't completed yet and the next
/// retry must hit the same region.
///
/// Returns `false` for:
/// - 503 ServiceUnavailable, 408 RequestTimeout, 410 Gone, 429/3092 (system
///   resource unavailable), 403/3 (write forbidden) — the retry-trigger set;
///   we have no proof any region accepted the request.
/// - 449 RetryWith — the request never completed; the backend is asking for
///   another attempt in the same region. Deferred effects must not flush.
/// - Client-synthesized statuses (e.g. `CLIENT_OPERATION_TIMEOUT`) — these
///   never came from a server.
///
/// Returns `true` for everything else, including:
/// - All 2xx (200 OK, 201 Created, 202 Accepted, 204 No Content, 207
///   MultiStatus). The 2xx case is normally taken via
///   `OperationAction::Complete`, but is preserved here for defense in depth.
/// - Definitive 4xx (400, 401, 404 with any non-1002 sub-status, 409
///   Conflict, 412 Precondition Failed, 413 Payload Too Large) — the server
///   processed and rejected the request.
/// - Server errors (500, 501, 504, 505) once the retry budget is exhausted.
pub(crate) fn is_region_confirming_status(status: &CosmosStatus) -> bool {
    let code = status.status_code();

    if code.is_success() {
        return true;
    }

    // Retry-trigger statuses — not confirming.
    if code == azure_core::http::StatusCode::ServiceUnavailable
        || code == azure_core::http::StatusCode::RequestTimeout
        || code == azure_core::http::StatusCode::Gone
    {
        return false;
    }

    // 449 RetryWith — request never completed, same-region retry pending.
    if status.is_retry_with() {
        return false;
    }

    if status.is_throttled()
        && status.sub_status()
            == Some(crate::error::status_codes::substatus::SYSTEM_RESOURCE_UNAVAILABLE)
    {
        return false;
    }

    if status.is_write_forbidden() || status.is_database_account_not_found() {
        return false;
    }

    // Synthesized client-side statuses (e.g., end-to-end timeout) — not from a server.
    if status.sub_status() == Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT)
    {
        return false;
    }

    // Any other status from the service confirms the region processed the request.
    true
}

/// Splits a list of location effects into immediate effects and deferred
/// write-path effects.
///
/// PPCB-managed operations — reads, and writes on a multi-master account —
/// always apply effects immediately. The per-partition failure counter is
/// what drives threshold-based failover, and it must see every failure
/// signal at the moment it is observed; deferring would prevent the breaker
/// from ever tripping for non-idempotent writes that abort.
///
/// Single-master writes (where the per-partition circuit breaker is not
/// applicable) defer `MarkPartitionUnavailable` until the write definitively
/// reaches a region. When PPAF is additionally enabled
/// (`ppaf_write_retry_allowed`), `MarkEndpointUnavailable` is also deferred
/// so a transient retry against the only known write region cannot pollute
/// the endpoint-unavailability state with an unverified failure.
pub(crate) fn partition_effects_for_deferral(
    is_read_only: bool,
    can_use_multiple_write_locations: bool,
    ppaf_write_retry_allowed: bool,
    effects: LocationEffects,
) -> (LocationEffects, LocationEffects) {
    // PPCB-managed paths (reads and multi-master writes) bypass deferral so
    // the partition failure counter increments immediately on every 503.
    if is_read_only || can_use_multiple_write_locations {
        return (effects, LocationEffects::default());
    }
    let mut immediate = Vec::with_capacity(effects.len());
    let mut deferred = Vec::new();
    for effect in effects {
        match effect {
            LocationEffect::MarkPartitionUnavailable(_) => deferred.push(effect),
            LocationEffect::MarkEndpointUnavailable { .. } if ppaf_write_retry_allowed => {
                deferred.push(effect);
            }
            other => immediate.push(other),
        }
    }
    (immediate.into(), deferred.into())
}
