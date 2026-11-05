// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Classifies wire outcomes without mutating diagnostics or retry state.

use super::{
    cosmos_transport_client::HttpResponse,
    dispatch::DispatchOutcome,
    gateway_v2,
    request_preparation::PreparationFailure,
    sent_status,
    types::{TransportMode, TransportOutcome, TransportResult},
};
use crate::{
    diagnostics::{FailedTransportShardDiagnostics, RequestSentStatus, TransportShardDiagnostics},
    error::{status_codes, CosmosError},
    models::{CosmosResponseHeaders, CosmosStatus},
};

#[derive(Clone, Copy)]
pub(super) enum CompletionKind {
    Response,
    AuthorizationFailure,
    RequestEncodingFailure,
    TransportFailure { headers_received: bool },
    ResponseDecodingFailure,
    Timeout,
}

pub(super) struct CompletedTransportAttempt {
    pub result: TransportResult,
    pub shard_id: Option<u64>,
    pub shard_diagnostics: Option<TransportShardDiagnostics>,
    pub completion: CompletionKind,
}

/// Converts signing or encoding failure into an unsent result and its diagnostic completion kind.
pub(super) fn classify_preparation_failure(
    failure: PreparationFailure,
) -> CompletedTransportAttempt {
    let (error, status, completion) = match failure {
        PreparationFailure::Authorization(error) => (
            error,
            status_codes::CLIENT_GENERATED_401,
            CompletionKind::AuthorizationFailure,
        ),
        PreparationFailure::Encoding(error) => (
            error,
            status_codes::CLIENT_BAD_REQUEST,
            CompletionKind::RequestEncodingFailure,
        ),
    };
    CompletedTransportAttempt {
        result: TransportResult {
            outcome: TransportOutcome::TransportError {
                status,
                error,
                request_sent: RequestSentStatus::NotSent,
            },
        },
        shard_id: None,
        shard_diagnostics: None,
        completion,
    }
}

/// Converts dispatch or timeout data into a result, shard metadata, and diagnostic completion kind.
pub(super) fn classify_dispatch(
    outcome: DispatchOutcome,
    mode: TransportMode,
) -> CompletedTransportAttempt {
    let dispatch = match outcome {
        DispatchOutcome::TimedOut { shard_hint } => {
            return CompletedTransportAttempt {
                result: TransportResult::deadline_exceeded(RequestSentStatus::Unknown),
                shard_id: shard_hint,
                shard_diagnostics: None,
                completion: CompletionKind::Timeout,
            }
        }
        DispatchOutcome::Completed(dispatch) => dispatch,
    };
    let (result, completion) = match dispatch.result {
        Ok(response) => match decode_response(response, mode) {
            Ok(response) => (map_http_response(response), CompletionKind::Response),
            Err(error) => (
                TransportResult {
                    outcome: TransportOutcome::TransportError {
                        status: status_codes::TRANSPORT_GENERATED_503,
                        error,
                        request_sent: RequestSentStatus::Sent,
                    },
                },
                CompletionKind::ResponseDecodingFailure,
            ),
        },
        Err(error) => {
            let headers_received = error.request_sent == RequestSentStatus::Sent;
            let request_sent = if headers_received {
                RequestSentStatus::Sent
            } else {
                sent_status::infer_request_sent_status(&error.error)
            };
            (
                TransportResult {
                    outcome: TransportOutcome::TransportError {
                        status: status_codes::TRANSPORT_GENERATED_503,
                        error: error.error,
                        request_sent,
                    },
                },
                CompletionKind::TransportFailure { headers_received },
            )
        }
    };
    CompletedTransportAttempt {
        result,
        shard_id: dispatch.shard_id,
        shard_diagnostics: dispatch.shard_diagnostics,
        completion,
    }
}

/// Unwraps Gateway V2 frames, preserving invalid non-2xx envelopes but rejecting invalid 2xx frames.
fn decode_response(
    response: HttpResponse,
    mode: TransportMode,
) -> Result<HttpResponse, CosmosError> {
    if mode != TransportMode::GatewayV2 {
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
fn map_http_response(response: HttpResponse) -> TransportResult {
    let headers = CosmosResponseHeaders::from_headers(&response.headers);
    let status = CosmosStatus::from_parts(response.status.into(), headers.substatus);
    TransportResult::from_http_response(status, headers, response.body)
}

/// Copies shard and error details from a transport failure, or returns `None` when either is absent.
pub(super) fn failed_transport_shard(
    attempt: &CompletedTransportAttempt,
) -> Option<FailedTransportShardDiagnostics> {
    let shard = attempt.shard_diagnostics.clone()?;
    match &attempt.result.outcome {
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

#[cfg(test)]
mod tests {
    use super::{
        classify_dispatch, status_codes, CompletionKind, CosmosError, DispatchOutcome,
        RequestSentStatus, TransportMode, TransportOutcome,
    };
    use crate::pipeline::transport::{
        cosmos_transport_client::TransportError, sharded_transport::TransportDispatch,
    };

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
                let attempt = classify_dispatch(
                    DispatchOutcome::Completed(TransportDispatch {
                        result: Err(TransportError::new(
                            CosmosError::builder().with_status(inner).build(),
                            raw,
                        )),
                        shard_id: Some(7),
                        shard_diagnostics: None,
                    }),
                    TransportMode::Gateway,
                );
                let TransportOutcome::TransportError {
                    status,
                    error,
                    request_sent,
                } = attempt.result.outcome
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
                assert_eq!(attempt.shard_id, Some(7));
                assert!(
                    matches!(attempt.completion, CompletionKind::TransportFailure { headers_received } if headers_received == (raw == RequestSentStatus::Sent))
                );
            }
        }
    }
}
