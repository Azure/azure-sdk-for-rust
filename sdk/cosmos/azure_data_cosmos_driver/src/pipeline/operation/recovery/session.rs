// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for session outcomes.

use crate::{
    models::{CosmosResponseHeaders, CosmosStatus},
    pipeline::operation::{
        recovery,
        recovery::state::{OperationAction, OperationRetryState},
        routing::LocationEffects,
    },
};

/// Handles 404/1002 ReadSessionNotAvailable — session token is ahead of the
/// region being read from (session lag).
///
/// On single-master accounts a session retry that has already happened twice
/// is treated as a permanent miss (the writes truly haven't replicated and
/// retrying further is unlikely to help). Otherwise issues a `SessionRetry`
/// which advances to a different region without consuming failover budget.
pub(crate) fn recover(
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
) -> Option<(OperationAction, LocationEffects)> {
    if !(status.is_read_session_not_available() && retry_state.can_retry_session()) {
        return None;
    }

    if !retry_state.can_use_multiple_write_locations && retry_state.session_token_retry_count >= 2 {
        return Some((
            OperationAction::Abort {
                error: recovery::error::build_service_error(status, cosmos_headers, body),
            },
            LocationEffects::default(),
        ));
    }

    Some((
        OperationAction::SessionRetry {
            new_state: recovery::session::build_session_retry_state(retry_state),
        },
        LocationEffects::default(),
    ))
}

/// Builds the `OperationRetryState` for a 404/1002 session retry,
/// latching the `hub_region_processing_only` flag when the trigger
/// conditions defined by Spec 0010: Hub-region processing header.
///
/// All four conditions must hold (Spec 0010 §7.1 / public spec §3.3):
///
/// 1. `is_dataplane` — metadata operations ride the same pipeline but
///    are scoped out per spec §1.5 (AC-8).
/// 2. `!can_use_multiple_write_locations` — single-master only (AC-4).
/// 3. `session_token_retry_count == 0` — first 1002 within the
///    operation; the count is incremented by `advance_session_retry`
///    so reading `retry_state.session_token_retry_count` here detects
///    the pre-increment value (AC-3, S2 / T-5).
/// 4. `!hub_region_processing_only` — defense-in-depth idempotency;
///    structurally already guaranteed by latch-once semantics.
///
/// Returns an independent proposal; the hedge coordinator owns cross-leg publication.
pub(crate) fn build_session_retry_state(retry_state: &OperationRetryState) -> OperationRetryState {
    let mut new_state = retry_state.clone().advance_session_retry();
    if retry_state.is_dataplane
        && !retry_state.can_use_multiple_write_locations
        && retry_state.session_token_retry_count == 0
        && !retry_state.hub_region_processing_only
    {
        new_state.hub_region_processing_only = true;
    }
    new_state
}
