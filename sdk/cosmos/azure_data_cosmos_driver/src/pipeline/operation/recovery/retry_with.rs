// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for retry with outcomes.

use crate::{
    models::CosmosStatus,
    pipeline::operation::{
        recovery::state::{OperationAction, OperationRetryState, RetryWithRetryState},
        routing::LocationEffects,
    },
};

/// Handles HTTP 449 RetryWith — the Cosmos backend is signaling a
/// transient concurrency conflict (concurrent writes racing through
/// the store, RBAC info momentarily unavailable, etc.) and asking the
/// client to retry in the same region after a short delay.
///
/// Mirrors the cross-SDK `RetryWithRetryPolicy` (nested in
/// `GoneAndRetryWithRequestRetryPolicy`): the retry schedule starts at
/// ~10ms + a small random salt, exponentially backs off up to ~1s per
/// retry, and gives up once the cumulative delay budget is exhausted
/// (~30s by default). The failover-retry budget is **not** consumed —
/// failing over to another region is not the correct response to a
/// concurrency conflict; the same record needs to be retried in the
/// same region.
///
/// Returns `None` when:
///
/// * The status is not 449 (falls through to other handlers).
/// * The cumulative-wait budget would be exceeded by the next retry
///   delay (caller surfaces the 449 to the application).
pub(crate) fn recover(
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, LocationEffects)> {
    if !status.is_retry_with() {
        return None;
    }

    // Look at (or initialize) the per-operation retry-with state to
    // compute the next delay, then check whether the cumulative budget
    // can absorb it. The delay is computed **once** here and threaded
    // through both the budget check and the cumulative-delay bookkeeping
    // in `advance_retry_with_delay` so the salt's randomness never
    // produces a drift between "budget-checked delay" and "actual sleep".
    let current_state = retry_state
        .retry_with_state
        .clone()
        .unwrap_or_else(RetryWithRetryState::new);
    let delay = current_state.next_delay();
    if !current_state.can_retry(delay) {
        return None;
    }

    Some((
        OperationAction::InRegionRetry {
            new_state: retry_state.clone().advance_retry_with_delay(delay),
            delay,
        },
        // No location effects — 449 is region-local, the next attempt
        // must hit the same region.
        LocationEffects::default(),
    ))
}
