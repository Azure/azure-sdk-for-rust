// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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

use azure_core::http::Request;
use futures::{future::Either, pin_mut};
use tracing::trace;

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineType, RequestEvent, RequestEventType,
        RequestHandle, RequestSentStatus, TransportSecurity,
    },
    models::{CosmosResponseHeaders, CosmosStatus, Credential},
};

use super::{
    adaptive_transport::AdaptiveTransport, cosmos_headers::apply_cosmos_headers,
    infer_request_sent_status, request_signing::sign_request,
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

/// Executes a single transport attempt.
///
/// Applies headers, signs the request, sends it via the `HttpClient`, and
/// handles 429 throttle retry internally. Returns a `TransportResult` to the
/// operation pipeline for higher-level decision making.
///
/// This is the core transport loop described in §5.2 of the spec.
pub(crate) async fn execute_transport_pipeline(
    request: TransportRequest,
    transport: &AdaptiveTransport,
    credential: &Credential,
    user_agent: &azure_core::http::headers::HeaderValue,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let mut throttle_state = ThrottleRetryState::new();

    loop {
        // Check deadline before each attempt
        if let Some(deadline) = request.deadline {
            if Instant::now() >= deadline {
                trace!("transport pipeline: deadline exceeded before attempt");
                return deadline_exceeded_result(RequestSentStatus::NotSent);
            }
        }

        // Record this attempt in diagnostics
        let execution_context = if throttle_state.attempt_count == 0 {
            request.execution_context
        } else {
            ExecutionContext::Retry
        };

        let request_handle = diagnostics.start_request(
            execution_context,
            pipeline_type,
            transport_security,
            &request.endpoint,
        );

        // Build HTTP request from TransportRequest
        let mut http_request = Request::new(request.url.clone(), request.method);

        // Copy headers from TransportRequest
        for (name, value) in request.headers.iter() {
            http_request.insert_header(name.clone(), value.clone());
        }

        // Set body
        if let Some(body) = &request.body {
            http_request.set_body(body.clone());
        }

        let per_request_timeout = remaining_request_timeout(request.deadline);
        // TODO(azure_core): Apply per-request timeout directly on Request/HttpClient
        // once azure_core/typespec_client_core exposes timeout options.
        // Tracking issue: https://github.com/Azure/azure-sdk-for-rust/issues/3878
        trace!(
            ?per_request_timeout,
            "transport pipeline: computed per-request timeout"
        );

        // Apply standard Cosmos headers
        apply_cosmos_headers(&mut http_request, user_agent);

        // Sign the request
        if let Err(e) = sign_request(&mut http_request, credential, &request.auth_context).await {
            diagnostics.fail_request(request_handle, e.to_string(), RequestSentStatus::NotSent);
            return TransportResult {
                outcome: TransportOutcome::TransportError {
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

        let result = execute_http_attempt(
            &http_request,
            transport,
            per_request_timeout,
            request_handle,
            diagnostics,
        )
        .await;

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
    http_request: &Request,
    transport: &AdaptiveTransport,
    per_request_timeout: Option<Duration>,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    if let Some(timeout_duration) = per_request_timeout {
        let transport_future = execute_http_attempt_future(http_request, transport);
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
                deadline_exceeded_result(RequestSentStatus::Unknown)
            }
        };
    }

    let attempt_result = execute_http_attempt_future(http_request, transport).await;
    finalize_http_attempt(attempt_result, request_handle, diagnostics)
}

async fn execute_http_attempt_future(
    http_request: &Request,
    transport: &AdaptiveTransport,
) -> HttpAttemptResult {
    match transport.send(http_request).await {
        Ok(response) => {
            let status_code = response.status();
            let headers = response.headers().clone();
            match response.try_into_raw_response().await {
                Ok(raw) => HttpAttemptResult::Response {
                    status_code,
                    headers,
                    body: raw.body().to_vec(),
                },
                Err(error) => HttpAttemptResult::Error {
                    error,
                    headers_received: true,
                },
            }
        }
        Err(error) => HttpAttemptResult::Error {
            error,
            headers_received: false,
        },
    }
}

fn finalize_http_attempt(
    attempt_result: HttpAttemptResult,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    match attempt_result {
        HttpAttemptResult::Response {
            status_code,
            headers,
            body,
        } => {
            diagnostics.add_event(
                request_handle,
                RequestEvent::new(RequestEventType::ResponseHeadersReceived),
            );
            map_http_response_payload(status_code, headers, body, request_handle, diagnostics)
        }
        HttpAttemptResult::Error {
            error,
            headers_received,
        } => transport_error_result(error, headers_received, request_handle, diagnostics),
    }
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

    if headers_received {
        diagnostics.add_event(
            request_handle,
            RequestEvent::new(RequestEventType::ResponseHeadersReceived),
        );
    }

    diagnostics.add_event(
        request_handle,
        RequestEvent::new(RequestEventType::TransportFailed).with_details(error.to_string()),
    );
    diagnostics.fail_request(request_handle, error.to_string(), sent_status);

    TransportResult {
        outcome: TransportOutcome::TransportError {
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
    },
    Error {
        error: azure_core::Error,
        headers_received: bool,
    },
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
    });

    diagnostics.complete_request(request_handle, status_code, sub_status);
    TransportResult::from_http_response(cosmos_status, headers, body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, time::Duration};

    use async_trait::async_trait;
    use azure_core::http::{AsyncRawResponse, Request};

    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        driver::{routing::CosmosEndpoint, transport::adaptive_transport::AdaptiveTransport},
        models::{ActivityId, Credential, ResourceType},
        options::DiagnosticsOptions,
    };

    #[derive(Debug)]
    struct HangingHttpClient {
        delay: Duration,
    }

    #[async_trait]
    impl azure_core::http::HttpClient for HangingHttpClient {
        async fn execute_request(
            &self,
            _request: &Request,
        ) -> azure_core::Result<AsyncRawResponse> {
            tokio::time::sleep(self.delay).await;
            Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Io,
                "request should have timed out before completion",
            ))
        }
    }

    fn make_throttled_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(azure_core::http::StatusCode::TooManyRequests),
                headers: azure_core::http::headers::Headers::new(),
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
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_success_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::Success {
                status: CosmosStatus::new(azure_core::http::StatusCode::Ok),
                headers: azure_core::http::headers::Headers::new(),
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
        let client = AdaptiveTransport::Http2Preferred(Arc::new(HangingHttpClient {
            delay: Duration::from_secs(2),
        }));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("transport-timeout".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );

        let result = execute_transport_pipeline(
            request,
            &client,
            &Credential::from(azure_core::credentials::Secret::new("dGVzdA==")),
            &azure_core::http::headers::HeaderValue::from_static("test-agent"),
            PipelineType::Metadata,
            TransportSecurity::Secure,
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
}
