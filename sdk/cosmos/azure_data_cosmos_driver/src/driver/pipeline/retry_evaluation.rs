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
    match &result.outcome {
        TransportOutcome::Success { .. } => OperationAction::Complete(result),

        TransportOutcome::HttpError {
            status,
            request_sent,
            ..
        } => {
            // In Step 1, all HTTP errors (non-429) are propagated as abort.
            // 429 is already handled by the transport pipeline's throttle retry.
            // Step 2 will add handling for 403/3, 503, 404/1002, etc.
            let status = *status;
            let request_sent = *request_sent;

            // If it's a non-success HTTP response and the request wasn't sent
            // (or is idempotent), we could retry. But in Step 1, HTTP errors
            // (other than 429, already handled) are not retried at the operation
            // level — that comes in Step 2 with failover.
            //
            // However, we do allow transport-level retry for transport errors
            // that surface as HTTP errors in some edge cases.
            if retry_state.can_retry_transport()
                && (request_sent.definitely_not_sent() || operation.is_idempotent())
            {
                // For Step 1, only retry on clearly transient errors.
                // Most HTTP errors need operation-level handling (Step 2).
                // Don't retry HTTP errors at transport level in Step 1.
            }

            OperationAction::Abort {
                error: build_http_error(&status),
                status: Some((status.status_code(), status.sub_status())),
            }
        }

        TransportOutcome::TransportError {
            error,
            request_sent,
        } => {
            let request_sent = *request_sent;
            let is_safe_to_retry = request_sent.definitely_not_sent() || operation.is_idempotent();

            if is_safe_to_retry && retry_state.can_retry_transport() {
                OperationAction::TransportRetry {
                    new_state: retry_state.advance_transport_retry(),
                }
            } else {
                OperationAction::Abort {
                    error: azure_core::Error::new(error.kind().clone(), error.to_string()),
                    status: None,
                }
            }
        }
    }
}

/// Builds an `azure_core::Error` from a Cosmos HTTP error status.
fn build_http_error(status: &CosmosStatus) -> azure_core::Error {
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

    azure_core::Error::new(
        azure_core::error::ErrorKind::HttpResponse {
            status: status_code,
            error_code,
            raw_response: None,
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
