// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for safety outcomes.

use crate::models::{CosmosOperation, CosmosStatus};

/// Handles the retry-trigger group — 503 ServiceUnavailable, 410 Gone,
/// 408 RequestTimeout, and 429/3092 SystemResourceUnavailable.
///
/// Two sub-cases:
///
/// 1. **Request not sent** — safe to retry against any region with no
///    location-state side effects (the failure is purely client-side).
/// 2. **Request sent** — failover retry with `MarkPartitionUnavailable`
///    (and, when not PPCB-managed, `MarkEndpointUnavailable`) so future
///    requests benefit from the updated routing state.
/// Whether retrying `operation` is unsafe because the backend may already have
/// processed the request.
///
/// Stored procedure execution and unsafe explicit server-side PATCH are affected — see
/// [`CosmosOperation::allows_ambiguous_outcome_retry`]. `503` stays retryable
/// because Cosmos DB returns it exclusively for requests it did *not* process;
/// `408` and the remaining `5xx` codes leave the outcome genuinely unknown, so
/// the procedure may have run to completion despite the error.
///
/// Callers apply this only after ruling out `RequestSentStatus::NotSent`, which
/// is safe for every operation type.
pub(crate) fn is_unsafe_retry_after_possible_execution(
    operation: &CosmosOperation,
    status: &CosmosStatus,
) -> bool {
    if operation.allows_ambiguous_outcome_retry() {
        return false;
    }
    let status_code = status.status_code();
    status_code == azure_core::http::StatusCode::RequestTimeout
        || (status_code.is_server_error()
            && status_code != azure_core::http::StatusCode::ServiceUnavailable)
}
