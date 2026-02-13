// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data components for the pipeline execution model.
//!
//! These are plain data structs representing the distinct concerns of
//! request execution: routing state, retry tracking, throttle tracking,
//! and immutable operation metadata. Following DOP principles, these types
//! carry no behavior beyond construction and are transformed by free
//! functions in the [`super::retry_decisions`] module.

use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::ResourceType;
use azure_core::time::Duration;
use std::collections::HashSet;
use url::Url;

/// Tracks which endpoint the request is routed to and which endpoints have failed.
///
/// This component is updated between retry attempts to steer the next request
/// to an alternative region or endpoint.
#[derive(Clone, Debug)]
pub(crate) struct RoutingState {
    /// Index into the preferred-locations list for the next attempt.
    pub(crate) location_index: i32,

    /// Whether to use preferred locations for routing.
    pub(crate) use_preferred_locations: bool,

    /// The resolved endpoint URL for the current attempt.
    pub(super) resolved_endpoint: Option<Url>,

    /// Endpoints that have been tried and failed.
    pub(super) failed_endpoints: HashSet<Url>,
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
pub(super) struct RetryState {
    /// Number of endpoint failover retries performed.
    pub(super) failover_count: i32,

    /// Number of session-token retry attempts.
    pub(super) session_retry_count: i32,

    /// Number of service-unavailable retry attempts.
    pub(super) service_unavailable_count: i32,
}

/// Tracks throttle (429) retry state with exponential backoff.
#[derive(Clone, Debug)]
pub(super) struct ThrottleState {
    /// Number of throttle retries performed.
    pub(super) attempt: i32,

    /// Maximum number of throttle retries allowed.
    pub(super) max_attempts: i32,

    /// Cumulative wait time used by throttle retries.
    pub(super) cumulative_wait: Duration,

    /// Maximum cumulative wait time allowed.
    pub(super) max_wait: Duration,
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
        Self::new(5, 10)
    }
}

/// Immutable metadata about the current operation, computed once at the start.
#[derive(Clone, Debug)]
pub(crate) struct OperationInfo {
    /// The type of operation being performed.
    pub(crate) operation_type: OperationType,

    /// The type of resource being operated on.
    pub(crate) resource_type: ResourceType,

    /// Whether this is a read-only operation.
    pub(crate) is_read_only: bool,

    /// Whether the account/operation supports multiple write locations.
    pub(super) can_use_multi_write: bool,

    /// Whether automatic endpoint discovery is enabled.
    pub(super) endpoint_discovery_enabled: bool,

    /// Number of preferred locations available.
    pub(super) preferred_location_count: usize,

    /// Regions excluded from routing.
    pub(crate) excluded_regions: Option<Vec<RegionName>>,

    /// Whether this is a metadata (control-plane) operation.
    pub(super) is_metadata: bool,
}

impl OperationInfo {
    /// Creates operation info from request properties and endpoint manager state.
    pub(super) fn new(
        operation_type: OperationType,
        resource_type: ResourceType,
        can_use_multi_write: bool,
        preferred_location_count: usize,
        excluded_regions: Option<Vec<RegionName>>,
    ) -> Self {
        Self {
            operation_type,
            resource_type,
            is_read_only: operation_type.is_read_only(),
            can_use_multi_write,
            endpoint_discovery_enabled: true,
            preferred_location_count,
            excluded_regions,
            is_metadata: resource_type.is_meta_data(),
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
        assert!(info.is_read_only);
        assert!(!info.is_metadata);
    }

    #[test]
    fn operation_info_metadata_detection() {
        let info = OperationInfo::new(OperationType::Read, ResourceType::Databases, false, 2, None);
        assert!(info.is_metadata);
    }
}
