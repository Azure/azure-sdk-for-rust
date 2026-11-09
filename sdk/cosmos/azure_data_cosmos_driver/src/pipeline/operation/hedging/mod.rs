// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Coordinates primary and alternate attempts without spawning tasks.

use std::{
    ops::ControlFlow,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use futures::{
    future,
    future::{Either, Future},
};

use crate::{
    diagnostics::{
        hedging::{HedgeDiagnostics, HedgingStrategyConfig},
        DiagnosticsContextBuilder, ExecutionContext, PipelineKind,
    },
    models::{ActivityId, CosmosOperation, CosmosResponse},
    options::{HedgeThreshold, OperationOptionsView, Region},
    pipeline::{
        operation::{
            self, deadline, hedging,
            hedging::budget::{HedgeBudget, HedgePermit},
            recovery,
            recovery::state::{OperationAction, OperationRetryState},
            request, response, routing,
            routing::{
                partition_key_range_id::PartitionKeyRangeId, AccountEndpointState, HedgeOutcome,
                LocationSnapshot, RoutingDecision,
            },
            session,
            types::OperationOverrides,
            AttemptContext, AttemptOutcome, OperationContext,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Admits a retry-triggered race only while the invocation's one-race allowance remains.
///
/// The returned permit must survive location feedback and retry-state advancement.
pub(crate) fn upgrade_retry<'a>(
    action: OperationAction,
    operation: &CosmosOperation,
    context: &OperationContext<'a>,
    captured: &LocationSnapshot,
    route: &RoutingDecision,
    state: &OperationRetryState,
) -> (OperationAction, Option<HedgePermit<'a>>) {
    if state.hedge_already_fired
        || self::hedging_suppressed_for_attempt(operation, context.overrides)
    {
        return (action, None);
    }
    self::maybe_upgrade_to_hedge(
        action,
        operation,
        context.config.options,
        &captured.account,
        captured.cross_region_hedging_disabled,
        route,
        context.policy.configured_request_timeout,
        context.services.hedge_budget,
        context.config.pipeline_type,
        context.config.activity_id,
    )
}

/// Rejects hedging when a continuation or internal correctness constraint requires one region.
pub(crate) fn hedging_suppressed_for_attempt(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
) -> bool {
    overrides.hedging_suppressed() || operation.suppresses_hedging()
}

/// Admits an initial race without queuing; the returned guard bounds its entire lifetime.
///
/// Later retries use the separate retry-upgrade gate because they have already
/// consumed region budget.
pub(crate) fn should_hedge<'a>(
    ctx: &AttemptContext<'a>,
    state: &OperationRetryState,
    captured: &LocationSnapshot,
    routing: &RoutingDecision,
) -> Option<(eligibility::HedgeUpgrade, HedgePermit<'a>)> {
    if state.failover_retry_count != 0
        || state.session_token_retry_count != 0
        || self::hedging_suppressed_for_attempt(ctx.operation, ctx.overrides)
    {
        return None;
    }

    let upgrade = eligibility::evaluate_hedge_eligibility(
        ctx.operation,
        ctx.config.options,
        &captured.account,
        routing,
        ctx.policy.configured_request_timeout,
        captured.cross_region_hedging_disabled,
    )?;
    let Some(permit) = ctx
        .services
        .hedge_budget
        .try_admit(ctx.config.pipeline_type)
    else {
        tracing::debug!(activity_id = %ctx.config.activity_id, "cosmos.hedge.concurrency_budget_exhausted");
        return None;
    };
    Some((upgrade, permit))
}

/// Runs an admitted race and returns a terminal result or sequential fallback progress.
///
/// The admission permit remains held through feedback and diagnostics finalization.
/// Both-transient recovery advances only the coordinator's retry state.
pub(crate) async fn hedge_operation(
    ctx: &AttemptContext<'_>,
    captured: &LocationSnapshot,
    primary: &RoutingDecision,
    plan: (eligibility::HedgeUpgrade, HedgePermit<'_>),
    retry_state: &mut OperationRetryState,
    diagnostics: DiagnosticsContextBuilder,
) -> ControlFlow<crate::error::Result<CosmosResponse>, DiagnosticsContextBuilder> {
    let (plan, _permit) = plan;
    retry_state.hedge_already_fired = true;
    let outcome = self::execute(
        ctx,
        primary,
        &plan.secondary_routing,
        plan.threshold,
        plan.strategy_config,
        diagnostics,
        retry_state,
    )
    .await;

    recovery::hedge::after_race(outcome, retry_state, captured, ctx.operation)
}

/// Resolves both regions again after a retry upgrade has advanced the primary.
///
/// An alternate selected before advancement may now be the primary. Returning
/// no pair releases the caller's admission guard without consuming its one-race allowance.
pub(crate) fn retarget(
    operation: &CosmosOperation,
    state: &OperationRetryState,
    location: &routing::LocationSnapshot,
    config: &crate::pipeline::operation::config::OperationPipelineConfig<'_>,
    services: &crate::pipeline::operation::config::OperationPipelineServices<'_>,
    policy: &crate::pipeline::operation::policy::Policy,
) -> Option<(RoutingDecision, RoutingDecision)> {
    let primary = routing::resolve(
        &services.clock,
        operation,
        state,
        location,
        policy.endpoint_unavailability_ttl,
    );
    let upgrade = eligibility::evaluate_hedge_eligibility(
        operation,
        config.options,
        &location.account,
        &primary,
        policy.configured_request_timeout,
        location.cross_region_hedging_disabled,
    )?;
    Some((primary, upgrade.secondary_routing))
}

/// Result classification used by [`execute`] to decide whether a
/// completed hedge attempt terminates the race or keeps the other side
/// running.
///
/// `Final` carries the [`TransportResult`] that becomes the operation's
/// outcome (success or final-classified HTTP error). `Transient`
/// indicates the attempt should be ignored in favor of the other side
/// (5xx / 429 / 408 / 410 / 404-1002 / 403-with-sub / transport error /
/// deadline) — when both sides land retriable, the most recent error
/// is surfaced as the operation error.
pub(crate) enum HedgeClass {
    Final(Box<TransportResult>),
    Transient,
}

/// Pure classifier for a per-attempt result.
///
/// `Err` (e.g. failure constructing the request) and any non-final
/// TransportOutcome map to `Transient`. A `Success` is always final; an
/// `HttpError` is final iff [`CosmosStatus::is_final_result`](crate::models::CosmosStatus::is_final_result) returns `true` for its
/// status.
pub(crate) fn classify_hedge_result(result: crate::error::Result<TransportResult>) -> HedgeClass {
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

/// Classifies the SECONDARY (hedge) leg, applying the metadata
/// primary-authoritative rule.
///
/// For the two metadata cache reads the primary region is authoritative: a
/// hedge may improve latency by winning with a definitive **success**, but it
/// must never override the primary with a definitive **error** (e.g. a
/// not-yet-replicated secondary returning `404` for a freshly-created container,
/// or a `409`/`412`/`429`-final). When `primary_authoritative` is set, a
/// secondary that produced a `Final` but non-`Success` outcome is therefore
/// downgraded to [`HedgeClass::Transient`] so the race discards it and awaits
/// the primary's authoritative outcome. A secondary definitive success still
/// wins; a secondary transient stays transient. Data-plane hedging passes
/// `false` and keeps the first-`Final`-wins semantics of
/// [`classify_hedge_result`].
///
/// `hub_only_read` must be set only for a read whose leg sent the
/// hub-region-processing-only header: its `403/3` means "not the hub here".
pub(crate) fn classify_secondary_hedge_result(
    result: crate::error::Result<TransportResult>,
    primary_authoritative: bool,
    hub_only_read: bool,
) -> HedgeClass {
    match hedging::classify_hedge_result(result) {
        HedgeClass::Final(tr)
            if primary_authoritative && !matches!(tr.outcome, TransportOutcome::Success { .. }) =>
        {
            HedgeClass::Transient
        }
        HedgeClass::Final(tr)
            if hub_only_read
                && matches!(
                    &tr.outcome,
                    TransportOutcome::HttpError { status, .. } if status.is_write_forbidden()
                ) =>
        {
            HedgeClass::Transient
        }
        other => other,
    }
}

/// Non-consuming version of [`classify_hedge_result`] for the
/// pre-threshold primary-completion branch, where the caller still
/// needs the original `TransportResult` to surface the response via
/// [`finalize_hedge_attempt`](crate::pipeline::operation::response::finalize_hedge_attempt) but only wants to know whether the
/// primary's outcome was `Final` for [`record_hedge_outcome`](crate::pipeline::operation::routing::record_hedge_outcome) gating.
///
/// Returns `true` iff a `Success` or an `HttpError` whose status passes
/// [`CosmosStatus::is_final_result`](crate::models::CosmosStatus::is_final_result). Mirrors `classify_hedge_result` exactly — keep
/// the two in lockstep.
pub(crate) fn result_is_final(tr: &TransportResult) -> bool {
    match &tr.outcome {
        TransportOutcome::Success { .. } => true,
        TransportOutcome::HttpError { status, .. } => status.is_final_result(),
        TransportOutcome::TransportError { .. } | TransportOutcome::DeadlineExceeded { .. } => {
            false
        }
    }
}

/// Reads a response's physical partition identity without consuming the leg outcome.
pub(crate) fn pk_range_id_from_result(
    result: &crate::error::Result<TransportResult>,
) -> Option<PartitionKeyRangeId> {
    result
        .as_ref()
        .ok()
        .and_then(|tr| tr.cosmos_headers())
        .and_then(|headers| headers.partition_key_range_id.as_deref())
        .map(|s| PartitionKeyRangeId::from(s.to_owned()))
}

/// If `action` is a same-pipeline retry and the operation is eligible
/// for cross-region hedging, rewrites it to `OperationAction::Hedge`.
/// Otherwise returns `action` unchanged.
///
/// Only `FailoverRetry` and `SessionRetry` are eligible for upgrade —
/// `Complete` is the operation's terminal success path, and `Abort`
/// already signals a non-retryable error. `request_timeout` is passed to
/// [`evaluate_hedge_eligibility`](crate::pipeline::operation::hedging::eligibility::evaluate_hedge_eligibility) so it can compute the driver default
/// threshold (`min(1000ms, request_timeout / 2)`).
///
/// Forwards the incoming variant's `new_state` into the rewritten
/// `Hedge` variant so STAGE 7 can apply it to the live `retry_state`
/// before building `AttemptContext` — see
/// `OperationAction::Hedge::new_state`.
///
/// Returns the (possibly rewritten) action alongside the [`HedgePermit`] that
/// admitted the race. The caller must hold the permit for as long as the race is
/// open; dropping it returns the slot to the [`HedgeBudget`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn maybe_upgrade_to_hedge<'a>(
    action: OperationAction,
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    account_state: &AccountEndpointState,
    cross_region_hedging_disabled: bool,
    primary: &RoutingDecision,
    request_timeout: Option<Duration>,
    hedge_budget: &'a HedgeBudget,
    pipeline_type: PipelineKind,
    activity_id: &ActivityId,
) -> (OperationAction, Option<HedgePermit<'a>>) {
    let new_state = match &action {
        OperationAction::FailoverRetry { delay: Some(d), .. } if !d.is_zero() => {
            return (action, None)
        }
        OperationAction::FailoverRetry { new_state, .. } => new_state.clone(),
        OperationAction::SessionRetry { new_state } => new_state.clone(),
        _ => return (action, None),
    };

    match eligibility::evaluate_hedge_eligibility(
        operation,
        options,
        account_state,
        primary,
        request_timeout,
        cross_region_hedging_disabled,
    ) {
        Some(upgrade) => {
            if new_state.failover_retry_count.saturating_add(2) > new_state.max_failover_retries {
                tracing::debug!(
                    failover_retry_count = new_state.failover_retry_count,
                    max_failover_retries = new_state.max_failover_retries,
                    "cosmos.hedge.budget_exhausted_skipping_upgrade",
                );
                return (action, None);
            }
            let Some(permit) = hedge_budget.try_admit(pipeline_type) else {
                tracing::debug!(
                    activity_id = %activity_id,
                    "cosmos.hedge.concurrency_budget_exhausted",
                );
                return (action, None);
            };
            tracing::debug!(
                threshold_ms = upgrade.threshold.get().as_millis() as u64,
                primary_region = ?primary.endpoint.region().map(crate::options::Region::as_str),
                secondary_region = ?upgrade.secondary_routing.endpoint.region().map(crate::options::Region::as_str),
                hub_region_processing_only = new_state.hub_region_processing_only,
                "cosmos.hedge.enabled_for_operation",
            );
            (
                OperationAction::Hedge {
                    threshold: upgrade.threshold,
                    strategy_config: upgrade.strategy_config,
                    new_state,
                },
                Some(permit),
            )
        }
        None => (action, None),
    }
}

/// Discriminator returned by the Stage-2 threshold-vs-deadline nested
/// race so the outer `select` against the primary can branch cleanly
/// without unwrapping a nested [`Either`].
pub(crate) enum TimerEvent {
    /// The hedge threshold elapsed first — Stage 3 should launch the
    /// secondary.
    ThresholdElapsed,
    /// The end-to-end deadline elapsed first — [`execute`] should
    /// harvest the still-pending primary within [`HARVEST_WINDOW`](crate::pipeline::operation::deadline::HARVEST_WINDOW) and
    /// re-raise an [`application_cancelled_error`](crate::pipeline::operation::deadline::application_cancelled_error).
    DeadlineFired,
}

/// Outcome of [`execute`] as observed by the surrounding
/// [`operation::execute()`] loop.
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
    /// 3. Otherwise advance the [`LocationIndex`](crate::pipeline::operation::routing::LocationIndex) past the two consumed
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

/// Coordinates primary, threshold, alternate, and deadline phases without spawning tasks.
///
/// A final primary before the threshold avoids alternate dispatch. Afterwards,
/// feedback is applied at each observed completion before awaiting its partner.
/// Metadata alternates may win only with success. Both transient legs return
/// observations to the outer retry loop; cancellation harvests diagnostics only.
///
/// Loser futures remain owned until response finalization, preserving journal and
/// drop ordering. The primary never reads the secondary's race-local hub signal.
pub(crate) async fn execute(
    ctx: &AttemptContext<'_>,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    threshold: HedgeThreshold,
    strategy_config: HedgingStrategyConfig,
    mut parent_diagnostics: DiagnosticsContextBuilder,
    retry_state_snapshot: &OperationRetryState,
) -> HedgedRaceResult {
    let primary_region = primary_routing.endpoint.region().cloned();
    let secondary_region = secondary_routing.endpoint.region().cloned();
    let primary_region_for_diag = primary_region
        .clone()
        .unwrap_or_else(|| Region::new(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL));
    let secondary_region_for_diag = secondary_region
        .clone()
        .unwrap_or_else(|| Region::new(HedgeDiagnostics::UNKNOWN_REGION_SENTINEL));

    let metadata_primary_authoritative = ctx.config.pipeline_type.is_metadata();

    tracing::debug!(
        activity_id = %ctx.config.activity_id,
        threshold_ms = ?threshold.get().as_millis(),
        primary_region = ?primary_region.as_ref().map(|r| r.as_str()),
        secondary_region = ?secondary_region.as_ref().map(|r| r.as_str()),
        "hedging::execute: launching primary attempt",
    );

    let primary_diag = parent_diagnostics.clone_for_hedge_attempt();
    let primary_dispatch =
        primary_diag.leg_dispatch(primary_region.clone(), ExecutionContext::Initial);
    // The primary deliberately runs without the secondary's hub signal, including on upgrade.
    let primary_attempt = Box::pin(async move {
        let mut diag = primary_diag;
        let result = operation::perform_single_attempt(
            ctx,
            primary_routing,
            request::hedge_options(ctx, primary_routing, ExecutionContext::Initial, None),
            &mut diag,
        )
        .await;
        (result.result, diag)
    });

    let _threshold_duration = match azure_core::time::Duration::try_from(threshold.get()) {
        Ok(d) => d,
        Err(_) => {
            parent_diagnostics.set_operation_status(
                azure_core::http::StatusCode::InternalServerError,
                Some(crate::error::status_codes::substatus::TRANSPORT_GENERATED_503),
            );
            let diagnostics_ctx = Arc::new(parent_diagnostics.complete());
            return HedgedRaceResult::Terminal(Err(crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::TRANSPORT_GENERATED_503)
                .with_message("hedge threshold exceeds azure_core::time::Duration range")
                .with_diagnostics(diagnostics_ctx)
                .build()));
        }
    };
    let threshold_timer = Box::pin(ctx.services.clock.sleep(threshold.get()));
    let deadline_timer = deadline::deadline_signal(&ctx.services.clock, ctx.policy.deadline);

    let timer_event = Box::pin(async move {
        futures::pin_mut!(deadline_timer);
        match future::select(threshold_timer, deadline_timer).await {
            Either::Left(((), _)) => TimerEvent::ThresholdElapsed,
            Either::Right(((), _)) => TimerEvent::DeadlineFired,
        }
    });

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
    let primary_attempt: PrimaryAttemptFuture<'_> = match future::select(
        primary_attempt,
        timer_event,
    )
    .await
    {
        Either::Left(((result, diag), _timer)) => {
            let primary_was_final = matches!(
                &result,
                Ok(tr) if hedging::result_is_final(tr),
            );
            if primary_was_final {
                parent_diagnostics.merge_hedge_attempt(diag);
                parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::primary_only(
                    strategy_config,
                    primary_region_for_diag.clone(),
                ));
                tracing::debug!(
                    activity_id = %ctx.config.activity_id,
                    "hedging::execute: primary won pre-threshold (zero-overhead happy path)",
                );
                tracing::debug!(
                    activity_id = %ctx.config.activity_id,
                    winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                    was_hedge = false,
                    "cosmos.hedge.winner_selected",
                );
                let pk_range_id_for_feedback = hedging::pk_range_id_from_result(&result)
                    .or_else(|| ctx.partition_key_range_id.clone());
                routing::record_hedge_outcome(
                    ctx.services.location_state_store,
                    HedgeOutcome::PrimaryWin,
                    pk_range_id_for_feedback.as_ref(),
                    primary_region.as_ref(),
                );
                let tr = result.expect("Ok by primary_was_final guard");
                session::capture(ctx, &tr);
                return HedgedRaceResult::Terminal(response::finalize_hedge_attempt(
                    Box::new(tr),
                    parent_diagnostics,
                ));
            }
            tracing::debug!(
                activity_id = %ctx.config.activity_id,
                "hedging::execute: primary completed pre-threshold transient; launching secondary",
            );
            Box::pin(futures::future::ready((result, diag)))
        }
        Either::Right((TimerEvent::ThresholdElapsed, remaining_primary)) => remaining_primary,
        Either::Right((TimerEvent::DeadlineFired, remaining_primary)) => {
            tracing::debug!(
                activity_id = %ctx.config.activity_id,
                "hedging::execute: deadline fired pre-threshold; harvesting primary",
            );
            deadline::harvest_remaining_attempt(
                &ctx.services.clock,
                remaining_primary,
                &mut parent_diagnostics,
                deadline::effective_harvest_window(ctx.policy.configured_request_timeout),
            )
            .await;
            parent_diagnostics.set_hedge_diagnostics(
                HedgeDiagnostics::primary_only_deadline_exceeded(
                    strategy_config,
                    primary_region_for_diag.clone(),
                ),
            );
            return HedgedRaceResult::Terminal(Err(deadline::application_cancelled_error(
                parent_diagnostics,
            )));
        }
    };

    // Borrowed by the secondary; only coordinator observation checkpoints publish to it.
    let shared_hub_region_latch = request::should_build_shared_hub_region_latch(
        ctx.config.pipeline_type,
        ctx.can_use_multiple_write_locations,
    )
    .then(|| AtomicBool::new(ctx.hub_region_processing_only_initial));
    let secondary_shared_latch = shared_hub_region_latch.as_ref();
    let secondary_diag = parent_diagnostics.clone_for_hedge_attempt();
    let secondary_dispatch = secondary_diag.leg_dispatch(
        secondary_region.clone(),
        crate::diagnostics::ExecutionContext::Hedging,
    );
    parent_diagnostics.record_hedge_fanout(primary_dispatch, secondary_dispatch);
    let secondary_attempt = Box::pin(async move {
        let mut diag = secondary_diag;
        let result = operation::perform_single_attempt(
            ctx,
            secondary_routing,
            request::hedge_options(
                ctx,
                primary_routing,
                ExecutionContext::Hedging,
                secondary_shared_latch,
            ),
            &mut diag,
        )
        .await;
        (result, diag)
    });

    tracing::debug!(
        activity_id = %ctx.config.activity_id,
        shared_hub_region_latch = shared_hub_region_latch.is_some(),
        "hedging::execute: threshold elapsed; secondary launched",
    );
    tracing::debug!(
        activity_id = %ctx.config.activity_id,
        threshold_ms = threshold.get().as_millis() as u64,
        secondary_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
        "cosmos.hedge.alternate_spawned",
    );

    let mut captured_pk_range_id: Option<PartitionKeyRangeId> = None;
    let mut race_observed_session_unavailable = false;
    // Keep the losing future alive until winner effects and diagnostics have been finalized.
    let outcome = match future::select(primary_attempt, secondary_attempt).await {
        Either::Left(((primary_result, primary_diag), secondary_remaining)) => {
            parent_diagnostics.merge_hedge_attempt(primary_diag);
            captured_pk_range_id =
                captured_pk_range_id.or_else(|| hedging::pk_range_id_from_result(&primary_result));
            race_observed_session_unavailable |= hedging::observe(
                ctx,
                retry_state_snapshot,
                &primary_routing.endpoint,
                &primary_result,
                shared_hub_region_latch.as_ref(),
                false,
            )
            .await;
            match hedging::classify_hedge_result(primary_result) {
                HedgeClass::Final(tr) => {
                    parent_diagnostics.set_hedge_diagnostics(
                        HedgeDiagnostics::primary_won_after_hedge(
                            strategy_config,
                            primary_region_for_diag.clone(),
                            secondary_region_for_diag.clone(),
                        ),
                    );
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        "hedging::execute: primary won after threshold",
                    );
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        which = "secondary",
                        target_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "primary_won_post_threshold",
                        "cosmos.hedge.canceled",
                    );
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.winner_selected",
                    );
                    routing::record_hedge_outcome(
                        ctx.services.location_state_store,
                        HedgeOutcome::PrimaryWin,
                        captured_pk_range_id
                            .as_ref()
                            .or(ctx.partition_key_range_id.as_ref()),
                        primary_region.as_ref(),
                    );
                    session::capture(ctx, &tr);
                    HedgedRaceResult::Terminal(response::finalize_hedge_attempt(
                        tr,
                        parent_diagnostics,
                    ))
                }
                HedgeClass::Transient => {
                    let Some((secondary_result, secondary_diag)) =
                        deadline::await_attempt_or_deadline_harvest(
                            &ctx.services.clock,
                            secondary_remaining,
                            ctx.policy.deadline,
                            &mut parent_diagnostics,
                            deadline::effective_harvest_window(
                                ctx.policy.configured_request_timeout,
                            ),
                        )
                        .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.config.activity_id,
                            "hedging::execute: deadline fired awaiting secondary after primary transient",
                        );
                        parent_diagnostics.set_hedge_diagnostics(
                            HedgeDiagnostics::cancelled_awaiting_partner(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ),
                        );
                        return HedgedRaceResult::Terminal(Err(
                            deadline::application_cancelled_error(parent_diagnostics),
                        ));
                    };
                    let AttemptOutcome {
                        result: secondary_result,
                        sent_hub_header,
                    } = secondary_result;
                    parent_diagnostics.merge_hedge_attempt(secondary_diag);
                    captured_pk_range_id = captured_pk_range_id
                        .or_else(|| hedging::pk_range_id_from_result(&secondary_result));
                    let hub_only_read = ctx.operation.is_read_only() && sent_hub_header;
                    race_observed_session_unavailable |= hedging::observe(
                        ctx,
                        retry_state_snapshot,
                        &secondary_routing.endpoint,
                        &secondary_result,
                        shared_hub_region_latch.as_ref(),
                        hub_only_read,
                    )
                    .await;
                    match hedging::classify_secondary_hedge_result(
                        secondary_result,
                        metadata_primary_authoritative,
                        hub_only_read,
                    ) {
                        HedgeClass::Final(tr) => {
                            parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ));
                            tracing::debug!(
                                activity_id = %ctx.config.activity_id,
                                "hedging::execute: secondary won after primary transient",
                            );
                            tracing::debug!(
                                activity_id = %ctx.config.activity_id,
                                winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.winner_selected",
                            );
                            routing::record_hedge_outcome(
                                ctx.services.location_state_store,
                                HedgeOutcome::AlternateWin,
                                captured_pk_range_id
                                    .as_ref()
                                    .or(ctx.partition_key_range_id.as_ref()),
                                primary_region.as_ref(),
                            );
                            session::capture(ctx, &tr);
                            routing::populate_hub_region_cache_for_hedge_winner(
                                ctx,
                                &secondary_routing.endpoint,
                                hub_only_read,
                                &tr,
                                captured_pk_range_id
                                    .as_ref()
                                    .or(ctx.partition_key_range_id.as_ref()),
                            )
                            .await;
                            HedgedRaceResult::Terminal(response::finalize_hedge_attempt(
                                tr,
                                parent_diagnostics,
                            ))
                        }
                        HedgeClass::Transient => hedging::finalize_both_transient(
                            &ctx.services.clock,
                            ctx.config.activity_id,
                            ctx.policy.deadline,
                            strategy_config,
                            primary_region,
                            secondary_region,
                            primary_region_for_diag.clone(),
                            secondary_region_for_diag.clone(),
                            parent_diagnostics,
                            captured_pk_range_id,
                            race_observed_session_unavailable,
                        ),
                    }
                }
            }
        }
        Either::Right(((secondary_result, secondary_diag), primary_remaining)) => {
            let AttemptOutcome {
                result: secondary_result,
                sent_hub_header,
            } = secondary_result;
            parent_diagnostics.merge_hedge_attempt(secondary_diag);
            captured_pk_range_id = captured_pk_range_id
                .or_else(|| hedging::pk_range_id_from_result(&secondary_result));
            let hub_only_read = ctx.operation.is_read_only() && sent_hub_header;
            race_observed_session_unavailable |= hedging::observe(
                ctx,
                retry_state_snapshot,
                &secondary_routing.endpoint,
                &secondary_result,
                shared_hub_region_latch.as_ref(),
                hub_only_read,
            )
            .await;
            match hedging::classify_secondary_hedge_result(
                secondary_result,
                metadata_primary_authoritative,
                hub_only_read,
            ) {
                HedgeClass::Final(tr) => {
                    parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                        strategy_config,
                        primary_region_for_diag.clone(),
                        secondary_region_for_diag.clone(),
                    ));
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        "hedging::execute: secondary won race",
                    );
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        which = "primary",
                        target_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                        reason = "secondary_won_race",
                        "cosmos.hedge.canceled",
                    );
                    tracing::debug!(
                        activity_id = %ctx.config.activity_id,
                        winner_region = ?secondary_region.as_ref().map(crate::options::Region::as_str),
                        was_hedge = true,
                        "cosmos.hedge.winner_selected",
                    );
                    routing::record_hedge_outcome(
                        ctx.services.location_state_store,
                        HedgeOutcome::AlternateWin,
                        captured_pk_range_id
                            .as_ref()
                            .or(ctx.partition_key_range_id.as_ref()),
                        primary_region.as_ref(),
                    );
                    session::capture(ctx, &tr);
                    routing::populate_hub_region_cache_for_hedge_winner(
                        ctx,
                        &secondary_routing.endpoint,
                        hub_only_read,
                        &tr,
                        captured_pk_range_id
                            .as_ref()
                            .or(ctx.partition_key_range_id.as_ref()),
                    )
                    .await;
                    HedgedRaceResult::Terminal(response::finalize_hedge_attempt(
                        tr,
                        parent_diagnostics,
                    ))
                }
                HedgeClass::Transient => {
                    let Some((primary_result, primary_diag)) =
                        deadline::await_attempt_or_deadline_harvest(
                            &ctx.services.clock,
                            primary_remaining,
                            ctx.policy.deadline,
                            &mut parent_diagnostics,
                            deadline::effective_harvest_window(
                                ctx.policy.configured_request_timeout,
                            ),
                        )
                        .await
                    else {
                        tracing::debug!(
                            activity_id = %ctx.config.activity_id,
                            "hedging::execute: deadline fired awaiting primary after secondary transient",
                        );
                        parent_diagnostics.set_hedge_diagnostics(
                            HedgeDiagnostics::cancelled_awaiting_partner(
                                strategy_config,
                                primary_region_for_diag.clone(),
                                secondary_region_for_diag.clone(),
                            ),
                        );
                        return HedgedRaceResult::Terminal(Err(
                            deadline::application_cancelled_error(parent_diagnostics),
                        ));
                    };
                    parent_diagnostics.merge_hedge_attempt(primary_diag);
                    captured_pk_range_id = captured_pk_range_id
                        .or_else(|| hedging::pk_range_id_from_result(&primary_result));
                    race_observed_session_unavailable |= hedging::observe(
                        ctx,
                        retry_state_snapshot,
                        &primary_routing.endpoint,
                        &primary_result,
                        shared_hub_region_latch.as_ref(),
                        false,
                    )
                    .await;
                    match hedging::classify_hedge_result(primary_result) {
                        HedgeClass::Final(tr) => {
                            parent_diagnostics.set_hedge_diagnostics(
                                HedgeDiagnostics::primary_won_after_hedge(
                                    strategy_config,
                                    primary_region_for_diag.clone(),
                                    secondary_region_for_diag.clone(),
                                ),
                            );
                            tracing::debug!(
                                activity_id = %ctx.config.activity_id,
                                "hedging::execute: primary won after secondary transient",
                            );
                            tracing::debug!(
                                activity_id = %ctx.config.activity_id,
                                winner_region = ?primary_region.as_ref().map(crate::options::Region::as_str),
                                was_hedge = true,
                                "cosmos.hedge.winner_selected",
                            );
                            routing::record_hedge_outcome(
                                ctx.services.location_state_store,
                                HedgeOutcome::PrimaryWin,
                                captured_pk_range_id
                                    .as_ref()
                                    .or(ctx.partition_key_range_id.as_ref()),
                                primary_region.as_ref(),
                            );
                            session::capture(ctx, &tr);
                            HedgedRaceResult::Terminal(response::finalize_hedge_attempt(
                                tr,
                                parent_diagnostics,
                            ))
                        }
                        HedgeClass::Transient => hedging::finalize_both_transient(
                            &ctx.services.clock,
                            ctx.config.activity_id,
                            ctx.policy.deadline,
                            strategy_config,
                            primary_region,
                            secondary_region,
                            primary_region_for_diag.clone(),
                            secondary_region_for_diag.clone(),
                            parent_diagnostics,
                            captured_pk_range_id,
                            race_observed_session_unavailable,
                        ),
                    }
                }
            }
        }
    };
    outcome
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
/// Status is set to [`crate::error::status_codes::TRANSPORT_GENERATED_503`] so
/// retry-evaluation and telemetry can discriminate a client-side
/// "both legs transient" classification from any other 5xx surface.
/// Diagnostics are not threaded here: the surrounding
/// `BothTransient` flow returns a builder that the failover loop
/// keeps mutating; only the terminal error-surfacing path (when the
/// failover budget is exhausted) attaches diagnostics, and it does
/// so from the loop's own builder via the standard error-promotion
/// path.
pub(crate) fn transient_outcome_error(
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
        .with_status(crate::error::status_codes::TRANSPORT_GENERATED_503)
        .with_message(format!(
            "hedging completed without producing a final response \
             (primary={p}, secondary={s})"
        ))
        .build()
}

/// Returns race observations for sequential fallback unless the operation deadline has elapsed.
///
/// Only deadline cancellation stamps terminal diagnostics here. Exhausted retry
/// budget is handled by the outer continuation stage, so later fallback success
/// cannot inherit a misleading both-transient terminal state.
#[allow(
    clippy::too_many_arguments,
    reason = "All ten values come from `hedging::execute`'s state and \
              are passed through to a single helper call site. Bundling \
              them into a struct would add an indirection without \
              improving readability — the two-region pairing (raw / \
              for-diag) is intentional and explicit at the call site."
)]
pub(crate) fn finalize_both_transient(
    clock: &crate::clock::Clock,
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
    let deadline_was_elapsed = deadline::deadline_elapsed(clock, deadline);
    tracing::warn!(
        activity_id = %activity_id,
        deadline_elapsed = deadline_was_elapsed,
        primary_region = ?primary_region.as_ref().map(Region::as_str),
        secondary_region = ?secondary_region.as_ref().map(Region::as_str),
        "cosmos.hedge.both_transient",
    );
    if deadline_was_elapsed {
        parent_diagnostics.set_hedge_diagnostics(HedgeDiagnostics::both_transient(
            strategy_config,
            primary_region_for_diag,
            secondary_region_for_diag,
            deadline_was_elapsed,
        ));
        tracing::debug!(
            activity_id = %activity_id,
            "hedging::execute: both transient under elapsed deadline; surfacing app-cancel",
        );
        HedgedRaceResult::Terminal(Err(deadline::application_cancelled_error(
            parent_diagnostics,
        )))
    } else {
        tracing::debug!(
            activity_id = %activity_id,
            primary_region = ?primary_region.as_ref().map(Region::as_str),
            secondary_region = ?secondary_region.as_ref().map(Region::as_str),
            observed_session_unavailable,
            "hedging::execute: both legs transient; bubbling up for failover-loop fallback",
        );
        HedgedRaceResult::BothTransient {
            last_error: hedging::transient_outcome_error(
                primary_region.as_ref(),
                secondary_region.as_ref(),
            ),
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

pub(crate) mod budget;
pub(crate) mod eligibility;

/// Applies a completed leg's feedback before publishing its session observation to its partner.
///
/// Only the coordinator writes the race-local signal. A secondary that has
/// already prepared its request retains the sent-header fact returned by that leg.
async fn observe(
    ctx: &AttemptContext<'_>,
    retry_state: &OperationRetryState,
    endpoint: &routing::CosmosEndpoint,
    result: &crate::error::Result<TransportResult>,
    signal: Option<&AtomicBool>,
    hub_only_read: bool,
) -> bool {
    let observed =
        recovery::hedge::observe(ctx, retry_state, endpoint, result, hub_only_read).await;
    if observed {
        if let Some(signal) = signal {
            signal.store(true, Ordering::Release);
        }
    }
    observed
}
