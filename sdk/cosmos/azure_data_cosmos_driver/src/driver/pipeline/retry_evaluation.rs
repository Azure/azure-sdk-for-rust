// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure function: evaluate the result of a transport attempt.
//!
//! Handles all HTTP error cases for the multi-region operation loop:
//! - Success → Complete
//! - Transport error (NotSent) → TransportRetry if budget allows
//! - Transport error (Sent/Unknown, idempotent) → TransportRetry if budget allows
//! - Transport error (Sent/Unknown, non-idempotent, no PPAF) → Abort
//! - 403/3 WriteForbidden → FailoverRetry + refresh + mark unavailable
//! - 404/1002 ReadSessionNotAvailable → SessionRetry (advances region)
//! - 408 RequestTimeout → FailoverRetry + mark partition/endpoint unavailable
//! - 503, 429/3092, 410 → FailoverRetry + mark partition/endpoint unavailable
//! - 503, 429/3092, 410, 408 (non-idempotent, PPAF) → FailoverRetry (write region discovery)
//! - 500 (reads only) → FailoverRetry + mark partition/endpoint unavailable
//! - Other HTTP errors → Abort

use azure_core::http::headers::Headers;

use crate::{
    driver::routing::{CosmosEndpoint, LocationEffect, UnavailablePartition, UnavailableReason},
    models::{CosmosOperation, CosmosStatus, SubStatusCode},
};

use super::components::{OperationAction, OperationRetryState, TransportOutcome, TransportResult};

/// Whether the current request is handled by the PPCB threshold mechanism.
///
/// When `true`, `MarkEndpointUnavailable` should be suppressed — failover
/// is driven by the partition-level failure counter instead.
fn is_ppcb_managed(operation: &CosmosOperation, retry_state: &OperationRetryState) -> bool {
    retry_state.ppcb_active
        && operation.resource_type().is_partitioned()
        && (operation.is_read_only() || retry_state.can_use_multiple_write_locations)
}

/// Builds an [`UnavailablePartition`] from the current operation context.
///
/// `is_read` is passed explicitly because some call sites hardcode it
/// (e.g., `false` for WriteForbidden, `true` for 500-on-reads) rather
/// than deriving it from `operation.is_read_only()`.
fn make_partition_unavailable(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    is_read: bool,
) -> UnavailablePartition {
    UnavailablePartition {
        partition_key_range_id: retry_state.partition_key_range_id.clone(),
        region: endpoint.region().cloned(),
        is_read,
        is_partitioned_resource: operation.resource_type().is_partitioned(),
    }
}

/// Evaluates the result of a transport attempt and decides what to do next.
///
/// This is a pure function: it takes the operation, result, and retry state,
/// and returns an `OperationAction`. No side effects.
pub(crate) fn evaluate_transport_result(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    result: TransportResult,
    retry_state: &OperationRetryState,
) -> (OperationAction, Vec<LocationEffect>) {
    // Destructure the owned outcome to move error values out without
    // losing the error source chain.
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => (
            OperationAction::Complete(Box::new(TransportResult { outcome })),
            Vec::new(),
        ),

        TransportOutcome::HttpError {
            status,
            headers,
            body,
            request_sent,
            ..
        } => {
            let request_definitely_not_sent = request_sent.definitely_not_sent();

            if status.is_write_forbidden() && retry_state.can_retry_failover() {
                return (
                    OperationAction::FailoverRetry {
                        new_state: retry_state.clone().advance_failover(),
                        delay: None,
                    },
                    vec![
                        LocationEffect::RefreshAccountProperties,
                        LocationEffect::MarkEndpointUnavailable {
                            endpoint: endpoint.clone(),
                            reason: UnavailableReason::WriteForbidden,
                        },
                        LocationEffect::MarkPartitionUnavailable(
                            make_partition_unavailable(operation, endpoint, retry_state, false),
                        ),
                    ],
                );
            }

            if status.is_read_session_not_available() && retry_state.can_retry_session() {
                if !retry_state.can_use_multiple_write_locations
                    && retry_state.session_token_retry_count >= 2
                {
                    return (
                        OperationAction::Abort {
                            error: build_http_error(&status, &headers, &body),
                            status: Some(status),
                        },
                        Vec::new(),
                    );
                }

                return (
                    OperationAction::SessionRetry {
                        new_state: retry_state.clone().advance_session_retry(),
                    },
                    Vec::new(),
                );
            }

            let is_system_resource_unavailable = status.is_throttled()
                && status.sub_status() == Some(SubStatusCode::SYSTEM_RESOURCE_UNAVAILABLE);
            let is_service_unavailable =
                status.status_code() == azure_core::http::StatusCode::ServiceUnavailable;
            let is_gone = status.is_gone();
            let is_request_timeout =
                status.status_code() == azure_core::http::StatusCode::RequestTimeout;

            if (is_system_resource_unavailable
                || is_service_unavailable
                || is_gone
                || is_request_timeout)
                && retry_state.can_retry_failover()
            {
                if request_definitely_not_sent {
                    return (
                        OperationAction::FailoverRetry {
                            new_state: retry_state.clone().advance_failover(),
                            delay: None,
                        },
                        Vec::new(),
                    );
                }

                let unavailable_reason = if is_request_timeout {
                    UnavailableReason::RequestTimeout
                } else {
                    UnavailableReason::ServiceUnavailable
                };

                if !(operation.is_read_only()
                    || operation.is_idempotent()
                    || retry_state.ppaf_write_retry_allowed)
                {
                    // Non-idempotent write that was already sent and PPAF is
                    // not available — unsafe to retry. We still mark partition
                    // and endpoint unavailable so future requests benefit from
                    // the updated routing state.
                    let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
                        make_partition_unavailable(operation, endpoint, retry_state, false),
                    )];
                    if !is_ppcb_managed(operation, retry_state) {
                        effects.push(LocationEffect::MarkEndpointUnavailable {
                            endpoint: endpoint.clone(),
                            reason: unavailable_reason,
                        });
                    }
                    return (
                        OperationAction::Abort {
                            error: build_http_error(&status, &headers, &body),
                            status: Some(status),
                        },
                        effects,
                    );
                }

                let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
                    make_partition_unavailable(
                        operation,
                        endpoint,
                        retry_state,
                        operation.is_read_only(),
                    ),
                )];
                if !is_ppcb_managed(operation, retry_state) {
                    effects.push(LocationEffect::MarkEndpointUnavailable {
                        endpoint: endpoint.clone(),
                        reason: unavailable_reason,
                    });
                }
                return (
                    OperationAction::FailoverRetry {
                        new_state: retry_state.clone().advance_failover(),
                        delay: None,
                    },
                    effects,
                );
            }

            if status.status_code() == azure_core::http::StatusCode::InternalServerError
                && operation.is_read_only()
                && retry_state.can_retry_failover()
            {
                let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
                    make_partition_unavailable(operation, endpoint, retry_state, true),
                )];
                if !is_ppcb_managed(operation, retry_state) {
                    effects.push(LocationEffect::MarkEndpointUnavailable {
                        endpoint: endpoint.clone(),
                        reason: UnavailableReason::InternalServerError,
                    });
                }
                return (
                    OperationAction::FailoverRetry {
                        new_state: retry_state.clone().advance_failover(),
                        delay: None,
                    },
                    effects,
                );
            }

            (
                OperationAction::Abort {
                    error: build_http_error(&status, &headers, &body),
                    status: Some(status),
                },
                Vec::new(),
            )
        }

        TransportOutcome::TransportError {
            status,
            error,
            request_sent,
        } => {
            if request_sent.definitely_not_sent() && retry_state.can_retry_failover() {
                return (
                    OperationAction::FailoverRetry {
                        new_state: retry_state.clone().advance_failover(),
                        delay: None,
                    },
                    Vec::new(),
                );
            }

            let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
                make_partition_unavailable(
                    operation,
                    endpoint,
                    retry_state,
                    operation.is_read_only(),
                ),
            )];
            if !is_ppcb_managed(operation, retry_state) {
                effects.push(LocationEffect::MarkEndpointUnavailable {
                    endpoint: endpoint.clone(),
                    reason: UnavailableReason::TransportError,
                });
            }

            if (operation.is_read_only()
                || operation.is_idempotent()
                || retry_state.ppaf_write_retry_allowed)
                && retry_state.can_retry_failover()
            {
                return (
                    OperationAction::FailoverRetry {
                        new_state: retry_state.clone().advance_failover(),
                        delay: None,
                    },
                    effects,
                );
            }

            // Non-idempotent write that was already sent and PPAF is not
            // available — unsafe to retry. We still mark partition and
            // endpoint unavailable so future requests benefit from the
            // updated routing state.
            (
                OperationAction::Abort {
                    error: build_transport_error(&status, error),
                    status: Some(status),
                },
                effects,
            )
        }

        TransportOutcome::DeadlineExceeded { request_sent } => {
            let message = if request_sent.definitely_not_sent() {
                "end-to-end operation timeout exceeded before request was sent"
            } else {
                "end-to-end operation timeout exceeded"
            };

            (
                OperationAction::Abort {
                    error: azure_core::Error::new(azure_core::error::ErrorKind::Other, message),
                    status: Some(CosmosStatus::from_parts(
                        azure_core::http::StatusCode::RequestTimeout,
                        Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                    )),
                },
                Vec::new(),
            )
        }
    }
}

/// Builds an `azure_core::Error` from a Cosmos HTTP error status.
///
/// Attaches the response body and headers as a `raw_response` so callers
/// can match on `ErrorKind::HttpResponse { raw_response: Some(_), .. }`
/// and inspect the service error payload.
fn build_http_error(status: &CosmosStatus, headers: &Headers, body: &[u8]) -> azure_core::Error {
    let status_code = status.status_code();
    let name = status.name().unwrap_or("Unknown");
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };
    let message = format!(
        "Cosmos DB returned HTTP {}{}: {}",
        u16::from(status_code),
        sub_status_str,
        name,
    );

    let error_code: Option<String> = status
        .sub_status()
        .map(|s: SubStatusCode| s.value().to_string());

    let raw_response =
        azure_core::http::RawResponse::from_bytes(status_code, headers.clone(), body.to_vec());

    azure_core::Error::new(
        azure_core::error::ErrorKind::HttpResponse {
            status: status_code,
            error_code,
            raw_response: Some(Box::new(raw_response)),
        },
        message,
    )
}

fn build_transport_error(status: &CosmosStatus, error: azure_core::Error) -> azure_core::Error {
    let status_code = status.status_code();
    let name = status.name().unwrap_or("Unknown");
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };

    let detail_summary = crate::driver::error_chain_summary(&error);
    let message = format!(
        "Cosmos DB transport failure HTTP {}{}: {} (kind: {}). Details: {}",
        u16::from(status_code),
        sub_status_str,
        name,
        error.kind(),
        detail_summary,
    );

    azure_core::Error::with_error(error.kind().clone(), error, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::RequestSentStatus,
        models::{
            AccountReference, CosmosOperation, CosmosResponseHeaders, CosmosStatus,
            DatabaseReference,
        },
    };
    use azure_core::http::StatusCode;

    fn make_read_operation() -> CosmosOperation {
        let account = AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==", // base64 "test"
        );
        let db_ref = DatabaseReference::from_name(account, "testdb".to_owned());
        CosmosOperation::read_database(db_ref)
    }

    fn make_create_operation() -> CosmosOperation {
        // create_database uses OperationType::Create which is NOT idempotent
        let account = AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        CosmosOperation::create_database(account)
    }

    fn make_success_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::Success {
                status: CosmosStatus::new(StatusCode::Ok),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: b"{}".to_vec(),
            },
        }
    }

    fn make_transport_error(sent: RequestSentStatus) -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::TransportError {
                status: CosmosStatus::TRANSPORT_GENERATED_503,
                error: azure_core::Error::new(
                    azure_core::error::ErrorKind::Connection,
                    "connection refused",
                ),
                request_sent: sent,
            },
        }
    }

    fn make_http_error(status_code: StatusCode) -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(status_code),
                headers: azure_core::http::headers::Headers::new(),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    #[test]
    fn success_completes() {
        let op = make_read_operation();
        let result = make_success_result();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::Complete(_)));
        assert!(effects.is_empty());
    }

    #[test]
    fn transport_error_not_sent_retries() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    }

    #[test]
    fn transport_error_sent_idempotent_retries() {
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn transport_error_sent_non_idempotent_aborts() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        match action {
            OperationAction::Abort { status, .. } => {
                assert_eq!(status, Some(CosmosStatus::TRANSPORT_GENERATED_503));
            }
            other => panic!("expected abort, got {other:?}"),
        }
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn transport_abort_error_includes_status_kind_and_details() {
        let op = make_create_operation();
        let result = TransportResult {
            outcome: TransportOutcome::TransportError {
                status: CosmosStatus::TRANSPORT_GENERATED_503,
                error: azure_core::Error::with_error(
                    azure_core::error::ErrorKind::Io,
                    std::io::Error::new(std::io::ErrorKind::BrokenPipe, "socket reset"),
                    "failed to execute `reqwest` request",
                ),
                request_sent: RequestSentStatus::Unknown,
            },
        };
        let state = OperationRetryState::initial(0, false, Vec::new(), 0, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        match action {
            OperationAction::Abort { status, error } => {
                assert_eq!(status, Some(CosmosStatus::TRANSPORT_GENERATED_503));
                assert_eq!(error.kind(), &azure_core::error::ErrorKind::Io);
                let text = error.to_string();
                assert!(text.contains("HTTP 503/20003"));
                assert!(text.contains("TransportGenerated503"));
                assert!(text.contains("kind: Io"));
                assert!(text.contains("failed to execute `reqwest` request"));
                assert!(text.contains("socket reset"));
            }
            other => panic!("expected abort, got {other:?}"),
        }
    }

    #[test]
    fn transport_error_over_budget_aborts() {
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState {
            location: crate::driver::routing::LocationIndex::initial(0),
            failover_retry_count: 1,
            session_token_retry_count: 0,
            max_failover_retries: 1,
            max_session_retries: 1,
            can_use_multiple_write_locations: false,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
        };

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        match action {
            OperationAction::Abort { status, .. } => {
                assert_eq!(status, Some(CosmosStatus::TRANSPORT_GENERATED_503));
            }
            other => panic!("expected abort, got {other:?}"),
        }
    }

    #[test]
    fn http_error_aborts() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::BadRequest);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
    }

    #[test]
    fn write_forbidden_triggers_failover_and_refresh_effect() {
        let op = make_create_operation();
        let result = TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::WRITE_FORBIDDEN,
                headers: azure_core::http::headers::Headers::new(),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        };
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    }

    #[test]
    fn read_session_not_available_triggers_session_retry() {
        let op = make_read_operation();
        let result = TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::READ_SESSION_NOT_AVAILABLE,
                headers: azure_core::http::headers::Headers::new(),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        };
        let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::SessionRetry { .. }));
        assert!(effects.is_empty());
    }

    #[test]
    fn service_unavailable_marks_endpoint_unavailable() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::ServiceUnavailable);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn service_unavailable_sent_non_idempotent_aborts() {
        let op = make_create_operation();
        let result = make_http_error(StatusCode::ServiceUnavailable);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn service_unavailable_non_idempotent_retries_when_ppaf_enabled() {
        let op = make_create_operation();
        let result = make_http_error(StatusCode::ServiceUnavailable);
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.ppaf_write_retry_allowed = true;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn transport_error_non_idempotent_retries_when_ppaf_enabled() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.ppaf_write_retry_allowed = true;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn deadline_exceeded_aborts_with_timeout_status() {
        let op = make_read_operation();
        let result = TransportResult {
            outcome: TransportOutcome::DeadlineExceeded {
                request_sent: RequestSentStatus::Unknown,
            },
        };
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        match action {
            OperationAction::Abort { status, .. } => {
                let status = status.expect("timeout status should be set");
                assert_eq!(status.status_code(), StatusCode::RequestTimeout);
                assert_eq!(
                    status.sub_status(),
                    Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
                );
            }
            _ => panic!("expected timeout to abort"),
        }
        assert!(effects.is_empty());
    }

    #[test]
    fn internal_server_error_on_read_fails_over() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::InternalServerError);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn internal_server_error_on_read_marks_partition_unavailable() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::InternalServerError);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    }

    #[test]
    fn transport_error_not_sent_does_not_mark_partition_or_endpoint() {
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects.is_empty());
    }

    #[test]
    fn request_timeout_from_server_marks_partition_and_endpoint_unavailable() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::RequestTimeout);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(effects.iter().any(|e| matches!(
            e,
            LocationEffect::MarkEndpointUnavailable {
                reason: UnavailableReason::RequestTimeout,
                ..
            }
        )));
    }
}
