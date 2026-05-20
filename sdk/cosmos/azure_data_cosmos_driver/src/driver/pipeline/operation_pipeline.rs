// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation pipeline: the core loop for executing Cosmos DB operations.
//!
//! Implements the 7-stage operation loop with multi-region failover,
//! session retry, endpoint unavailability tracking, partition-level failover
//! (PPAF/PPCB), and deadline enforcement.

use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use futures::future::{pending, select, Either, Future};

use crate::{
    diagnostics::{DiagnosticsContextBuilder, ExecutionContext, PipelineType, TransportSecurity},
    driver::routing::{
        can_circuit_breaker_trigger_failover, is_eligible_for_ppaf, is_eligible_for_ppcb,
        partition_endpoint_state::HealthStatus, partition_key_range_id::PartitionKeyRangeId,
        remove_probe_succeeded_entry, session_manager::SessionManager, AccountEndpointState,
        CosmosEndpoint, LocationEffect, LocationSnapshot, LocationStateStore,
    },
    driver::transport::CosmosTransport,
    models::{
        request_header_names, AccountEndpoint, ActivityId, CosmosOperation, CosmosResponse,
        Credential, DefaultConsistencyLevel, OperationType, SessionToken, SubStatusCode,
    },
    options::{
        HedgeThreshold, OperationOptionsView, ReadConsistencyStrategy, Region,
        ThroughputControlGroupSnapshot,
    },
};

use super::{
    components::{
        OperationAction, OperationRetryState, RoutingDecision, TransportMode, TransportOutcome,
        TransportRequest, TransportResult,
    },
    hedging_diagnostics::{HedgeDiagnostics, HedgingStrategyConfig},
    hedging_eligibility::{evaluate_hedge_eligibility, is_final_result},
    retry_evaluation::{
        build_http_error, evaluate_transport_result, is_region_confirming_status,
        partition_effects_for_deferral,
    },
};

use crate::driver::transport::{
    transport_pipeline::{execute_transport_pipeline, TransportPipelineContext},
    AuthorizationContext,
};

/// Executes a Cosmos DB operation through the new pipeline architecture.
///
/// This is the entry point called by `CosmosDriver::execute_operation`.
/// It orchestrates the 7-stage operation loop described in the spec.
///
/// When `pre_resolved_pk_range_id` is `Some`, it is used to seed the
/// `OperationRetryState` so that partition-level failover overrides (PPAF/PPCB)
/// can take effect from the very first attempt.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    location_state_store: &LocationStateStore,
    transport: &CosmosTransport,
    account_endpoint: &AccountEndpoint,
    credential: &Credential,
    user_agent: &azure_core::http::headers::HeaderValue,
    activity_id: &ActivityId,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    diagnostics: DiagnosticsContextBuilder,
    session_manager: &SessionManager,
    account_default_consistency: DefaultConsistencyLevel,
    throughput_control: Option<&ThroughputControlGroupSnapshot>,
    pre_resolved_pk_range_id: Option<PartitionKeyRangeId>,
) -> azure_core::Result<CosmosResponse> {
    let mut diagnostics = diagnostics;
    let location_snapshot = location_state_store.snapshot();
    let max_failover_retries = options.max_failover_retry_count().copied().unwrap_or(3);

    // Determine if session consistency is active for this operation.
    let session_capturing_disabled = options
        .session_capturing_disabled()
        .copied()
        .unwrap_or(false);
    let read_consistency_strategy = options
        .read_consistency_strategy()
        .copied()
        .unwrap_or(ReadConsistencyStrategy::Default);
    let session_consistency_active = !session_capturing_disabled
        && read_consistency_strategy.is_session_effective(account_default_consistency);
    let max_session_retries = options
        .max_session_retry_count()
        .copied()
        .unwrap_or_else(|| {
            // Java SDK parity: 2 for single-write, endpoints.len() for multi-write.
            // Uses the original endpoint count (before unavailability filtering).
            if location_snapshot.account.multiple_write_locations_enabled {
                let endpoints_len = location_snapshot
                    .account
                    .preferred_endpoints(operation.is_read_only())
                    .len();
                endpoints_len as u32
            } else {
                2
            }
        });

    let mut retry_state = OperationRetryState::initial(
        location_snapshot.account.generation,
        location_snapshot.account.multiple_write_locations_enabled,
        options
            .excluded_regions()
            .map(|r| r.0.clone())
            .unwrap_or_default(),
        max_failover_retries,
        max_session_retries,
    );

    // Seed the partition key range ID from pre-resolution (PK range cache).
    // This enables PPAF/PPCB partition-level overrides from the very first attempt
    // instead of only after the first retry captures it from response headers.
    retry_state.partition_key_range_id = pre_resolved_pk_range_id;

    // PPAF write-retry: on single-master accounts with per-partition automatic
    // failover enabled, only PPAF-eligible operations (partitioned writes) may
    // be retried to a different region for write region discovery. This avoids
    // enabling unsafe retries for non-partitioned writes such as database or
    // container creates.
    retry_state.ppaf_write_retry_allowed = location_snapshot
        .partitions
        .per_partition_automatic_failover_enabled
        && !location_snapshot.account.multiple_write_locations_enabled
        && operation
            .resource_type()
            .is_partitioned(operation.operation_type());

    // PPCB: when circuit breaker is enabled, partition-level thresholds
    // drive failover instead of marking the whole endpoint unavailable.
    retry_state.ppcb_active = location_snapshot
        .partitions
        .per_partition_circuit_breaker_enabled
        && location_snapshot.account.preferred_read_endpoints.len() > 1;

    // HUB_REGION_PROCESSING_HEADER_SPEC.md §1.5: gate the
    // `x-ms-cosmos-hub-region-processing-only` latch on data-plane scope
    // so metadata-pipeline operations (which ride the same
    // `execute_operation_pipeline`) never emit the header.
    //
    // Use the `PipelineType::is_data_plane()` accessor — NOT `==` matching
    // — because `PipelineType` is `#[non_exhaustive]` and a future variant
    // would silently bypass an equality gate. Equivalently
    // `!pipeline_type.is_metadata()` (the metadata pipeline is the only
    // current variant that is out of spec scope).
    retry_state.is_dataplane = pipeline_type.is_data_plane();

    let deadline = options
        .end_to_end_latency_policy()
        .map(|p| Instant::now() + p.timeout());

    loop {
        // ── STAGE 1: Acquire LocationSnapshot ──────────────────────────
        let location = location_state_store.snapshot();

        // ── STAGE 2: Resolve endpoint ──────────────────────────────────
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            &location,
            pipeline_type.is_data_plane(),
            location_state_store.endpoint_unavailability_ttl(),
        );

        // Emit one structured debug record per attempt with the chosen
        // routing decision. Tests and SREs filter on this to verify which
        // region/endpoint each operation actually went to. Keep the field
        // name (`routing_decision`) and message (`routing decision made`)
        // stable -- `azure_data_cosmos`'s multi-write integration tests grep
        // for them.
        tracing::debug!(routing_decision = %routing, "routing decision made");

        // ── STAGE 2b: Pre-attempt hedging dispatch ─────────────────────
        // HEDGING_SPEC.md §6.1 — on the **first** attempt of a
        // hedge-eligible operation, race the primary against the threshold
        // timer from t=0 rather than running it to completion and
        // post-classifying. This is the spec's timer-driven
        // speculative-read model: a slow-but-eventually-successful primary
        // still loses to a fast alternate, and a successful primary that
        // finishes pre-threshold still gets `HedgeDiagnostics::primary_only`
        // attached (§10.1 attachment contract).
        //
        // Falls through to STAGE 3 (sequential execute-then-classify) on:
        //
        // * **Subsequent retries** (`failover_retry_count > 0` or
        //   `session_token_retry_count > 0`). A retry has already consumed
        //   region budget and the next attempt is to a different region;
        //   re-racing it makes no sense. The post-attempt
        //   `maybe_upgrade_to_hedge` at STAGE 5b is the safety net for the
        //   rare "first attempt non-eligible, later retry became eligible"
        //   case.
        // * **Non-hedge-eligible operations** — writes, single-region
        //   accounts, operations whose user `ExcludeRegions` leaves < 2
        //   applicable read endpoints, env-disabled hedging, or per-op
        //   `AvailabilityStrategy::Disabled`. All gated by
        //   [`evaluate_hedge_eligibility`].
        if retry_state.failover_retry_count == 0 && retry_state.session_token_retry_count == 0 {
            if let Some(upgrade) = evaluate_hedge_eligibility(
                operation,
                options,
                &location.account,
                &routing,
                deadline.map(|d| d.saturating_duration_since(Instant::now())),
            ) {
                let attempt_ctx = AttemptContext {
                    operation,
                    custom_headers,
                    transport,
                    account_endpoint,
                    credential,
                    user_agent,
                    activity_id,
                    pipeline_type,
                    transport_security,
                    session_manager,
                    session_consistency_active,
                    options,
                    throughput_control,
                    deadline,
                    can_use_multiple_write_locations: retry_state.can_use_multiple_write_locations,
                    // First attempt: no 1002 has been observed yet, so the
                    // per-state latch is `false`. The shared
                    // `Arc<AtomicBool>` is constructed inside
                    // `execute_hedged` only if the threshold elapses and a
                    // secondary spawns, preserving the §6.5 #3
                    // zero-overhead happy path.
                    hub_region_processing_only_initial: retry_state.hub_region_processing_only,
                    // First attempt: no response has been captured yet, so
                    // PK range ID is unknown. PPCB feedback is correctly
                    // gated to no-op on `None` (`record_hedge_outcome`).
                    partition_key_range_id: retry_state.partition_key_range_id.clone(),
                    hedge_outcome_recorder: location_state_store,
                };
                return execute_hedged(
                    &attempt_ctx,
                    &routing,
                    &upgrade.secondary_routing,
                    upgrade.threshold,
                    upgrade.strategy_config,
                    diagnostics,
                )
                .await;
            }
        }

        // ── STAGE 3: Build transport request ───────────────────────────
        let execution_context = compute_execution_context(&retry_state);

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id,
            execution_context,
            deadline,
            resolved_session_token: session_consistency_active
                .then(|| {
                    session_manager.resolve_session_token(
                        operation,
                        operation.request_headers().session_token.as_ref(),
                    )
                })
                .flatten(),
            throughput_control,
        };
        let mut transport_request = build_transport_request(operation, custom_headers, &ctx)?;

        // HUB_REGION_PROCESSING_HEADER_SPEC.md §3 / public-spec §3.4:
        // Emit the `x-ms-cosmos-hub-region-processing-only: True` header
        // when the latch is set. The latch is flipped in
        // `try_handle_read_session_not_available` on the first 1002 of a
        // single-master data-plane operation, and is sticky for the
        // remainder of the operation's transport attempts (AC-1, AC-2).
        apply_hub_region_header(&mut transport_request, &retry_state);

        apply_optional_request_headers(&mut transport_request, operation, options);

        tracing::trace!(
            method = ?transport_request.method,
            url = %transport_request.url,
            "transport request created");

        let selected_transport = match pipeline_type {
            PipelineType::DataPlane => {
                transport.get_dataplane_transport(account_endpoint, routing.transport_mode)?
            }
            PipelineType::Metadata => transport.get_metadata_transport(account_endpoint)?,
        };

        // ── STAGE 4: Execute via transport pipeline ────────────────────

        let result = execute_transport_pipeline(
            transport_request,
            &TransportPipelineContext {
                transport: &selected_transport,
                allow_sent_transport_retry: operation.is_read_only() || operation.is_idempotent(),
                credential,
                user_agent,
                pipeline_type,
                transport_security,
                endpoint_key: routing.endpoint.endpoint_key(),
            },
            &mut diagnostics,
        )
        .await;

        // Capture partition key range ID from response headers (first time only).
        if retry_state.partition_key_range_id.is_none() {
            if let Some(headers) = result.cosmos_headers() {
                if let Some(pk_range_id) = headers.partition_key_range_id.as_deref() {
                    retry_state.partition_key_range_id =
                        Some(PartitionKeyRangeId::from(pk_range_id.to_owned()));
                }
            }
        }

        // ── STAGE 4b: Capture session token ─────────────────────────────
        // Capture session tokens from both successful and certain error
        // responses (409 Conflict, 412 Precondition Failed, 404 non-1002).
        // The server advances the session token even on these errors, so
        // not capturing would break read-your-writes guarantees.
        //
        // This runs BEFORE evaluate_transport_result so that tokens are
        // captured regardless of whether the response maps to Complete,
        // Abort, or a retry action. 409/412 map to Abort, and the Abort
        // variant does not carry headers — capturing after evaluation
        // would silently drop tokens from those responses.
        if session_consistency_active {
            if let Some(cosmos_headers) = result.cosmos_headers() {
                if should_capture_session_token_from_status(
                    cosmos_headers.substatus.as_ref(),
                    &result.outcome,
                ) {
                    session_manager.capture_session_token(operation, cosmos_headers);
                }
            }
        }

        // ── STAGE 5: Evaluate result → action ──────────────────────────
        let (action, effects) =
            evaluate_transport_result(operation, &routing.endpoint, result, &retry_state);

        // ── STAGE 5b: Optional hedging upgrade ─────────────────────────
        // HEDGING_SPEC.md §6.1: when the just-classified action would have
        // advanced to a different region anyway (FailoverRetry / SessionRetry)
        // and the operation is eligible for cross-region hedging (§5.1) with
        // a strategy resolved (§11.3.1), replace it with `OperationAction::Hedge`
        // so STAGE 7 races primary and secondary in parallel via
        // `execute_hedged()`. If no upgrade applies the original action passes
        // through unchanged — preserving today's sequential-failover semantics
        // for non-hedgeable operations, single-region accounts, env-disabled
        // hedging, and explicit `AvailabilityStrategy::Disabled`.
        let action = maybe_upgrade_to_hedge(
            action,
            operation,
            options,
            &location.account,
            &routing,
            deadline.map(|d| d.saturating_duration_since(Instant::now())),
        );

        // ── STAGE 6: Apply location effects ────────────────────────────
        // Single-master write effects are deferred into
        // `retry_state.pending_write_effects` instead of being applied
        // immediately. They are flushed only when the write definitively
        // reaches a region (Complete, or Abort with a region-confirming
        // status such as 409 Conflict). This prevents transient retry
        // failures from polluting the routing state with unverified
        // partition or endpoint outages — critical for PPAF on single-master
        // accounts where prematurely marking the only known write region
        // unavailable would force an unnecessary cross-region failover.
        // Read-path and multi-master write effects are applied immediately
        // so PPCB counters can drive threshold-based failover.
        let (immediate_effects, deferred_effects) = partition_effects_for_deferral(
            operation.is_read_only(),
            retry_state.can_use_multiple_write_locations,
            retry_state.ppaf_write_retry_allowed,
            effects,
        );
        retry_state.pending_write_effects.extend(deferred_effects);
        location_state_store.apply(&immediate_effects).await;

        // ── STAGE 7: Act on the control-flow decision ──────────────────
        match action {
            OperationAction::Complete(result) => {
                // Flush any deferred write-path effects now that the write
                // has definitively succeeded. The current region is proven
                // healthy, so the previously-failed regions can be safely
                // marked unavailable for this partition (and endpoint, when
                // PPAF is active).
                flush_pending_write_effects(&mut retry_state, location_state_store).await;

                // If a PPCB probe request succeeded, remove the ProbeCandidate entry.
                try_cleanup_probe_candidate(&retry_state, location_state_store);

                return build_cosmos_response(result, diagnostics);
            }
            OperationAction::FailoverRetry { new_state, delay } => {
                tracing::debug!(
                    activity_id = %activity_id,
                    failover_attempt = new_state.failover_retry_count,
                    delay = ?delay,
                    effects = ?immediate_effects,
                    deferred_effects = retry_state.pending_write_effects.len(),
                    "failover retry triggered",
                );
                apply_failover_delay(delay).await;
                advance_to_next_attempt(
                    &mut retry_state,
                    new_state,
                    location_state_store,
                    operation.is_read_only(),
                );
                enforce_deadline_or_timeout(deadline, options, &mut diagnostics)?;
            }
            OperationAction::SessionRetry { new_state } => {
                // Retry to a different region — the 404/1002 is likely a
                // transient replica lag. Session tokens are intentionally
                // preserved; clearing them would break read-your-writes
                // guarantees. Container-recreation (RID change) handling
                // will be addressed via deterministic RID comparison in
                // a future change.
                advance_to_next_attempt(
                    &mut retry_state,
                    new_state,
                    location_state_store,
                    operation.is_read_only(),
                );
                enforce_deadline_or_timeout(deadline, options, &mut diagnostics)?;
            }
            OperationAction::Abort { error, status } => {
                // Flush deferred write-path effects if the abort status
                // confirms the region processed the request (e.g., 409
                // Conflict, 412 Precondition Failed). On non-confirming
                // aborts (503/429-3092/410/408/403-3/transport error/deadline)
                // the buffered effects are discarded — we never proved any
                // region was actually healthy, so polluting routing state
                // would be wrong.
                let confirming = status.as_ref().is_some_and(is_region_confirming_status);
                if confirming {
                    flush_pending_write_effects(&mut retry_state, location_state_store).await;
                } else {
                    retry_state.pending_write_effects.clear();
                }

                tracing::error!(
                    activity_id = %activity_id,
                    status = ?status,
                    error = %error,
                    operation_type = ?operation.operation_type(),
                    resource_type = ?operation.resource_type(),
                    is_read_only = operation.is_read_only(),
                    is_idempotent = operation.is_idempotent(),
                    failover_retries = retry_state.failover_retry_count,
                    session_retries = retry_state.session_token_retry_count,
                    pk_range_id = ?retry_state.partition_key_range_id,
                    "operation aborted",
                );
                if let Some(cosmos_status) = status {
                    diagnostics.set_operation_status(
                        cosmos_status.status_code(),
                        cosmos_status.sub_status(),
                    );
                }
                return Err(error);
            }
            OperationAction::Hedge {
                secondary_routing,
                secondary_excluded_regions: _,
                threshold,
                strategy_config,
            } => {
                // HEDGING_SPEC.md §6.1 / §6.4 — race the primary attempt
                // against a single cross-region secondary via
                // `execute_hedged`. Terminal per the Part 4 design: the
                // result of the race is the final operation result and
                // we do not re-enter the operation loop, even when both
                // sides fail transient (their errors are surfaced as the
                // operation error). Per spec §6.5 #9 the `Hedge` variant
                // is the only entry point to `execute_hedged`.
                //
                // We move `diagnostics` into `execute_hedged` because the
                // function takes ownership of the parent builder to merge
                // each hedge attempt's sub-builder back into it and finalize
                // the response (mirroring the `Complete` arm's
                // `build_cosmos_response(result, diagnostics)` shape).
                let attempt_ctx = AttemptContext {
                    operation,
                    custom_headers,
                    transport,
                    account_endpoint,
                    credential,
                    user_agent,
                    activity_id,
                    pipeline_type,
                    transport_security,
                    session_manager,
                    session_consistency_active,
                    options,
                    throughput_control,
                    deadline,
                    can_use_multiple_write_locations: retry_state.can_use_multiple_write_locations,
                    hub_region_processing_only_initial: retry_state.hub_region_processing_only,
                    partition_key_range_id: retry_state.partition_key_range_id.clone(),
                    hedge_outcome_recorder: location_state_store,
                };
                return execute_hedged(
                    &attempt_ctx,
                    &routing,
                    &secondary_routing,
                    threshold,
                    strategy_config,
                    diagnostics,
                )
                .await;
            }
        }
    }
}

/// Drains `retry_state.pending_write_effects` and applies the subset that is
/// not already reflected in the current routing state.
///
/// Called when a write definitively reaches a region — either a successful
/// completion (HTTP 2xx) or a region-confirming abort (e.g., 409 Conflict).
/// At that point the previously-failed regions are proven to have been the
/// unhealthy ones, and their accumulated effects (partition marks for all
/// writes; endpoint marks too for PPAF on single-master) should be applied.
///
/// Once an override has been established for a partition (PPAF entry exists
/// with the failed region rotated past) or an endpoint has already been
/// marked unavailable, re-applying the same effect is a no-op semantically
/// but still costs a CAS-loop clone of `PartitionEndpointState` /
/// `AccountEndpointState`. We skip such effects here so the steady-state
/// cost of a successful write that retried converges to a single snapshot
/// read.
async fn flush_pending_write_effects(
    retry_state: &mut OperationRetryState,
    location_state_store: &LocationStateStore,
) {
    if retry_state.pending_write_effects.is_empty() {
        return;
    }
    let pending = std::mem::take(&mut retry_state.pending_write_effects);
    let snapshot = location_state_store.snapshot();
    let effects: Vec<LocationEffect> = pending
        .into_iter()
        .filter(|effect| !is_effect_already_applied(effect, &snapshot))
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
fn is_effect_already_applied(effect: &LocationEffect, snapshot: &LocationSnapshot) -> bool {
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
            let already_moved = |entry: &crate::driver::routing::partition_endpoint_state::PartitionFailoverEntry| -> bool {
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
    }
}

/// Resolves the endpoint for this attempt.
///
/// Uses `LocationSnapshot` and `AccountEndpointState` to select the best
/// available endpoint, respecting excluded regions and unavailability TTL.
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    prefer_gateway20: bool,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    let account = location.account.as_ref();
    let read_only = operation.is_read_only();
    // Build an in-flight skip set from effects deferred during this
    // operation. On retries this skips regions we've already failed against
    // so the next attempt picks a different region. Both kinds of deferred
    // effects contribute regions:
    //   * `MarkPartitionUnavailable` — partition-level failure with a region
    //   * `MarkEndpointUnavailable`  — endpoint-level failure (PPAF SM only)
    //
    // For PPAF writes on single-master accounts, `primary` is the full read
    // endpoint list (all regions in preferred order) — see
    // `preferred_endpoints_for_attempt`. That allows write region discovery:
    // when the original write region fails, the next attempt naturally rolls
    // over to the next region in the read list rather than retrying the same
    // (potentially failed-over) write region.
    let in_flight_failed: Vec<&Region> = if !read_only {
        retry_state
            .pending_write_effects
            .iter()
            .filter_map(|e| match e {
                LocationEffect::MarkPartitionUnavailable(p) => p.region.as_ref(),
                LocationEffect::MarkEndpointUnavailable { endpoint, .. } => endpoint.region(),
                LocationEffect::RefreshAccountProperties => None,
            })
            .collect()
    } else {
        Vec::new()
    };

    let primary = preferred_endpoints_for_attempt(account, retry_state, read_only);
    let selected = try_select_endpoint(
        operation,
        retry_state,
        account,
        primary,
        &in_flight_failed,
        endpoint_unavailability_ttl,
    );

    // If every region in the primary list has been attempted (or excluded),
    // fall back to the standard selection ignoring the in-flight skip set so
    // we always have an endpoint to call.
    let selected = selected.unwrap_or_else(|| {
        try_select_endpoint(
            operation,
            retry_state,
            account,
            primary,
            &[],
            endpoint_unavailability_ttl,
        )
        .unwrap_or_else(|| account.default_endpoint.clone())
    });
    let use_gateway20 = selected.uses_gateway20(prefer_gateway20);
    let transport_mode = if use_gateway20 {
        TransportMode::Gateway20
    } else {
        TransportMode::Gateway
    };

    // Check for partition-level override (PPAF/PPCB).
    if let Some(pk_range_id) = &retry_state.partition_key_range_id {
        let partitions = location.partitions.as_ref();
        let is_read = operation.is_read_only();
        let is_partitioned = operation
            .resource_type()
            .is_partitioned(operation.operation_type());

        // Helper: build a RoutingDecision from a partition override endpoint.
        let make_partition_routing = |ep: CosmosEndpoint| -> RoutingDecision {
            let ep_use_gw20 = ep.uses_gateway20(prefer_gateway20);
            RoutingDecision {
                selected_url: ep.selected_url(ep_use_gw20).clone(),
                transport_mode: if ep_use_gw20 {
                    TransportMode::Gateway20
                } else {
                    TransportMode::Gateway
                },
                endpoint: ep,
            }
        };

        // Helper: returns true when the override endpoint's region has
        // already been attempted in the current operation (i.e., its mark
        // is sitting in the deferred-effect buffer). This bridges the gap
        // between a stale PPAF/PPCB override entry and the in-flight skip
        // set: PPAF defers `MarkPartitionUnavailable` until success, so
        // `entry.current_endpoint` may still point at the freshly-failed
        // region across retries within the same operation. Skipping the
        // override here lets the primary selection (which already consulted
        // `in_flight_failed`) pick a different region for the next attempt.
        let override_region_already_failed = |ep: &CosmosEndpoint| -> bool {
            ep.region().is_some_and(|r| in_flight_failed.contains(&r))
        };

        if is_eligible_for_ppcb(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.circuit_breaker_overrides.get(pk_range_id) {
                if entry.health_status == HealthStatus::ProbeCandidate
                    && !override_region_already_failed(&entry.first_failed_endpoint)
                {
                    // Route probe request to the original (first failed) endpoint.
                    return make_partition_routing(entry.first_failed_endpoint.clone());
                }
                if can_circuit_breaker_trigger_failover(entry, is_read, &partitions.config)
                    && !override_region_already_failed(&entry.current_endpoint)
                {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        } else if is_eligible_for_ppaf(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.failover_overrides.get(pk_range_id) {
                // PPAF overrides do not use probe-based failback (no ProbeCandidate
                // handling). The override persists until the backend signals a change.
                //
                // Skip the override when its `current_endpoint` is in the in-flight
                // skip set: PPAF defers partition marks until success, so the
                // persistent entry can lag the actual per-attempt failure history.
                // Falling through to `selected` lets the next attempt cross-region
                // retry rather than re-hitting the same failed override region.
                if !override_region_already_failed(&entry.current_endpoint) {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        }
    }

    RoutingDecision {
        selected_url: selected.selected_url(use_gateway20).clone(),
        endpoint: selected,
        transport_mode,
    }
}

fn preferred_endpoints_for_attempt<'a>(
    account: &'a AccountEndpointState,
    retry_state: &OperationRetryState,
    read_only: bool,
) -> &'a [CosmosEndpoint] {
    if read_only && retry_state.route_reads_to_write_endpoints() {
        &account.preferred_write_endpoints
    } else if !read_only && retry_state.ppaf_write_retry_allowed {
        // PPAF on single-master accounts: writes iterate over the full read
        // endpoint list (all regions in preferred order) so the client can
        // probe other regions to discover the current write region after a
        // backend-side failover. preferred_write_endpoints contains only the
        // currently known write region, which would prevent cross-region
        // write retry.
        &account.preferred_read_endpoints
    } else {
        account.preferred_endpoints(read_only)
    }
}

/// Walks `endpoints` starting at the retry-state base index and returns the
/// first endpoint that is not excluded, not in `skip_regions`, and currently
/// available. Falls back to the first unavailable-but-not-skipped endpoint
/// (mirrors SDK pattern: [available] → [unavailable regional]).
///
/// Returns `None` only when every endpoint is either excluded or in the
/// skip set — the caller decides the next fallback list.
fn try_select_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    account: &AccountEndpointState,
    endpoints: &[CosmosEndpoint],
    skip_regions: &[&Region],
    endpoint_unavailability_ttl: Duration,
) -> Option<CosmosEndpoint> {
    if endpoints.is_empty() {
        return None;
    }
    let base_index = if retry_state.location.is_current(account.generation) {
        retry_state.location.index()
    } else {
        0
    };
    let now = Instant::now();
    let len = endpoints.len();
    let mut first_unavailable = None;
    for i in 0..len {
        let candidate = &endpoints[(base_index + i) % len];
        let candidate_region = candidate.region();
        let excluded =
            candidate_region.is_some_and(|r| retry_state.excluded_regions.iter().any(|e| e == r));
        if excluded {
            continue;
        }
        let in_skip_set = candidate_region.is_some_and(|r| skip_regions.contains(&r));
        if in_skip_set {
            continue;
        }
        if endpoint_is_available(
            operation,
            candidate,
            account,
            now,
            endpoint_unavailability_ttl,
        ) {
            return Some(candidate.clone());
        }
        if first_unavailable.is_none() {
            first_unavailable = Some(candidate.clone());
        }
    }
    first_unavailable
}

fn endpoint_is_available(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    account: &AccountEndpointState,
    now: Instant,
    endpoint_unavailability_ttl: Duration,
) -> bool {
    !account
        .unavailable_endpoints
        .get(endpoint.url())
        .is_some_and(|(marked_at, reason)| {
            if operation.is_read_only()
                && matches!(
                    reason,
                    crate::driver::routing::UnavailableReason::WriteForbidden
                )
            {
                return false;
            }

            now.saturating_duration_since(*marked_at) < endpoint_unavailability_ttl
        })
}

/// Parameters resolved per-attempt for building a transport request.
///
/// Groups per-attempt state that varies across retries and failovers,
/// reducing the number of arguments passed to `build_transport_request`.
struct TransportRequestContext<'a> {
    routing: &'a RoutingDecision,
    activity_id: &'a ActivityId,
    execution_context: ExecutionContext,
    deadline: Option<Instant>,
    resolved_session_token: Option<SessionToken>,
    throughput_control: Option<&'a ThroughputControlGroupSnapshot>,
}

/// Builds a `TransportRequest` from the operation and routing decision.
///
/// If `resolved_session_token` is provided, it is added to the request headers.
fn build_transport_request(
    operation: &CosmosOperation,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    ctx: &TransportRequestContext<'_>,
) -> azure_core::Result<TransportRequest> {
    let paths = operation.compute_resource_paths();
    let url = {
        let mut base = ctx.routing.selected_url.clone();
        let request_path = paths.request_path();
        let normalized = if request_path.starts_with('/') {
            request_path.to_string()
        } else if request_path.is_empty() {
            String::new()
        } else {
            format!("/{}", request_path)
        };
        base.set_path(&normalized);
        base
    };

    let method = operation.operation_type().http_method();
    let resource_type = operation.resource_type();
    // Move `paths` into AuthorizationContext so the signing link is a zero-copy
    // sub-slice of the path buffer — no additional string allocation needed.
    let auth_context = AuthorizationContext::from_paths(method, resource_type, paths);

    // Build headers from the operation.
    // Custom headers are inserted first so that SDK-set headers below always
    // take precedence on conflicts (matching the SDK's ItemOptions::apply_headers
    // pattern where custom headers are added before SDK headers).
    let mut headers = azure_core::http::headers::Headers::new();
    if let Some(custom) = custom_headers {
        for (name, value) in custom {
            headers.insert(name.clone(), value.clone());
        }
    }
    operation.request_headers().write_to_headers(&mut headers);

    // Add activity ID if not already set by the operation
    if operation.request_headers().activity_id.is_none() {
        headers.insert(
            HeaderName::from_static("x-ms-activity-id"),
            HeaderValue::from(ctx.activity_id.as_str().to_owned()),
        );
    }

    // Add partition key headers
    if let Some(pk) = operation.partition_key() {
        let pk_headers = pk.as_headers()?;
        for (name, value) in pk_headers {
            headers.insert(name, value);
        }
    }

    // Cosmos DB uses POST for both create and upsert; the service
    // distinguishes them via this header.
    if operation.operation_type() == OperationType::Upsert {
        headers.insert(
            HeaderName::from_static(request_header_names::IS_UPSERT),
            HeaderValue::from_static("true"),
        );
    }

    // Cosmos DB queries are POST requests with a JSON body of shape
    // `{"query": "...", "parameters": [...]}`. The service requires both the
    // IsQuery flag and the query+json content type. Emitting these here
    // (driven by OperationType::Query) means SDK callers and tests never need
    // to put these well-known headers into `custom_headers`.
    if operation.operation_type() == OperationType::Query {
        headers.insert(
            HeaderName::from_static(request_header_names::IS_QUERY),
            HeaderValue::from_static("True"),
        );
        headers.insert(
            azure_core::http::headers::CONTENT_TYPE,
            HeaderValue::from_static(request_header_names::QUERY_CONTENT_TYPE),
        );
    }

    // Cosmos DB uses POST for batch (same endpoint as create/upsert);
    // the service requires these headers to process the request as a batch.
    if operation.operation_type() == OperationType::Batch {
        headers.insert(
            HeaderName::from_static(request_header_names::IS_BATCH_REQUEST),
            HeaderValue::from_static("True"),
        );
        headers.insert(
            HeaderName::from_static(request_header_names::BATCH_ATOMIC),
            HeaderValue::from_static("True"),
        );
        headers.insert(
            HeaderName::from_static(request_header_names::BATCH_CONTINUE_ON_ERROR),
            HeaderValue::from_static("False"),
        );
    }

    // Add operation type header for fault injection rule matching
    #[cfg(feature = "fault_injection")]
    {
        if let Some(fault_op) =
            crate::fault_injection::FaultOperationType::from_operation_and_resource(
                &operation.operation_type(),
                &operation.resource_type(),
            )
        {
            use crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION;
            headers.insert(FAULT_INJECTION_OPERATION, fault_op.as_str());
        }
    }

    // Add resolved session token
    if let Some(token) = &ctx.resolved_session_token {
        headers.insert(
            request_header_names::SESSION_TOKEN,
            HeaderValue::from(token.as_str().to_owned()),
        );
    }

    // Add throughput control headers from the resolved group
    if let Some(group) = ctx.throughput_control {
        if let Some(priority) = group.priority_level() {
            headers.insert(
                request_header_names::PRIORITY_LEVEL,
                HeaderValue::from(priority.as_str().to_owned()),
            );
        }
        if let Some(bucket) = group.throughput_bucket() {
            headers.insert(
                request_header_names::THROUGHPUT_BUCKET,
                HeaderValue::from(bucket.to_string()),
            );
        }
    }

    Ok(TransportRequest {
        method,
        endpoint: ctx.routing.endpoint.clone(),
        url,
        headers,
        body: operation.body().map(azure_core::Bytes::copy_from_slice),
        auth_context,
        execution_context: ctx.execution_context,
        deadline: ctx.deadline,
    })
}

/// Builds a `CosmosResponse` from a successful `TransportResult`.
fn build_cosmos_response(
    result: Box<TransportResult>,
    mut diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    match result.outcome {
        TransportOutcome::Success {
            status,
            cosmos_headers,
            body,
        } => {
            diagnostics.set_operation_status(status.status_code(), status.sub_status());

            let diagnostics_ctx = Arc::new(diagnostics.complete());

            Ok(CosmosResponse::new(
                body,
                cosmos_headers,
                status,
                diagnostics_ctx,
            ))
        }
        _ => {
            // This should only be called with a Complete(Success) result
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "build_cosmos_response called with non-success result",
            ))
        }
    }
}

/// Determines whether a session token should be captured from this response.
///
/// Follows Java/.NET patterns: capture on success (2xx), and on error responses
/// where the server still advanced the session token:
/// - 409 Conflict
/// - 412 Precondition Failed
/// - 404 when substatus is NOT ReadSessionNotAvailable (1002)
///
/// Does NOT capture on 404/1002 (that's the trigger for session retry/clear).
fn should_capture_session_token_from_status(
    substatus: Option<&SubStatusCode>,
    outcome: &TransportOutcome,
) -> bool {
    match outcome {
        TransportOutcome::Success { .. } => true,
        TransportOutcome::HttpError { status, .. } => {
            let code = status.status_code();
            if code == azure_core::http::StatusCode::Conflict
                || code == azure_core::http::StatusCode::PreconditionFailed
            {
                return true;
            }
            if code == azure_core::http::StatusCode::NotFound {
                // Capture on 404 unless substatus is ReadSessionNotAvailable (1002)
                return substatus != Some(&SubStatusCode::READ_SESSION_NOT_AVAILABLE);
            }
            false
        }
        _ => false,
    }
}

// ── Pipeline stage helpers ────────────────────────────────────────────
//
// These pure (or narrowly-scoped) helpers carry the per-stage logic for
// `execute_operation_pipeline` so the main loop body stays readable and
// each stage can be unit-tested in isolation.

/// Maps the current retry counts to the `ExecutionContext` value that the
/// transport pipeline expects for diagnostics annotation.
///
/// - First attempt (no failover, no session retry) → `Initial`
/// - Any session retry in progress → `Retry`
/// - Otherwise (a failover retry) → `RegionFailover`
///
/// Session-retry takes precedence over failover-retry because in the rare
/// case where both counters are non-zero, the most recent advance was the
/// session retry (failover counters are not reset on session retry, but the
/// session retry happens later in the loop).
fn compute_execution_context(retry_state: &OperationRetryState) -> ExecutionContext {
    if retry_state.failover_retry_count == 0 && retry_state.session_token_retry_count == 0 {
        ExecutionContext::Initial
    } else if retry_state.session_token_retry_count > 0 {
        ExecutionContext::Retry
    } else {
        ExecutionContext::RegionFailover
    }
}

/// Conditionally emits the `x-ms-cosmos-hub-region-processing-only: True`
/// header on the outbound transport request when the latch on
/// `retry_state` is set.
///
/// Extracted as a free function (rather than left inline at the call site
/// in `execute_operation_pipeline`) so that the emission rule can be
/// exercised by unit tests without spinning up the full pipeline. The
/// production call site is the loop iteration after `build_transport_request`
/// and before `apply_optional_request_headers`.
///
/// HUB_REGION_PROCESSING_HEADER_SPEC.md §3 / public-spec §3.4. See
/// `try_handle_read_session_not_available` for the latch trigger.
///
/// **Hedging coordination (future).** Per HEDGING_SPEC.md §9.5, the
/// emission decision MUST OR in a `shared_hub_region_latch:
/// Option<Arc<AtomicBool>>` field added to `OperationRetryState`, read
/// with `Acquire` ordering. That field is set from `build_session_retry_state`
/// the first time any hedge in the same `execute_with_hedging()`
/// fan-out observes 1002 and is what makes the other (still-latch-clean)
/// hedges immediately emit the header — the Rust counterpart of .NET v3's
/// `CrossRegionAvailabilityContext.ShouldAddHubRegionProcessingOnlyHeader`
/// from azure-cosmos-dotnet-v3#5815.
fn apply_hub_region_header(
    transport_request: &mut TransportRequest,
    retry_state: &OperationRetryState,
) {
    if should_emit_hub_region_header(
        retry_state.hub_region_processing_only,
        retry_state.shared_hub_region_latch.as_ref(),
    ) {
        transport_request.headers.insert(
            HeaderName::from_static(request_header_names::HUB_REGION_PROCESSING_ONLY),
            HeaderValue::from_static("True"),
        );
    }
}

/// Returns `true` when the `x-ms-cosmos-hub-region-processing-only`
/// header should be emitted on a transport request, per spec
/// HUB_REGION_PROCESSING_HEADER_SPEC.md §3 / HEDGING_SPEC.md §9.6.
///
/// Emission is the OR of the per-state latch (set by
/// `build_session_retry_state` on the first 1002 of a single-master
/// data-plane operation) and the shared latch (set by any sibling
/// hedge inside the same `execute_hedged` fan-out). The shared latch
/// is read with `Acquire` ordering, pairing with the `Release` store
/// in `build_session_retry_state`. When the shared latch is absent
/// (`None` — the non-hedged pipeline and the zero-overhead happy
/// path), the rule collapses to the pre-PR-#5815 per-state behavior.
fn should_emit_hub_region_header(
    per_state_latched: bool,
    shared_latch: Option<&Arc<AtomicBool>>,
) -> bool {
    per_state_latched || shared_latch.is_some_and(|s| s.load(Ordering::Acquire))
}

/// Returns `true` when [`execute_hedged`] should construct an
/// `Arc<AtomicBool>` shared hub-region-processing-only latch for the
/// hedge fan-out, per spec
/// [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md) §9.6.3.
///
/// The latch only matters when both:
///
/// 1. **Data-plane scope** — metadata operations are scoped out per
///    `HUB_REGION_PROCESSING_HEADER_SPEC.md` §1.5 / AC-8, mirroring the
///    per-state latch's `is_dataplane` trigger gate.
/// 2. **Single-master account** — multi-master accounts never emit the
///    `x-ms-cosmos-hub-region-processing-only` header
///    (`HUB_REGION_PROCESSING_HEADER_SPEC.md` AC-4); a latch on a
///    multi-master hedge would be inert and would waste an `Arc`
///    allocation.
///
/// Construction at the call site is further gated on the threshold
/// having elapsed, so the §6.5 #3 zero-overhead happy path (primary
/// wins pre-threshold) is preserved.
fn should_build_shared_hub_region_latch(
    pipeline_type: PipelineType,
    can_use_multiple_write_locations: bool,
) -> bool {
    pipeline_type.is_data_plane() && !can_use_multiple_write_locations
}

/// Applies operation-options-driven request headers that are only known
/// after the request has been built by `build_transport_request`.
///
/// Two concerns are layered here, in this order:
///
/// 1. **content-response-on-write**: For non-read operations whose
///    `content_response_on_write` is `None` (default) or `Disabled`,
///    inject `Prefer: return=minimal` so the service suppresses the
///    response body. Reads always need the body and are unaffected.
/// 2. **custom_headers**: Pass-through of caller-supplied headers, but
///    only when no SDK-set header with the same name already exists —
///    SDK headers always take precedence.
fn apply_optional_request_headers(
    transport_request: &mut TransportRequest,
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
) {
    if !operation.operation_type().is_read_only()
        && !matches!(
            options.content_response_on_write(),
            Some(&crate::options::ContentResponseOnWrite::Enabled)
        )
    {
        transport_request.headers.insert(
            request_header_names::PREFER,
            HeaderValue::from_static("return=minimal"),
        );
    }

    if let Some(custom_headers) = options.custom_headers() {
        for (name, value) in custom_headers {
            if !transport_request.headers.iter().any(|(n, _)| n == name) {
                transport_request
                    .headers
                    .insert(name.clone(), value.clone());
            }
        }
    }
}

/// Sleeps for the failover-retry delay if one was requested.
///
/// Treats `None` and zero-duration delays as no-ops so callers don't have
/// to repeat that guard themselves. Conversion to `azure_core::time::Duration`
/// is performed once; if it fails (e.g., overflow) the sleep is silently
/// skipped because a too-large delay is no worse than no delay at all.
async fn apply_failover_delay(delay: Option<Duration>) {
    let Some(delay) = delay else {
        return;
    };
    if delay.is_zero() {
        return;
    }
    if let Ok(duration) = azure_core::time::Duration::try_from(delay) {
        azure_core::sleep(duration).await;
    }
}

/// Advances `retry_state` to a fresh attempt against an updated location
/// snapshot, preserving any deferred write-path effects.
///
/// `evaluate_transport_result` cloned `retry_state` BEFORE this attempt's
/// deferred effects were appended, so `new_state.pending_write_effects` is
/// the pre-extend snapshot. Without explicit transfer, every retry would
/// start with an empty `pending_write_effects` — which means
/// `in_flight_failed` (built from those effects in `resolve_endpoint`) would
/// be empty, so the next attempt would not skip the region that just failed
/// and may pick the same region again (or, when all regions are unavailable,
/// fall back to the global default endpoint).
fn advance_to_next_attempt(
    retry_state: &mut OperationRetryState,
    new_state: OperationRetryState,
    location_state_store: &LocationStateStore,
    is_read_only: bool,
) {
    let next_location = location_state_store.snapshot();
    let endpoints_len =
        preferred_endpoints_for_attempt(next_location.account.as_ref(), &new_state, is_read_only)
            .len();
    let pending = std::mem::take(&mut retry_state.pending_write_effects);
    *retry_state = new_state.advance_location(endpoints_len, next_location.account.generation);
    retry_state.pending_write_effects = pending;
}

/// Returns `Err` with a `RequestTimeout`-coded error if the end-to-end
/// deadline has been reached, otherwise `Ok(())`.
///
/// On timeout, the diagnostics builder is updated with
/// `RequestTimeout` + `CLIENT_OPERATION_TIMEOUT` so downstream telemetry
/// can distinguish a client-side end-to-end timeout from a service 408.
fn enforce_deadline_or_timeout(
    deadline: Option<Instant>,
    options: &OperationOptionsView<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> azure_core::Result<()> {
    let Some(d) = deadline else {
        return Ok(());
    };
    if Instant::now() < d {
        return Ok(());
    }

    let timeout_duration = options
        .end_to_end_latency_policy()
        .map(|p| p.timeout())
        .unwrap_or_default();

    diagnostics.set_operation_status(
        azure_core::http::StatusCode::RequestTimeout,
        Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
    );
    Err(azure_core::Error::new(
        azure_core::error::ErrorKind::Other,
        format!("end-to-end operation timeout exceeded ({timeout_duration:?})"),
    ))
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
fn try_cleanup_probe_candidate(
    retry_state: &OperationRetryState,
    location_state_store: &LocationStateStore,
) {
    let Some(pk_range_id) = &retry_state.partition_key_range_id else {
        return;
    };

    let snapshot = location_state_store.snapshot();
    let needs_cleanup = snapshot
        .partitions
        .circuit_breaker_overrides
        .get(pk_range_id.as_str())
        .is_some_and(|e| e.health_status == HealthStatus::ProbeCandidate);
    if !needs_cleanup {
        return;
    }

    location_state_store.apply_partition(|current| {
        let is_probe = current
            .circuit_breaker_overrides
            .get(pk_range_id.as_str())
            .is_some_and(|e| e.health_status == HealthStatus::ProbeCandidate);
        if is_probe {
            remove_probe_succeeded_entry(current, pk_range_id)
        } else {
            current.clone()
        }
    });
}

// ── Hedging dispatch (Part 4b) ────────────────────────────────────────
//
// `maybe_upgrade_to_hedge` + `AttemptContext` + `perform_single_attempt`
// + `execute_hedged` together implement the cross-region hedging race
// described in [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md)
// §6.1–§6.5. They sit between the main `execute_operation_pipeline`
// loop's STAGE 5 (evaluator) and STAGE 7 (dispatch):
//
//  1. After `evaluate_transport_result` returns a per-attempt action,
//     `maybe_upgrade_to_hedge` checks whether the action is a
//     same-pipeline retry (`FailoverRetry` / `SessionRetry`) and the
//     operation is hedge-eligible per `hedging_eligibility`. If so the
//     action is rewritten to `OperationAction::Hedge`.
//  2. The STAGE 7 `Hedge` arm bundles the per-operation shared state
//     into `AttemptContext` and calls `execute_hedged`.
//  3. `execute_hedged` fires the primary attempt immediately, races it
//     against the threshold timer (zero-overhead happy path when the
//     primary completes first), then spawns the secondary and races
//     primary vs secondary. The first **final** result wins (`§7.1`
//     `is_final_result`); a transient on either side keeps the other
//     side racing.
//
// Diagnostics flow: each hedge attempt records into its own
// sub-builder cloned from the parent via
// `DiagnosticsContextBuilder::clone_for_hedge_attempt`. After the race
// resolves the surviving sub-builders are merged back into the parent
// via `merge_hedge_attempt`, and the winning side's
// [`HedgeDiagnostics`] is attached via `set_hedge_diagnostics`. Loser
// futures dropped by the race never merge — their request_diagnostics
// are intentionally discarded (matches spec §6.5 #6 "single writer to
// diagnostics" for the per-operation summary).

/// Per-operation context shared across hedge attempts.
///
/// Bundles the read-only references that every per-attempt transport
/// invocation needs so [`execute_hedged`] and [`perform_single_attempt`]
/// can be called without 14-argument call sites. The `'a` lifetime is
/// the outer `execute_operation_pipeline` invocation's lifetime — every
/// field is borrowed from there.
/// Sink for hedge-outcome feedback used by [`execute_hedged`].
///
/// Per spec [`docs/HEDGING_SPEC.md`] §9.5 the outcome of every hedge
/// race feeds back into PPCB so that repeated alternate-region wins on
/// the same `(partition, primary_region)` pair eventually trip the
/// primary partition to `Unhealthy`. The trait exists so that:
///
/// 1. The hedge race in [`execute_hedged`] depends on a minimal sink
///    interface rather than on the full
///    [`LocationStateStore`] surface, and
/// 2. Unit tests can substitute a record-into-`Vec` fake recorder
///    without needing the heavyweight
///    [`LocationStateStore::new`] dependencies.
///
/// Production uses the [`LocationStateStore`] impl which today tracing-
/// logs the events; the real counter / state-transition wiring is
/// deferred to PPCB-owner co-design
/// (`HEDGING_IMPLEMENTATION_PLAN.md` sub-plan 6c).
pub(crate) trait HedgeOutcomeRecorder: Send + Sync {
    /// Record an alternate-region win.
    fn record_consecutive_hedge_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    );

    /// Record a primary-region win (resets the consecutive-hedge-win
    /// counter — spec §9.5 invariant #2).
    fn record_primary_win(&self, partition: &PartitionKeyRangeId, primary_region: Option<&Region>);
}

impl HedgeOutcomeRecorder for LocationStateStore {
    fn record_consecutive_hedge_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    ) {
        LocationStateStore::record_consecutive_hedge_win(self, partition, primary_region);
    }

    fn record_primary_win(&self, partition: &PartitionKeyRangeId, primary_region: Option<&Region>) {
        LocationStateStore::record_primary_win(self, partition, primary_region);
    }
}

/// Dispatches a hedge-race outcome to the [`HedgeOutcomeRecorder`].
///
/// Spec §9.5 keys feedback by `(partition, primary_region)`. When the
/// operation never resolved a partition key range ID (first attempt
/// against an unresolved item — see [`AttemptContext::partition_key_range_id`])
/// the call is a no-op: there is no key to attribute feedback against.
/// The same gating applies to both alternate-wins and primary-wins so
/// the reset (invariant #2) only fires on pairs that could ever
/// accumulate a count in the first place.
fn record_hedge_outcome(
    recorder: &dyn HedgeOutcomeRecorder,
    outcome: HedgeOutcome,
    partition: Option<&PartitionKeyRangeId>,
    primary_region: Option<&Region>,
) {
    let Some(partition) = partition else {
        return;
    };
    match outcome {
        HedgeOutcome::AlternateWin => {
            recorder.record_consecutive_hedge_win(partition, primary_region)
        }
        HedgeOutcome::PrimaryWin => recorder.record_primary_win(partition, primary_region),
    }
}

/// Hedge race outcome reported to the [`HedgeOutcomeRecorder`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HedgeOutcome {
    /// The primary attempt produced the operation result (whether
    /// pre-threshold in the zero-overhead happy path or post-threshold
    /// after racing the secondary).
    PrimaryWin,
    /// The alternate (secondary) attempt produced the operation result.
    AlternateWin,
}

/// Per-operation context shared across hedge attempts.
///
/// Bundles the read-only references that every per-attempt transport
/// invocation needs so [`execute_hedged`] and [`perform_single_attempt`]
/// can be called without 14-argument call sites. The `'a` lifetime is
/// the outer `execute_operation_pipeline` invocation's lifetime — every
/// field is borrowed from there.
struct AttemptContext<'a> {
    operation: &'a CosmosOperation,
    custom_headers: Option<&'a std::collections::HashMap<HeaderName, HeaderValue>>,
    transport: &'a CosmosTransport,
    account_endpoint: &'a AccountEndpoint,
    credential: &'a Credential,
    user_agent: &'a azure_core::http::headers::HeaderValue,
    activity_id: &'a ActivityId,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    session_manager: &'a SessionManager,
    /// Whether session consistency is in effect for this operation
    /// (drives session-token resolve/capture inside the attempt).
    session_consistency_active: bool,
    options: &'a OperationOptionsView<'a>,
    throughput_control: Option<&'a ThroughputControlGroupSnapshot>,
    /// End-to-end deadline (operation timeout) — passed through to each
    /// per-attempt transport invocation.
    deadline: Option<Instant>,
    /// Whether this operation runs against a multi-master account.
    /// Used by [`execute_hedged`] to gate construction of the shared
    /// hub-region-processing-only latch (spec [`docs/HEDGING_SPEC.md`]
    /// §9.6.3 — the latch is only meaningful on single-master
    /// data-plane operations).
    can_use_multiple_write_locations: bool,
    /// Current value of the per-state `hub_region_processing_only`
    /// latch at the moment hedging upgraded from `FailoverRetry` /
    /// `SessionRetry`. Used by [`execute_hedged`] to seed the shared
    /// `Arc<AtomicBool>` so a 1002 already discovered by the main
    /// pipeline before hedging fired carries forward into both hedge
    /// attempts (spec [`docs/HEDGING_SPEC.md`] §9.6.2).
    hub_region_processing_only_initial: bool,
    /// Identifies the physical partition serving this operation. Used by
    /// [`execute_hedged`] to attribute hedge-win signals back to the
    /// `(partition, primary_region)` pair consumed by PPCB feedback
    /// (spec [`docs/HEDGING_SPEC.md`] §9.5). `None` for operations
    /// dispatched before any response captured the partition key range
    /// ID (first attempt of an unresolved item) — in that case no
    /// feedback can be attributed and the recorder calls become no-ops.
    partition_key_range_id: Option<PartitionKeyRangeId>,
    /// Sink for hedge-outcome feedback. Always backed by the real
    /// `LocationStateStore` in production; the indirection is via the
    /// [`HedgeOutcomeRecorder`] trait so unit tests can substitute a
    /// fake without standing up a full store (spec
    /// [`docs/HEDGING_SPEC.md`] §9.5).
    hedge_outcome_recorder: &'a dyn HedgeOutcomeRecorder,
}

/// Result classification used by [`execute_hedged`] to decide whether a
/// completed hedge attempt terminates the race or keeps the other side
/// running.
///
/// `Final` carries the [`TransportResult`] that becomes the operation's
/// outcome (success or final-classified HTTP error per spec §7.1).
/// `Transient` indicates the attempt should be ignored in favor of the
/// other side (5xx / 429 / 408 / 410 / 404-1002 / 403-with-sub / transport
/// error / deadline) — when both sides land transient, the most recent
/// transient is surfaced as the operation error.
enum HedgeClass {
    Final(Box<TransportResult>),
    Transient,
}

/// Pure classifier for a per-attempt result.
///
/// `Err` (e.g. failure constructing the request) and any non-final
/// TransportOutcome map to `Transient`. Per spec §7.1 a `Success` is
/// always final; an `HttpError` is final iff `is_final_result` returns
/// `true` for its status.
fn classify_hedge_result(result: azure_core::Result<TransportResult>) -> HedgeClass {
    match result {
        Ok(tr) => match &tr.outcome {
            TransportOutcome::Success { .. } => HedgeClass::Final(Box::new(tr)),
            TransportOutcome::HttpError { status, .. } => {
                if is_final_result(status) {
                    HedgeClass::Final(Box::new(tr))
                } else {
                    HedgeClass::Transient
                }
            }
            TransportOutcome::TransportError { .. } | TransportOutcome::DeadlineExceeded { .. } => {
                HedgeClass::Transient
            }
        },
        Err(_) => HedgeClass::Transient,
    }
}

/// Converts a final-classified [`TransportResult`] (success or final HTTP
/// error) into the response shape that `execute_operation_pipeline` would
/// otherwise return from STAGE 7's `Complete` or `Abort` arms.
///
/// `Success` flows through [`build_cosmos_response`] exactly like the
/// non-hedged happy path. A final `HttpError` (e.g. 409 Conflict) is
/// converted to an `azure_core::Error` via the same `build_http_error`
/// the evaluator uses for the non-hedged `Abort` path, so callers cannot
/// observe a behavioral difference between hedged and non-hedged final
/// HTTP errors. Transient outcomes are converted to a generic error —
/// reaching this function with a transient outcome would be a programmer
/// error in `execute_hedged` (the race loop only calls this on `Final`
/// classifications and on the "both transient" fallback where the diff
/// between calling and not is one branch).
fn finalize_hedge_attempt(
    result: Box<TransportResult>,
    diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => {
            build_cosmos_response(Box::new(TransportResult { outcome }), diagnostics)
        }
        TransportOutcome::HttpError {
            status,
            headers,
            body,
            ..
        } => {
            // Operation-level errors don't carry diagnostics today (the
            // builder is dropped here); that's a pre-existing limitation
            // of the driver shared with the non-hedged Abort path.
            let _ = diagnostics;
            Err(build_http_error(&status, &headers, &body))
        }
        TransportOutcome::TransportError { error, .. } => {
            let _ = diagnostics;
            Err(error)
        }
        TransportOutcome::DeadlineExceeded { .. } => {
            let _ = diagnostics;
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "deadline exceeded during hedged attempt",
            ))
        }
    }
}

/// If `action` is a same-pipeline retry and the operation is eligible
/// for cross-region hedging, rewrites it to `OperationAction::Hedge`.
/// Otherwise returns `action` unchanged.
///
/// Only `FailoverRetry` and `SessionRetry` are eligible for upgrade —
/// `Complete` is the operation's terminal success path, and `Abort`
/// already signals a non-retryable error. `request_timeout` is passed to
/// [`evaluate_hedge_eligibility`] so it can compute the §5.2 driver
/// default threshold (`min(1000ms, request_timeout / 2)`).
fn maybe_upgrade_to_hedge(
    action: OperationAction,
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    account_state: &AccountEndpointState,
    primary: &RoutingDecision,
    request_timeout: Option<Duration>,
) -> OperationAction {
    if !matches!(
        &action,
        OperationAction::FailoverRetry { .. } | OperationAction::SessionRetry { .. }
    ) {
        return action;
    }

    match evaluate_hedge_eligibility(operation, options, account_state, primary, request_timeout) {
        Some(upgrade) => {
            // Spec §10.4 reserved tracing surface: emit a structured event
            // when an operation is upgraded into the hedge race. Fields
            // mirror the inputs that drove the eligibility decision so
            // operators can correlate threshold tuning with observed
            // upgrades.
            tracing::debug!(
                threshold_ms = upgrade.threshold.get().as_millis() as u64,
                primary_region = ?primary.endpoint.region().map(crate::options::Region::as_str),
                secondary_region = ?upgrade.secondary_routing.endpoint.region().map(crate::options::Region::as_str),
                "cosmos.hedge.enabled_for_operation",
            );
            OperationAction::Hedge {
                secondary_routing: upgrade.secondary_routing,
                secondary_excluded_regions: upgrade.secondary_excluded_regions,
                threshold: upgrade.threshold,
                strategy_config: upgrade.strategy_config,
            }
        }
        None => action,
    }
}

/// Runs a single transport attempt against `routing` and returns the raw
/// [`TransportResult`]. Mirrors STAGE 3 + STAGE 4 + STAGE 4b of the main
/// operation loop, but operates on a pre-built routing so the call site
/// (`execute_hedged`) can use the same code path for both primary and
/// secondary hedge attempts.
///
/// Not invoked by the main pipeline loop today — the loop body inlines
/// the same STAGE 3/4/4b code for compatibility and to keep the diff
/// for Part 4b focused on hedging. A follow-up refactor may collapse
/// the duplication.
///
/// Differences from the inline loop body:
///
/// - **No per-state `hub_region_processing_only` latch.** Each hedge
///   today runs a single transport attempt with no retry loop, so the
///   per-state latch has nothing to drive. The shared latch (spec
///   §9.6) is plumbed through `shared_hub_region_latch` instead — the
///   secondary reads it on its first request so a 1002 latched by the
///   main pipeline (before hedging upgraded) immediately gets the
///   `x-ms-cosmos-hub-region-processing-only` header.
/// - **No `partition_key_range_id` capture.** The PK range ID is already
///   known to the surrounding `execute_operation_pipeline` (the
///   triggering attempt populated it) and is read-only inside a hedge.
async fn perform_single_attempt(
    ctx: &AttemptContext<'_>,
    routing: &RoutingDecision,
    execution_context: ExecutionContext,
    shared_hub_region_latch: Option<&Arc<AtomicBool>>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> azure_core::Result<TransportResult> {
    // Resolve session token using the same precedence the main loop uses.
    let resolved_session_token = ctx
        .session_consistency_active
        .then(|| {
            ctx.session_manager.resolve_session_token(
                ctx.operation,
                ctx.operation.request_headers().session_token.as_ref(),
            )
        })
        .flatten();

    let request_ctx = TransportRequestContext {
        routing,
        activity_id: ctx.activity_id,
        execution_context,
        deadline: ctx.deadline,
        resolved_session_token,
        throughput_control: ctx.throughput_control,
    };

    let mut transport_request =
        build_transport_request(ctx.operation, ctx.custom_headers, &request_ctx)?;
    // Hedging attempts have no per-state latch to consult — the only
    // signal is the cross-hedge shared latch (§9.6).
    if should_emit_hub_region_header(false, shared_hub_region_latch) {
        transport_request.headers.insert(
            HeaderName::from_static(request_header_names::HUB_REGION_PROCESSING_ONLY),
            HeaderValue::from_static("True"),
        );
    }
    apply_optional_request_headers(&mut transport_request, ctx.operation, ctx.options);

    let selected_transport = match ctx.pipeline_type {
        PipelineType::DataPlane => ctx
            .transport
            .get_dataplane_transport(ctx.account_endpoint, routing.transport_mode)?,
        PipelineType::Metadata => ctx.transport.get_metadata_transport(ctx.account_endpoint)?,
    };

    let result = execute_transport_pipeline(
        transport_request,
        &TransportPipelineContext {
            transport: &selected_transport,
            allow_sent_transport_retry: ctx.operation.is_read_only()
                || ctx.operation.is_idempotent(),
            credential: ctx.credential,
            user_agent: ctx.user_agent,
            pipeline_type: ctx.pipeline_type,
            transport_security: ctx.transport_security,
            endpoint_key: routing.endpoint.endpoint_key(),
        },
        diagnostics,
    )
    .await;

    // STAGE 4b: capture session token from a session-eligible response.
    if ctx.session_consistency_active {
        if let Some(cosmos_headers) = result.cosmos_headers() {
            if should_capture_session_token_from_status(
                cosmos_headers.substatus.as_ref(),
                &result.outcome,
            ) {
                ctx.session_manager
                    .capture_session_token(ctx.operation, cosmos_headers);
            }
        }
    }

    Ok(result)
}

/// Upper bound on how long [`execute_hedged`] waits for in-flight hedge
/// attempts to land their diagnostics after the application-cancel
/// deadline fires. Per spec §6.5 #7 / §14.2 the Rust hedge path
/// deliberately bounds this — .NET v3 awaits the most-recently-completed
/// task with no timeout, but the Rust path trades slightly less-rich
/// diagnostics-on-cancel for predictable user-visible cancel latency
/// when a transport future is stuck.
const HARVEST_WINDOW: Duration = Duration::from_millis(50);

/// Discriminator returned by the Stage-2 threshold-vs-deadline nested
/// race so the outer `select` against the primary can branch cleanly
/// without unwrapping a nested [`Either`].
enum TimerEvent {
    /// The hedge threshold elapsed first — Stage 3 should launch the
    /// secondary.
    ThresholdElapsed,
    /// The end-to-end deadline elapsed first — [`execute_hedged`] should
    /// harvest the still-pending primary within [`HARVEST_WINDOW`] and
    /// re-raise an [`application_cancelled_error`].
    DeadlineFired,
}

/// Builds a future that resolves when the supplied `deadline` elapses,
/// or never resolves when `deadline` is `None`. Used by [`execute_hedged`]
/// to layer end-to-end-deadline observation onto its `select`-based
/// races without changing those races' shapes when no deadline is set.
fn deadline_signal(deadline: Option<Instant>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    let Some(d) = deadline else {
        return Box::pin(pending::<()>());
    };
    let remaining_std = d.saturating_duration_since(Instant::now());
    match azure_core::time::Duration::try_from(remaining_std) {
        Ok(remaining) => Box::pin(azure_core::sleep(remaining)),
        Err(_) => Box::pin(futures::future::ready(())),
    }
}

/// Races `attempt` against [`HARVEST_WINDOW`] and, if `attempt` completes
/// within the window, merges its diagnostics into `parent` so the
/// returned [`application_cancelled_error`] carries the most-advanced
/// attempt's request trail (spec §6.5 #7 / §14.2). The result itself is
/// discarded — once the application cancellation has fired the operation
/// outcome is the cancellation error regardless of whether the in-flight
/// pipeline would have produced a final response.
async fn harvest_remaining_attempt<F>(attempt: F, parent: &mut DiagnosticsContextBuilder)
where
    F: Future<
            Output = (
                azure_core::Result<TransportResult>,
                DiagnosticsContextBuilder,
            ),
        > + Unpin
        + Send,
{
    let window = match azure_core::time::Duration::try_from(HARVEST_WINDOW) {
        Ok(d) => d,
        Err(_) => return,
    };
    let timer = Box::pin(azure_core::sleep(window));
    if let Either::Left(((_result, diag), _timer)) = select(attempt, timer).await {
        parent.merge_hedge_attempt(diag);
    }
    // Timer arm: harvest window exceeded; loser future is dropped and its
    // Drop chain cancels the in-flight transport (spec §6.5 #5).
}

/// Synthetic operation error returned by [`execute_hedged`] when the
/// application-cancellation deadline fires while one or both hedge
/// attempts are still racing. Mirrors the .NET v3 cancellation behavior
/// described in spec §6.5 #7 / §14.2. The actual diagnostics from the
/// most-advanced in-flight pipeline (when harvested within
/// [`HARVEST_WINDOW`]) are attached to the operation's
/// [`DiagnosticsContextBuilder`] before this error is returned — they
/// are not carried inside the error itself, matching the rest of the
/// operation pipeline's diagnostics model.
fn application_cancelled_error() -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        "operation cancelled by application deadline during cross-region hedging",
    )
}

/// Races a still-pending hedge attempt against the end-to-end deadline.
/// Returns `Some(result)` when the attempt completes first (normal
/// path); returns `None` after harvesting the attempt's diagnostics
/// within [`HARVEST_WINDOW`] when the deadline wins (spec §6.5 #7 / §14.2).
///
/// Used by [`execute_hedged`] in Stage 4's transient-fallthrough
/// branches where one side has already classified as transient and the
/// loop is `.await`-ing the other side — without this wrap a stuck
/// transport future would block the operation past the deadline.
async fn await_attempt_or_deadline_harvest<F>(
    attempt: F,
    deadline: Option<Instant>,
    parent: &mut DiagnosticsContextBuilder,
) -> Option<(
    azure_core::Result<TransportResult>,
    DiagnosticsContextBuilder,
)>
where
    F: Future<
            Output = (
                azure_core::Result<TransportResult>,
                DiagnosticsContextBuilder,
            ),
        > + Unpin
        + Send,
{
    let deadline_fut = deadline_signal(deadline);
    match select(attempt, deadline_fut).await {
        Either::Left((result, _deadline)) => Some(result),
        Either::Right(((), remaining)) => {
            harvest_remaining_attempt(remaining, parent).await;
            None
        }
    }
}

/// Returns `true` when `deadline` has elapsed relative to "now". A
/// `None` deadline never elapses. Used by [`execute_hedged`]'s
/// terminal both-transient branches to choose between surfacing
/// [`application_cancelled_error`] (when the deadline drove the
/// outcome) and [`transient_outcome_error`] (genuine transport-side
/// transient failure on both regions).
fn deadline_elapsed(deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|d| Instant::now() >= d)
}

/// Races a primary attempt against a single cross-region secondary
/// attempt per [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md)
/// §6.4. Returns the first **final** result; surfaces the most recent
/// transient as the operation error when both sides exhaust transient
/// outcomes.
///
/// The implementation is structured as two `futures::future::select`
/// calls so the borrow checker can reclaim each sub-builder cleanly
/// as its owning future completes:
///
/// 1. **Threshold race.** The primary future is launched immediately
///    against an `azure_core::sleep(threshold)` timer. If the primary
///    completes first this is the zero-overhead happy path
///    (spec §6.5 #3) — no secondary future is constructed, no extra
///    diagnostics sub-builder is cloned, no `Arc` allocations occur
///    beyond what the single primary attempt already needs.
/// 2. **Primary vs secondary race.** Once the timer fires we clone a
///    sub-builder for the secondary, launch it with
///    `ExecutionContext::Hedging`, and `select` against the still-pending
///    primary. The first side to land a `HedgeClass::Final` wins; a
///    transient on the first-to-complete side simply `.await`s the
///    other side. Both transient → most recent error.
///
/// Application-cancellation (observed via [`AttemptContext::deadline`])
/// is layered onto both races: the deadline future is wrapped into a
/// 3-way `select` with the threshold timer (Stage 2) and the
/// primary/secondary pair (Stage 4). When the deadline wins,
/// [`harvest_remaining_attempt`] / [`harvest_remaining_pair`] give the
/// most-advanced in-flight pipeline up to `HARVEST_WINDOW` to complete
/// so its diagnostics can be attached to the returned
/// [`application_cancelled_error`] (spec §6.5 #7 / §14.2). Loser
/// futures are dropped — structural cancellation propagates through
/// the transport pipeline (spec §6.5 #5).
///
/// PPCB feedback (spec §6.5 #8 / §9.5) is wired here via
/// [`record_hedge_outcome`]: every Final outcome routed to either side
/// is reported to the [`HedgeOutcomeRecorder`] so the PPCB side can
/// count consecutive alternate-wins and reset on primary-wins. The
/// real counter / state-transition machinery is a no-op stub today
/// (sub-plan 6a); the load-bearing callsites are these branches.
///
/// Spec items NOT yet implemented (tracked for follow-up commits per
/// [`docs/HEDGING_IMPLEMENTATION_PLAN.md`]):
///
/// - Operation-level retry inside each hedge (§8.4); we rely on
///   per-attempt transport-pipeline retry only.
async fn execute_hedged(
    ctx: &AttemptContext<'_>,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    threshold: HedgeThreshold,
    strategy_config: HedgingStrategyConfig,
    mut parent_diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    let primary_region = primary_routing.endpoint.region().cloned();
    let secondary_region = secondary_routing.endpoint.region().cloned();

    tracing::debug!(
        activity_id = %ctx.activity_id,
        threshold_ms = ?threshold.get().as_millis(),
        primary_region = ?primary_region.as_ref().map(|r| r.as_str()),
        secondary_region = ?secondary_region.as_ref().map(|r| r.as_str()),
        "execute_hedged: launching primary attempt",
    );

    // ── Stage 1: Build the primary future ─────────────────────────────
    // The diag clone is owned by the future and returned alongside the
    // result, so the borrow checker can reclaim it after `select` resolves.
    let primary_diag = parent_diagnostics.clone_for_hedge_attempt();
    let primary_attempt = Box::pin(async move {
        let mut diag = primary_diag;
        // Primary is launched before Stage 2 elapses, so no shared
        // hub-region latch can possibly exist yet — a `None` here
        // preserves the spec §6.5 #3 zero-overhead happy path (no
        // `Arc` allocation when the primary wins pre-threshold).
        let result = perform_single_attempt(
            ctx,
            primary_routing,
            ExecutionContext::Initial,
            None,
            &mut diag,
        )
        .await;
        (result, diag)
    });

    // ── Stage 2: Threshold race (zero-overhead happy path) ────────────
    // 3-way race: primary attempt vs threshold timer vs deadline. The
    // deadline arm enables the §6.5 #7 application-cancel path; when
    // the deadline has no value the deadline future never resolves and
    // the race collapses to the 2-way primary-vs-threshold semantics.
    let threshold_duration =
        azure_core::time::Duration::try_from(threshold.get()).map_err(|_| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "hedge threshold exceeds azure_core::time::Duration range",
            )
        })?;
    let threshold_timer = Box::pin(azure_core::sleep(threshold_duration));
    let deadline_timer = deadline_signal(ctx.deadline);

    // Nest threshold-vs-deadline first so the outer select against the
    // primary returns a single tagged event.
    let timer_event = Box::pin(async move {
        match select(threshold_timer, deadline_timer).await {
            Either::Left(((), _)) => TimerEvent::ThresholdElapsed,
            Either::Right(((), _)) => TimerEvent::DeadlineFired,
        }
    });

    let primary_attempt = match select(primary_attempt, timer_event).await {
        Either::Left(((result, diag), _timer)) => {
            // Primary completed before either timer — no secondary attempt
            // was constructed (spec §6.5 #3).
            parent_diagnostics.merge_hedge_attempt(diag);
            if let Some(region) = primary_region.clone() {
                parent_diagnostics
                    .set_hedge_diagnostics(HedgeDiagnostics::primary_only(strategy_config, region));
            }
            tracing::debug!(
                activity_id = %ctx.activity_id,
                "execute_hedged: primary won pre-threshold (zero-overhead happy path)",
            );
            // Spec §10.4 reserved: structured win event. `was_hedge=false`
            // distinguishes the zero-overhead happy path from a post-threshold
            // primary win.
            tracing::info!(
                activity_id = %ctx.activity_id,
                winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                was_hedge = false,
                "cosmos.hedge.won",
            );
            // Spec §9.5: primary-region win resets the consecutive-hedge-win
            // counter on this (partition, primary_region) pair. Recorded
            // even in the zero-overhead path because a streak of fast
            // primaries should clear any prior alternate-win backlog.
            if result.is_ok() {
                record_hedge_outcome(
                    ctx.hedge_outcome_recorder,
                    HedgeOutcome::PrimaryWin,
                    ctx.partition_key_range_id.as_ref(),
                    primary_region.as_ref(),
                );
            }
            return finalize_hedge_attempt(Box::new(result?), parent_diagnostics);
        }
        Either::Right((TimerEvent::ThresholdElapsed, remaining_primary)) => remaining_primary,
        Either::Right((TimerEvent::DeadlineFired, remaining_primary)) => {
            // §6.5 #7 / §14.2: app-cancel observed at Stage 2 — only the
            // primary is in-flight (no secondary was ever built). Harvest
            // it within HARVEST_WINDOW for diagnostics, then re-raise.
            tracing::debug!(
                activity_id = %ctx.activity_id,
                "execute_hedged: deadline fired pre-threshold; harvesting primary",
            );
            harvest_remaining_attempt(remaining_primary, &mut parent_diagnostics).await;
            if let Some(region) = primary_region.clone() {
                parent_diagnostics
                    .set_hedge_diagnostics(HedgeDiagnostics::primary_only(strategy_config, region));
            }
            return Err(application_cancelled_error());
        }
    };

    // ── Stage 3: Launch secondary ─────────────────────────────────────
    // The cross-hedge shared hub-region-processing-only latch (spec §9.6)
    // is constructed here — *after* the threshold elapses, so the
    // zero-overhead happy path (§6.5 #3) never allocates the `Arc`.
    // Gated on the same predicates as the per-state latch trigger:
    // data-plane scope + single-master account. Seeded from whatever
    // value the main pipeline's per-state latch held at the moment
    // hedging upgraded, so a 1002 already discovered before hedging
    // fired carries forward into the secondary's first request.
    let shared_hub_region_latch = should_build_shared_hub_region_latch(
        ctx.pipeline_type,
        ctx.can_use_multiple_write_locations,
    )
    .then(|| Arc::new(AtomicBool::new(ctx.hub_region_processing_only_initial)));
    let secondary_shared_latch = shared_hub_region_latch.clone();
    let secondary_diag = parent_diagnostics.clone_for_hedge_attempt();
    let secondary_attempt = Box::pin(async move {
        let mut diag = secondary_diag;
        let result = perform_single_attempt(
            ctx,
            secondary_routing,
            ExecutionContext::Hedging,
            secondary_shared_latch.as_ref(),
            &mut diag,
        )
        .await;
        (result, diag)
    });

    tracing::debug!(
        activity_id = %ctx.activity_id,
        shared_hub_region_latch = shared_hub_region_latch.is_some(),
        "execute_hedged: threshold elapsed; secondary launched",
    );
    // Spec §10.4 reserved tracing surface: structured "alternate spawned"
    // event distinct from the freeform message above. Fields carry the
    // race timing (threshold) and the target region so operators can
    // correlate spawn rate with observed tail-latency improvements.
    tracing::debug!(
        activity_id = %ctx.activity_id,
        threshold_ms = threshold.get().as_millis() as u64,
        secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
        "cosmos.hedge.alternate_spawned",
    );

    // ── Stage 4: Primary vs secondary race ────────────────────────────
    // Per-attempt deadline observation lives inside the transport
    // pipeline (TPS §5.1) — when the end-to-end deadline fires while an
    // attempt is in flight, the transport returns `DeadlineExceeded`
    // which classifies as transient. The two `.await` calls in the
    // transient-fallthrough branches below are wrapped with
    // [`await_attempt_or_deadline_harvest`] so a deadline that fires
    // while we're waiting on the still-pending side triggers the
    // §6.5 #7 harvest path instead of blocking. The both-transient
    // terminal branches inspect the deadline to choose between
    // [`application_cancelled_error`] and [`transient_outcome_error`].
    match select(primary_attempt, secondary_attempt).await {
        Either::Left(((primary_result, primary_diag), secondary_remaining)) => {
            parent_diagnostics.merge_hedge_attempt(primary_diag);
            match classify_hedge_result(primary_result) {
                HedgeClass::Final(tr) => {
                    // Primary won post-threshold; drop secondary.
                    if let (Some(p), Some(s)) = (primary_region.clone(), secondary_region.clone()) {
                        parent_diagnostics.set_hedge_diagnostics(
                            HedgeDiagnostics::primary_won_after_hedge(strategy_config, p, s),
                        );
                    }
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        "execute_hedged: primary won after threshold",
                    );
                    // Spec §10.4 reserved: secondary attempt was dropped
                    // structurally when this branch was taken.
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        which = "secondary",
                        target_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "primary_won_post_threshold",
                        "cosmos.hedge.canceled",
                    );
                    tracing::info!(
                        activity_id = %ctx.activity_id,
                        winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.won",
                    );
                    record_hedge_outcome(
                        ctx.hedge_outcome_recorder,
                        HedgeOutcome::PrimaryWin,
                        ctx.partition_key_range_id.as_ref(),
                        primary_region.as_ref(),
                    );
                    finalize_hedge_attempt(tr, parent_diagnostics)
                }
                HedgeClass::Transient => {
                    // Primary transient — wait for secondary (observing deadline).
                    let Some((secondary_result, secondary_diag)) =
                        await_attempt_or_deadline_harvest(
                            secondary_remaining,
                            ctx.deadline,
                            &mut parent_diagnostics,
                        )
                        .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.activity_id,
                            "execute_hedged: deadline fired awaiting secondary after primary transient",
                        );
                        if let (Some(p), Some(s)) = (primary_region, secondary_region) {
                            parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                                strategy_config,
                                p,
                                s,
                            ));
                        }
                        return Err(application_cancelled_error());
                    };
                    parent_diagnostics.merge_hedge_attempt(secondary_diag);
                    match classify_hedge_result(secondary_result) {
                        HedgeClass::Final(tr) => {
                            if let (Some(p), Some(s)) =
                                (primary_region.clone(), secondary_region.clone())
                            {
                                parent_diagnostics.set_hedge_diagnostics(
                                    HedgeDiagnostics::hedge_won(strategy_config, p, s),
                                );
                            }
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                "execute_hedged: secondary won after primary transient",
                            );
                            tracing::info!(
                                activity_id = %ctx.activity_id,
                                winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.won",
                            );
                            record_hedge_outcome(
                                ctx.hedge_outcome_recorder,
                                HedgeOutcome::AlternateWin,
                                ctx.partition_key_range_id.as_ref(),
                                primary_region.as_ref(),
                            );
                            finalize_hedge_attempt(tr, parent_diagnostics)
                        }
                        HedgeClass::Transient => {
                            // Both transient — attach hedge diagnostics for
                            // observability and surface either the app-cancel
                            // error (§6.5 #7) or the synthetic
                            // both-transient error depending on whether the
                            // deadline drove the outcome.
                            if let (Some(p), Some(s)) = (primary_region, secondary_region) {
                                parent_diagnostics.set_hedge_diagnostics(
                                    HedgeDiagnostics::hedge_won(strategy_config, p, s),
                                );
                            }
                            if deadline_elapsed(ctx.deadline) {
                                tracing::debug!(
                                    activity_id = %ctx.activity_id,
                                    "execute_hedged: both transient under elapsed deadline; surfacing app-cancel",
                                );
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    deadline_elapsed = true,
                                    "cosmos.hedge.both_transient",
                                );
                                Err(application_cancelled_error())
                            } else {
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    "execute_hedged: both primary and secondary transient; surfacing secondary error",
                                );
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    deadline_elapsed = false,
                                    "cosmos.hedge.both_transient",
                                );
                                Err(transient_outcome_error())
                            }
                        }
                    }
                }
            }
        }
        Either::Right(((secondary_result, secondary_diag), primary_remaining)) => {
            parent_diagnostics.merge_hedge_attempt(secondary_diag);
            match classify_hedge_result(secondary_result) {
                HedgeClass::Final(tr) => {
                    if let (Some(p), Some(s)) = (primary_region.clone(), secondary_region.clone()) {
                        parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                            strategy_config,
                            p,
                            s,
                        ));
                    }
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        "execute_hedged: secondary won race",
                    );
                    // Spec §10.4 reserved: primary attempt was dropped
                    // structurally when secondary won the race.
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        which = "primary",
                        target_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "secondary_won_race",
                        "cosmos.hedge.canceled",
                    );
                    tracing::info!(
                        activity_id = %ctx.activity_id,
                        winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.won",
                    );
                    record_hedge_outcome(
                        ctx.hedge_outcome_recorder,
                        HedgeOutcome::AlternateWin,
                        ctx.partition_key_range_id.as_ref(),
                        primary_region.as_ref(),
                    );
                    finalize_hedge_attempt(tr, parent_diagnostics)
                }
                HedgeClass::Transient => {
                    // Secondary transient — wait for primary (observing deadline).
                    let Some((primary_result, primary_diag)) = await_attempt_or_deadline_harvest(
                        primary_remaining,
                        ctx.deadline,
                        &mut parent_diagnostics,
                    )
                    .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.activity_id,
                            "execute_hedged: deadline fired awaiting primary after secondary transient",
                        );
                        if let (Some(p), Some(s)) = (primary_region, secondary_region) {
                            parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                                strategy_config,
                                p,
                                s,
                            ));
                        }
                        return Err(application_cancelled_error());
                    };
                    parent_diagnostics.merge_hedge_attempt(primary_diag);
                    match classify_hedge_result(primary_result) {
                        HedgeClass::Final(tr) => {
                            if let (Some(p), Some(s)) =
                                (primary_region.clone(), secondary_region.clone())
                            {
                                parent_diagnostics.set_hedge_diagnostics(
                                    HedgeDiagnostics::primary_won_after_hedge(
                                        strategy_config,
                                        p,
                                        s,
                                    ),
                                );
                            }
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                "execute_hedged: primary won after secondary transient",
                            );
                            tracing::info!(
                                activity_id = %ctx.activity_id,
                                winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.won",
                            );
                            record_hedge_outcome(
                                ctx.hedge_outcome_recorder,
                                HedgeOutcome::PrimaryWin,
                                ctx.partition_key_range_id.as_ref(),
                                primary_region.as_ref(),
                            );
                            finalize_hedge_attempt(tr, parent_diagnostics)
                        }
                        HedgeClass::Transient => {
                            if let (Some(p), Some(s)) = (primary_region, secondary_region) {
                                parent_diagnostics.set_hedge_diagnostics(
                                    HedgeDiagnostics::hedge_won(strategy_config, p, s),
                                );
                            }
                            if deadline_elapsed(ctx.deadline) {
                                tracing::debug!(
                                    activity_id = %ctx.activity_id,
                                    "execute_hedged: both transient under elapsed deadline; surfacing app-cancel",
                                );
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    deadline_elapsed = true,
                                    "cosmos.hedge.both_transient",
                                );
                                Err(application_cancelled_error())
                            } else {
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    "execute_hedged: both secondary and primary transient; surfacing primary error",
                                );
                                tracing::warn!(
                                    activity_id = %ctx.activity_id,
                                    deadline_elapsed = false,
                                    "cosmos.hedge.both_transient",
                                );
                                Err(transient_outcome_error())
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Generic "both sides transient" error returned by [`execute_hedged`]
/// when neither the primary nor the secondary produced a final result.
/// Mirrors the .NET v3 hedging behavior of producing a synthetic
/// operation-level error in this case rather than re-entering the
/// per-pipeline retry loop (Terminal semantics per
/// [`docs/HEDGING_IMPLEMENTATION_PLAN.md`] Part 4).
fn transient_outcome_error() -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        "hedging completed without producing a final response",
    )
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use azure_core::http::headers::HeaderName;
    use url::Url;

    use super::build_transport_request;
    use super::TransportRequestContext;
    use crate::{
        diagnostics::ExecutionContext,
        driver::{
            pipeline::components::{RoutingDecision, TransportMode},
            routing::{
                AccountEndpointState, CosmosEndpoint, LocationEffect, LocationIndex,
                LocationSnapshot,
            },
        },
        models::{
            request_header_names, AccountReference, ActivityId, ContainerProperties,
            ContainerReference, CosmosOperation, DatabaseReference, ItemReference, PartitionKey,
            PartitionKeyDefinition, SystemProperties, ThroughputControlGroupName,
        },
        options::{PriorityLevel, ThroughputControlGroupSnapshot},
    };

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &test_container_props(),
        )
    }

    fn test_routing() -> RoutingDecision {
        let endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        RoutingDecision {
            selected_url: endpoint.url().clone(),
            endpoint,
            transport_mode: TransportMode::Gateway,
        }
    }

    #[test]
    fn build_transport_request_feed_path_is_resolved() {
        let operation = CosmosOperation::read_all_databases(test_account());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert_eq!(request.url.path(), "/dbs");
    }

    #[test]
    fn build_transport_request_single_resource_path_is_resolved() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let operation = CosmosOperation::read_database(db);

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert_eq!(request.url.path(), "/dbs/mydb");
    }

    #[test]
    fn build_transport_request_uses_operation_activity_id_when_present() {
        let operation = CosmosOperation::read_all_databases(test_account())
            .with_activity_id(ActivityId::from_string("operation-activity".to_string()));

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        let activity_header = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-activity-id"))
            .expect("activity id should be set");
        assert_eq!(activity_header, "operation-activity");
    }

    #[test]
    fn build_transport_request_adds_partition_key_header_for_item_operation() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item_ref);

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Retry,
            deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        let partition_key_header = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-documentdb-partitionkey"))
            .expect("partition key header should be set");
        assert_eq!(partition_key_header, "[\"pk1\"]");
    }

    #[test]
    fn build_transport_request_uses_routed_endpoint_url_directly() {
        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let routing = RoutingDecision {
            endpoint: CosmosEndpoint::regional_with_gateway20(
                "westus2".into(),
                Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
                Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
            ),
            selected_url: Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
            transport_mode: TransportMode::Gateway20,
        };

        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert_eq!(
            request.url.as_str(),
            "https://test-westus2-thin.documents.azure.com:444/dbs/mydb"
        );
    }

    #[test]
    fn build_transport_request_uses_default_url_for_global_endpoint() {
        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let routing = RoutingDecision {
            endpoint: CosmosEndpoint::global(
                Url::parse("https://test.documents.azure.com:443/").unwrap(),
            ),
            selected_url: Url::parse("https://test.documents.azure.com:443/").unwrap(),
            transport_mode: TransportMode::Gateway,
        };

        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert_eq!(
            request.url.as_str(),
            "https://test.documents.azure.com/dbs/mydb"
        );
    }

    #[test]
    fn resolve_endpoint_uses_write_region_for_single_write_session_retry() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let write_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint].into(),
            preferred_write_endpoints: vec![write_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: write_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 1,
            max_failover_retries: 3,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredWriteEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, write_endpoint);
    }

    #[test]
    fn resolve_endpoint_deprioritizes_unavailable_over_global_fallback() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            read_endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::TransportError,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![default_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            max_failover_retries: 3,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        // Unavailable regional endpoint is de-prioritized but still preferred
        // over the global fallback.
        assert_eq!(routing.endpoint, read_endpoint);
    }

    #[test]
    fn resolve_endpoint_ignores_write_forbidden_for_reads() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            read_endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::WriteForbidden,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![read_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: read_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            max_failover_retries: 3,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, read_endpoint);
    }

    #[test]
    fn stale_generation_advances_across_refreshed_endpoint_list() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let endpoint_a = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let endpoint_b = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );
        let endpoint_c = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 1,
            preferred_read_endpoints: vec![
                endpoint_a.clone(),
                endpoint_b.clone(),
                endpoint_c.clone(),
            ]
            .into(),
            preferred_write_endpoints: vec![
                endpoint_a.clone(),
                endpoint_b.clone(),
                endpoint_c.clone(),
            ]
            .into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: endpoint_a.clone(),
        }));

        let stale_retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0).next(3),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            max_failover_retries: 3,
            max_session_retries: 3,
            can_use_multiple_write_locations: true,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let first_routing = super::resolve_endpoint(
            &operation,
            &stale_retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(first_routing.endpoint, endpoint_a);

        let advanced_state = stale_retry_state
            .advance_failover()
            .advance_location(3, location.account.generation);

        let second_routing = super::resolve_endpoint(
            &operation,
            &advanced_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(second_routing.endpoint, endpoint_b);
    }

    mod should_capture_session_token_from_status_tests {
        use azure_core::http::{headers::Headers, StatusCode};

        use crate::{
            driver::pipeline::components::TransportOutcome,
            models::{CosmosResponseHeaders, CosmosStatus, SubStatusCode},
        };

        use super::super::should_capture_session_token_from_status;

        fn success_outcome() -> TransportOutcome {
            TransportOutcome::Success {
                status: CosmosStatus::new(StatusCode::Ok),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: Vec::new(),
            }
        }

        fn http_error_outcome(status: StatusCode) -> TransportOutcome {
            TransportOutcome::HttpError {
                status: CosmosStatus::new(status),
                headers: Headers::new(),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: Vec::new(),
                request_sent: crate::diagnostics::RequestSentStatus::Sent,
            }
        }

        #[test]
        fn captures_on_success() {
            let outcome = success_outcome();
            assert!(should_capture_session_token_from_status(None, &outcome));
        }

        #[test]
        fn captures_on_409_conflict() {
            let outcome = http_error_outcome(StatusCode::Conflict);
            assert!(should_capture_session_token_from_status(None, &outcome));
        }

        #[test]
        fn captures_on_412_precondition_failed() {
            let outcome = http_error_outcome(StatusCode::PreconditionFailed);
            assert!(should_capture_session_token_from_status(None, &outcome));
        }

        #[test]
        fn skips_on_404_with_substatus_1002() {
            let outcome = http_error_outcome(StatusCode::NotFound);
            let substatus = SubStatusCode::READ_SESSION_NOT_AVAILABLE;
            assert!(!should_capture_session_token_from_status(
                Some(&substatus),
                &outcome
            ));
        }

        #[test]
        fn captures_on_404_without_substatus_1002() {
            let outcome = http_error_outcome(StatusCode::NotFound);
            assert!(should_capture_session_token_from_status(None, &outcome));
        }

        #[test]
        fn skips_on_500_internal_server_error() {
            let outcome = http_error_outcome(StatusCode::InternalServerError);
            assert!(!should_capture_session_token_from_status(None, &outcome));
        }
    }

    mod effective_consistency_tests {
        use crate::{models::DefaultConsistencyLevel, options::ReadConsistencyStrategy};

        #[test]
        fn default_strategy_with_session_account() {
            assert!(ReadConsistencyStrategy::Default
                .is_session_effective(DefaultConsistencyLevel::Session));
        }

        #[test]
        fn default_strategy_with_strong_account() {
            assert!(!ReadConsistencyStrategy::Default
                .is_session_effective(DefaultConsistencyLevel::Strong));
        }

        #[test]
        fn session_strategy_overrides_account() {
            assert!(ReadConsistencyStrategy::Session
                .is_session_effective(DefaultConsistencyLevel::Strong));
        }

        #[test]
        fn eventual_strategy_never_session() {
            assert!(!ReadConsistencyStrategy::Eventual
                .is_session_effective(DefaultConsistencyLevel::Session));
        }

        #[test]
        fn consistent_prefix_not_session() {
            assert!(!ReadConsistencyStrategy::Default
                .is_session_effective(DefaultConsistencyLevel::ConsistentPrefix));
        }
    }

    #[test]
    fn resolve_endpoint_prefers_gateway20_for_dataplane_reads() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let endpoint = CosmosEndpoint::regional_with_gateway20(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, endpoint);
        assert_eq!(routing.transport_mode, TransportMode::Gateway20);
        assert_eq!(
            routing.selected_url.as_str(),
            "https://test-westus2-thin.documents.azure.com:444/"
        );
    }

    #[test]
    fn resolve_endpoint_skips_unavailable_region_when_gateway20_is_present() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let endpoint = CosmosEndpoint::regional_with_gateway20(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
        );
        let fallback_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::TransportError,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone(), fallback_endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: fallback_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            3,
        );

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, fallback_endpoint);
    }

    #[test]
    fn resolve_endpoint_falls_back_to_global_when_all_excluded() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![default_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            max_failover_retries: 3,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: vec!["westus2".into()],
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        // When all endpoints are excluded, the global endpoint is the only option.
        assert_eq!(routing.endpoint, default_endpoint);
    }

    #[test]
    fn resolve_endpoint_picks_first_available_over_unavailable() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let unavailable_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );
        let available_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            unavailable_endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![
                unavailable_endpoint.clone(),
                available_endpoint.clone(),
            ]
            .into(),
            preferred_write_endpoints: vec![default_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        // Available endpoint is preferred over the unavailable one.
        assert_eq!(routing.endpoint, available_endpoint);
    }

    // ── PPAF write-retry cross-region fallback ─────────────────────────

    fn make_pending_partition_mark_for_region(
        region: &'static str,
    ) -> crate::driver::routing::LocationEffect {
        crate::driver::routing::LocationEffect::MarkPartitionUnavailable(
            crate::driver::routing::UnavailablePartition {
                partition_key_range_id: None,
                region: Some(region.into()),
                is_read: false,
                is_partitioned_resource: true,
            },
        )
    }

    #[test]
    fn resolve_endpoint_skips_in_flight_failed_write_region() {
        // Multi-write account: writes can rotate across both write regions.
        // After the first attempt to eastus fails, the in-flight skip set
        // should route the next attempt to westus2 instead of repeating eastus.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let west = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
            preferred_write_endpoints: vec![east.clone(), west.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: east.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        // Simulate one prior failed attempt against eastus that deferred
        // a partition mark for that region.
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("eastus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, west,
            "in-flight skip set must route the retry to westus2"
        );
    }

    #[test]
    fn resolve_endpoint_ppaf_write_uses_read_endpoints_as_primary_list() {
        // Single-master account with PPAF: preferred_write_endpoints = [eastus]
        // only, but preferred_read_endpoints = [westus2, eastus] (different
        // ordering — westus2 is the user's preferred region). With PPAF
        // enabled, a write must iterate over the READ list so the first
        // attempt targets westus2 (the user's preferred region) and probes for
        // the actual write region, rather than blindly hitting the only entry
        // in the write list.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let west = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![west.clone(), east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );
        retry_state.ppaf_write_retry_allowed = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, west,
            "PPAF write must use the read endpoint list (preferred order) as the primary candidate set"
        );
    }

    #[test]
    fn resolve_endpoint_ppaf_falls_back_to_read_region_when_write_exhausted() {
        // Single-master account with PPAF: write_endpoints = [eastus] only.
        // After eastus has failed, PPAF write retry should fall back to a
        // read region (westus2) for cross-regional write region discovery.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let west = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );
        retry_state.ppaf_write_retry_allowed = true;
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("eastus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, west,
            "PPAF write retry must fall back to a read region when all write regions are in the in-flight skip set"
        );
    }

    #[test]
    fn resolve_endpoint_ppaf_override_skipped_when_current_endpoint_failed_in_flight() {
        // Regression test for PPAF write failback infinite-retry bug.
        //
        // Scenario: an earlier successful PPAF discovery recorded an
        // override pointing at `centralus`. A new write begins:
        //   1. resolve_endpoint sees the PPAF override → routes to centralus.
        //   2. centralus is failing back (returns 403/3) → the failure-path
        //      effects are deferred into `pending_write_effects` because
        //      PPAF SM defers writes until success/region-confirming abort.
        //   3. The persistent `failover_overrides[pk_range_id].current_endpoint`
        //      still says centralus (deferred effects haven't been flushed).
        //   4. On retry, the PPAF override branch in resolve_endpoint MUST
        //      consult the in-flight skip set and fall through to the primary
        //      selection (which has already rotated past centralus) — otherwise
        //      every retry hammers the same failed override region.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let north = CosmosEndpoint::regional(
            "northcentralus".into(),
            Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
        );
        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
            preferred_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        // Build a partition state with PPAF enabled and a stale override
        // entry that points at the now-failing centralus region.
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverConfig, PartitionFailoverEntry,
        };
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverConfig::default());
        partitions.per_partition_automatic_failover_enabled = true;
        partitions.failover_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: central.clone(),
                first_failed_endpoint: central.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );
        retry_state.ppaf_write_retry_allowed = true;
        retry_state.partition_key_range_id = Some(pk_range_id);
        // Simulate that the current operation already failed against centralus
        // (override target) and the deferred mark is sitting in the buffer.
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("centralus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, north,
            "PPAF override pointing at a region already in the in-flight skip set must be skipped, \
             so cross-region retry can rotate to a different region instead of looping on the failed override"
        );
    }

    #[test]
    fn resolve_endpoint_ppaf_override_honored_when_current_endpoint_not_failed() {
        // Counterpart to the regression test above: when the override's
        // current_endpoint is NOT in the in-flight skip set, the PPAF
        // override MUST be honored (otherwise prior failover decisions
        // would be silently ignored on every retry).
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let north = CosmosEndpoint::regional(
            "northcentralus".into(),
            Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
        );
        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
            preferred_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverConfig, PartitionFailoverEntry,
        };
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverConfig::default());
        partitions.per_partition_automatic_failover_enabled = true;
        partitions.failover_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: central.clone(),
                first_failed_endpoint: north.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );
        retry_state.ppaf_write_retry_allowed = true;
        retry_state.partition_key_range_id = Some(pk_range_id);
        // pending_write_effects is empty — first attempt of the operation,
        // so the override should be applied as-is.

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, central,
            "PPAF override with a healthy current_endpoint must be honored"
        );
    }

    #[test]
    fn resolve_endpoint_does_not_skip_for_reads() {
        // The in-flight skip set is only consulted for writes — reads should
        // continue to use the standard preferred-endpoint selection even when
        // pending_write_effects is non-empty (which can never occur
        // for a read in practice, but the invariant should hold defensively).
        let operation = CosmosOperation::read_all_databases(test_account());

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            2,
        );
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("eastus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, east);
    }

    // ── flush_pending_write_effects: skip already-applied ──────────────

    #[test]
    fn already_applied_skips_endpoint_mark_when_endpoint_in_unavailable_set() {
        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            east.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::TransportError,
            ),
        );
        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let effect = LocationEffect::MarkEndpointUnavailable {
            endpoint: east,
            reason: crate::driver::routing::UnavailableReason::TransportError,
        };
        assert!(
            super::is_effect_already_applied(&effect, &location),
            "endpoint already in unavailable set must be considered already applied"
        );
    }

    #[test]
    fn already_applied_returns_false_when_endpoint_not_in_unavailable_set() {
        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let effect = LocationEffect::MarkEndpointUnavailable {
            endpoint: east,
            reason: crate::driver::routing::UnavailableReason::TransportError,
        };
        assert!(!super::is_effect_already_applied(&effect, &location));
    }

    #[test]
    fn already_applied_skips_partition_mark_when_override_already_moved_past_failed_region() {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let west = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );
        let pk_range_id = PartitionKeyRangeId::from(String::from("0"));

        let mut partitions = PartitionEndpointState {
            per_partition_automatic_failover_enabled: true,
            ..Default::default()
        };
        partitions.failover_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: west.clone(), // already moved off eastus
                first_failed_endpoint: east.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let location = LocationSnapshot::for_tests_with_partitions(
            Arc::new(AccountEndpointState {
                generation: 0,
                preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
                preferred_write_endpoints: vec![east.clone()].into(),
                unavailable_endpoints: Default::default(),
                multiple_write_locations_enabled: false,
                default_endpoint: east.clone(),
            }),
            Arc::new(partitions),
        );

        let effect = LocationEffect::MarkPartitionUnavailable(
            crate::driver::routing::UnavailablePartition {
                partition_key_range_id: Some(pk_range_id),
                region: Some("eastus".into()),
                is_read: false,
                is_partitioned_resource: true,
            },
        );
        assert!(
            super::is_effect_already_applied(&effect, &location),
            "PPAF override that has already moved past the failed region must be considered already applied"
        );
    }

    #[test]
    fn already_applied_returns_false_when_partition_override_still_on_failed_region() {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;

        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let pk_range_id = PartitionKeyRangeId::from(String::from("0"));

        let mut partitions = PartitionEndpointState {
            per_partition_automatic_failover_enabled: true,
            ..Default::default()
        };
        partitions.failover_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: east.clone(), // still on the failed region
                first_failed_endpoint: east.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let location = LocationSnapshot::for_tests_with_partitions(
            Arc::new(AccountEndpointState {
                generation: 0,
                preferred_read_endpoints: vec![east.clone()].into(),
                preferred_write_endpoints: vec![east.clone()].into(),
                unavailable_endpoints: Default::default(),
                multiple_write_locations_enabled: false,
                default_endpoint: east.clone(),
            }),
            Arc::new(partitions),
        );

        let effect = LocationEffect::MarkPartitionUnavailable(
            crate::driver::routing::UnavailablePartition {
                partition_key_range_id: Some(pk_range_id),
                region: Some("eastus".into()),
                is_read: false,
                is_partitioned_resource: true,
            },
        );
        assert!(
            !super::is_effect_already_applied(&effect, &location),
            "override still pointing at the failed region must NOT be skipped"
        );
    }

    #[test]
    fn already_applied_returns_false_when_no_partition_override_exists() {
        let east = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }));

        let effect = LocationEffect::MarkPartitionUnavailable(
            crate::driver::routing::UnavailablePartition {
                partition_key_range_id: Some(
                    crate::driver::routing::partition_key_range_id::PartitionKeyRangeId::from(
                        String::from("0"),
                    ),
                ),
                region: Some("eastus".into()),
                is_read: false,
                is_partitioned_resource: true,
            },
        );
        assert!(!super::is_effect_already_applied(&effect, &location));
    }

    #[test]
    fn build_transport_request_sets_is_upsert_header() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::upsert_item(item).with_body(b"{}".to_vec());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        let is_upsert = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-documentdb-is-upsert"))
            .expect("is-upsert header should be set");
        assert_eq!(is_upsert, "true");

        // Upsert targets the collection feed URL, not the individual document.
        assert_eq!(
            request.url.path(),
            "/dbs/testdb/colls/testcontainer/docs",
            "upsert should POST to the collection feed, not /docs/doc1"
        );
    }

    #[test]
    fn build_transport_request_omits_is_upsert_header_for_create() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-documentdb-is-upsert"))
                .is_none(),
            "is-upsert header should not be set for create"
        );

        // Create targets the collection feed URL, not the individual document.
        assert_eq!(
            request.url.path(),
            "/dbs/testdb/colls/testcontainer/docs",
            "create should POST to the collection feed, not /docs/doc1"
        );
    }

    #[test]
    fn build_transport_request_sets_batch_headers() {
        let operation = CosmosOperation::batch(test_container(), PartitionKey::from("pk1"))
            .with_body(b"[]".to_vec());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request")),
            Some("True"),
            "is-batch-request header should be set"
        );
        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-cosmos-batch-atomic")),
            Some("True"),
            "batch-atomic header should be set"
        );
        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                "x-ms-cosmos-batch-continue-on-error"
            )),
            Some("False"),
            "batch-continue-on-error header should be set"
        );
    }

    #[test]
    fn build_transport_request_omits_batch_headers_for_create() {
        let container = test_container();
        let operation = CosmosOperation::create_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ))
        .with_body(b"{}".to_vec());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, None, &ctx).expect("request should build");

        assert!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request"))
                .is_none(),
            "batch headers should not be set for create"
        );
    }

    #[test]
    fn build_transport_request_sets_priority_level_header() {
        let container = test_container();
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let routing = test_routing();
        let activity_id = ActivityId::new_uuid();

        let snapshot = ThroughputControlGroupSnapshot::new(
            ThroughputControlGroupName::new("test-priority"),
            container,
            false,
        )
        .with_priority_level(PriorityLevel::Low);

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: Some(&snapshot),
        };
        let request = build_transport_request(&operation, None, &ctx).unwrap();

        let priority = request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::PRIORITY_LEVEL,
            ))
            .expect("priority level header should be set");
        assert_eq!(priority, "Low");
        assert!(request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::THROUGHPUT_BUCKET
            ))
            .is_none());
    }

    #[test]
    fn build_transport_request_sets_throughput_bucket_header() {
        let container = test_container();
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let routing = test_routing();
        let activity_id = ActivityId::new_uuid();

        let snapshot = ThroughputControlGroupSnapshot::new(
            ThroughputControlGroupName::new("test-bucket"),
            container,
            false,
        )
        .with_throughput_bucket(42);

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: Some(&snapshot),
        };
        let request = build_transport_request(&operation, None, &ctx).unwrap();

        let bucket = request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::THROUGHPUT_BUCKET,
            ))
            .expect("throughput bucket header should be set");
        assert_eq!(bucket, "42");
        assert!(request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::PRIORITY_LEVEL
            ))
            .is_none());
    }

    #[test]
    fn build_transport_request_sets_both_throughput_headers() {
        let container = test_container();
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let routing = test_routing();
        let activity_id = ActivityId::new_uuid();

        let snapshot = ThroughputControlGroupSnapshot::new(
            ThroughputControlGroupName::new("test-both"),
            container,
            false,
        )
        .with_priority_level(PriorityLevel::High)
        .with_throughput_bucket(100);

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: Some(&snapshot),
        };
        let request = build_transport_request(&operation, None, &ctx).unwrap();

        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                request_header_names::PRIORITY_LEVEL
            )),
            Some("High")
        );
        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                request_header_names::THROUGHPUT_BUCKET
            )),
            Some("100")
        );
    }

    #[test]
    fn build_transport_request_auto_emits_query_headers_for_query_operations() {
        // Single-partition item query
        let op = CosmosOperation::query_items(test_container(), PartitionKey::from("pk1"))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
        assert_query_headers_present(&op, "query_items");

        // Cross-partition item query
        let op = CosmosOperation::query_items_cross_partition(test_container())
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
        assert_query_headers_present(&op, "query_items_cross_partition");

        // Offer query (used by find_offer / throughput poller path)
        let op = CosmosOperation::query_offers(test_account())
            .with_body(br#"{"query":"SELECT * FROM root"}"#.to_vec());
        assert_query_headers_present(&op, "query_offers");
    }

    /// Helper: builds a transport request from `op` and asserts the two
    /// well-known query headers (`x-ms-documentdb-isquery` and the
    /// `application/query+json` content type) are auto-emitted by the pipeline.
    fn assert_query_headers_present(op: &CosmosOperation, label: &str) {
        let routing = test_routing();
        let activity_id = ActivityId::new_uuid();
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        let req = build_transport_request(op, None, &ctx).expect("request should build");
        assert_eq!(
            req.headers
                .get_optional_str(&HeaderName::from_static(request_header_names::IS_QUERY)),
            Some("True"),
            "{label}: x-ms-documentdb-isquery should be 'True'"
        );
        assert_eq!(
            req.headers
                .get_optional_str(&azure_core::http::headers::CONTENT_TYPE),
            Some(request_header_names::QUERY_CONTENT_TYPE),
            "{label}: Content-Type should be application/query+json"
        );
    }
    // ── compute_execution_context ─────────────────────────────────────

    fn retry_state_with_counts(
        failover_retry_count: u32,
        session_token_retry_count: u32,
    ) -> super::OperationRetryState {
        let mut state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.failover_retry_count = failover_retry_count;
        state.session_token_retry_count = session_token_retry_count;
        state
    }

    #[test]
    fn execution_context_initial_when_no_retries() {
        let state = retry_state_with_counts(0, 0);
        assert!(matches!(
            super::compute_execution_context(&state),
            ExecutionContext::Initial
        ));
    }

    #[test]
    fn execution_context_retry_when_session_retry_active() {
        // Session-retry takes precedence over failover-retry: when both
        // counters are non-zero, the most recent advance was the session
        // retry, so the attempt is annotated as a `Retry`.
        let state = retry_state_with_counts(1, 1);
        assert!(matches!(
            super::compute_execution_context(&state),
            ExecutionContext::Retry
        ));

        let state = retry_state_with_counts(0, 1);
        assert!(matches!(
            super::compute_execution_context(&state),
            ExecutionContext::Retry
        ));
    }

    #[test]
    fn execution_context_region_failover_when_only_failover_active() {
        let state = retry_state_with_counts(1, 0);
        assert!(matches!(
            super::compute_execution_context(&state),
            ExecutionContext::RegionFailover
        ));
    }

    // ── apply_hub_region_header ──────────────────────────────────────
    //
    // See HUB_REGION_PROCESSING_HEADER_SPEC.md §3.4 / public-spec §4.2.
    // The emission logic itself is a 4-line conditional; these tests
    // exercise both branches so AC-1/AC-5 don't drift on a refactor.

    fn build_minimal_transport_request() -> super::TransportRequest {
        let operation = CosmosOperation::read_all_databases(test_account());
        let routing = test_routing();
        let activity_id = ActivityId::from_string("hub-region-test".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
        };
        build_transport_request(&operation, None, &ctx).expect("request should build")
    }

    /// T-6 — When the latch is set on `retry_state`, the helper emits
    /// `x-ms-cosmos-hub-region-processing-only: True` on the transport
    /// request (AC-1).
    #[test]
    fn transport_request_emits_hub_region_header_when_latched() {
        let mut request = build_minimal_transport_request();
        let mut state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.is_dataplane = true;
        state.hub_region_processing_only = true;

        super::apply_hub_region_header(&mut request, &state);

        let value = request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::HUB_REGION_PROCESSING_ONLY,
        ));
        assert_eq!(value, Some("True"));
    }

    /// T-7 — When the latch is NOT set, the helper does not emit the
    /// header. Covers AC-5 / cross-operation isolation guarantee at the
    /// emission layer.
    #[test]
    fn transport_request_omits_hub_region_header_when_not_latched() {
        let mut request = build_minimal_transport_request();
        let state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        assert!(!state.hub_region_processing_only);

        super::apply_hub_region_header(&mut request, &state);

        let value = request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::HUB_REGION_PROCESSING_ONLY,
        ));
        assert!(
            value.is_none(),
            "hub-region header must not be present when latch is unset, got {value:?}",
        );
    }

    // ── apply_failover_delay ──────────────────────────────────────────

    #[tokio::test]
    async fn failover_delay_none_returns_immediately() {
        let start = std::time::Instant::now();
        super::apply_failover_delay(None).await;
        // Allow generous slack for CI scheduling jitter; the goal is to
        // confirm that `None` does not invoke the sleep path at all.
        assert!(start.elapsed() < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn failover_delay_zero_returns_immediately() {
        let start = std::time::Instant::now();
        super::apply_failover_delay(Some(Duration::ZERO)).await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }

    #[tokio::test(start_paused = true)]
    async fn failover_delay_real_value_actually_sleeps() {
        // Use tokio's pause-time to verify the sleep path is taken
        // without making the test wall-clock-slow.
        let start = tokio::time::Instant::now();
        super::apply_failover_delay(Some(Duration::from_secs(5))).await;
        assert!(start.elapsed() >= Duration::from_secs(5));
    }

    // ── enforce_deadline_or_timeout ───────────────────────────────────

    fn empty_options_view() -> crate::options::OperationOptionsView<'static> {
        crate::options::OperationOptionsView::new(None, None, None, None)
    }

    fn test_diagnostics() -> crate::diagnostics::DiagnosticsContextBuilder {
        crate::diagnostics::DiagnosticsContextBuilder::new(
            crate::models::ActivityId::from_string("test-deadline".to_owned()),
            std::sync::Arc::new(crate::options::DiagnosticsOptions::default()),
        )
    }

    #[test]
    fn enforce_deadline_none_is_ok() {
        let options = empty_options_view();
        let mut diagnostics = test_diagnostics();
        let result = super::enforce_deadline_or_timeout(None, &options, &mut diagnostics);
        assert!(result.is_ok());
    }

    #[test]
    fn enforce_deadline_in_future_is_ok() {
        let options = empty_options_view();
        let mut diagnostics = test_diagnostics();
        let deadline = std::time::Instant::now() + Duration::from_secs(60);
        let result = super::enforce_deadline_or_timeout(Some(deadline), &options, &mut diagnostics);
        assert!(result.is_ok());
    }

    #[test]
    fn enforce_deadline_in_past_returns_timeout_error() {
        let options = empty_options_view();
        let mut diagnostics = test_diagnostics();
        let deadline = std::time::Instant::now() - Duration::from_millis(1);
        let result = super::enforce_deadline_or_timeout(Some(deadline), &options, &mut diagnostics);
        let err = result.expect_err("past deadline should produce an error");
        assert!(matches!(err.kind(), azure_core::error::ErrorKind::Other));
        let msg = err.to_string();
        assert!(
            msg.contains("end-to-end operation timeout exceeded"),
            "unexpected error message: {msg}"
        );
    }

    // ── classify_hedge_result (Part 4b) ────────────────────────────────

    fn http_result(status_code: u16, sub_status: Option<u32>) -> super::TransportResult {
        use azure_core::http::{headers::Headers, StatusCode};
        let mut status = crate::models::CosmosStatus::new(StatusCode::from(status_code));
        if let Some(v) = sub_status {
            status = status.with_sub_status(v);
        }
        super::TransportResult::from_http_response(
            status,
            Headers::new(),
            crate::models::CosmosResponseHeaders::default(),
            Vec::new(),
        )
    }

    #[test]
    fn classify_hedge_result_success_is_final() {
        let tr = http_result(200, None);
        assert!(matches!(
            super::classify_hedge_result(Ok(tr)),
            super::HedgeClass::Final(_)
        ));
    }

    #[test]
    fn classify_hedge_result_409_conflict_is_final() {
        // 409 is a final HTTP error per spec §7.1 — terminates hedging.
        let tr = http_result(409, None);
        assert!(matches!(
            super::classify_hedge_result(Ok(tr)),
            super::HedgeClass::Final(_)
        ));
    }

    #[test]
    fn classify_hedge_result_503_is_transient() {
        // 503 ServiceUnavailable is transient — keeps the other side racing.
        let tr = http_result(503, None);
        assert!(matches!(
            super::classify_hedge_result(Ok(tr)),
            super::HedgeClass::Transient
        ));
    }

    #[test]
    fn classify_hedge_result_404_1002_is_transient() {
        // 404/1002 ReadSessionNotAvailable is transient (§7.2).
        let tr = http_result(404, Some(1002));
        assert!(matches!(
            super::classify_hedge_result(Ok(tr)),
            super::HedgeClass::Transient
        ));
    }

    #[test]
    fn classify_hedge_result_deadline_exceeded_is_transient() {
        let tr =
            super::TransportResult::deadline_exceeded(crate::diagnostics::RequestSentStatus::Sent);
        assert!(matches!(
            super::classify_hedge_result(Ok(tr)),
            super::HedgeClass::Transient
        ));
    }

    #[test]
    fn classify_hedge_result_request_build_error_is_transient() {
        let err = azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "synthetic build error",
        );
        assert!(matches!(
            super::classify_hedge_result(Err(err)),
            super::HedgeClass::Transient
        ));
    }

    // ── finalize_hedge_attempt (Part 4b) ──────────────────────────────

    #[test]
    fn finalize_hedge_attempt_http_error_returns_error_with_status() {
        let tr = Box::new(http_result(409, None));
        let diagnostics = test_diagnostics();
        let err = super::finalize_hedge_attempt(tr, diagnostics)
            .expect_err("409 should be surfaced as an error");
        match err.kind() {
            azure_core::error::ErrorKind::HttpResponse { status, .. } => {
                assert_eq!(u16::from(*status), 409);
            }
            other => panic!("expected HttpResponse error kind, got {other:?}"),
        }
    }

    #[test]
    fn finalize_hedge_attempt_deadline_returns_other_error() {
        let tr = Box::new(super::TransportResult::deadline_exceeded(
            crate::diagnostics::RequestSentStatus::Sent,
        ));
        let diagnostics = test_diagnostics();
        let err = super::finalize_hedge_attempt(tr, diagnostics)
            .expect_err("deadline should produce an error");
        assert!(matches!(err.kind(), azure_core::error::ErrorKind::Other));
        assert!(err.to_string().contains("deadline exceeded"));
    }

    // ── DiagnosticsContextBuilder hedge helpers (Part 4b) ─────────────

    #[test]
    fn diagnostics_clone_for_hedge_attempt_starts_empty() {
        let parent = test_diagnostics();
        let child = parent.clone_for_hedge_attempt();
        // A fresh sub-builder must not carry the parent's request list,
        // status, or accumulated hedge diagnostics.
        assert_eq!(child.request_count(), 0);
    }

    #[test]
    fn diagnostics_merge_hedge_attempt_absorbs_requests() {
        use crate::diagnostics::{TransportHttpVersion, TransportKind};

        let mut parent = test_diagnostics();
        let mut child = parent.clone_for_hedge_attempt();

        let endpoint = crate::driver::routing::CosmosEndpoint::global(
            url::Url::parse("https://acct.example/").unwrap(),
        );
        let _ = child.start_request(
            super::ExecutionContext::Hedging,
            super::PipelineType::DataPlane,
            super::TransportSecurity::Secure,
            TransportKind::Gateway,
            TransportHttpVersion::Http11,
            &endpoint,
        );

        assert_eq!(child.request_count(), 1);
        assert_eq!(parent.request_count(), 0);

        parent.merge_hedge_attempt(child);
        // After merge the parent reflects the absorbed request and the
        // child is consumed.
        assert_eq!(parent.request_count(), 1);
    }

    // ── application-cancel harvest (Part 4c) ───────────────────────────

    #[test]
    fn deadline_elapsed_none_is_false() {
        assert!(!super::deadline_elapsed(None));
    }

    #[test]
    fn deadline_elapsed_future_is_false() {
        let d = std::time::Instant::now() + Duration::from_secs(60);
        assert!(!super::deadline_elapsed(Some(d)));
    }

    #[test]
    fn deadline_elapsed_past_is_true() {
        let d = std::time::Instant::now() - Duration::from_millis(1);
        assert!(super::deadline_elapsed(Some(d)));
    }

    #[test]
    fn application_cancelled_error_is_other_kind_with_app_cancel_message() {
        let err = super::application_cancelled_error();
        assert!(matches!(err.kind(), azure_core::error::ErrorKind::Other));
        let msg = err.to_string();
        assert!(
            msg.contains("cancelled by application deadline"),
            "unexpected error message: {msg}"
        );
    }

    #[test]
    fn harvest_window_is_50ms() {
        // Load-bearing spec constant — surfaced as a test so the
        // hedging_spec.md §6.5 #7 / §14.2 contract is enforced.
        assert_eq!(super::HARVEST_WINDOW, Duration::from_millis(50));
    }

    #[tokio::test]
    async fn deadline_signal_none_does_not_complete() {
        // A `None` deadline must produce a never-resolving future so
        // `select` against deadline collapses to whatever the other arm
        // is awaiting (zero deadline observation overhead when unset).
        let fut = super::deadline_signal(None);
        let timer = Box::pin(azure_core::sleep(
            azure_core::time::Duration::try_from(Duration::from_millis(20)).unwrap(),
        ));
        match futures::future::select(fut, timer).await {
            futures::future::Either::Right(((), _)) => { /* expected */ }
            futures::future::Either::Left(((), _)) => {
                panic!("deadline_signal(None) must never resolve");
            }
        }
    }

    #[tokio::test]
    async fn deadline_signal_past_resolves_immediately() {
        let past = std::time::Instant::now() - Duration::from_millis(10);
        let fut = super::deadline_signal(Some(past));
        // A short timer that should NOT win this race.
        let timer = Box::pin(azure_core::sleep(
            azure_core::time::Duration::try_from(Duration::from_millis(50)).unwrap(),
        ));
        match futures::future::select(fut, timer).await {
            futures::future::Either::Left(((), _)) => { /* expected */ }
            futures::future::Either::Right(((), _)) => {
                panic!("deadline_signal(past) must resolve before a 50ms sleep");
            }
        }
    }

    #[tokio::test]
    async fn harvest_remaining_attempt_merges_diagnostics_when_attempt_completes_in_window() {
        // Simulates a hedge attempt that produces a result quickly after
        // the app-cancel deadline fires — the harvest window MUST capture
        // its diagnostics into the parent.
        use crate::diagnostics::{TransportHttpVersion, TransportKind};

        let mut parent = test_diagnostics();
        let mut child = parent.clone_for_hedge_attempt();
        let endpoint = crate::driver::routing::CosmosEndpoint::global(
            url::Url::parse("https://acct.example/").unwrap(),
        );
        let _ = child.start_request(
            super::ExecutionContext::Hedging,
            super::PipelineType::DataPlane,
            super::TransportSecurity::Secure,
            TransportKind::Gateway,
            TransportHttpVersion::Http11,
            &endpoint,
        );

        // The "attempt" completes immediately (well within HARVEST_WINDOW).
        let attempt = Box::pin(async move {
            (
                Err::<super::TransportResult, _>(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "synthetic transport error",
                )),
                child,
            )
        });

        super::harvest_remaining_attempt(attempt, &mut parent).await;
        assert_eq!(parent.request_count(), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn harvest_remaining_attempt_drops_attempt_when_window_exceeded() {
        // A hedge attempt that never completes within HARVEST_WINDOW
        // must be dropped — the parent diagnostics MUST NOT be mutated.
        use crate::diagnostics::{TransportHttpVersion, TransportKind};

        let mut parent = test_diagnostics();
        let mut child = parent.clone_for_hedge_attempt();
        let endpoint = crate::driver::routing::CosmosEndpoint::global(
            url::Url::parse("https://acct.example/").unwrap(),
        );
        let _ = child.start_request(
            super::ExecutionContext::Hedging,
            super::PipelineType::DataPlane,
            super::TransportSecurity::Secure,
            TransportKind::Gateway,
            TransportHttpVersion::Http11,
            &endpoint,
        );

        // The "attempt" sleeps far beyond HARVEST_WINDOW.
        let attempt = Box::pin(async move {
            azure_core::sleep(
                azure_core::time::Duration::try_from(Duration::from_secs(60)).unwrap(),
            )
            .await;
            (
                Err::<super::TransportResult, _>(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "should not reach here",
                )),
                child,
            )
        });

        // Drive the test clock past HARVEST_WINDOW.
        let parent_before = parent.request_count();
        super::harvest_remaining_attempt(attempt, &mut parent).await;
        assert_eq!(parent.request_count(), parent_before);
    }

    // ── Shared hub-region latch (Part 5 / HEDGING_SPEC.md §9.6) ───────

    /// T-S5 — Eligibility predicate: data-plane + single-master → build.
    #[test]
    fn shared_hub_region_latch_eligibility_dataplane_single_master() {
        assert!(super::should_build_shared_hub_region_latch(
            super::PipelineType::DataPlane,
            false, // single-master
        ));
    }

    /// T-S6 — Eligibility predicate: multi-master → skip. Mirrors AC-4.
    #[test]
    fn shared_hub_region_latch_eligibility_skip_multi_master() {
        assert!(!super::should_build_shared_hub_region_latch(
            super::PipelineType::DataPlane,
            true, // multi-master
        ));
    }

    /// T-S7 — Eligibility predicate: metadata pipeline → skip. Mirrors
    /// AC-8 and the §1.5 data-plane scope gate of
    /// `HUB_REGION_PROCESSING_HEADER_SPEC.md`.
    #[test]
    fn shared_hub_region_latch_eligibility_skip_metadata() {
        assert!(!super::should_build_shared_hub_region_latch(
            super::PipelineType::Metadata,
            false,
        ));
        assert!(!super::should_build_shared_hub_region_latch(
            super::PipelineType::Metadata,
            true,
        ));
    }

    /// T-S8 — Emission helper: per-state latch alone emits.
    #[test]
    fn should_emit_hub_region_header_per_state_only() {
        assert!(super::should_emit_hub_region_header(true, None));
    }

    /// T-S9 — Emission helper: shared latch alone (Acquire-load `true`)
    /// emits, even when per-state latch is `false`. This is the new
    /// cross-hedge propagation rule from PR #5815.
    #[test]
    fn should_emit_hub_region_header_shared_only() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };
        let shared = Arc::new(AtomicBool::new(true));
        assert!(super::should_emit_hub_region_header(false, Some(&shared)));
        // Sanity-check the atomic ordering pairing.
        assert!(shared.load(Ordering::Acquire));
    }

    /// T-S10 — Emission helper: neither latch set → no header. Mirrors
    /// AC-5 / `shared_hub_region_latch_no_1002_emits_no_header` from
    /// spec §15.1.
    #[test]
    fn should_emit_hub_region_header_neither_latched() {
        use std::sync::{atomic::AtomicBool, Arc};
        let shared = Arc::new(AtomicBool::new(false));
        assert!(!super::should_emit_hub_region_header(false, None));
        assert!(!super::should_emit_hub_region_header(false, Some(&shared)));
    }

    /// T-S11 — `apply_hub_region_header` emits the header when only the
    /// shared latch is `true` on the retry state. Mirrors the PR #5815
    /// `CrossRegionAvailabilityContext_PropagatesHubHeaderFlagToHedgedRequests`
    /// test at the emission layer.
    #[test]
    fn apply_hub_region_header_emits_when_only_shared_latch_set() {
        use std::sync::{atomic::AtomicBool, Arc};

        let mut request = build_minimal_transport_request();
        let shared = Arc::new(AtomicBool::new(true));
        let mut state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.is_dataplane = true;
        state = state.with_shared_hub_region_latch(shared);
        assert!(!state.hub_region_processing_only);

        super::apply_hub_region_header(&mut request, &state);

        let value = request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::HUB_REGION_PROCESSING_ONLY,
        ));
        assert_eq!(value, Some("True"));
    }

    /// T-S12 — `apply_hub_region_header` omits the header when the
    /// shared latch is attached but its bool is `false`. Defends
    /// against an over-eager "present means set" emission rule.
    #[test]
    fn apply_hub_region_header_omits_when_shared_latch_present_but_false() {
        use std::sync::{atomic::AtomicBool, Arc};

        let mut request = build_minimal_transport_request();
        let shared = Arc::new(AtomicBool::new(false));
        let mut state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.is_dataplane = true;
        state = state.with_shared_hub_region_latch(shared);

        super::apply_hub_region_header(&mut request, &state);

        let value = request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::HUB_REGION_PROCESSING_ONLY,
        ));
        assert!(value.is_none());
    }

    // -----------------------------------------------------------------
    // Part 6 — PPCB hedge-win feedback (spec §9.5).
    // Tests the `HedgeOutcomeRecorder` dispatch performed by
    // `record_hedge_outcome` using a Vec-backed fake recorder. The
    // real `LocationStateStore` impl is tracing-only today; once the
    // PPCB-side counter lands (sub-plan 6c), spec §15.2/§15.3
    // integration tests replace these unit tests as the authoritative
    // PPCB-feedback verification.
    // -----------------------------------------------------------------

    #[derive(Debug, Default)]
    struct FakeHedgeRecorder {
        events: std::sync::Mutex<Vec<(super::HedgeOutcome, String, Option<String>)>>,
    }

    impl FakeHedgeRecorder {
        fn drain(&self) -> Vec<(super::HedgeOutcome, String, Option<String>)> {
            std::mem::take(&mut *self.events.lock().unwrap())
        }
    }

    impl super::HedgeOutcomeRecorder for FakeHedgeRecorder {
        fn record_consecutive_hedge_win(
            &self,
            partition: &super::PartitionKeyRangeId,
            primary_region: Option<&crate::options::Region>,
        ) {
            self.events.lock().unwrap().push((
                super::HedgeOutcome::AlternateWin,
                partition.as_str().to_owned(),
                primary_region.map(|r| r.as_str().to_owned()),
            ));
        }

        fn record_primary_win(
            &self,
            partition: &super::PartitionKeyRangeId,
            primary_region: Option<&crate::options::Region>,
        ) {
            self.events.lock().unwrap().push((
                super::HedgeOutcome::PrimaryWin,
                partition.as_str().to_owned(),
                primary_region.map(|r| r.as_str().to_owned()),
            ));
        }
    }

    /// T-P1 (spec §15.1: record_hedge_win_increments_ppcb_counter).
    /// An alternate-region win dispatches exactly one
    /// `record_consecutive_hedge_win` call against the primary partition.
    #[test]
    fn alternate_win_dispatches_consecutive_hedge_win() {
        let recorder = FakeHedgeRecorder::default();
        let pk_range: super::PartitionKeyRangeId = "0".parse().unwrap();
        let region = crate::options::Region::WEST_US_2;

        super::record_hedge_outcome(
            &recorder,
            super::HedgeOutcome::AlternateWin,
            Some(&pk_range),
            Some(&region),
        );

        let events = recorder.drain();
        assert_eq!(
            events,
            vec![(
                super::HedgeOutcome::AlternateWin,
                "0".to_owned(),
                Some("westus2".to_owned()),
            )]
        );
    }

    /// T-P2 (spec §15.1: primary_win_resets_hedge_win_counter).
    /// A primary-region win dispatches exactly one `record_primary_win`
    /// call (which, on the real recorder, would reset the
    /// consecutive-hedge-win counter per §9.5 invariant #2).
    #[test]
    fn primary_win_dispatches_record_primary_win() {
        let recorder = FakeHedgeRecorder::default();
        let pk_range: super::PartitionKeyRangeId = "5".parse().unwrap();
        let region = crate::options::Region::EAST_US;

        super::record_hedge_outcome(
            &recorder,
            super::HedgeOutcome::PrimaryWin,
            Some(&pk_range),
            Some(&region),
        );

        let events = recorder.drain();
        assert_eq!(
            events,
            vec![(
                super::HedgeOutcome::PrimaryWin,
                "5".to_owned(),
                Some("eastus".to_owned()),
            )]
        );
    }

    /// T-P3. Without a partition key range ID (first attempt against an
    /// unresolved item — `retry_state.partition_key_range_id == None`),
    /// the helper is a no-op for both outcomes. Spec §9.5 keys feedback
    /// by `(partition, primary_region)`; without a partition there is
    /// nothing to attribute to.
    #[test]
    fn record_hedge_outcome_noop_when_partition_missing() {
        let recorder = FakeHedgeRecorder::default();
        let region = crate::options::Region::WEST_US_2;

        super::record_hedge_outcome(
            &recorder,
            super::HedgeOutcome::AlternateWin,
            None,
            Some(&region),
        );
        super::record_hedge_outcome(
            &recorder,
            super::HedgeOutcome::PrimaryWin,
            None,
            Some(&region),
        );

        assert!(recorder.drain().is_empty());
    }

    /// T-P4. Default-endpoint accounts whose snapshot carries no named
    /// region still attribute feedback to the partition — the
    /// `primary_region` argument is `Option` precisely to handle that
    /// case (spec §9.5: the counter key is
    /// `(partition, primary_region)` with `primary_region` allowed to
    /// be `None`).
    #[test]
    fn record_hedge_outcome_dispatches_when_primary_region_missing() {
        let recorder = FakeHedgeRecorder::default();
        let pk_range: super::PartitionKeyRangeId = "7".parse().unwrap();

        super::record_hedge_outcome(
            &recorder,
            super::HedgeOutcome::AlternateWin,
            Some(&pk_range),
            None,
        );

        let events = recorder.drain();
        assert_eq!(
            events,
            vec![(super::HedgeOutcome::AlternateWin, "7".to_owned(), None)]
        );
    }

    /// T-P5. The production `LocationStateStore` impl of
    /// `HedgeOutcomeRecorder` is reachable through trait dispatch (i.e.
    /// the trait impl in `operation_pipeline.rs` compiles and the
    /// inherent stubs on `LocationStateStore` are visible). We only
    /// type-check this here — the actual stubs are tracing-only no-ops
    /// pending PPCB-side co-design (sub-plan 6c).
    #[test]
    fn location_state_store_implements_hedge_outcome_recorder() {
        fn assert_impl<T: super::HedgeOutcomeRecorder + ?Sized>() {}
        assert_impl::<super::LocationStateStore>();
    }
}
