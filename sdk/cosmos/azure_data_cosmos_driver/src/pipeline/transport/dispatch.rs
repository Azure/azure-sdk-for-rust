// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sends prepared requests and classifies HTTP, transport, decoding, and timeout outcomes.

use super::{
    adaptive_transport::AdaptiveTransport,
    cosmos_transport_client::HttpResponse,
    diagnostics, gateway_v2,
    request::PreparedRequest,
    sent_status,
    sharded_transport::TransportDispatch,
    types::{TransportMode, TransportOutcome, TransportResult},
    EndpointKey,
};
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, FailedTransportShardDiagnostics, RequestHandle,
        RequestSentStatus, TransportShardDiagnostics,
    },
    error::{status_codes, CosmosError},
    models::{CosmosResponseHeaders, CosmosStatus},
    timeout,
};

/// The classified outcome of sending one prepared request.
pub(super) enum DispatchOutcome {
    Success(TransportResult),
    HttpError(DispatchFailure),
    TransportFailure {
        failure: DispatchFailure,
        headers_received: bool,
    },
    ResponseDecodingFailure(DispatchFailure),
    Timeout(DispatchFailure),
}

/// A failed result and the shard information needed for same-endpoint recovery.
pub(super) struct DispatchFailure {
    pub result: TransportResult,
    pub shard_id: Option<u64>,
    pub shard_diagnostics: Option<TransportShardDiagnostics>,
}

impl DispatchOutcome {
    /// Borrows the result without discarding its dispatch classification.
    // Inline the small enum projection.
    #[inline(always)]
    pub fn result(&self) -> &TransportResult {
        match self {
            Self::Success(result) => result,
            Self::HttpError(failure)
            | Self::TransportFailure { failure, .. }
            | Self::ResponseDecodingFailure(failure)
            | Self::Timeout(failure) => &failure.result,
        }
    }

    /// Returns whether a transport error occurred after response headers arrived.
    // Inline the small enum predicate.
    #[inline(always)]
    pub fn failed_after_headers(&self) -> bool {
        matches!(
            self,
            Self::TransportFailure {
                headers_received: true,
                ..
            }
        )
    }
}

impl From<DispatchOutcome> for TransportResult {
    // Inline the small enum projection.
    #[inline(always)]
    fn from(outcome: DispatchOutcome) -> Self {
        match outcome {
            DispatchOutcome::Success(result) => result,
            DispatchOutcome::HttpError(failure)
            | DispatchOutcome::TransportFailure { failure, .. }
            | DispatchOutcome::ResponseDecodingFailure(failure)
            | DispatchOutcome::Timeout(failure) => failure.result,
        }
    }
}

impl DispatchFailure {
    /// Copies shard and error details for connectivity-retry diagnostics, if both are available.
    pub fn failed_shard(&self) -> Option<FailedTransportShardDiagnostics> {
        let shard = self.shard_diagnostics.clone()?;
        match &self.result.outcome {
            TransportOutcome::TransportError {
                error,
                request_sent,
                ..
            } => Some(FailedTransportShardDiagnostics::new(
                shard,
                *request_sent,
                error.to_string(),
            )),
            _ => None,
        }
    }
}

/// Sends the prepared request and classifies its response or failure.
///
/// Records raw headers and shard observations before decoding. A timeout drops the send future;
/// shard preselection is used only when there is a deadline.
pub(super) async fn send(
    prepared: &PreparedRequest,
    transport: &AdaptiveTransport,
    excluded_shard_id: Option<u64>,
    endpoint_key: &EndpointKey,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> DispatchOutcome {
    let request = &prepared.http_request;
    let dispatch = if let Some(duration) = prepared.timeout {
        let shard_hint = transport.pre_select_shard(excluded_shard_id, endpoint_key);
        let future =
            transport.send_with_dispatch(request, excluded_shard_id, endpoint_key, shard_hint);
        match timeout::with_timeout(duration, future).await {
            Ok(dispatch) => dispatch,
            Err(timeout::TimedOut) => {
                return DispatchOutcome::Timeout(DispatchFailure {
                    result: TransportResult::deadline_exceeded(RequestSentStatus::Unknown),
                    shard_id: shard_hint,
                    shard_diagnostics: None,
                });
            }
        }
    } else {
        transport
            .send_with_dispatch(request, excluded_shard_id, endpoint_key, None)
            .await
    };
    diagnostics::record_dispatch(&dispatch, handle, diagnostics);
    classify(dispatch, prepared.mode)
}

/// Decodes a completed send and preserves the failure kind and shard data needed by its callers.
pub(super) fn classify(dispatch: TransportDispatch, mode: TransportMode) -> DispatchOutcome {
    match dispatch.result {
        Ok(response) => match decode(response, mode) {
            Ok(response) => {
                let result = response_result(response);
                if result.outcome.is_successful() {
                    DispatchOutcome::Success(result)
                } else {
                    DispatchOutcome::HttpError(DispatchFailure {
                        result,
                        shard_id: dispatch.shard_id,
                        shard_diagnostics: dispatch.shard_diagnostics,
                    })
                }
            }
            Err(error) => DispatchOutcome::ResponseDecodingFailure(DispatchFailure {
                result: TransportResult {
                    outcome: TransportOutcome::TransportError {
                        status: status_codes::TRANSPORT_GENERATED_503,
                        error,
                        request_sent: RequestSentStatus::Sent,
                    },
                },
                shard_id: dispatch.shard_id,
                shard_diagnostics: dispatch.shard_diagnostics,
            }),
        },
        Err(error) => {
            let headers_received = error.request_sent == RequestSentStatus::Sent;
            let request_sent = if headers_received {
                RequestSentStatus::Sent
            } else {
                sent_status::infer_request_sent_status(&error.error)
            };
            DispatchOutcome::TransportFailure {
                failure: DispatchFailure {
                    result: TransportResult {
                        outcome: TransportOutcome::TransportError {
                            status: status_codes::TRANSPORT_GENERATED_503,
                            error: error.error,
                            request_sent,
                        },
                    },
                    shard_id: dispatch.shard_id,
                    shard_diagnostics: dispatch.shard_diagnostics,
                },
                headers_received,
            }
        }
    }
}

/// Unwraps Gateway V2 frames, preserving invalid non-2xx envelopes but rejecting invalid 2xx frames.
fn decode(response: HttpResponse, mode: TransportMode) -> Result<HttpResponse, CosmosError> {
    if !mode.is_gateway_v2() {
        return Ok(response);
    }
    // Proxy errors need not contain a frame; successful envelopes must contain one.
    let fallback = (!(200..300).contains(&response.status)).then(|| response.clone());
    match gateway_v2::unwrap_response_for_gateway_v2(response) {
        Ok(response) => Ok(response),
        Err(error) => match fallback {
            Some(response) => Ok(response),
            None => Err(CosmosError::builder()
                .with_status(status_codes::TRANSPORT_GENERATED_503)
                .with_message(format!("Gateway 2.0 response unwrap failed: {error}"))
                .with_source(error)
                .build()),
        },
    }
}

/// Parses Cosmos response headers and combines them with the status and body into a transport result.
// Inline result assembly; header parsing retains its call frame.
#[inline]
fn response_result(response: HttpResponse) -> TransportResult {
    let headers = CosmosResponseHeaders::from_headers(&response.headers);
    let status = CosmosStatus::from_parts(response.status.into(), headers.substatus);
    TransportResult::from_http_response(status, headers, response.body)
}

#[cfg(test)]
mod tests {
    use super::{
        classify, status_codes, CosmosError, DispatchOutcome, HttpResponse, RequestSentStatus,
        TransportMode, TransportOutcome,
    };
    use crate::pipeline::transport::{
        cosmos_transport_client::TransportError, sharded_transport::TransportDispatch,
    };

    #[test]
    fn response_classification_distinguishes_http_and_decoding_failures() {
        for mode in [TransportMode::Gateway, TransportMode::GatewayV2] {
            for status in [200, 429, 502] {
                let outcome = classify(
                    TransportDispatch {
                        result: Ok(HttpResponse {
                            status,
                            headers: Default::default(),
                            body: b"not an RNTBD frame".to_vec(),
                        }),
                        shard_id: Some(7),
                        shard_diagnostics: None,
                    },
                    mode,
                );
                match (mode, status, outcome) {
                    (TransportMode::Gateway, 200, DispatchOutcome::Success(result)) => {
                        assert!(result.outcome.is_successful());
                    }
                    (
                        TransportMode::GatewayV2,
                        200,
                        DispatchOutcome::ResponseDecodingFailure(failure),
                    ) => {
                        assert_eq!(failure.shard_id, Some(7));
                        assert!(matches!(
                            failure.result.outcome,
                            TransportOutcome::TransportError {
                                status: status_codes::TRANSPORT_GENERATED_503,
                                request_sent: RequestSentStatus::Sent,
                                ..
                            }
                        ));
                    }
                    (_, 429 | 502, DispatchOutcome::HttpError(failure)) => {
                        assert_eq!(failure.shard_id, Some(7));
                        let TransportOutcome::HttpError {
                            status: actual,
                            body,
                            request_sent,
                            ..
                        } = failure.result.outcome
                        else {
                            panic!("expected an HTTP error");
                        };
                        assert_eq!(u16::from(actual.status_code()), status);
                        assert_eq!(body, b"not an RNTBD frame");
                        assert_eq!(request_sent, RequestSentStatus::Sent);
                    }
                    _ => panic!("unexpected dispatch classification for {mode:?}, HTTP {status}"),
                }
            }
        }
    }

    #[test]
    fn raw_sent_status_and_underlying_error_are_mapped_independently() {
        for raw in [
            RequestSentStatus::NotSent,
            RequestSentStatus::Unknown,
            RequestSentStatus::Sent,
        ] {
            for (inner, inferred) in [
                (
                    status_codes::TRANSPORT_CONNECTION_FAILED,
                    RequestSentStatus::NotSent,
                ),
                (
                    status_codes::TRANSPORT_IO_FAILED,
                    RequestSentStatus::Unknown,
                ),
            ] {
                let attempt = classify(
                    TransportDispatch {
                        result: Err(TransportError::new(
                            CosmosError::builder().with_status(inner).build(),
                            raw,
                        )),
                        shard_id: Some(7),
                        shard_diagnostics: None,
                    },
                    TransportMode::Gateway,
                );
                assert_eq!(
                    attempt.failed_after_headers(),
                    raw == RequestSentStatus::Sent
                );
                let DispatchOutcome::TransportFailure { failure, .. } = attempt else {
                    panic!("expected a transport failure");
                };
                let TransportOutcome::TransportError {
                    status,
                    error,
                    request_sent,
                } = failure.result.outcome
                else {
                    panic!("expected a transport failure");
                };
                assert_eq!(status, status_codes::TRANSPORT_GENERATED_503);
                assert_eq!(error.status(), inner);
                assert_eq!(
                    request_sent,
                    if raw == RequestSentStatus::Sent {
                        raw
                    } else {
                        inferred
                    }
                );
                assert_eq!(failure.shard_id, Some(7));
            }
        }
    }
}
