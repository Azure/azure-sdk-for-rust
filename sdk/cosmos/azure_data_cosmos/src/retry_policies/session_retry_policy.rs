// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
//! Internal implementation for session read/write unavailable retry policy
//! in the Azure Cosmos database service.

use std::sync::Arc;

/// Operation types for Cosmos DB operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationType {
    Create,
    Delete,
    ExecuteJavaScript,
    Head,
    HeadFeed,
    Patch,
    Query,
    QueryPlan,
    Read,
    ReadFeed,
    Recreate,
    Replace,
    SqlQuery,
    Update,
    Upsert,
    Batch,
}

impl OperationType {
    /// Returns true if the operation is a read-only operation
    pub fn is_read_only_operation(&self) -> bool {
        matches!(
            self,
            OperationType::Read
                | OperationType::ReadFeed
                | OperationType::Head
                | OperationType::HeadFeed
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
        )
    }
}

/// Trait representing a partition key range wrapper
pub trait PartitionKeyRangeWrapper: Send + Sync {
    // Add methods as needed for partition key range operations
}

/// Trait representing a Cosmos request with routing capabilities
pub trait CosmosRequest: Send + Sync {
    fn operation_type(&self) -> &OperationType;
    fn clear_route_to_location(&mut self);
    fn route_to_location(&mut self, endpoint: &str);
    fn route_to_location_with_preferred_location_flag(&mut self, location_index: usize, use_preferred: bool);
    fn set_should_clear_session_token_on_session_read_failure(&mut self, should_clear: bool);
}

/// Trait representing a global endpoint manager
pub trait GlobalEndpointManager: Send + Sync {
    fn can_use_multiple_write_locations(&self, request: &dyn CosmosRequest) -> bool;
    fn resolve_service_endpoint_for_partition(
        &self,
        request: &dyn CosmosRequest,
        pk_range_wrapper: &dyn PartitionKeyRangeWrapper,
    ) -> String;
    fn get_ordered_read_locations(&self) -> Vec<String>;
    fn get_ordered_write_locations(&self) -> Vec<String>;
}

/// Trait representing an exception
pub trait CosmosHttpResponseError: Send + Sync {
    // Add methods as needed for exception handling
}

/// Session retry policy for handling read/write session unavailability
pub struct SessionRetryPolicy {
    global_endpoint_manager: Arc<dyn GlobalEndpointManager>,
    max_retry_attempt_count: u32,
    session_token_retry_count: u32,
    pk_range_wrapper: Arc<dyn PartitionKeyRangeWrapper>,
    pub retry_after_in_milliseconds: u64,
    endpoint_discovery_enable: bool,
    request: Option<Box<dyn CosmosRequest>>,
    can_use_multiple_write_locations: bool,
    location_endpoint: Option<String>,
}

impl SessionRetryPolicy {
    /// Maximum retry attempt count constant
    pub const MAX_RETRY_ATTEMPT_COUNT: u32 = 1;

    /// Retry after in milliseconds constant
    pub const RETRY_AFTER_IN_MILLISECONDS: u64 = 0;

    /// Creates a new SessionRetryPolicy
    ///
    /// # Arguments
    /// * `endpoint_discovery_enable` - Whether endpoint discovery is enabled
    /// * `global_endpoint_manager` - The global endpoint manager
    /// * `pk_range_wrapper` - The partition key range wrapper
    /// * `request` - Optional request object
    pub fn new(
        endpoint_discovery_enable: bool,
        global_endpoint_manager: Arc<dyn GlobalEndpointManager>,
        pk_range_wrapper: Arc<dyn PartitionKeyRangeWrapper>,
        request: Option<Box<dyn CosmosRequest>>,
    ) -> Self {
        let mut policy = Self {
            global_endpoint_manager,
            max_retry_attempt_count: Self::MAX_RETRY_ATTEMPT_COUNT,
            session_token_retry_count: 0,
            pk_range_wrapper,
            retry_after_in_milliseconds: Self::RETRY_AFTER_IN_MILLISECONDS,
            endpoint_discovery_enable,
            request: None,
            can_use_multiple_write_locations: false,
            location_endpoint: None,
        };

        if let Some(mut req) = request {
            policy.can_use_multiple_write_locations = policy
                .global_endpoint_manager
                .can_use_multiple_write_locations(req.as_ref());

            // Clear previous location-based routing directive
            req.clear_route_to_location();

            // Resolve the endpoint for the request and pin the resolution to the resolved endpoint
            // This enables marking the endpoint unavailability on endpoint failover/unreachability
            let location_endpoint = policy
                .global_endpoint_manager
                .resolve_service_endpoint_for_partition(req.as_ref(), policy.pk_range_wrapper.as_ref());

            req.route_to_location(&location_endpoint);
            policy.location_endpoint = Some(location_endpoint);
            policy.request = Some(req);
        }

        policy
    }

    /// Returns true if the request should retry based on the passed-in exception.
    ///
    /// # Arguments
    /// * `_exception` - The CosmosHttpResponseError instance
    ///
    /// # Returns
    /// A boolean stating whether the request should be retried
    pub fn should_retry(&mut self, _exception: &dyn CosmosHttpResponseError) -> bool {
        let request = match self.request.as_mut() {
            Some(req) => req,
            None => return false,
        };

        self.session_token_retry_count += 1;

        // Clear previous location-based routing directive
        request.clear_route_to_location();

        if !self.endpoint_discovery_enable {
            // If endpoint discovery is disabled, the request cannot be retried anywhere else
            return false;
        }

        if self.can_use_multiple_write_locations {
            let locations = if request.operation_type().is_read_only_operation() {
                self.global_endpoint_manager.get_ordered_read_locations()
            } else {
                self.global_endpoint_manager.get_ordered_write_locations()
            };

            if self.session_token_retry_count as usize > locations.len() {
                // When use multiple write locations is true and the request has been tried
                // on all locations, then don't retry the request
                return false;
            }

            // Set location-based routing directive based on request retry context
            request.route_to_location_with_preferred_location_flag(
                (self.session_token_retry_count - 1) as usize,
                self.session_token_retry_count > self.max_retry_attempt_count,
            );

            // Clear on last attempt
            request.set_should_clear_session_token_on_session_read_failure(
                self.session_token_retry_count as usize == locations.len()
            );

            // Resolve the endpoint for the request and pin the resolution to the resolved endpoint
            // This enables marking the endpoint unavailability on endpoint failover/unreachability
            let location_endpoint = self
                .global_endpoint_manager
                .resolve_service_endpoint_for_partition(request.as_ref(), self.pk_range_wrapper.as_ref());

            request.route_to_location(&location_endpoint);
            self.location_endpoint = Some(location_endpoint);
            return true;
        }

        if self.session_token_retry_count > self.max_retry_attempt_count {
            // When cannot use multiple write locations, then don't retry the request if
            // we have already tried this request on the write location
            return false;
        }

        // Set location-based routing directive based on request retry context
        request.route_to_location_with_preferred_location_flag((self.session_token_retry_count - 1) as usize, false);
        request.set_should_clear_session_token_on_session_read_failure(true);

        // Resolve the endpoint for the request and pin the resolution to the resolved endpoint
        // This enables marking the endpoint unavailability on endpoint failover/unreachability
        let location_endpoint = self
            .global_endpoint_manager
            .resolve_service_endpoint_for_partition(request.as_ref(), self.pk_range_wrapper.as_ref());

        request.route_to_location(&location_endpoint);
        self.location_endpoint = Some(location_endpoint);
        true
    }

    /// Gets the maximum retry attempt count
    pub fn max_retry_attempt_count(&self) -> u32 {
        self.max_retry_attempt_count
    }

    /// Gets the current session token retry count
    pub fn session_token_retry_count(&self) -> u32 {
        self.session_token_retry_count
    }

    /// Gets whether endpoint discovery is enabled
    pub fn endpoint_discovery_enabled(&self) -> bool {
        self.endpoint_discovery_enable
    }

    /// Gets whether multiple write locations can be used
    pub fn can_use_multiple_write_locations(&self) -> bool {
        self.can_use_multiple_write_locations
    }

    /// Gets the current location endpoint
    pub fn location_endpoint(&self) -> Option<&String> {
        self.location_endpoint.as_ref()
    }

    /// Resets the retry policy state
    pub fn reset(&mut self) {
        self.session_token_retry_count = 0;
        self.location_endpoint = None;
        if let Some(ref mut request) = self.request {
            request.clear_route_to_location();
        }
    }

    /// Gets a reference to the request
    pub fn request(&self) -> Option<&dyn CosmosRequest> {
        self.request.as_ref().map(|r| r.as_ref())
    }

    /// Sets a new request
    pub fn set_request(&mut self, request: Option<Box<dyn CosmosRequest>>) {
        if let Some(mut req) = request {
            self.can_use_multiple_write_locations = self
                .global_endpoint_manager
                .can_use_multiple_write_locations(req.as_ref());

            req.clear_route_to_location();

            let location_endpoint = self
                .global_endpoint_manager
                .resolve_service_endpoint_for_partition(req.as_ref(), self.pk_range_wrapper.as_ref());

            req.route_to_location(&location_endpoint);
            self.location_endpoint = Some(location_endpoint);
            self.request = Some(req);
        } else {
            self.request = None;
            self.location_endpoint = None;
            self.can_use_multiple_write_locations = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Mock implementations for testing
    struct MockPartitionKeyRangeWrapper;
    impl PartitionKeyRangeWrapper for MockPartitionKeyRangeWrapper {}

    struct MockCosmosRequest {
        operation_type: OperationType,
        routed_location: Option<String>,
        clear_session_token: bool,
        preferred_location_calls: Vec<(usize, bool)>,
    }

    impl MockCosmosRequest {
        fn new(operation_type: OperationType) -> Self {
            Self {
                operation_type,
                routed_location: None,
                clear_session_token: false,
                preferred_location_calls: Vec::new(),
            }
        }
    }

    impl CosmosRequest for MockCosmosRequest {
        fn operation_type(&self) -> &OperationType {
            &self.operation_type
        }

        fn clear_route_to_location(&mut self) {
            self.routed_location = None;
        }

        fn route_to_location(&mut self, endpoint: &str) {
            self.routed_location = Some(endpoint.to_string());
        }

        fn route_to_location_with_preferred_location_flag(&mut self, location_index: usize, use_preferred: bool) {
            self.preferred_location_calls.push((location_index, use_preferred));
        }

        fn set_should_clear_session_token_on_session_read_failure(&mut self, should_clear: bool) {
            self.clear_session_token = should_clear;
        }
    }

    struct MockGlobalEndpointManager {
        can_use_multiple_writes: bool,
        read_locations: Vec<String>,
        write_locations: Vec<String>,
        resolve_calls: Arc<Mutex<u32>>,
    }

    impl MockGlobalEndpointManager {
        fn new(can_use_multiple_writes: bool) -> Self {
            Self {
                can_use_multiple_writes,
                read_locations: vec!["read1".to_string(), "read2".to_string(), "read3".to_string()],
                write_locations: vec!["write1".to_string(), "write2".to_string()],
                resolve_calls: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl GlobalEndpointManager for MockGlobalEndpointManager {
        fn can_use_multiple_write_locations(&self, _request: &dyn CosmosRequest) -> bool {
            self.can_use_multiple_writes
        }

        fn resolve_service_endpoint_for_partition(
            &self,
            _request: &dyn CosmosRequest,
            _pk_range_wrapper: &dyn PartitionKeyRangeWrapper,
        ) -> String {
            let mut calls = self.resolve_calls.lock().unwrap();
            *calls += 1;
            format!("endpoint{}", *calls)
        }

        fn get_ordered_read_locations(&self) -> Vec<String> {
            self.read_locations.clone()
        }

        fn get_ordered_write_locations(&self) -> Vec<String> {
            self.write_locations.clone()
        }
    }

    struct MockCosmosHttpResponseError;
    impl CosmosHttpResponseError for MockCosmosHttpResponseError {}

    #[test]
    fn test_new_policy_with_request() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Read));

        let policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, Some(request));

        assert_eq!(policy.max_retry_attempt_count(), 1);
        assert_eq!(policy.session_token_retry_count(), 0);
        assert!(policy.endpoint_discovery_enabled());
        assert!(policy.can_use_multiple_write_locations());
        assert!(policy.location_endpoint().is_some());
        assert!(policy.request().is_some());
    }

    #[test]
    fn test_new_policy_without_request() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(false));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);

        let policy = SessionRetryPolicy::new(false, endpoint_manager, pk_wrapper, None);

        assert!(!policy.endpoint_discovery_enabled());
        assert!(!policy.can_use_multiple_write_locations());
        assert!(policy.location_endpoint().is_none());
        assert!(policy.request().is_none());
    }

    #[test]
    fn test_should_retry_no_request() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let mut policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, None);
        let exception = MockCosmosHttpResponseError;

        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 0);
    }

    #[test]
    fn test_should_retry_endpoint_discovery_disabled() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Read));
        let mut policy = SessionRetryPolicy::new(false, endpoint_manager, pk_wrapper, Some(request));
        let exception = MockCosmosHttpResponseError;

        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 1);
    }

    #[test]
    fn test_should_retry_multiple_write_locations_read_operation() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Read));
        let mut policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, Some(request));
        let exception = MockCosmosHttpResponseError;

        // First retry should succeed
        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 1);

        // Continue retrying until all read locations are exhausted
        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 2);

        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 3);

        // Should fail after all locations are tried
        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 4);
    }

    #[test]
    fn test_should_retry_multiple_write_locations_write_operation() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Create));
        let mut policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, Some(request));
        let exception = MockCosmosHttpResponseError;

        // Should succeed for write locations
        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 1);

        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 2);

        // Should fail after all write locations are tried
        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 3);
    }

    #[test]
    fn test_should_retry_single_write_location() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(false));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Read));
        let mut policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, Some(request));
        let exception = MockCosmosHttpResponseError;

        // First retry should succeed
        assert!(policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 1);

        // Second retry should fail (exceeded max attempts)
        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.session_token_retry_count(), 2);
    }

    #[test]
    fn test_reset() {
        let endpoint_manager = Arc::new(MockGlobalEndpointManager::new(true));
        let pk_wrapper = Arc::new(MockPartitionKeyRangeWrapper);
        let request = Box::new(MockCosmosRequest::new(OperationType::Read));
        let mut policy = SessionRetryPolicy::new(true, endpoint_manager, pk_wrapper, Some(request));
        let exception = MockCosmosHttpResponseError;

        // Make a retry
        policy.should_retry(&exception);
        assert_eq!(policy.session_token_retry_count(), 1);

        // Reset the policy
        policy.reset();
        assert_eq!(policy.session_token_retry_count(), 0);
        assert!(policy.location_endpoint().is_none());
    }

    #[test]
    fn test_constants() {
        assert_eq!(SessionRetryPolicy::MAX_RETRY_ATTEMPT_COUNT, 1);
        assert_eq!(SessionRetryPolicy::RETRY_AFTER_IN_MILLISECONDS, 0);
    }
}