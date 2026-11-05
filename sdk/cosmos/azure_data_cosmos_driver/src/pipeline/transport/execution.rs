// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Orchestrates preparation, dispatch, diagnostics, and same-endpoint recovery.

#[cfg(feature = "fault_injection")]
use super::fault_injection;
use super::{
    adaptive_transport::AdaptiveTransport,
    deadline, diagnostics, dispatch,
    recovery::{
        self, RecoveryDecision, RetryDelay, ThrottleDecision, ThrottleRetryPolicy,
        ThrottleRetryProgress, TransportRetryProgress,
    },
    request_preparation,
    response::{self, CompletedTransportAttempt, CompletionKind},
    types::{TransportRequest, TransportResult},
    EndpointKey,
};
use crate::{
    diagnostics::{DiagnosticsContextBuilder, PipelineKind, RequestSentStatus, TransportSecurity},
    driver::jitter,
    models::Credential,
};
use azure_core::http::headers::HeaderValue;
use std::{
    ops::ControlFlow,
    time::{Duration, Instant},
};

/// Fixed data for an invocation; each operation attempt or hedge leg supplies its own limits.
pub(crate) struct TransportPipelineConfig<'a> {
    pub allow_sent_transport_retry: bool,
    pub user_agent: &'a HeaderValue,
    pub client_id: &'a HeaderValue,
    pub pipeline_type: PipelineKind,
    pub transport_security: TransportSecurity,
    pub endpoint_key: EndpointKey,
    pub account_name: Option<String>,
    pub collection_rid: Option<String>,
    pub throttle_policy: ThrottleRetryPolicy,
}

/// Dependencies that perform I/O, separate from request data and retry policy.
pub(crate) struct TransportPipelineServices<'a> {
    pub transport: &'a AdaptiveTransport,
    pub credential: &'a Credential,
}

/// Executes a request, including transport retries, against its selected target.
pub(crate) async fn execute_transport_pipeline(
    request: TransportRequest,
    config: &TransportPipelineConfig<'_>,
    services: &TransportPipelineServices<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let mut progress = TransportRetryProgress::default();
    loop {
        if request
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            tracing::trace!("transport pipeline: deadline exceeded before attempt");
            return TransportResult::deadline_exceeded(RequestSentStatus::NotSent);
        }

        let attempt = execute_attempt(&request, config, services, &mut progress, diagnostics).await;
        if matches!(
            attempt.completion,
            CompletionKind::AuthorizationFailure | CompletionKind::RequestEncodingFailure
        ) {
            return attempt.result;
        }

        let decision = observe_recovery(&request, config, services.transport, &progress, &attempt);
        if let ControlFlow::Break(result) =
            apply_recovery(decision, request.deadline, &mut progress, attempt).await
        {
            return result;
        }
    }
}

/// Prepares and sends one request, then records diagnostics and returns its classified outcome.
async fn execute_attempt(
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    services: &TransportPipelineServices<'_>,
    progress: &mut TransportRetryProgress,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> CompletedTransportAttempt {
    let handle = diagnostics::start_attempt(request, config, services, progress, diagnostics);
    let http_request = request_preparation::build_http_request(request);
    let timeout = request
        .deadline
        .map(|deadline| deadline::remaining_request_timeout(deadline, Instant::now()));
    tracing::trace!(
        per_request_timeout = ?timeout, "transport pipeline: computed per-request timeout");

    let http_request = match request_preparation::prepare_request(
        http_request,
        request,
        config,
        services.credential,
    )
    .await
    {
        Ok(request) => request,
        Err(failure) => {
            let attempt = response::classify_preparation_failure(failure);
            diagnostics::complete_attempt(&attempt, handle, diagnostics);
            return attempt;
        }
    };
    diagnostics::record_transport_start(handle, diagnostics);

    #[cfg(feature = "fault_injection")]
    let (http_request, collector) =
        fault_injection::attach_collector(http_request, diagnostics.fault_injection_enabled());

    let dispatch = dispatch::dispatch_http_request(
        &http_request,
        services.transport,
        timeout,
        progress.excluded_shard_id.take(),
        &config.endpoint_key,
    )
    .await;
    diagnostics::record_dispatch_observation(&dispatch, handle, diagnostics);
    let attempt = response::classify_dispatch(dispatch, request.transport_mode);
    diagnostics::complete_attempt(&attempt, handle, diagnostics);

    #[cfg(feature = "fault_injection")]
    fault_injection::record_evaluations(collector, handle, diagnostics);
    tracing::debug!(
        outcome = ?attempt.result.outcome, "transport request complete");
    attempt
}

/// Checks alternate-shard availability or samples a throttle delay to choose the next action.
///
/// Connectivity replay takes precedence; throttle waits also account for the remaining deadline.
fn observe_recovery(
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    transport: &AdaptiveTransport,
    progress: &TransportRetryProgress,
    attempt: &CompletedTransportAttempt,
) -> RecoveryDecision {
    if let Some(shard_id) = recovery::connectivity_retry_candidate(
        &attempt.result,
        attempt.shard_id,
        progress.local_connectivity_retry_count,
        config.allow_sent_transport_retry,
    ) {
        let decision = recovery::resolve_connectivity_retry(
            shard_id,
            transport.can_retry_on_different_shard(shard_id, &config.endpoint_key),
        );
        if !matches!(decision, RecoveryDecision::Propagate) {
            return decision;
        }
    }
    #[cfg(feature = "unstable_dtx")]
    let is_dtx = request.resource_type == crate::models::ResourceType::DistributedTransactionBatch;
    #[cfg(not(feature = "unstable_dtx"))]
    let is_dtx = false;

    let Some(source) = recovery::throttle_delay_source(
        &attempt.result,
        &config.throttle_policy,
        progress.throttle,
        is_dtx,
    ) else {
        return RecoveryDecision::Propagate;
    };
    let delay = realize_retry_delay(source);
    let decision =
        recovery::evaluate_throttle_budget(delay, &config.throttle_policy, progress.throttle);
    if matches!(decision, ThrottleDecision::Propagate) {
        return RecoveryDecision::Propagate;
    }
    let remaining = request
        .deadline
        .map(|deadline| deadline.saturating_duration_since(Instant::now()));
    recovery::plan_throttle_wait(decision, remaining)
}

/// Selects a service or fallback delay and returns the retry decision and next throttle progress.
///
/// Used by bootstrap requests that share throttle limits but do not run the transport loop.
pub(crate) fn observe_throttle_retry(
    retry_after_ms: Option<u64>,
    policy: &ThrottleRetryPolicy,
    progress: ThrottleRetryProgress,
) -> ThrottleDecision {
    match recovery::retry_delay_source(retry_after_ms, policy, progress) {
        Some(source) => {
            recovery::evaluate_throttle_budget(realize_retry_delay(source), policy, progress)
        }
        None => ThrottleDecision::Propagate,
    }
}

/// Returns the service delay unchanged, or samples jitter around the supplied fallback delay.
fn realize_retry_delay(source: RetryDelay) -> Duration {
    match source {
        RetryDelay::Service(delay) => delay,
        RetryDelay::Jittered { base, ratio } => {
            if ratio == 0.0 || base.is_zero() {
                base
            } else {
                Duration::from_secs_f64(jitter::with_jitter(base.as_secs_f64(), ratio))
            }
        }
    }
}

/// Applies the chosen wait and progress updates, or returns the result that ends the invocation.
///
/// Ordinary throttle progress is committed only after sleeping and checking the deadline.
async fn apply_recovery(
    decision: RecoveryDecision,
    deadline: Option<Instant>,
    progress: &mut TransportRetryProgress,
    attempt: CompletedTransportAttempt,
) -> ControlFlow<TransportResult> {
    match decision {
        RecoveryDecision::RetryDifferentShard { failed_shard_id } => {
            if let Some(failed) = response::failed_transport_shard(&attempt) {
                progress.failed_shards.push(failed);
            }
            progress.local_connectivity_retry_count += 1;
            progress.excluded_shard_id = Some(failed_shard_id);
        }
        RecoveryDecision::RetryThrottle { delay, next } => {
            sleep(delay).await;
            if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
                return ControlFlow::Break(TransportResult::deadline_exceeded(
                    RequestSentStatus::Sent,
                ));
            }
            progress.throttle = next;
        }
        RecoveryDecision::ForceFinalThrottle { delay, next } => {
            if !delay.is_zero() {
                sleep(delay).await;
            }
            progress.throttle = next;
        }
        RecoveryDecision::Propagate => return ControlFlow::Break(attempt.result),
        RecoveryDecision::DeadlineExceeded => {
            return ControlFlow::Break(TransportResult::deadline_exceeded(RequestSentStatus::Sent));
        }
    }
    ControlFlow::Continue(())
}

/// Waits for the retry delay, using zero if it exceeds the timer's supported duration range.
async fn sleep(delay: Duration) {
    azure_core::sleep(
        azure_core::time::Duration::try_from(delay).unwrap_or(azure_core::time::Duration::ZERO),
    )
    .await;
}

#[cfg(test)]
mod tests;
