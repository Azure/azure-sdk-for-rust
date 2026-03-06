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
use tracing::trace;

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineType, RequestEvent, RequestEventType,
        RequestHandle, RequestSentStatus, TransportSecurity,
    },
    models::{CosmosResponseHeaders, CosmosStatus, Credential},
};

use super::{
    cosmos_headers::apply_cosmos_headers, infer_request_sent_status, request_signing::sign_request,
};

use crate::driver::pipeline::components::{
    ThrottleAction, ThrottleRetryState, TransportOutcome, TransportRequest, TransportResult,
};

/// Cosmos DB retry-after header (milliseconds).
const RETRY_AFTER_MS: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-retry-after-ms");

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
    http_client: &dyn azure_core::http::HttpClient,
    credential: &Credential,
    user_agent: &str,
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
                return deadline_exceeded_result();
            }
        }

        // Record this attempt in diagnostics
        let execution_context = if throttle_state.attempt_count == 0 {
            request.execution_context
        } else {
            ExecutionContext::Retry
        };

        let endpoint_string = request.url.as_str().to_owned();
        let region = request.region.clone();
        let request_handle = diagnostics.start_request(
            execution_context,
            pipeline_type,
            transport_security,
            region,
            endpoint_string,
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

        // Send the HTTP request
        let http_result = http_client.execute_request(&http_request).await;

        // Map the result to TransportResult
        let result = match http_result {
            Ok(response) => {
                diagnostics.add_event(
                    request_handle,
                    RequestEvent::new(RequestEventType::ResponseHeadersReceived),
                );

                map_http_response(response, request_handle, diagnostics).await
            }
            Err(error) => {
                let sent_status = infer_request_sent_status(&error);
                diagnostics.add_event(
                    request_handle,
                    RequestEvent::new(RequestEventType::TransportFailed)
                        .with_details(error.to_string()),
                );
                diagnostics.fail_request(request_handle, error.to_string(), sent_status);

                TransportResult {
                    outcome: TransportOutcome::TransportError {
                        error,
                        request_sent: sent_status,
                    },
                }
            }
        };

        // Check for 429 throttling → transport-level retry
        let action = evaluate_transport_retry(&result, &throttle_state);
        match action {
            ThrottleAction::Retry { delay, new_state } => {
                // Never sleep past the end-to-end deadline. If there is no remaining
                // budget, fail fast instead of delaying.
                let mut effective_delay = delay;
                if let Some(deadline) = request.deadline {
                    let now = Instant::now();
                    if now >= deadline {
                        return deadline_exceeded_result();
                    }

                    let remaining = deadline.saturating_duration_since(now);
                    if remaining.is_zero() {
                        return deadline_exceeded_result();
                    }
                    effective_delay = effective_delay.min(remaining);
                }

                if effective_delay.is_zero() {
                    return deadline_exceeded_result();
                }

                azure_core::sleep(
                    azure_core::time::Duration::try_from(effective_delay)
                        .unwrap_or(azure_core::time::Duration::ZERO),
                )
                .await;

                if let Some(deadline) = request.deadline {
                    if Instant::now() >= deadline {
                        return deadline_exceeded_result();
                    }
                }

                throttle_state = new_state;
                continue;
            }
            ThrottleAction::Propagate => return result,
        }
    }
}

fn deadline_exceeded_result() -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::TransportError {
            error: azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                "end-to-end operation timeout exceeded",
            ),
            request_sent: RequestSentStatus::NotSent,
        },
    }
}

/// Maps an HTTP response to a `TransportResult`.
async fn map_http_response(
    response: azure_core::http::response::AsyncRawResponse,
    request_handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let status_code = response.status();
    let headers = response.headers().clone();

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

    // Read the body by converting to a raw response.
    // If conversion fails, this is a transport failure (truncated/invalid response stream),
    // not a successful HTTP response with an empty payload.
    let body = match response.try_into_raw_response().await {
        Ok(raw) => raw.body().to_vec(),
        Err(error) => {
            diagnostics.add_event(
                request_handle,
                RequestEvent::new(RequestEventType::TransportFailed)
                    .with_details(error.to_string()),
            );
            diagnostics.fail_request(
                request_handle,
                error.to_string(),
                RequestSentStatus::Unknown,
            );
            return TransportResult {
                outcome: TransportOutcome::TransportError {
                    error,
                    request_sent: RequestSentStatus::Unknown,
                },
            };
        }
    };

    if cosmos_status.is_success() {
        diagnostics.complete_request(request_handle, status_code, sub_status);

        TransportResult {
            outcome: TransportOutcome::Success {
                status: cosmos_status,
                headers,
                body,
            },
        }
    } else {
        diagnostics.complete_request(request_handle, status_code, sub_status);

        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: cosmos_status,
                headers,
                body,
                request_sent: RequestSentStatus::Sent,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

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
                // fallback: 5ms * 2^0 = 5ms
                assert_eq!(delay, Duration::from_millis(5));
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
}
