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

use azure_core::error::ErrorKind;
use futures::{future::Either, pin_mut};
use tracing::trace;

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, FailedTransportShardDiagnostics, PipelineType,
        RequestEvent, RequestEventType, RequestHandle, RequestSentStatus, TransportSecurity,
        TransportShardDiagnostics,
    },
    models::{CosmosResponseHeaders, CosmosStatus, Credential},
};

use super::{
    adaptive_transport::AdaptiveTransport, cosmos_headers::apply_cosmos_headers,
    cosmos_transport_client::HttpRequest, infer_request_sent_status, request_signing::sign_request,
    sharded_transport::EndpointKey,
};

use crate::driver::pipeline::components::{
    ThrottleAction, ThrottleRetryState, TransportOutcome, TransportRequest, TransportResult,
};

/// Cosmos DB retry-after header (milliseconds).
const RETRY_AFTER_MS: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-retry-after-ms");

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
) -> ThrottleAction {
    let is_throttled = match &result.outcome {
        TransportOutcome::HttpError { status, .. } => status.is_throttled(),
        _ => false,
    };

    if !is_throttled {
        return ThrottleAction::Propagate;
    }

    if throttle_state.attempt_count >= throttle_state.max_attempts {
        return ThrottleAction::Propagate;
    }

    // Extract the service-specified retry delay from response headers,
    // or fall back to exponential backoff.
    let service_delay = result
        .response_headers()
        .and_then(|h| h.get_optional_str(&RETRY_AFTER_MS))
        .and_then(|v| v.parse::<u64>().ok())
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
}

/// Executes a single transport attempt.
///
/// Applies headers, signs the request, sends it via the selected transport, and
/// handles 429 throttle retry internally. Returns a `TransportResult` to the
/// operation pipeline for higher-level decision making.
///
/// This is the core transport loop described in §5.2 of the spec.
pub(crate) async fn execute_transport_pipeline(
    request: TransportRequest,
    ctx: &TransportPipelineContext<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let mut throttle_state = ThrottleRetryState::new();
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

        // Sign the request
        if let Err(e) = sign_request(&mut http_request, ctx.credential, &request.auth_context).await
        {
            diagnostics.fail_transport_request(
                request_handle,
                e.to_string(),
                RequestSentStatus::NotSent,
                CosmosStatus::CLIENT_GENERATED_401,
            );
            return TransportResult {
                outcome: TransportOutcome::TransportError {
                    status: CosmosStatus::CLIENT_GENERATED_401,
                    error: e,
                    request_sent: RequestSentStatus::NotSent,
                },
            };
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

        let result = execute_http_attempt(
            &http_request,
            ctx.transport,
            per_request_timeout,
            request_handle,
            diagnostics,
            excluded_shard_id.take(),
            endpoint_key,
        )
        .await;

        #[cfg(feature = "fault_injection")]
        if let Some(collector) = evaluation_collector.take() {
            let evals = collector.take();
            if !evals.is_empty() {
                diagnostics.set_fault_injection_evaluations(request_handle, evals);
            }
        }
        tracing::debug!("transport request complete");

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

        let result = result.result;

        // Check for 429 throttling → transport-level retry
        let action = evaluate_transport_retry(&result, &throttle_state);
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
                let is_throttled = matches!(
                    &result.outcome,
                    TransportOutcome::HttpError { status, .. } if status.is_throttled()
                );

                if throttle_state.can_use_forced_final_retry() && is_throttled {
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

async fn execute_http_attempt(
    http_request: &HttpRequest,
    transport: &AdaptiveTransport,
    per_request_timeout: Option<Duration>,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
    excluded_shard_id: Option<u64>,
    endpoint_key: &EndpointKey,
) -> ExecutedTransportAttempt {
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
            Either::Left((attempt_result, _)) => {
                finalize_http_attempt(attempt_result, request_handle, diagnostics)
            }
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
    finalize_http_attempt(attempt_result, request_handle, diagnostics)
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

fn is_connectivity_error(error: &azure_core::Error) -> bool {
    matches!(error.kind(), ErrorKind::Connection | ErrorKind::Io)
}

fn format_transport_error_details(error: &azure_core::Error) -> String {
    crate::driver::error_chain_summary(error)
}

fn transport_error_result(
    error: azure_core::Error,
    headers_received: bool,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let sent_status = if headers_received {
        RequestSentStatus::Sent
    } else {
        infer_request_sent_status(&error)
    };
    let status = CosmosStatus::TRANSPORT_GENERATED_503;
    let error_details = format_transport_error_details(&error);

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
            error,
            request_sent: sent_status,
        },
    }
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
        error: azure_core::Error,
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
    let sub_status = cosmos_headers.substatus;
    let cosmos_status = CosmosStatus::from_parts(status_code, sub_status);

    // Update diagnostics with response metadata
    diagnostics.update_request(request_handle, |req| {
        if let Some(charge) = cosmos_headers.request_charge {
            req.with_charge(charge);
        }
        if let Some(activity_id) = cosmos_headers.activity_id.clone() {
            req.with_activity_id(activity_id);
        }
        if let Some(token) = cosmos_headers.session_token.clone() {
            req.with_session_token(token.to_string());
        }
        if let Some(duration) = cosmos_headers.server_duration_ms {
            req.with_server_duration_ms(duration);
        }
    });

    diagnostics.complete_request(request_handle, status_code, sub_status);
    TransportResult::from_http_response(cosmos_status, headers, cosmos_headers, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use async_trait::async_trait;

    use crate::{
        diagnostics::DiagnosticsContextBuilder,
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
        models::{ActivityId, Credential, ResourceType},
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
                azure_core::Error::new(
                    azure_core::error::ErrorKind::Io,
                    "request should have timed out before completion",
                ),
                crate::diagnostics::RequestSentStatus::Unknown,
            ))
        }
    }

    fn make_throttled_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(azure_core::http::StatusCode::TooManyRequests),
                headers: azure_core::http::headers::Headers::new(),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_throttled_result_with_retry_after(ms: u64) -> TransportResult {
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert("x-ms-retry-after-ms", ms.to_string());
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(azure_core::http::StatusCode::TooManyRequests),
                headers,
                cosmos_headers: CosmosResponseHeaders::default(),
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

        match evaluate_transport_retry(&result, &state) {
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

        match evaluate_transport_retry(&result, &state) {
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

        match evaluate_transport_retry(&result, &state) {
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
            evaluate_transport_retry(&result, &state),
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

        // cumulative = 29s + 2s = 31s > 30s max
        assert!(matches!(
            evaluate_transport_retry(&result, &state),
            ThrottleAction::Propagate
        ));
    }

    #[test]
    fn evaluate_transport_retry_non_429_propagates() {
        let result = make_success_result();
        let state = ThrottleRetryState::new();

        assert!(matches!(
            evaluate_transport_retry(&result, &state),
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
            url: endpoint.url().clone(),
            headers: azure_core::http::headers::Headers::new(),
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

    #[derive(Debug)]
    struct ScriptedTransportClient {
        error_kind: azure_core::error::ErrorKind,
        message: &'static str,
    }

    #[async_trait]
    impl TransportClient for ScriptedTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            let error_kind = match &self.error_kind {
                ErrorKind::Connection => ErrorKind::Connection,
                ErrorKind::Io => ErrorKind::Io,
                ErrorKind::Other => ErrorKind::Other,
                _ => ErrorKind::Other,
            };
            Err(TransportError::new(
                azure_core::Error::with_message(error_kind, self.message),
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
        ) -> azure_core::Result<Arc<dyn TransportClient>> {
            self.clients.lock().unwrap().pop().ok_or_else(|| {
                azure_core::Error::with_message(ErrorKind::Other, "no scripted client available")
            })
        }
    }

    fn scripted_transport(
        error_kind_a: azure_core::error::ErrorKind,
        message_a: &'static str,
        error_kind_b: azure_core::error::ErrorKind,
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
                error_kind: error_kind_a,
                message: message_a,
            }),
            Arc::new(ScriptedTransportClient {
                error_kind: error_kind_b,
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
            url: endpoint.url().clone(),
            headers: azure_core::http::headers::Headers::new(),
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

    #[tokio::test]
    async fn execute_transport_pipeline_retries_not_sent_connectivity_error_on_different_shard() {
        let client = scripted_transport(
            ErrorKind::Connection,
            "first shard failed",
            ErrorKind::Connection,
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
        assert_eq!(
            requests[1].failed_transport_shards()[0].error(),
            "first shard failed"
        );
    }

    #[tokio::test]
    async fn execute_transport_pipeline_only_retries_unknown_connectivity_error_when_allowed() {
        let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");

        let client_without_retry = scripted_transport(
            ErrorKind::Io,
            "first io shard failed",
            ErrorKind::Io,
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
            ErrorKind::Io,
            "first io shard failed",
            ErrorKind::Io,
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
    async fn execute_transport_pipeline_preserves_client_generated_401_in_diagnostics() {
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

    #[test]
    fn format_transport_error_details_includes_error_chain() {
        let inner = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "socket reset");
        let error = azure_core::Error::with_error(
            ErrorKind::Io,
            inner,
            "failed to execute `reqwest` request",
        );

        let details = format_transport_error_details(&error);
        assert!(details.contains("failed to execute `reqwest` request"));
        assert!(details.contains("socket reset"));
    }
}
