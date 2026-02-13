// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure functions that evaluate request outcomes and produce retry decisions.
//!
//! These functions implement the "systems" in the DOP/ECS-inspired pipeline
//! model. They take focused data components (not the entire request) and
//! return either a [`RetryDecision`] describing what to do next, or a new
//! state value. **No mutation** of shared state happens here — the caller
//! applies the returned values.

use crate::constants::SubStatusCode;
use azure_core::http::StatusCode;
use azure_core::time::Duration;
use url::Url;

use super::components::{OperationInfo, RetryState, RoutingState, ThrottleState};

/// Default retry interval between attempts (milliseconds).
const RETRY_INTERVAL_MS: i64 = 1000;

/// Maximum retry count for endpoint failover.
const MAX_FAILOVER_RETRIES: i32 = 120;

/// Maximum retry count for service-unavailable errors.
const MAX_SERVICE_UNAVAILABLE_RETRIES: i32 = 1;

/// Default throttle backoff base (milliseconds).
const DEFAULT_THROTTLE_BACKOFF_MS: i64 = 500;

/// Outcome of evaluating a request result against the retry policy.
///
/// Each variant describes a distinct action for the orchestration loop. The
/// loop matches exhaustively to transform state and either retry or return.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum RetryDecision {
    /// Request succeeded or is a non-retryable failure — return as-is.
    Done,

    /// Failover to the next preferred region.
    RetryNextRegion { delay: Duration },

    /// Retry on the same endpoint after a delay (e.g. throttle backoff).
    RetrySameEndpoint { delay: Duration },

    /// Route to the primary write endpoint (session-not-available scenario).
    RetryOnWriteEndpoint { delay: Duration },

    /// Non-retryable — return the result to the caller.
    Abort,
}

/// Evaluates a data-plane request outcome and returns a retry decision.
///
/// This is a pure function: it reads the current state without mutating it.
/// The caller is responsible for applying state updates based on the returned
/// decision via [`apply_data_plane_decision`].
pub(super) fn decide_data_plane_retry(
    status: StatusCode,
    sub_status: Option<SubStatusCode>,
    retry: &RetryState,
    op: &OperationInfo,
) -> RetryDecision {
    // 403.3 — Write Forbidden → endpoint failover with cache refresh
    if status == StatusCode::Forbidden && sub_status == Some(SubStatusCode::WRITE_FORBIDDEN) {
        return decide_endpoint_failover(retry, op, false);
    }

    // 404.1022 — READ_SESSION_NOT_AVAILABLE → session retry
    if status == StatusCode::NotFound
        && sub_status == Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
    {
        return decide_session_retry(retry, op);
    }

    // 503 — Service Unavailable → preferred-location failover
    if status == StatusCode::ServiceUnavailable {
        return decide_service_unavailable(retry, op);
    }

    // 500 on reads, or 410 + LeaseNotFound → preferred-location failover
    if (status == StatusCode::InternalServerError && op.is_read_only)
        || (status == StatusCode::Gone && sub_status == Some(SubStatusCode::LEASE_NOT_FOUND))
    {
        return decide_service_unavailable(retry, op);
    }

    // 429 — Too Many Requests → handled by throttle path, signal Done here
    // so the caller knows to check the throttle decision instead.
    if status == StatusCode::TooManyRequests {
        // Throttle decisions are handled separately — return Done to signal
        // "not handled by data-plane logic, fall through to throttle".
        return RetryDecision::Done;
    }

    // Any other status — no retry from data-plane logic.
    RetryDecision::Done
}

/// Evaluates a metadata-plane request outcome and returns a retry decision.
///
/// Metadata operations have simpler retry logic: service unavailable, internal
/// errors, gone/lease-not-found, and forbidden/account-not-found all retry
/// on the next preferred location up to `max_retry_count`.
pub(super) fn decide_metadata_retry(
    status: StatusCode,
    sub_status: Option<SubStatusCode>,
    retry: &RetryState,
    op: &OperationInfo,
) -> RetryDecision {
    let max_retry_count = std::cmp::max(op.preferred_location_count as i32, 1);

    let should_retry = status == StatusCode::ServiceUnavailable
        || status == StatusCode::InternalServerError
        || (status == StatusCode::Gone && sub_status == Some(SubStatusCode::LEASE_NOT_FOUND))
        || (status == StatusCode::Forbidden
            && sub_status == Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND));

    if should_retry {
        let next_count = retry.service_unavailable_count + 1;
        if next_count > max_retry_count {
            return RetryDecision::Abort;
        }
        return RetryDecision::RetryNextRegion {
            delay: Duration::ZERO,
        };
    }

    // 429 — fall through to throttle
    if status == StatusCode::TooManyRequests {
        return RetryDecision::Done;
    }

    RetryDecision::Done
}

/// Evaluates a throttle (429) scenario and returns a retry decision.
pub(super) fn decide_throttle_retry(
    throttle: &ThrottleState,
    retry_after_ms: Option<i64>,
) -> RetryDecision {
    if throttle.attempt >= throttle.max_attempts {
        return RetryDecision::Abort;
    }

    let backoff_ms = retry_after_ms.unwrap_or(DEFAULT_THROTTLE_BACKOFF_MS);
    let delay = Duration::milliseconds(backoff_ms);

    let new_cumulative = throttle.cumulative_wait + delay;
    if new_cumulative > throttle.max_wait {
        return RetryDecision::Abort;
    }

    RetryDecision::RetrySameEndpoint { delay }
}

// ─── Internal decision helpers ──────────────────────────────────────────────

/// Decides whether to failover to a different endpoint after a request failure.
fn decide_endpoint_failover(
    retry: &RetryState,
    op: &OperationInfo,
    _overwrite_discovery: bool,
) -> RetryDecision {
    if retry.failover_count >= MAX_FAILOVER_RETRIES || !op.endpoint_discovery_enabled {
        return RetryDecision::Abort;
    }

    let delay = if !op.is_read_only {
        // First write failover has zero delay, subsequent have RETRY_INTERVAL_MS
        if retry.failover_count > 0 {
            Duration::milliseconds(RETRY_INTERVAL_MS)
        } else {
            Duration::ZERO
        }
    } else {
        Duration::milliseconds(RETRY_INTERVAL_MS)
    };

    RetryDecision::RetryNextRegion { delay }
}

/// Decides whether to retry on a different endpoint for session-not-available.
fn decide_session_retry(retry: &RetryState, op: &OperationInfo) -> RetryDecision {
    if !op.endpoint_discovery_enabled {
        return RetryDecision::Abort;
    }

    let next_count = retry.session_retry_count + 1;

    if op.can_use_multi_write {
        // Multi-write: try all endpoints
        if next_count > op.preferred_location_count as i32 {
            return RetryDecision::Abort;
        }
        RetryDecision::RetryNextRegion {
            delay: Duration::ZERO,
        }
    } else {
        // Single-write: retry once on write endpoint
        if next_count > 1 {
            return RetryDecision::Abort;
        }
        RetryDecision::RetryOnWriteEndpoint {
            delay: Duration::ZERO,
        }
    }
}

/// Decides whether to retry on the next preferred region for service-unavailable.
fn decide_service_unavailable(retry: &RetryState, op: &OperationInfo) -> RetryDecision {
    let next_count = retry.service_unavailable_count + 1;

    if next_count > MAX_SERVICE_UNAVAILABLE_RETRIES {
        return RetryDecision::Abort;
    }

    // Write operations require multi-write support for failover
    if !op.can_use_multi_write && !op.is_read_only {
        return RetryDecision::Abort;
    }

    if op.preferred_location_count <= 1 {
        return RetryDecision::Abort;
    }

    RetryDecision::RetryNextRegion {
        delay: Duration::ZERO,
    }
}

// ─── State update functions ─────────────────────────────────────────────────

/// Applies a retry decision to the retry state, returning the updated state.
///
/// This is a pure transformation: old state in, new state out.
pub(super) fn apply_data_plane_decision(
    mut retry: RetryState,
    decision: &RetryDecision,
) -> RetryState {
    match decision {
        RetryDecision::RetryNextRegion { .. } => {
            // Could be endpoint failover or service-unavailable;
            // the caller context determines which counter to bump.
            // We bump failover_count as default; callers override for 503.
            retry.failover_count += 1;
        }
        RetryDecision::RetryOnWriteEndpoint { .. } => {
            retry.session_retry_count += 1;
        }
        RetryDecision::RetrySameEndpoint { .. } | RetryDecision::Done | RetryDecision::Abort => {}
    }
    retry
}

/// Applies a service-unavailable decision to the retry state.
pub(super) fn apply_service_unavailable_decision(mut retry: RetryState) -> RetryState {
    retry.service_unavailable_count += 1;
    retry
}

/// Applies a session-retry decision to the retry state.
pub(super) fn apply_session_decision(mut retry: RetryState) -> RetryState {
    retry.session_retry_count += 1;
    retry
}

/// Applies a metadata-plane retry decision to the retry state.
pub(super) fn apply_metadata_decision(mut retry: RetryState) -> RetryState {
    retry.service_unavailable_count += 1;
    retry
}

/// Applies a throttle retry decision to the throttle state.
pub(super) fn apply_throttle_decision(
    mut throttle: ThrottleState,
    decision: &RetryDecision,
) -> ThrottleState {
    if let RetryDecision::RetrySameEndpoint { delay } = decision {
        throttle.attempt += 1;
        throttle.cumulative_wait += *delay;
    }
    throttle
}

/// Updates routing state for a retry-next-region decision.
pub(super) fn apply_routing_for_next_region(
    mut routing: RoutingState,
    retry: &RetryState,
    use_preferred: bool,
) -> RoutingState {
    routing.location_index = if use_preferred {
        0
    } else {
        retry.failover_count
    };
    routing.use_preferred_locations = use_preferred;
    routing.resolved_endpoint = None;
    routing
}

/// Updates routing state for a retry-on-write-endpoint decision (session retry).
pub(super) fn apply_routing_for_write_endpoint(mut routing: RoutingState) -> RoutingState {
    routing.location_index = 0;
    routing.use_preferred_locations = false;
    routing.resolved_endpoint = None;
    routing
}

/// Marks the current endpoint as failed in the routing state.
pub(super) fn mark_endpoint_failed(mut routing: RoutingState, endpoint: &Url) -> RoutingState {
    routing.failed_endpoints.insert(endpoint.clone());
    routing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::resource_context::ResourceType;

    fn read_op(preferred_count: usize) -> OperationInfo {
        OperationInfo::new(
            OperationType::Read,
            ResourceType::Documents,
            false,
            preferred_count,
            None,
        )
    }

    fn write_op(multi_write: bool, preferred_count: usize) -> OperationInfo {
        OperationInfo::new(
            OperationType::Create,
            ResourceType::Documents,
            multi_write,
            preferred_count,
            None,
        )
    }

    fn metadata_op(preferred_count: usize) -> OperationInfo {
        OperationInfo::new(
            OperationType::Read,
            ResourceType::Databases,
            false,
            preferred_count,
            None,
        )
    }

    // ─── Data-plane retry decisions ─────────────────────────

    #[test]
    fn write_forbidden_triggers_failover() {
        let retry = RetryState::default();
        let op = write_op(false, 2);
        let decision = decide_data_plane_retry(
            StatusCode::Forbidden,
            Some(SubStatusCode::WRITE_FORBIDDEN),
            &retry,
            &op,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn write_forbidden_aborts_at_max_retries() {
        let retry = RetryState {
            failover_count: MAX_FAILOVER_RETRIES,
            ..Default::default()
        };
        let op = write_op(false, 2);
        let decision = decide_data_plane_retry(
            StatusCode::Forbidden,
            Some(SubStatusCode::WRITE_FORBIDDEN),
            &retry,
            &op,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn session_not_available_retries_on_write_endpoint_single_write() {
        let retry = RetryState::default();
        let op = read_op(2);
        let decision = decide_data_plane_retry(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            &retry,
            &op,
        );
        assert!(matches!(
            decision,
            RetryDecision::RetryOnWriteEndpoint { .. }
        ));
    }

    #[test]
    fn session_not_available_retries_next_region_multi_write() {
        let retry = RetryState::default();
        let op = write_op(true, 3);
        let decision = decide_data_plane_retry(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            &retry,
            &op,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn session_not_available_aborts_after_all_regions() {
        let retry = RetryState {
            session_retry_count: 3,
            ..Default::default()
        };
        let op = write_op(true, 3);
        let decision = decide_data_plane_retry(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            &retry,
            &op,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn service_unavailable_retries_read_with_preferred_locations() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = decide_data_plane_retry(StatusCode::ServiceUnavailable, None, &retry, &op);
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn service_unavailable_aborts_write_without_multi_write() {
        let retry = RetryState::default();
        let op = write_op(false, 3);
        let decision = decide_data_plane_retry(StatusCode::ServiceUnavailable, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn service_unavailable_aborts_after_max_retries() {
        let retry = RetryState {
            service_unavailable_count: MAX_SERVICE_UNAVAILABLE_RETRIES,
            ..Default::default()
        };
        let op = read_op(3);
        let decision = decide_data_plane_retry(StatusCode::ServiceUnavailable, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn internal_error_retries_for_reads() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = decide_data_plane_retry(StatusCode::InternalServerError, None, &retry, &op);
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn internal_error_done_for_writes() {
        let retry = RetryState::default();
        let op = write_op(false, 3);
        // ISE on write without matching sub-status: no data-plane retry
        let decision = decide_data_plane_retry(StatusCode::InternalServerError, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Done);
    }

    #[test]
    fn gone_lease_not_found_retries() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = decide_data_plane_retry(
            StatusCode::Gone,
            Some(SubStatusCode::LEASE_NOT_FOUND),
            &retry,
            &op,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn too_many_requests_returns_done_for_throttle_path() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = decide_data_plane_retry(StatusCode::TooManyRequests, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Done);
    }

    #[test]
    fn success_returns_done() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = decide_data_plane_retry(StatusCode::Ok, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Done);
    }

    // ─── Metadata retry decisions ───────────────────────────

    #[test]
    fn metadata_service_unavailable_retries() {
        let retry = RetryState::default();
        let op = metadata_op(3);
        let decision = decide_metadata_retry(StatusCode::ServiceUnavailable, None, &retry, &op);
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn metadata_account_not_found_retries() {
        let retry = RetryState::default();
        let op = metadata_op(3);
        let decision = decide_metadata_retry(
            StatusCode::Forbidden,
            Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND),
            &retry,
            &op,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn metadata_aborts_after_max_retries() {
        let retry = RetryState {
            service_unavailable_count: 3,
            ..Default::default()
        };
        let op = metadata_op(3);
        let decision = decide_metadata_retry(StatusCode::ServiceUnavailable, None, &retry, &op);
        assert_eq!(decision, RetryDecision::Abort);
    }

    // ─── Throttle decisions ─────────────────────────────────

    #[test]
    fn throttle_retries_with_backoff() {
        let throttle = ThrottleState::default();
        let decision = decide_throttle_retry(&throttle, Some(200));
        assert!(matches!(decision, RetryDecision::RetrySameEndpoint { .. }));
    }

    #[test]
    fn throttle_aborts_at_max_attempts() {
        let throttle = ThrottleState {
            attempt: 5,
            ..Default::default()
        };
        let decision = decide_throttle_retry(&throttle, Some(200));
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn throttle_aborts_when_cumulative_exceeds_max() {
        let throttle = ThrottleState {
            attempt: 1,
            max_attempts: 5,
            cumulative_wait: Duration::seconds(9),
            max_wait: Duration::seconds(10),
        };
        // 9s + 2s > 10s max
        let decision = decide_throttle_retry(&throttle, Some(2000));
        assert_eq!(decision, RetryDecision::Abort);
    }

    // ─── State update functions ─────────────────────────────

    #[test]
    fn apply_data_plane_decision_increments_failover() {
        let retry = RetryState::default();
        let decision = RetryDecision::RetryNextRegion {
            delay: Duration::ZERO,
        };
        let updated = apply_data_plane_decision(retry, &decision);
        assert_eq!(updated.failover_count, 1);
    }

    #[test]
    fn apply_session_decision_increments_session_count() {
        let retry = RetryState::default();
        let updated = apply_session_decision(retry);
        assert_eq!(updated.session_retry_count, 1);
    }

    #[test]
    fn apply_throttle_decision_increments_and_accumulates() {
        let throttle = ThrottleState::default();
        let decision = RetryDecision::RetrySameEndpoint {
            delay: Duration::milliseconds(500),
        };
        let updated = apply_throttle_decision(throttle, &decision);
        assert_eq!(updated.attempt, 1);
        assert_eq!(updated.cumulative_wait, Duration::milliseconds(500));
    }

    #[test]
    fn routing_for_next_region_resets_endpoint() {
        let routing = RoutingState {
            resolved_endpoint: Some("https://old.example.com".parse().unwrap()),
            ..Default::default()
        };
        let retry = RetryState {
            failover_count: 2,
            ..Default::default()
        };
        let updated = apply_routing_for_next_region(routing, &retry, true);
        assert_eq!(updated.location_index, 0);
        assert!(updated.use_preferred_locations);
        assert!(updated.resolved_endpoint.is_none());
    }

    #[test]
    fn routing_for_write_endpoint_sets_index_zero() {
        let routing = RoutingState {
            location_index: 5,
            ..Default::default()
        };
        let updated = apply_routing_for_write_endpoint(routing);
        assert_eq!(updated.location_index, 0);
        assert!(!updated.use_preferred_locations);
    }

    #[test]
    fn mark_endpoint_failed_adds_to_set() {
        let routing = RoutingState::default();
        let endpoint: Url = "https://cosmos-east.example.com".parse().unwrap();
        let updated = mark_endpoint_failed(routing, &endpoint);
        assert!(updated.failed_endpoints.contains(&endpoint));
    }
}
