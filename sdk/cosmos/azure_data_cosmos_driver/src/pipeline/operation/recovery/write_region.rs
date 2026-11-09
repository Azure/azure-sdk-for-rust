// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for write region outcomes.

use crate::{
    models::{CosmosOperation, CosmosStatus},
    pipeline::operation::{
        recovery::state::{OperationAction, OperationRetryState},
        routing,
        routing::{CosmosEndpoint, LocationEffect, LocationEffects, UnavailableReason},
    },
};

/// Handles 403/3 WriteForbidden — the gateway has identified that this region
/// is not currently the write region for the partition.
///
/// Always retries cross-region when the failover budget allows, and emits
/// effects to (a) refresh account properties so the new write region is
/// learned, (b) mark this endpoint unavailable, and (c) mark this partition
/// unavailable in the current (read) region for write traffic. Both
/// single-write and multi-write pace retries with exponential backoff bounded
/// by the 5s budget, with the first retry immediate.
///
/// **Hub-region discovery branch.** When the
/// `hub_region_processing_only` latch is active on a read with a known
/// partition key range ID, the 403/3 instead means "this region is not the
/// current hub for this partition". The handler emits an
/// [`LocationEffect::AdvanceHubRegionDiscovery`] that rotates the cached
/// hub endpoint to the next preferred-read endpoint and consumes a
/// failover-budget attempt. Once that budget is exhausted the branch returns
/// `None`, which aborts the operation rather than retrying indefinitely. It
/// does NOT emit
/// [`LocationEffect::MarkEndpointUnavailable`] or
/// [`LocationEffect::RefreshAccountProperties`] — the endpoint is healthy
/// for non-hub reads of other partitions, and the topology is unchanged.
pub(crate) fn recover(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, LocationEffects)> {
    if !status.is_write_forbidden() {
        return None;
    }

    // Hub-region discovery on the read path: rotate the per-partition cache
    // entry to the next preferred-read endpoint and retry without polluting
    // account-level state. The latch is single-master-only (enforced at both
    // setters); the `!can_use_multiple_write_locations` guard below is
    // defense-in-depth so a stray multi-master latch can never route a read
    // into this single-master branch instead of the multi-write budget.
    //
    // **No fall-through to standard write-forbidden effects.** A 403/3 on a
    // read with the hub-region header carries a *partition-scoped* meaning
    // ("this region is not the hub for this partition"), not a region- or
    // endpoint-level health signal. Falling back to
    // `RefreshAccountProperties` / `MarkEndpointUnavailable` /
    // `MarkPartitionUnavailable` would poison routing state for unrelated
    // partitions. When the partition key range ID is known, emit
    // `AdvanceHubRegionDiscovery` to rotate the cache; when missing, retry
    // failover with no location effects.
    if retry_state.hub_region_processing_only
        && operation.is_read_only()
        && !retry_state.can_use_multiple_write_locations
    {
        // Enforce the failover budget here — the outer pipeline does not cap
        // failover attempts on its own, so returning `None` (which aborts) is
        // the only thing that terminates a persistent 403/3 read.
        if !retry_state.can_retry_failover() {
            return None;
        }

        let effects = if let Some(pk_range_id) = retry_state.partition_key_range_id.clone() {
            vec![LocationEffect::AdvanceHubRegionDiscovery {
                partition_key_range_id: pk_range_id,
                failed_endpoint: endpoint.clone(),
            }]
            .into()
        } else {
            // No partition key range ID available for this hub-region read
            // (e.g. a read that did not go through PK-range pre-resolution).
            // Without it we cannot rotate a per-partition cache entry, so fall
            // back to a plain failover retry with no location effects.
            tracing::warn!(
                endpoint = %endpoint.url(),
                failover_retries = retry_state.failover_retry_count,
                "hub-region 403/3 retry has no partition_key_range_id; \
                 skipping AdvanceHubRegionDiscovery and falling back to plain failover retry",
            );
            LocationEffects::default()
        };
        return Some((
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            effects,
        ));
    }

    if !retry_state.can_retry_backend_failover() {
        return None;
    }
    let delay = retry_state.backend_failover_delay();
    let new_state = retry_state.clone().advance_backend_failover(delay);
    let delay = Some(delay);

    let mut effects = vec![
        LocationEffect::RefreshAccountProperties,
        LocationEffect::MarkPartitionUnavailable(routing::make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            false,
        )),
    ];
    if !routing::is_ppcb_managed(operation, retry_state) {
        // PPCB-managed 403/3 is per-partition; do not block the whole endpoint for 60s.
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::WriteForbidden,
        });
    }
    Some((
        OperationAction::FailoverRetry { new_state, delay },
        effects.into(),
    ))
}
