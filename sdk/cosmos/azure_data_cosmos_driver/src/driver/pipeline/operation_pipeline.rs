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
    driver::{
        routing::{
            can_circuit_breaker_trigger_failover, is_eligible_for_ppaf, is_eligible_for_ppcb,
            partition_endpoint_state::HealthStatus, partition_key_range_id::PartitionKeyRangeId,
            remove_probe_succeeded_entry, session_manager::SessionManager, AccountEndpointState,
            CosmosEndpoint, LocationEffect, LocationSnapshot, LocationStateStore,
        },
        transport::CosmosTransport,
    },
    models::{
        cosmos_headers::QUERY_CONTENT_TYPE, effective_partition_key::EffectivePartitionKey,
        request_header_names, AccountEndpoint, ActivityId, CosmosOperation, CosmosResponse,
        Credential, DefaultConsistencyLevel, OperationType, SessionToken, SubStatusCode,
    },
    options::{
        resolve_effective_consistency, HedgeThreshold, OperationOptionsView,
        ReadConsistencyStrategy, Region, ResolvedThroughputControl,
    },
};

use super::{
    components::{
        OperationAction, OperationRetryState, RoutingDecision, TransportMode, TransportOutcome,
        TransportRequest, TransportResult, DATA_PLANE_MAX_PER_RETRY_DELAY,
        DATA_PLANE_MAX_THROTTLE_ATTEMPTS, DATA_PLANE_MAX_THROTTLE_WAIT,
        METADATA_MAX_PER_RETRY_DELAY, METADATA_MAX_THROTTLE_ATTEMPTS, METADATA_MAX_THROTTLE_WAIT,
    },
    hedging_diagnostics::{HedgeDiagnostics, HedgingStrategyConfig},
    hedging_eligibility::evaluate_hedge_eligibility,
    retry_evaluation::{
        build_service_error, evaluate_hedge_leg_effects, evaluate_transport_result,
        is_region_confirming_status, partition_effects_for_deferral,
    },
};

use crate::driver::transport::{
    is_operation_supported_by_gateway_v2,
    transport_pipeline::{execute_transport_pipeline, TransportPipelineContext},
    AuthorizationContext, EndpointKey,
};

/// Default throttle-retry budget for a pipeline class when the caller hasn't
/// configured `ThrottlingRetryOptions`: the maximum number of 429 retries, the
/// cumulative-wait budget, and the per-retry delay cap ("interval"). Data-plane
/// gets more retries at a longer interval (count-limited); metadata keeps the
/// patient, shorter-interval budget.
fn default_throttle_budget(pipeline_type: PipelineType) -> (u32, Duration, Duration) {
    if pipeline_type.is_data_plane() {
        (
            DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
            DATA_PLANE_MAX_THROTTLE_WAIT,
            DATA_PLANE_MAX_PER_RETRY_DELAY,
        )
    } else {
        (
            METADATA_MAX_THROTTLE_ATTEMPTS,
            METADATA_MAX_THROTTLE_WAIT,
            METADATA_MAX_PER_RETRY_DELAY,
        )
    }
}

/// Per-request overrides that take precedence over values from [`CosmosOperation`].
///
/// Used by the dataflow pipeline to inject routing and pagination state that
/// varies per physical partition or per page, without mutating the shared
/// `CosmosOperation`. Each field, when `Some`, emits the corresponding request
/// header in [`OperationOverrides::apply_headers`].
#[derive(Debug, Clone, Default)]
pub(crate) struct OperationOverrides {
    /// Feed range to constrain the request to (emits `x-ms-start-epk` / `x-ms-end-epk`).
    pub feed_range: Option<crate::models::FeedRange>,

    /// Physical pkrange bounds (full EPK span of the resolved partition). Carried
    /// to the GW_V2 dispatcher via internal `x-ms-thinclient-pkrange-min`/`-max`
    /// headers so it can derive `StartEpkHash`/`EndEpkHash` RNTBD tokens for the
    /// full-pkrange XPK case without emitting the public EPK headers (which the
    /// legacy gateway rejects when paired with `partitionkeyrangeid`).
    pub pkrange_bounds: Option<crate::models::FeedRange>,

    /// Physical partition key range ID (emits `x-ms-documentdb-partitionkeyrangeid`).
    pub partition_key_range_id: Option<String>,

    /// Logical partition key (emits `x-ms-documentdb-partitionkey`).
    pub partition_key: Option<crate::models::PartitionKey>,

    /// Continuation token for pagination (emits `x-ms-continuation`).
    pub continuation: Option<String>,
}

impl OperationOverrides {
    /// Applies the override headers to the given header map.
    ///
    /// Headers set here take precedence over any previously-set values for
    /// the same header name (they overwrite on conflict).
    ///
    /// When `continuation_as_if_none_match` is `true` (change feed reads), the
    /// continuation token is emitted as the `If-None-Match` header instead of
    /// `x-ms-continuation`, since the change feed carries its continuation via
    /// the ETag/`If-None-Match` mechanism.
    pub fn apply_headers(
        &self,
        headers: &mut azure_core::http::headers::Headers,
        continuation_as_if_none_match: bool,
    ) -> crate::error::Result<()> {
        if let Some(feed_range) = &self.feed_range {
            // Narrowed-range XPK case (range < pkrange) AND scoped reads via
            // `FeedRange`. These public EPK headers are honored by Gateway 2.0;
            // the standard gateway rejects them when paired with
            // `partitionkeyrangeid` (HTTP 400, regardless of the min bound —
            // verified against live accounts), so the full-pkrange XPK fan-out
            // path uses the internal `pkrange_bounds` headers (below) instead
            // and leaves these absent.
            headers.insert(
                HeaderName::from_static(request_header_names::START_EPK),
                HeaderValue::from(feed_range.min_inclusive().to_hex()),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::END_EPK),
                HeaderValue::from(feed_range.max_exclusive().to_hex()),
            );
            // `x-ms-start-epk`/`x-ms-end-epk` describe an effective-partition-key
            // *range*, so the key type must be `EffectivePartitionKeyRange`. The
            // point value `EffectivePartitionKey` is rejected by the gateway with
            // `400 "One of the input values is invalid"` for range-scoped requests
            // (issues #4680 and #4681).
            headers.insert(
                HeaderName::from_static(request_header_names::READ_FEED_KEY_TYPE),
                HeaderValue::from_static(request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE),
            );
        }

        if let Some(bounds) = &self.pkrange_bounds {
            // Internal-only headers consumed by the GW_V2 dispatcher to
            // synthesize `StartEpkHash`/`EndEpkHash` RNTBD tokens for the
            // full-pkrange XPK case. Legacy gateway ignores unknown headers.
            headers.insert(
                HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MIN),
                HeaderValue::from(bounds.min_inclusive().to_hex()),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MAX),
                HeaderValue::from(bounds.max_exclusive().to_hex()),
            );
        }

        if let Some(pk_range_id) = &self.partition_key_range_id {
            headers.insert(
                HeaderName::from_static(request_header_names::PARTITION_KEY_RANGE_ID),
                HeaderValue::from(pk_range_id.clone()),
            );
        }

        if let Some(pk) = &self.partition_key {
            let pk_headers = pk.as_headers()?;
            for (name, value) in pk_headers {
                headers.insert(name, value);
            }
        }

        if let Some(continuation) = &self.continuation {
            let header_name = if continuation_as_if_none_match {
                request_header_names::IF_NONE_MATCH
            } else {
                request_header_names::CONTINUATION
            };
            headers.insert(
                HeaderName::from_static(header_name),
                HeaderValue::from(continuation.clone()),
            );

            // For change feed reads, the per-partition continuation (carried
            // via `If-None-Match`) fully describes the resume position. Any
            // start-from marker the operation set for not-yet-polled partitions
            // (e.g. `If-Modified-Since` for a `PointInTime` start) must not
            // co-exist with it, otherwise the request carries two conflicting
            // position headers. The `Now` start marker (`If-None-Match: *`) is
            // already overwritten above by this same header insert.
            if continuation_as_if_none_match {
                headers.remove(HeaderName::from_static(
                    request_header_names::IF_MODIFIED_SINCE,
                ));
            }
        }

        Ok(())
    }
}

/// Executes a Cosmos DB operation through the new pipeline architecture.
///
/// This is the entry point called by `CosmosDriver::execute_operation`.
/// It orchestrates the 7-stage operation loop.
///
/// When `pre_resolved_pk_range_id` is `Some`, it is used to seed the
/// `OperationRetryState` so that partition-level failover overrides (PPAF/PPCB)
/// can take effect from the very first attempt.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    overrides: OperationOverrides,
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
    throughput_control: Option<ResolvedThroughputControl>,
    pre_resolved_pk_range_id: Option<PartitionKeyRangeId>,
) -> crate::error::Result<CosmosResponse> {
    let mut diagnostics = diagnostics;
    let location_snapshot = location_state_store.snapshot();
    let max_failover_retries = options.max_failover_retry_count().copied().unwrap_or(3);

    // Throttle (HTTP 429) retry limits, resolved from the effective operation
    // options. These are the analogs of the .NET SDK's
    // `MaxRetryAttemptsOnRateLimitedRequests` /
    // `MaxRetryWaitTimeOnRateLimitedRequests`. They are forwarded to the
    // transport pipeline, which owns the 429 retry loop. `0` attempts disables
    // throttle retries.
    //
    // **Scope note**: each value is forwarded as a *per-transport-pipeline*
    // budget — `execute_transport_pipeline` is invoked once per attempt that
    // this operation pipeline makes (e.g., per region during failover or per
    // leg during hedging), and each invocation starts with a fresh
    // throttle-retry budget. Total wall-clock cost across an operation is
    // bounded by `end_to_end_latency_policy` (see the per-attempt deadline
    // wiring below), not by these knobs in aggregate.
    let throttling_retry_options = options.throttling_retry_options();
    let (default_attempts, default_wait, max_throttle_per_retry_delay) =
        default_throttle_budget(pipeline_type);
    let max_throttle_attempts = throttling_retry_options
        .max_retry_count()
        .copied()
        .unwrap_or(default_attempts);
    let max_throttle_wait_time = throttling_retry_options
        .max_retry_wait_time()
        .copied()
        .unwrap_or(default_wait);

    // Determine if session consistency is active for this operation.
    let session_capturing_disabled = options
        .session_capturing_disabled()
        .copied()
        .unwrap_or(false);
    let read_consistency_strategy = options
        .read_consistency_strategy()
        .copied()
        .unwrap_or(ReadConsistencyStrategy::Default);
    let effective_consistency =
        resolve_effective_consistency(read_consistency_strategy, account_default_consistency);
    let session_consistency_active = !session_capturing_disabled
        && read_consistency_strategy.is_session_effective(account_default_consistency);

    // Rule 4 (RCS validation): GlobalStrong is
    // valid only on reads against accounts whose default consistency is Strong.
    // For writes or non-Strong accounts, server-side semantics would not be
    // applied — fail fast client-side with BadRequest before incurring a round
    // trip.
    if matches!(
        read_consistency_strategy,
        ReadConsistencyStrategy::GlobalStrong
    ) && operation.is_read_only()
        && account_default_consistency != DefaultConsistencyLevel::Strong
    {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
            .with_message(
                "ReadConsistencyStrategy::GlobalStrong is only valid against accounts whose \
                 default consistency level is Strong",
            )
            .build());
    }
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

    // The hedge threshold is `min(1000ms, request_timeout / 2)` where
    // `request_timeout` is the **configured** end-to-end latency budget,
    // not the time remaining until the deadline. We compute the
    // configured timeout once here and pass it (not the remaining
    // duration) into `evaluate_hedge_eligibility` / `maybe_upgrade_to_hedge`
    // below so the threshold stays stable across retry-driven upgrades.
    let configured_request_timeout = options.end_to_end_latency_policy().map(|p| p.timeout());
    let deadline = configured_request_timeout.map(|t| Instant::now() + t);

    loop {
        // ── STAGE 1: Acquire LocationSnapshot ──────────────────────────
        let location = location_state_store.snapshot();

        // ── STAGE 2: Resolve endpoint ──────────────────────────────────
        // The Gateway 2.0 RNTBD `GlobalDatabaseAccountName` token must carry
        // the global account label. Prefer the account metadata `id` (Java
        // parity), falling back to parsing the customer-provided global
        // endpoint hostname when metadata has not synced yet.
        let account_name = location_state_store.global_database_account_name();
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            &location,
            pipeline_type.is_data_plane(),
            account_name.is_some(),
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
        // On the **first** attempt of a hedge-eligible operation, race
        // the primary against the threshold timer from t=0 rather than
        // running it to completion and post-classifying. A slow-but-
        // eventually-successful primary still loses to a fast alternate,
        // and a successful primary that finishes pre-threshold still gets
        // `HedgeDiagnostics::primary_only` attached.
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
                configured_request_timeout,
            ) {
                let attempt_ctx = AttemptContext {
                    operation,
                    overrides: &overrides,
                    custom_headers,
                    transport,
                    account_endpoint,
                    credential,
                    user_agent,
                    activity_id,
                    pipeline_type,
                    transport_security,
                    account_name: account_name.clone(),
                    effective_consistency,
                    read_consistency_strategy,
                    session_manager,
                    session_consistency_active,
                    options,
                    throughput_control,
                    deadline,
                    configured_request_timeout,
                    can_use_multiple_write_locations: retry_state.can_use_multiple_write_locations,
                    // First attempt: no 1002 has been observed yet, so the
                    // per-state latch is `false`. The shared
                    // `Arc<AtomicBool>` is constructed inside
                    // `execute_hedged` only if the threshold elapses and a
                    // secondary spawns, preserving the zero-overhead
                    // happy path.
                    hub_region_processing_only_initial: retry_state.hub_region_processing_only,
                    // First attempt: no response has been captured yet, so
                    // PK range ID is unknown. PPCB feedback is correctly
                    // gated to no-op on `None` (`record_hedge_outcome`).
                    partition_key_range_id: retry_state.partition_key_range_id.clone(),
                    location_state_store,
                };
                // Latch the one-race-per-operation cap before dispatch
                // so the post-`BothTransient` fallback iteration's
                // STAGE 5b suppresses a second hedge upgrade. The
                // `BothTransient` handler below preserves this flag
                // since it mutates only `partition_key_range_id`,
                // `failover_retry_count`, and `location` on
                // `retry_state` — see [`OperationRetryState`].
                retry_state.hedge_already_fired = true;
                match execute_hedged(
                    &attempt_ctx,
                    &routing,
                    &upgrade.secondary_routing,
                    upgrade.threshold,
                    upgrade.strategy_config,
                    diagnostics,
                    &retry_state,
                )
                .await
                {
                    HedgedRaceResult::Terminal(result) => return result,
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
                        // Both legs failed with retriable errors; fall back
                        // into the failover loop against the untried regions.
                        tracing::warn!(
                            activity_id = %activity_id,
                            primary_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                            secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                            "hedge race: both transient on STAGE 2b; attempting failover fallback",
                        );
                        diagnostics = returned_diagnostics;
                        // Fold race-observed PK-range-id and 1002 latch back
                        // into retry_state so the fallback behaves like a
                        // non-hedged attempt that saw the same conditions.
                        if retry_state.partition_key_range_id.is_none() {
                            retry_state.partition_key_range_id = race_pk_range_id;
                        }
                        propagate_hedge_session_unavailable(&mut retry_state, race_observed_1002);
                        if let Err(e) = try_advance_after_both_transient(
                            &mut retry_state,
                            &location,
                            operation.is_read_only(),
                            primary_region.as_ref(),
                            secondary_region.as_ref(),
                            last_error,
                        ) {
                            // Budget exhausted: the race is the terminal
                            // outcome, so stamp the both-transient result
                            // before grafting diagnostics onto the error.
                            // `deadline_elapsed = false`: the budget, not
                            // the deadline, ended the race.
                            diagnostics.set_hedge_diagnostics(HedgeDiagnostics::both_transient(
                                strategy_config,
                                primary_region_for_diag,
                                secondary_region_for_diag,
                                false,
                            ));
                            let diagnostics_ctx = Arc::new(diagnostics.complete());
                            return Err(crate::error::CosmosErrorBuilder::from_error(e)
                                .with_diagnostics(diagnostics_ctx)
                                .build());
                        }
                        diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
                        continue;
                    }
                }
            }
        }

        // ── STAGE 3: Build transport request ───────────────────────────
        let execution_context = compute_execution_context(&retry_state);

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id,
            execution_context,
            deadline,
            effective_consistency,
            read_consistency_strategy: if operation.is_read_only() {
                read_consistency_strategy
            } else {
                ReadConsistencyStrategy::Default
            },
            resolved_session_token: session_consistency_active
                .then(|| {
                    // Scope the session token to the target partition-key-range
                    // only for thin-client (Gateway 2.0) requests: the RNTBD
                    // backend rejects a composite multi-range token on a
                    // single-partition request. Classic gateway accepts the
                    // composite (and maps parent->child across splits), so keep
                    // sending it there to stay read-your-writes safe.
                    let scoped_pk_range_id =
                        if matches!(routing.transport_mode, TransportMode::GatewayV2) {
                            retry_state
                                .partition_key_range_id
                                .as_ref()
                                .map(|id| id.as_str())
                        } else {
                            None
                        };
                    session_manager.resolve_session_token(
                        operation,
                        operation.request_headers().session_token.as_ref(),
                        scoped_pk_range_id,
                    )
                })
                .flatten(),
            throughput_control,
        };
        let mut transport_request =
            build_transport_request(operation, &overrides, custom_headers, &ctx)?;

        // HUB_REGION_PROCESSING_HEADER_SPEC.md §3 / public-spec §3.4:
        // Emit the `x-ms-cosmos-hub-region-processing-only: True` header
        // when the latch is set. The latch is flipped in
        // `try_handle_read_session_not_available` on the first 1002 of a
        // single-master data-plane operation, and is sticky for the
        // remainder of the operation's transport attempts (AC-1, AC-2).
        apply_hub_region_header(&mut transport_request, &retry_state);

        apply_tentative_writes_header(
            &mut transport_request,
            operation,
            location.account.multiple_write_locations_enabled,
        );

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
                endpoint_key: routing.endpoint_key.clone(),
                account_name: account_name.clone(),
                collection_rid: operation.container().map(|c| c.rid().to_owned()),
                max_throttle_attempts,
                max_throttle_wait_time,
                max_throttle_per_retry_delay,
            },
            &mut diagnostics,
        )
        .await;

        // Fallback: capture the partition key range ID from response headers
        // only if pre-resolution (from the request's partition key / EPK range)
        // did not already seed it. Set once, on the first response that carries it.
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
        // When the just-classified action would have advanced to a
        // different region anyway (FailoverRetry / SessionRetry) and the
        // operation is eligible for cross-region hedging with a strategy
        // resolved, replace it with `OperationAction::Hedge` so STAGE 7
        // races primary and secondary in parallel via `execute_hedged()`.
        // If no upgrade applies the original action passes through
        // unchanged — preserving today's sequential-failover semantics
        // for non-hedgeable operations, single-region accounts,
        // env-disabled hedging, and explicit `AvailabilityStrategy::Disabled`.
        //
        // Cap to one hedge race per operation: once a race has been
        // dispatched (via STAGE 2b or a prior STAGE 7 Hedge arm), do
        // not upgrade again. Each race already burns two regions of RU
        // even on the `BothTransient` fallback path; allowing
        // back-to-back upgrades would compound RU consumption without
        // letting the surrounding failover loop make sequential
        // progress against the remaining regions.
        let action = if retry_state.hedge_already_fired {
            action
        } else {
            maybe_upgrade_to_hedge(
                action,
                operation,
                options,
                &location.account,
                &routing,
                configured_request_timeout,
            )
        };

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

                // Hub-region cache populate: when the hub-region-processing-only
                // latch was active and we have a partition key range ID, the
                // region that produced this 2xx is — by definition — the
                // partition's current hub. Cache it so subsequent operations
                // that latch the header skip the 403/3 discovery chain.
                if let Some(pk_range_id) = hub_region_cache_populate_target(&retry_state, operation)
                {
                    // Skip the apply when it would not change state: PPAF off
                    // (the cache effect is a no-op) or the entry already points
                    // at this hub endpoint. Both would only churn a clone + CAS.
                    let partitions = location_state_store.snapshot().partitions;
                    let already_cached = partitions
                        .failover_overrides
                        .get(pk_range_id.as_str())
                        .is_some_and(|entry| entry.current_endpoint == routing.endpoint);
                    if partitions.per_partition_automatic_failover_enabled && !already_cached {
                        location_state_store
                            .apply(&[LocationEffect::CacheHubRegion {
                                partition_key_range_id: pk_range_id,
                                hub_endpoint: routing.endpoint.clone(),
                            }])
                            .await;
                    }
                }

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
                    operation.operation_type().routes_to_write_endpoints(),
                    is_distributed_transaction_operation(operation),
                );
                diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
            }
            OperationAction::InRegionRetry { new_state, delay } => {
                // Same-region retry path used by `try_handle_retry_with`
                // (449 RetryWith). Deliberately does NOT call
                // `advance_to_next_attempt` — we want the next attempt to
                // hit the same endpoint/region. Deferred write-path effects
                // also stay buffered: we haven't proven any region was
                // healthy, so polluting routing state would be premature.
                tracing::debug!(
                    activity_id = %activity_id,
                    retry_with_attempts = new_state
                        .retry_with_state
                        .as_ref()
                        .map(|s| s.attempt_count)
                        .unwrap_or(0),
                    delay = ?delay,
                    "in-region retry triggered",
                );
                apply_failover_delay(Some(delay)).await;
                retry_state = new_state;
                diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
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
                    operation.operation_type().routes_to_write_endpoints(),
                    is_distributed_transaction_operation(operation),
                );
                diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
            }
            #[cfg(feature = "preview_dtx")]
            OperationAction::DtxRetry { new_state, delay } => {
                tracing::debug!(
                    activity_id = %activity_id,
                    delay = ?delay,
                    dtx_coordinator_retries = new_state.dtx_coordinator_retry_count,
                    dtx_infra_retries = new_state.dtx_infra_retry_count,
                    "dtx bodyless retry triggered",
                );
                apply_failover_delay(Some(delay)).await;
                retry_state = new_state;
                diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
            }
            OperationAction::Abort { error } => {
                // Flush deferred write-path effects if the abort status
                // confirms the region processed the request (e.g., 409
                // Conflict, 412 Precondition Failed). On non-confirming
                // aborts (503/429-3092/410/408/403-3/transport error/deadline)
                // the buffered effects are discarded — we never proved any
                // region was actually healthy, so polluting routing state
                // would be wrong.
                let cosmos_status = error.status();
                let confirming = is_region_confirming_status(&cosmos_status);
                if confirming {
                    flush_pending_write_effects(&mut retry_state, location_state_store).await;
                } else {
                    retry_state.pending_write_effects.clear();
                }

                tracing::error!(
                    activity_id = %activity_id,
                    status = ?cosmos_status,
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
                diagnostics
                    .set_operation_status(cosmos_status.status_code(), cosmos_status.sub_status());
                // Graft the completed operation diagnostics (retry history,
                // region attempts, per-request events) onto the error before
                // returning. Without this, callers reading
                // `error.diagnostics()` would see `None` on every aborted
                // operation even though the pipeline tracked everything —
                // the only path that attaches diagnostics in the
                // non-aborted case is `build_cosmos_response`.
                let diagnostics_ctx = Arc::new(diagnostics.complete());
                return Err(crate::error::CosmosErrorBuilder::from_error(error)
                    .with_diagnostics(diagnostics_ctx)
                    .build());
            }
            OperationAction::Hedge {
                secondary_routing: _pre_advance_secondary,
                threshold,
                strategy_config,
                new_state,
            } => {
                // Race the primary attempt against a single cross-region
                // secondary via `execute_hedged`. Returns either a
                // terminal result (return it directly) or a
                // both-transient outcome that we fold back into the
                // failover loop with the next region.
                //
                // We move `diagnostics` into `execute_hedged` because the
                // function takes ownership of the parent builder to merge
                // each hedge attempt's sub-builder back into it and finalize
                // the response (mirroring the `Complete` arm's
                // `build_cosmos_response(result, diagnostics)` shape).
                //
                // Apply `new_state` before building `AttemptContext` so
                // the race inherits the post-transition exclusions, latch,
                // and counters — see `OperationAction::Hedge::new_state`.
                advance_to_next_attempt(
                    &mut retry_state,
                    new_state,
                    location_state_store,
                    operation.is_read_only(),
                    operation.operation_type().routes_to_write_endpoints(),
                    is_distributed_transaction_operation(operation),
                );
                // Re-resolve the primary routing against the advanced
                // retry_state and freshly snapshotted location. The
                // pre-advance `routing` referenced the failed region that
                // triggered the upgrade — racing it again as the hedge's
                // primary would double-pay RU on a known-bad region.
                let location = location_state_store.snapshot();
                let primary_routing = resolve_endpoint(
                    operation,
                    &retry_state,
                    &location,
                    pipeline_type.is_data_plane(),
                    account_name.is_some(),
                    location_state_store.endpoint_unavailability_ttl(),
                );
                // Re-evaluate hedge eligibility against the *post-advance*
                // primary. After `advance_to_next_attempt` rotates the
                // LocationIndex, the new primary may coincide with the
                // pre-selected `_pre_advance_secondary`, which would race
                // both legs in the same region (zero benefit, 2× RU). If
                // no distinct alternate remains, fall back to non-hedged
                // dispatch via `continue` — we intentionally do NOT set
                // `hedge_already_fired` since no race actually started.
                let secondary_routing = match evaluate_hedge_eligibility(
                    operation,
                    options,
                    &location.account,
                    &primary_routing,
                    configured_request_timeout,
                ) {
                    Some(upgrade) => upgrade.secondary_routing,
                    None => {
                        tracing::debug!(
                            activity_id = %activity_id,
                            "STAGE 7 hedge upgrade: no distinct alternate region after \
                             advance_to_next_attempt; falling back to non-hedged dispatch",
                        );
                        continue;
                    }
                };
                let attempt_ctx = AttemptContext {
                    operation,
                    overrides: &overrides,
                    custom_headers,
                    transport,
                    account_endpoint,
                    credential,
                    user_agent,
                    activity_id,
                    pipeline_type,
                    transport_security,
                    account_name: account_name.clone(),
                    effective_consistency,
                    read_consistency_strategy,
                    session_manager,
                    session_consistency_active,
                    options,
                    throughput_control,
                    deadline,
                    configured_request_timeout,
                    can_use_multiple_write_locations: retry_state.can_use_multiple_write_locations,
                    hub_region_processing_only_initial: retry_state.hub_region_processing_only,
                    partition_key_range_id: retry_state.partition_key_range_id.clone(),
                    location_state_store,
                };
                // Same one-race-per-operation latch as STAGE 2b. Set
                // *after* `advance_to_next_attempt` because the latter
                // replaces `retry_state` from `new_state` (which carries
                // `hedge_already_fired = false`).
                retry_state.hedge_already_fired = true;
                match execute_hedged(
                    &attempt_ctx,
                    &primary_routing,
                    &secondary_routing,
                    threshold,
                    strategy_config,
                    diagnostics,
                    &retry_state,
                )
                .await
                {
                    HedgedRaceResult::Terminal(result) => return result,
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
                        // Both legs returned a retriable failure with
                        // budget remaining — fall back into the failover
                        // loop (same shape as the STAGE 2b post-race
                        // fallback earlier in this function).
                        tracing::warn!(
                            activity_id = %activity_id,
                            primary_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                            secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                            "hedge race: both transient on STAGE 7; attempting failover fallback",
                        );
                        diagnostics = returned_diagnostics;
                        // Same fold-back as the STAGE 2b handler above.
                        if retry_state.partition_key_range_id.is_none() {
                            retry_state.partition_key_range_id = race_pk_range_id;
                        }
                        propagate_hedge_session_unavailable(&mut retry_state, race_observed_1002);
                        if let Err(e) = try_advance_after_both_transient(
                            &mut retry_state,
                            &location,
                            operation.is_read_only(),
                            primary_region.as_ref(),
                            secondary_region.as_ref(),
                            last_error,
                        ) {
                            // Budget exhausted: stamp the both-transient
                            // result before grafting diagnostics onto the
                            // error (see STAGE 2b). `deadline_elapsed =
                            // false`: the budget, not the deadline, ended it.
                            diagnostics.set_hedge_diagnostics(HedgeDiagnostics::both_transient(
                                strategy_config,
                                primary_region_for_diag,
                                secondary_region_for_diag,
                                false,
                            ));
                            let diagnostics_ctx = Arc::new(diagnostics.complete());
                            return Err(crate::error::CosmosErrorBuilder::from_error(e)
                                .with_diagnostics(diagnostics_ctx)
                                .build());
                        }
                        diagnostics = enforce_deadline_or_timeout(deadline, options, diagnostics)?;
                        continue;
                    }
                }
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
        // Hub-region cache mutations are emitted at most once per operation
        // (CacheHubRegion at Complete, AdvanceHubRegionDiscovery per 403/3
        // attempt). Never treat them as already-applied — letting them
        // run is idempotent at the routing-systems level (cache_hub_region
        // does an upsert, advance_hub_region_discovery rotates the entry).
        LocationEffect::CacheHubRegion { .. }
        | LocationEffect::AdvanceHubRegionDiscovery { .. } => false,
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
    prefer_gateway_v2: bool,
    account_name_present: bool,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    let account = location.account.as_ref();
    let read_only = operation.is_read_only();
    let route_to_write_endpoints = operation.operation_type().routes_to_write_endpoints();
    // Build an in-flight skip set from effects deferred during this
    // operation. On retries this skips regions we've already failed against
    // so the next attempt picks a different region. Both kinds of deferred
    // effects contribute regions:
    //   * `MarkPartitionUnavailable` — partition-level failure with a region
    //   * `MarkEndpointUnavailable`  — endpoint-level failure (PPAF SM only)
    //
    // Multi-write retries rotate via `LocationIndex`; PPAF single-master writes
    // use the read endpoint list to discover the current write region.
    let in_flight_failed: Vec<&Region> = if route_to_write_endpoints {
        retry_state
            .pending_write_effects
            .iter()
            .filter_map(|e| match e {
                LocationEffect::MarkPartitionUnavailable(p) => p.region.as_ref(),
                LocationEffect::MarkEndpointUnavailable { endpoint, .. } => endpoint.region(),
                LocationEffect::RefreshAccountProperties
                | LocationEffect::CacheHubRegion { .. }
                | LocationEffect::AdvanceHubRegionDiscovery { .. } => None,
            })
            .collect()
    } else {
        Vec::new()
    };

    let primary = preferred_endpoints_for_attempt(
        account,
        retry_state,
        read_only,
        route_to_write_endpoints,
        is_distributed_transaction_operation(operation),
    );
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
        .unwrap_or_else(|| {
            // Last-resort fallback for both data-plane and pipeline-routed
            // metadata operations (Database/Container/Offer/etc. CRUD) when
            // `try_select_endpoint` returned `None` from both passes. The
            // hub regional endpoint is the canonical landing site: it is
            // the only region with guaranteed-fresh state for metadata
            // writes, and it is the most authoritative single region for
            // data-plane operations.
            //
            // When does this fire? Only when EVERY preferred region was
            // excluded by `OperationOptions::excluded_regions`. Note that
            // `excluded_regions` is a hard per-region filter inside
            // `try_select_endpoint` above: excluded regions are
            // unconditionally skipped in both passes and are not even
            // eligible to be returned as a deprioritized `first_unavailable`
            // fallback. So excluding the hub keeps requests out of the hub
            // region as long as at least one other region remains preferred
            // — see the test
            // `resolve_endpoint_dataplane_isolated_region_unavailable_retries_same_region`
            // which pins that behavior.
            //
            // This site is reserved for the degenerate "everything is
            // excluded" configuration. Rather than fail the operation
            // outright we make a single best-effort attempt against the
            // hub regional endpoint, because failing on exhaustion would
            // surprise callers more than landing on the most authoritative
            // region. If we ever need to add an opt-in "strict / fail on
            // exhaustion" routing mode, this is the site to gate it on.
            //
            // The only requests that legitimately target the global
            // endpoint are account-topology fetches (initial bootstrap,
            // periodic refresh, and `LocationEffect::RefreshAccountProperties`),
            // all of which go through
            // `CosmosDriver::fetch_account_properties_with_transport` and
            // bypass this routing path entirely.
            //
            // `preferred_write_endpoints` is guaranteed non-empty by
            // `build_account_endpoint_state`. During normal operation it
            // contains regional endpoints; during initial bootstrap (before
            // account metadata is fetched) it may contain the global
            // default endpoint, but no pipeline operation should run before
            // topology discovery completes. The `debug_assert!` below
            // guards that invariant.
            let fallback_write_endpoints = if is_distributed_transaction_operation(operation) {
                &account.account_write_endpoints
            } else {
                &account.preferred_write_endpoints
            };
            fallback_write_endpoints
                .first()
                .expect("preferred_write_endpoints is always non-empty")
                .clone()
        })
    });
    debug_assert!(
        !selected.is_global(),
        "pipeline operation resolved to global endpoint; \
         this should never happen — only account-topology fetches \
         (which bypass this routing path) may use the global endpoint"
    );
    let use_gateway_v2 = selected.uses_gateway_v2(prefer_gateway_v2)
        && account_name_present
        && is_operation_supported_by_gateway_v2(
            operation.resource_type(),
            operation.operation_type(),
        );
    let transport_mode = if use_gateway_v2 {
        TransportMode::GatewayV2
    } else {
        TransportMode::Gateway
    };
    let selected_url = selected.selected_url(use_gateway_v2).clone();
    let endpoint_key = if use_gateway_v2 {
        EndpointKey::try_from(&selected_url).expect("selected URL must have a valid host and port")
    } else {
        selected.endpoint_key()
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
            let ep_use_gw_v2 = ep.uses_gateway_v2(prefer_gateway_v2)
                && account_name_present
                && is_operation_supported_by_gateway_v2(
                    operation.resource_type(),
                    operation.operation_type(),
                );
            let ep_url = ep.selected_url(ep_use_gw_v2).clone();
            let ep_endpoint_key = if ep_use_gw_v2 {
                EndpointKey::try_from(&ep_url)
                    .expect("selected URL must have a valid host and port")
            } else {
                ep.endpoint_key()
            };
            RoutingDecision {
                selected_url: ep_url,
                transport_mode: if ep_use_gw_v2 {
                    TransportMode::GatewayV2
                } else {
                    TransportMode::Gateway
                },
                endpoint_key: ep_endpoint_key,
                endpoint: ep,
            }
        };

        // PPCB skips stale/excluded/unavailable overrides; PPAF skips only
        // in-flight failures because it targets the single current write region.
        let now = Instant::now();
        let region_in_flight_failed =
            |ep: &CosmosEndpoint| ep.region().is_some_and(|r| in_flight_failed.contains(&r));
        // Topology refresh does not prune partition overrides; stale PPCB
        // targets must fall through to refreshed preferred endpoints.
        let region_not_in_topology = |ep: &CosmosEndpoint| -> bool {
            let Some(region) = ep.region() else {
                return false; // global / hub endpoint stays valid
            };
            !account
                .preferred_read_endpoints
                .iter()
                .chain(account.preferred_write_endpoints.iter())
                .any(|known| known.region() == Some(region))
        };
        // Caller-supplied `excluded_regions` is a hard per-region filter on
        // override fast-paths (reads). A global / hub endpoint with no region
        // is never excluded.
        let region_excluded = |ep: &CosmosEndpoint| -> bool {
            ep.region()
                .is_some_and(|r| retry_state.excluded_regions.iter().any(|e| e == r))
        };
        let ppcb_should_skip = |ep: &CosmosEndpoint| -> bool {
            if region_in_flight_failed(ep) {
                return true;
            }
            if region_not_in_topology(ep) {
                return true;
            }
            if region_excluded(ep) {
                return true;
            }
            !endpoint_is_available(operation, ep, account, now, endpoint_unavailability_ttl)
        };
        let ppaf_should_skip = region_in_flight_failed;

        // Hub-region cache (warm path): if the hub-region-processing-only
        // latch is set on this attempt, PPAF is enabled on the partition
        // state, and the cache is populated for this partition, route
        // directly to the cached hub endpoint, skipping the 403/3
        // discovery chain.
        //
        // Reuses `failover_overrides` (the PPAF cache) — on a single-master
        // account the partition's hub region *is* its write region, so
        // entries serve both PPAF write routing and hub-region read
        // routing. PPAF's existing `is_eligible_for_ppaf` gate rejects
        // reads, so we add this independent read-side gate; the
        // `per_partition_automatic_failover_enabled` check mirrors that
        // gate so the hub cache is consulted only when PPAF is on.
        //
        // Skip the cached hub endpoint and fall through to default selection
        // when it is not a safe target for this read: buffered as failed in
        // this operation, excluded via `excluded_regions`, or marked
        // unavailable account-wide (the same availability check the PPCB read
        // path uses).
        let hub_latch_active = is_read
            && retry_state.hub_region_processing_only
            && partitions.per_partition_automatic_failover_enabled;
        if hub_latch_active {
            if let Some(entry) = partitions.failover_overrides.get(pk_range_id) {
                if !ppaf_should_skip(&entry.current_endpoint)
                    && !region_excluded(&entry.current_endpoint)
                    && endpoint_is_available(
                        operation,
                        &entry.current_endpoint,
                        account,
                        now,
                        endpoint_unavailability_ttl,
                    )
                {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        }

        if !hub_latch_active && is_eligible_for_ppcb(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.circuit_breaker_overrides.get(pk_range_id) {
                if entry.health_status == HealthStatus::ProbeCandidate
                    && !ppcb_should_skip(&entry.first_failed_endpoint)
                {
                    // Route probe request to the original (first failed) endpoint.
                    return make_partition_routing(entry.first_failed_endpoint.clone());
                }
                if can_circuit_breaker_trigger_failover(entry, is_read, &partitions.config)
                    && !ppcb_should_skip(&entry.current_endpoint)
                {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        } else if is_eligible_for_ppaf(partitions, account, is_read, is_partitioned) {
            if let Some(entry) = partitions.failover_overrides.get(pk_range_id) {
                // PPAF overrides do not use probe-based failback (no ProbeCandidate
                // handling). The override persists until the backend signals a change.
                //
                // PPAF defers marks until success, so skip only override targets
                // already failed by this operation.
                if !ppaf_should_skip(&entry.current_endpoint) {
                    return make_partition_routing(entry.current_endpoint.clone());
                }
            }
        }
    }

    RoutingDecision {
        selected_url,
        endpoint: selected,
        endpoint_key,
        transport_mode,
    }
}

fn is_distributed_transaction_operation(operation: &CosmosOperation) -> bool {
    #[cfg(feature = "preview_dtx")]
    {
        operation.resource_type() == crate::models::ResourceType::DistributedTransactionBatch
    }

    #[cfg(not(feature = "preview_dtx"))]
    {
        let _ = operation;
        false
    }
}

fn preferred_endpoints_for_attempt<'a>(
    account: &'a AccountEndpointState,
    retry_state: &OperationRetryState,
    read_only: bool,
    route_to_write_endpoints: bool,
    is_distributed_transaction: bool,
) -> &'a [CosmosEndpoint] {
    if is_distributed_transaction {
        &account.account_write_endpoints
    } else if read_only
        && (route_to_write_endpoints || retry_state.route_reads_to_write_endpoints())
    {
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
    effective_consistency: DefaultConsistencyLevel,
    /// Raw (uncollapsed) RCS to be emitted on the wire for read operations.
    /// `Default` for non-reads or when caller did not specify an RCS.
    read_consistency_strategy: ReadConsistencyStrategy,
    resolved_session_token: Option<SessionToken>,
    throughput_control: Option<ResolvedThroughputControl>,
}

/// Builds a `TransportRequest` from the operation and routing decision.
///
/// If `resolved_session_token` is provided, it is added to the request headers.
/// Override headers from `overrides` are applied after operation headers, so they
/// take precedence.
fn build_transport_request(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    ctx: &TransportRequestContext<'_>,
) -> crate::error::Result<TransportRequest> {
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

    // Apply operation type-specific headers.
    match operation.operation_type() {
        OperationType::Upsert => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_UPSERT),
                HeaderValue::from_static("true"),
            );
        }
        OperationType::Batch => {
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
        OperationType::Query | OperationType::SqlQuery => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                azure_core::http::headers::CONTENT_TYPE,
                HeaderValue::from_static(QUERY_CONTENT_TYPE),
            );
            let supported_features_header =
                HeaderName::from_static(request_header_names::SUPPORTED_QUERY_FEATURES);
            if headers
                .get_optional_str(&supported_features_header)
                .is_none()
            {
                headers.insert(
                    supported_features_header,
                    HeaderValue::from_static(crate::query::SUPPORTED_QUERY_FEATURES),
                );
            }
            let query_version_header = HeaderName::from_static(request_header_names::QUERY_VERSION);
            if headers.get_optional_str(&query_version_header).is_none() {
                headers.insert(query_version_header, HeaderValue::from_static("1.0"));
            }
        }
        OperationType::QueryPlan => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                azure_core::http::headers::CONTENT_TYPE,
                HeaderValue::from_static(QUERY_CONTENT_TYPE),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY_PLAN_REQUEST),
                HeaderValue::from_static("True"),
            );
            // These two headers must always be set on QueryPlan
            // requests. The thin-client proxy reads them out of the RNTBD body
            // (mirrored from these HTTP headers in gateway_v2_dispatch) and rejects
            // requests where they're missing entirely. Default them here when the
            // caller hasn't already set explicit values.
            let supported_features_header =
                HeaderName::from_static(request_header_names::SUPPORTED_QUERY_FEATURES);
            if headers
                .get_optional_str(&supported_features_header)
                .is_none()
            {
                headers.insert(
                    supported_features_header,
                    HeaderValue::from_static(crate::query::SUPPORTED_QUERY_FEATURES),
                );
            }
            let query_version_header = HeaderName::from_static(request_header_names::QUERY_VERSION);
            if headers.get_optional_str(&query_version_header).is_none() {
                headers.insert(query_version_header, HeaderValue::from_static("1.0"));
            }
        }
        _ => {}
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
            crate::driver::transport::cosmos_headers::apply_fault_injection_operation_tag(
                &mut headers,
                fault_op,
            );
        }
    }

    // Apply overrides — these take precedence over operation-level headers
    // (e.g., an override partition key replaces the operation's partition key).
    // Change feed reads carry their continuation via `If-None-Match`, not
    // `x-ms-continuation`.
    overrides.apply_headers(&mut headers, operation.is_change_feed())?;

    // Add resolved session token
    if let Some(token) = &ctx.resolved_session_token {
        headers.insert(
            request_header_names::SESSION_TOKEN,
            HeaderValue::from(token.as_str().to_owned()),
        );
    }

    // Add throughput control headers, if any layer (direct override or
    // resolved group) produced a value.
    if let Some(throughput_control) = ctx.throughput_control {
        if let Some(priority) = throughput_control.priority_level {
            headers.insert(
                request_header_names::PRIORITY_LEVEL,
                HeaderValue::from(priority.as_str().to_owned()),
            );
        }
        if let Some(bucket) = throughput_control.throughput_bucket {
            headers.insert(
                request_header_names::THROUGHPUT_BUCKET,
                HeaderValue::from(bucket.to_string()),
            );
        }
    }

    Ok(TransportRequest {
        method,
        endpoint: ctx.routing.endpoint.clone(),
        transport_mode: ctx.routing.transport_mode,
        operation_type: operation.operation_type(),
        effective_partition_key: effective_partition_key_for_request(operation)?,
        effective_consistency: ctx.effective_consistency,
        read_consistency_strategy: ctx.read_consistency_strategy,
        url,
        headers,
        #[cfg(feature = "preview_dtx")]
        resource_type,
        body: operation.body().map(azure_core::Bytes::copy_from_slice),
        auth_context,
        execution_context: ctx.execution_context,
        deadline: ctx.deadline,
    })
}

/// Computes the effective partition key for an item-scoped request before the
/// transport pipeline runs, so wire layers receive a ready-to-encode EPK rather
/// than raw partition-key/definition values.
///
/// Returns `Ok(None)` when the operation has no partition key, no container
/// context, or an empty partition key. Surfaces a `BadRequest` when the caller
/// supplies more components than the container's partition-key definition
/// declares (the hash routines would otherwise silently hash the extras into a
/// broken EPK).
fn effective_partition_key_for_request(
    operation: &CosmosOperation,
) -> crate::error::Result<Option<EffectivePartitionKey>> {
    let (Some(partition_key), Some(container)) = (operation.partition_key(), operation.container())
    else {
        return Ok(None);
    };

    if partition_key.is_empty() {
        return Ok(None);
    }

    let partition_key_definition = container.partition_key_definition();
    if partition_key.values().len() > partition_key_definition.paths().len() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
            .with_message(
                "Partition key supplies more components than the container's \
                 partition-key definition declares",
            )
            .build());
    }

    Ok(Some(EffectivePartitionKey::compute(
        partition_key.values(),
        partition_key_definition.kind(),
        partition_key_definition.version(),
    )))
}

/// Builds a `CosmosResponse` from a successful `TransportResult`.
fn build_cosmos_response(
    result: Box<TransportResult>,
    mut diagnostics: DiagnosticsContextBuilder,
) -> crate::error::Result<CosmosResponse> {
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
            // This should only be called with a Complete(Success) result.
            // Treat as a programmer-error invariant violation.
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE)
                .with_message("build_cosmos_response called with non-success result")
                .build())
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
/// See `try_handle_read_session_not_available` for the latch trigger.
/// When a hedge race is in flight, the per-state latch is OR'd against
/// the cross-hedge `shared_hub_region_latch` so all hedge legs emit the
/// header once any one of them has observed a 1002.
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
/// header should be emitted on a transport request.
///
/// Emission is the OR of the per-state latch (set by
/// `build_session_retry_state` on the first 1002 of a single-master
/// data-plane operation) and the shared latch (set by any sibling
/// hedge inside the same `execute_hedged` fan-out). The shared latch
/// is read with `Acquire` ordering, pairing with the `Release` store
/// in `build_session_retry_state`. When the shared latch is absent
/// (`None` — the non-hedged pipeline and the zero-overhead happy
/// path), the rule collapses to the per-state behavior.
fn should_emit_hub_region_header(
    per_state_latched: bool,
    shared_latch: Option<&Arc<AtomicBool>>,
) -> bool {
    per_state_latched || shared_latch.is_some_and(|s| s.load(Ordering::Acquire))
}

/// Returns `true` when [`execute_hedged`] should construct an
/// `Arc<AtomicBool>` shared hub-region-processing-only latch for the
/// hedge fan-out.
///
/// The latch only matters when both:
///
/// 1. **Data-plane scope** — metadata operations are scoped out,
///    mirroring the per-state latch's `is_dataplane` trigger gate.
/// 2. **Single-master account** — multi-master accounts never emit the
///    `x-ms-cosmos-hub-region-processing-only` header; a latch on a
///    multi-master hedge would be inert and would waste an `Arc`
///    allocation.
///
/// Construction at the call site is further gated on the threshold
/// having elapsed, so the zero-overhead happy path (primary wins
/// pre-threshold) is preserved.
fn should_build_shared_hub_region_latch(
    pipeline_type: PipelineType,
    can_use_multiple_write_locations: bool,
) -> bool {
    pipeline_type.is_data_plane() && !can_use_multiple_write_locations
}

/// Emits `x-ms-cosmos-allow-tentative-writes: true` on writes when the account
/// is multi-write. Without this header, satellite write regions reject writes
/// with `403 / 3 (WriteForbidden)` because the request is treated as
/// single-master traffic to a non-primary region. The header is gated on
/// `enableMultipleWriteLocations` from the account metadata.
fn apply_tentative_writes_header(
    transport_request: &mut TransportRequest,
    operation: &CosmosOperation,
    multiple_write_locations_enabled: bool,
) {
    if multiple_write_locations_enabled && !operation.is_read_only() {
        transport_request.headers.insert(
            HeaderName::from_static(request_header_names::ALLOW_TENTATIVE_WRITES),
            HeaderValue::from_static("true"),
        );
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

/// Advances `retry_state`, preserving deferred write effects across the snapshot swap.
///
/// Note: a generation bump can rebase to position 1 and retry the just-failed region once.
fn advance_to_next_attempt(
    retry_state: &mut OperationRetryState,
    new_state: OperationRetryState,
    location_state_store: &LocationStateStore,
    is_read_only: bool,
    route_to_write_endpoints: bool,
    is_distributed_transaction: bool,
) {
    let next_location = location_state_store.snapshot();
    let endpoints_len = preferred_endpoints_for_attempt(
        next_location.account.as_ref(),
        &new_state,
        is_read_only,
        route_to_write_endpoints,
        is_distributed_transaction,
    )
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
/// Enforces the operation's end-to-end deadline, surfacing a typed
/// `408 / CLIENT_OPERATION_TIMEOUT` error when exceeded so callers
/// can distinguish a client-side end-to-end timeout from a service 408.
///
/// Takes the [`DiagnosticsContextBuilder`] by value so the timeout-error
/// path can finalize diagnostics and graft them onto the synthesized
/// error in one step (without that graft, callers reading
/// `error.diagnostics()` would see `None` on every end-to-end-timeout
/// outcome even though the pipeline tracked every attempt). The builder
/// is returned unchanged on the happy path so the caller can keep
/// mutating it on subsequent iterations.
fn enforce_deadline_or_timeout(
    deadline: Option<Instant>,
    options: &OperationOptionsView<'_>,
    mut diagnostics: DiagnosticsContextBuilder,
) -> Result<DiagnosticsContextBuilder, crate::error::CosmosError> {
    let Some(d) = deadline else {
        return Ok(diagnostics);
    };
    if Instant::now() < d {
        return Ok(diagnostics);
    }

    let timeout_duration = options
        .end_to_end_latency_policy()
        .map(|p| p.timeout())
        .unwrap_or_default();

    diagnostics.set_operation_status(
        azure_core::http::StatusCode::RequestTimeout,
        Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
    );
    let diagnostics_ctx = Arc::new(diagnostics.complete());
    Err(crate::error::CosmosError::builder()
        .with_status(crate::models::CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message(format!(
            "end-to-end operation timeout exceeded ({timeout_duration:?})"
        ))
        .with_diagnostics(diagnostics_ctx)
        .build())
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

/// Returns the partition key range ID to cache as the hub region if the
/// completed operation satisfies the populate gate; otherwise `None`.
///
/// The gate requires all three: the hub-region-processing-only latch was
/// active for this attempt, a partition key range ID is known, and the
/// operation is a read. The caller additionally skips the apply when PPAF is
/// disabled or the entry already points at the hub endpoint.
fn hub_region_cache_populate_target(
    retry_state: &OperationRetryState,
    operation: &CosmosOperation,
) -> Option<PartitionKeyRangeId> {
    if retry_state.hub_region_processing_only && operation.is_read_only() {
        retry_state.partition_key_range_id.clone()
    } else {
        None
    }
}

// ── Hedging dispatch (Part 4b) ────────────────────────────────────────
//
// `maybe_upgrade_to_hedge` + `AttemptContext` + `perform_single_attempt`
// + `execute_hedged` together implement the cross-region hedging race:
//
//  1. After `evaluate_transport_result` returns a per-attempt action,
//     `maybe_upgrade_to_hedge` rewrites a same-pipeline retry into
//     `OperationAction::Hedge` when the operation is hedge-eligible.
//  2. The STAGE 7 `Hedge` arm bundles the per-operation shared state
//     into `AttemptContext` and calls `execute_hedged`.
//  3. `execute_hedged` fires the primary attempt immediately, races it
//     against the threshold timer (zero-overhead happy path when the
//     primary completes first), then spawns the secondary and races
//     primary vs secondary. The first final result wins; a retriable
//     failure on either side keeps the other side racing.
//
// Diagnostics: each hedge attempt records into its own sub-builder
// cloned from the parent via `clone_for_hedge_attempt`. After the race
// resolves the surviving sub-builders are merged back into the parent
// via `merge_hedge_attempt`, and the winning side's `HedgeDiagnostics`
// is attached via `set_hedge_diagnostics`. Loser futures dropped by the
// race never merge — their request diagnostics are intentionally
// discarded.

/// Dispatches a hedge-race outcome to the [`LocationStateStore`]'s
/// PPCB-feedback counters.
///
/// Feedback is keyed by `(partition, primary_region)`. When the
/// operation never resolved a partition key range ID (first attempt
/// against an unresolved item) the call is a no-op: there is no key to
/// attribute feedback against. The same gating applies to both
/// alternate-wins and primary-wins so the reset only fires on pairs
/// that could ever accumulate a count in the first place.
fn record_hedge_outcome(
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

/// Per-operation context shared across hedge attempts.
///
/// Bundles the read-only references that every per-attempt transport
/// invocation needs.
struct AttemptContext<'a> {
    operation: &'a CosmosOperation,
    /// Per-operation header/body overrides applied by
    /// [`build_transport_request`] (e.g., PK-range-id targeting for
    /// query/feed operations dispatched against a specific partition).
    /// Carried so hedge legs in [`execute_hedged`] / [`perform_single_attempt`]
    /// rebuild the request identically to the main pipeline's STAGE 3.
    overrides: &'a OperationOverrides,
    custom_headers: Option<&'a std::collections::HashMap<HeaderName, HeaderValue>>,
    transport: &'a CosmosTransport,
    account_endpoint: &'a AccountEndpoint,
    credential: &'a Credential,
    user_agent: &'a azure_core::http::headers::HeaderValue,
    activity_id: &'a ActivityId,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    /// Global database account name parsed from `account_endpoint`. Used by
    /// Gateway 2.0 request wrapping when an attempt routes to a G2 endpoint.
    account_name: Option<String>,
    /// Effective `DefaultConsistencyLevel` resolved for this operation
    /// (read-strategy + account default). Threaded through hedge legs so they
    /// build the request identically to the non-hedged path.
    effective_consistency: DefaultConsistencyLevel,
    /// Configured `ReadConsistencyStrategy` for this operation. Same threading
    /// rationale as `effective_consistency`.
    read_consistency_strategy: ReadConsistencyStrategy,
    session_manager: &'a SessionManager,
    /// Whether session consistency is in effect for this operation
    /// (drives session-token resolve/capture inside the attempt).
    session_consistency_active: bool,
    options: &'a OperationOptionsView<'a>,
    throughput_control: Option<ResolvedThroughputControl>,
    /// End-to-end deadline (operation timeout) — passed through to each
    /// per-attempt transport invocation.
    deadline: Option<Instant>,
    /// Original configured timeout (the value used to derive `deadline`).
    /// Carried so [`execute_hedged`] can compute the effective harvest
    /// window via [`effective_harvest_window`].
    configured_request_timeout: Option<Duration>,
    /// Whether this operation runs against a multi-master account.
    /// Used by [`execute_hedged`] to gate construction of the shared
    /// hub-region-processing-only latch (the latch is only meaningful
    /// on single-master data-plane operations).
    can_use_multiple_write_locations: bool,
    /// Current value of the per-state `hub_region_processing_only`
    /// latch at the moment hedging upgraded from `FailoverRetry` /
    /// `SessionRetry`. Used by [`execute_hedged`] to seed the shared
    /// `Arc<AtomicBool>` so a 1002 already discovered by the main
    /// pipeline before hedging fired carries forward into both hedge
    /// attempts.
    hub_region_processing_only_initial: bool,
    /// Identifies the physical partition serving this operation. Used by
    /// [`execute_hedged`] to attribute hedge-win signals back to the
    /// `(partition, primary_region)` pair consumed by PPCB feedback.
    /// `None` for operations dispatched before any response captured the
    /// partition key range ID (first attempt of an unresolved item) —
    /// in that case no feedback can be attributed and the recorder calls
    /// become no-ops.
    partition_key_range_id: Option<PartitionKeyRangeId>,
    /// Sink for hedge-outcome feedback (PPCB counters keyed by
    /// `(partition, primary_region)`). Held as a borrow so
    /// [`execute_hedged`] can hand it through [`record_hedge_outcome`]
    /// at each of the four race-outcome branches without cloning the
    /// `Arc` on every operation.
    location_state_store: &'a LocationStateStore,
}

/// Result classification used by [`execute_hedged`] to decide whether a
/// completed hedge attempt terminates the race or keeps the other side
/// running.
///
/// `Final` carries the [`TransportResult`] that becomes the operation's
/// outcome (success or final-classified HTTP error). `Transient`
/// indicates the attempt should be ignored in favor of the other side
/// (5xx / 429 / 408 / 410 / 404-1002 / 403-with-sub / transport error /
/// deadline) — when both sides land retriable, the most recent error
/// is surfaced as the operation error.
enum HedgeClass {
    Final(Box<TransportResult>),
    Transient,
}

/// Pure classifier for a per-attempt result.
///
/// `Err` (e.g. failure constructing the request) and any non-final
/// TransportOutcome map to `Transient`. A `Success` is always final; an
/// `HttpError` is final iff [`CosmosStatus::is_final_result`] returns `true` for its
/// status.
fn classify_hedge_result(result: crate::error::Result<TransportResult>) -> HedgeClass {
    match result {
        Ok(tr) => match &tr.outcome {
            TransportOutcome::Success { .. } => HedgeClass::Final(Box::new(tr)),
            TransportOutcome::HttpError { status, .. } => {
                if status.is_final_result() {
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

/// Non-consuming version of [`classify_hedge_result`] for the
/// pre-threshold primary-completion branch, where the caller still
/// needs the original `TransportResult` to surface the response via
/// [`finalize_hedge_attempt`] but only wants to know whether the
/// primary's outcome was `Final` for [`record_hedge_outcome`] gating.
///
/// Returns `true` iff a `Success` or an `HttpError` whose status passes
/// [`CosmosStatus::is_final_result`]. Mirrors `classify_hedge_result` exactly — keep
/// the two in lockstep.
fn result_is_final(tr: &TransportResult) -> bool {
    match &tr.outcome {
        TransportOutcome::Success { .. } => true,
        TransportOutcome::HttpError { status, .. } => status.is_final_result(),
        TransportOutcome::TransportError { .. } | TransportOutcome::DeadlineExceeded { .. } => {
            false
        }
    }
}

/// Peeks at a hedge-leg's [`TransportResult`] for the partition-key-range-id
/// header without consuming the result.
///
/// The non-hedged STAGE 4 captures this id on first observation by
/// pulling it out of `result.cosmos_headers()` and stashing it on
/// `OperationRetryState::partition_key_range_id`. Hedge legs never
/// performed the equivalent capture, so when [`execute_hedged`] returned
/// [`HedgedRaceResult::BothTransient`] the surrounding loop re-entered
/// with `retry_state.partition_key_range_id = None` even though one or
/// both legs had received responses that carried the id. Subsequent
/// PPCB feedback then short-circuited (`record_hedge_outcome` no-ops on
/// `partition.is_none()`) and partition-level overrides could not be
/// installed for later failures.
///
/// Called at each `classify_hedge_result` site in [`execute_hedged`] to
/// accumulate the first non-`None` id observed across either leg, which
/// is then surfaced via
/// [`HedgedRaceResult::BothTransient::partition_key_range_id`] for the
/// caller to write back into `retry_state` before the loop re-enters.
fn pk_range_id_from_result(
    result: &crate::error::Result<TransportResult>,
) -> Option<PartitionKeyRangeId> {
    result
        .as_ref()
        .ok()
        .and_then(|tr| tr.cosmos_headers())
        .and_then(|headers| headers.partition_key_range_id.as_deref())
        .map(|s| PartitionKeyRangeId::from(s.to_owned()))
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
///
/// # Diagnostics-on-error
///
/// Every `Err`-producing arm grafts the operation's
/// `DiagnosticsContextBuilder` onto the returned error via
/// `CosmosErrorBuilder::from_error(...).with_diagnostics(...)` so
/// callers reading `error.diagnostics()` see the full
/// per-attempt request chain — operators investigating "why didn't
/// hedging save me?" need the diagnostics on hedge-terminal errors
/// even more than the non-hedged path, because the operator paid
/// for two regions of requests on a final error.
fn finalize_hedge_attempt(
    result: Box<TransportResult>,
    diagnostics: DiagnosticsContextBuilder,
) -> crate::error::Result<CosmosResponse> {
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => {
            build_cosmos_response(Box::new(TransportResult { outcome }), diagnostics)
        }
        TransportOutcome::HttpError {
            status,
            cosmos_headers,
            body,
            ..
        } => {
            tracing::warn!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                http_status = u16::from(status.status_code()),
                sub_status = ?status.sub_status(),
                "cosmos.hedge.terminal_http_error",
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            let base = build_service_error(&status, &cosmos_headers, &body);
            Err(crate::error::CosmosErrorBuilder::from_error(base)
                .with_diagnostics(diagnostics_ctx)
                .build())
        }
        TransportOutcome::TransportError { error, .. } => {
            tracing::warn!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                error = %error,
                "cosmos.hedge.terminal_transport_error",
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            Err(crate::error::CosmosErrorBuilder::from_error(error)
                .with_diagnostics(diagnostics_ctx)
                .build())
        }
        TransportOutcome::DeadlineExceeded { .. } => {
            tracing::warn!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                "cosmos.hedge.terminal_deadline_exceeded",
            );
            // Typed status (408 + CLIENT_OPERATION_TIMEOUT) mirrors
            // `enforce_deadline_or_timeout` so retry-evaluation and
            // telemetry treat hedged deadline-exceeded identically to
            // non-hedged.
            let mut diagnostics = diagnostics;
            diagnostics.set_operation_status(
                azure_core::http::StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            Err(crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::from_parts(
                    azure_core::http::StatusCode::RequestTimeout,
                    Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                ))
                .with_message("deadline exceeded during hedged attempt")
                .with_diagnostics(diagnostics_ctx)
                .build())
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
/// [`evaluate_hedge_eligibility`] so it can compute the driver default
/// threshold (`min(1000ms, request_timeout / 2)`).
///
/// Forwards the incoming variant's `new_state` into the rewritten
/// `Hedge` variant so STAGE 7 can apply it to the live `retry_state`
/// before building `AttemptContext` — see
/// `OperationAction::Hedge::new_state`.
fn maybe_upgrade_to_hedge(
    action: OperationAction,
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    account_state: &AccountEndpointState,
    primary: &RoutingDecision,
    request_timeout: Option<Duration>,
) -> OperationAction {
    // Extract `new_state` from the retry-upgrade-eligible variants;
    // return everything else unchanged.
    let new_state = match &action {
        OperationAction::FailoverRetry { new_state, .. } => new_state.clone(),
        OperationAction::SessionRetry { new_state } => new_state.clone(),
        _ => return action,
    };

    match evaluate_hedge_eligibility(operation, options, account_state, primary, request_timeout) {
        Some(upgrade) => {
            // Hedge consumes two failover-budget slots on the race
            // (primary + secondary) and a third on BothTransient
            // advance. If fewer than two slots remain, stay on the
            // non-hedged path so the remaining budget is spent on
            // sequential failovers.
            if new_state.failover_retry_count.saturating_add(2) > new_state.max_failover_retries {
                tracing::debug!(
                    failover_retry_count = new_state.failover_retry_count,
                    max_failover_retries = new_state.max_failover_retries,
                    "cosmos.hedge.budget_exhausted_skipping_upgrade",
                );
                return action;
            }
            // Emit a structured event when an operation is upgraded
            // into the hedge race. Fields mirror the inputs that drove
            // the eligibility decision so operators can correlate
            // threshold tuning with observed upgrades.
            tracing::debug!(
                threshold_ms = upgrade.threshold.get().as_millis() as u64,
                primary_region = ?primary.endpoint.region().map(crate::options::Region::as_str),
                secondary_region = ?upgrade.secondary_routing.endpoint.region().map(crate::options::Region::as_str),
                hub_region_processing_only = new_state.hub_region_processing_only,
                "cosmos.hedge.enabled_for_operation",
            );
            OperationAction::Hedge {
                secondary_routing: upgrade.secondary_routing,
                threshold: upgrade.threshold,
                strategy_config: upgrade.strategy_config,
                new_state,
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
/// the same STAGE 3/4/4b code for compatibility and to keep the
/// hedging diff focused. A follow-up refactor may collapse the
/// duplication.
///
/// Differences from the inline loop body:
///
/// - **No per-state `hub_region_processing_only` latch.** Each hedge
///   today runs a single transport attempt with no retry loop, so the
///   per-state latch has nothing to drive. The cross-hedge shared
///   latch is plumbed through `shared_hub_region_latch` instead — the
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
) -> crate::error::Result<TransportResult> {
    // Resolve session token using the same precedence the main loop uses.
    // Scope to the target range only for thin-client (Gateway 2.0); classic
    // gateway keeps the composite token (see main-loop rationale).
    let resolved_session_token = ctx
        .session_consistency_active
        .then(|| {
            let scoped_pk_range_id = if matches!(routing.transport_mode, TransportMode::GatewayV2) {
                ctx.partition_key_range_id.as_ref().map(|id| id.as_str())
            } else {
                None
            };
            ctx.session_manager.resolve_session_token(
                ctx.operation,
                ctx.operation.request_headers().session_token.as_ref(),
                scoped_pk_range_id,
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
        effective_consistency: ctx.effective_consistency,
        read_consistency_strategy: ctx.read_consistency_strategy,
    };

    let mut transport_request = build_transport_request(
        ctx.operation,
        ctx.overrides,
        ctx.custom_headers,
        &request_ctx,
    )?;
    // Hedging attempts have no per-state latch to consult — the only
    // signal is the cross-hedge shared latch.
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

    // Resolve the per-leg throttle (429) retry budget from the same effective
    // options the main pipeline uses, so a hedge leg honors the configured
    // `ThrottlingRetryOptions` identically to a non-hedged attempt. Each leg
    // enters the transport pipeline once and starts with a fresh budget.
    let throttling_retry_options = ctx.options.throttling_retry_options();
    let (default_attempts, default_wait, max_throttle_per_retry_delay) =
        default_throttle_budget(ctx.pipeline_type);
    let max_throttle_attempts = throttling_retry_options
        .max_retry_count()
        .copied()
        .unwrap_or(default_attempts);
    let max_throttle_wait_time = throttling_retry_options
        .max_retry_wait_time()
        .copied()
        .unwrap_or(default_wait);

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
            endpoint_key: routing.endpoint_key.clone(),
            account_name: ctx.account_name.clone(),
            collection_rid: ctx.operation.container().map(|c| c.rid().to_owned()),
            max_throttle_attempts,
            max_throttle_wait_time,
            max_throttle_per_retry_delay,
        },
        diagnostics,
    )
    .await;

    // Session-token capture deliberately deferred to the winning
    // branch in `execute_hedged`. Capturing here would advance the
    // session token off the *losing* leg's response on every race,
    // which can leak stale state from a region whose mutation the
    // caller never observes. See `capture_session_token_for_winner`.
    Ok(result)
}

/// Captures the session token from a hedge race's **winning** leg's
/// response, mirroring the non-hedged STAGE 4b logic without the inline
/// capture that `perform_single_attempt` used to perform.
///
/// Called from each terminal "winner" branch in [`execute_hedged`] (the
/// pre-threshold primary win, the post-threshold primary win, and the
/// two alternate-win branches). The loser's headers are intentionally
/// discarded — advancing the operation's session token from a region
/// whose response the caller never observes would leak stale state and
/// violate read-your-writes against the winning region.
fn capture_session_token_for_winner(ctx: &AttemptContext<'_>, result: &TransportResult) {
    if !ctx.session_consistency_active {
        return;
    }
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

/// Upper bound on how long [`execute_hedged`] waits for in-flight hedge
/// attempts to land their diagnostics after the application-cancel
/// deadline fires. The Rust hedge path deliberately bounds this — .NET
/// v3 awaits the most-recently-completed task with no timeout, but the
/// Rust path trades slightly less-rich diagnostics-on-cancel for
/// predictable user-visible cancel latency when a transport future is
/// stuck. The effective window is further capped by
/// [`effective_harvest_window`] for callers with aggressive timeouts.
const HARVEST_WINDOW: Duration = Duration::from_millis(50);

/// Lower bound on the harvest window so an already-completed `Ready`
/// future still resolves before the timer fires on typical OSes.
const HARVEST_WINDOW_FLOOR: Duration = Duration::from_millis(1);

/// Effective harvest window for a given operation timeout:
/// `min(HARVEST_WINDOW, request_timeout / 10)`, clamped at
/// [`HARVEST_WINDOW_FLOOR`]. Returns [`HARVEST_WINDOW`] when no timeout
/// is configured. Aggressive callers (e.g. `request_timeout = 100ms`)
/// avoid a fixed 50ms cancel-latency tax.
fn effective_harvest_window(configured_request_timeout: Option<Duration>) -> Duration {
    match configured_request_timeout {
        Some(t) => HARVEST_WINDOW.min(t / 10).max(HARVEST_WINDOW_FLOOR),
        None => HARVEST_WINDOW,
    }
}

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
/// attempt's request trail. The result itself is discarded — once the
/// application cancellation has fired the operation outcome is the
/// cancellation error regardless of whether the in-flight pipeline
/// would have produced a final response.
async fn harvest_remaining_attempt<F>(
    attempt: F,
    parent: &mut DiagnosticsContextBuilder,
    harvest_window: Duration,
) where
    F: Future<
            Output = (
                crate::error::Result<TransportResult>,
                DiagnosticsContextBuilder,
            ),
        > + Unpin
        + Send,
{
    let window = match azure_core::time::Duration::try_from(harvest_window) {
        Ok(d) => d,
        Err(_) => return,
    };
    let timer = Box::pin(azure_core::sleep(window));
    if let Either::Left(((_result, diag), _timer)) = select(attempt, timer).await {
        parent.merge_hedge_attempt(diag);
    }
    // Timer arm: harvest window exceeded; loser future is dropped and its
    // Drop chain cancels the in-flight transport.
}

/// Synthetic operation error returned by [`execute_hedged`] when the
/// application-cancellation deadline fires while one or both hedge
/// attempts are still racing. Mirrors the .NET v3 cancellation behavior
/// and `enforce_deadline_or_timeout`'s typed-status pattern.
///
/// Takes the [`DiagnosticsContextBuilder`] by value so the cancel-error
/// path can finalize diagnostics and graft them onto the synthesized
/// error in one step — without that graft, callers reading
/// `error.diagnostics()` would see `None` on every hedge-cancel
/// outcome even though the pipeline tracked every attempt and harvested
/// the most-advanced in-flight leg within [`HARVEST_WINDOW`].
///
/// Status is set to `408 RequestTimeout` + `CLIENT_OPERATION_TIMEOUT`
/// (20008) so callers and telemetry can discriminate a client-side
/// hedge cancel from a service-returned 408.
fn application_cancelled_error(
    mut diagnostics: DiagnosticsContextBuilder,
) -> crate::error::CosmosError {
    diagnostics.set_operation_status(
        azure_core::http::StatusCode::RequestTimeout,
        Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
    );
    let diagnostics_ctx = Arc::new(diagnostics.complete());
    crate::error::CosmosError::builder()
        .with_status(crate::models::CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message("operation cancelled by application deadline during cross-region hedging")
        .with_diagnostics(diagnostics_ctx)
        .build()
}

/// Races a still-pending hedge attempt against the end-to-end deadline.
/// Returns `Some(result)` when the attempt completes first (normal
/// path); returns `None` after harvesting the attempt's diagnostics
/// within [`HARVEST_WINDOW`] when the deadline wins.
///
/// Used by [`execute_hedged`] in Stage 4's transient-fallthrough
/// branches where one side has already classified as transient and the
/// loop is `.await`-ing the other side — without this wrap a stuck
/// transport future would block the operation past the deadline.
async fn await_attempt_or_deadline_harvest<F>(
    attempt: F,
    deadline: Option<Instant>,
    parent: &mut DiagnosticsContextBuilder,
    harvest_window: Duration,
) -> Option<(
    crate::error::Result<TransportResult>,
    DiagnosticsContextBuilder,
)>
where
    F: Future<
            Output = (
                crate::error::Result<TransportResult>,
                DiagnosticsContextBuilder,
            ),
        > + Unpin
        + Send,
{
    let deadline_fut = deadline_signal(deadline);
    match select(attempt, deadline_fut).await {
        Either::Left((result, _deadline)) => Some(result),
        Either::Right(((), remaining)) => {
            harvest_remaining_attempt(remaining, parent, harvest_window).await;
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

/// Outcome of [`execute_hedged`] as observed by the surrounding
/// [`execute_operation_pipeline`] loop.
///
/// The race is structured around two qualitatively different outcomes:
///
/// - [`HedgedRaceResult::Terminal`] — the race produced a final result
///   (success, final HTTP error, or application-cancellation). The
///   operation pipeline must surface this directly to the caller. The
///   inner `Result` mirrors what the non-hedged
///   [`OperationAction::Complete`] / [`OperationAction::Abort`] arms
///   produce so STAGE 7 remains the canonical translation point.
/// - [`HedgedRaceResult::BothTransient`] — both legs returned transient
///   outcomes **and** the deadline has not elapsed. The operation
///   pipeline must consume the two regions used in the race against
///   [`OperationRetryState::failover_retry_count`] (incrementing by 2)
///   and re-enter the loop against the remaining regions. If the budget
///   is exhausted, `last_error` is surfaced as the operation's terminal
///   error.
///
/// Without this loop-fallback the hedge-eligible code path would be
/// strictly *less* resilient than the non-hedged path in multi-region
/// brown-outs: a 503 burst across the first two preferred regions of a
/// 3-region account would cause a non-hedged read to consume one
/// failover-retry slot per region and reach region 3 on the third
/// attempt, while a hedge-eligible read would burn both regions in a
/// single race and give up immediately.
#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "single return value from an async function on a hot path; \
              boxing either variant would add a heap allocation per race \
              without enabling reuse — at most one `HedgedRaceResult` \
              exists at a time per operation."
)]
pub(crate) enum HedgedRaceResult {
    /// The race ended in a state the operation pipeline must surface
    /// directly to the caller.
    Terminal(crate::error::Result<CosmosResponse>),

    /// Both legs returned `Transient` outcomes without the deadline
    /// firing. The caller must:
    ///
    /// 1. Increment `failover_retry_count` by 2 (the race consumed two
    ///    regions).
    /// 2. If the budget is exhausted, return `last_error` as the
    ///    operation's terminal error.
    /// 3. Otherwise advance the [`LocationIndex`] past the two consumed
    ///    regions and `continue` the operation loop.
    ///
    /// `primary_region` / `secondary_region` are surfaced via tracing.
    /// `partition_key_range_id` and `observed_session_unavailable` carry
    /// state the race observed that the fallback folds back into
    /// `retry_state` before re-entering.
    ///
    /// `strategy_config` and the `*_for_diag` regions are carried so the
    /// fallback can stamp `HedgeDiagnostics::both_transient` only when the
    /// failover budget is exhausted (the terminal outcome). They are not
    /// stamped on the non-terminal path, where a later successful retry
    /// would otherwise carry a misleading `terminal_state = BothTransient`.
    BothTransient {
        primary_region: Option<Region>,
        secondary_region: Option<Region>,
        strategy_config: HedgingStrategyConfig,
        primary_region_for_diag: Region,
        secondary_region_for_diag: Region,
        last_error: crate::error::CosmosError,
        diagnostics: DiagnosticsContextBuilder,
        partition_key_range_id: Option<PartitionKeyRangeId>,
        observed_session_unavailable: bool,
    },
}

/// Applies one hedge leg's `LocationEffect`s to the shared
/// `LocationStateStore` and OR-accumulates its
/// `observed_session_unavailable` signal (also `Release`-storing the
/// shared latch on first trigger).
///
/// Called at every classify site in [`execute_hedged`] so the hedged
/// path's routing-state and hub-region-latch mutations stay symmetric
/// with the non-hedged `evaluate_transport_result` path.
async fn apply_hedge_leg_effects(
    ctx: &AttemptContext<'_>,
    retry_state_snapshot: &OperationRetryState,
    endpoint: &CosmosEndpoint,
    result: &crate::error::Result<TransportResult>,
    shared_hub_region_latch: Option<&Arc<AtomicBool>>,
    race_observed_session_unavailable: &mut bool,
) {
    // Pre-transport / request-build errors carry no `TransportResult`
    // and therefore no observable side effects to mirror — they fail
    // before the transport pipeline classifies any response.
    let Ok(transport_result) = result.as_ref() else {
        return;
    };
    let eval = evaluate_hedge_leg_effects(
        ctx.operation,
        endpoint,
        retry_state_snapshot,
        transport_result,
    );
    if !eval.effects.is_empty() {
        ctx.location_state_store.apply(&eval.effects).await;
    }
    if eval.observed_session_unavailable {
        *race_observed_session_unavailable = true;
        if let Some(latch) = shared_hub_region_latch {
            latch.store(true, Ordering::Release);
        }
    }
}

/// Races a primary attempt against a single cross-region secondary
/// attempt. Returns the first **final** result via
/// [`HedgedRaceResult::Terminal`]; surfaces a both-transient outcome via
/// [`HedgedRaceResult::BothTransient`] so the surrounding
/// [`execute_operation_pipeline`] loop can re-enter against the
/// remaining regions.
///
/// The implementation is structured as two `futures::future::select`
/// calls so the borrow checker can reclaim each sub-builder cleanly
/// as its owning future completes:
///
/// 1. **Threshold race.** The primary future is launched immediately
///    against an `azure_core::sleep(threshold)` timer. If the primary
///    completes first this is the zero-overhead happy path — no
///    secondary future is constructed, no extra diagnostics sub-builder
///    is cloned, no `Arc` allocations occur beyond what the single
///    primary attempt already needs.
/// 2. **Primary vs secondary race.** Once the timer fires we clone a
///    sub-builder for the secondary, launch it with
///    `ExecutionContext::Hedging`, and `select` against the still-pending
///    primary. The first side to land a `HedgeClass::Final` wins; a
///    transient on the first-to-complete side simply `.await`s the
///    other side. **Both legs transient** with the deadline still in
///    budget → returns [`HedgedRaceResult::BothTransient`] so the
///    caller loop can fall back against the remaining regions.
///
/// Application-cancellation (observed via [`AttemptContext::deadline`])
/// is layered onto both races: the deadline future is wrapped into a
/// 3-way `select` with the threshold timer (Stage 2) and the
/// primary/secondary pair (Stage 4). When the deadline wins,
/// [`harvest_remaining_attempt`] gives the most-advanced in-flight
/// pipeline up to `HARVEST_WINDOW` to complete so its diagnostics can
/// be attached to the returned [`application_cancelled_error`]. Loser
/// futures are dropped — structural cancellation propagates through
/// the transport pipeline.
///
/// PPCB feedback is wired here via [`record_hedge_outcome`]: every
/// Final outcome routed to either side is reported to the
/// [`LocationStateStore`] so the PPCB side can count consecutive
/// alternate-wins (and trip the partition at the configured threshold)
/// and reset the counter on primary-wins. The counter / partition-trip
/// machinery is fully implemented in
/// [`record_hedge_alternate_win`](crate::driver::routing) /
/// [`record_hedge_primary_win`](crate::driver::routing); the load-bearing
/// call sites are these branches.
///
/// Items NOT yet implemented:
///
/// - Operation-level retry inside each hedge; we rely on per-attempt
///   transport-pipeline retry only.
async fn execute_hedged(
    ctx: &AttemptContext<'_>,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    threshold: HedgeThreshold,
    strategy_config: HedgingStrategyConfig,
    mut parent_diagnostics: DiagnosticsContextBuilder,
    // Borrowed snapshot read by `apply_hedge_leg_effects`; the race
    // never mutates the parent retry_state — STAGE 2b / STAGE 7 do
    // that post-race once `HedgedRaceResult` is destructured.
    retry_state_snapshot: &OperationRetryState,
) -> HedgedRaceResult {
    let primary_region = primary_routing.endpoint.region().cloned();
    let secondary_region = secondary_routing.endpoint.region().cloned();
    // `HedgeDiagnostics` is attached iff `execute_hedged` ran,
    // regardless of whether the routed endpoints carry a named region
    // (global-endpoint accounts do not). For diagnostics-construction
    // sites we substitute the public
    // [`HedgeDiagnostics::UNKNOWN_REGION_SENTINEL`] `Region` so the
    // attachment contract holds; PPCB recording paths below still use
    // the original `Option<Region>` so a `None` correctly attributes to
    // "no region" rather than collapsing all global-endpoint hedges
    // under a sentinel key.
    let primary_region_for_diag = primary_region
        .clone()
        .unwrap_or_else(|| Region::new(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL));
    let secondary_region_for_diag = secondary_region
        .clone()
        .unwrap_or_else(|| Region::new(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL));

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
        // preserves the zero-overhead happy path (no `Arc` allocation
        // when the primary wins pre-threshold).
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
    // deadline arm enables the application-cancel path; when the
    // deadline has no value the deadline future never resolves and the
    // race collapses to the 2-way primary-vs-threshold semantics.
    let threshold_duration = match azure_core::time::Duration::try_from(threshold.get()) {
        Ok(d) => d,
        Err(_) => {
            // Programmer-error-class invariant violation: a hedge
            // threshold above `Duration::MAX` should be impossible by
            // construction (`HedgeThreshold::new` already guards this).
            // Synthesize the same typed status + diagnostics graft as
            // the other Terminal(Err) paths so operators inspecting
            // `error.diagnostics()` still see what the operation
            // attempted before bailing.
            parent_diagnostics.set_operation_status(
                azure_core::http::StatusCode::InternalServerError,
                Some(SubStatusCode::TRANSPORT_GENERATED_503),
            );
            let diagnostics_ctx = Arc::new(parent_diagnostics.complete());
            return HedgedRaceResult::Terminal(Err(crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message("hedge threshold exceeds azure_core::time::Duration range")
                .with_diagnostics(diagnostics_ctx)
                .build()));
        }
    };
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

    // The post-match value is the primary future the Stage 4 race will
    // consume. We unify the three possible future types (still-pending
    // primary, pre-resolved transient primary, deadline branch returns
    // early) by erasing to a boxed `dyn Future` trait object.
    type PrimaryAttemptFuture<'fut> = Pin<
        Box<
            dyn Future<
                    Output = (
                        crate::error::Result<TransportResult>,
                        DiagnosticsContextBuilder,
                    ),
                > + Send
                + 'fut,
        >,
    >;
    let primary_attempt: PrimaryAttemptFuture<'_> = match select(primary_attempt, timer_event).await
    {
        Either::Left(((result, diag), _timer)) => {
            // Primary resolved before either timer. Short-circuiting
            // the secondary launch is only safe when the primary's
            // outcome is **final** (HTTP success or application-classified
            // error per `CosmosStatus::is_final_result`). A *retriable* outcome
            // (5xx / 429 / 408 / 410 / 404-1002 / 403-with-sub /
            // transport error / deadline) must still race against the
            // secondary so a healthy alternate region can win —
            // otherwise hedging would actively suppress the
            // cross-region failover the operation pipeline would have
            // performed without the hedge upgrade.
            let primary_was_final = matches!(
                &result,
                Ok(tr) if result_is_final(tr),
            );
            if primary_was_final {
                // Zero-overhead happy path — no secondary attempt is
                // ever constructed.
                parent_diagnostics.merge_hedge_attempt(diag);
                parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::primary_only(
                    strategy_config,
                    primary_region_for_diag.clone(),
                ));
                tracing::debug!(
                    activity_id = %ctx.activity_id,
                    "execute_hedged: primary won pre-threshold (zero-overhead happy path)",
                );
                // Structured win event. `was_hedge=false` distinguishes
                // the zero-overhead happy path from a post-threshold
                // primary win.
                tracing::debug!(
                    activity_id = %ctx.activity_id,
                    winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                    was_hedge = false,
                    "cosmos.hedge.winner_selected",
                );
                // Only a `Final` primary outcome resets the
                // consecutive-hedge-win counter on this
                // `(partition, primary_region)` pair. Recording for a
                // `Transient` outcome (a fast 503/449/transport-reset
                // that completes before the threshold) would silently
                // neutralize the PPCB signal for exactly the failure
                // mode it is designed to detect.
                //
                // Prefer the PK range ID parsed from the winning
                // response over `ctx.partition_key_range_id` (which is
                // the pre-race snapshot from the AttemptContext). On
                // the first attempt of an operation the snapshot is
                // `None`, so PPCB feedback would otherwise no-op even
                // though the response now carries the resolved range.
                let pk_range_id_for_feedback =
                    pk_range_id_from_result(&result).or_else(|| ctx.partition_key_range_id.clone());
                record_hedge_outcome(
                    ctx.location_state_store,
                    HedgeOutcome::PrimaryWin,
                    pk_range_id_for_feedback.as_ref(),
                    primary_region.as_ref(),
                );
                // result is Ok by the `primary_was_final` guard above.
                let tr = result.expect("Ok by primary_was_final guard");
                capture_session_token_for_winner(ctx, &tr);
                return HedgedRaceResult::Terminal(finalize_hedge_attempt(
                    Box::new(tr),
                    parent_diagnostics,
                ));
            }
            // Primary resolved pre-threshold but transient (or `Err`). Do
            // not merge diagnostics yet — fall through to Stage 3, wrap
            // the resolved primary into a `ready` future, and let the
            // Stage 4 race merge it on the winning-side path the same
            // way it merges a post-threshold primary completion.
            tracing::debug!(
                activity_id = %ctx.activity_id,
                "execute_hedged: primary completed pre-threshold transient; launching secondary",
            );
            Box::pin(futures::future::ready((result, diag)))
        }
        Either::Right((TimerEvent::ThresholdElapsed, remaining_primary)) => remaining_primary,
        Either::Right((TimerEvent::DeadlineFired, remaining_primary)) => {
            // App-cancel observed at Stage 2 — only the primary is
            // in-flight (no secondary was ever built). Harvest it
            // within HARVEST_WINDOW for diagnostics, then re-raise.
            // No leg produced a final response, so attach the
            // DeadlineExceededPreThreshold terminal state
            // (was_hedge=false) rather than primary_only — the
            // operation surfaces application_cancelled_error and hedge
            // win-rate metrics must not count this as a primary win.
            tracing::debug!(
                activity_id = %ctx.activity_id,
                "execute_hedged: deadline fired pre-threshold; harvesting primary",
            );
            harvest_remaining_attempt(
                remaining_primary,
                &mut parent_diagnostics,
                effective_harvest_window(ctx.configured_request_timeout),
            )
            .await;
            parent_diagnostics.set_hedge_diagnostics(
                HedgeDiagnostics::primary_only_deadline_exceeded(
                    strategy_config,
                    primary_region_for_diag.clone(),
                ),
            );
            return HedgedRaceResult::Terminal(Err(application_cancelled_error(
                parent_diagnostics,
            )));
        }
    };

    // ── Stage 3: Launch secondary ─────────────────────────────────────
    // The cross-hedge shared hub-region-processing-only latch is
    // constructed here — *after* the threshold elapses, so the
    // zero-overhead happy path never allocates the `Arc`. Gated on
    // the same predicates as the per-state latch trigger:
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
    // Structured "alternate spawned" event distinct from the freeform
    // message above. Fields carry the race timing (threshold) and the
    // target region so operators can correlate spawn rate with observed
    // tail-latency improvements.
    tracing::debug!(
        activity_id = %ctx.activity_id,
        threshold_ms = threshold.get().as_millis() as u64,
        secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
        "cosmos.hedge.alternate_spawned",
    );

    // ── Stage 4: Primary vs secondary race ────────────────────────────
    // Per-attempt deadline observation lives inside the transport
    // pipeline — when the end-to-end deadline fires while an
    // attempt is in flight, the transport returns `DeadlineExceeded`
    // which classifies as transient. The two `.await` calls in the
    // transient-fallthrough branches below are wrapped with
    // [`await_attempt_or_deadline_harvest`] so a deadline that fires
    // while we're waiting on the still-pending side triggers the
    // harvest path instead of blocking. The both-transient
    // terminal branches inspect the deadline to choose between
    // [`application_cancelled_error`] and [`transient_outcome_error`].
    //
    // Per-leg signals threaded into `BothTransient` so the failover-loop
    // fallback can rehydrate `retry_state` with what the race observed
    // — see `pk_range_id_from_result` and `apply_hedge_leg_effects`.
    let mut captured_pk_range_id: Option<PartitionKeyRangeId> = None;
    let mut race_observed_session_unavailable = false;
    match select(primary_attempt, secondary_attempt).await {
        Either::Left(((primary_result, primary_diag), secondary_remaining)) => {
            parent_diagnostics.merge_hedge_attempt(primary_diag);
            captured_pk_range_id =
                captured_pk_range_id.or_else(|| pk_range_id_from_result(&primary_result));
            apply_hedge_leg_effects(
                ctx,
                retry_state_snapshot,
                &primary_routing.endpoint,
                &primary_result,
                shared_hub_region_latch.as_ref(),
                &mut race_observed_session_unavailable,
            )
            .await;
            match classify_hedge_result(primary_result) {
                HedgeClass::Final(tr) => {
                    // Primary won post-threshold; drop secondary.
                    parent_diagnostics.set_hedge_diagnostics(
                        HedgeDiagnostics::primary_won_after_hedge(
                            strategy_config,
                            primary_region_for_diag.clone(),
                            secondary_region_for_diag.clone(),
                        ),
                    );
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        "execute_hedged: primary won after threshold",
                    );
                    // Secondary attempt was dropped structurally when
                    // this branch was taken.
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        which = "secondary",
                        target_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "primary_won_post_threshold",
                        "cosmos.hedge.canceled",
                    );
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.winner_selected",
                    );
                    record_hedge_outcome(
                        ctx.location_state_store,
                        HedgeOutcome::PrimaryWin,
                        captured_pk_range_id
                            .as_ref()
                            .or(ctx.partition_key_range_id.as_ref()),
                        primary_region.as_ref(),
                    );
                    capture_session_token_for_winner(ctx, &tr);
                    HedgedRaceResult::Terminal(finalize_hedge_attempt(tr, parent_diagnostics))
                }
                HedgeClass::Transient => {
                    // Primary transient — wait for secondary (observing deadline).
                    let Some((secondary_result, secondary_diag)) =
                        await_attempt_or_deadline_harvest(
                            secondary_remaining,
                            ctx.deadline,
                            &mut parent_diagnostics,
                            effective_harvest_window(ctx.configured_request_timeout),
                        )
                        .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.activity_id,
                            "execute_hedged: deadline fired awaiting secondary after primary transient",
                        );
                        // No leg produced a final response — attach
                        // CancelledAwaitingPartner terminal state
                        // (was_hedge=false) so hedge win-rate metrics
                        // do not count this as an alternate win.
                        parent_diagnostics.set_hedge_diagnostics(
                            HedgeDiagnostics::cancelled_awaiting_partner(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ),
                        );
                        return HedgedRaceResult::Terminal(Err(application_cancelled_error(
                            parent_diagnostics,
                        )));
                    };
                    parent_diagnostics.merge_hedge_attempt(secondary_diag);
                    captured_pk_range_id =
                        captured_pk_range_id.or_else(|| pk_range_id_from_result(&secondary_result));
                    apply_hedge_leg_effects(
                        ctx,
                        retry_state_snapshot,
                        &secondary_routing.endpoint,
                        &secondary_result,
                        shared_hub_region_latch.as_ref(),
                        &mut race_observed_session_unavailable,
                    )
                    .await;
                    match classify_hedge_result(secondary_result) {
                        HedgeClass::Final(tr) => {
                            parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ));
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                "execute_hedged: secondary won after primary transient",
                            );
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.winner_selected",
                            );
                            record_hedge_outcome(
                                ctx.location_state_store,
                                HedgeOutcome::AlternateWin,
                                captured_pk_range_id
                                    .as_ref()
                                    .or(ctx.partition_key_range_id.as_ref()),
                                primary_region.as_ref(),
                            );
                            capture_session_token_for_winner(ctx, &tr);
                            HedgedRaceResult::Terminal(finalize_hedge_attempt(
                                tr,
                                parent_diagnostics,
                            ))
                        }
                        HedgeClass::Transient => {
                            // Both legs transient. Delegate to the helper
                            // so the diagnostics-attachment + tracing +
                            // outcome classification stay in one place.
                            // When the deadline has NOT elapsed this
                            // returns `BothTransient` so the operation
                            // pipeline can re-enter the failover loop
                            // against the remaining regions.
                            finalize_both_transient(
                                ctx.activity_id,
                                ctx.deadline,
                                strategy_config,
                                primary_region,
                                secondary_region,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                                parent_diagnostics,
                                captured_pk_range_id,
                                race_observed_session_unavailable,
                            )
                        }
                    }
                }
            }
        }
        Either::Right(((secondary_result, secondary_diag), primary_remaining)) => {
            parent_diagnostics.merge_hedge_attempt(secondary_diag);
            captured_pk_range_id =
                captured_pk_range_id.or_else(|| pk_range_id_from_result(&secondary_result));
            apply_hedge_leg_effects(
                ctx,
                retry_state_snapshot,
                &secondary_routing.endpoint,
                &secondary_result,
                shared_hub_region_latch.as_ref(),
                &mut race_observed_session_unavailable,
            )
            .await;
            match classify_hedge_result(secondary_result) {
                HedgeClass::Final(tr) => {
                    parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                        strategy_config,
                        primary_region_for_diag.clone(),
                        secondary_region_for_diag.clone(),
                    ));
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        "execute_hedged: secondary won race",
                    );
                    // Primary attempt was dropped structurally when
                    // secondary won the race.
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        which = "primary",
                        target_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "secondary_won_race",
                        "cosmos.hedge.canceled",
                    );
                    tracing::debug!(
                        activity_id = %ctx.activity_id,
                        winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.winner_selected",
                    );
                    record_hedge_outcome(
                        ctx.location_state_store,
                        HedgeOutcome::AlternateWin,
                        captured_pk_range_id
                            .as_ref()
                            .or(ctx.partition_key_range_id.as_ref()),
                        primary_region.as_ref(),
                    );
                    capture_session_token_for_winner(ctx, &tr);
                    HedgedRaceResult::Terminal(finalize_hedge_attempt(tr, parent_diagnostics))
                }
                HedgeClass::Transient => {
                    // Secondary transient — wait for primary (observing deadline).
                    let Some((primary_result, primary_diag)) = await_attempt_or_deadline_harvest(
                        primary_remaining,
                        ctx.deadline,
                        &mut parent_diagnostics,
                        effective_harvest_window(ctx.configured_request_timeout),
                    )
                    .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.activity_id,
                            "execute_hedged: deadline fired awaiting primary after secondary transient",
                        );
                        // No leg produced a final response — attach
                        // CancelledAwaitingPartner terminal state
                        // (was_hedge=false) so hedge win-rate metrics
                        // do not count this as an alternate win.
                        parent_diagnostics.set_hedge_diagnostics(
                            HedgeDiagnostics::cancelled_awaiting_partner(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ),
                        );
                        return HedgedRaceResult::Terminal(Err(application_cancelled_error(
                            parent_diagnostics,
                        )));
                    };
                    parent_diagnostics.merge_hedge_attempt(primary_diag);
                    captured_pk_range_id =
                        captured_pk_range_id.or_else(|| pk_range_id_from_result(&primary_result));
                    apply_hedge_leg_effects(
                        ctx,
                        retry_state_snapshot,
                        &primary_routing.endpoint,
                        &primary_result,
                        shared_hub_region_latch.as_ref(),
                        &mut race_observed_session_unavailable,
                    )
                    .await;
                    match classify_hedge_result(primary_result) {
                        HedgeClass::Final(tr) => {
                            parent_diagnostics.set_hedge_diagnostics(
                                HedgeDiagnostics::primary_won_after_hedge(
                                    strategy_config,
                                    primary_region_for_diag.clone(),
                                    secondary_region_for_diag.clone(),
                                ),
                            );
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                "execute_hedged: primary won after secondary transient",
                            );
                            tracing::debug!(
                                activity_id = %ctx.activity_id,
                                winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.winner_selected",
                            );
                            record_hedge_outcome(
                                ctx.location_state_store,
                                HedgeOutcome::PrimaryWin,
                                captured_pk_range_id
                                    .as_ref()
                                    .or(ctx.partition_key_range_id.as_ref()),
                                primary_region.as_ref(),
                            );
                            capture_session_token_for_winner(ctx, &tr);
                            HedgedRaceResult::Terminal(finalize_hedge_attempt(
                                tr,
                                parent_diagnostics,
                            ))
                        }
                        HedgeClass::Transient => {
                            // Both legs transient. Same loop-fallback
                            // semantics as the symmetric Either::Left arm
                            // — see [`finalize_both_transient`].
                            finalize_both_transient(
                                ctx.activity_id,
                                ctx.deadline,
                                strategy_config,
                                primary_region,
                                secondary_region,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                                parent_diagnostics,
                                captured_pk_range_id,
                                race_observed_session_unavailable,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Generic "both sides transient" error carried inside
/// [`HedgedRaceResult::BothTransient`] when neither leg produced a
/// final response and the deadline has not elapsed. The surrounding
/// operation pipeline will only surface this error to the caller if
/// the failover-retry budget has also been exhausted by the race —
/// otherwise the loop re-enters against the remaining regions.
///
/// The two raced regions are included in the message so operators
/// investigating "why didn't hedging save me?" can see at a glance
/// which two regions burnt RU on the failed race. `None` is rendered
/// as the [`HedgeDiagnostics::UNKNOWN_REGION_SENTINEL`] string for
/// consistency with the diagnostics-attachment surface.
///
/// Status is set to [`CosmosStatus::TRANSPORT_GENERATED_503`] so
/// retry-evaluation and telemetry can discriminate a client-side
/// "both legs transient" classification from any other 5xx surface.
/// Diagnostics are not threaded here: the surrounding
/// `BothTransient` flow returns a builder that the failover loop
/// keeps mutating; only the terminal error-surfacing path (when the
/// failover budget is exhausted) attaches diagnostics, and it does
/// so from the loop's own builder via the standard error-promotion
/// path.
fn transient_outcome_error(
    primary_region: Option<&Region>,
    secondary_region: Option<&Region>,
) -> crate::error::CosmosError {
    let p = primary_region
        .map(Region::as_str)
        .unwrap_or(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL);
    let s = secondary_region
        .map(Region::as_str)
        .unwrap_or(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL);
    crate::error::CosmosError::builder()
        .with_status(crate::models::CosmosStatus::TRANSPORT_GENERATED_503)
        .with_message(format!(
            "hedging completed without producing a final response \
             (primary={p}, secondary={s})"
        ))
        .build()
}

/// Centralizes the diagnostics attachment, structured tracing, and
/// outcome classification for the **both-legs-transient** terminal
/// state of [`execute_hedged`]'s Stage 4 race.
///
/// Returns:
///
/// - [`HedgedRaceResult::Terminal`] with [`application_cancelled_error`]
///   when the application deadline has elapsed (no remaining time
///   budget to spend on the failover-loop fallback).
/// - [`HedgedRaceResult::BothTransient`] otherwise, threading
///   `parent_diagnostics` back so the surrounding failover loop can
///   keep accumulating per-attempt request traces into the same
///   operation-scoped diagnostics chain.
///
/// `primary_region` / `secondary_region` are the original (potentially
/// `None`-for-global-endpoint) regions surfaced inside
/// `BothTransient` for telemetry; `primary_region_for_diag` /
/// `secondary_region_for_diag` are the
/// [`HedgeDiagnostics::UNKNOWN_REGION_SENTINEL`] sentinels used at the
/// `HedgeDiagnostics` attachment site so the attachment contract
/// holds even on global-endpoint accounts.
///
/// `partition_key_range_id` and `observed_session_unavailable` are
/// per-leg signals accumulated during the race; both are surfaced
/// verbatim on `HedgedRaceResult::BothTransient` for the failover
/// loop's write-back contract (see the variant docs).
#[allow(
    clippy::too_many_arguments,
    reason = "All ten values come from `execute_hedged`'s state and \
              are passed through to a single helper call site. Bundling \
              them into a struct would add an indirection without \
              improving readability — the two-region pairing (raw / \
              for-diag) is intentional and explicit at the call site."
)]
fn finalize_both_transient(
    activity_id: &ActivityId,
    deadline: Option<Instant>,
    strategy_config: HedgingStrategyConfig,
    primary_region: Option<Region>,
    secondary_region: Option<Region>,
    primary_region_for_diag: Region,
    secondary_region_for_diag: Region,
    mut parent_diagnostics: DiagnosticsContextBuilder,
    partition_key_range_id: Option<PartitionKeyRangeId>,
    observed_session_unavailable: bool,
) -> HedgedRaceResult {
    let deadline_was_elapsed = deadline_elapsed(deadline);
    tracing::warn!(
        activity_id = %activity_id,
        deadline_elapsed = deadline_was_elapsed,
        primary_region = ?primary_region.as_ref().map(Region::as_str),
        secondary_region = ?secondary_region.as_ref().map(Region::as_str),
        "cosmos.hedge.both_transient",
    );
    if deadline_was_elapsed {
        // Terminal both-transient under elapsed deadline — stamp
        // BothTransient here because the application_cancelled_error
        // path is the operation's final answer.
        parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::both_transient(
            strategy_config,
            primary_region_for_diag,
            secondary_region_for_diag,
            deadline_was_elapsed,
        ));
        tracing::debug!(
            activity_id = %activity_id,
            "execute_hedged: both transient under elapsed deadline; surfacing app-cancel",
        );
        HedgedRaceResult::Terminal(Err(application_cancelled_error(parent_diagnostics)))
    } else {
        // Non-terminal: the failover loop will run more attempts. Do not
        // stamp HedgeDiagnostics here — a later successful retry would
        // carry a misleading `terminal_state = BothTransient`. Instead,
        // carry `strategy_config` and the `*_for_diag` regions on the
        // variant so the fallback can stamp it only if the failover budget
        // turns out to be exhausted.
        tracing::debug!(
            activity_id = %activity_id,
            primary_region = ?primary_region.as_ref().map(Region::as_str),
            secondary_region = ?secondary_region.as_ref().map(Region::as_str),
            observed_session_unavailable,
            "execute_hedged: both legs transient; bubbling up for failover-loop fallback",
        );
        HedgedRaceResult::BothTransient {
            last_error: transient_outcome_error(primary_region.as_ref(), secondary_region.as_ref()),
            primary_region,
            secondary_region,
            strategy_config,
            primary_region_for_diag,
            secondary_region_for_diag,
            diagnostics: parent_diagnostics,
            partition_key_range_id,
            observed_session_unavailable,
        }
    }
}

/// Handles the [`HedgedRaceResult::BothTransient`] outcome of
/// [`execute_hedged`] by advancing `retry_state` so the surrounding
/// failover loop can re-enter against the remaining regions.
///
/// Returns:
///
/// - `Ok(())` when the budget allowed advancing (caller `continue`s
///   the loop).
/// - `Err(last_error)` when `failover_retry_count + 2 >
///   max_failover_retries` (no remaining budget; caller surfaces
///   `last_error` as the operation's terminal error).
///
/// # Region accounting
///
/// The race consumed exactly two regions (the primary at the
/// snapshot's current `LocationIndex` and the secondary picked by
/// `evaluate_hedge_eligibility`). We charge two slots against
/// `failover_retry_count` and walk the `LocationIndex` forward until
/// it lands on an endpoint whose region matches neither the raced
/// primary nor the raced secondary, so the next iteration's
/// `resolve_endpoint` selects a region not yet tried in this race.
///
/// Walking forward (rather than blindly advancing by a fixed offset)
/// matters because the secondary picker no longer guarantees the
/// secondary sits at `primary_index + 1` — it scans the preferred
/// endpoints from the front and returns the first endpoint whose
/// region and endpoint key differ from the primary. With a fixed
/// `+2` advance the post-race `LocationIndex` could land back on the
/// just-failed primary or secondary (or skip an untried region) when
/// the primary was promoted to a non-zero index by prior retry state.
///
/// # Why two slots, not one?
///
/// A non-hedged operation that hit transients on the same two regions
/// would have consumed two failover-retry slots — one per region.
/// Charging the same amount here keeps the hedge-eligible path's
/// worst-case region coverage identical to the non-hedged path's.
/// Charging only one would let pathologically slow regions occupy
/// both slots of every race indefinitely; charging two converges on
/// the third region in bounded time exactly as the non-hedged loop
/// does.
fn try_advance_after_both_transient(
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
    let endpoints = preferred_endpoints_for_attempt(
        location.account.as_ref(),
        retry_state,
        is_read_only,
        !is_read_only,
        false,
    );
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

/// Flips the per-state and shared `hub_region_processing_only` latches
/// when a hedge leg observed a 1002 the non-hedged
/// `build_session_retry_state` would have latched on, and advances
/// `session_token_retry_count` symmetrically so `max_session_retries`
/// is honored across both non-hedged and hedge-fallback flows.
fn propagate_hedge_session_unavailable(
    retry_state: &mut OperationRetryState,
    observed_session_unavailable: bool,
) {
    if !observed_session_unavailable {
        return;
    }
    // Mirror `.advance_session_retry()` in the non-hedged path so the
    // hedge fallback can't exceed `max_session_retries`. Gate on
    // `can_retry_session` to avoid overflowing past the cap.
    if retry_state.can_retry_session() {
        retry_state.session_token_retry_count =
            retry_state.session_token_retry_count.saturating_add(1);
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
    if let Some(shared) = retry_state.shared_hub_region_latch.as_ref() {
        shared.store(true, Ordering::Release);
    }
    tracing::debug!(
        session_token_retry_count = retry_state.session_token_retry_count,
        max_session_retries = retry_state.max_session_retries,
        "hedge both-transient: 1002 observed by at least one leg; \
         flipped hub_region_processing_only latch and advanced \
         session-retry counter for next attempt",
    );
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use azure_core::http::headers::HeaderName;
    use url::Url;

    use super::build_transport_request;
    use super::OperationOverrides;
    use super::TransportRequestContext;
    use crate::{
        diagnostics::ExecutionContext,
        driver::{
            pipeline::components::{RoutingDecision, TransportMode},
            routing::{
                AccountEndpointState, CosmosEndpoint, LocationEffect, LocationIndex,
                LocationSnapshot,
            },
            transport::EndpointKey,
        },
        models::{
            request_header_names, AccountReference, ActivityId, ContainerProperties,
            ContainerReference, CosmosOperation, DatabaseReference, DefaultConsistencyLevel,
            EffectivePartitionKey, FeedRange, ItemReference, PartitionKey, PartitionKeyDefinition,
            PartitionKeyValue, SystemProperties,
        },
        options::{PriorityLevel, ResolvedThroughputControl},
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

    /// Supplying more partition-key components than the container's single-path
    /// definition declares must surface as a `BadRequest` from the EPK
    /// precomputation, never silently hash the extras into a broken EPK. This
    /// validation runs in the operation pipeline (before the transport
    /// pipeline) so wire layers receive a ready-to-encode EPK.
    #[test]
    fn effective_partition_key_rejects_too_many_components() {
        let partition_key = PartitionKey::from(vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::from("extra".to_string()),
        ]);
        let item = ItemReference::from_name(&test_container(), partition_key, "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let error = super::effective_partition_key_for_request(&operation)
            .expect_err("too many components must error");

        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::CLIENT_BAD_REQUEST
        );
    }

    /// A single-component key matching the single-path definition yields a
    /// point EPK (no error, `Some` value).
    #[test]
    fn effective_partition_key_for_matching_components() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let epk = super::effective_partition_key_for_request(&operation)
            .expect("matching components must compute")
            .expect("partition key present yields Some");

        assert!(!epk.as_bytes().is_empty());
    }

    fn test_routing() -> RoutingDecision {
        let endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        RoutingDecision {
            selected_url: endpoint.url().clone(),
            endpoint_key: endpoint.endpoint_key(),
            endpoint,
            transport_mode: TransportMode::Gateway,
        }
    }

    #[test]
    fn apply_headers_pk_range_only_omits_read_key_type() {
        let overrides = OperationOverrides {
            partition_key_range_id: Some("0".to_string()),
            ..Default::default()
        };
        let mut headers = azure_core::http::headers::Headers::new();
        overrides
            .apply_headers(&mut headers, false)
            .expect("apply_headers should succeed");

        assert_eq!(
            headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::PARTITION_KEY_RANGE_ID
                ))
                .map(|s| s.to_string()),
            Some("0".to_string())
        );
        assert!(
            headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::READ_FEED_KEY_TYPE
                ))
                .is_none(),
            "whole-PK-range targets must not emit x-ms-read-key-type"
        );
        assert!(headers
            .get_optional_str(&HeaderName::from_static(request_header_names::START_EPK))
            .is_none());
        assert!(headers
            .get_optional_str(&HeaderName::from_static(request_header_names::END_EPK))
            .is_none());
    }

    #[test]
    fn apply_headers_feed_range_emits_read_key_type_and_epk_bounds() {
        let feed_range = FeedRange::new(
            EffectivePartitionKey::from("10"),
            EffectivePartitionKey::from("20"),
        )
        .unwrap();
        let overrides = OperationOverrides {
            partition_key_range_id: Some("pkrange".to_string()),
            feed_range: Some(feed_range),
            ..Default::default()
        };
        let mut headers = azure_core::http::headers::Headers::new();
        overrides
            .apply_headers(&mut headers, false)
            .expect("apply_headers should succeed");

        assert_eq!(
            headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::READ_FEED_KEY_TYPE
                ))
                .map(|s| s.to_string()),
            Some(request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE.to_string())
        );
        assert_eq!(
            headers
                .get_optional_str(&HeaderName::from_static(request_header_names::START_EPK))
                .map(|s| s.to_string()),
            Some("10".to_string())
        );
        assert_eq!(
            headers
                .get_optional_str(&HeaderName::from_static(request_header_names::END_EPK))
                .map(|s| s.to_string()),
            Some("20".to_string())
        );
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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let overrides = OperationOverrides {
            partition_key: Some(PartitionKey::from("pk1")),
            ..Default::default()
        };
        let request = build_transport_request(&operation, &overrides, None, &ctx)
            .expect("request should build");

        let partition_key_header = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-documentdb-partitionkey"))
            .expect("partition key header should be set");
        assert_eq!(partition_key_header, "[\"pk1\"]");
    }

    #[test]
    fn build_transport_request_change_feed_continuation_uses_if_none_match() {
        let pk_def = test_partition_key_definition("/partition_key");
        let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
        let operation = CosmosOperation::change_feed(test_container(), Some(target));
        assert!(operation.is_change_feed());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Retry,
            deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let overrides = OperationOverrides {
            partition_key: Some(PartitionKey::from("pk1")),
            continuation: Some("\"etag-123\"".to_string()),
            ..Default::default()
        };
        let request = build_transport_request(&operation, &overrides, None, &ctx)
            .expect("request should build");

        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::IF_NONE_MATCH
                ))
                .map(|s| s.to_string()),
            Some("\"etag-123\"".to_string()),
            "change feed continuation must be sent as If-None-Match"
        );
        assert!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(request_header_names::CONTINUATION))
                .is_none(),
            "change feed must not send x-ms-continuation"
        );
        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION
                ))
                .map(|s| s.to_string()),
            Some(request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15.to_string()),
            "change feed must send the wire-format-version header"
        );
    }

    #[test]
    fn build_transport_request_change_feed_continuation_drops_if_modified_since() {
        // PointInTime start sets If-Modified-Since on the shared operation.
        // Once a partition has a continuation (ETag) it must resume purely
        // from that ETag (sent as If-None-Match); the stale start marker must
        // not co-exist, otherwise the request carries two conflicting
        // position headers.
        let pk_def = test_partition_key_definition("/partition_key");
        let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
        let operation = CosmosOperation::change_feed(test_container(), Some(target))
            .with_if_modified_since("Mon, 01 Jan 2024 00:00:00 GMT".to_string());
        assert!(operation.is_change_feed());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Retry,
            deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let overrides = OperationOverrides {
            partition_key: Some(PartitionKey::from("pk1")),
            continuation: Some("\"etag-123\"".to_string()),
            ..Default::default()
        };
        let request = build_transport_request(&operation, &overrides, None, &ctx)
            .expect("request should build");

        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::IF_NONE_MATCH
                ))
                .map(|s| s.to_string()),
            Some("\"etag-123\"".to_string()),
            "continuation must be sent as If-None-Match"
        );
        assert!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::IF_MODIFIED_SINCE
                ))
                .is_none(),
            "stale PointInTime start marker must be dropped once a continuation is present"
        );
    }

    #[test]
    fn build_transport_request_change_feed_keeps_if_modified_since_without_continuation() {
        // A partition that has never been polled (no continuation) must still
        // honor the PointInTime start marker.
        let pk_def = test_partition_key_definition("/partition_key");
        let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
        let operation = CosmosOperation::change_feed(test_container(), Some(target))
            .with_if_modified_since("Mon, 01 Jan 2024 00:00:00 GMT".to_string());

        let routing = test_routing();
        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Retry,
            deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let overrides = OperationOverrides {
            partition_key: Some(PartitionKey::from("pk1")),
            ..Default::default()
        };
        let request = build_transport_request(&operation, &overrides, None, &ctx)
            .expect("request should build");

        assert_eq!(
            request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::IF_MODIFIED_SINCE
                ))
                .map(|s| s.to_string()),
            Some("Mon, 01 Jan 2024 00:00:00 GMT".to_string()),
            "PointInTime start marker must be kept when no continuation is present"
        );
    }

    #[test]
    fn build_transport_request_uses_routed_endpoint_url_directly() {
        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let selected_url =
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap();
        let routing = RoutingDecision {
            endpoint: CosmosEndpoint::regional_with_gateway_v2(
                "westus2".into(),
                Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
                selected_url.clone(),
            ),
            endpoint_key: EndpointKey::try_from(&selected_url).unwrap(),
            selected_url,
            transport_mode: TransportMode::GatewayV2,
        };

        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

        assert_eq!(
            request.url.as_str(),
            "https://test-westus2-thin.documents.azure.com:444/dbs/mydb"
        );
    }

    #[test]
    fn build_transport_request_uses_default_url_for_global_endpoint() {
        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let routing = RoutingDecision {
            selected_url: endpoint.url().clone(),
            endpoint_key: endpoint.endpoint_key(),
            endpoint,
            transport_mode: TransportMode::Gateway,
        };

        let activity_id = ActivityId::from_string("default-activity".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            account_write_endpoints: vec![write_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: write_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 1,
            retry_with_state: None,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, write_endpoint);
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn resolve_endpoint_uses_account_write_region_order_for_read_dtx() {
        let operation = CosmosOperation::distributed_transaction(
            test_account(),
            crate::models::DistributedTransactionType::Read,
        );
        let account_write_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let preferred_write_endpoint = CosmosEndpoint::regional(
            "westus3".into(),
            Url::parse("https://test-westus3.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint].into(),
            preferred_write_endpoints: vec![preferred_write_endpoint].into(),
            account_write_endpoints: vec![account_write_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: account_write_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            Vec::new(),
            3,
            1,
        );

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, account_write_endpoint);
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn resolve_endpoint_uses_account_write_region_order_for_write_dtx() {
        let operation = CosmosOperation::distributed_transaction(
            test_account(),
            crate::models::DistributedTransactionType::Write,
        );
        let account_write_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let preferred_write_endpoint = CosmosEndpoint::regional(
            "westus3".into(),
            Url::parse("https://test-westus3.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint].into(),
            preferred_write_endpoints: vec![preferred_write_endpoint].into(),
            account_write_endpoints: vec![account_write_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: account_write_endpoint.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            1,
        );
        retry_state.ppaf_write_retry_allowed = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, account_write_endpoint);
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
            account_write_endpoints: vec![default_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            retry_with_state: None,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
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
            account_write_endpoints: vec![read_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: read_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            retry_with_state: None,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
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
            account_write_endpoints: vec![
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
            retry_with_state: None,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
        };

        let first_routing = super::resolve_endpoint(
            &operation,
            &stale_retry_state,
            &location,
            false,
            true,
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
            true,
            Duration::from_secs(60),
        );
        assert_eq!(second_routing.endpoint, endpoint_b);
    }

    // ── Hub region cache warm-path ─────────────────────────────────────

    /// Builds a single-master `LocationSnapshot` with regional
    /// endpoints and a populated hub-region cache pointing the supplied
    /// partition key range at `hub_endpoint`.
    fn hub_cache_location_snapshot(
        pk_range_id: &str,
        hub_endpoint: &CosmosEndpoint,
    ) -> (LocationSnapshot, CosmosEndpoint, CosmosEndpoint) {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };

        let eastus = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let westus = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
            preferred_write_endpoints: vec![eastus.clone()].into(),
            account_write_endpoints: vec![eastus.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: eastus.clone(),
        });

        let mut partitions = PartitionEndpointState::default();
        partitions.per_partition_automatic_failover_enabled = true;
        partitions.failover_overrides.insert(
            pk_range_id.parse().unwrap(),
            PartitionFailoverEntry {
                current_endpoint: hub_endpoint.clone(),
                first_failed_endpoint: hub_endpoint.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        (
            LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions)),
            eastus,
            westus,
        )
    }

    /// Builds a retry state representing a single-master data-plane read
    /// that has latched the hub-region header on the first 1002 and has
    /// resolved a partition key range ID from the response.
    fn read_state_with_hub_latch(pk_range_id: &str) -> super::OperationRetryState {
        super::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 1,
            max_failover_retries: 3,
            max_session_retries: 3,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_backend_failover_retries:
                crate::driver::pipeline::components::MAX_BACKEND_FAILOVER_RETRIES,
            can_use_multiple_write_locations: false,
            is_dataplane: true,
            hub_region_processing_only: true,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredWriteEndpoints,
            partition_key_range_id: Some(pk_range_id.parse().unwrap()),
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
            retry_with_state: None,
        }
    }

    #[test]
    fn resolve_endpoint_routes_to_cached_hub_when_latch_set() {
        let pk_range = "0";
        let hub = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let (location, _eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let retry_state = read_state_with_hub_latch(pk_range);

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_eq!(
            routing.endpoint, hub,
            "warm cache hit must route directly to the cached hub region",
        );
    }

    #[test]
    fn resolve_endpoint_ignores_hub_cache_when_latch_not_set() {
        let pk_range = "0";
        let hub = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let (location, eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let mut retry_state = read_state_with_hub_latch(pk_range);
        retry_state.hub_region_processing_only = false; // latch off
        retry_state.session_retry_routing =
            crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints;
        retry_state.session_token_retry_count = 0;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_eq!(
            routing.endpoint, eastus,
            "without the latch, normal selection picks the first preferred read endpoint",
        );
    }

    #[test]
    fn resolve_endpoint_ignores_hub_cache_when_pk_range_id_missing() {
        let pk_range = "0";
        let hub = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let (location, _eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let mut retry_state = read_state_with_hub_latch(pk_range);
        // PK range ID not yet captured from a response header.
        retry_state.partition_key_range_id = None;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_ne!(
            routing.endpoint, hub,
            "without partition_key_range_id we cannot key into the cache",
        );
    }

    /// Hub-region warm-path routing is gated on
    /// `per_partition_automatic_failover_enabled` — even when the latch is
    /// set and the cache contains an entry, an account without PPAF must
    /// not be silently routed by the hub cache. Mirrors the
    /// `cache_hub_region_skips_when_ppaf_disabled` writer-side gate.
    #[test]
    fn resolve_endpoint_ignores_hub_cache_when_ppaf_disabled() {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };

        let pk_range = "0";
        let eastus = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let westus = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
            preferred_write_endpoints: vec![eastus.clone()].into(),
            account_write_endpoints: vec![eastus.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: eastus.clone(),
        });

        // PPAF is OFF on this partition state but the cache nevertheless
        // contains a hub entry (e.g., a stale entry from a feature flip).
        let mut partitions = PartitionEndpointState::default();
        partitions.per_partition_automatic_failover_enabled = false;
        partitions.failover_overrides.insert(
            pk_range.parse().unwrap(),
            PartitionFailoverEntry {
                current_endpoint: westus.clone(),
                first_failed_endpoint: westus.clone(),
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

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let retry_state = read_state_with_hub_latch(pk_range);

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_ne!(
            routing.endpoint, westus,
            "warm-path hub cache must not route when PPAF is disabled on the partition state",
        );
    }

    /// Hub-region warm-path routing honors the caller's `excluded_regions`:
    /// even with the latch set, PPAF enabled, and a cache entry present, a
    /// cached hub endpoint whose region the caller excluded must not be
    /// selected. Selection falls through to the default path, which also
    /// honors the exclusion. Mirrors the PPCB read-side exclusion gate.
    #[test]
    fn resolve_endpoint_hub_cache_honors_excluded_regions() {
        let pk_range = "0";
        let hub = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let (location, eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let mut retry_state = read_state_with_hub_latch(pk_range);
        // Caller pins reads away from the cached hub region.
        retry_state.excluded_regions = vec![crate::options::Region::from("westus")];

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_ne!(
            routing.endpoint, hub,
            "warm-path hub cache must not route to an excluded region",
        );
        assert_eq!(
            routing.endpoint, eastus,
            "selection must fall through to the non-excluded preferred region",
        );
    }

    /// Hub-region warm-path routing skips a cached hub endpoint that has been
    /// marked unavailable account-wide and falls through to the next available
    /// read endpoint. (Validates #604.)
    #[test]
    fn resolve_endpoint_hub_cache_skips_unavailable_hub() {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::driver::routing::UnavailableReason;

        let pk_range = "0";
        let eastus = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let westus = CosmosEndpoint::regional(
            "westus".into(),
            Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );

        // The cache points the partition at the westus hub, but westus has
        // just been marked unavailable for a non-write reason.
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            westus.url().clone(),
            (
                std::time::Instant::now(),
                UnavailableReason::ServiceUnavailable,
            ),
        );
        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
            preferred_write_endpoints: vec![eastus.clone()].into(),
            account_write_endpoints: vec![eastus.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: eastus.clone(),
        });
        let mut partitions = PartitionEndpointState::default();
        partitions.per_partition_automatic_failover_enabled = true;
        partitions.failover_overrides.insert(
            pk_range.parse().unwrap(),
            PartitionFailoverEntry {
                current_endpoint: westus.clone(),
                first_failed_endpoint: westus.clone(),
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

        let operation =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
        let retry_state = read_state_with_hub_latch(pk_range);

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        assert_ne!(
            routing.endpoint, westus,
            "warm-path hub cache must not route to an unavailable hub endpoint",
        );
        assert_eq!(
            routing.endpoint, eastus,
            "with the hub unavailable, selection falls through to the next available read endpoint",
        );
    }

    #[test]
    fn hub_region_cache_populate_target_gates_emission() {
        let pk = "0";
        let write_op = CosmosOperation::create_database(test_account());
        let read_op =
            CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));

        // All three conditions met → returns Some(pk).
        let all_met = read_state_with_hub_latch(pk);
        assert_eq!(
            super::hub_region_cache_populate_target(&all_met, &read_op),
            Some(pk.parse().unwrap()),
            "populate gate must fire when latch + pk_range_id + read are all present",
        );

        // Latch off → None.
        let mut no_latch = read_state_with_hub_latch(pk);
        no_latch.hub_region_processing_only = false;
        assert!(
            super::hub_region_cache_populate_target(&no_latch, &read_op).is_none(),
            "populate gate must NOT fire when the hub-region latch is off",
        );

        // No partition key range → None.
        let mut no_pk = read_state_with_hub_latch(pk);
        no_pk.partition_key_range_id = None;
        assert!(
            super::hub_region_cache_populate_target(&no_pk, &read_op).is_none(),
            "populate gate must NOT fire without a partition_key_range_id (cache cannot be keyed)",
        );

        // Write op → None (writes use PPAF write-side routing, not the hub cache).
        assert!(
            super::hub_region_cache_populate_target(&all_met, &write_op).is_none(),
            "populate gate must NOT fire on write operations",
        );
    }

    mod should_capture_session_token_from_status_tests {
        use azure_core::http::StatusCode;

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
    fn resolve_endpoint_prefers_gateway_v2_for_dataplane_reads() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            account_write_endpoints: vec![endpoint.clone()].into(),
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
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, endpoint);
        assert_eq!(routing.transport_mode, TransportMode::GatewayV2);
        assert_eq!(
            routing.selected_url.as_str(),
            "https://test-westus2-thin.documents.azure.com:444/"
        );
    }

    #[test]
    fn resolve_endpoint_falls_back_to_gateway_when_op_ineligible_for_gateway_v2() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            account_write_endpoints: vec![endpoint.clone()].into(),
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
            true,
            Duration::from_secs(60),
        );

        assert_eq!(routing.transport_mode, TransportMode::Gateway);
        assert_eq!(routing.selected_url, *endpoint.url());
    }

    #[test]
    fn resolve_endpoint_falls_back_to_gateway_when_account_name_unparseable() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            account_write_endpoints: vec![endpoint.clone()].into(),
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
            false,
            Duration::from_secs(60),
        );

        assert_eq!(routing.transport_mode, TransportMode::Gateway);
        assert_eq!(routing.selected_url, *endpoint.url());
    }

    #[test]
    fn resolve_endpoint_uses_gateway_v2_authority_for_endpoint_key() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let gateway_v2_url = Url::parse("https://central.gateway_v2.azure.com:444/").unwrap();
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
            "centralus".into(),
            Url::parse("https://central.documents.azure.com:443/").unwrap(),
            gateway_v2_url.clone(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![endpoint.clone()].into(),
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            account_write_endpoints: vec![endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: endpoint,
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
            true,
            Duration::from_secs(60),
        );

        assert_eq!(routing.transport_mode, TransportMode::GatewayV2);
        assert_eq!(
            routing.selected_url.host_str(),
            Some("central.gateway_v2.azure.com")
        );
        assert_eq!(
            routing.endpoint_key,
            EndpointKey::try_from(&gateway_v2_url).unwrap()
        );
    }

    #[test]
    fn resolve_endpoint_skips_unavailable_region_when_gateway_v2_is_present() {
        let operation = CosmosOperation::read_item(ItemReference::from_name(
            &test_container(),
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
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
            preferred_write_endpoints: vec![endpoint.clone()].into(),
            account_write_endpoints: vec![endpoint].into(),
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
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, fallback_endpoint);
    }

    #[test]
    fn resolve_endpoint_metadata_falls_back_to_hub_when_all_excluded() {
        // Pipeline-routed metadata operations (e.g. read_all_databases,
        // CreateDatabase, CreateContainer, Offer reads) must fall back to
        // the hub regional endpoint when every preferred region is excluded
        // — never the global endpoint. The global endpoint is only used by
        // account-topology fetches, which bypass this routing path.
        let operation = CosmosOperation::read_all_databases(test_account());
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let hub_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
            account_write_endpoints: vec![hub_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            retry_with_state: None,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: vec!["westus2".into(), "eastus".into()],
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
        };

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, hub_endpoint);
        assert!(!routing.endpoint.is_global());
    }

    #[test]
    fn resolve_endpoint_dataplane_excluded_read_region_routes_to_hub() {
        // Only the read region is excluded — the hub write region is still
        // available, so resolve_endpoint should pick it directly from the
        // write endpoint list (not via the last-resort fallback path).
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let hub_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
            account_write_endpoints: vec![hub_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
            retry_with_state: None,
        };
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(routing.endpoint, hub_endpoint);
        assert!(!routing.endpoint.is_global());
    }

    #[test]
    fn resolve_endpoint_dataplane_all_regions_excluded_falls_back_to_hub() {
        // Both the read region AND the hub write region are excluded.
        // resolve_endpoint must still use the hub write region via the
        // last-resort fallback (preferred_write_endpoints[0]), never the
        // global endpoint.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let hub_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
            account_write_endpoints: vec![hub_endpoint.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: vec!["westus2".into(), "eastus".into()],
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
            retry_with_state: None,
        };
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        // Even with all regions excluded, the hub write region is used as
        // the last-resort fallback for data-plane operations.
        assert_eq!(routing.endpoint, hub_endpoint);
        assert!(!routing.endpoint.is_global());
    }

    #[test]
    fn resolve_endpoint_dataplane_excluded_and_unavailable_still_uses_hub() {
        // When a read endpoint is both user-excluded AND marked unavailable,
        // the data-plane fallback must still use the hub write region.
        // Note: if an endpoint is only unavailable (not excluded),
        // try_select_endpoint already returns it as a deprioritized fallback
        // — this test validates the combined exclusion + unavailability case.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let hub_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        // Mark the only read endpoint as unavailable.
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            read_endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
            account_write_endpoints: vec![hub_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
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
            hedge_already_fired: false,
            retry_with_state: None,
        };
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        // Excluded + unavailable: data-plane op must get the hub write
        // region, not the global endpoint.
        assert_eq!(routing.endpoint, hub_endpoint);
        assert!(!routing.endpoint.is_global());
    }

    #[test]
    fn resolve_endpoint_dataplane_isolated_region_unavailable_retries_same_region() {
        // Region-isolation guarantee: when the caller excludes every region
        // EXCEPT one (intending to confine the workload to that single
        // region), the SDK must NOT silently fall back to the hub region
        // when the chosen region becomes unavailable. Instead the chosen
        // region is returned as a deprioritized-but-not-replaced endpoint
        // (`first_unavailable`), preserving isolation. The caller's retry
        // budget eventually surfaces the error, which is the correct
        // signal — a workload pinned to one region should fail, not silently
        // cross a region boundary.
        //
        // This is the scenario where the customer wants the hub region
        // excluded (e.g. compliance / cost / blast-radius reasons) and the
        // only retained region (WestUS2 here) is unavailable. We must NOT
        // route to EastUS.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);
        let default_endpoint =
            CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
        let hub_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let isolated_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        // Mark the caller's chosen region as unavailable.
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            isolated_endpoint.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        // Account topology: both regions are readable; hub is the write
        // region. The caller's preference puts the isolated region first.
        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![isolated_endpoint.clone(), hub_endpoint.clone()].into(),
            preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
            account_write_endpoints: vec![hub_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 3,
            max_backend_failover_retries: 120,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            // Caller excludes the hub region — they want isolation to
            // WestUS2 only.
            excluded_regions: vec!["eastus".into()],
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
            retry_with_state: None,
        };
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );

        // The isolated region is returned even though it's unavailable —
        // crucially NOT the hub region. This is what preserves
        // region-isolation intent: the caller's retry policy will eventually
        // exhaust and surface the error, but no request ever crosses into
        // the excluded hub region.
        assert_eq!(
            routing.endpoint, isolated_endpoint,
            "region-isolation broken: resolve_endpoint silently fell back \
             to a region the caller excluded"
        );
        assert_ne!(
            routing.endpoint, hub_endpoint,
            "region-isolation broken: resolve_endpoint chose the excluded hub region"
        );
        assert!(!routing.endpoint.is_global());
    }

    #[test]
    fn resolve_endpoint_multi_write_isolated_region_unavailable_read_returns_isolated_not_excluded()
    {
        // Multi-write (multi-master) account, read op.
        // Topology: preferred_read = [r1, r2], preferred_write = [r1, r2]
        //           (both regions readable AND writable).
        // Caller config: excluded_regions = [r1] (pin workload to r2).
        // Service state: r2 marked unavailable in account.unavailable_endpoints.
        //
        // Expected: r2 is returned as `first_unavailable`. We must NOT leak to
        // r1 — even though r1 is in the preferred list and is the "first
        // preferred region", it was hard-excluded by the caller and must
        // never be selected.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);

        let r1 = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let r2 = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            r2.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![r1.clone(), r2.clone()].into(),
            preferred_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            account_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: r1.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            vec!["eastus".into()],
            3,
            2,
        );
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, r2,
            "isolation broken on read: resolve_endpoint did not return the \
             caller's isolated region (r2)"
        );
        assert_ne!(
            routing.endpoint, r1,
            "isolation broken on read: resolve_endpoint silently routed to \
             the excluded region (r1)"
        );
    }

    #[test]
    fn resolve_endpoint_multi_write_isolated_region_unavailable_write_retry_returns_isolated_not_excluded(
    ) {
        // Multi-write (multi-master) account, write op, simulating attempt 2
        // after the first attempt against r2 failed.
        // Topology: preferred_read = [r1, r2], preferred_write = [r1, r2].
        // Caller config: excluded_regions = [r1] (pin workload to r2).
        // Service state: r2 marked unavailable in account.unavailable_endpoints.
        // In-flight state: pending_write_effects contains a partition mark
        //                  for r2, so the in-flight skip set on this attempt
        //                  is [r2].
        //
        // Trace through resolve_endpoint:
        //   pass 1 (skip=[r2]): r1 excluded → continue; r2 in skip →
        //                       continue; returns None.
        //   pass 2 (skip=[]):   r1 excluded → continue; r2 not excluded,
        //                       not in skip, unavailable → recorded as
        //                       first_unavailable; returns Some(r2).
        //   selected = Some(r2). The last-resort
        //   `preferred_write_endpoints[0]` fallback never fires because
        //   pass 2 returned Some.
        //
        // Expected: r2 returned. We must NOT leak to r1.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let r1 = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let r2 = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            r2.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![r1.clone(), r2.clone()].into(),
            preferred_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            account_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: r1.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            vec!["eastus".into()],
            3,
            2,
        );
        retry_state.is_dataplane = true;
        // Simulate one prior failed write attempt against r2 (westus2).
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("westus2"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, r2,
            "isolation broken on write retry: resolve_endpoint did not \
             return the caller's isolated region (r2)"
        );
        assert_ne!(
            routing.endpoint, r1,
            "isolation broken on write retry: resolve_endpoint silently \
             routed to the excluded region (r1) via the last-resort fallback"
        );
    }

    #[test]
    fn resolve_endpoint_single_write_excluded_hub_satellite_unavailable_read_returns_satellite() {
        // Single-write (single-master) account, read op.
        // Topology: preferred_read = [hub, satellite], preferred_write = [hub]
        //           (only hub is writable; both readable).
        // Caller config: excluded_regions = [hub] (pin reads to satellite).
        // Service state: satellite marked unavailable.
        //
        // For READS on a single-write account, the routing primary list is
        // preferred_read_endpoints (which contains BOTH hub and satellite).
        // Excluding hub leaves satellite as the only eligible region, and
        // even though satellite is unavailable, `first_unavailable` returns
        // it. We must NOT leak to hub on reads — the caller asked for
        // isolation, and the satellite-unavailable case is exactly when the
        // caller wants to see the error, not a silent cross-region hop.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item);

        let hub = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let satellite = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            satellite.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::ServiceUnavailable,
            ),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![hub.clone(), satellite.clone()].into(),
            preferred_write_endpoints: vec![hub.clone()].into(),
            account_write_endpoints: vec![hub.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: hub.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            false,
            vec!["eastus".into()],
            3,
            2,
        );
        retry_state.is_dataplane = true;

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            true,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, satellite,
            "isolation broken on single-write read: resolve_endpoint did \
             not return the caller's isolated region (satellite)"
        );
        assert_ne!(
            routing.endpoint, hub,
            "isolation broken on single-write read: resolve_endpoint \
             silently routed to the excluded hub region"
        );
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
            account_write_endpoints: vec![default_endpoint.clone()].into(),
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
            account_write_endpoints: vec![east.clone(), west.clone()].into(),
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
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, west,
            "in-flight skip set must route the retry to westus2"
        );
    }

    #[test]
    fn resolve_endpoint_honors_excluded_regions() {
        // Pins `excluded_regions` as a hard dataplane write filter.
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
            account_write_endpoints: vec![east.clone(), west.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: east.clone(),
        }));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            vec!["westus2".into()],
            3,
            2,
        );
        retry_state.is_dataplane = true;
        // West is excluded; only East should be eligible.
        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, east,
            "excluded_regions must gate selection — got {:?}",
            routing.endpoint
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
            account_write_endpoints: vec![east.clone()].into(),
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
            account_write_endpoints: vec![east.clone()].into(),
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
            account_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        // Build a partition state with PPAF enabled and a stale override
        // entry that points at the now-failing centralus region.
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
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
            account_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
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
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, central,
            "PPAF override with a healthy current_endpoint must be honored"
        );
    }

    // ── PPCB override skip-closure ────────────────────────────────────

    /// Builds a PPCB override that is already over the write-failover threshold.
    fn make_partition_state_with_ppcb_override(
        pk_range_id: &super::PartitionKeyRangeId,
        override_target: CosmosEndpoint,
    ) -> crate::driver::routing::partition_endpoint_state::PartitionEndpointState {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let config = PartitionFailoverOptions::default();
        let mut partitions = PartitionEndpointState::new(config);
        partitions.per_partition_circuit_breaker_enabled = true;
        partitions.circuit_breaker_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: override_target.clone(),
                first_failed_endpoint: override_target,
                failed_endpoints: Default::default(),
                read_failure_count: 100,
                write_failure_count: 100,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        partitions
    }

    #[test]
    fn resolve_endpoint_ppcb_override_skipped_when_in_excluded_regions() {
        // PPCB overrides must not route to caller-excluded regions.
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
            preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
            account_write_endpoints: vec![north.clone(), central.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: north.clone(),
        });

        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);
        retry_state.excluded_regions = vec![crate::options::Region::from("centralus")];

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, north,
            "PPCB override pointing at a caller-excluded region must be skipped; \
             fall-through to primary selection should pick the non-excluded region"
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_skipped_when_current_endpoint_not_in_topology() {
        // Topology refresh does not prune PPCB overrides; this pins the stale-region skip.
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

        // Post-refresh topology dropped `central`; the PPCB override still points there.
        let account = Arc::new(AccountEndpointState {
            generation: 1,
            preferred_read_endpoints: vec![north.clone()].into(),
            preferred_write_endpoints: vec![north.clone()].into(),
            account_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: north.clone(),
        });

        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);
        // No excluded_regions, no in-flight failures, no endpoint mark —
        // `region_not_in_topology` is the only gate that can fire.

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, north,
            "PPCB override pointing at a region dropped from the post-refresh \
             topology must be skipped; fall-through to primary selection should \
             pick the only surviving region (`north`). The override map is not \
             pruned by topology refresh, so `region_not_in_topology` in \
             `ppcb_should_skip` is the only thing preventing a 1008 loop."
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_skipped_when_endpoint_marked_unavailable() {
        // PPCB overrides must not resurrect transport-dead endpoints.
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

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            central.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::TransportError,
            ),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
            preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
            account_write_endpoints: vec![north.clone(), central.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: north.clone(),
        });

        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, north,
            "PPCB override pointing at an endpoint marked unavailable (e.g. transport-dead) \
             must be skipped so the next attempt does not repeat the same connect failure"
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_skipped_when_current_endpoint_failed_in_flight() {
        // PPCB skips override targets already failed by this operation.
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
            preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
            account_write_endpoints: vec![north.clone(), central.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: north.clone(),
        });

        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("centralus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, north,
            "PPCB override pointing at a region in the in-flight skip set must be skipped"
        );
    }

    // ── PPAF override does NOT honor excluded / unavailable ────────────

    #[test]
    fn resolve_endpoint_ppaf_override_honored_even_when_in_excluded_regions() {
        // PPAF targets the single current write region, even if the caller excluded it.
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
            account_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
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
        retry_state.excluded_regions = vec![crate::options::Region::from("centralus")];

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, central,
            "PPAF override target must be honored even when in excluded_regions: \
             for a single-master account there is no other write region, so the SDK \
             routes to the PPAF target and surfaces the failure to the caller"
        );
    }

    #[test]
    fn resolve_endpoint_ppaf_override_honored_even_when_endpoint_marked_unavailable() {
        // PPAF must still try the only write region even when it is marked unavailable.
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

        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            central.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::TransportError,
            ),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
            preferred_write_endpoints: vec![north.clone()].into(),
            account_write_endpoints: vec![north.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: north.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
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

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, central,
            "PPAF override target must be honored even when marked unavailable: \
             single-master account has no other write region to fail over to"
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_probe_skipped_when_first_failed_endpoint_failed_in_flight() {
        // ProbeCandidate retries must skip the probe target after it fails in-flight.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );
        let east = CosmosEndpoint::regional(
            "eastus2".into(),
            Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
            preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
            account_write_endpoints: vec![central.clone(), east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: central.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
        partitions.per_partition_circuit_breaker_enabled = true;
        partitions.circuit_breaker_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                // Probe routes to `first_failed_endpoint` regardless of
                // `current_endpoint`; both point at centralus here.
                current_endpoint: east.clone(),
                first_failed_endpoint: central.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::ProbeCandidate,
                failback_jitter: Duration::ZERO,
            },
        );
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true, // multi-write
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);
        retry_state
            .pending_write_effects
            .push(make_pending_partition_mark_for_region("centralus"));

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_ne!(
            routing.endpoint, central,
            "PPCB ProbeCandidate must skip the probe target when its region is already in \
             the in-flight skip set; otherwise retry pins to the failing probe region"
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_honored_when_current_endpoint_not_failed() {
        // Fresh attempts must honor PPCB overrides until the target fails in-flight.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );
        let east = CosmosEndpoint::regional(
            "eastus2".into(),
            Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
            preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
            account_write_endpoints: vec![central.clone(), east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: central.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
        partitions.per_partition_circuit_breaker_enabled = true;
        let write_threshold = partitions.config.write_failure_threshold();
        partitions.circuit_breaker_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: east.clone(),
                first_failed_endpoint: central.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: write_threshold as i32 + 10,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);
        // pending_write_effects empty — first attempt of the operation.

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, east,
            "PPCB override with a healthy current_endpoint must be honored on a fresh attempt"
        );
    }

    #[test]
    fn resolve_endpoint_ppcb_override_pinned_to_unavailable_endpoint_reproduces_prod_403_3_loop() {
        // Production-shape repro: PPCB override threshold met while central is unavailable.
        // Multi-write leaves `pending_write_effects` empty, so availability is the only guard.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );
        let east = CosmosEndpoint::regional(
            "eastus2".into(),
            Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
        );

        // Multi-write 403/3 applied the endpoint mark before this retry.
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            central.url().clone(),
            (
                std::time::Instant::now(),
                crate::driver::routing::UnavailableReason::WriteForbidden,
            ),
        );

        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
            preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
            account_write_endpoints: vec![central.clone(), east.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: central.clone(),
        });

        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        };
        use crate::options::PartitionFailoverOptions;
        let pk_range_id: super::PartitionKeyRangeId = "0".parse().unwrap();
        let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
        partitions.per_partition_circuit_breaker_enabled = true;
        // PPCB override still points at the now-unavailable endpoint.
        partitions.circuit_breaker_overrides.insert(
            pk_range_id.clone(),
            PartitionFailoverEntry {
                current_endpoint: central.clone(),
                first_failed_endpoint: central.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                // Above the default threshold so PPCB failover triggers.
                write_failure_count: 10,
                first_failure_time: std::time::Instant::now(),
                last_failure_time: std::time::Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

        // Multi-write applies attempt effects immediately, leaving no in-flight skip set.
        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        retry_state.partition_key_range_id = Some(pk_range_id);

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_ne!(
            routing.endpoint, central,
            "BUG: PPCB override pinned the partition to centralus even though \
             centralus is in account.unavailable_endpoints. This is the \
             production 4-attempt-all-to-central failure mode. The PPCB \
             override-skip path in resolve_endpoint must honor \
             account.unavailable_endpoints (and excluded_regions) the same \
             way try_select_endpoint does."
        );
        assert_eq!(
            routing.endpoint, east,
            "with central in unavailable_endpoints and east the only other write \
             region, resolve_endpoint must route the next attempt to east"
        );
    }

    #[test]
    fn resolve_endpoint_rotates_to_next_region_via_location_index_on_multi_write() {
        // Pins LocationIndex rotation when multi-write PPCB-managed statuses
        // suppress endpoint marks and leave the deferred bucket empty.
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        let central = CosmosEndpoint::regional(
            "centralus".into(),
            Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        );
        let east = CosmosEndpoint::regional(
            "eastus2".into(),
            Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
        );

        // The bumped LocationIndex is the only signal that central failed.
        let account = Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
            preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
            account_write_endpoints: vec![central.clone(), east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: central.clone(),
        });
        let location = LocationSnapshot::for_tests(account);

        let mut retry_state = crate::driver::pipeline::components::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        );
        // Mirror production advancement while preserving the empty multi-write bucket.
        retry_state.location = retry_state.location.next_for_generation(2, 0);
        assert_eq!(
            retry_state.location.index(),
            1,
            "precondition: LocationIndex must have advanced from 0 to 1"
        );
        assert!(
            retry_state.pending_write_effects.is_empty(),
            "precondition: multi-write deferred bucket must be empty"
        );

        let routing = super::resolve_endpoint(
            &operation,
            &retry_state,
            &location,
            false,
            false,
            Duration::from_secs(60),
        );
        assert_eq!(
            routing.endpoint, east,
            "BUG: with LocationIndex=1 and empty in_flight_failed, \
             resolve_endpoint must route to east. The multi-write \
             PPCB-managed path has no other skip signal — if LocationIndex \
             rotation regresses, the next attempt will pin to the just-failed \
             centralus and reproduce the same-region-retry loop."
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
            account_write_endpoints: vec![east.clone()].into(),
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
            account_write_endpoints: vec![east.clone()].into(),
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
            account_write_endpoints: vec![east.clone()].into(),
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
                account_write_endpoints: vec![east.clone()].into(),
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
                account_write_endpoints: vec![east.clone()].into(),
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
            account_write_endpoints: vec![east.clone()].into(),
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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .expect("request should build");

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

        let throughput_control = Some(ResolvedThroughputControl {
            throughput_bucket: None,
            priority_level: Some(PriorityLevel::Low),
        });

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .unwrap();

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

        let throughput_control = Some(ResolvedThroughputControl {
            throughput_bucket: Some(42),
            priority_level: None,
        });

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .unwrap();

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

        let throughput_control = Some(ResolvedThroughputControl {
            throughput_bucket: Some(100),
            priority_level: Some(PriorityLevel::High),
        });

        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control,
        };
        let request =
            build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
                .unwrap();

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
        let container = test_container();
        let pk_def = container.partition_key_definition().clone();

        // Single-partition item query (scoped to a logical partition via FeedRange)
        let feed_range = FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
        let op = CosmosOperation::query_items(container.clone(), Some(feed_range))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
        assert_query_headers_present(&op, "query_items (single partition)");

        // Cross-partition item query (full container via FeedRange::full)
        let op = CosmosOperation::query_items(container, Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
        assert_query_headers_present(&op, "query_items (cross partition)");

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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        let req = build_transport_request(op, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");
        assert_eq!(
            req.headers
                .get_optional_str(&HeaderName::from_static(request_header_names::IS_QUERY)),
            Some("True"),
            "{label}: x-ms-documentdb-isquery should be 'True'"
        );
        assert_eq!(
            req.headers
                .get_optional_str(&azure_core::http::headers::CONTENT_TYPE),
            Some(crate::models::cosmos_headers::QUERY_CONTENT_TYPE),
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
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            resolved_session_token: None,
            throughput_control: None,
        };
        build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build")
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

    /// Multi-master invariant: the hedge-fallback setter must NOT latch
    /// `hub_region_processing_only` on a multi-master account (mirrors
    /// `build_session_retry_state`'s topology gate), so the header never
    /// leaks and the hub-region paths stay single-master-only. The
    /// session-retry counter is still advanced — that cap applies to all
    /// account types.
    #[test]
    fn propagate_hedge_session_unavailable_does_not_latch_on_multi_master() {
        let mut state = super::OperationRetryState::initial(0, true, Vec::new(), 3, 3);
        state.is_dataplane = true;

        super::propagate_hedge_session_unavailable(&mut state, true);

        assert!(
            !state.hub_region_processing_only,
            "multi-master must not latch hub_region_processing_only",
        );
        assert_eq!(
            state.session_token_retry_count, 1,
            "session-retry counter must still advance on multi-master",
        );
    }

    /// Single-master: the hedge-fallback setter latches
    /// `hub_region_processing_only` (and advances the session-retry counter),
    /// matching the non-hedged `build_session_retry_state` path.
    #[test]
    fn propagate_hedge_session_unavailable_latches_on_single_master() {
        let mut state = super::OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        super::propagate_hedge_session_unavailable(&mut state, true);

        assert!(
            state.hub_region_processing_only,
            "single-master must latch hub_region_processing_only",
        );
        assert_eq!(state.session_token_retry_count, 1);
    }

    // ── apply_tentative_writes_header ────────────────────────────────
    //
    // Required on multi-write accounts. Without
    // `x-ms-cosmos-allow-tentative-writes: true`, satellite write
    // regions reject writes with `403 / 3 (WriteForbidden)`.

    fn build_write_transport_request() -> super::TransportRequest {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());
        let routing = test_routing();
        let activity_id = ActivityId::from_string("tentative-writes-test".to_string());
        let ctx = TransportRequestContext {
            routing: &routing,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            resolved_session_token: None,
            throughput_control: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        };
        build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build")
    }

    fn tentative_writes_header(request: &super::TransportRequest) -> Option<&str> {
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::ALLOW_TENTATIVE_WRITES,
        ))
    }

    /// Multi-write account + write op → header MUST be emitted as "true".
    /// Without it, satellite write regions return
    /// `403 / 3 (WriteForbidden)`.
    #[test]
    fn tentative_writes_header_set_for_write_on_multi_write_account() {
        let mut request = build_write_transport_request();
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        super::apply_tentative_writes_header(&mut request, &operation, true);

        assert_eq!(tentative_writes_header(&request), Some("true"));
    }

    /// Reads on a multi-write account never need the tentative-writes
    /// header — they're idempotent and route per the read policy.
    #[test]
    fn tentative_writes_header_omitted_for_read_on_multi_write_account() {
        let mut request = build_minimal_transport_request();
        let operation = CosmosOperation::read_all_databases(test_account());

        super::apply_tentative_writes_header(&mut request, &operation, true);

        assert!(tentative_writes_header(&request).is_none());
    }

    /// Single-master accounts must not see the header even on writes; the
    /// hub region rejects unknown headers in some build configurations
    /// and this header is only meaningful for multi-write topologies.
    #[test]
    fn tentative_writes_header_omitted_for_write_on_single_master_account() {
        let mut request = build_write_transport_request();
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

        super::apply_tentative_writes_header(&mut request, &operation, false);

        assert!(tentative_writes_header(&request).is_none());
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
        let diagnostics = test_diagnostics();
        let result = super::enforce_deadline_or_timeout(None, &options, diagnostics);
        assert!(result.is_ok());
    }

    #[test]
    fn enforce_deadline_in_future_is_ok() {
        let options = empty_options_view();
        let diagnostics = test_diagnostics();
        let deadline = std::time::Instant::now() + Duration::from_secs(60);
        let result = super::enforce_deadline_or_timeout(Some(deadline), &options, diagnostics);
        assert!(result.is_ok());
    }

    #[test]
    fn enforce_deadline_in_past_returns_timeout_error_with_diagnostics() {
        let options = empty_options_view();
        let diagnostics = test_diagnostics();
        let deadline = std::time::Instant::now() - Duration::from_millis(1);
        let result = super::enforce_deadline_or_timeout(Some(deadline), &options, diagnostics);
        let err = result.expect_err("past deadline should produce an error");
        let msg = err.to_string();
        assert!(
            msg.contains("end-to-end operation timeout exceeded"),
            "unexpected error message: {msg}"
        );
        // Diagnostics must be attached so callers reading
        // `error.diagnostics()` on a timeout outcome get the
        // pipeline's tracked retry history rather than `None`.
        assert!(
            err.diagnostics().is_some(),
            "timeout error must carry finalized diagnostics"
        );
    }

    // ── classify_hedge_result (Part 4b) ────────────────────────────────

    fn http_result(status_code: u16, sub_status: Option<u32>) -> super::TransportResult {
        use azure_core::http::StatusCode;
        let mut status = crate::models::CosmosStatus::new(StatusCode::from(status_code));
        if let Some(v) = sub_status {
            status = status.with_sub_status(v as u16);
        }
        super::TransportResult::from_http_response(
            status,
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
        // 409 is a final HTTP error — terminates hedging.
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
        // 404/1002 ReadSessionNotAvailable is retriable.
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
        let err = crate::error::CosmosError::builder()
            .with_message("synthetic build error")
            .build();
        assert!(matches!(
            super::classify_hedge_result(Err(err)),
            super::HedgeClass::Transient
        ));
    }

    // ── result_is_final (Part 4b — pre-threshold PrimaryWin gating) ────
    //
    // Regression tests for the pre-threshold primary-completion branch
    // in `execute_hedged`. Before the fix the branch recorded
    // `PrimaryWin` unconditionally — including for `Transient` primary
    // outcomes — which would perpetually reset the
    // consecutive-hedge-win counter under sticky-fast-transient
    // traffic, neutralizing the very PPCB signal the counter was
    // designed to detect. `result_is_final` is the non-consuming
    // classifier used to gate the recording; it must agree with
    // `classify_hedge_result` for every input shape.

    #[test]
    fn result_is_final_success_is_true() {
        let tr = http_result(200, None);
        assert!(super::result_is_final(&tr));
    }

    #[test]
    fn result_is_final_409_conflict_is_true() {
        let tr = http_result(409, None);
        assert!(super::result_is_final(&tr));
    }

    #[test]
    fn result_is_final_503_is_false() {
        let tr = http_result(503, None);
        assert!(!super::result_is_final(&tr));
    }

    #[test]
    fn result_is_final_404_1002_is_false() {
        // 404/1002 ReadSessionNotAvailable is transient — the
        // pre-threshold branch must NOT record PrimaryWin (per the
        // c328cf50 over-correction).
        let tr = http_result(404, Some(1002));
        assert!(!super::result_is_final(&tr));
    }

    #[test]
    fn result_is_final_deadline_exceeded_is_false() {
        let tr =
            super::TransportResult::deadline_exceeded(crate::diagnostics::RequestSentStatus::Sent);
        assert!(!super::result_is_final(&tr));
    }

    #[test]
    fn result_is_final_429_ru_budget_and_hot_partition_are_final() {
        // RU-budget / hot-partition throttles cannot be relieved by racing a
        // second region, so they short-circuit the race.
        assert!(super::result_is_final(&http_result(429, Some(3200)))); // RU_BUDGET_EXCEEDED
        assert!(super::result_is_final(&http_result(429, Some(3210)))); // RU_BUDGET_EXCEEDED_FOR_MASTER
        assert!(super::result_is_final(&http_result(429, Some(3214)))); // HOT_PARTITION_KEY_THROTTLED
    }

    #[test]
    fn result_is_final_429_generic_and_3092_are_transient() {
        // A generic 429 and the transient-capacity 3092 sub-status keep the
        // other leg racing — another region may have spare capacity.
        assert!(!super::result_is_final(&http_result(429, None)));
        assert!(!super::result_is_final(&http_result(429, Some(3092))));
    }

    /// Cross-check: every `Ok(tr)` shape that `classify_hedge_result`
    /// maps to `HedgeClass::Final` must also be `true` under
    /// `result_is_final`, and vice versa. Keeping the two in lockstep
    /// is the load-bearing invariant for the pre-threshold PrimaryWin
    /// gating fix — drift between them re-opens the c328cf50 bug.
    #[test]
    fn result_is_final_agrees_with_classify_hedge_result() {
        let cases = [
            http_result(200, None),
            http_result(201, None),
            http_result(204, None),
            http_result(400, None),
            http_result(404, None),
            http_result(404, Some(1002)),
            http_result(409, None),
            http_result(412, None),
            http_result(429, None),
            http_result(429, Some(3092)),
            http_result(429, Some(3200)),
            http_result(429, Some(3210)),
            http_result(429, Some(3214)),
            http_result(500, None),
            http_result(503, None),
            super::TransportResult::deadline_exceeded(crate::diagnostics::RequestSentStatus::Sent),
        ];
        for tr in cases {
            let by_peek = super::result_is_final(&tr);
            let by_classify = matches!(
                super::classify_hedge_result(Ok(tr)),
                super::HedgeClass::Final(_)
            );
            assert_eq!(
                by_peek, by_classify,
                "result_is_final must agree with classify_hedge_result",
            );
        }
    }

    // ── finalize_hedge_attempt (Part 4b) ──────────────────────────────

    #[test]
    fn finalize_hedge_attempt_http_error_returns_error_with_status() {
        let tr = Box::new(http_result(409, None));
        let diagnostics = test_diagnostics();
        let err = super::finalize_hedge_attempt(tr, diagnostics)
            .expect_err("409 should be surfaced as an error");
        assert_eq!(u16::from(err.status().status_code()), 409);
    }

    #[test]
    fn finalize_hedge_attempt_deadline_returns_other_error() {
        let tr = Box::new(super::TransportResult::deadline_exceeded(
            crate::diagnostics::RequestSentStatus::Sent,
        ));
        let diagnostics = test_diagnostics();
        let err = super::finalize_hedge_attempt(tr, diagnostics)
            .expect_err("deadline should produce an error");
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
    fn application_cancelled_error_carries_app_cancel_message() {
        let err = super::application_cancelled_error(test_diagnostics());
        let msg = err.to_string();
        assert!(
            msg.contains("cancelled by application deadline"),
            "unexpected error message: {msg}"
        );
        // Typed-status invariant: telemetry/retry-evaluation can
        // discriminate a client-side hedge cancel from a service 408.
        let status = err.status();
        assert_eq!(
            status.status_code(),
            azure_core::http::StatusCode::RequestTimeout
        );
        assert_eq!(
            status.sub_status(),
            Some(crate::models::SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        // Diagnostics-on-error invariant: the synthesized error must
        // carry the operation's diagnostics chain (cf. P0 #1).
        assert!(
            err.diagnostics().is_some(),
            "application_cancelled_error must graft diagnostics"
        );
    }

    #[test]
    fn harvest_window_is_50ms() {
        // Load-bearing spec constant — surfaced as a test so the
        // HARVEST_WINDOW contract is enforced.
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
                Err::<super::TransportResult, _>(
                    crate::error::CosmosError::builder()
                        .with_message("synthetic transport error")
                        .build(),
                ),
                child,
            )
        });

        super::harvest_remaining_attempt(attempt, &mut parent, super::HARVEST_WINDOW).await;
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
                Err::<super::TransportResult, _>(
                    crate::error::CosmosError::builder()
                        .with_message("should not reach here")
                        .build(),
                ),
                child,
            )
        });

        // Drive the test clock past HARVEST_WINDOW.
        let parent_before = parent.request_count();
        super::harvest_remaining_attempt(attempt, &mut parent, super::HARVEST_WINDOW).await;
        assert_eq!(parent.request_count(), parent_before);
    }

    // ── Shared hub-region latch (Part 5) ──────────────────────────

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
    /// AC-8 and the data-plane scope gate of
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
    /// emits, even when per-state latch is `false`. This is the
    /// cross-hedge propagation rule.
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
    /// integration tests.
    #[test]
    fn should_emit_hub_region_header_neither_latched() {
        use std::sync::{atomic::AtomicBool, Arc};
        let shared = Arc::new(AtomicBool::new(false));
        assert!(!super::should_emit_hub_region_header(false, None));
        assert!(!super::should_emit_hub_region_header(false, Some(&shared)));
    }

    /// T-S11 — `apply_hub_region_header` emits the header when only the
    /// shared latch is `true` on the retry state, verifying cross-hedge
    /// propagation at the emission layer.
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

    // ── try_advance_after_both_transient (region-aware advancement) ───
    //
    // After the `evaluate_hedge_eligibility` secondary picker change
    // (commit 5dea1c9) the raced secondary is no longer guaranteed
    // to sit at `primary_index + 1`. These tests pin the
    // `try_advance_after_both_transient` contract: the post-race
    // `LocationIndex` must NOT land on either of the two regions the
    // race already consumed, regardless of where the picker placed
    // the secondary relative to the primary.
    fn make_endpoint(region: &'static str) -> CosmosEndpoint {
        CosmosEndpoint::regional(
            region.into(),
            Url::parse(&format!("https://acc-{region}.documents.azure.com:443/"))
                .expect("test endpoint url should parse"),
        )
    }

    fn make_advance_test_state(
        location_index: usize,
        endpoint_count: usize,
    ) -> crate::driver::pipeline::components::OperationRetryState {
        let mut location = LocationIndex::initial(0);
        for _ in 0..location_index {
            location = location.next(endpoint_count);
        }
        crate::driver::pipeline::components::OperationRetryState {
            location,
            failover_retry_count: 0,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 10,
            max_backend_failover_retries: 120,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            is_dataplane: true,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
            retry_with_state: None,
        }
    }

    fn make_advance_test_location(regions: &[&'static str]) -> super::LocationSnapshot {
        let endpoints: Vec<CosmosEndpoint> = regions.iter().copied().map(make_endpoint).collect();
        let default = endpoints[0].clone();
        super::LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: endpoints.clone().into(),
            preferred_write_endpoints: endpoints.clone().into(),
            account_write_endpoints: endpoints.into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default,
        }))
    }

    fn dummy_last_error() -> crate::error::CosmosError {
        crate::error::CosmosError::builder()
            .with_message("test-both-transient")
            .build()
    }

    /// Regression: STAGE 7 picker now selects the secondary as the
    /// first endpoint with a region/key different from the primary,
    /// which can sit at an index earlier than the primary. The
    /// post-race advance must skip BOTH raced regions instead of
    /// blindly stepping `LocationIndex` by `+2`.
    ///
    /// Endpoints: [A, B, C, D]. Primary promoted to index 2 (C);
    /// picker walks from the front and lands on A (index 0) as the
    /// secondary. After `BothTransient` the next attempt must target
    /// D (the only untried region), not A again.
    #[test]
    fn try_advance_after_both_transient_skips_raced_regions_when_secondary_is_before_primary() {
        let regions = ["region-a", "region-b", "region-c", "region-d"];
        let location = make_advance_test_location(&regions);
        let mut state = make_advance_test_state(2, regions.len()); // primary = region-c

        let primary = crate::options::Region::new("region-c");
        let secondary = crate::options::Region::new("region-a");

        super::try_advance_after_both_transient(
            &mut state,
            &location,
            true,
            Some(&primary),
            Some(&secondary),
            dummy_last_error(),
        )
        .expect("budget should allow advance");

        let landed = location.account.preferred_read_endpoints[state.location.index()].region();
        assert_eq!(
            landed.map(crate::options::Region::as_str),
            Some("region-d"),
            "post-BothTransient LocationIndex must skip the raced primary and \
             the raced secondary (no matter where it sat) and land on the \
             only untried region",
        );
        assert_eq!(state.failover_retry_count, 2);
    }

    /// Sanity: when the secondary sits immediately after the primary
    /// (the historical layout the original `+2` advance assumed), the
    /// new implementation still skips both raced regions and lands on
    /// the next untried region.
    #[test]
    fn try_advance_after_both_transient_skips_raced_regions_when_secondary_is_after_primary() {
        let regions = ["region-a", "region-b", "region-c", "region-d"];
        let location = make_advance_test_location(&regions);
        let mut state = make_advance_test_state(0, regions.len()); // primary = region-a

        let primary = crate::options::Region::new("region-a");
        let secondary = crate::options::Region::new("region-b");

        super::try_advance_after_both_transient(
            &mut state,
            &location,
            true,
            Some(&primary),
            Some(&secondary),
            dummy_last_error(),
        )
        .expect("budget should allow advance");

        let landed = location.account.preferred_read_endpoints[state.location.index()].region();
        assert_eq!(
            landed.map(crate::options::Region::as_str),
            Some("region-c"),
            "secondary-after-primary case must still skip both raced regions",
        );
    }

    /// Edge case: only two regions exist and both were raced. The
    /// function must terminate (no infinite loop) and charge the two
    /// slots against the failover budget. With nowhere untried left
    /// to go, the `LocationIndex` is allowed to wrap; the next
    /// iteration's `resolve_endpoint` relies on the unavailable-
    /// endpoint de-prioritization to make a sensible choice.
    #[test]
    fn try_advance_after_both_transient_terminates_when_all_regions_are_raced() {
        let regions = ["region-a", "region-b"];
        let location = make_advance_test_location(&regions);
        let mut state = make_advance_test_state(0, regions.len());

        let primary = crate::options::Region::new("region-a");
        let secondary = crate::options::Region::new("region-b");

        let result = super::try_advance_after_both_transient(
            &mut state,
            &location,
            true,
            Some(&primary),
            Some(&secondary),
            dummy_last_error(),
        );

        assert!(
            result.is_ok(),
            "should not surface a terminal error when budget remains"
        );
        assert_eq!(
            state.failover_retry_count, 2,
            "two slots are always charged regardless of layout",
        );
    }

    /// Budget-exhaustion path remains untouched: when
    /// `failover_retry_count + 2 > max_failover_retries`, the
    /// function returns the supplied error without mutating
    /// `LocationIndex`.
    #[test]
    fn try_advance_after_both_transient_surfaces_terminal_error_when_budget_exhausted() {
        let regions = ["region-a", "region-b", "region-c"];
        let location = make_advance_test_location(&regions);
        let mut state = make_advance_test_state(0, regions.len());
        state.max_failover_retries = 1;
        let starting_index = state.location.index();

        let primary = crate::options::Region::new("region-a");
        let secondary = crate::options::Region::new("region-b");

        let result = super::try_advance_after_both_transient(
            &mut state,
            &location,
            true,
            Some(&primary),
            Some(&secondary),
            dummy_last_error(),
        );

        assert!(result.is_err(), "exhausted budget must surface terminal");
        assert_eq!(
            state.location.index(),
            starting_index,
            "exhausted budget must not mutate LocationIndex",
        );
        assert_eq!(state.failover_retry_count, 0);
    }

    /// When a hedge race ends in `BothTransient` and the failover budget
    /// is exhausted, the race is the terminal outcome, so the non-terminal
    /// path must still carry `strategy_config` + the diagnostic regions on
    /// the variant for the budget-exhausted handler to stamp them.
    #[test]
    fn both_transient_budget_exhausted_carries_hedge_diagnostics() {
        let threshold =
            crate::options::HedgeThreshold::new(std::time::Duration::from_millis(200)).unwrap();
        let strategy_config = super::HedgingStrategyConfig::new(threshold);
        let primary_for_diag = crate::options::Region::new("region-a");
        let secondary_for_diag = crate::options::Region::new("region-b");

        // Non-terminal both-transient (deadline = None) must return the
        // `BothTransient` variant carrying the data the budget-exhausted
        // handler needs to stamp diagnostics.
        let race = super::finalize_both_transient(
            &crate::models::ActivityId::from_string("both-transient-budget".to_owned()),
            None, // deadline not elapsed
            strategy_config,
            Some(crate::options::Region::new("region-a")),
            Some(crate::options::Region::new("region-b")),
            primary_for_diag.clone(),
            secondary_for_diag.clone(),
            test_diagnostics(),
            None,
            false,
        );

        let super::HedgedRaceResult::BothTransient {
            strategy_config: carried_strategy,
            primary_region_for_diag,
            secondary_region_for_diag,
            mut diagnostics,
            ..
        } = race
        else {
            panic!("deadline-not-elapsed both-transient must return BothTransient");
        };

        // The variant must carry exactly what the handler stamps —
        // otherwise the hedge diagnostics would be silently dropped on
        // the budget-exhausted terminal error.
        assert_eq!(carried_strategy, strategy_config);
        assert_eq!(primary_region_for_diag, primary_for_diag);
        assert_eq!(secondary_region_for_diag, secondary_for_diag);

        // Reproduce the budget-exhausted handler step: stamp the
        // both-transient hedge result with `deadline_elapsed = false`
        // (the budget, not the deadline, ended the race).
        diagnostics.set_hedge_diagnostics(super::HedgeDiagnostics::both_transient(
            carried_strategy,
            primary_region_for_diag,
            secondary_region_for_diag,
            false,
        ));
        let ctx = diagnostics.complete();

        let hedge = ctx
            .hedge_diagnostics()
            .expect("budget-exhausted both-transient must carry hedge diagnostics");
        assert_eq!(
            hedge.terminal_state(),
            crate::diagnostics::HedgeTerminalState::BothTransient {
                deadline_elapsed: false,
            },
        );
        assert_eq!(hedge.primary_region(), &primary_for_diag);
        assert_eq!(hedge.alternate_region(), Some(&secondary_for_diag));
        // No leg produced a final response in a both-transient terminal.
        assert_eq!(hedge.response_region(), None);
    }
}
