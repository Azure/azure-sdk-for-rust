// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for hedge outcomes.

use std::sync::Arc;

use crate::{
    diagnostics::hedging::HedgeDiagnostics,
    models::CosmosOperation,
    options::Region,
    pipeline::{
        operation::{
            hedging::HedgedRaceResult,
            recovery,
            recovery::state::OperationRetryState,
            routing,
            routing::{
                CosmosEndpoint, LocationEffect, LocationEffects, LocationSnapshot,
                UnavailableReason,
            },
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Non-consuming counterpart of [`recovery::classify_result()`] for the
/// hedge race loop. Returns the same `LocationEffect`s the consuming
/// path would have emitted, plus an `observed_session_unavailable` bool
/// in place of the 404/1002 `OperationRetryState` transition (the race
/// coordinator applies the latch flip at the `BothTransient` boundary).
/// The `OperationAction` is discarded since `classify_hedge_result`
/// picks the next-step action.
pub(crate) fn evaluate_hedge_leg_effects(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    result: &TransportResult,
) -> HedgeLegEvaluation {
    let mut eval = HedgeLegEvaluation::default();
    match &result.outcome {
        TransportOutcome::Success { .. } => {}

        TransportOutcome::HttpError {
            status,
            request_sent,
            ..
        } => {
            // Mirror `build_session_retry_state`'s four-condition latch
            // trigger.
            if status.is_read_session_not_available()
                && retry_state.can_retry_session()
                && retry_state.is_dataplane
                && !retry_state.can_use_multiple_write_locations
                && retry_state.session_token_retry_count == 0
                && !retry_state.hub_region_processing_only
            {
                eval.observed_session_unavailable = true;
            }

            // Walk `evaluate_http_outcome`'s priority chain by-reference
            // and take only the `LocationEffect`s. 404/1002 is handled
            // above via `observed_session_unavailable` and emits no
            // effects, so it is skipped here.
            if let Some((_action, effects)) =
                recovery::write_region::recover(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            } else if let Some((_action, effects)) =
                recovery::account::recover(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            } else if let Some((_action, effects)) =
                recovery::failover::recover(operation, endpoint, retry_state, status, *request_sent)
            {
                eval.effects = effects;
            } else if let Some((_action, effects)) =
                recovery::server::recover(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            }
        }

        TransportOutcome::TransportError { request_sent, .. } => {
            // Mirrors `evaluate_transport_layer_outcome`: a not-sent failure is
            // endpoint-wide (mark the endpoint, leave PPCB's partition counter
            // untouched); a sent/unknown failure means the endpoint is
            // reachable but this partition had an issue (mark the partition
            // only — sibling partitions on the endpoint are unaffected).
            if request_sent.definitely_not_sent() {
                eval.effects.push(LocationEffect::MarkEndpointUnavailable {
                    endpoint: endpoint.clone(),
                    reason: UnavailableReason::TransportError,
                });
            } else {
                eval.effects.push(LocationEffect::MarkPartitionUnavailable(
                    routing::make_partition_unavailable(
                        operation,
                        endpoint,
                        retry_state,
                        operation.is_read_only(),
                    ),
                ));
            }
        }

        TransportOutcome::DeadlineExceeded { .. } => {
            // Client-side timeout — no routing-state effects.
        }
    }
    eval
}

/// Side effects observed by a single hedge leg, returned by
/// [`evaluate_hedge_leg_effects`] so the race coordinator can mirror the
/// non-hedged [`recovery::classify_result()`] path without consuming the
/// `TransportResult`.
#[derive(Debug, Default)]
pub(crate) struct HedgeLegEvaluation {
    pub(crate) effects: LocationEffects,
    /// `true` when this leg observed a 404/1002 that would have triggered
    /// [`recovery::session::build_session_retry_state()`]'s `hub_region_processing_only` latch
    /// on the non-hedged path.
    pub(crate) observed_session_unavailable: bool,
}
/// Applies one completed leg's routing feedback and returns its session-unavailable observation.
///
/// Classification uses the header actually sent by that leg, not a later signal value.
/// Request-preparation failures have no response feedback. Publication happens only
/// after the existing location-update await has completed.
pub(crate) async fn observe(
    ctx: &crate::pipeline::operation::AttemptContext<'_>,
    retry_state_snapshot: &OperationRetryState,
    endpoint: &CosmosEndpoint,
    result: &crate::error::Result<TransportResult>,
    hub_only_read: bool,
) -> bool {
    // Pre-transport / request-build errors carry no `TransportResult`
    // and therefore no observable side effects to mirror — they fail
    // before the transport pipeline classifies any response.
    let Ok(transport_result) = result.as_ref() else {
        return false;
    };
    // Judge the leg by the header it sent: the latch may have flipped after the snapshot.
    let latched_state;
    let retry_state = if hub_only_read && !retry_state_snapshot.hub_region_processing_only {
        let mut state = retry_state_snapshot.clone();
        state.hub_region_processing_only = true;
        latched_state = state;
        &latched_state
    } else {
        retry_state_snapshot
    };
    let eval = recovery::hedge::evaluate_hedge_leg_effects(
        ctx.operation,
        endpoint,
        retry_state,
        transport_result,
    );
    routing::apply_hedge_effects(ctx, &eval.effects).await;
    eval.observed_session_unavailable
}

/// Charges two raced regions and advances past both before sequential fallback.
///
/// Returns the last error if retry budget is exhausted. The secondary need not be
/// adjacent to the primary in preferred-region order.
pub(crate) fn try_advance_after_both_transient(
    retry_state: &mut OperationRetryState,
    location: &LocationSnapshot,
    is_read_only: bool,
    primary_region: Option<&Region>,
    secondary_region: Option<&Region>,
    last_error: crate::error::CosmosError,
) -> Result<(), crate::error::CosmosError> {
    let consumed: u32 = 2;
    let next_count = retry_state.failover_retry_count.saturating_add(consumed);
    if next_count > retry_state.max_failover_retries {
        tracing::debug!(
            failover_retry_count = retry_state.failover_retry_count,
            max_failover_retries = retry_state.max_failover_retries,
            "hedge both-transient: failover budget exhausted; surfacing terminal error",
        );
        return Err(last_error);
    }

    retry_state.failover_retry_count = next_count;
    let endpoints = routing::resolve_candidate_endpoints(
        location,
        retry_state,
        &routing::RoutingPolicy {
            read_only: is_read_only,
            requires_write_endpoint: !is_read_only,
            distributed_transaction: false,
            prefer_write_endpoints_for_read: false,
        },
    )
    .endpoints;
    let endpoints_len = endpoints.len();
    if endpoints_len > 0 {
        // Walk the LocationIndex forward starting from its current
        // position. `next_for_generation` rebases stale indices on the
        // first call when the snapshot generation changed, so the
        // first step lands on a position in the refreshed ordering.
        // We then keep advancing while the endpoint at the resolved
        // index belongs to a region that was just raced (primary or
        // secondary), bounded by `endpoints_len` iterations so we
        // always make forward progress even when only the raced
        // regions remain.
        let raced_region = |candidate: Option<&Region>| -> bool {
            match candidate {
                Some(c) => {
                    primary_region.is_some_and(|p| p == c)
                        || secondary_region.is_some_and(|s| s == c)
                }
                None => false,
            }
        };

        let mut advanced = retry_state
            .location
            .next_for_generation(endpoints_len, location.account.generation);
        // `endpoints_len - 1` extra steps is enough: combined with the
        // initial advance above we cover every distinct index in the
        // list, and the loop bails out the moment we find one that
        // is not in the raced set.
        for _ in 0..endpoints_len.saturating_sub(1) {
            if !raced_region(endpoints[advanced.index()].region()) {
                break;
            }
            advanced = advanced.next_for_generation(endpoints_len, location.account.generation);
        }
        retry_state.location = advanced;
    }
    tracing::debug!(
        failover_retry_count = retry_state.failover_retry_count,
        max_failover_retries = retry_state.max_failover_retries,
        "hedge both-transient: failover loop will continue against remaining regions",
    );
    Ok(())
}

/// Adopts a race's session-unavailable observation into exclusively owned retry progress.
///
/// Consumes session budget when available and preserves the single-master-only hub latch.
pub(crate) fn propagate_hedge_session_unavailable(
    retry_state: &mut OperationRetryState,
    observed_session_unavailable: bool,
) {
    if !observed_session_unavailable {
        return;
    }
    // Gate on `can_retry_session` to avoid advancing past the cap.
    if retry_state.can_retry_session() {
        *retry_state = retry_state.clone().advance_session_retry();
    }
    // The `hub_region_processing_only` latch is a single-master-only concept:
    // multi-master accounts never emit the hub-region header and must not enter
    // the hub-region 403/3 or cache paths. Mirror `build_session_retry_state`'s
    // `!can_use_multiple_write_locations` gate so the hedge-fallback setter
    // cannot latch it on a multi-master account. (The session-retry counter is
    // advanced above regardless, since that cap applies to every account type.)
    if retry_state.can_use_multiple_write_locations || retry_state.hub_region_processing_only {
        return;
    }
    retry_state.hub_region_processing_only = true;
    tracing::debug!(
        session_token_retry_count = retry_state.session_token_retry_count,
        max_session_retries = retry_state.max_session_retries,
        "hedge both-transient: 1002 observed by at least one leg; \
         flipped hub_region_processing_only latch and advanced \
         session-retry counter for next attempt",
    );
}

/// Adopts race observations and charges both raced regions before sequential fallback.
///
/// Only an exhausted budget makes both-transient diagnostics terminal; a later
/// successful fallback must not retain that terminal classification.
pub(crate) fn after_race(
    outcome: crate::pipeline::operation::hedging::HedgedRaceResult,
    retry_state: &mut OperationRetryState,
    location: &LocationSnapshot,
    operation: &CosmosOperation,
) -> std::ops::ControlFlow<
    crate::error::Result<crate::models::CosmosResponse>,
    crate::diagnostics::DiagnosticsContextBuilder,
> {
    match outcome {
        HedgedRaceResult::Terminal(result) => std::ops::ControlFlow::Break(result),
        HedgedRaceResult::BothTransient {
            primary_region,
            secondary_region,
            strategy_config,
            primary_region_for_diag,
            secondary_region_for_diag,
            last_error,
            diagnostics: returned_diagnostics,
            partition_key_range_id: race_pk_range_id,
            observed_session_unavailable: race_observed_1002,
        } => {
            tracing::warn!(
                primary_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                "hedge race: both transient on STAGE 2b; attempting failover fallback",
            );
            let mut diagnostics = returned_diagnostics;

            if retry_state.partition_key_range_id.is_none() {
                retry_state.partition_key_range_id = race_pk_range_id;
            }
            recovery::hedge::propagate_hedge_session_unavailable(retry_state, race_observed_1002);
            if let Err(e) = recovery::hedge::try_advance_after_both_transient(
                retry_state,
                location,
                operation.is_read_only(),
                primary_region.as_ref(),
                secondary_region.as_ref(),
                last_error,
            ) {
                diagnostics.set_hedge_diagnostics(HedgeDiagnostics::both_transient(
                    strategy_config,
                    primary_region_for_diag,
                    secondary_region_for_diag,
                    false,
                ));
                let diagnostics_ctx = Arc::new(diagnostics.complete());
                return std::ops::ControlFlow::Break(Err(
                    crate::error::CosmosErrorBuilder::from_error(e)
                        .with_diagnostics(diagnostics_ctx)
                        .build(),
                ));
            }

            std::ops::ControlFlow::Continue(diagnostics)
        }
    }
}
