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

use std::sync::atomic::Ordering;

use super::components::{
    OperationAction, OperationRetryState, TransportOutcome, TransportResult,
    BACKEND_FAILOVER_RETRY_INTERVAL,
};
#[cfg(feature = "preview_dtx")]
use super::components::{
    DTX_COORDINATOR_RETRY_INTERVAL, DTX_INFRA_BASE_BACKOFF, DTX_INFRA_MAX_BACKOFF,
    DTX_INFRA_MAX_EXPONENT,
};

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

    if status.is_write_forbidden() || status.is_database_account_not_found() {
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

/// Side effects observed by a single hedge leg, returned by
/// [`evaluate_hedge_leg_effects`] so the race coordinator can mirror the
/// non-hedged [`evaluate_transport_result`] path without consuming the
/// `TransportResult`.
#[derive(Debug, Default)]
pub(crate) struct HedgeLegEvaluation {
    pub(crate) effects: Vec<LocationEffect>,
    /// `true` when this leg observed a 404/1002 that would have triggered
    /// [`build_session_retry_state`]'s `hub_region_processing_only` latch
    /// on the non-hedged path.
    pub(crate) observed_session_unavailable: bool,
}

/// Non-consuming counterpart of [`evaluate_transport_result`] for the
/// hedge race loop. Returns the same `LocationEffect`s the consuming
/// path would have emitted, plus an `observed_session_unavailable` bool
/// in place of the 404/1002 `OperationRetryState` transition (the race
/// coordinator applies the latch flip at the `BothTransient` boundary).
/// The `OperationAction` is discarded since `classify_hedge_result`
/// picks the next-step action.
pub(crate) fn evaluate_hedge_leg_effects(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    result: &TransportResult,
) -> HedgeLegEvaluation {
    let mut eval = HedgeLegEvaluation::default();
    match &result.outcome {
        TransportOutcome::Success { .. } => {}

        TransportOutcome::HttpError {
            status,
            request_sent,
            ..
        } => {
            // Mirror `build_session_retry_state`'s four-condition latch
            // trigger.
            if status.is_read_session_not_available()
                && retry_state.can_retry_session()
                && retry_state.is_dataplane
                && !retry_state.can_use_multiple_write_locations
                && retry_state.session_token_retry_count == 0
                && !retry_state.hub_region_processing_only
            {
                eval.observed_session_unavailable = true;
            }

            // Walk `evaluate_http_outcome`'s priority chain by-reference
            // and take only the `LocationEffect`s. 404/1002 is handled
            // above via `observed_session_unavailable` and emits no
            // effects, so it is skipped here.
            if let Some((_action, effects)) =
                try_handle_write_forbidden(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            } else if let Some((_action, effects)) =
                try_handle_database_account_not_found(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            } else if let Some((_action, effects)) = try_handle_retry_trigger_group(
                operation,
                endpoint,
                retry_state,
                status,
                *request_sent,
            ) {
                eval.effects = effects;
            } else if let Some((_action, effects)) =
                try_handle_server_error(operation, endpoint, retry_state, status)
            {
                eval.effects = effects;
            }
        }

        TransportOutcome::TransportError { request_sent, .. } => {
            // Mirrors `evaluate_transport_layer_outcome`: `definitely_not_sent`
            // emits no effects; `sent` marks the partition unavailable and
            // (when PPCB is not managing failover) the endpoint too.
            if !request_sent.definitely_not_sent() {
                eval.effects.push(LocationEffect::MarkPartitionUnavailable(
                    make_partition_unavailable(
                        operation,
                        endpoint,
                        retry_state,
                        operation.is_read_only(),
                    ),
                ));
                if !is_ppcb_managed(operation, retry_state) {
                    eval.effects.push(LocationEffect::MarkEndpointUnavailable {
                        endpoint: endpoint.clone(),
                        reason: UnavailableReason::TransportError,
                    });
                }
            }
        }

        TransportOutcome::DeadlineExceeded { .. } => {
            // Client-side timeout — no routing-state effects.
        }
    }
    eval
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

    // 403/1008 DatabaseAccountNotFound is a topology-divergence signal that
    // applies to every operation type, including writes (PR #4590): the region
    // no longer owns the account, so the request must refresh account
    // properties and fail over rather than surface a stale-topology error.
    // Run it *before* the DTX short-circuit (mirroring 403/3 WriteForbidden
    // above) so a distributed transaction still recovers topology instead of
    // completing the raw 403 up to the coordinator loop.
    if let Some(result) =
        try_handle_database_account_not_found(operation, endpoint, retry_state, &status)
    {
        return result;
    }

    #[cfg(feature = "preview_dtx")]
    if operation.resource_type() == crate::models::ResourceType::DistributedTransactionBatch {
        return evaluate_dtx_http_outcome(status, cosmos_headers, body, retry_state);
    }

    if let Some(result) =
        try_handle_read_session_not_available(retry_state, &status, &cosmos_headers, &body)
    {
        return result;
    }

    if let Some(result) =
        try_handle_retry_trigger_group(operation, endpoint, retry_state, &status, request_sent)
    {
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

#[cfg(feature = "preview_dtx")]
fn evaluate_dtx_http_outcome(
    status: CosmosStatus,
    cosmos_headers: CosmosResponseHeaders,
    body: Vec<u8>,
    retry_state: &OperationRetryState,
) -> (OperationAction, Vec<LocationEffect>) {
    if body.is_empty() {
        if let Some((new_state, delay)) =
            try_dtx_bodyless_retry(&status, &cosmos_headers, retry_state)
        {
            return (OperationAction::DtxRetry { new_state, delay }, Vec::new());
        }
    }

    // A DTX coordinator response that is not retried here is handed to the outer
    // loop as a transport-level *delivery* success: the body reached us intact.
    // This is NOT a DTX success — the real per-operation outcome (including
    // `452`/`500`/etc.) is (re)derived by `DistributedTransactionResponse::from_body`
    // downstream. Wrapping it as `Success` here just means "deliver the body up".
    (
        OperationAction::Complete(Box::new(TransportResult {
            outcome: TransportOutcome::Success {
                status,
                cosmos_headers,
                body,
            },
        })),
        Vec::new(),
    )
}

#[cfg(feature = "preview_dtx")]
fn try_dtx_bodyless_retry(
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    retry_state: &OperationRetryState,
) -> Option<(OperationRetryState, std::time::Duration)> {
    if is_dtx_bodyless_coordinator_retriable(status) {
        if !retry_state.can_retry_dtx_coordinator() {
            return None;
        }

        let delay = cosmos_headers
            .retry_after_ms
            .map(std::time::Duration::from_millis)
            .unwrap_or(DTX_COORDINATOR_RETRY_INTERVAL);
        return Some((retry_state.clone().advance_dtx_coordinator_retry(), delay));
    }

    if is_dtx_bodyless_infra_retriable(status) {
        if !retry_state.can_retry_dtx_infra() {
            return None;
        }

        let delay = dtx_infra_retry_delay(retry_state.dtx_infra_retry_count);
        return Some((retry_state.clone().advance_dtx_infra_retry(), delay));
    }

    None
}

#[cfg(feature = "preview_dtx")]
fn is_dtx_bodyless_coordinator_retriable(status: &CosmosStatus) -> bool {
    status.status_code() == azure_core::http::StatusCode::RequestTimeout
        || (u16::from(status.status_code()) == 449
            && status.sub_status() == Some(SubStatusCode::DTC_COORDINATOR_RACE_CONFLICT))
}

#[cfg(feature = "preview_dtx")]
fn is_dtx_bodyless_infra_retriable(status: &CosmosStatus) -> bool {
    status.status_code() == azure_core::http::StatusCode::InternalServerError
        && matches!(
            status.sub_status(),
            Some(SubStatusCode::DTC_LEDGER_FAILURE)
                | Some(SubStatusCode::DTC_ACCOUNT_CONFIG_FAILURE)
                | Some(SubStatusCode::DTC_DISPATCH_FAILURE)
        )
}

#[cfg(feature = "preview_dtx")]
fn dtx_infra_retry_delay(attempt: u32) -> std::time::Duration {
    let exponent = attempt.min(DTX_INFRA_MAX_EXPONENT);
    let delay = DTX_INFRA_BASE_BACKOFF.mul_f64(2_f64.powi(exponent as i32));
    delay.min(DTX_INFRA_MAX_BACKOFF)
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
    if !status.is_write_forbidden() {
        return None;
    }

    // Multi-write 403/3 gets the larger backend-failover budget; single-write uses the generic budget.
    let (new_state, delay) = if retry_state.can_use_multiple_write_locations {
        if !retry_state.can_retry_backend_failover() {
            return None;
        }
        (
            retry_state.clone().advance_backend_failover(),
            Some(BACKEND_FAILOVER_RETRY_INTERVAL),
        )
    } else {
        if !retry_state.can_retry_failover() {
            return None;
        }
        (retry_state.clone().advance_failover(), None)
    };

    let mut effects = vec![
        LocationEffect::RefreshAccountProperties,
        LocationEffect::MarkPartitionUnavailable(make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            false,
        )),
    ];
    if !is_ppcb_managed(operation, retry_state) {
        // PPCB-managed 403/3 is per-partition; do not block the whole endpoint for 60s.
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::WriteForbidden,
        });
    }
    Some((OperationAction::FailoverRetry { new_state, delay }, effects))
}

/// Handles 403/1008 DatabaseAccountNotFound for all operation types.
///
/// The region no longer owns the account; refresh topology and fail over with bounded retries.
fn try_handle_database_account_not_found(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, Vec<LocationEffect>)> {
    if !status.is_database_account_not_found() {
        return None;
    }

    if !retry_state.can_retry_backend_failover() {
        return None;
    }
    let new_state = retry_state.clone().advance_backend_failover();
    let delay = Some(BACKEND_FAILOVER_RETRY_INTERVAL);

    let mut effects = vec![
        LocationEffect::RefreshAccountProperties,
        LocationEffect::MarkPartitionUnavailable(make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            operation.is_read_only(),
        )),
    ];
    if !is_ppcb_managed(operation, retry_state) {
        // PPCB-managed 1008 is per-partition; do not block the whole endpoint for 60s.
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::DatabaseAccountNotFound,
        });
    }
    Some((OperationAction::FailoverRetry { new_state, delay }, effects))
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
/// **Hedging coordination (future).** When
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
        // Cross-hedge propagation. When this
        // operation is running inside `execute_hedged` the shared
        // `Arc<AtomicBool>` lets sibling hedges discover the 1002 latch
        // without re-running the 404/1002 cycle themselves. `Release`
        // ordering pairs with the `Acquire` load in `apply_hub_region_header`
        // — publishes the bool, which is the only datum being shared.
        if let Some(shared) = new_state.shared_hub_region_latch.as_ref() {
            shared.store(true, Ordering::Release);
        }
    }
    new_state
}

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
fn try_handle_retry_trigger_group(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
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

/// Handles transport-layer errors where no HTTP response was produced.
///
/// Not-sent marks the endpoint only; sent/unknown marks the partition only, matching .NET/Python.
fn evaluate_transport_layer_outcome(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: CosmosStatus,
    error: crate::error::CosmosError,
    request_sent: RequestSentStatus,
) -> (OperationAction, Vec<LocationEffect>) {
    if request_sent.definitely_not_sent() {
        // Not-sent means endpoint-wide failure; do not inflate PPCB's partition counter.
        let effects = vec![LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::TransportError,
        }];

        if retry_state.can_retry_failover() {
            return (
                OperationAction::FailoverRetry {
                    new_state: retry_state.clone().advance_failover(),
                    delay: None,
                },
                effects,
            );
        }

        return (
            OperationAction::Abort {
                error: build_transport_error(&status, error),
            },
            effects,
        );
    }

    // Request was sent (or unknown) — the endpoint is reachable, but this
    // partition had an issue. Only mark the partition; do NOT mark the
    // endpoint since other partitions on it are unaffected.
    let effects = vec![LocationEffect::MarkPartitionUnavailable(
        make_partition_unavailable(operation, endpoint, retry_state, operation.is_read_only()),
    )];

    if retry_state.can_retry_failover() {
        return (
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            effects,
        );
    }

    // Budget exhausted — no more failover attempts available.
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
    // `crate::error::CosmosError` directly via `OperationAction::Abort.error`.
    let cosmos_err = crate::error::CosmosError::builder()
        .with_status(CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(crate::models::SubStatusCode::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message(message)
        .build();

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

/// Builds a typed [`CosmosError`] for a Cosmos HTTP error response.
///
/// Captures the parsed response headers and the raw response body bytes
/// (e.g. the JSON error payload returned by the service for a 400 /
/// BadRequest) on the resulting `CosmosError`. The error propagates through the
/// pipeline as `crate::error::CosmosError` end-to-end. Callers inspect the wire
/// payload directly via [`CosmosError::status`](crate::error::CosmosError::status),
/// [`CosmosError::cosmos_headers`](crate::error::CosmosError::cosmos_headers), and
/// [`CosmosError::response_body`](crate::error::CosmosError::response_body).
///
/// The returned error carries **no** `DiagnosticsContext`. The operation
/// pipeline's abort branch (the only production caller of this helper, via
/// [`OperationAction::Abort`]) grafts the completed operation diagnostics
/// onto the error via `CosmosError::builder().from_error(err).with_diagnostics(ctx).build()`
/// before it leaves the pipeline. Keeping this module free of any diagnostics plumbing preserves
/// `evaluate_transport_result` as a pure function over its inputs and
/// avoids constructing a throw-away diagnostics value that would
/// immediately be overwritten downstream.
pub(crate) fn build_service_error(
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
) -> crate::error::CosmosError {
    // Some gateway versions return HTTP 400 for cross-partition queries with
    // unsupported features (ORDER BY, aggregates, GROUP BY, ...) without
    // emitting the `x-ms-substatus: 1004` header that the .NET / Java SDKs
    // rely on. Detect that case from the response body and synthesize the
    // canonical [`CosmosStatus::CROSS_PARTITION_QUERY_NOT_SERVABLE`] so
    // callers get a consistent typed status regardless of gateway version.
    let effective_status = synthesize_cross_partition_query_status(*status, body);
    crate::error::CosmosError::builder()
        .with_status(effective_status)
        .with_message(service_error_message(&effective_status))
        .with_response_parts(crate::models::CosmosResponsePayload::new(
            body.to_vec(),
            cosmos_headers.clone(),
        ))
        .build()
}

/// Returns [`CosmosStatus::CROSS_PARTITION_QUERY_NOT_SERVABLE`] when `status`
/// is a bare HTTP 400 (no sub-status) and `body` is the gateway's
/// "unsupported query features" rejection. Otherwise returns `status`
/// unchanged.
fn synthesize_cross_partition_query_status(status: CosmosStatus, body: &[u8]) -> CosmosStatus {
    use azure_core::http::StatusCode;
    if status.status_code() != StatusCode::BadRequest || status.sub_status().is_some() {
        return status;
    }
    let Ok(text) = std::str::from_utf8(body) else {
        return status;
    };

    // Match the gateway's well-known message rather than parsing JSON to
    // avoid a serde dependency on the hot error path. The fragment is
    // stable across .NET / Java / Python emulator gateways.
    if text.contains("unsupported features") && text.contains("Upgrade your SDK") {
        crate::error::CosmosStatus::CROSS_PARTITION_QUERY_NOT_SERVABLE
    } else {
        status
    }
}

fn build_transport_error(
    status: &CosmosStatus,
    error: crate::error::CosmosError,
) -> crate::error::CosmosError {
    let status_code = status.status_code();
    let name = status.name().unwrap_or("Unknown");
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };

    let detail_summary = crate::driver::error_chain_summary(&error);
    let message = format!(
        "Cosmos DB transport failure HTTP {}{}: {}. Details: {}",
        u16::from(status_code),
        sub_status_str,
        name,
        detail_summary,
    );

    // Wrap into a fresh transport-kind error carrying the enriched message
    // and the original Cosmos error as source. Forward the inner error's
    // diagnostics so `outer.diagnostics()` is not silently `None` — callers
    // should not have to walk `source()` to recover the operation's
    // diagnostic context.
    let mut b = crate::error::CosmosError::builder()
        .with_status(*status)
        .with_message(message)
        .with_arc_source(std::sync::Arc::new(error.clone()));
    if let Some(diag) = error.diagnostics() {
        b = b.with_diagnostics(diag);
    }
    b.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::RequestSentStatus,
        models::{
            AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
            CosmosResponseHeaders, CosmosStatus, DatabaseReference, ItemReference, PartitionKey,
            PartitionKeyDefinition, SystemProperties,
        },
    };
    use azure_core::http::StatusCode;

    #[cfg(feature = "preview_dtx")]
    use super::super::components::{MAX_DTX_COORDINATOR_RETRIES, MAX_DTX_INFRA_RETRIES};

    fn make_create_item_operation() -> CosmosOperation {
        let account = AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "testcontainer".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &props,
        );
        let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
        CosmosOperation::create_item(item).with_body(b"{}".to_vec())
    }

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

    #[cfg(feature = "preview_dtx")]
    fn make_dtx_operation() -> CosmosOperation {
        make_dtx_operation_for(crate::models::DistributedTransactionType::Write)
    }

    #[cfg(feature = "preview_dtx")]
    fn make_dtx_operation_for(
        transaction_type: crate::models::DistributedTransactionType,
    ) -> CosmosOperation {
        let account = AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        CosmosOperation::distributed_transaction(account, transaction_type)
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
                error: crate::error::CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                    .with_message("connection refused")
                    .build(),
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

    #[cfg(feature = "preview_dtx")]
    fn make_dtx_http_error(
        status: CosmosStatus,
        body: Vec<u8>,
        retry_after_ms: Option<u64>,
    ) -> TransportResult {
        let cosmos_headers = CosmosResponseHeaders {
            retry_after_ms,
            ..Default::default()
        };
        TransportResult {
            outcome: TransportOutcome::HttpError {
                status,
                cosmos_headers,
                body,
                request_sent: RequestSentStatus::Sent,
            },
        }
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_449_5352_uses_coordinator_retry_budget() {
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(
                StatusCode::from(449_u16),
                Some(SubStatusCode::DTC_COORDINATOR_RACE_CONFLICT),
            ),
            Vec::new(),
            Some(250),
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(effects.is_empty());
        match action {
            OperationAction::DtxRetry { new_state, delay } => {
                assert_eq!(new_state.dtx_coordinator_retry_count, 1);
                assert_eq!(new_state.dtx_infra_retry_count, 0);
                assert_eq!(delay, std::time::Duration::from_millis(250));
            }
            other => panic!("expected DtxRetry, got {other:?}"),
        }
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_500_5411_uses_infra_retry_budget() {
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(
                StatusCode::InternalServerError,
                Some(SubStatusCode::DTC_LEDGER_FAILURE),
            ),
            Vec::new(),
            None,
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(effects.is_empty());
        match action {
            OperationAction::DtxRetry { new_state, delay } => {
                assert_eq!(new_state.dtx_coordinator_retry_count, 0);
                assert_eq!(new_state.dtx_infra_retry_count, 1);
                assert_eq!(delay, std::time::Duration::from_millis(100));
            }
            other => panic!("expected DtxRetry, got {other:?}"),
        }
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_retry_budget_exhaustion_completes_response() {
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(
                StatusCode::from(449_u16),
                Some(SubStatusCode::DTC_COORDINATOR_RACE_CONFLICT),
            ),
            Vec::new(),
            None,
        );
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.dtx_coordinator_retry_count = MAX_DTX_COORDINATOR_RETRIES;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(effects.is_empty());
        assert!(matches!(action, OperationAction::Complete(_)));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_infra_retry_budget_exhaustion_completes_response() {
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(
                StatusCode::InternalServerError,
                Some(SubStatusCode::DTC_LEDGER_FAILURE),
            ),
            Vec::new(),
            None,
        );
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.dtx_infra_retry_count = MAX_DTX_INFRA_RETRIES;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(effects.is_empty());
        assert!(matches!(action, OperationAction::Complete(_)));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_body_bearing_449_completes_for_outer_loop() {
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(StatusCode::from(449_u16), Some(SubStatusCode::UNKNOWN)),
            br#"{"isRetriable":true}"#.to_vec(),
            None,
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(effects.is_empty());
        assert!(matches!(action, OperationAction::Complete(_)));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_write_forbidden_refreshes_topology_before_dtx_classification() {
        for transaction_type in [
            crate::models::DistributedTransactionType::Write,
            crate::models::DistributedTransactionType::Read,
        ] {
            let op = make_dtx_operation_for(transaction_type);
            let result = make_dtx_http_error(
                CosmosStatus::from_parts(StatusCode::Forbidden, Some(SubStatusCode::new(3))),
                Vec::new(),
                None,
            );
            let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
            let endpoint = CosmosEndpoint::global(
                url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            );

            let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

            assert!(
                matches!(action, OperationAction::FailoverRetry { .. }),
                "{transaction_type:?} DTX 403/3 should failover-retry"
            );
            assert!(
                effects
                    .iter()
                    .any(|effect| matches!(effect, LocationEffect::RefreshAccountProperties)),
                "{transaction_type:?} DTX 403/3 should refresh topology"
            );
            assert!(
                effects
                    .iter()
                    .any(|effect| matches!(effect, LocationEffect::MarkPartitionUnavailable(_))),
                "{transaction_type:?} DTX 403/3 should mark partition unavailable"
            );
        }
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_database_account_not_found_refreshes_topology_before_dtx_classification() {
        // 403/1008 DatabaseAccountNotFound applies to every op type, including
        // DTX writes (PR #4590): it must refresh account properties and fail
        // over rather than being swallowed by the DTX classification into a
        // stale-topology `Complete`.
        let op = make_dtx_operation();
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(StatusCode::Forbidden, Some(SubStatusCode::new(1008))),
            Vec::new(),
            None,
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);

        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|effect| matches!(effect, LocationEffect::RefreshAccountProperties)));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_http_outcomes_never_failover_session_or_hedge() {
        // DTX is a write-bearing resource routed through the shared pipeline.
        // The DTX classification in `evaluate_http_outcome` keeps DTX responses
        // out of the cross-region failover / session-retry / hedging machinery,
        // which is unsafe for writes (see PR #4432). This pins that guard except
        // for 403/3 WriteForbidden and 403/1008 DatabaseAccountNotFound, which
        // must refresh topology first (both are handled before the DTX
        // short-circuit). By the time a DTX `429` reaches this classifier, the
        // transport-level throttle retry path has already propagated it (for
        // example, after exhausting throttle budget). Every other coordinator HTTP
        // outcome must resolve to `DtxRetry` (bodyless coordinator/infra retry) or
        // `Complete` (body handed to the outer coordinator loop) — never
        // `FailoverRetry`, `SessionRetry`, or `Hedge`, and never a location effect.
        let op = make_dtx_operation();
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        // Statuses/sub-statuses that drive failover / session-retry / abort for a
        // normal (non-DTX) operation.
        let statuses = [
            CosmosStatus::new(StatusCode::from(503_u16)),
            CosmosStatus::from_parts(StatusCode::from(429_u16), Some(SubStatusCode::new(3092))),
            CosmosStatus::from_parts(StatusCode::from(404_u16), Some(SubStatusCode::new(1002))),
            CosmosStatus::from_parts(StatusCode::from(449_u16), Some(SubStatusCode::UNKNOWN)),
            CosmosStatus::new(StatusCode::from(408_u16)),
            CosmosStatus::new(StatusCode::from(410_u16)),
            CosmosStatus::new(StatusCode::from(500_u16)),
        ];

        for status in statuses {
            let code = u16::from(status.status_code());
            for body in [Vec::new(), b"{}".to_vec()] {
                let body_len = body.len();
                let result = make_dtx_http_error(status, body, None);
                let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
                assert!(
                    effects.is_empty(),
                    "DTX status {code} (body_len={body_len}) emitted location effects",
                );
                assert!(
                    matches!(
                        action,
                        OperationAction::DtxRetry { .. } | OperationAction::Complete(_)
                    ),
                    "DTX status {code} (body_len={body_len}) produced {action:?}",
                );
            }
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
        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        // Not-sent marks the endpoint only; Gateway connect failures are not partition-specific.
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
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
        // Sent → endpoint is reachable, only mark partition (not endpoint).
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn transport_error_sent_non_idempotent_retries() {
        let op = make_create_operation();
        let result = make_transport_error(RequestSentStatus::Sent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        // Sent → endpoint is reachable, only mark partition (not endpoint).
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn build_transport_error_forwards_inner_diagnostics() {
        // Wrapping must preserve the inner diagnostics `Arc`.
        let diag: std::sync::Arc<crate::diagnostics::DiagnosticsContext> = std::sync::Arc::new(
            crate::diagnostics::DiagnosticsContextBuilder::new(
                crate::models::ActivityId::new_uuid(),
                std::sync::Arc::new(crate::options::DiagnosticsOptions::default()),
            )
            .complete(),
        );
        let inner = crate::error::CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("inner transport failure")
            .with_diagnostics(std::sync::Arc::clone(&diag))
            .build();

        let outer = build_transport_error(&CosmosStatus::TRANSPORT_GENERATED_503, inner);

        let outer_diag = outer
            .diagnostics()
            .expect("outer error must inherit inner diagnostics");
        assert!(
            std::sync::Arc::ptr_eq(&outer_diag, &diag),
            "outer diagnostics must be the same Arc as the inner's"
        );
    }

    #[test]
    fn transport_abort_error_includes_status_kind_and_details() {
        let op = make_create_operation();
        let result = TransportResult {
            outcome: TransportOutcome::TransportError {
                status: CosmosStatus::TRANSPORT_GENERATED_503,
                error: crate::error::CosmosError::builder()
                    .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                    .with_message("failed to execute `reqwest` request")
                    .with_source(std::io::Error::new(
                        std::io::ErrorKind::BrokenPipe,
                        "socket reset",
                    ))
                    .build(),
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
                // `.status()` compiling proves this stayed a typed `CosmosError`.
                assert_eq!(error.status(), CosmosStatus::TRANSPORT_GENERATED_503);
                let text = error.to_string();
                assert!(text.contains("HTTP 503/20003"));
                assert!(text.contains("TransportGenerated503"));
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
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 1,
            max_backend_failover_retries: 120,
            max_session_retries: 1,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
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
        // PPCB disabled (default) → endpoint mark is emitted so non-PPCB
        // accounts still get region-wide failover for the 60 s TTL.
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
            "non-PPCB 403/3 must mark the endpoint unavailable"
        );
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
            "403/3 must always mark the partition unavailable"
        );
    }

    #[test]
    fn write_forbidden_when_ppcb_managed_skips_endpoint_mark() {
        // PPCB-managed 403/3 must not mark the whole endpoint unavailable.
        let op = make_create_item_operation();
        let mut state =
            OperationRetryState::initial(0, true /* multi-write */, Vec::new(), 3, 1);
        state.ppcb_active = true;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::WRITE_FORBIDDEN),
            &state,
        );

        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
            "403/3 must still refresh account properties"
        );
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
            "PPCB-managed 403/3 must still mark the partition unavailable"
        );
        assert!(
            !effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
            "PPCB-managed 403/3 must NOT mark the endpoint unavailable; \
             per-partition counter drives failover"
        );
    }

    fn http_error_status(status: CosmosStatus) -> TransportResult {
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
    fn database_account_not_found_on_write_emits_refresh_mark_endpoint_and_failover() {
        // 403/1008 must refresh topology, mark endpoint/partition, and failover-retry.
        let op = make_create_operation();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        assert!(
            matches!(
                action,
                OperationAction::FailoverRetry { delay: Some(_), .. }
            ),
            "expected FailoverRetry with backend-failover delay, got {:?}",
            action
        );
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
            "missing RefreshAccountProperties effect; effects={:?}",
            effects
        );
        assert!(
            effects.iter().any(|e| matches!(
                e,
                LocationEffect::MarkEndpointUnavailable {
                    reason: UnavailableReason::DatabaseAccountNotFound,
                    ..
                }
            )),
            "missing MarkEndpointUnavailable{{DatabaseAccountNotFound}}; effects={:?}",
            effects
        );
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
            "missing MarkPartitionUnavailable; effects={:?}",
            effects
        );
    }

    #[test]
    fn database_account_not_found_on_read_uses_op_read_only_for_partition_mark() {
        // 1008 fires on reads too; partition marks must preserve operation direction.
        let op = make_read_operation();
        let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        let mark = effects.iter().find_map(|e| match e {
            LocationEffect::MarkPartitionUnavailable(p) => Some(p),
            _ => None,
        });
        let mark = mark.expect("expected MarkPartitionUnavailable for 1008 on read");
        assert!(
            mark.is_read,
            "1008 on a read op must mark the partition with is_read=true so PPCB \
             credits the read-direction failure counter, not the write counter"
        );
    }

    #[test]
    fn database_account_not_found_aborts_when_backend_failover_budget_exhausted() {
        // Exhausting the 1008 budget must bubble up the original backend status.
        let op = make_create_operation();
        // Multi-write account so the handler uses the dedicated
        // backend-failover budget rather than the generic one.
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        // Drive the backend-failover counter directly to the cap rather than
        // looping 120× through `advance_backend_failover`.
        state.backend_failover_retry_count = state.max_backend_failover_retries;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        match action {
            OperationAction::Abort { error } => {
                assert_eq!(
                    error.status(),
                    CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND,
                    "1008 exhausted-budget bubble-up must surface the original status unchanged"
                );
            }
            other => panic!(
                "expected Abort once backend-failover budget is exhausted, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn write_forbidden_aborts_when_backend_failover_budget_exhausted() {
        // Exhausting the 403/3 backend-failover budget must bubble up the original status.
        let op = make_create_operation();
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        state.backend_failover_retry_count = state.max_backend_failover_retries;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::WRITE_FORBIDDEN),
            &state,
        );

        match action {
            OperationAction::Abort { error } => {
                assert_eq!(
                    error.status(),
                    CosmosStatus::WRITE_FORBIDDEN,
                    "403/3 exhausted-budget bubble-up must surface the original status unchanged"
                );
            }
            other => panic!("expected Abort, got {:?}", other),
        }
    }

    #[test]
    fn database_account_not_found_does_not_consume_generic_failover_budget() {
        // Backend-failover retries must not consume the generic failover budget.
        let op = make_create_operation();
        let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        match action {
            OperationAction::FailoverRetry { new_state, delay } => {
                assert_eq!(new_state.failover_retry_count, 0);
                assert_eq!(new_state.backend_failover_retry_count, 1);
                assert_eq!(
                    delay,
                    Some(BACKEND_FAILOVER_RETRY_INTERVAL),
                    "multi-write 1008 must pace retries with BACKEND_FAILOVER_RETRY_INTERVAL"
                );
            }
            other => panic!("expected FailoverRetry, got {:?}", other),
        }
    }

    #[test]
    fn write_forbidden_does_not_consume_generic_failover_budget() {
        // Mirror of the 1008 test for 403/3 on a multi-write account.
        let op = make_create_operation();
        let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::WRITE_FORBIDDEN),
            &state,
        );

        match action {
            OperationAction::FailoverRetry { new_state, delay } => {
                assert_eq!(new_state.failover_retry_count, 0);
                assert_eq!(new_state.backend_failover_retry_count, 1);
                assert_eq!(
                    delay,
                    Some(BACKEND_FAILOVER_RETRY_INTERVAL),
                    "multi-write 403/3 must pace retries with BACKEND_FAILOVER_RETRY_INTERVAL"
                );
            }
            other => panic!("expected FailoverRetry, got {:?}", other),
        }
    }

    #[test]
    fn write_forbidden_on_single_write_uses_generic_failover_budget() {
        // Single-write 403/3 uses the generic budget, not the multi-write rotation budget.
        let op = make_create_operation();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::WRITE_FORBIDDEN),
            &state,
        );

        match action {
            OperationAction::FailoverRetry { new_state, delay } => {
                assert_eq!(new_state.failover_retry_count, 1);
                assert_eq!(new_state.backend_failover_retry_count, 0);
                assert_eq!(
                    delay, None,
                    "single-write 403/3 uses the generic budget and must not pace retries"
                );
            }
            other => panic!("expected FailoverRetry, got {:?}", other),
        }
    }

    #[test]
    fn write_forbidden_on_single_write_aborts_when_generic_budget_exhausted() {
        // Single-write 403/3 must bubble up once the generic budget is exhausted.
        let op = make_create_operation();
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.failover_retry_count = state.max_failover_retries;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::WRITE_FORBIDDEN),
            &state,
        );

        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "single-write 403/3 must abort once generic failover budget is exhausted, got {:?}",
            action
        );
    }

    #[test]
    fn database_account_not_found_on_single_write_uses_backend_failover_budget() {
        // Single-write 1008 also uses backend-failover budget + delay; the topology-not-found
        // signal is identical to multi-write and needs the same convergence window.
        let op = make_create_operation();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, _effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        match action {
            OperationAction::FailoverRetry { new_state, delay } => {
                assert_eq!(new_state.failover_retry_count, 0);
                assert_eq!(new_state.backend_failover_retry_count, 1);
                assert_eq!(
                    delay,
                    Some(BACKEND_FAILOVER_RETRY_INTERVAL),
                    "single-write 1008 must pace retries with BACKEND_FAILOVER_RETRY_INTERVAL"
                );
            }
            other => panic!("expected FailoverRetry, got {:?}", other),
        }
    }

    #[test]
    fn database_account_not_found_when_ppcb_managed_skips_endpoint_mark() {
        // PPCB-managed 1008 must not mark the whole endpoint unavailable;
        // the per-partition counter drives failover.
        let op = make_create_item_operation();
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
        state.ppcb_active = true;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(
            &op,
            &endpoint,
            http_error_status(CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND),
            &state,
        );

        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
            "1008 must still refresh account properties"
        );
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
            "PPCB-managed 1008 must still mark the partition unavailable"
        );
        assert!(
            !effects
                .iter()
                .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
            "PPCB-managed 1008 must NOT mark the endpoint unavailable; \
             per-partition counter drives failover"
        );
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

    /// Regression guard: a cross-region hedge
    /// can only be spawned from a region-changing retry action, and for a
    /// 429 that path is gated to sub-status `3092`
    /// (`SystemResourceUnavailable`). Every other throttle sub-status —
    /// `3200` (`RU_BUDGET_EXCEEDED`), `3210` (`RU_BUDGET_EXCEEDED_FOR_MASTER`),
    /// and `3214` (`HOT_PARTITION_KEY_THROTTLED`) — must instead `Abort`:
    /// failing them over to another region cannot conjure throughput or
    /// cool a hot logical partition and only spreads the throttle. Pinning
    /// this here ensures a future widening of the retry-trigger group
    /// cannot silently begin hedging RU-exhaustion or hot-partition throttles.
    #[test]
    fn throttle_substatus_gates_hedge_eligibility() {
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let throttle_result = |sub: SubStatusCode| TransportResult {
            outcome: TransportOutcome::HttpError {
                status: CosmosStatus::from_parts(StatusCode::TooManyRequests, Some(sub)),
                cosmos_headers: CosmosResponseHeaders::default(),
                body: vec![],
                request_sent: RequestSentStatus::Sent,
            },
        };

        // 429/3092 — transient backend pressure → failover-eligible, hence
        // the only 429 the hedge upgrade in `maybe_upgrade_to_hedge` may act on.
        let op = make_read_operation();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let (action, _) = evaluate_transport_result(
            &op,
            &endpoint,
            throttle_result(SubStatusCode::SYSTEM_RESOURCE_UNAVAILABLE),
            &state,
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "429/3092 SystemResourceUnavailable must be failover-eligible; got {action:?}",
        );

        // Every other throttle sub-status must NOT become a region-changing
        // retry, so it can never be upgraded into a cross-region hedge.
        for sub in [
            SubStatusCode::RU_BUDGET_EXCEEDED,            // 3200
            SubStatusCode::RU_BUDGET_EXCEEDED_FOR_MASTER, // 3210
            SubStatusCode::HOT_PARTITION_KEY_THROTTLED,   // 3214
        ] {
            let op = make_read_operation();
            let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
            let (action, _) =
                evaluate_transport_result(&op, &endpoint, throttle_result(sub), &state);
            assert!(
                !matches!(
                    action,
                    OperationAction::FailoverRetry { .. } | OperationAction::SessionRetry { .. }
                ),
                "429/{sub:?} must not become a region-changing retry; \
                 got {action:?}",
            );
        }
    }

    #[test]
    fn service_unavailable_non_idempotent_write_retries() {
        let op = make_create_operation();
        let result = make_http_error(StatusCode::ServiceUnavailable);
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
    fn service_unavailable_non_idempotent_retries_with_ppaf() {
        // With PPAF enabled, behavior is the same as without — non-idempotent
        // writes always retry. This test validates PPAF doesn't interfere.
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
    fn transport_error_non_idempotent_retries_with_ppaf() {
        // With PPAF enabled, behavior is the same as without — non-idempotent
        // writes always retry. Sent → only partition mark (no endpoint mark).
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
        assert!(!effects
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
    fn transport_error_not_sent_marks_endpoint_only() {
        // Not-sent marks the endpoint only; Gateway connect failures are endpoint-wide.
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    }

    #[test]
    fn transport_error_not_sent_with_ppcb_still_marks_endpoint() {
        // Not-sent with PPCB active → endpoint is unreachable regardless of
        // PPCB state. Connection failures are endpoint-wide, so the endpoint
        // mark must not be suppressed by PPCB. The per-partition mark is
        // intentionally not emitted in either case for this branch.
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        state.ppcb_active = true;
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    }

    #[test]
    fn transport_error_unknown_sent_status_marks_partition_only() {
        // Unknown sent status is treated as "possibly sent" → endpoint is
        // potentially reachable, so only partition-level marking is applied.
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::Unknown);
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::FailoverRetry { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    #[test]
    fn transport_error_not_sent_over_budget_aborts_with_marks() {
        // Even over budget, not-sent updates endpoint routing state only.
        let op = make_read_operation();
        let result = make_transport_error(RequestSentStatus::NotSent);
        let state = OperationRetryState {
            location: crate::driver::routing::LocationIndex::initial(0),
            failover_retry_count: 1,
            session_token_retry_count: 0,
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries: 1,
            max_backend_failover_retries: 120,
            max_session_retries: 1,
            can_use_multiple_write_locations: false,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
            hedge_already_fired: false,
        };
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = evaluate_transport_result(&op, &endpoint, result, &state);
        assert!(matches!(action, OperationAction::Abort { .. }));
        assert!(effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(!effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
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
            SubStatusCode::from(0u16)
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
        // 403/1008 is a topology signal; the dedicated handler refreshes and fails over.
        assert!(!is_region_confirming_status(&status_with_substatus(
            StatusCode::Forbidden,
            SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND
        )));
        // 410/1008 is partition migration, not DatabaseAccountNotFound.
        assert!(!is_region_confirming_status(&status_with_substatus(
            StatusCode::Gone,
            SubStatusCode::COMPLETING_PARTITION_MIGRATION
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

    // ── Shared hub-region latch (Part 5) ──────────────────────────

    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    /// T-S1 — When the per-state latch fires and a shared latch is
    /// attached, the shared `Arc<AtomicBool>` is `Release`-stored as
    /// `true`. Counterpart of .NET PR #5815's `CrossRegionAvailabilityContext`
    /// propagation test.
    #[test]
    fn shared_hub_region_latch_propagates_first_1002_across_hedges() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;
        let shared = Arc::new(AtomicBool::new(false));
        state = state.with_shared_hub_region_latch(shared.clone());

        let after = session_retry_state_for_1002(&state);

        assert!(
            after.hub_region_processing_only,
            "per-state latch must still fire on the first 1002",
        );
        assert!(
            shared.load(Ordering::Acquire),
            "shared latch must be Release-stored when per-state latch fires",
        );
        // The new state must continue to carry the same shared latch
        // (propagation through `..self` in `advance_session_retry`).
        assert!(
            after.shared_hub_region_latch.as_ref().map(Arc::as_ptr) == Some(Arc::as_ptr(&shared)),
            "advance_session_retry must propagate the shared latch via ..self",
        );
    }

    /// T-S2 — A non-triggering 1002 (multi-master) must NOT flip the
    /// shared latch even if one is attached. Mirrors T-2 / AC-4 for the
    /// shared path.
    #[test]
    fn shared_hub_region_latch_does_not_set_on_multi_master_1002() {
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3); // multi-master
        state.is_dataplane = true;
        let shared = Arc::new(AtomicBool::new(false));
        state = state.with_shared_hub_region_latch(shared.clone());

        let after = session_retry_state_for_1002(&state);

        assert!(
            !after.hub_region_processing_only,
            "multi-master never latches per-state",
        );
        assert!(
            !shared.load(Ordering::Acquire),
            "multi-master never flips the shared latch either",
        );
    }

    /// T-S3 — A non-triggering 1002 on metadata pipeline must NOT flip
    /// the shared latch. Mirrors T-AC-8 / AC-8 for the shared path.
    #[test]
    fn shared_hub_region_latch_does_not_set_on_metadata_pipeline_1002() {
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = false; // metadata
        let shared = Arc::new(AtomicBool::new(false));
        state = state.with_shared_hub_region_latch(shared.clone());

        let after = session_retry_state_for_1002(&state);

        assert!(!after.hub_region_processing_only);
        assert!(!shared.load(Ordering::Acquire));
    }

    /// T-S4 — A pre-set shared latch on a multi-master state must NOT
    /// be cleared by the retry path. The latch is monotonic — once set,
    /// stays set. Guards against an accidental store-of-false on a
    /// non-triggering retry.
    #[test]
    fn shared_hub_region_latch_is_monotonic_once_set() {
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3); // multi-master
        state.is_dataplane = true;
        let shared = Arc::new(AtomicBool::new(true)); // pre-set
        state = state.with_shared_hub_region_latch(shared.clone());

        let _ = session_retry_state_for_1002(&state);

        assert!(
            shared.load(Ordering::Acquire),
            "shared latch must stay set across non-triggering retries",
        );
    }

    // ────────────────────────────────────────────────────────────────
    // `evaluate_hedge_leg_effects` — non-consuming hedge-leg evaluator
    // ────────────────────────────────────────────────────────────────
    //
    // Mirrors the same `(operation, endpoint, retry_state, result)`
    // tuple as `evaluate_transport_result` and emits the same
    // `LocationEffect`s, minus the consumed `OperationAction`. These
    // tests pin that mirror so a future refactor of either function
    // can't silently diverge their side-effect surface.

    /// Success transport result emits no effects and no session-retry
    /// signal — same as `evaluate_transport_result`.
    #[test]
    fn hedge_leg_effects_success_is_empty() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = make_success_result();

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(eval.effects.is_empty());
        assert!(!eval.observed_session_unavailable);
    }

    /// 503 ServiceUnavailable on a read emits `MarkPartitionUnavailable`
    /// + `MarkEndpointUnavailable` — exactly what the non-hedged path
    /// would have applied.
    #[test]
    fn hedge_leg_effects_503_emits_partition_and_endpoint_marks() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = make_http_error(StatusCode::ServiceUnavailable);

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
        assert!(!eval.observed_session_unavailable);
    }

    /// Transport error with `RequestSentStatus::Sent` on a read emits
    /// the same `MarkPartitionUnavailable` + `MarkEndpointUnavailable`
    /// pair the consuming path emits — see
    /// `evaluate_transport_layer_outcome`.
    #[test]
    fn hedge_leg_effects_transport_sent_emits_marks() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = make_transport_error(RequestSentStatus::Sent);

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
        assert!(eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    }

    /// Transport error with `RequestSentStatus::NotSent` emits NO
    /// effects (the failure is purely client-side; failing over is safe
    /// and incurs no routing-state consequences) — matches the
    /// `definitely_not_sent` branch in `evaluate_transport_layer_outcome`.
    #[test]
    fn hedge_leg_effects_transport_not_sent_is_empty() {
        let op = make_create_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = make_transport_error(RequestSentStatus::NotSent);

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(
            eval.effects.is_empty(),
            "not-sent transport error must not mark routing state",
        );
    }

    /// 404/1002 on a single-master data-plane read with budget remaining
    /// surfaces `observed_session_unavailable=true` AND emits no
    /// effects (matches `try_handle_read_session_not_available`'s
    /// empty-effects contract). The hedge race coordinator uses this
    /// signal to flip the parent `hub_region_processing_only` latch at
    /// the `BothTransient` upgrade boundary.
    #[test]
    fn hedge_leg_effects_1002_signals_session_unavailable() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;
        let result = make_read_session_not_available_result();

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(eval.effects.is_empty());
        assert!(
            eval.observed_session_unavailable,
            "first 1002 on single-master dataplane should signal session-unavailable",
        );
    }

    /// 404/1002 on a multi-master account does NOT signal
    /// `observed_session_unavailable` — matches
    /// `build_session_retry_state`'s 4-condition trigger (AC-4
    /// per HUB_REGION_PROCESSING_HEADER_SPEC.md §7.1).
    #[test]
    fn hedge_leg_effects_1002_multi_master_no_signal() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3); // multi-master
        state.is_dataplane = true;
        let result = make_read_session_not_available_result();

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(
            !eval.observed_session_unavailable,
            "multi-master 1002 must not flip the latch (AC-4)",
        );
    }

    /// Already-latched state does not re-signal `observed_session_unavailable`
    /// — idempotency boundary matching the non-hedged path.
    #[test]
    fn hedge_leg_effects_1002_no_signal_when_already_latched() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        state.is_dataplane = true;
        state.hub_region_processing_only = true;
        let result = make_read_session_not_available_result();

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(
            !eval.observed_session_unavailable,
            "already-latched state should not re-signal",
        );
    }

    /// DeadlineExceeded transport result emits no effects — matches
    /// `evaluate_deadline_exceeded_outcome`.
    #[test]
    fn hedge_leg_effects_deadline_exceeded_is_empty() {
        let op = make_read_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = TransportResult {
            outcome: TransportOutcome::DeadlineExceeded {
                request_sent: RequestSentStatus::Sent,
            },
        };

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(eval.effects.is_empty());
        assert!(!eval.observed_session_unavailable);
    }

    /// Final HTTP errors that aren't classified by any per-status helper
    /// (e.g. 409 Conflict) emit no effects from the hedge-leg
    /// evaluator — same as the consuming path's `Abort` fallthrough.
    #[test]
    fn hedge_leg_effects_409_conflict_is_empty() {
        let op = make_create_operation();
        let endpoint = test_endpoint();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
        let result = make_http_error(StatusCode::Conflict);

        let eval = evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
        assert!(
            eval.effects.is_empty(),
            "409 Conflict has no per-status handler; emits no effects",
        );
    }
}
