// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure functions that evaluate request outcomes and produce retry decisions.
//!
//! These functions implement the "systems" in the DOP/ECS-inspired pipeline
//! model. They take focused data components (not the entire request) and
//! return either a [`RetryDecision`] describing what to do next, or a new
//! state value. **No mutation** of shared state happens here — the caller
//! applies the returned values.

use azure_core::time::Duration;
use url::Url;

use super::components::{CosmosStatus, OperationInfo, RetryState, RoutingState, ThrottleState};

/// Default retry interval between attempts.
const ENDPOINT_FAILOVER_RETRY_INTERVAL: Duration = Duration::milliseconds(500);

/// Maximum retry count for endpoint failover.
const MAX_FAILOVER_RETRIES: i32 = 120;

/// Maximum retry count for service-unavailable errors.
const MAX_SERVICE_UNAVAILABLE_RETRIES: i32 = 1;

/// Default throttle backoff base when no server retry-after is provided.
const DEFAULT_THROTTLE_BACKOFF: Duration = Duration::milliseconds(100);

/// Outcome of evaluating a request result against the retry policy.
///
/// Each variant describes a distinct action for the orchestration loop. The
/// loop matches exhaustively to transform state and either retry or return.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RetryDecision {
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

impl RetryDecision {
    /// Evaluates a data-plane request outcome and returns a retry decision.
    ///
    /// This is a pure function: it reads the current state without mutating it.
    /// The caller is responsible for applying state updates based on the returned
    /// decision via the `apply_*` methods on the state types.
    ///
    /// Throttle (429) handling is integrated: when the status is `TooManyRequests`,
    /// the decision is delegated to the throttle backoff logic using the provided
    /// `throttle` state and optional server `retry_after` hint.
    pub fn for_data_plane(
        status: &CosmosStatus,
        retry: &RetryState,
        op: &OperationInfo,
        throttle: &ThrottleState,
        retry_after: Option<Duration>,
    ) -> RetryDecision {
        // 403.3 — Write Forbidden → endpoint failover with cache refresh
        if status.is_write_forbidden() {
            return decide_endpoint_failover(retry, op);
        }

        // 404.1002 — READ_SESSION_NOT_AVAILABLE → session retry
        if status.is_read_session_not_available() {
            return decide_session_retry(retry, op);
        }

        // 503, 500 on reads, 410+LeaseNotFound → preferred-location failover
        if status.is_service_unavailable_class(op.is_read_only()) {
            return decide_service_unavailable(retry, op);
        }

        // 429 — Too Many Requests → throttle backoff
        if status.is_too_many_requests() {
            return Self::for_throttle(throttle, retry_after);
        }

        // Any other status — no retry from data-plane logic.
        RetryDecision::Done
    }

    /// Evaluates a metadata-plane request outcome and returns a retry decision.
    ///
    /// Metadata operations have simpler retry logic: service unavailable, internal
    /// errors, gone/lease-not-found, and forbidden/account-not-found all retry
    /// on the next preferred location up to `max_retry_count`.
    ///
    /// Throttle (429) handling is integrated: when the status is `TooManyRequests`,
    /// the decision is delegated to the throttle backoff logic.
    pub fn for_metadata(
        status: &CosmosStatus,
        retry: &RetryState,
        op: &OperationInfo,
        throttle: &ThrottleState,
        retry_after: Option<Duration>,
    ) -> RetryDecision {
        let max_retry_count = std::cmp::max(op.preferred_location_count as i32, 1);

        let should_retry = status.is_service_unavailable()
            || status.is_internal_server_error()
            || status.is_gone_lease_not_found()
            || status.is_account_not_found();

        if should_retry {
            let next_count = retry.service_unavailable_count + 1;
            if next_count > max_retry_count {
                return RetryDecision::Abort;
            }
            return RetryDecision::RetryNextRegion {
                delay: Duration::ZERO,
            };
        }

        // 429 — Too Many Requests → throttle backoff
        if status.is_too_many_requests() {
            return Self::for_throttle(throttle, retry_after);
        }

        RetryDecision::Done
    }

    /// Evaluates a throttle (429) scenario and returns a retry decision.
    ///
    /// Uses a hybrid backoff strategy:
    /// - When no server `retry_after` is provided: exponential backoff from
    ///   [`DEFAULT_THROTTLE_BACKOFF`] (`base * 2^attempt`).
    /// - When `retry_after` is provided and `attempt < 3`: uses the server
    ///   value directly.
    /// - When `retry_after` is provided and `attempt >= 3`: applies an
    ///   exponential multiplier to the server value (`d * 2^(attempt - 2)`).
    ///
    /// The delay is clamped to the remaining budget (`max_wait - cumulative_wait`).
    ///
    /// This is called internally by [`for_data_plane`](Self::for_data_plane) and
    /// [`for_metadata`](Self::for_metadata) when the status is 429.
    fn for_throttle(throttle: &ThrottleState, retry_after: Option<Duration>) -> RetryDecision {
        if throttle.attempt >= throttle.max_attempts {
            return RetryDecision::Abort;
        }

        let delay = match retry_after {
            None => {
                // No server hint — pure exponential backoff
                let multiplier = 1i32 << throttle.attempt.min(10);
                DEFAULT_THROTTLE_BACKOFF * multiplier
            }
            Some(server_delay) if throttle.attempt < 3 => {
                // Trust server value for the first 3 attempts
                server_delay
            }
            Some(server_delay) => {
                // After 3 attempts, apply exponential multiplier
                let exponent = (throttle.attempt - 2).min(10);
                let multiplier = 1i32 << exponent;
                server_delay * multiplier
            }
        };

        // Clamp to remaining budget
        let remaining = throttle.max_wait - throttle.cumulative_wait;
        if remaining <= Duration::ZERO {
            return RetryDecision::Abort;
        }
        let clamped = if delay > remaining { remaining } else { delay };

        let new_cumulative = throttle.cumulative_wait + clamped;
        if new_cumulative > throttle.max_wait {
            return RetryDecision::Abort;
        }

        RetryDecision::RetrySameEndpoint { delay: clamped }
    }
}

// ─── Internal decision helpers ──────────────────────────────────────────────

/// Decides whether to failover to a different endpoint after a request failure.
// TODO @dkunda: support overwrite_discovery to force endpoint discovery when disabled
fn decide_endpoint_failover(retry: &RetryState, op: &OperationInfo) -> RetryDecision {
    if retry.failover_count >= MAX_FAILOVER_RETRIES || !op.endpoint_discovery_enabled {
        return RetryDecision::Abort;
    }

    let delay = if !op.is_read_only() {
        // First write failover has zero delay, subsequent have ENDPOINT_FAILOVER_RETRY_INTERVAL
        if retry.failover_count > 0 {
            ENDPOINT_FAILOVER_RETRY_INTERVAL
        } else {
            Duration::ZERO
        }
    } else {
        ENDPOINT_FAILOVER_RETRY_INTERVAL
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
    if !op.can_use_multi_write && !op.is_read_only() {
        return RetryDecision::Abort;
    }

    if op.preferred_location_count <= 1 {
        return RetryDecision::Abort;
    }

    RetryDecision::RetryNextRegion {
        delay: Duration::ZERO,
    }
}

// ─── State update methods ───────────────────────────────────────────────────

impl RetryState {
    /// Applies a data-plane retry decision, returning the updated state.
    pub fn apply_data_plane(mut self, decision: &RetryDecision) -> Self {
        match decision {
            RetryDecision::RetryNextRegion { .. } => {
                self.failover_count += 1;
            }
            RetryDecision::RetryOnWriteEndpoint { .. } => {
                self.session_retry_count += 1;
            }
            RetryDecision::RetrySameEndpoint { .. }
            | RetryDecision::Done
            | RetryDecision::Abort => {}
        }
        self
    }

    /// Bumps the service-unavailable counter.
    pub fn apply_service_unavailable(mut self) -> Self {
        self.service_unavailable_count += 1;
        self
    }

    /// Bumps the session retry counter.
    pub fn apply_session(mut self) -> Self {
        self.session_retry_count += 1;
        self
    }

    /// Bumps the service-unavailable counter for metadata retries.
    pub fn apply_metadata(mut self) -> Self {
        self.service_unavailable_count += 1;
        self
    }
}

impl ThrottleState {
    /// Applies a throttle retry decision, bumping the attempt counter and
    /// accumulating wait time.
    pub fn apply(mut self, decision: &RetryDecision) -> Self {
        if let RetryDecision::RetrySameEndpoint { delay } = decision {
            self.attempt += 1;
            self.cumulative_wait += *delay;
        }
        self
    }
}

impl RoutingState {
    /// Updates routing state for a retry-next-region decision.
    ///
    /// `new_index` controls which location to try next. Callers should pass
    /// the relevant retry counter (e.g., `failover_count` or
    /// `service_unavailable_count`) so the index advances through locations.
    pub fn apply_for_next_region(mut self, new_index: i32, use_preferred: bool) -> Self {
        self.location_index = new_index;
        self.use_preferred_locations = use_preferred;
        self.resolved_endpoint = None;
        self
    }

    /// Updates routing state for a retry-on-write-endpoint decision (session retry).
    pub fn apply_for_write_endpoint(mut self) -> Self {
        self.location_index = 0;
        self.use_preferred_locations = false;
        self.resolved_endpoint = None;
        self
    }

    /// Marks the current endpoint as failed in the routing state.
    pub fn mark_endpoint_failed(mut self, endpoint: &Url) -> Self {
        self.failed_endpoints.insert(endpoint.clone());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SubStatusCode;
    use crate::operation_context::OperationType;
    use crate::resource_context::ResourceType;
    use azure_core::http::StatusCode;

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

    fn status(code: StatusCode, sub: Option<SubStatusCode>) -> CosmosStatus {
        CosmosStatus::new(code, sub)
    }

    // ─── Data-plane retry decisions ─────────────────────────

    #[test]
    fn write_forbidden_triggers_failover() {
        let retry = RetryState::default();
        let op = write_op(false, 2);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::Forbidden, Some(SubStatusCode::WRITE_FORBIDDEN)),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
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
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::Forbidden, Some(SubStatusCode::WRITE_FORBIDDEN)),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn session_not_available_retries_on_write_endpoint_single_write() {
        let retry = RetryState::default();
        let op = read_op(2);
        let decision = RetryDecision::for_data_plane(
            &status(
                StatusCode::NotFound,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            ),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
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
        let decision = RetryDecision::for_data_plane(
            &status(
                StatusCode::NotFound,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            ),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
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
        let decision = RetryDecision::for_data_plane(
            &status(
                StatusCode::NotFound,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            ),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn service_unavailable_retries_read_with_preferred_locations() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::ServiceUnavailable, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn service_unavailable_aborts_write_without_multi_write() {
        let retry = RetryState::default();
        let op = write_op(false, 3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::ServiceUnavailable, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn service_unavailable_aborts_after_max_retries() {
        let retry = RetryState {
            service_unavailable_count: MAX_SERVICE_UNAVAILABLE_RETRIES,
            ..Default::default()
        };
        let op = read_op(3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::ServiceUnavailable, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn internal_error_retries_for_reads() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::InternalServerError, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn internal_error_done_for_writes() {
        let retry = RetryState::default();
        let op = write_op(false, 3);
        // ISE on write without matching sub-status: no data-plane retry
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::InternalServerError, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Done);
    }

    #[test]
    fn gone_lease_not_found_retries() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::Gone, Some(SubStatusCode::LEASE_NOT_FOUND)),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn too_many_requests_delegates_to_throttle() {
        let retry = RetryState::default();
        let op = read_op(3);
        let throttle = ThrottleState::default();
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::TooManyRequests, None),
            &retry,
            &op,
            &throttle,
            None,
        );
        // Should produce a throttle backoff, not Done
        assert!(matches!(decision, RetryDecision::RetrySameEndpoint { .. }));
    }

    #[test]
    fn success_returns_done() {
        let retry = RetryState::default();
        let op = read_op(3);
        let decision = RetryDecision::for_data_plane(
            &status(StatusCode::Ok, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Done);
    }

    // ─── Metadata retry decisions ───────────────────────────

    #[test]
    fn metadata_service_unavailable_retries() {
        let retry = RetryState::default();
        let op = metadata_op(3);
        let decision = RetryDecision::for_metadata(
            &status(StatusCode::ServiceUnavailable, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert!(matches!(decision, RetryDecision::RetryNextRegion { .. }));
    }

    #[test]
    fn metadata_account_not_found_retries() {
        let retry = RetryState::default();
        let op = metadata_op(3);
        let decision = RetryDecision::for_metadata(
            &status(
                StatusCode::Forbidden,
                Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND),
            ),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
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
        let decision = RetryDecision::for_metadata(
            &status(StatusCode::ServiceUnavailable, None),
            &retry,
            &op,
            &ThrottleState::default(),
            None,
        );
        assert_eq!(decision, RetryDecision::Abort);
    }

    // ─── Throttle decisions ─────────────────────────────────

    #[test]
    fn throttle_retries_with_server_retry_after() {
        let throttle = ThrottleState::default();
        let decision = RetryDecision::for_throttle(&throttle, Some(Duration::milliseconds(200)));
        assert!(matches!(decision, RetryDecision::RetrySameEndpoint { .. }));
        if let RetryDecision::RetrySameEndpoint { delay } = decision {
            // First attempt with server hint: use server value directly
            assert_eq!(delay, Duration::milliseconds(200));
        }
    }

    #[test]
    fn throttle_uses_exponential_backoff_without_server_hint() {
        let throttle = ThrottleState::default();
        let decision = RetryDecision::for_throttle(&throttle, None);
        assert!(matches!(decision, RetryDecision::RetrySameEndpoint { .. }));
        if let RetryDecision::RetrySameEndpoint { delay } = decision {
            // attempt=0: DEFAULT_THROTTLE_BACKOFF * 2^0 = 100ms
            assert_eq!(delay, Duration::milliseconds(100));
        }

        // attempt=1: 100ms * 2^1 = 200ms
        let throttle1 = ThrottleState {
            attempt: 1,
            ..Default::default()
        };
        let decision1 = RetryDecision::for_throttle(&throttle1, None);
        if let RetryDecision::RetrySameEndpoint { delay } = decision1 {
            assert_eq!(delay, Duration::milliseconds(200));
        }
    }

    #[test]
    fn throttle_applies_multiplier_after_third_attempt() {
        // attempt=3 with server hint 200ms: 200ms * 2^(3-2) = 400ms
        let throttle = ThrottleState {
            attempt: 3,
            ..Default::default()
        };
        let decision = RetryDecision::for_throttle(&throttle, Some(Duration::milliseconds(200)));
        if let RetryDecision::RetrySameEndpoint { delay } = decision {
            assert_eq!(delay, Duration::milliseconds(400));
        } else {
            panic!("expected RetrySameEndpoint");
        }
    }

    #[test]
    fn throttle_aborts_at_max_attempts() {
        let throttle = ThrottleState {
            attempt: 5,
            ..Default::default()
        };
        let decision = RetryDecision::for_throttle(&throttle, Some(Duration::milliseconds(200)));
        assert_eq!(decision, RetryDecision::Abort);
    }

    #[test]
    fn throttle_aborts_when_cumulative_exceeds_max() {
        let throttle = ThrottleState {
            attempt: 1,
            max_attempts: 5,
            cumulative_wait: Duration::seconds(199),
            max_wait: Duration::seconds(200),
        };
        // 199s + 2s > 200s max
        let decision = RetryDecision::for_throttle(&throttle, Some(Duration::seconds(2)));
        // Should be clamped to 1s (remaining budget) or abort
        assert!(matches!(
            decision,
            RetryDecision::RetrySameEndpoint { .. } | RetryDecision::Abort
        ));
        if let RetryDecision::RetrySameEndpoint { delay } = decision {
            // Clamped to remaining budget of 1s
            assert_eq!(delay, Duration::seconds(1));
        }
    }

    // ─── State update methods ───────────────────────────────

    #[test]
    fn apply_data_plane_increments_failover() {
        let retry = RetryState::default();
        let decision = RetryDecision::RetryNextRegion {
            delay: Duration::ZERO,
        };
        let updated = retry.apply_data_plane(&decision);
        assert_eq!(updated.failover_count, 1);
    }

    #[test]
    fn apply_session_increments_session_count() {
        let retry = RetryState::default();
        let updated = retry.apply_session();
        assert_eq!(updated.session_retry_count, 1);
    }

    #[test]
    fn apply_throttle_increments_and_accumulates() {
        let throttle = ThrottleState::default();
        let decision = RetryDecision::RetrySameEndpoint {
            delay: Duration::milliseconds(500),
        };
        let updated = throttle.apply(&decision);
        assert_eq!(updated.attempt, 1);
        assert_eq!(updated.cumulative_wait, Duration::milliseconds(500));
    }

    #[test]
    fn routing_for_next_region_advances_index() {
        let routing = RoutingState {
            resolved_endpoint: Some("https://old.example.com".parse().unwrap()),
            ..Default::default()
        };
        // Advancing with new_index=2 and use_preferred=true
        let updated = routing.apply_for_next_region(2, true);
        assert_eq!(updated.location_index, 2);
        assert!(updated.use_preferred_locations);
        assert!(updated.resolved_endpoint.is_none());
    }

    #[test]
    fn routing_for_next_region_without_preferred() {
        let routing = RoutingState::default();
        let updated = routing.apply_for_next_region(3, false);
        assert_eq!(updated.location_index, 3);
        assert!(!updated.use_preferred_locations);
    }

    #[test]
    fn routing_for_write_endpoint_sets_index_zero() {
        let routing = RoutingState {
            location_index: 5,
            ..Default::default()
        };
        let updated = routing.apply_for_write_endpoint();
        assert_eq!(updated.location_index, 0);
        assert!(!updated.use_preferred_locations);
    }

    #[test]
    fn mark_endpoint_failed_adds_to_set() {
        let routing = RoutingState::default();
        let endpoint: Url = "https://cosmos-east.example.com".parse().unwrap();
        let updated = routing.mark_endpoint_failed(&endpoint);
        assert!(updated.failed_endpoints.contains(&endpoint));
    }
}
