// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport pipeline: bare functions for executing a single HTTP attempt.
//!
//! This replaces the policy chain (`HeadersPolicy` → `AuthorizationPolicy` →
//! `TrackedTransportPolicy`) with direct function calls. The pipeline handles:
//!
//! - Applying standard Cosmos headers (`x-ms-version`, `Content-Type`, etc.)
//! - Generating and attaching the authorization header
//! - 429 throttle retry with exponential backoff
//! - Request-sent-status tracking for retry safety
//! - Per-attempt diagnostics event recording
//! - End-to-end deadline enforcement

use std::time::Instant;

use azure_core::http::{
    headers::{HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Request,
};
use azure_core::time::{self, OffsetDateTime};
use tracing::trace;

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineType, RequestEvent, RequestEventType,
        RequestHandle, TransportSecurity,
    },
    models::{CosmosResponseHeaders, CosmosStatus, Credential},
};

use super::{
    generate_authorization, infer_request_sent_status, AuthorizationContext,
    RequestSentStatus as TransportRequestSentStatus, COSMOS_API_VERSION,
};

use crate::driver::pipeline::components::{
    ThrottleAction, ThrottleRetryState, TransportOutcome, TransportRequest, TransportResult,
};

// Re-export for convenience within this module.
use crate::diagnostics::RequestSentStatus;

/// Maps transport-level `RequestSentStatus` to diagnostics-level `RequestSentStatus`.
fn map_sent_status(status: TransportRequestSentStatus) -> RequestSentStatus {
    match status {
        TransportRequestSentStatus::Sent => RequestSentStatus::Sent,
        TransportRequestSentStatus::NotSent => RequestSentStatus::NotSent,
        TransportRequestSentStatus::Unknown => RequestSentStatus::Unknown,
    }
}

// ── Header constants (same values as CosmosHeadersPolicy) ──────────────

const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
const SDK_SUPPORTED_CAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");
const SUPPORTED_CAPABILITIES_VALUE: &str = "0";
const CACHE_CONTROL: HeaderName = HeaderName::from_static("cache-control");
const NO_CACHE: HeaderValue = HeaderValue::from_static("no-cache");
const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");

// ── Bare functions ─────────────────────────────────────────────────────

/// Applies standard Cosmos DB headers to an outgoing HTTP request.
///
/// Sets `x-ms-version`, `x-ms-cosmos-sdk-supportedcapabilities`, `Content-Type`,
/// `Accept`, `Cache-Control`, and `User-Agent`.
pub(crate) fn apply_cosmos_headers(request: &mut Request, user_agent: &str) {
    let headers = request.headers_mut();

    headers.insert(VERSION, HeaderValue::from_static(COSMOS_API_VERSION));
    headers.insert(
        SDK_SUPPORTED_CAPABILITIES,
        HeaderValue::from_static(SUPPORTED_CAPABILITIES_VALUE),
    );

    if headers.get_optional_str(&CONTENT_TYPE).is_none() {
        headers.insert(CONTENT_TYPE, APPLICATION_JSON.clone());
    }

    headers.insert(ACCEPT, APPLICATION_JSON.clone());
    headers.insert(CACHE_CONTROL, NO_CACHE.clone());
    headers.insert(USER_AGENT, HeaderValue::from(user_agent.to_owned()));
}

/// Generates and attaches the Authorization header to an HTTP request.
///
/// Computes the HMAC-SHA256 signature (master key) or obtains an AAD token,
/// then sets both `x-ms-date` and `Authorization` headers.
pub(crate) async fn sign_request(
    request: &mut Request,
    credential: &Credential,
    auth_context: &AuthorizationContext,
) -> azure_core::Result<()> {
    let date_string = time::to_rfc7231(&OffsetDateTime::now_utc()).to_lowercase();

    let auth = generate_authorization(credential, auth_context, &date_string).await?;

    request.insert_header(MS_DATE, HeaderValue::from(date_string));
    request.insert_header(AUTHORIZATION, HeaderValue::from(auth));

    Ok(())
}

/// Decides whether to retry a 429 throttling response at the transport level.
///
/// Pure function: checks whether the result is a 429, whether the retry budget
/// allows another attempt, and computes the appropriate backoff delay.
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

    let delay = throttle_state.current_delay();
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
                return TransportResult {
                    outcome: TransportOutcome::TransportError {
                        error: azure_core::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "end-to-end operation timeout exceeded",
                        ),
                        request_sent: RequestSentStatus::NotSent,
                    },
                };
            }
        }

        // Record this attempt in diagnostics
        let execution_context = if throttle_state.attempt_count == 0 {
            request.execution_context
        } else {
            ExecutionContext::Retry
        };

        let endpoint_string = request.url.as_str().to_owned();
        let region = None; // Step 1: single-region, no region tracking in transport
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
                let transport_sent_status = infer_request_sent_status(&error);
                let sent_status = map_sent_status(transport_sent_status);
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
                azure_core::sleep(
                    azure_core::time::Duration::try_from(delay)
                        .unwrap_or(azure_core::time::Duration::ZERO),
                )
                .await;
                throttle_state = new_state;
                continue;
            }
            ThrottleAction::Propagate => return result,
        }
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

    // Read the body by converting to a raw response
    let raw_response = response.try_into_raw_response().await;
    let body = match &raw_response {
        Ok(raw) => raw.body().to_vec(),
        Err(_) => vec![],
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
    fn evaluate_transport_retry_429_within_budget() {
        let result = make_throttled_result();
        let state = ThrottleRetryState::new();

        match evaluate_transport_retry(&result, &state) {
            ThrottleAction::Retry { delay, new_state } => {
                assert_eq!(delay, Duration::from_secs(1)); // 1s * 2^0
                assert_eq!(new_state.attempt_count, 1);
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
        let result = make_throttled_result();
        let state = ThrottleRetryState {
            attempt_count: 5,
            cumulative_delay: Duration::from_secs(29),
            ..ThrottleRetryState::new()
        };

        // attempt 5: delay = 1s * 2^5 = 32s, cumulative = 29 + 32 = 61s > 30s
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
