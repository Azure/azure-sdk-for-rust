// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sends prepared requests and classifies HTTP, transport, decoding, and timeout outcomes.

use super::{
    adaptive_transport::AdaptiveTransport,
    cosmos_transport_client::HttpResponse,
    diagnostics, gateway_v2,
    request::PreparedRequest,
    sent_status,
    types::{TransportMode, TransportOutcome, TransportResult},
    EndpointKey,
};
use crate::{
    clock::Clock,
    diagnostics::{
        FailedTransportShardDiagnostics, RequestDiagnosticsBuilder, RequestSentStatus,
        TransportShardDiagnostics,
    },
    error::{status_codes, CosmosError},
    models::{CosmosResponseHeaders, CosmosStatus},
    timeout,
};

/// Identifies why dispatch failed and retains the result and shard data needed for recovery.
#[derive(Debug)]
pub(super) enum DispatchError {
    Http(DispatchErrorDetails),
    Transport {
        details: DispatchErrorDetails,
        headers_received: bool,
    },
    ResponseDecoding(DispatchErrorDetails),
    Timeout(DispatchErrorDetails),
}

/// A failed result and the shard information needed for same-endpoint recovery.
#[derive(Debug)]
pub(super) struct DispatchErrorDetails {
    pub result: TransportResult,
    pub shard_id: Option<u64>,
    pub shard_diagnostics: Option<TransportShardDiagnostics>,
}

impl DispatchError {
    /// Borrows the result without discarding its dispatch classification.
    #[inline(always)]
    pub fn result(&self) -> &TransportResult {
        match self {
            Self::Http(details)
            | Self::Transport { details, .. }
            | Self::ResponseDecoding(details)
            | Self::Timeout(details) => &details.result,
        }
    }

    /// Returns whether a transport error occurred after response headers arrived.
    #[inline(always)]
    pub fn failed_after_headers(&self) -> bool {
        matches!(
            self,
            Self::Transport {
                headers_received: true,
                ..
            }
        )
    }
}

impl From<DispatchError> for DispatchErrorDetails {
    #[inline(always)]
    fn from(error: DispatchError) -> Self {
        match error {
            DispatchError::Http(details)
            | DispatchError::Transport { details, .. }
            | DispatchError::ResponseDecoding(details)
            | DispatchError::Timeout(details) => details,
        }
    }
}

impl DispatchErrorDetails {
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

/// Sends the prepared request, returning a successful transport result or a classified error.
///
/// Records raw headers and shard observations before decoding. A timeout drops the send future;
/// shard preselection is used only when there is a deadline.
#[allow(
    clippy::result_large_err,
    reason = "Success also carries a large response; keep existing error data inline without adding allocations."
)]
pub(super) async fn send(
    prepared: &PreparedRequest,
    transport: &AdaptiveTransport,
    excluded_shard_id: Option<u64>,
    endpoint_key: &EndpointKey,
    diagnostics: &mut RequestDiagnosticsBuilder<'_>,
    clock: Clock,
) -> Result<TransportResult, DispatchError> {
    let request = &prepared.http_request;
    let dispatch = if let Some(duration) = prepared.timeout {
        let shard_hint = transport.pre_select_shard(excluded_shard_id, endpoint_key);
        let future =
            transport.send_with_dispatch(request, excluded_shard_id, endpoint_key, shard_hint);
        match timeout::with_timeout(duration, future, clock).await {
            Ok(dispatch) => dispatch,
            Err(timeout::TimedOut) => {
                return Err(DispatchError::Timeout(DispatchErrorDetails {
                    result: TransportResult::deadline_exceeded(RequestSentStatus::Unknown),
                    shard_id: shard_hint,
                    shard_diagnostics: None,
                }));
            }
        }
    } else {
        transport
            .send_with_dispatch(request, excluded_shard_id, endpoint_key, None)
            .await
    };
    diagnostics::record_dispatch(&dispatch, diagnostics);
    match dispatch.result {
        Ok(response) => match decode(response, prepared.mode) {
            Ok(response) => {
                let headers = CosmosResponseHeaders::from_headers(&response.headers);
                let status = CosmosStatus::from_parts(response.status.into(), headers.substatus);
                let result = TransportResult::from_http_response(status, headers, response.body);
                if result.outcome.is_successful() {
                    Ok(result)
                } else {
                    Err(DispatchError::Http(DispatchErrorDetails {
                        result,
                        shard_id: dispatch.shard_id,
                        shard_diagnostics: dispatch.shard_diagnostics,
                    }))
                }
            }
            Err(error) => Err(DispatchError::ResponseDecoding(DispatchErrorDetails {
                result: TransportResult {
                    outcome: TransportOutcome::TransportError {
                        status: status_codes::TRANSPORT_GENERATED_503,
                        error,
                        request_sent: RequestSentStatus::Sent,
                    },
                },
                shard_id: dispatch.shard_id,
                shard_diagnostics: dispatch.shard_diagnostics,
            })),
        },
        Err(error) => {
            let headers_received = error.request_sent == RequestSentStatus::Sent;
            let request_sent = if headers_received {
                RequestSentStatus::Sent
            } else {
                sent_status::infer_request_sent_status(&error.error)
            };
            Err(DispatchError::Transport {
                details: DispatchErrorDetails {
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
            })
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

#[cfg(test)]
mod tests {
    use super::{
        send, status_codes, AdaptiveTransport, CosmosError, DispatchError, HttpResponse,
        PreparedRequest, RequestDiagnosticsBuilder, RequestSentStatus, TransportMode,
        TransportOutcome, TransportResult,
    };
    use crate::{
        diagnostics::{
            DiagnosticsContextBuilder, ExecutionContext, PipelineKind, TransportSecurity,
        },
        driver::routing::CosmosEndpoint,
        models::ActivityId,
        options::DiagnosticsOptions,
        pipeline::transport::cosmos_transport_client::{
            HttpRequest, TransportClient, TransportError,
        },
    };
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct MockTransportClient(Mutex<Option<Result<HttpResponse, TransportError>>>);

    #[async_trait]
    impl TransportClient for MockTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.0
                .lock()
                .unwrap()
                .take()
                .expect("only one send is expected")
        }
    }

    #[allow(
        clippy::result_large_err,
        reason = "Exercises the production dispatch result without changing its inline representation."
    )]
    async fn dispatch_result(
        result: Result<HttpResponse, TransportError>,
        mode: TransportMode,
    ) -> Result<TransportResult, DispatchError> {
        let transport =
            AdaptiveTransport::Gateway(Arc::new(MockTransportClient(Mutex::new(Some(result)))));
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let prepared = PreparedRequest {
            http_request: HttpRequest {
                url: endpoint.url().clone(),
                method: azure_core::http::Method::Get,
                headers: Default::default(),
                body: None,
                timeout: None,
                #[cfg(feature = "fault_injection")]
                evaluation_collector: None,
            },
            mode,
            timeout: None,
        };
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::from_string("dispatch-classification".to_owned()),
            Arc::new(DiagnosticsOptions::default()),
        );
        let handle = diagnostics.start_request(
            ExecutionContext::Initial,
            PipelineKind::DataPlane,
            TransportSecurity::Secure,
            transport.diagnostics_kind(),
            transport.diagnostics_http_version(),
            &endpoint,
        );
        send(
            &prepared,
            &transport,
            None,
            &endpoint.endpoint_key(),
            &mut RequestDiagnosticsBuilder::new(handle, &mut diagnostics),
            Default::default(),
        )
        .await
    }

    #[tokio::test]
    async fn response_classification_distinguishes_http_and_decoding_errors() {
        for mode in [TransportMode::Gateway, TransportMode::GatewayV2] {
            for status in [200, 429, 502] {
                let outcome = dispatch_result(
                    Ok(HttpResponse {
                        status,
                        headers: Default::default(),
                        body: b"not an RNTBD frame".to_vec(),
                    }),
                    mode,
                )
                .await;
                match (mode, status, outcome) {
                    (TransportMode::Gateway, 200, Ok(result)) => {
                        assert!(result.outcome.is_successful());
                    }
                    (
                        TransportMode::GatewayV2,
                        200,
                        Err(DispatchError::ResponseDecoding(details)),
                    ) => {
                        assert_eq!(details.shard_id, None);
                        assert!(matches!(
                            details.result.outcome,
                            TransportOutcome::TransportError {
                                status: status_codes::TRANSPORT_GENERATED_503,
                                request_sent: RequestSentStatus::Sent,
                                ..
                            }
                        ));
                    }
                    (_, 429 | 502, Err(DispatchError::Http(details))) => {
                        assert_eq!(details.shard_id, None);
                        let TransportOutcome::HttpError {
                            status: actual,
                            body,
                            request_sent,
                            ..
                        } = details.result.outcome
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

    #[tokio::test]
    async fn raw_sent_status_and_underlying_error_are_mapped_independently() {
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
                let error = dispatch_result(
                    Err(TransportError::new(
                        CosmosError::builder().with_status(inner).build(),
                        raw,
                    )),
                    TransportMode::Gateway,
                )
                .await
                .unwrap_err();
                assert_eq!(error.failed_after_headers(), raw == RequestSentStatus::Sent);
                let DispatchError::Transport { details, .. } = error else {
                    panic!("expected a transport failure");
                };
                let TransportOutcome::TransportError {
                    status,
                    error,
                    request_sent,
                } = details.result.outcome
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
                assert_eq!(details.shard_id, None);
            }
        }
    }
}
