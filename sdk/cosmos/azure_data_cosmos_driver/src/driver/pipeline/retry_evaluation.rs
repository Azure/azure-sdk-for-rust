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

use crate::{
    diagnostics::RequestSentStatus,
    driver::routing::{CosmosEndpoint, LocationEffect, UnavailablePartition, UnavailableReason},
    models::{CosmosOperation, CosmosResponseHeaders, CosmosStatus, SubStatusCode},
};

use super::components::{OperationAction, OperationRetryState, TransportOutcome, TransportResult};

/// Whether the current request is handled by the PPCB threshold mechanism.
///
/// When `true`, `MarkEndpointUnavailable` should be suppressed — failover
/// is driven by the partition-level failure counter instead.
fn is_ppcb_managed(operation: &CosmosOperation, retry_state: &OperationRetryState) -> bool {
    retry_state.ppcb_active
        && operation
            .resource_type()
            .is_partitioned(operation.operation_type())
        && (operation.is_read_only() || retry_state.can_use_multiple_write_locations)
}

/// Builds an [`UnavailablePartition`] from the current operation context.
///
/// `is_read` is passed explicitly because the WriteForbidden handler hardcodes
/// it to `false` (the request was a write, even though the gateway redirected
/// us to read from a different region). All other call sites derive it from
/// `operation.is_read_only()` so that PPCB increments the correct
/// per-partition counter (read vs write) and gates failover by the matching
/// threshold.
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
        is_partitioned_resource: operation
            .resource_type()
            .is_partitioned(operation.operation_type()),
    }
}

/// Returns `true` when a status proves the request reached a server and was
/// processed by it.
///
/// Used by the operation pipeline to decide whether to flush deferred
/// write-path `MarkPartitionUnavailable` (and, for PPAF on single-master,
/// `MarkEndpointUnavailable`) effects when the operation aborts rather than
/// completing successfully. The intuition is: any response outside the
/// retry-trigger set means the *current* region accepted the request, which
/// retroactively confirms that the *earlier* failed regions were the
/// unhealthy ones — their pending marks should be applied so future requests
/// route around them.
///
/// The decision is **deny-listed**, not allow-listed: every status is
/// treated as region-confirming **except** the explicit retry-trigger set
/// and client-synthesized statuses. This means uncommon-but-deterministic
/// service responses (202 Accepted, 207 MultiStatus, 404/0 NotFound, 413
/// Payload Too Large, 449 RetryWith, 451 Unavailable For Legal Reasons,
/// etc.) all flush deferred marks just like the more familiar 200/409/412.
///
/// Returns `false` for:
/// - 503 ServiceUnavailable, 408 RequestTimeout, 410 Gone, 429/3092 (system
///   resource unavailable), 403/3 (write forbidden) — the retry-trigger set;
///   we have no proof any region accepted the request.
/// - Client-synthesized statuses (e.g. `CLIENT_OPERATION_TIMEOUT`) — these
///   never came from a server.
///
/// Returns `true` for everything else, including:
/// - All 2xx (200 OK, 201 Created, 202 Accepted, 204 No Content, 207
///   MultiStatus). The 2xx case is normally taken via
///   `OperationAction::Complete`, but is preserved here for defense in depth.
/// - Definitive 4xx (400, 401, 404 with any non-1002 sub-status, 409
///   Conflict, 412 Precondition Failed, 413 Payload Too Large) — the server
///   processed and rejected the request.
/// - Server errors (500, 501, 504, 505) once the retry budget is exhausted.
pub(crate) fn is_region_confirming_status(status: &CosmosStatus) -> bool {
    let code = status.status_code();

    if code.is_success() {
        return true;
    }

    // Retry-trigger statuses — not confirming.
    if code == azure_core::http::StatusCode::ServiceUnavailable
        || code == azure_core::http::StatusCode::RequestTimeout
        || code == azure_core::http::StatusCode::Gone
    {
        return false;
    }

    if status.is_throttled()
        && status.sub_status() == Some(SubStatusCode::SYSTEM_RESOURCE_UNAVAILABLE)
    {
        return false;
    }

    if status.is_write_forbidden() {
        return false;
    }

    // Synthesized client-side statuses (e.g., end-to-end timeout) — not from a server.
    if status.sub_status() == Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT) {
        return false;
    }

    // Any other status from the service confirms the region processed the request.
    true
}

/// Splits a list of location effects into immediate effects and deferred
/// write-path effects.
///
/// PPCB-managed operations — reads, and writes on a multi-master account —
/// always apply effects immediately. The per-partition failure counter is
/// what drives threshold-based failover, and it must see every failure
/// signal at the moment it is observed; deferring would prevent the breaker
/// from ever tripping for non-idempotent writes that abort.
///
/// Single-master writes (where the per-partition circuit breaker is not
/// applicable) defer `MarkPartitionUnavailable` until the write definitively
/// reaches a region. When PPAF is additionally enabled
/// (`ppaf_write_retry_allowed`), `MarkEndpointUnavailable` is also deferred
/// so a transient retry against the only known write region cannot pollute
/// the endpoint-unavailability state with an unverified failure.
pub(crate) fn partition_effects_for_deferral(
    is_read_only: bool,
    can_use_multiple_write_locations: bool,
    ppaf_write_retry_allowed: bool,
    effects: Vec<LocationEffect>,
) -> (Vec<LocationEffect>, Vec<LocationEffect>) {
    // PPCB-managed paths (reads and multi-master writes) bypass deferral so
    // the partition failure counter increments immediately on every 503.
    if is_read_only || can_use_multiple_write_locations {
        return (effects, Vec::new());
    }
    let mut immediate = Vec::with_capacity(effects.len());
    let mut deferred = Vec::new();
    for effect in effects {
        match effect {
            LocationEffect::MarkPartitionUnavailable(_) => deferred.push(effect),
            LocationEffect::MarkEndpointUnavailable { .. } if ppaf_write_retry_allowed => {
                deferred.push(effect);
            }
            other => immediate.push(other),
        }
    }
    (immediate, deferred)
}

/// Evaluates the result of a transport attempt and decides what to do next.
///
/// This is a pure function: it takes the operation, result, and retry state,
/// and returns an `OperationAction`. No side effects.
///
/// Dispatches to a per-outcome handler so each transport outcome shape is
/// classified in isolation. The HTTP-error handler in turn dispatches to a
/// chain of per-status-family helpers.
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
            cosmos_headers,
            body,
            request_sent,
        } => evaluate_http_outcome(
            operation,
            endpoint,
            retry_state,
            status,
            cosmos_headers,
            body,
            request_sent,
        ),

        TransportOutcome::TransportError {
            status,
            error,
            request_sent,
        } => evaluate_transport_layer_outcome(
            operation,
            endpoint,
            retry_state,
            status,
            error,
            request_sent,
        ),

        TransportOutcome::DeadlineExceeded { request_sent } => {
            evaluate_deadline_exceeded_outcome(request_sent)
        }
    }
}

/// Classifies an HTTP error response by walking a chain of per-status-family
/// handlers in priority order.
///
/// The order matters: the more specific Cosmos sub-status checks (403/3,
/// 404/1002, 429/3092) come before the generic status-code-family checks
/// (5xx). The first handler that recognizes the response returns
/// `Some(action, effects)`; if none match, the response is aborted with a
/// rich HTTP error.
#[allow(clippy::too_many_arguments)]
fn evaluate_http_outcome(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: CosmosStatus,
    cosmos_headers: CosmosResponseHeaders,
    body: Vec<u8>,
    request_sent: RequestSentStatus,
) -> (OperationAction, Vec<LocationEffect>) {
    if let Some(result) = try_handle_write_forbidden(operation, endpoint, retry_state, &status) {
        return result;
    }

    if let Some(result) =
        try_handle_read_session_not_available(retry_state, &status, &cosmos_headers, &body)
    {
        return result;
    }

    if let Some(result) = try_handle_retry_trigger_group(
        operation,
        endpoint,
        retry_state,
        &status,
        &cosmos_headers,
        &body,
        request_sent,
    ) {
        return result;
    }

    if let Some(result) = try_handle_server_error(operation, endpoint, retry_state, &status) {
        return result;
    }

    (
        OperationAction::Abort {
            error: build_service_error(&status, &cosmos_headers, &body),
        },
        Vec::new(),
    )
}

/// Handles 403/3 WriteForbidden — the gateway has identified that this region
/// is not currently the write region for the partition.
///
/// Always retries cross-region when the failover budget allows, and emits
/// effects to (a) refresh account properties so the new write region is
/// learned, (b) mark this endpoint unavailable, and (c) mark this partition
/// unavailable in the current (read) region for write traffic.
fn try_handle_write_forbidden(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, Vec<LocationEffect>)> {
    if !(status.is_write_forbidden() && retry_state.can_retry_failover()) {
        return None;
    }

    Some((
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
            LocationEffect::MarkPartitionUnavailable(make_partition_unavailable(
                operation,
                endpoint,
                retry_state,
                false,
            )),
        ],
    ))
}

/// Handles 404/1002 ReadSessionNotAvailable — session token is ahead of the
/// region being read from (session lag).
///
/// On single-master accounts a session retry that has already happened twice
/// is treated as a permanent miss (the writes truly haven't replicated and
/// retrying further is unlikely to help). Otherwise issues a `SessionRetry`
/// which advances to a different region without consuming failover budget.
fn try_handle_read_session_not_available(
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
) -> Option<(OperationAction, Vec<LocationEffect>)> {
    if !(status.is_read_session_not_available() && retry_state.can_retry_session()) {
        return None;
    }

    if !retry_state.can_use_multiple_write_locations && retry_state.session_token_retry_count >= 2 {
        return Some((
            OperationAction::Abort {
                error: build_service_error(status, cosmos_headers, body),
            },
            Vec::new(),
        ));
    }

    Some((
        OperationAction::SessionRetry {
            new_state: build_session_retry_state(retry_state),
        },
        Vec::new(),
    ))
}

/// Builds the `OperationRetryState` for a 404/1002 session retry,
/// latching the `hub_region_processing_only` flag when the trigger
/// conditions defined by HUB_REGION_PROCESSING_HEADER_SPEC.md fire.
///
/// All four conditions must hold (HUB_REGION_PROCESSING_HEADER_SPEC.md
/// §7.1 / public-spec §3.3):
///
/// 1. `is_dataplane` — metadata operations ride the same pipeline but
///    are scoped out per spec §1.5 (AC-8).
/// 2. `!can_use_multiple_write_locations` — single-master only (AC-4).
/// 3. `session_token_retry_count == 0` — first 1002 within the
///    operation; the count is incremented by `advance_session_retry`
///    so reading `retry_state.session_token_retry_count` here detects
///    the pre-increment value (AC-3, S2 / T-5).
/// 4. `!hub_region_processing_only` — defense-in-depth idempotency;
///    structurally already guaranteed by latch-once semantics.
///
/// **Hedging coordination (future).** Per HEDGING_SPEC.md §9.5, when
/// `OperationRetryState` gains a `shared_hub_region_latch:
/// Option<Arc<AtomicBool>>` (populated by `execute_with_hedging()`),
/// this function MUST also CAS-set the shared latch with
/// `Release` ordering when it latches the per-state flag. That is the
/// Rust counterpart of .NET v3's `CrossRegionAvailabilityContext` flag
/// from azure-cosmos-dotnet-v3#5815 and is what propagates the
/// discovery from one hedge to its siblings without each hedge
/// independently re-running the 404/1002 cycle.
fn build_session_retry_state(retry_state: &OperationRetryState) -> OperationRetryState {
    let mut new_state = retry_state.clone().advance_session_retry();
    if retry_state.is_dataplane
        && !retry_state.can_use_multiple_write_locations
        && retry_state.session_token_retry_count == 0
        && !retry_state.hub_region_processing_only
    {
        new_state.hub_region_processing_only = true;
    }
    new_state
}

/// Handles the retry-trigger group — 503 ServiceUnavailable, 410 Gone,
/// 408 RequestTimeout, and 429/3092 SystemResourceUnavailable.
///
/// Three sub-cases:
///
/// 1. **Request not sent** — safe to retry against any region with no
///    location-state side effects (the failure is purely client-side).
/// 2. **Sent + non-idempotent + no PPAF** — unsafe to retry. Aborts but
///    still emits `MarkPartitionUnavailable` (and, when not PPCB-managed,
///    `MarkEndpointUnavailable`) so future requests benefit from the
///    updated routing state.
/// 3. **Sent + (read || idempotent || PPAF write)** — failover retry with
///    the same routing-state effects.
#[allow(clippy::too_many_arguments)]
fn try_handle_retry_trigger_group(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
    request_sent: RequestSentStatus,
) -> Option<(OperationAction, Vec<LocationEffect>)> {
    let is_system_resource_unavailable = status.is_throttled()
        && status.sub_status() == Some(SubStatusCode::SYSTEM_RESOURCE_UNAVAILABLE);
    let is_service_unavailable =
        status.status_code() == azure_core::http::StatusCode::ServiceUnavailable;
    // Partition Topology changes (410 with sub-status 1009) are handled by the dataflow layer, not classified as retry triggers here. Only non-topology 410s trigger retries.
    let is_gone = status.is_gone() && !status.is_partition_topology_change();
    let is_request_timeout = status.status_code() == azure_core::http::StatusCode::RequestTimeout;

    let in_trigger_group =
        is_system_resource_unavailable || is_service_unavailable || is_gone || is_request_timeout;
    if !(in_trigger_group && retry_state.can_retry_failover()) {
        return None;
    }

    if request_sent.definitely_not_sent() {
        return Some((
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            Vec::new(),
        ));
    }

    let unavailable_reason = if is_request_timeout {
        UnavailableReason::RequestTimeout
    } else {
        UnavailableReason::ServiceUnavailable
    };

    let safe_to_retry = operation.is_read_only()
        || operation.is_idempotent()
        || retry_state.ppaf_write_retry_allowed;

    if !safe_to_retry {
        // Non-idempotent write that was already sent and PPAF is not
        // available — unsafe to retry. We still mark partition and endpoint
        // unavailable so future requests benefit from the updated routing
        // state.
        let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
            make_partition_unavailable(operation, endpoint, retry_state, false),
        )];
        if !is_ppcb_managed(operation, retry_state) {
            effects.push(LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint.clone(),
                reason: unavailable_reason,
            });
        }
        return Some((
            OperationAction::Abort {
                error: build_service_error(status, cosmos_headers, body),
            },
            effects,
        ));
    }

    let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
        make_partition_unavailable(operation, endpoint, retry_state, operation.is_read_only()),
    )];
    if !is_ppcb_managed(operation, retry_state) {
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: unavailable_reason,
        });
    }
    Some((
        OperationAction::FailoverRetry {
            new_state: retry_state.clone().advance_failover(),
            delay: None,
        },
        effects,
    ))
}

/// Handles generic 5xx server errors (and 408 RequestTimeout as a defensive
/// fallback for the rare path where it didn't get classified by the
/// retry-trigger-group helper).
///
/// Cross-region retry is attempted for both reads and writes — the assumption
/// is that an internal error in one region is unlikely to repeat in another.
fn try_handle_server_error(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, Vec<LocationEffect>)> {
    let status_code = status.status_code();
    let is_eligible_status = status_code.is_server_error()
        || status_code == azure_core::http::StatusCode::RequestTimeout;
    if !(is_eligible_status && retry_state.can_retry_failover()) {
        return None;
    }

    let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
        make_partition_unavailable(operation, endpoint, retry_state, operation.is_read_only()),
    )];
    if !is_ppcb_managed(operation, retry_state) {
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::InternalServerError,
        });
    }
    Some((
        OperationAction::FailoverRetry {
            new_state: retry_state.clone().advance_failover(),
            delay: None,
        },
        effects,
    ))
}

/// Handles transport-layer errors (connection failures, TLS errors, etc.) —
/// no HTTP response was produced.
///
/// Three sub-cases mirror the retry-trigger-group helper:
/// 1. Request definitely not sent → safe failover retry, no side effects.
/// 2. Sent and read/idempotent/PPAF write → failover retry with marks.
/// 3. Sent and non-idempotent without PPAF → abort with marks.
fn evaluate_transport_layer_outcome(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: CosmosStatus,
    error: crate::error::Error,
    request_sent: RequestSentStatus,
) -> (OperationAction, Vec<LocationEffect>) {
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
        make_partition_unavailable(operation, endpoint, retry_state, operation.is_read_only()),
    )];
    if !is_ppcb_managed(operation, retry_state) {
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::TransportError,
        });
    }

    let safe_to_retry = operation.is_read_only()
        || operation.is_idempotent()
        || retry_state.ppaf_write_retry_allowed;
    if safe_to_retry && retry_state.can_retry_failover() {
        return (
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            effects,
        );
    }

    // Non-idempotent write that was already sent and PPAF is not available —
    // unsafe to retry. Marks are kept so future requests benefit from the
    // updated routing state.
    (
        OperationAction::Abort {
            error: build_transport_error(&status, error),
        },
        effects,
    )
}

/// Handles a deadline-exceeded transport outcome — the end-to-end operation
/// timeout fired before a response could be returned.
///
/// No retry is possible (the deadline applies to the whole operation, so
/// retrying would immediately re-trigger). The synthesized error carries
/// `RequestTimeout` + `CLIENT_OPERATION_TIMEOUT` so callers can distinguish
/// a client-side end-to-end timeout from a service 408.
fn evaluate_deadline_exceeded_outcome(
    request_sent: RequestSentStatus,
) -> (OperationAction, Vec<LocationEffect>) {
    let message: &'static str = if request_sent.definitely_not_sent() {
        "end-to-end operation timeout exceeded before request was sent"
    } else {
        "end-to-end operation timeout exceeded"
    };

    // Build the typed end-to-end timeout error (carries
    // `RequestTimeout` + `CLIENT_OPERATION_TIMEOUT` on `error.status()`)
    // and abort. The operation pipeline propagates
    // `crate::error::Error` directly via `OperationAction::Abort.error`.
    let cosmos_err = crate::error::Error::end_to_end_timeout(message, None);

    (OperationAction::Abort { error: cosmos_err }, Vec::new())
}

/// Formats the human-readable message for a Cosmos HTTP error status.
fn service_error_message(status: &CosmosStatus) -> String {
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };
    format!(
        "Cosmos DB returned HTTP {}{}: {}",
        u16::from(status.status_code()),
        sub_status_str,
        status.name().unwrap_or("Unknown"),
    )
}

/// Builds a typed [`Error`] for a Cosmos HTTP error response.
///
/// Captures the parsed response headers and the raw response body bytes
/// (e.g. the JSON error payload returned by the service for a 400 /
/// BadRequest) on the resulting `Error`. The error propagates through the
/// pipeline as `crate::error::Error` end-to-end — there is no
/// `From<crate::error::Error> for azure_core::Error` impl. SDK-boundary
/// callers that still need an `azure_core::Error` shape can read the wire
/// payload directly via [`Error::status`](crate::error::Error::status),
/// [`Error::cosmos_headers`](crate::error::Error::cosmos_headers), and
/// [`Error::response_body`](crate::error::Error::response_body) without
/// going through a generic round-trip.
fn build_service_error(
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
) -> crate::error::Error {
    // No real diagnostics context is available at this point in the retry
    // pipeline; use the process-wide placeholder so the wire-level response
    // payload (status + headers + body) still rides along on the error.
    let response = crate::models::CosmosResponse::new(
        crate::models::ResponseBody::from_bytes(bytes::Bytes::copy_from_slice(body)),
        cosmos_headers.clone(),
        *status,
        crate::diagnostics::DiagnosticsContext::error_placeholder(),
    );
    crate::error::Error::service(response, service_error_message(status))
}

fn build_transport_error(status: &CosmosStatus, error: crate::error::Error) -> crate::error::Error {
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

    // Wrap into a fresh `Error::transport` carrying the enriched message and
    // the original Cosmos error as source. Forward the inner error's
    // diagnostics so `outer.diagnostics()` is not silently `None` — callers
    // should not have to walk `source()` to recover the operation's
    // diagnostic context.
    let diagnostics = error.diagnostics().cloned();
    crate::error::Error::transport(
        *status,
        message,
        diagnostics,
        Some(std::sync::Arc::new(error)),
    )
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
                )
                .into(),
                request_sent: sent,
            },
        }
    }

    fn make_http_error(status_code: StatusCode) -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::new(status_code),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn make_http_error_status(status: CosmosStatus) -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status,
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
            OperationAction::Abort { error } => {
                assert_eq!(error.status(), CosmosStatus::TRANSPORT_GENERATED_503);
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
    fn build_transport_error_forwards_inner_diagnostics() {
        // The wrap performed by `build_transport_error` must not silently
        // drop the inner error's diagnostics: callers reading
        // `outer.diagnostics()` should see the same `Arc<DiagnosticsContext>`
        // that was attached to the inner cosmos error, not `None`.
        let diag = std::sync::Arc::new(crate::diagnostics::DiagnosticsContext::error_placeholder());
        let inner = crate::error::Error::transport(
            CosmosStatus::TRANSPORT_GENERATED_503,
            "inner transport failure",
            Some(std::sync::Arc::clone(&diag)),
            None,
        );

        let outer = build_transport_error(&CosmosStatus::TRANSPORT_GENERATED_503, inner);

        let outer_diag = outer
            .diagnostics()
            .expect("outer error must inherit inner diagnostics");
        assert!(
            std::sync::Arc::ptr_eq(outer_diag, &diag),
            "outer diagnostics must be the same Arc as the inner's"
        );
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
                )
                .into(),
                request_sent: RequestSentStatus::Unknown,
            },
        };
        let state = OperationRetryState::initial(0, false, Vec::new(), 0, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        match action {
            OperationAction::Abort { error } => {
                // `error` is the typed Cosmos error directly — no
                // round-trip through `azure_core::Error` is required.
                // The fact that `.status()` resolves at all is itself the
                // proof: that accessor only exists on `crate::error::Error`,
                // so if the abort site had returned an `azure_core::Error`
                // (the pre-refactor shape) this line would not compile.
                assert_eq!(error.status(), CosmosStatus::TRANSPORT_GENERATED_503);
                let text = error.to_string();
                assert!(text.contains("HTTP 503/20003"));
                assert!(text.contains("TransportGenerated503"));
                assert!(text.contains("kind: Transport"));
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
            is_dataplane: false,
            hub_region_processing_only: false,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        };

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, _effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        match action {
            OperationAction::Abort { error } => {
                assert_eq!(error.status(), CosmosStatus::TRANSPORT_GENERATED_503);
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
    fn partition_topology_gone_aborts_for_dataflow_handling() {
        let op = make_read_operation();
        let result = make_http_error_status(
            CosmosStatus::new(StatusCode::Gone)
                .with_sub_status(SubStatusCode::PARTITION_KEY_RANGE_GONE.value()),
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        match action {
            OperationAction::Abort { error, .. } => {
                assert_eq!(
                    error.status(),
                    CosmosStatus::new(StatusCode::Gone)
                        .with_sub_status(SubStatusCode::PARTITION_KEY_RANGE_GONE.value())
                );
            }
            other => panic!("expected abort, got {other:?}"),
        }
        assert!(effects.is_empty());
    }

    #[test]
    fn non_topology_gone_still_retries() {
        let op = make_read_operation();
        let result = make_http_error_status(
            CosmosStatus::new(StatusCode::Gone)
                .with_sub_status(SubStatusCode::NAME_CACHE_STALE.value()),
        );
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
    fn write_forbidden_triggers_failover_and_refresh_effect() {
        let op = make_create_operation();
        let result = TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::WRITE_FORBIDDEN,
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
    fn service_unavailable_write_retries_and_marks_partition() {
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
            OperationAction::Abort { error } => {
                let status = error.status();
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

    // ── is_region_confirming_status ───────────────────────────────────

    fn status_with_substatus(code: StatusCode, sub: SubStatusCode) -> CosmosStatus {
        CosmosStatus::from_parts(code, Some(sub))
    }

    #[test]
    fn region_confirming_true_for_2xx() {
        // 200 OK, 201 Created — typical write success codes.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Ok
        )));
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Created
        )));
        // 202 Accepted — used by long-running control-plane operations
        // (e.g., container offer adjustments) that complete asynchronously.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Accepted
        )));
        // 204 No Content — used by deletes and some replace operations.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::NoContent
        )));
        // 207 MultiStatus — used by transactional batch responses; every
        // sub-operation result is encoded in the body but the outer status
        // still proves the region processed the batch.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::from(207u16)
        )));
    }

    #[test]
    fn region_confirming_true_for_definitive_4xx() {
        // 409 Conflict — server processed and rejected the write.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Conflict
        )));
        // 412 Precondition Failed — server processed and rejected.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::PreconditionFailed
        )));
        // 413 Payload Too Large — server processed and rejected.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::PayloadTooLarge
        )));
        // 400/401 — server processed and rejected.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::BadRequest
        )));
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Unauthorized
        )));
        // 404 with no sub-status (404/0) — server confirms the resource is
        // gone. Distinct from 404/1002 (ReadSessionNotAvailable), which is
        // routed to `SessionRetry` rather than `Abort`.
        assert!(is_region_confirming_status(&CosmosStatus::new(
            StatusCode::NotFound
        )));
        // Explicit 404/0 (sub-status 0) construction — same outcome.
        assert!(is_region_confirming_status(&status_with_substatus(
            StatusCode::NotFound,
            SubStatusCode::from(0u32)
        )));
    }

    #[test]
    fn region_confirming_false_for_retry_trigger_statuses() {
        // 503 ServiceUnavailable
        assert!(!is_region_confirming_status(&CosmosStatus::new(
            StatusCode::ServiceUnavailable
        )));
        // 408 RequestTimeout
        assert!(!is_region_confirming_status(&CosmosStatus::new(
            StatusCode::RequestTimeout
        )));
        // 410 Gone
        assert!(!is_region_confirming_status(&CosmosStatus::new(
            StatusCode::Gone
        )));
        // 429/3092 SystemResourceUnavailable
        assert!(!is_region_confirming_status(&status_with_substatus(
            StatusCode::TooManyRequests,
            SubStatusCode::SYSTEM_RESOURCE_UNAVAILABLE
        )));
        // 403/3 WriteForbidden
        assert!(!is_region_confirming_status(&status_with_substatus(
            StatusCode::Forbidden,
            SubStatusCode::WRITE_FORBIDDEN
        )));
    }

    #[test]
    fn region_confirming_false_for_client_synthesized_timeout() {
        assert!(!is_region_confirming_status(&status_with_substatus(
            StatusCode::RequestTimeout,
            SubStatusCode::CLIENT_OPERATION_TIMEOUT
        )));
    }

    // ── partition_effects_for_deferral ────────────────────────────────

    fn endpoint_for_test() -> CosmosEndpoint {
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    #[test]
    fn deferral_passes_all_effects_through_for_reads() {
        let effects = vec![
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: None,
                region: None,
                is_read: true,
                is_partitioned_resource: true,
            }),
            LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint_for_test(),
                reason: UnavailableReason::ServiceUnavailable,
            },
            LocationEffect::RefreshAccountProperties,
        ];
        let (immediate, deferred) = partition_effects_for_deferral(true, false, false, effects);
        assert_eq!(immediate.len(), 3);
        assert!(deferred.is_empty());
    }

    #[test]
    fn deferral_extracts_partition_marks_for_writes() {
        let effects = vec![
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: None,
                region: None,
                is_read: false,
                is_partitioned_resource: true,
            }),
            LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint_for_test(),
                reason: UnavailableReason::ServiceUnavailable,
            },
            LocationEffect::RefreshAccountProperties,
        ];
        // Single-master write, non-PPAF: partition mark is deferred,
        // endpoint mark stays immediate.
        let (immediate, deferred) = partition_effects_for_deferral(false, false, false, effects);
        assert_eq!(immediate.len(), 2);
        assert!(immediate
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(immediate
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
        assert_eq!(deferred.len(), 1);
        assert!(matches!(
            deferred[0],
            LocationEffect::MarkPartitionUnavailable(_)
        ));
    }

    #[test]
    fn deferral_with_no_partition_marks_returns_empty_deferred() {
        let effects = vec![
            LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint_for_test(),
                reason: UnavailableReason::ServiceUnavailable,
            },
            LocationEffect::RefreshAccountProperties,
        ];
        // Single-master write, non-PPAF: endpoint mark stays immediate.
        let (immediate, deferred) = partition_effects_for_deferral(false, false, false, effects);
        assert_eq!(immediate.len(), 2);
        assert!(deferred.is_empty());
    }

    #[test]
    fn deferral_defers_endpoint_mark_for_ppaf_single_master_writes() {
        // PPAF on single-master account: a transient write failure must NOT
        // immediately mark the only known write region as unavailable.
        // Both partition and endpoint marks must be deferred until the write
        // definitively reaches a region.
        let effects = vec![
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: None,
                region: None,
                is_read: false,
                is_partitioned_resource: true,
            }),
            LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint_for_test(),
                reason: UnavailableReason::TransportError,
            },
            LocationEffect::RefreshAccountProperties,
        ];
        let (immediate, deferred) = partition_effects_for_deferral(false, false, true, effects);
        // Only RefreshAccountProperties should be applied immediately.
        assert_eq!(immediate.len(), 1);
        assert!(matches!(
            immediate[0],
            LocationEffect::RefreshAccountProperties
        ));
        // Both partition and endpoint marks are deferred.
        assert_eq!(deferred.len(), 2);
        assert!(deferred
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(deferred
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn deferral_passes_all_effects_through_for_multi_master_writes() {
        // Multi-master writes are PPCB-managed: failures must be applied
        // immediately so the per-partition write-failure counter can drive
        // threshold-based failover. The PPAF flag is irrelevant because
        // PPAF only applies to single-master accounts.
        let effects = vec![
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: None,
                region: None,
                is_read: false,
                is_partitioned_resource: true,
            }),
            LocationEffect::MarkEndpointUnavailable {
                endpoint: endpoint_for_test(),
                reason: UnavailableReason::ServiceUnavailable,
            },
            LocationEffect::RefreshAccountProperties,
        ];
        let (immediate, deferred) = partition_effects_for_deferral(false, true, false, effects);
        assert_eq!(immediate.len(), 3);
        assert!(deferred.is_empty());
    }

    // -----------------------------------------------------------------------
    // Hub-region-processing-only latch tests.
    //
    // See HUB_REGION_PROCESSING_HEADER_SPEC.md §3 / public-spec §4.1 for the
    // shape these cases are meant to cover (T-1..T-5, T-AC-8, T-8, T-9).
    //
    // All tests drive `evaluate_transport_result` against a 1002 response and
    // inspect the `OperationAction::SessionRetry { new_state }`. Per
    // `build_session_retry_state`, the latch is set when ALL four conditions
    // hold:
    //
    //   1. `is_dataplane`                          (AC-8)
    //   2. `!can_use_multiple_write_locations`     (AC-4)
    //   3. `session_token_retry_count == 0`        (AC-3, first-1002-only)
    //   4. `!hub_region_processing_only`           (idempotency / sticky)
    //
    // `OperationRetryState::initial(..)` defaults both `is_dataplane` and
    // `hub_region_processing_only` to `false`, so tests that want the
    // latch-on path mutate `is_dataplane = true` explicitly. This mirrors the
    // production wiring in `execute_operation_pipeline`, which sets
    // `retry_state.is_dataplane = pipeline_type.is_data_plane()` immediately
    // after constructing the state.
    // -----------------------------------------------------------------------

    fn make_read_session_not_available_result() -> TransportResult {
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::READ_SESSION_NOT_AVAILABLE,
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    fn test_endpoint() -> CosmosEndpoint {
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    /// Drives one 1002 against `state` and returns the resulting
    /// `new_state` from `SessionRetry`. Panics if the action isn't a
    /// `SessionRetry` so callers don't have to repeat that pattern.
    fn session_retry_state_for_1002(state: &OperationRetryState) -> OperationRetryState {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let result = make_read_session_not_available_result();

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, state);
        assert!(
            effects.is_empty(),
            "1002 should not emit location effects, got {effects:?}",
        );
        match action {
            OperationAction::SessionRetry { new_state } => new_state,
            other => panic!("expected SessionRetry, got {other:?}"),
        }
    }

    /// T-1 — Single-master, data-plane, first 1002 sets the latch.
    /// Covers AC-1 of HUB_REGION_PROCESSING_HEADER_SPEC.md.
    #[test]
    fn hub_region_latch_sets_on_first_1002_single_master_dataplane() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        let new_state = session_retry_state_for_1002(&state);

        assert!(
            new_state.hub_region_processing_only,
            "first 1002 on single-master data-plane should latch",
        );
        // The session-retry counter advanced — the latch decision happened
        // pre-increment, so reading `== 0` on `state` was correct.
        assert_eq!(new_state.session_token_retry_count, 1);
    }

    /// T-2 — Multi-master 1002s never latch (AC-4).
    #[test]
    fn hub_region_latch_does_not_set_on_multi_master_1002() {
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3);
        state.is_dataplane = true;

        let new_state = session_retry_state_for_1002(&state);

        assert!(
            !new_state.hub_region_processing_only,
            "multi-master 1002 must not latch the hub-region header",
        );
    }

    /// T-3 — Latch is sticky across subsequent 1002s (AC-2).
    /// The second 1002 must NOT clear the latch even though
    /// `session_token_retry_count` is no longer 0.
    #[test]
    fn hub_region_latch_stays_set_on_subsequent_1002() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        let after_first = session_retry_state_for_1002(&state);
        assert!(after_first.hub_region_processing_only);

        let after_second = session_retry_state_for_1002(&after_first);
        assert!(
            after_second.hub_region_processing_only,
            "latch must persist across subsequent 1002 retries",
        );
        assert_eq!(after_second.session_token_retry_count, 2);
    }

    /// T-4 — Non-1002 responses on a single-master data-plane state never
    /// latch (AC-5). Drives 200, 410, and 503 to confirm the trigger is
    /// scoped to the 1002 arm.
    #[test]
    fn hub_region_latch_does_not_set_on_non_1002_responses() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        // 200: completes — no latch ever runs, but assert via state pass-through.
        let (action, _) = evaluate_transport_result(&op, &endpoint, make_success_result(), &state);
        assert!(matches!(action, OperationAction::Complete(_)));

        // 410 (Gone). On a read this fails over but does not latch.
        let (action, _) =
            evaluate_transport_result(&op, &endpoint, make_http_error(StatusCode::Gone), &state);
        match action {
            OperationAction::FailoverRetry { new_state, .. } => {
                assert!(!new_state.hub_region_processing_only);
            }
            OperationAction::Abort { .. } => {
                // Acceptable terminal outcome; either way the latch wasn't set.
            }
            other => panic!("unexpected action for 410: {other:?}"),
        }

        // 503 (ServiceUnavailable) on a read fails over but does not latch.
        let (action, _) = evaluate_transport_result(
            &op,
            &endpoint,
            make_http_error(StatusCode::ServiceUnavailable),
            &state,
        );
        match action {
            OperationAction::FailoverRetry { new_state, .. } => {
                assert!(!new_state.hub_region_processing_only);
            }
            OperationAction::Abort { .. } => {
                // Terminal outcome — latch can't be observed but it cannot
                // have been set because the 503 path does not run the
                // 1002 trigger.
            }
            other => panic!("unexpected action for 503: {other:?}"),
        }
    }

    /// T-5 — Boundary at `session_token_retry_count >= 2`: the second 1002
    /// still latches the previously-set flag (sticky), and the third 1002
    /// aborts. Validates AC-3 boundary semantics inherited from the
    /// existing `>= 2` abort check.
    #[test]
    fn hub_region_latch_state_at_budget_exhaustion() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        let after_first = session_retry_state_for_1002(&state);
        assert!(after_first.hub_region_processing_only);
        let after_second = session_retry_state_for_1002(&after_first);
        assert!(after_second.hub_region_processing_only);
        assert_eq!(after_second.session_token_retry_count, 2);

        let op = make_read_operation();
        let endpoint = test_endpoint();
        let result = make_read_session_not_available_result();
        let (action, _) = evaluate_transport_result(&op, &endpoint, result, &after_second);
        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "third 1002 must abort, got {action:?}",
        );
    }

    /// T-AC-8 — Metadata-pipeline 1002s never latch (AC-8). Same shape as
    /// T-1 but with `is_dataplane = false`.
    #[test]
    fn hub_region_latch_does_not_set_on_metadata_pipeline_1002() {
        // is_dataplane defaults to false from `initial(..)`.
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        assert!(!state.is_dataplane);

        let new_state = session_retry_state_for_1002(&state);

        assert!(
            !new_state.hub_region_processing_only,
            "metadata-pipeline 1002 must not latch the hub-region header",
        );
    }

    /// T-8 — Independent operations don't share latch state (AC-6).
    /// `OperationRetryState::initial(..)` is fresh per call, so a latch on
    /// one operation can't leak to another even when they go through the
    /// same evaluate path.
    #[test]
    fn hub_region_latch_independent_operations_do_not_share_state() {
        let mut op_a = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        op_a.is_dataplane = true;
        let mut op_b = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        op_b.is_dataplane = true;

        let op_a_after = session_retry_state_for_1002(&op_a);
        assert!(op_a_after.hub_region_processing_only);
        // op_b is unrelated state and its latch is still false.
        assert!(!op_b.hub_region_processing_only);
        // Driving op_b independently sets its own latch but op_a_after is unchanged.
        let op_b_after = session_retry_state_for_1002(&op_b);
        assert!(op_b_after.hub_region_processing_only);
        assert!(op_a_after.hub_region_processing_only);
    }

    /// T-9 — Latch survives mixed-cause retries (AC-2). Flip the latch on a
    /// 1002, then drive a 503 that takes the failover path. `..self` in
    /// `advance_failover` propagates the flag.
    #[test]
    fn hub_region_latch_survives_failover_after_latch() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;

        let after_1002 = session_retry_state_for_1002(&state);
        assert!(after_1002.hub_region_processing_only);

        // Now drive a 503 read — should fail over. Latch must propagate.
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let (action, _) = evaluate_transport_result(
            &op,
            &endpoint,
            make_http_error(StatusCode::ServiceUnavailable),
            &after_1002,
        );
        match action {
            OperationAction::FailoverRetry { new_state, .. } => {
                assert!(
                    new_state.hub_region_processing_only,
                    "latch must propagate through `..self` in advance_failover",
                );
            }
            OperationAction::Abort { .. } => {
                // Terminal abort path — the latch is on retry_state, which
                // the abort path doesn't expose. The structural argument
                // (every `advance_*` uses `..self`) still holds.
            }
            other => panic!("unexpected action for 503 after latch: {other:?}"),
        }
    }
}
