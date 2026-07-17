// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:ignore evals

//! Transport pipeline: the core loop for executing a single HTTP attempt.
//!
//! Header application and request signing live in their own modules:
//! - [`super::cosmos_headers`] — `apply_cosmos_headers`
//! - [`super::request_signing`] — `sign_request`
//!
//! This module handles the transport-level retry loop (429 throttling),
//! request-sent-status tracking, per-attempt diagnostics, and deadline
//! enforcement.

use std::time::{Duration, Instant};

use futures::{future::Either, pin_mut};
use tracing::trace;

#[cfg(feature = "preview_dtx")]
use crate::models::ResourceType;
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, FailedTransportShardDiagnostics, PipelineType,
        RequestEvent, RequestEventType, RequestHandle, RequestSentStatus, TransportSecurity,
        TransportShardDiagnostics,
    },
    models::{CosmosResponseHeaders, CosmosStatus, Credential, SubStatusCode},
};

use super::{
    adaptive_transport::AdaptiveTransport,
    cosmos_headers::{apply_cosmos_headers, apply_read_consistency_strategy, NO_RETRY_449},
    cosmos_transport_client::HttpRequest,
    infer_request_sent_status,
    request_signing::sign_request,
    sharded_transport::EndpointKey,
    unwrap_response_for_gateway_v2, wrap_request_for_gateway_v2, WrapInputs,
};

use crate::driver::pipeline::components::{
    ThrottleAction, ThrottleRetryState, TransportMode, TransportOutcome, TransportRequest,
    TransportResult,
};

/// Keep a small budget before the e2e deadline so we still have time
/// to send one final attempt.
const DEADLINE_RETRY_SAFETY_MARGIN: Duration = Duration::from_millis(100);
// Internal floor used only when computing remaining deadline budget.
// This is intentionally lower than public option validation to avoid
// collapsing near-deadline retries to an entire second.
const MIN_REMAINING_REQUEST_TIMEOUT: Duration = Duration::from_millis(1);
const MAX_LOCAL_CONNECTIVITY_RETRIES: u32 = 1;

fn deadline_capped_delay(requested_delay: Duration, remaining: Duration) -> Duration {
    let budget_for_delay = remaining.saturating_sub(DEADLINE_RETRY_SAFETY_MARGIN);
    requested_delay.min(budget_for_delay)
}

fn remaining_request_timeout(deadline: Option<Instant>) -> Option<Duration> {
    deadline.map(|deadline| {
        deadline
            .saturating_duration_since(Instant::now())
            .max(MIN_REMAINING_REQUEST_TIMEOUT)
    })
}

fn forced_final_retry_delay_from_remaining(remaining: Duration) -> Option<Duration> {
    if remaining.is_zero() {
        return None;
    }

    let half_remaining = remaining / 2;

    if half_remaining < DEADLINE_RETRY_SAFETY_MARGIN {
        return Some(Duration::ZERO);
    }

    Some(half_remaining)
}

fn forced_final_retry_delay(deadline: Option<Instant>) -> Option<Duration> {
    match deadline {
        Some(deadline) => forced_final_retry_delay_from_remaining(
            deadline.saturating_duration_since(Instant::now()),
        ),
        None => Some(Duration::ZERO),
    }
}

/// Decides whether to retry a 429 throttling response at the transport level.
///
/// Honors the service-specified `x-ms-retry-after-ms` header when present.
/// Falls back to exponential backoff from a small base delay (5ms) if the
/// header is absent. Individual retry delays are capped at `max_per_retry_delay`
/// (5s default) to avoid excessive waits from a misbehaving service response.
pub(crate) fn evaluate_transport_retry(
    result: &TransportResult,
    throttle_state: &ThrottleRetryState,
    is_distributed_transaction_request: bool,
) -> ThrottleAction {
    let is_throttled =
        is_transport_throttle_retry_eligible(result, is_distributed_transaction_request);

    if !is_throttled {
        return ThrottleAction::Propagate;
    }

    if throttle_state.attempt_count >= throttle_state.max_attempts {
        return ThrottleAction::Propagate;
    }

    // Extract the service-specified retry delay from the parsed cosmos
    // response headers, or fall back to exponential backoff.
    let service_delay = result
        .cosmos_headers()
        .and_then(|h| h.retry_after_ms)
        .map(Duration::from_millis);

    let delay = service_delay.unwrap_or_else(|| throttle_state.fallback_delay());

    // Cap individual retry delay to avoid excessive waits.
    let delay = delay.min(throttle_state.max_per_retry_delay);

    let new_cumulative = throttle_state.cumulative_delay + delay;

    if new_cumulative > throttle_state.max_wait_time {
        return ThrottleAction::Propagate;
    }

    ThrottleAction::Retry {
        delay,
        new_state: ThrottleRetryState {
            attempt_count: throttle_state.attempt_count + 1,
            cumulative_delay: new_cumulative,
            ..*throttle_state
        },
    }
}

/// Whether a throttled (`429`) transport result is eligible for the shared
/// transport throttle-retry path.
///
/// A **body-bearing** distributed-transaction `429` is intentionally excluded:
/// it carries a coordinator transaction result, so it is routed to the DTX outer
/// loop (governed by the coordinator's `isRetriable` + `retry-after`) instead of
/// the shared throttle path. As a result the user-configured throttle cap
/// (`ThrottlingRetryOptions`) does not bound a body-bearing DTX `429`; the DTX
/// outer-loop budget and the operation deadline do. A **bodyless** DTX `429`
/// (and every non-DTX `429`) uses the shared path and honors that cap.
fn is_transport_throttle_retry_eligible(
    result: &TransportResult,
    is_distributed_transaction_request: bool,
) -> bool {
    match &result.outcome {
        TransportOutcome::HttpError { status, body, .. } if status.is_throttled() => {
            !is_distributed_transaction_request || body.is_empty()
        }
        _ => false,
    }
}

/// Context parameters for the transport pipeline that remain constant
/// across retries within a single operation attempt.
pub(crate) struct TransportPipelineContext<'a> {
    pub transport: &'a AdaptiveTransport,
    pub allow_sent_transport_retry: bool,
    pub credential: &'a Credential,
    pub user_agent: &'a azure_core::http::headers::HeaderValue,
    pub pipeline_type: PipelineType,
    pub transport_security: TransportSecurity,
    /// Pre-computed `host:port` key for the target endpoint.
    ///
    /// Computed once by the operation pipeline from the routing-level endpoint
    /// so the transport pipeline doesn't need to allocate a `String` per attempt.
    pub endpoint_key: EndpointKey,
    /// Global database account name used by Gateway 2.0 request wrapping.
    pub account_name: Option<String>,
    /// Container `_rid` used by Gateway 2.0 request wrapping. Emitted as the
    /// RNTBD `CollectionRid` token (0x0035) so the thin-client proxy can resolve
    /// the partition without an extra cache round-trip.
    pub collection_rid: Option<String>,
    /// Maximum number of 429 (throttle) retries for this operation.
    ///
    /// Resolved by the operation pipeline from the effective
    /// [`ThrottlingRetryOptionsView::max_retry_count`](crate::options::ThrottlingRetryOptionsView::max_retry_count)
    /// (defaulting to `9`). `0` disables throttle retries.
    ///
    /// **Scope**: This budget is per `execute_transport_pipeline` invocation,
    /// not per logical operation — an operation that performs cross-region
    /// failover or hedging will enter this pipeline once per leg, each with
    /// a fresh budget. Per-operation total time is bounded by the operation's
    /// `end_to_end_latency_policy` deadline, not by this knob.
    pub max_throttle_attempts: u32,
    /// Maximum cumulative wait budget across 429 (throttle) retries.
    ///
    /// Resolved by the operation pipeline from the effective
    /// [`ThrottlingRetryOptionsView::max_retry_wait_time`](crate::options::ThrottlingRetryOptionsView::max_retry_wait_time)
    /// (defaulting to 30 seconds). Same per-invocation scope note as
    /// [`max_throttle_attempts`](Self::max_throttle_attempts).
    pub max_throttle_wait_time: Duration,
}

/// Executes a single transport attempt.
///
/// Applies headers, signs the request, sends it via the selected transport, and
/// handles 429 throttle retry internally. Returns a `TransportResult` to the
/// operation pipeline for higher-level decision making.
///
/// This is the core transport loop.
pub(crate) async fn execute_transport_pipeline(
    request: TransportRequest,
    ctx: &TransportPipelineContext<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let mut throttle_state =
        ThrottleRetryState::with_limits(ctx.max_throttle_attempts, ctx.max_throttle_wait_time);
    let mut local_connectivity_retry_count = 0_u32;
    let mut prior_failed_transport_shards = Vec::<FailedTransportShardDiagnostics>::new();
    let mut excluded_shard_id = None;

    // The endpoint key is pre-computed by the operation pipeline from the
    // routing-level CosmosEndpoint so no allocation is needed here.
    let endpoint_key = &ctx.endpoint_key;

    loop {
        // Check deadline before each attempt
        if let Some(deadline) = request.deadline {
            if Instant::now() >= deadline {
                trace!("transport pipeline: deadline exceeded before attempt");
                return deadline_exceeded_result(RequestSentStatus::NotSent);
            }
        }

        // Record this attempt in diagnostics
        let execution_context = if local_connectivity_retry_count > 0 {
            ExecutionContext::TransportRetry
        } else if throttle_state.attempt_count == 0 {
            request.execution_context
        } else {
            ExecutionContext::Retry
        };

        let request_handle = diagnostics.start_request(
            execution_context,
            ctx.pipeline_type,
            ctx.transport_security,
            ctx.transport.diagnostics_kind(),
            ctx.transport.diagnostics_http_version(),
            &request.endpoint,
        );

        for failed_transport_shard in prior_failed_transport_shards.iter().cloned() {
            diagnostics.add_failed_transport_shard(request_handle, failed_transport_shard);
        }
        for _ in 0..local_connectivity_retry_count {
            diagnostics.increment_local_shard_retry_count(request_handle);
        }

        // Build HTTP request from TransportRequest
        let mut http_request = HttpRequest {
            url: request.url.clone(),
            method: request.method,
            headers: request.headers.clone(),
            body: request.body.clone(),
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };

        let per_request_timeout = remaining_request_timeout(request.deadline);
        // TODO(azure_core): Apply per-request timeout directly on Request/HttpClient
        // once azure_core/typespec_client_core exposes timeout options.
        // Tracking issue: https://github.com/Azure/azure-sdk-for-rust/issues/3878
        trace!(
            ?per_request_timeout,
            "transport pipeline: computed per-request timeout"
        );

        // Apply standard Cosmos headers
        apply_cosmos_headers(&mut http_request, ctx.user_agent);
        // V1 RCS emission: when RCS is non-Default on a read,
        // set `x-ms-cosmos-read-consistency-strategy` and strip any
        // `x-ms-consistency-level` header. GatewayV2 emits the equivalent via
        // the RNTBD `ReadConsistencyStrategy` token in
        // `wrap_request_for_gateway_v2`, so we skip the HTTP header there to
        // avoid double-encoding the same intent.
        if request.transport_mode != TransportMode::GatewayV2 {
            apply_read_consistency_strategy(
                &mut http_request,
                request.read_consistency_strategy,
                request.operation_type.is_read_only(),
            );
            // Disable ComputeGateway's server-side 449 RetryWith retry so the
            // SDK owns RetryWith handling on Gateway V1 too, normalizing it
            // with Gateway 2.0 (where 449 is never produced server-side).
            http_request.headers.insert(
                NO_RETRY_449,
                azure_core::http::headers::HeaderValue::from_static("true"),
            );
        }

        if let Err(cosmos_err) =
            sign_request(&mut http_request, ctx.credential, &request.auth_context).await
        {
            diagnostics.fail_transport_request(
                request_handle,
                cosmos_err.to_string(),
                RequestSentStatus::NotSent,
                CosmosStatus::CLIENT_GENERATED_401,
            );
            return TransportResult {
                outcome: TransportOutcome::TransportError {
                    status: CosmosStatus::CLIENT_GENERATED_401,
                    error: cosmos_err,
                    request_sent: RequestSentStatus::NotSent,
                },
            };
        }

        let should_unwrap_gateway_v2 = request.transport_mode == TransportMode::GatewayV2;
        if should_unwrap_gateway_v2 {
            let wrap_inputs = WrapInputs {
                auth_context: &request.auth_context,
                operation_type: request.operation_type,
                resource_type: request.auth_context.resource_type,
                effective_partition_key: request.effective_partition_key.as_ref(),
                effective_consistency: request.effective_consistency,
                read_consistency_strategy: request.read_consistency_strategy,
                account_name: ctx.account_name.as_deref(),
                collection_rid: ctx.collection_rid.as_deref(),
            };
            match wrap_request_for_gateway_v2(&http_request, &wrap_inputs) {
                Ok(wrapped_request) => http_request = wrapped_request,
                Err(e) => {
                    let cosmos_err = crate::error::CosmosError::builder()
                        .with_status(CosmosStatus::CLIENT_BAD_REQUEST)
                        .with_message(format!("Gateway 2.0 request wrap failed: {e}"))
                        .with_source(e)
                        .build();
                    return gateway_v2_wrap_error_result(cosmos_err, request_handle, diagnostics);
                }
            }
        }

        // Record transport start event
        diagnostics.add_event(
            request_handle,
            RequestEvent::new(RequestEventType::TransportStart),
        );

        #[cfg(feature = "fault_injection")]
        let mut evaluation_collector = if diagnostics.fault_injection_enabled() {
            let collector = crate::fault_injection::EvaluationCollector::default();
            http_request.evaluation_collector = Some(collector.clone());
            Some(collector)
        } else {
            None
        };

        let result = execute_http_attempt(HttpAttemptInputs {
            http_request: &http_request,
            transport: ctx.transport,
            per_request_timeout,
            request_handle,
            diagnostics,
            excluded_shard_id: excluded_shard_id.take(),
            endpoint_key,
            should_unwrap_gateway_v2,
        })
        .await;

        #[cfg(feature = "fault_injection")]
        if let Some(collector) = evaluation_collector.take() {
            let evals = collector.take();
            if !evals.is_empty() {
                diagnostics.set_fault_injection_evaluations(request_handle, evals);
            }
        }
        tracing::debug!(
            outcome = ?result.result.outcome,
            "transport request complete"
        );

        if result.shard_id.is_some_and(|failed_shard_id| {
            local_connectivity_retry_count < MAX_LOCAL_CONNECTIVITY_RETRIES
                && should_retry_connectivity_failure(&result.result, ctx.allow_sent_transport_retry)
                && ctx
                    .transport
                    .can_retry_on_different_shard(failed_shard_id, endpoint_key)
        }) {
            if let Some(failed_transport_shard) = failed_transport_shard(&result) {
                prior_failed_transport_shards.push(failed_transport_shard);
            }
            local_connectivity_retry_count += 1;
            excluded_shard_id = result.shard_id;
            continue;
        }

        // Check for 429 throttling → transport-level retry
        let result = result.result;
        // DTX request detection only exists when the preview feature is enabled;
        // in default builds no request can carry the DTX typed resource value, so
        // the flag is a constant `false` and the throttle path behaves as before.
        #[cfg(feature = "preview_dtx")]
        let is_distributed_transaction_request =
            request.resource_type == ResourceType::DistributedTransactionBatch;
        #[cfg(not(feature = "preview_dtx"))]
        let is_distributed_transaction_request = false;
        let action =
            evaluate_transport_retry(&result, &throttle_state, is_distributed_transaction_request);
        match action {
            ThrottleAction::Retry { delay, new_state } => {
                // Never sleep past the end-to-end deadline. If there is no remaining
                // budget, fail fast instead of delaying.
                let mut effective_delay = delay;
                if let Some(deadline) = request.deadline {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        return deadline_exceeded_result(RequestSentStatus::Sent);
                    }

                    // Never consume the entire remaining budget with delay;
                    // keep a small margin for one final request attempt.
                    effective_delay = deadline_capped_delay(effective_delay, remaining);
                }

                azure_core::sleep(
                    azure_core::time::Duration::try_from(effective_delay)
                        .unwrap_or(azure_core::time::Duration::ZERO),
                )
                .await;

                if let Some(deadline) = request.deadline {
                    if Instant::now() >= deadline {
                        return deadline_exceeded_result(RequestSentStatus::Sent);
                    }
                }

                throttle_state = new_state;
                continue;
            }
            ThrottleAction::Propagate => {
                let is_throttled = is_transport_throttle_retry_eligible(
                    &result,
                    is_distributed_transaction_request,
                );

                // Honor the user-configured `max_retry_count` as the cap on
                // *total* retries on the wire (matching the .NET-parity
                // `MaxRetryAttemptsOnRateLimitedRequests` contract): when the
                // count budget is exhausted (`attempt_count >= max_attempts`),
                // suppress the one-shot forced-final retry too. Otherwise a
                // user-configured `max_retry_count = N` would still produce
                // `N + 1` retries on the wire (one extra forced-final round
                // trip), which is an off-by-one against .NET and a surprising
                // extra request for the `N = 0` "fail-fast" configuration.
                //
                // The forced-final retry remains active when
                // `evaluate_transport_retry` returns `Propagate` for a
                // *non-count* reason (i.e., the cumulative-wait budget was
                // hit before the count budget), preserving the historical
                // safety net for `max_retry_wait_time` exhaustion.
                if throttle_state.attempt_count < throttle_state.max_attempts
                    && throttle_state.can_use_forced_final_retry()
                    && is_throttled
                {
                    if let Some(final_delay) = forced_final_retry_delay(request.deadline) {
                        // One extra retry attempt after throttle budget is exhausted.
                        // When no deadline exists, this retry is immediate.
                        if !final_delay.is_zero() {
                            azure_core::sleep(
                                azure_core::time::Duration::try_from(final_delay)
                                    .unwrap_or(azure_core::time::Duration::ZERO),
                            )
                            .await;
                        }

                        throttle_state = throttle_state.mark_forced_final_retry_used();
                        continue;
                    }
                }

                return result;
            }
        }
    }
}

fn deadline_exceeded_result(request_sent: RequestSentStatus) -> TransportResult {
    TransportResult::deadline_exceeded(request_sent)
}

/// Bundled inputs for [`execute_http_attempt`], mirroring the [`WrapInputs`]
/// envelope used by the operation pipeline so the function avoids a long
/// positional argument list.
struct HttpAttemptInputs<'a> {
    http_request: &'a HttpRequest,
    transport: &'a AdaptiveTransport,
    per_request_timeout: Option<Duration>,
    request_handle: RequestHandle,
    diagnostics: &'a mut DiagnosticsContextBuilder,
    excluded_shard_id: Option<u64>,
    endpoint_key: &'a EndpointKey,
    should_unwrap_gateway_v2: bool,
}

async fn execute_http_attempt(inputs: HttpAttemptInputs<'_>) -> ExecutedTransportAttempt {
    let HttpAttemptInputs {
        http_request,
        transport,
        per_request_timeout,
        request_handle,
        diagnostics,
        excluded_shard_id,
        endpoint_key,
        should_unwrap_gateway_v2,
    } = inputs;
    if let Some(timeout_duration) = per_request_timeout {
        // Pre-select the shard so we know which shard the request was dispatched
        // to even if the transport future is cancelled by the timeout race.
        // The ID is passed as a preferred_shard_id hint to the actual dispatch
        // so the same shard is reused when still selectable, keeping the
        // diagnostic shard ID accurate.
        let dispatched_shard = transport.pre_select_shard(excluded_shard_id, endpoint_key);

        let transport_future = execute_http_attempt_future(
            http_request,
            transport,
            excluded_shard_id,
            endpoint_key,
            dispatched_shard,
        );
        let timeout_future = async {
            azure_core::sleep(
                azure_core::time::Duration::try_from(timeout_duration)
                    .unwrap_or(azure_core::time::Duration::ZERO),
            )
            .await;
        };

        pin_mut!(transport_future);
        pin_mut!(timeout_future);

        return match futures::future::select(transport_future, timeout_future).await {
            Either::Left((attempt_result, _)) => finalize_http_attempt(
                attempt_result,
                request_handle,
                diagnostics,
                should_unwrap_gateway_v2,
            ),
            Either::Right((_, _remaining_transport_future)) => {
                diagnostics.add_event(
                    request_handle,
                    RequestEvent::new(RequestEventType::TransportFailed)
                        .with_details("end-to-end operation timeout exceeded"),
                );
                diagnostics.timeout_request(request_handle);
                ExecutedTransportAttempt {
                    result: deadline_exceeded_result(RequestSentStatus::Unknown),
                    shard_id: dispatched_shard,
                    shard_diagnostics: None,
                }
            }
        };
    }

    let attempt_result = execute_http_attempt_future(
        http_request,
        transport,
        excluded_shard_id,
        endpoint_key,
        None,
    )
    .await;
    finalize_http_attempt(
        attempt_result,
        request_handle,
        diagnostics,
        should_unwrap_gateway_v2,
    )
}

async fn execute_http_attempt_future(
    http_request: &HttpRequest,
    transport: &AdaptiveTransport,
    excluded_shard_id: Option<u64>,
    endpoint_key: &EndpointKey,
    preferred_shard_id: Option<u64>,
) -> HttpAttemptResult {
    let dispatch = transport
        .send_with_dispatch(
            http_request,
            excluded_shard_id,
            endpoint_key,
            preferred_shard_id,
        )
        .await;

    match dispatch.result {
        Ok(response) => HttpAttemptResult::Response {
            status_code: azure_core::http::StatusCode::from(response.status),
            headers: response.headers,
            body: response.body,
            shard_id: dispatch.shard_id,
            shard_diagnostics: dispatch.shard_diagnostics,
        },
        Err(transport_err) => HttpAttemptResult::Error {
            error: transport_err.error,
            headers_received: transport_err.request_sent == RequestSentStatus::Sent,
            shard_id: dispatch.shard_id,
            shard_diagnostics: dispatch.shard_diagnostics,
        },
    }
}

fn finalize_http_attempt(
    attempt_result: HttpAttemptResult,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
    should_unwrap_gateway_v2: bool,
) -> ExecutedTransportAttempt {
    match attempt_result {
        HttpAttemptResult::Response {
            status_code,
            headers,
            body,
            shard_id,
            shard_diagnostics,
        } => {
            diagnostics.add_event(
                request_handle,
                RequestEvent::new(RequestEventType::ResponseHeadersReceived),
            );
            if let Some(shard_diagnostics) = shard_diagnostics.clone() {
                diagnostics.set_transport_shard(request_handle, shard_diagnostics);
            }

            let envelope_status_u16 = u16::from(status_code);
            // Thin-client (Gateway 2.0) proxies return both success and error responses
            // as an RNTBD frame in the HTTP body, with errors arriving as HTTP-error
            // envelopes (e.g. a 404 envelope wrapping a 404/1002 RNTBD frame). Unwrap
            // regardless of envelope status so the real Cosmos status, sub-status,
            // activity id, and session token are recovered rather than discarded.
            let (status_code, headers, body) = if should_unwrap_gateway_v2 {
                let envelope_was_success = (200..300).contains(&envelope_status_u16);
                // Preserve the originals only for the rare non-2xx fallback where the
                // body is a proxy-level error rather than an RNTBD frame.
                let fallback =
                    (!envelope_was_success).then(|| (status_code, headers.clone(), body.clone()));
                match unwrap_response_for_gateway_v2(super::cosmos_transport_client::HttpResponse {
                    status: envelope_status_u16,
                    headers,
                    body,
                }) {
                    Ok(response) => (
                        azure_core::http::StatusCode::from(response.status),
                        response.headers,
                        response.body,
                    ),
                    Err(error) => match fallback {
                        // Non-2xx envelope whose body is not an RNTBD frame: a genuine
                        // proxy-level error. Surface the envelope status unchanged.
                        Some(original) => original,
                        // A 2xx envelope must carry a valid RNTBD frame, so a parse
                        // failure here is a real protocol error.
                        None => {
                            let cosmos_err = crate::error::CosmosError::builder()
                                .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                                .with_message(format!(
                                    "Gateway 2.0 response unwrap failed: {error}"
                                ))
                                .with_source(error)
                                .build();
                            return ExecutedTransportAttempt {
                                result: gateway_v2_unwrap_error_result(
                                    cosmos_err,
                                    request_handle,
                                    diagnostics,
                                ),
                                shard_id,
                                shard_diagnostics,
                            };
                        }
                    },
                }
            } else {
                (status_code, headers, body)
            };

            ExecutedTransportAttempt {
                result: map_http_response_payload(
                    status_code,
                    headers,
                    body,
                    request_handle,
                    diagnostics,
                ),
                shard_id,
                shard_diagnostics,
            }
        }
        HttpAttemptResult::Error {
            error,
            headers_received,
            shard_id,
            shard_diagnostics,
        } => {
            if let Some(shard_diagnostics) = shard_diagnostics.clone() {
                diagnostics.set_transport_shard(request_handle, shard_diagnostics);
            }
            ExecutedTransportAttempt {
                result: transport_error_result(
                    error,
                    headers_received,
                    request_handle,
                    diagnostics,
                ),
                shard_id,
                shard_diagnostics,
            }
        }
    }
}

fn should_retry_connectivity_failure(
    result: &TransportResult,
    allow_sent_transport_retry: bool,
) -> bool {
    match &result.outcome {
        TransportOutcome::TransportError {
            error,
            request_sent,
            ..
        } => {
            is_connectivity_error(error)
                && (request_sent.definitely_not_sent() || allow_sent_transport_retry)
        }
        _ => false,
    }
}

fn is_connectivity_error(error: &crate::error::CosmosError) -> bool {
    // Transport / connectivity failures are synthetic errors (no wire
    // response) whose sub-status is one of the well-known transport
    // boundary-mapping codes minted by the SDK.
    if error.is_from_wire() {
        return false;
    }
    matches!(
        error.status().sub_status(),
        Some(SubStatusCode::TRANSPORT_GENERATED_503)
            | Some(SubStatusCode::TRANSPORT_CONNECTION_FAILED)
            | Some(SubStatusCode::TRANSPORT_IO_FAILED)
            | Some(SubStatusCode::TRANSPORT_DNS_FAILED)
            | Some(SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE)
            | Some(SubStatusCode::TRANSPORT_BODY_READ_FAILED)
            | Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
    )
}

fn gateway_v2_wrap_error_result(
    error: crate::error::CosmosError,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let status = CosmosStatus::CLIENT_BAD_REQUEST;
    let error_details = format_transport_error_details_cosmos(&error);
    diagnostics.fail_transport_request(
        request_handle,
        error_details,
        RequestSentStatus::NotSent,
        status,
    );

    TransportResult {
        outcome: TransportOutcome::TransportError {
            status,
            error,
            request_sent: RequestSentStatus::NotSent,
        },
    }
}

fn gateway_v2_unwrap_error_result(
    error: crate::error::CosmosError,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let status = CosmosStatus::TRANSPORT_GENERATED_503;
    let error_details = format_transport_error_details_cosmos(&error);
    diagnostics.add_event(
        request_handle,
        RequestEvent::new(RequestEventType::TransportFailed).with_details(error_details.clone()),
    );
    diagnostics.fail_transport_request(
        request_handle,
        error_details,
        RequestSentStatus::Sent,
        status,
    );

    TransportResult {
        outcome: TransportOutcome::TransportError {
            status,
            error,
            request_sent: RequestSentStatus::Sent,
        },
    }
}

fn transport_error_result(
    cosmos_error: crate::error::CosmosError,
    headers_received: bool,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let sent_status = if headers_received {
        RequestSentStatus::Sent
    } else {
        infer_request_sent_status(&cosmos_error)
    };
    let status = CosmosStatus::TRANSPORT_GENERATED_503;
    let error_details = format_transport_error_details_cosmos(&cosmos_error);

    if headers_received {
        diagnostics.add_event(
            request_handle,
            RequestEvent::new(RequestEventType::ResponseHeadersReceived),
        );
    }

    diagnostics.add_event(
        request_handle,
        RequestEvent::new(RequestEventType::TransportFailed).with_details(error_details.clone()),
    );
    diagnostics.fail_transport_request(request_handle, error_details, sent_status, status);

    TransportResult {
        outcome: TransportOutcome::TransportError {
            status,
            error: cosmos_error,
            request_sent: sent_status,
        },
    }
}

fn format_transport_error_details_cosmos(error: &crate::error::CosmosError) -> String {
    crate::driver::error_chain_summary(error)
}

enum HttpAttemptResult {
    Response {
        status_code: azure_core::http::StatusCode,
        headers: azure_core::http::headers::Headers,
        body: Vec<u8>,
        shard_id: Option<u64>,
        shard_diagnostics: Option<TransportShardDiagnostics>,
    },
    Error {
        error: crate::error::CosmosError,
        headers_received: bool,
        shard_id: Option<u64>,
        shard_diagnostics: Option<TransportShardDiagnostics>,
    },
}

struct ExecutedTransportAttempt {
    result: TransportResult,
    shard_id: Option<u64>,
    shard_diagnostics: Option<TransportShardDiagnostics>,
}

fn failed_transport_shard(
    attempt: &ExecutedTransportAttempt,
) -> Option<FailedTransportShardDiagnostics> {
    let transport_shard = attempt.shard_diagnostics.clone()?;
    match &attempt.result.outcome {
        TransportOutcome::TransportError {
            error,
            request_sent,
            ..
        } => Some(FailedTransportShardDiagnostics::new(
            transport_shard,
            *request_sent,
            // Surface just the underlying message — the [Kind] / status
            // prefix from the Cosmos Display is captured separately in
            // the request status.
            error.to_string(),
        )),
        _ => None,
    }
}

/// Maps an HTTP response payload to a `TransportResult`.
fn map_http_response_payload(
    status_code: azure_core::http::StatusCode,
    headers: azure_core::http::headers::Headers,
    body: Vec<u8>,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
    let cosmos_status = CosmosStatus::from_parts(status_code, cosmos_headers.substatus);

    diagnostics.record_response(request_handle, status_code, &cosmos_headers);
    TransportResult::from_http_response(cosmos_status, cosmos_headers, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
        time::Duration,
    };

    use async_trait::async_trait;

    use crate::{
        diagnostics::{DiagnosticsContextBuilder, RequestSentStatus},
        driver::{
            routing::CosmosEndpoint,
            transport::{
                adaptive_transport::AdaptiveTransport,
                cosmos_transport_client::{
                    HttpRequest, HttpResponse, TransportClient, TransportError,
                },
                http_client_factory::{HttpClientConfig, HttpClientFactory},
            },
        },
        models::{ActivityId, Credential, DefaultConsistencyLevel, OperationType, ResourceType},
        options::DiagnosticsOptions,
    };

    #[derive(Debug)]
    struct HangingTransportClient {
        delay: Duration,
    }

    #[async_trait]
    impl TransportClient for HangingTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            azure_core::sleep(
                azure_core::time::Duration::try_from(self.delay)
                    .unwrap_or(azure_core::time::Duration::ZERO),
            )
            .await;
            Err(TransportError::new(
                crate::error::CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
                    .with_message("request should have timed out before completion")
                    .build(),
                crate::diagnostics::RequestSentStatus::Unknown,
            ))
        }
    }

    fn make_throttled_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(azure_core::http::StatusCode::TooManyRequests),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_throttled_result_with_retry_after(ms: u64) -> TransportResult {
        let mut cosmos_headers = CosmosResponseHeaders::default();
        cosmos_headers.retry_after_ms = Some(ms);
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(azure_core::http::StatusCode::TooManyRequests),
                cosmos_headers,
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_throttled_result_with_substatus_and_retry_after(
        sub_status: SubStatusCode,
        ms: u64,
    ) -> TransportResult {
        let mut cosmos_headers = CosmosResponseHeaders::default();
        cosmos_headers.retry_after_ms = Some(ms);
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::from_parts(
                    azure_core::http::StatusCode::TooManyRequests,
                    Some(sub_status),
                ),
                cosmos_headers,
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_success_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::Success {
                status: CosmosStatus::new(azure_core::http::StatusCode::Ok),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
            },
        }
    }

    #[test]
    fn evaluate_transport_retry_429_uses_service_retry_after() {
        let result = make_throttled_result_with_retry_after(42);
        let state = ThrottleRetryState::new();

        match evaluate_transport_retry(&result, &state, false) {
            ThrottleAction::Retry { delay, new_state } => {
                assert_eq!(delay, Duration::from_millis(42));
                assert_eq!(new_state.attempt_count, 1);
            }
            ThrottleAction::Propagate => panic!("expected Retry"),
        }
    }

    #[test]
    fn evaluate_transport_retry_429_fallback_without_header() {
        let result = make_throttled_result();
        let state = ThrottleRetryState::new();

        match evaluate_transport_retry(&result, &state, false) {
            ThrottleAction::Retry { delay, new_state } => {
                // fallback base is 5ms with +/-25% jitter.
                assert!(delay >= Duration::from_nanos(3_750_000));
                assert!(delay <= Duration::from_nanos(6_250_000));
                assert_eq!(new_state.attempt_count, 1);
            }
            ThrottleAction::Propagate => panic!("expected Retry"),
        }
    }

    #[test]
    fn evaluate_transport_retry_429_caps_large_service_value() {
        // Service says wait 10s, but max_per_retry_delay is 5s
        let result = make_throttled_result_with_retry_after(10_000);
        let state = ThrottleRetryState::new();

        match evaluate_transport_retry(&result, &state, false) {
            ThrottleAction::Retry { delay, .. } => {
                assert_eq!(delay, Duration::from_secs(5)); // capped
            }
            ThrottleAction::Propagate => panic!("expected Retry"),
        }
    }

    #[test]
    fn evaluate_transport_retry_429_at_max_attempts() {
        let result = make_throttled_result();
        let state = ThrottleRetryState {
            attempt_count: 9,
            ..ThrottleRetryState::new()
        };

        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_429_disabled_when_max_attempts_zero() {
        // `max_retry_count = 0` (the analog of .NET's
        // MaxRetryAttemptsOnRateLimitedRequests = 0) must surface the first
        // 429 to the caller without any retry.
        let result = make_throttled_result_with_retry_after(42);
        let state = ThrottleRetryState::with_limits(0, Duration::from_secs(30));

        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_429_honors_custom_max_attempts() {
        // With a custom cap of 2, the third 429 (attempt_count == 2) stops.
        let result = make_throttled_result_with_retry_after(1);
        let max_wait = Duration::from_secs(30);

        // attempt_count 0 and 1 still retry.
        for attempt in 0..2 {
            let state = ThrottleRetryState {
                attempt_count: attempt,
                ..ThrottleRetryState::with_limits(2, max_wait)
            };
            assert!(
                matches!(
                    evaluate_transport_retry(&result, &state, false),
                    ThrottleAction::Retry { .. }
                ),
                "attempt {attempt} should retry under a cap of 2"
            );
        }

        // attempt_count 2 reaches the cap and propagates.
        let state = ThrottleRetryState {
            attempt_count: 2,
            ..ThrottleRetryState::with_limits(2, max_wait)
        };
        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_429_honors_custom_max_wait_time() {
        // A tight cumulative wait budget propagates once the next delay would
        // exceed it, mirroring .NET's MaxRetryWaitTimeOnRateLimitedRequests.
        let result = make_throttled_result_with_retry_after(2_000);
        let state = ThrottleRetryState {
            cumulative_delay: Duration::from_millis(500),
            ..ThrottleRetryState::with_limits(9, Duration::from_secs(1))
        };

        // 500ms accumulated + 2000ms next delay = 2.5s > 1s budget.
        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_429_exceeds_max_wait() {
        let result = make_throttled_result_with_retry_after(2_000);
        let state = ThrottleRetryState {
            attempt_count: 5,
            cumulative_delay: Duration::from_secs(29),
            ..ThrottleRetryState::new()
        };

        // cumulative = 29s + 2s = 31s; well above the 30s default max wait,
        // so the throttle classifier propagates rather than scheduling
        // another retry.
        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_non_429_propagates() {
        let result = make_success_result();
        let state = ThrottleRetryState::new();

        assert!(matches!(
            evaluate_transport_retry(&result, &state, false),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_dtx_bodyless_429_uses_shared_throttle() {
        let result = make_throttled_result_with_retry_after(42);
        let state = ThrottleRetryState::new();

        assert!(matches!(
            evaluate_transport_retry(&result, &state, true),
            ThrottleAction::Retry { .. }
        ));
    }

    #[test]
    fn evaluate_transport_retry_dtx_bodyless_429_ru_budget_uses_shared_throttle() {
        let result = make_throttled_result_with_substatus_and_retry_after(
            SubStatusCode::RU_BUDGET_EXCEEDED,
            42,
        );
        let state = ThrottleRetryState::new();

        match evaluate_transport_retry(&result, &state, true) {
            ThrottleAction::Retry { delay, .. } => {
                assert_eq!(delay, Duration::from_millis(42));
            }
            other => panic!("expected shared throttle retry, got {other:?}"),
        }
    }

    #[test]
    fn evaluate_transport_retry_dtx_body_bearing_429_propagates_to_outer_loop() {
        let mut result = make_throttled_result_with_retry_after(42);
        if let TransportOutcome::HttpError { body, .. } = &mut result.outcome {
            *body = br#"{"isRetriable":true}"#.to_vec();
        }
        let state = ThrottleRetryState::new();

        assert!(matches!(
            evaluate_transport_retry(&result, &state, true),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn deadline_capped_delay_uses_max_zero_when_remaining_below_margin() {
        let requested = Duration::from_millis(500);
        let remaining = Duration::from_millis(50);

        let capped = deadline_capped_delay(requested, remaining);
        assert_eq!(capped, Duration::ZERO);
    }

    #[test]
    fn deadline_capped_delay_caps_to_remaining_minus_margin() {
        let requested = Duration::from_secs(5);
        let remaining = Duration::from_millis(250);

        let capped = deadline_capped_delay(requested, remaining);
        assert_eq!(capped, Duration::from_millis(150));
    }

    #[test]
    fn forced_final_retry_delay_without_deadline_is_immediate() {
        let delay = forced_final_retry_delay(None);
        assert_eq!(delay, Some(Duration::ZERO));
    }

    #[test]
    fn forced_final_retry_delay_with_expired_deadline_is_none() {
        let delay = forced_final_retry_delay(Some(Instant::now() - Duration::from_millis(1)));
        assert_eq!(delay, None);
    }

    #[test]
    fn forced_final_retry_delay_under_margin_is_immediate() {
        let delay = forced_final_retry_delay_from_remaining(Duration::from_millis(50));
        assert_eq!(delay, Some(Duration::ZERO));
    }

    #[test]
    fn forced_final_retry_delay_when_half_remaining_below_margin_is_immediate() {
        let delay = forced_final_retry_delay_from_remaining(Duration::from_millis(150));
        assert_eq!(delay, Some(Duration::ZERO));
    }

    #[test]
    fn forced_final_retry_delay_uses_half_remaining() {
        let delay = forced_final_retry_delay_from_remaining(Duration::from_millis(400));
        assert_eq!(delay, Some(Duration::from_millis(200)));
    }

    #[test]
    fn remaining_request_timeout_has_minimum_of_one_millisecond() {
        let timeout = remaining_request_timeout(Some(Instant::now() - Duration::from_millis(1)))
            .expect("timeout should be present when deadline exists");
        assert_eq!(timeout, Duration::from_millis(1));
    }

    #[tokio::test]
    async fn execute_transport_pipeline_times_out_in_flight_request() {
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let request = TransportRequest {
            method: azure_core::http::Method::Get,
            endpoint: endpoint.clone(),
            transport_mode: TransportMode::Gateway,
            operation_type: OperationType::Read,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            url: endpoint.url().clone(),
            headers: azure_core::http::headers::Headers::new(),
            #[cfg(feature = "preview_dtx")]
            resource_type: ResourceType::Database,
            body: None,
            auth_context: super::super::AuthorizationContext::new(
                azure_core::http::Method::Get,
                ResourceType::Database,
                "",
            ),
            execution_context: ExecutionContext::Initial,
            deadline: Some(Instant::now() + Duration::from_millis(100)),
        };
        let client = AdaptiveTransport::Gateway(Arc::new(HangingTransportClient {
            delay: Duration::from_secs(2),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-timeout".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = execute_transport_pipeline(
            request,
            &TransportPipelineContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                pipeline_type: PipelineType::Metadata,
                transport_security: TransportSecurity::Secure,
                endpoint_key: endpoint.endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        assert!(matches!(
            result.outcome,
            TransportOutcome::DeadlineExceeded { .. }
        ));

        let completed = diagnostics.complete();
        let requests = completed.requests();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].timed_out());
    }

    /// Always returns an HTTP 429 response and counts how many times it was
    /// invoked. Used by the end-to-end `execute_transport_pipeline` tests that
    /// need to drive the throttle-retry loop without standing up a real
    /// service.
    #[derive(Debug)]
    struct AlwaysThrottlesTransportClient {
        request_count: Arc<std::sync::atomic::AtomicUsize>,
    }

    #[async_trait]
    impl TransportClient for AlwaysThrottlesTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.request_count
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(HttpResponse {
                status: 429,
                headers: azure_core::http::headers::Headers::new(),
                body: vec![],
            })
        }
    }

    /// End-to-end regression: `max_throttle_attempts = 0` must surface the
    /// first 429 to the caller with **exactly one** request on the wire,
    /// honoring the `MaxRetryAttemptsOnRateLimitedRequests = 0` .NET-parity
    /// contract.
    ///
    /// This test guards the full `execute_transport_pipeline` loop, including
    /// the one-shot `forced_final_retry` safety net which is suppressed when
    /// the user has explicitly opted out of throttle retries.
    /// `evaluate_transport_retry_429_disabled_when_max_attempts_zero` only
    /// covers the classifier; the `forced_final_retry` fires *after* the
    /// classifier returns `Propagate`, so it is invisible to the
    /// classifier-level test.
    #[tokio::test]
    async fn execute_transport_pipeline_with_zero_max_attempts_does_not_retry_429() {
        let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
            request_count: Arc::clone(&request_count),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("throttle-max-attempts-zero".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = execute_transport_pipeline(
            test_request(None),
            &TransportPipelineContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 0,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        // Exactly one request hit the wire — no throttle retry, no forced
        // final retry.
        assert_eq!(
            request_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "max_throttle_attempts=0 must surface the first 429 with no retry, but observed {} total transport requests",
            request_count.load(std::sync::atomic::Ordering::SeqCst),
        );

        // The 429 propagates as an HttpError so the caller can react to it.
        match result.outcome {
            TransportOutcome::HttpError { status, .. } => {
                assert!(
                    status.is_throttled(),
                    "expected 429/throttled outcome, got {:?}",
                    status,
                );
            }
            other => panic!("expected HttpError(429), got {other:?}"),
        }

        // Diagnostics record the single attempt.
        let completed = diagnostics.complete();
        assert_eq!(completed.requests().len(), 1);
    }

    /// End-to-end companion: with the default budget (≥ 1 attempt) the
    /// forced-final retry remains active. This pins the historical safety-net
    /// behavior so a future change to the gating logic that over-suppresses
    /// the forced-final retry would be caught by tests.
    #[tokio::test]
    async fn execute_transport_pipeline_with_default_attempts_uses_forced_final_retry() {
        let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
            request_count: Arc::clone(&request_count),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("throttle-forced-final".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        // A very tight cumulative-wait budget (1ms) so the regular throttle
        // loop bails out immediately, leaving only the forced-final-retry
        // path to consider.
        let _ = execute_transport_pipeline(
            test_request(None),
            &TransportPipelineContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_millis(1),
            },
            &mut diagnostics,
        )
        .await;

        // Initial request + one forced-final-retry attempt = 2 total.
        assert_eq!(
            request_count.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "default (≥1 attempt) configuration must permit the one-shot \
             forced-final retry, expected 2 transport requests, observed {}",
            request_count.load(std::sync::atomic::Ordering::SeqCst),
        );
    }

    /// End-to-end fault-injection regression: a transport that *always* throws
    /// 429 must be retried exactly `max_throttle_attempts` times before the
    /// throttle propagates to the caller, for **several** configured limits.
    ///
    /// This is the direct verification that the
    /// [`ThrottlingRetryOptionsView::max_retry_count`](crate::options::ThrottlingRetryOptionsView::max_retry_count)
    /// knob (surfaced here as
    /// [`TransportPipelineContext::max_throttle_attempts`]) is honored: with a
    /// generous cumulative-wait budget and no end-to-end deadline, the only
    /// limiter is the attempt count, so the total number of requests that hit
    /// the wire is deterministic.
    ///
    /// Wire-request accounting for `max_throttle_attempts = N` (where `N > 0`):
    ///
    /// * `1` initial attempt, plus
    /// * `N` throttle retries (the classifier keeps retrying while
    ///   `attempt_count < N`).
    ///
    /// Total = `N + 1`. Once the count budget is exhausted the one-shot
    /// `forced_final_retry` safety net is suppressed too, matching the
    /// .NET-parity `MaxRetryAttemptsOnRateLimitedRequests` semantic. The
    /// forced-final retry still fires when the *cumulative-wait* budget is
    /// the limiter (rather than the count), which is covered by
    /// [`execute_transport_pipeline_with_default_attempts_uses_forced_final_retry`].
    /// The `N = 0` opt-out (exactly one request, no forced-final retry) is
    /// covered by
    /// [`execute_transport_pipeline_with_zero_max_attempts_does_not_retry_429`].
    #[tokio::test]
    async fn execute_transport_pipeline_honors_configured_max_throttle_attempts() {
        for max_throttle_attempts in [1_u32, 2, 3, 5] {
            let request_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let client = AdaptiveTransport::Gateway(Arc::new(AlwaysThrottlesTransportClient {
                request_count: Arc::clone(&request_count),
            }));
            let mut diagnostics = DiagnosticsContextBuilder::new(
                ActivityId::from_string(format!("throttle-attempts-{max_throttle_attempts}")),
                Arc::new(DiagnosticsOptions::default()),
            );

            let result = execute_transport_pipeline(
                // No deadline so the cumulative-wait/deadline guards never cut
                // the loop short — the attempt count is the sole limiter.
                test_request(None),
                &TransportPipelineContext {
                    transport: &client,
                    allow_sent_transport_retry: false,
                    credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                    user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                    pipeline_type: PipelineType::DataPlane,
                    transport_security: TransportSecurity::Secure,
                    endpoint_key: test_endpoint_key(),
                    account_name: None,
                    collection_rid: None,
                    max_throttle_attempts,
                    // Generous budget so the cumulative-wait cap is never the
                    // limiter for these small attempt counts.
                    max_throttle_wait_time: Duration::from_secs(300),
                },
                &mut diagnostics,
            )
            .await;

            // 1 initial + N throttle retries = N + 1. The forced-final retry
            // is gated on `attempt_count < max_attempts`, so once the count
            // budget is exhausted the safety net does NOT fire too — matching
            // .NET's `MaxRetryAttemptsOnRateLimitedRequests` semantic.
            let expected = max_throttle_attempts as usize + 1;
            assert_eq!(
                request_count.load(std::sync::atomic::Ordering::SeqCst),
                expected,
                "max_throttle_attempts={max_throttle_attempts} must yield {expected} total \
                 transport requests (1 initial + {max_throttle_attempts} retries), but observed {}",
                request_count.load(std::sync::atomic::Ordering::SeqCst),
            );

            // After the budget is exhausted the 429 propagates to the caller.
            match result.outcome {
                TransportOutcome::HttpError { status, .. } => {
                    assert!(
                        status.is_throttled(),
                        "expected 429/throttled outcome for \
                         max_throttle_attempts={max_throttle_attempts}, got {status:?}",
                    );
                }
                other => panic!(
                    "expected HttpError(429) for max_throttle_attempts={max_throttle_attempts}, \
                     got {other:?}"
                ),
            }
        }
    }

    #[derive(Debug)]
    struct ScriptedTransportClient {
        status: CosmosStatus,
        message: &'static str,
    }

    #[async_trait]
    impl TransportClient for ScriptedTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            Err(TransportError::new(
                crate::error::CosmosError::builder()
                    .with_status(self.status)
                    .with_message(self.message)
                    .build(),
                crate::diagnostics::RequestSentStatus::Unknown,
            ))
        }
    }

    #[derive(Debug)]
    struct ScriptedFactory {
        clients: Mutex<Vec<Arc<dyn TransportClient>>>,
    }

    impl ScriptedFactory {
        fn new(clients: Vec<Arc<dyn TransportClient>>) -> Self {
            Self {
                clients: Mutex::new(clients.into_iter().rev().collect()),
            }
        }
    }

    impl HttpClientFactory for ScriptedFactory {
        fn build(
            &self,
            _connection_pool: &crate::options::ConnectionPoolOptions,
            _config: HttpClientConfig,
        ) -> crate::error::Result<Arc<dyn TransportClient>> {
            self.clients.lock().unwrap().pop().ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("no scripted client available")
                    .build()
            })
        }
    }

    fn scripted_transport(
        status_a: CosmosStatus,
        message_a: &'static str,
        status_b: CosmosStatus,
        message_b: &'static str,
    ) -> AdaptiveTransport {
        let pool = crate::options::ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(1)
            .with_min_http2_connections_per_endpoint(2)
            .with_max_http2_connections_per_endpoint(2)
            .build()
            .unwrap();
        let factory = Arc::new(ScriptedFactory::new(vec![
            Arc::new(ScriptedTransportClient {
                status: status_a,
                message: message_a,
            }),
            Arc::new(ScriptedTransportClient {
                status: status_b,
                message: message_b,
            }),
        ]));

        AdaptiveTransport::from_config(
            &pool,
            factory,
            HttpClientConfig::dataplane_gateway(
                &pool,
                crate::diagnostics::TransportHttpVersion::Http2,
            ),
        )
        .unwrap()
    }

    fn test_endpoint_key() -> EndpointKey {
        EndpointKey::try_from(&url::Url::parse("https://test.documents.azure.com:443/").unwrap())
            .unwrap()
    }

    fn test_request(deadline: Option<Instant>) -> TransportRequest {
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        TransportRequest {
            method: azure_core::http::Method::Get,
            endpoint: endpoint.clone(),
            transport_mode: TransportMode::Gateway,
            operation_type: OperationType::Read,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            url: endpoint.url().clone(),
            headers: azure_core::http::headers::Headers::new(),
            #[cfg(feature = "preview_dtx")]
            resource_type: ResourceType::Database,
            body: None,
            auth_context: super::super::AuthorizationContext::new(
                azure_core::http::Method::Get,
                ResourceType::Database,
                "",
            ),
            execution_context: ExecutionContext::Initial,
            deadline,
        }
    }

    /// Builds a minimal RNTBD response frame carrying `http_status` and a
    /// `SubStatus` token, used to emulate a thin-client error envelope body.
    fn gateway_v2_error_frame(http_status: u32, sub_status: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0_u32.to_le_bytes()); // total_len placeholder
        bytes.extend_from_slice(&http_status.to_le_bytes());
        bytes.extend_from_slice(&[0_u8; 16]); // nil activity id
        bytes.extend_from_slice(&0x001C_u16.to_le_bytes()); // SubStatus token id
        bytes.push(0x02); // ULong token type
        bytes.extend_from_slice(&sub_status.to_le_bytes());
        let total_len = u32::try_from(bytes.len()).unwrap();
        bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
        bytes
    }

    fn finalize_gateway_v2_response(
        status: azure_core::http::StatusCode,
        body: Vec<u8>,
    ) -> ExecutedTransportAttempt {
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("gw2-finalize".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );
        let request_handle = diagnostics.start_request(
            ExecutionContext::Initial,
            PipelineType::DataPlane,
            TransportSecurity::Secure,
            crate::diagnostics::TransportKind::GatewayV2,
            crate::diagnostics::TransportHttpVersion::Http2,
            &endpoint,
        );
        finalize_http_attempt(
            HttpAttemptResult::Response {
                status_code: status,
                headers: azure_core::http::headers::Headers::new(),
                body,
                shard_id: None,
                shard_diagnostics: None,
            },
            request_handle,
            &mut diagnostics,
            true,
        )
    }

    /// Regression: a Gateway 2.0 error response arrives as an HTTP-error
    /// envelope (404) wrapping an RNTBD `404/1002` frame. The pipeline must
    /// unwrap the body and surface the real sub-status rather than a bare 404.
    #[test]
    fn finalize_unwraps_gateway_v2_error_envelope() {
        let executed = finalize_gateway_v2_response(
            azure_core::http::StatusCode::NotFound,
            gateway_v2_error_frame(404, 1002),
        );

        match executed.result.outcome {
            TransportOutcome::HttpError { status, .. } => {
                assert_eq!(
                    status.status_code(),
                    azure_core::http::StatusCode::NotFound,
                    "expected the unwrapped 404 status, got {status:?}"
                );
                assert_eq!(
                    status.sub_status().map(|s| s.value()),
                    Some(1002),
                    "expected sub-status 1002 recovered from the RNTBD body, got {status:?}"
                );
            }
            other => panic!("expected HttpError(404/1002), got {other:?}"),
        }
    }

    /// A non-2xx thin-client envelope whose body is *not* an RNTBD frame (a
    /// genuine proxy-level error) must fall back to the bare envelope status
    /// instead of being masked as a transport failure.
    #[test]
    fn finalize_falls_back_on_non_rntbd_gateway_v2_error_envelope() {
        let executed = finalize_gateway_v2_response(
            azure_core::http::StatusCode::Unauthorized,
            b"unauthorized".to_vec(),
        );

        match executed.result.outcome {
            TransportOutcome::HttpError { status, .. } => {
                assert_eq!(
                    status.status_code(),
                    azure_core::http::StatusCode::Unauthorized,
                    "expected the bare envelope 401 to be preserved, got {status:?}"
                );
            }
            other => panic!("expected HttpError(401), got {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_transport_pipeline_retries_not_sent_connectivity_error_on_different_shard() {
        let client = scripted_transport(
            CosmosStatus::TRANSPORT_CONNECTION_FAILED,
            "first shard failed",
            CosmosStatus::TRANSPORT_CONNECTION_FAILED,
            "second shard failed",
        );
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-retry-not-sent".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = execute_transport_pipeline(
            test_request(Some(Instant::now() + Duration::from_secs(2))),
            &TransportPipelineContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::TransportError { error, .. } => {
                assert!(error.to_string().contains("second shard failed"));
            }
            other => panic!("expected transport error, got {other:?}"),
        }

        let diagnostics = diagnostics.complete();
        let requests = diagnostics.requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[1].local_shard_retry_count(), 1);
        assert_eq!(requests[1].failed_transport_shards().len(), 1);
        let recorded = requests[1].failed_transport_shards()[0].error();
        assert!(
            recorded.ends_with("first shard failed"),
            "unexpected: {recorded}"
        );
    }

    #[tokio::test]
    async fn execute_transport_pipeline_only_retries_unknown_connectivity_error_when_allowed() {
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");

        let client_without_retry = scripted_transport(
            CosmosStatus::TRANSPORT_IO_FAILED,
            "first io shard failed",
            CosmosStatus::TRANSPORT_IO_FAILED,
            "second io shard failed",
        );
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-retry-io-disabled".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );
        let result_without_retry = execute_transport_pipeline(
            test_request(Some(Instant::now() + Duration::from_secs(2))),
            &TransportPipelineContext {
                transport: &client_without_retry,
                allow_sent_transport_retry: false,
                credential: &credential,
                user_agent: &user_agent,
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        match result_without_retry.outcome {
            TransportOutcome::TransportError {
                error,
                request_sent,
                ..
            } => {
                assert!(error.to_string().contains("first io shard failed"));
                assert_eq!(request_sent, RequestSentStatus::Unknown);
            }
            other => panic!("expected transport error, got {other:?}"),
        }

        let client_with_retry = scripted_transport(
            CosmosStatus::TRANSPORT_IO_FAILED,
            "first io shard failed",
            CosmosStatus::TRANSPORT_IO_FAILED,
            "second io shard failed",
        );
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-retry-io-enabled".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );
        let result_with_retry = execute_transport_pipeline(
            test_request(Some(Instant::now() + Duration::from_secs(2))),
            &TransportPipelineContext {
                transport: &client_with_retry,
                allow_sent_transport_retry: true,
                credential: &credential,
                user_agent: &user_agent,
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        match result_with_retry.outcome {
            TransportOutcome::TransportError { error, .. } => {
                assert!(error.to_string().contains("second io shard failed"));
            }
            other => panic!("expected transport error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_transport_pipeline_preserves_client_unauthorized_in_diagnostics() {
        let client = AdaptiveTransport::Gateway(Arc::new(HangingTransportClient {
            delay: Duration::from_secs(1),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-signing-failure".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = execute_transport_pipeline(
            test_request(Some(Instant::now() + Duration::from_secs(1))),
            &TransportPipelineContext {
                transport: &client,
                allow_sent_transport_retry: false,
                credential: &Credential::from(azure_core::credentials::Secret::new(
                    "***not-base64***",
                )),
                user_agent: &azure_core::http::headers::HeaderValue::from_static("test-agent"),
                pipeline_type: PipelineType::DataPlane,
                transport_security: TransportSecurity::Secure,
                endpoint_key: test_endpoint_key(),
                account_name: None,
                collection_rid: None,
                max_throttle_attempts: 9,
                max_throttle_wait_time: Duration::from_secs(30),
            },
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::TransportError {
                status,
                request_sent,
                ..
            } => {
                assert_eq!(status, CosmosStatus::CLIENT_GENERATED_401);
                assert_eq!(
                    status.sub_status(),
                    Some(SubStatusCode::CLIENT_GENERATED_401)
                );
                assert_eq!(request_sent, RequestSentStatus::NotSent);
            }
            other => panic!("expected transport error, got {other:?}"),
        }

        let completed = diagnostics.complete();
        let requests = completed.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].status(), &CosmosStatus::CLIENT_GENERATED_401);
        assert_eq!(requests[0].request_sent(), RequestSentStatus::NotSent);
    }

    #[derive(Debug)]
    struct GatewayV2MockTransportClient {
        responses: Mutex<VecDeque<HttpResponse>>,
        requests: Mutex<Vec<HttpRequest>>,
    }

    impl GatewayV2MockTransportClient {
        fn new(responses: Vec<HttpResponse>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
                requests: Mutex::new(Vec::new()),
            }
        }

        fn requests(&self) -> Vec<HttpRequest> {
            self.requests.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl TransportClient for GatewayV2MockTransportClient {
        async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.requests.lock().unwrap().push(request.clone());
            self.responses.lock().unwrap().pop_front().ok_or_else(|| {
                TransportError::new(
                    crate::error::CosmosError::builder()
                        .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
                        .with_message("no response queued")
                        .build(),
                    RequestSentStatus::Unknown,
                )
            })
        }
    }

    const GATEWAY_V2_ACTIVITY_ID: &str = "00112233-4455-6677-8899-aabbccddeeff";

    fn gateway_v2_transport_request(transport_mode: TransportMode) -> TransportRequest {
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test-thin.documents.azure.com:444/").unwrap(),
        );
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert("x-ms-activity-id", GATEWAY_V2_ACTIVITY_ID);
        TransportRequest {
            method: azure_core::http::Method::Get,
            endpoint: endpoint.clone(),
            transport_mode,
            operation_type: OperationType::Read,
            effective_partition_key: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            url: endpoint.url().clone(),
            headers,
            #[cfg(feature = "preview_dtx")]
            resource_type: ResourceType::Document,
            body: None,
            auth_context: super::super::AuthorizationContext::new(
                azure_core::http::Method::Get,
                ResourceType::Document,
                "dbs/db1/colls/coll1/docs/doc1",
            ),
            execution_context: ExecutionContext::Initial,
            deadline: None,
        }
    }

    fn gateway_v2_context<'a>(
        client: &'a AdaptiveTransport,
        endpoint_key: EndpointKey,
        account_name: Option<String>,
        credential: &'a Credential,
        user_agent: &'a azure_core::http::headers::HeaderValue,
    ) -> TransportPipelineContext<'a> {
        TransportPipelineContext {
            transport: client,
            allow_sent_transport_retry: false,
            credential,
            user_agent,
            pipeline_type: PipelineType::DataPlane,
            transport_security: TransportSecurity::Secure,
            endpoint_key,
            account_name,
            collection_rid: None,
            max_throttle_attempts: 9,
            max_throttle_wait_time: Duration::from_secs(30),
        }
    }

    fn gateway_v2_diagnostics() -> DiagnosticsContextBuilder {
        DiagnosticsContextBuilder::new(
            ActivityId::from_string(GATEWAY_V2_ACTIVITY_ID.to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        )
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_wraps_request_and_unwraps_success_response() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
            gateway_v2_response(200, |_| {}, b"{}"),
        ]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(TransportMode::GatewayV2);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(
                &client,
                endpoint_key,
                Some("account".to_owned()),
                &credential,
                &user_agent,
            ),
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::Success { status, body, .. } => {
                assert_eq!(status.status_code(), azure_core::http::StatusCode::Ok);
                assert_eq!(body, b"{}".to_vec());
            }
            other => panic!("expected success, got {other:?}"),
        }
        let captured = mock.requests();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].method, azure_core::http::Method::Post);
        assert_eq!(
            captured[0]
                .headers
                .get_optional_str(&azure_core::http::headers::AUTHORIZATION),
            None
        );
        assert_eq!(
            captured[0]
                .headers
                .get_optional_str(&azure_core::http::headers::USER_AGENT),
            Some("test-agent")
        );
        assert!(captured[0]
            .body
            .as_ref()
            .is_some_and(|body| !body.is_empty()));
        // Gateway 2.0 never carries the V1-only no-retry-449 header.
        assert_eq!(
            captured[0].headers.get_optional_str(&NO_RETRY_449),
            None,
            "Gateway 2.0 must not send x-ms-noretry-449"
        );
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_leaves_standard_gateway_request_unwrapped() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: b"plain".to_vec(),
        }]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(TransportMode::Gateway);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(&client, endpoint_key, None, &credential, &user_agent),
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::Success { body, .. } => assert_eq!(body, b"plain".to_vec()),
            other => panic!("expected success, got {other:?}"),
        }
        let captured = mock.requests();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0].method, azure_core::http::Method::Get);
        assert!(captured[0]
            .headers
            .get_optional_str(&azure_core::http::headers::AUTHORIZATION)
            .is_some());
        // Gateway V1 disables CGW's server-side 449 retry so the SDK owns
        // RetryWith, matching Java's RxGatewayStoreModel.
        assert_eq!(
            captured[0].headers.get_optional_str(&NO_RETRY_449),
            Some("true"),
            "Gateway V1 must send x-ms-noretry-449: true"
        );
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_decode_failure_is_sent_transport_error() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: vec![1, 2, 3],
        }]));
        let client = AdaptiveTransport::Gateway(mock);
        let request = gateway_v2_transport_request(TransportMode::GatewayV2);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(
                &client,
                endpoint_key,
                Some("account".to_owned()),
                &credential,
                &user_agent,
            ),
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::TransportError {
                status,
                request_sent,
                ..
            } => {
                assert_eq!(status, CosmosStatus::TRANSPORT_GENERATED_503);
                assert_eq!(request_sent, RequestSentStatus::Sent);
            }
            other => panic!("expected transport error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_outer_502_propagates_unchanged_without_unwrap() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![HttpResponse {
            status: 502,
            headers: azure_core::http::headers::Headers::new(),
            body: vec![],
        }]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(TransportMode::GatewayV2);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(
                &client,
                endpoint_key,
                Some("account".to_owned()),
                &credential,
                &user_agent,
            ),
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::HttpError {
                status,
                body,
                request_sent,
                ..
            } => {
                assert_eq!(u16::from(status.status_code()), 502);
                assert_eq!(status.sub_status(), None);
                assert_eq!(body, Vec::<u8>::new());
                assert_eq!(request_sent, RequestSentStatus::Sent);
            }
            other => panic!("expected HTTP error, got {other:?}"),
        }
        assert_eq!(mock.requests().len(), 1);
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_inner_401_surfaces_as_inner_status() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
            gateway_v2_response(401, |_| {}, b""),
        ]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(TransportMode::GatewayV2);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(
                &client,
                endpoint_key,
                Some("account".to_owned()),
                &credential,
                &user_agent,
            ),
            &mut diagnostics,
        )
        .await;

        match result.outcome {
            TransportOutcome::HttpError {
                status,
                body,
                request_sent,
                ..
            } => {
                assert_eq!(
                    status.status_code(),
                    azure_core::http::StatusCode::Unauthorized
                );
                assert_eq!(status.sub_status(), None);
                assert_eq!(body, Vec::<u8>::new());
                assert_eq!(request_sent, RequestSentStatus::Sent);
            }
            other => panic!("expected HTTP error, got {other:?}"),
        }
        assert_eq!(mock.requests().len(), 1);
    }

    #[tokio::test]
    async fn gateway_v2_pipeline_uses_inner_retry_after_for_throttle_retry() {
        let mock = Arc::new(GatewayV2MockTransportClient::new(vec![
            gateway_v2_response(
                429,
                |bytes| write_gateway_v2_u32_token(bytes, 0x000C, 0),
                b"",
            ),
            gateway_v2_response(200, |_| {}, b"{}"),
        ]));
        let client = AdaptiveTransport::Gateway(mock.clone());
        let request = gateway_v2_transport_request(TransportMode::GatewayV2);
        let endpoint_key = request.endpoint.endpoint_key();
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
        let mut diagnostics = gateway_v2_diagnostics();

        let result = execute_transport_pipeline(
            request,
            &gateway_v2_context(
                &client,
                endpoint_key,
                Some("account".to_owned()),
                &credential,
                &user_agent,
            ),
            &mut diagnostics,
        )
        .await;

        assert!(matches!(result.outcome, TransportOutcome::Success { .. }));
        assert_eq!(mock.requests().len(), 2);
    }

    fn gateway_v2_response(
        status: u32,
        write_tokens: impl FnOnce(&mut Vec<u8>),
        body: &[u8],
    ) -> HttpResponse {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&status.to_le_bytes());
        write_gateway_v2_uuid(
            &mut bytes,
            uuid::Uuid::parse_str(GATEWAY_V2_ACTIVITY_ID).unwrap(),
        );
        write_tokens(&mut bytes);
        if !body.is_empty() {
            // PayloadPresent = true (id 0x0000, type Byte).
            bytes.extend_from_slice(&0x0000_u16.to_le_bytes());
            bytes.push(0x00);
            bytes.push(1);
        }
        let total_len = u32::try_from(bytes.len()).unwrap();
        bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
        if !body.is_empty() {
            bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
            bytes.extend_from_slice(body);
        }
        HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: bytes,
        }
    }

    fn write_gateway_v2_u32_token(bytes: &mut Vec<u8>, id: u16, value: u32) {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(0x02);
        bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn write_gateway_v2_uuid(bytes: &mut Vec<u8>, value: uuid::Uuid) {
        let value = value.as_u128();
        bytes.extend_from_slice(&((value >> 64) as u64).to_le_bytes());
        bytes.extend_from_slice(&(value as u64).to_le_bytes());
    }

    #[test]
    fn format_transport_error_details_includes_error_chain() {
        let inner = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "socket reset");
        let cosmos = crate::error::CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
            .with_message("failed to execute `reqwest` request")
            .with_source(inner)
            .build();

        let details = format_transport_error_details_cosmos(&cosmos);
        assert!(details.contains("failed to execute `reqwest` request"));
        assert!(details.contains("socket reset"));
    }
}
