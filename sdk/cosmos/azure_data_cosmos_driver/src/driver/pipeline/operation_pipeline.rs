// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation pipeline: the core loop for executing Cosmos DB operations.
//!
//! Implements the 7-stage operation loop with multi-region failover,
//! session retry, endpoint unavailability tracking, partition-level failover
//! (PPAF/PPCB), and deadline enforcement.

use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};

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
        OperationOptionsView, ReadConsistencyStrategy, Region, ThroughputControlGroupSnapshot,
    },
};

use super::{
    components::{
        OperationAction, OperationRetryState, RoutingDecision, TransportMode, TransportOutcome,
        TransportRequest, TransportResult,
    },
    retry_evaluation::{
        evaluate_transport_result, is_region_confirming_status, partition_effects_for_deferral,
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
            pipeline_type == PipelineType::DataPlane,
            location_state_store.endpoint_unavailability_ttl(),
        );

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
            if let Some(headers) = result.response_headers() {
                if let Some(pk_range_id) = headers.get_optional_string(&HeaderName::from_static(
                    "x-ms-documentdb-partitionkeyrangeid",
                )) {
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

        // ── STAGE 6: Apply location effects ────────────────────────────
        // Write-path effects are deferred into
        // `retry_state.pending_write_effects` instead of being applied
        // immediately. They are flushed only when the write definitively
        // reaches a region (Complete, or Abort with a region-confirming
        // status such as 409 Conflict). This prevents transient retry
        // failures from polluting the routing state with unverified
        // partition or endpoint outages — critical for PPAF on single-master
        // accounts where prematurely marking the only known write region
        // unavailable would force an unnecessary cross-region failover.
        // Read-path effects are applied immediately so PPCB read counters
        // can drive threshold-based failover.
        let (immediate_effects, deferred_effects) = partition_effects_for_deferral(
            operation.is_read_only(),
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

        if is_eligible_for_ppcb(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.circuit_breaker_overrides.get(pk_range_id) {
                if entry.health_status == HealthStatus::ProbeCandidate {
                    // Route probe request to the original (first failed) endpoint.
                    return make_partition_routing(entry.first_failed_endpoint.clone());
                }
                if can_circuit_breaker_trigger_failover(entry, is_read, &partitions.config) {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        } else if is_eligible_for_ppaf(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.failover_overrides.get(pk_range_id) {
                // PPAF overrides do not use probe-based failback (no ProbeCandidate
                // handling). The override persists until the backend signals a change.
                return make_partition_routing(entry.current_endpoint.clone());
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
}
