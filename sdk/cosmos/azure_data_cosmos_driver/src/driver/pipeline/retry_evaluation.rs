// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure function: evaluate the result of a transport attempt.
//!
//! This is the "slim" version for Step 1:
//! - Success → Complete
//! - Transport error (NotSent) → TransportRetry if budget allows
//! - Transport error (Sent/Unknown, idempotent) → TransportRetry if budget allows
//! - Transport error (Sent/Unknown, non-idempotent) → Abort
//! - HTTP error (non-429; 429 is handled by the transport pipeline) → Abort
//!
//! Step 2 will expand this with failover, session retry, and location effects.

use azure_core::http::headers::Headers;

use crate::models::{CosmosOperation, CosmosStatus, SubStatusCode};

use super::components::{OperationAction, OperationRetryState, TransportOutcome, TransportResult};

/// Evaluates the result of a transport attempt and decides what to do next.
///
/// This is a pure function: it takes the operation, result, and retry state,
/// and returns an `OperationAction`. No side effects.
pub(crate) fn evaluate_transport_result(
    operation: &CosmosOperation,
    result: TransportResult,
    retry_state: &OperationRetryState,
) -> OperationAction {
    // Destructure the owned outcome to move error values out without
    // losing the error source chain.
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => {
            OperationAction::Complete(TransportResult { outcome })
        }

        TransportOutcome::HttpError {
            status,
            headers,
            body,
            request_sent,
        } => {
            // In Step 1, all HTTP errors (non-429) are propagated as abort.
            // 429 is already handled by the transport pipeline's throttle retry.
            // Step 2 will add handling for 403/3, 503, 404/1002, etc.
            if retry_state.can_retry_transport()
                && (request_sent.definitely_not_sent() || operation.is_idempotent())
            {
                // TODO(Step 2): retry on transient HTTP errors (403/3, 503,
                // 404/1002, 429/3092, 500-for-reads) with failover.
            }

            OperationAction::Abort {
                error: build_http_error(&status, &headers, &body),
                status: Some(status),
            }
        }

        TransportOutcome::TransportError {
            error,
            request_sent,
        } => {
            let is_safe_to_retry = request_sent.definitely_not_sent() || operation.is_idempotent();

            if is_safe_to_retry && retry_state.can_retry_transport() {
                OperationAction::TransportRetry {
                    new_state: retry_state.advance_transport_retry(),
                }
            } else {
                OperationAction::Abort {
                    error,
                    status: None,
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::RequestSentStatus,
        models::{AccountReference, CosmosOperation, CosmosStatus, DatabaseReference},
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
                headers: azure_core::http::headers::Headers::new(),
                body: b"{}".to_vec(),
            },
        }
    }

    fn make_transport_error(sent: RequestSentStatus) -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::TransportError {
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
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    #[test]
    fn success_completes() {
        let op = make_read_operation();
        let result = make_success_result();
        let state = OperationRetryState::initial();

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::Complete(_)));
    }

    #[test]
    fn transport_error_not_sent_retries() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState::initial();

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::TransportRetry { .. }));
    }

    #[test]
    fn transport_error_sent_idempotent_retries() {
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let state = OperationRetryState::initial();

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::TransportRetry { .. }));
    }

    #[test]
    fn transport_error_sent_non_idempotent_aborts() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let state = OperationRetryState::initial();

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
    }

    #[test]
    fn transport_error_over_budget_aborts() {
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState {
            transport_retry_count: 1,
            max_transport_retries: 1,
        };

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
    }

    #[test]
    fn http_error_aborts() {
        let op = make_read_operation();
        let result = make_http_error(StatusCode::InternalServerError);
        let state = OperationRetryState::initial();

        let action = evaluate_transport_result(&op, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
    }
}
