// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Assembles terminal responses and errors with their diagnostics.

use std::sync::Arc;

use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::CosmosResponse,
    pipeline::{
        operation::{recovery, response},
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Attaches the completed operation journal and terminal status to an error.
pub(crate) fn fail(
    error: crate::error::CosmosError,
    mut diagnostics: DiagnosticsContextBuilder,
) -> crate::error::CosmosError {
    diagnostics.set_operation_status(error.status().status_code(), error.status().sub_status());
    crate::error::CosmosErrorBuilder::from_error(error)
        .with_diagnostics(Arc::new(diagnostics.complete()))
        .build()
}

/// Preserves the stale-container service error while the caller rebuilds its execution plan.
pub(crate) fn recreation_error(
    result: &TransportResult,
    diagnostics: DiagnosticsContextBuilder,
) -> crate::error::CosmosError {
    let TransportOutcome::HttpError {
        status,
        cosmos_headers,
        body,
        ..
    } = &result.outcome
    else {
        unreachable!("recreation signals are HTTP responses");
    };
    self::fail(
        recovery::error::build_service_error(status, cosmos_headers, body),
        diagnostics,
    )
}

/// Builds a `CosmosResponse` from a successful `TransportResult`.
pub(crate) fn build_cosmos_response(
    result: Box<TransportResult>,
    mut diagnostics: DiagnosticsContextBuilder,
    routing_fallback: bool,
) -> crate::error::Result<CosmosResponse> {
    match result.outcome {
        TransportOutcome::Success {
            status,
            cosmos_headers,
            body,
        } => {
            diagnostics.set_operation_status(status.status_code(), status.sub_status());

            let diagnostics_ctx = Arc::new(diagnostics.complete());

            Ok(
                CosmosResponse::new(body, cosmos_headers, status, diagnostics_ctx)
                    .with_routing_fallback(routing_fallback),
            )
        }
        _ => {
            // This should only be called with a Complete(Success) result.
            // Treat as a programmer-error invariant violation.
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE)
                .with_message("build_cosmos_response called with non-success result")
                .build())
        }
    }
}

/// Converts a winning leg into the operation response, retaining its status, body, and journal.
///
/// HTTP errors use the same service-error representation as sequential execution.
/// The coordinator calls this while it still owns any unfinished losing future.
pub(crate) fn finalize_hedge_attempt(
    result: Box<TransportResult>,
    diagnostics: DiagnosticsContextBuilder,
) -> crate::error::Result<CosmosResponse> {
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => response::build_cosmos_response(
            Box::new(TransportResult { outcome }),
            diagnostics,
            false,
        ),
        TransportOutcome::HttpError {
            status,
            cosmos_headers,
            body,
            ..
        } => {
            tracing::debug!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                http_status = u16::from(status.status_code()),
                sub_status = ?status.sub_status(),
                "non-retriable http error in hedging attempt",
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            let base = recovery::error::build_service_error(&status, &cosmos_headers, &body);
            Err(crate::error::CosmosErrorBuilder::from_error(base)
                .with_diagnostics(diagnostics_ctx)
                .build())
        }
        TransportOutcome::TransportError { error, .. } => {
            tracing::debug!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                error = %error,
                "non-retriable transport error in hedging attempt",
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            Err(crate::error::CosmosErrorBuilder::from_error(error)
                .with_diagnostics(diagnostics_ctx)
                .build())
        }
        TransportOutcome::DeadlineExceeded { .. } => {
            tracing::warn!(
                activity_id = %diagnostics.activity_id(),
                request_count = diagnostics.request_count(),
                "deadline exceeded in hedging attempt",
            );
            // Typed status (408 + CLIENT_OPERATION_TIMEOUT) mirrors
            // `enforce_deadline_or_timeout` so retry-evaluation and
            // telemetry treat hedged deadline-exceeded identically to
            // non-hedged.
            let mut diagnostics = diagnostics;
            diagnostics.set_operation_status(
                azure_core::http::StatusCode::RequestTimeout,
                Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
            );
            let diagnostics_ctx = Arc::new(diagnostics.complete());
            Err(crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::from_parts(
                    azure_core::http::StatusCode::RequestTimeout,
                    Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
                ))
                .with_message("deadline exceeded during hedged attempt")
                .with_diagnostics(diagnostics_ctx)
                .build())
        }
    }
}

/// Rejects an incompatible account consistency setting with zero-request diagnostics.
pub(crate) fn global_strong_account_validation_error(
    mut diagnostics: DiagnosticsContextBuilder,
) -> crate::error::CosmosError {
    let status = crate::error::status_codes::CLIENT_BAD_REQUEST;
    diagnostics.set_operation_status(status.status_code(), status.sub_status());
    crate::error::CosmosError::builder()
        .with_status(status)
        .with_message(
            "ReadConsistencyStrategy::GlobalStrong is only valid against accounts whose \
             default consistency level is Strong",
        )
        .with_diagnostics(Arc::new(diagnostics.complete()))
        .build()
}
