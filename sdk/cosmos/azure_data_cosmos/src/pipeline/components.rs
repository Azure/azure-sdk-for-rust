// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data components for the pipeline execution model.
//!
//! These are plain data structs representing the distinct concerns of
//! request execution: routing state, retry tracking, throttle tracking,
//! and immutable operation metadata. Following DOP principles, these types
//! carry no behavior beyond construction and are transformed by free
//! functions in the [`super::retry_decisions`] module.

use crate::constants::SubStatusCode;
use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::ResourceType;
use azure_core::http::StatusCode;
use azure_core::time::Duration;
use std::collections::HashSet;
use url::Url;

/// Tracks which endpoint the request is routed to and which endpoints have failed.
///
/// This component is updated between retry attempts to steer the next request
/// to an alternative region or endpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RoutingState {
    /// Index into the preferred-locations list for the next attempt.
    pub location_index: i32,

    /// Whether to use preferred locations for routing.
    pub use_preferred_locations: bool,

    /// The resolved endpoint URL for the current attempt.
    pub resolved_endpoint: Option<Url>,

    /// Endpoints that have been tried and failed.
    pub failed_endpoints: HashSet<Url>,
}

impl Default for RoutingState {
    fn default() -> Self {
        Self {
            location_index: 0,
            use_preferred_locations: true,
            resolved_endpoint: None,
            failed_endpoints: HashSet::new(),
        }
    }
}

/// Tracks retry attempt counts and delays for the data-plane and metadata retry logic.
#[derive(Clone, Debug, Default)]
pub(crate) struct RetryState {
    /// Number of endpoint failover retries performed.
    pub failover_count: i32,

    /// Number of session-token retry attempts.
    pub session_retry_count: i32,

    /// Number of service-unavailable retry attempts.
    pub service_unavailable_count: i32,
}

/// Tracks throttle (429) retry state with exponential backoff.
#[derive(Clone, Debug)]
pub(crate) struct ThrottleState {
    /// Number of throttle retries performed.
    pub attempt: i32,

    /// Maximum number of throttle retries allowed.
    pub max_attempts: i32,

    /// Cumulative wait time used by throttle retries.
    pub cumulative_wait: Duration,

    /// Maximum cumulative wait time allowed.
    pub max_wait: Duration,
}

impl ThrottleState {
    fn new(max_attempts: i32, max_wait_seconds: i64) -> Self {
        Self {
            attempt: 0,
            max_attempts,
            cumulative_wait: Duration::ZERO,
            max_wait: Duration::seconds(max_wait_seconds),
        }
    }
}

impl Default for ThrottleState {
    fn default() -> Self {
        // Aligns with historical ResourceThrottleRetryPolicy behavior (200s max wait)
        Self::new(5, 200)
    }
}

/// Immutable metadata about the current operation, computed once at the start.
#[derive(Clone, Debug)]
pub(crate) struct OperationInfo {
    /// The type of operation being performed.
    pub operation_type: OperationType,

    /// The type of resource being operated on.
    pub resource_type: ResourceType,

    /// Whether the account/operation supports multiple write locations.
    pub can_use_multi_write: bool,

    /// Whether automatic endpoint discovery is enabled.
    pub endpoint_discovery_enabled: bool,

    /// Number of preferred locations available.
    pub preferred_location_count: usize,

    /// Regions excluded from routing.
    pub excluded_regions: Option<Vec<RegionName>>,
}

impl OperationInfo {
    /// Creates operation info from request properties and endpoint manager state.
    pub fn new(
        operation_type: OperationType,
        resource_type: ResourceType,
        can_use_multi_write: bool,
        preferred_location_count: usize,
        excluded_regions: Option<Vec<RegionName>>,
    ) -> Self {
        Self {
            operation_type,
            resource_type,
            can_use_multi_write,
            endpoint_discovery_enabled: true,
            preferred_location_count,
            excluded_regions,
        }
    }

    /// Whether this is a read-only operation.
    ///
    /// Delegates to [`OperationType::is_read_only`] — the compiler optimizes
    /// this to a single comparison instruction, so there is no need to
    /// pre-cache the result in a field.
    pub fn is_read_only(&self) -> bool {
        self.operation_type.is_read_only()
    }

    /// Whether this is a metadata (control-plane) operation.
    ///
    /// Delegates to [`ResourceType::is_meta_data`].
    pub fn is_metadata(&self) -> bool {
        self.resource_type.is_meta_data()
    }
}

/// Pairs an HTTP status code with a Cosmos sub-status code.
///
/// Many Cosmos retry and routing decisions depend on the combination of the
/// HTTP status and the proprietary `x-ms-substatus` header. This newtype
/// keeps them together and provides readable predicates for the common
/// status/sub-status pairs used in retry logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CosmosStatus {
    /// The HTTP status code from the response.
    pub status: StatusCode,

    /// The Cosmos sub-status code (from the `x-ms-substatus` header), if present.
    pub sub_status: Option<SubStatusCode>,
}

impl CosmosStatus {
    /// Creates a new `CosmosStatus` from an HTTP status and optional sub-status.
    pub fn new(status: StatusCode, sub_status: Option<SubStatusCode>) -> Self {
        Self { status, sub_status }
    }

    /// 403 + sub-status 3 — the write was forbidden (endpoint failover needed).
    pub fn is_write_forbidden(&self) -> bool {
        self.status == StatusCode::Forbidden
            && self.sub_status == Some(SubStatusCode::WRITE_FORBIDDEN)
    }

    /// 404 + sub-status 1002 — read session not available.
    pub fn is_read_session_not_available(&self) -> bool {
        self.status == StatusCode::NotFound
            && self.sub_status == Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
    }

    /// 503 — service unavailable.
    pub fn is_service_unavailable(&self) -> bool {
        self.status == StatusCode::ServiceUnavailable
    }

    /// 429 — too many requests (throttled).
    pub fn is_too_many_requests(&self) -> bool {
        self.status == StatusCode::TooManyRequests
    }

    /// 410 + sub-status 1022 — lease not found (partition moved).
    pub fn is_gone_lease_not_found(&self) -> bool {
        self.status == StatusCode::Gone && self.sub_status == Some(SubStatusCode::LEASE_NOT_FOUND)
    }

    /// 403 + sub-status 1008 — database account not found.
    pub fn is_account_not_found(&self) -> bool {
        self.status == StatusCode::Forbidden
            && self.sub_status == Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)
    }

    /// 500 — internal server error.
    pub fn is_internal_server_error(&self) -> bool {
        self.status == StatusCode::InternalServerError
    }

    /// Whether this is a service-unavailable-class error that should trigger
    /// preferred-location failover (503, 500 on reads, 410+LeaseNotFound).
    pub fn is_service_unavailable_class(&self, is_read_only: bool) -> bool {
        self.is_service_unavailable()
            || (self.is_internal_server_error() && is_read_only)
            || self.is_gone_lease_not_found()
    }
}

impl std::fmt::Display for CosmosStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sub_status {
            Some(sub) => write!(f, "{}/{}", self.status, sub),
            None => write!(f, "{}", self.status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_state_default_is_initial() {
        let state = RoutingState::default();
        assert_eq!(state.location_index, 0);
        assert!(state.use_preferred_locations);
        assert!(state.resolved_endpoint.is_none());
        assert!(state.failed_endpoints.is_empty());
    }

    #[test]
    fn retry_state_default_has_zero_counts() {
        let state = RetryState::default();
        assert_eq!(state.failover_count, 0);
        assert_eq!(state.session_retry_count, 0);
        assert_eq!(state.service_unavailable_count, 0);
    }

    #[test]
    fn throttle_state_default_values() {
        let state = ThrottleState::default();
        assert_eq!(state.attempt, 0);
        assert_eq!(state.max_attempts, 5);
        assert_eq!(state.cumulative_wait, Duration::ZERO);
    }

    #[test]
    fn operation_info_read_detection() {
        let info = OperationInfo::new(OperationType::Read, ResourceType::Documents, false, 2, None);
        assert!(info.is_read_only());
        assert!(!info.is_metadata());
    }

    #[test]
    fn operation_info_metadata_detection() {
        let info = OperationInfo::new(OperationType::Read, ResourceType::Databases, false, 2, None);
        assert!(info.is_metadata());
    }

    #[test]
    fn cosmos_status_display_with_sub_status() {
        let cs = CosmosStatus::new(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
        );
        let display = format!("{}", cs);
        assert!(display.contains("404"));
        assert!(display.contains("1002"));
    }

    #[test]
    fn cosmos_status_display_without_sub_status() {
        let cs = CosmosStatus::new(StatusCode::ServiceUnavailable, None);
        let display = format!("{}", cs);
        assert!(display.contains("503"));
    }

    #[test]
    fn cosmos_status_predicates() {
        assert!(
            CosmosStatus::new(StatusCode::Forbidden, Some(SubStatusCode::WRITE_FORBIDDEN))
                .is_write_forbidden()
        );
        assert!(CosmosStatus::new(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
        )
        .is_read_session_not_available());
        assert!(CosmosStatus::new(StatusCode::ServiceUnavailable, None).is_service_unavailable());
        assert!(CosmosStatus::new(StatusCode::TooManyRequests, None).is_too_many_requests());
        assert!(
            CosmosStatus::new(StatusCode::Gone, Some(SubStatusCode::LEASE_NOT_FOUND))
                .is_gone_lease_not_found()
        );
        assert!(CosmosStatus::new(
            StatusCode::Forbidden,
            Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)
        )
        .is_account_not_found());
    }

    #[test]
    fn cosmos_status_service_unavailable_class() {
        // 503 is always service-unavailable-class
        assert!(CosmosStatus::new(StatusCode::ServiceUnavailable, None)
            .is_service_unavailable_class(false));
        assert!(CosmosStatus::new(StatusCode::ServiceUnavailable, None)
            .is_service_unavailable_class(true));

        // 500 only on reads
        assert!(CosmosStatus::new(StatusCode::InternalServerError, None)
            .is_service_unavailable_class(true));
        assert!(!CosmosStatus::new(StatusCode::InternalServerError, None)
            .is_service_unavailable_class(false));

        // 410+LeaseNotFound always
        assert!(
            CosmosStatus::new(StatusCode::Gone, Some(SubStatusCode::LEASE_NOT_FOUND))
                .is_service_unavailable_class(false)
        );
    }
}
