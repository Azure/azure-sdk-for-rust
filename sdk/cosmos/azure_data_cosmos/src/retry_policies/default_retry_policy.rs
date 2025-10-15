// The MIT License (MIT)
// Copyright (c) Microsoft Corporation. All rights reserved.

//! Internal implementation for connection reset retry policy in the Azure
//! Cosmos database service.

use std::collections::HashSet;

/// Error codes constants
pub mod error_codes {
    // Windows Socket Error Codes
    pub const WINDOWS_INTERRUPTED_FUNCTION_CALL: i32 = 10004;
    pub const WINDOWS_FILE_HANDLE_NOT_VALID: i32 = 10009;
    pub const WINDOWS_PERMISSION_DENIED: i32 = 10013;
    pub const WINDOWS_BAD_ADDRESS: i32 = 10014;
    pub const WINDOWS_INVALID_ARGUMENT: i32 = 10022;
    pub const WINDOWS_RESOURCE_TEMPORARILY_UNAVAILABLE: i32 = 10035;
    pub const WINDOWS_OPERATION_NOW_IN_PROGRESS: i32 = 10036;
    pub const WINDOWS_ADDRESS_ALREADY_IN_USE: i32 = 10048;
    pub const WINDOWS_CONNECTION_RESET_BY_PEER: i32 = 10054;
    pub const WINDOWS_CANNOT_SEND_AFTER_SOCKET_SHUTDOWN: i32 = 10058;
    pub const WINDOWS_CONNECTION_TIMED_OUT: i32 = 10060;
    pub const WINDOWS_CONNECTION_REFUSED: i32 = 10061;
    pub const WINDOWS_NAME_TOO_LONG: i32 = 10063;
    pub const WINDOWS_HOST_IS_DOWN: i32 = 10064;
    pub const WINDOWS_NO_ROUTE_TO_HOST: i32 = 10065;

    // Linux Error Codes
    pub const LINUX_CONNECTION_RESET: i32 = 131;
}

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
    /// Returns true if the operation is a write operation
    pub fn is_write_operation(&self) -> bool {
        matches!(
            self,
            OperationType::Create
                | OperationType::Delete
                | OperationType::Recreate
                | OperationType::ExecuteJavaScript
                | OperationType::Replace
                | OperationType::Upsert
                | OperationType::Update
                | OperationType::Batch
                | OperationType::Patch
        )
    }

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

/// Trait representing a request with an operation type
pub trait CosmosRequest {
    fn operation_type(&self) -> &OperationType;
}

/// Trait representing an exception with a status code
pub trait CosmosHttpResponseError {
    fn status_code(&self) -> i32;
}

/// Default retry policy for handling connection errors in Cosmos DB requests
pub struct DefaultRetryPolicy<T: CosmosRequest> {
    max_retry_attempt_count: u32,
    pub current_retry_attempt_count: u32,
    pub retry_after_in_milliseconds: u64,
    connection_error_codes: HashSet<i32>,
    request: Option<T>,
}

impl<T: CosmosRequest> DefaultRetryPolicy<T> {
    /// Creates a new DefaultRetryPolicy
    ///
    /// # Arguments
    /// * `request` - Optional request object with operation type information
    pub fn new(request: Option<T>) -> Self {
        let connection_error_codes = [
            error_codes::WINDOWS_INTERRUPTED_FUNCTION_CALL,
            error_codes::WINDOWS_FILE_HANDLE_NOT_VALID,
            error_codes::WINDOWS_PERMISSION_DENIED,
            error_codes::WINDOWS_BAD_ADDRESS,
            error_codes::WINDOWS_INVALID_ARGUMENT,
            error_codes::WINDOWS_RESOURCE_TEMPORARILY_UNAVAILABLE,
            error_codes::WINDOWS_OPERATION_NOW_IN_PROGRESS,
            error_codes::WINDOWS_ADDRESS_ALREADY_IN_USE,
            error_codes::WINDOWS_CONNECTION_RESET_BY_PEER,
            error_codes::WINDOWS_CANNOT_SEND_AFTER_SOCKET_SHUTDOWN,
            error_codes::WINDOWS_CONNECTION_TIMED_OUT,
            error_codes::WINDOWS_CONNECTION_REFUSED,
            error_codes::WINDOWS_NAME_TOO_LONG,
            error_codes::WINDOWS_HOST_IS_DOWN,
            error_codes::WINDOWS_NO_ROUTE_TO_HOST,
            error_codes::LINUX_CONNECTION_RESET,
        ]
            .iter()
            .cloned()
            .collect();

        Self {
            max_retry_attempt_count: 10,
            current_retry_attempt_count: 0,
            retry_after_in_milliseconds: 1000,
            connection_error_codes,
            request,
        }
    }

    /// Creates a new DefaultRetryPolicy with custom parameters
    ///
    /// # Arguments
    /// * `max_retry_attempt_count` - Maximum number of retry attempts
    /// * `retry_after_in_milliseconds` - Retry delay in milliseconds
    /// * `request` - Optional request object with operation type information
    pub fn with_config(
        max_retry_attempt_count: u32,
        retry_after_in_milliseconds: u64,
        request: Option<T>,
    ) -> Self {
        let mut policy = Self::new(request);
        policy.max_retry_attempt_count = max_retry_attempt_count;
        policy.retry_after_in_milliseconds = retry_after_in_milliseconds;
        policy
    }

    /// Determines if a retry is needed based on the error code
    ///
    /// # Arguments
    /// * `error_code` - The error code from the exception
    ///
    /// # Returns
    /// True if the error is a connection error that should be retried
    pub fn needs_retry(&self, error_code: i32) -> bool {
        if self.connection_error_codes.contains(&error_code) {
            if let Some(ref request) = self.request {
                // If we have a request, only retry for read-only operations
                return request.operation_type().is_read_only_operation();
            }
            // If no request context, allow retry for connection errors
            return true;
        }
        false
    }

    /// Returns true if the request should retry based on the passed-in exception.
    ///
    /// # Arguments
    /// * `exception` - The CosmosHttpResponseError instance
    ///
    /// # Returns
    /// A boolean stating whether the request should be retried
    pub fn should_retry(&mut self, exception: &dyn CosmosHttpResponseError) -> bool {
        if self.current_retry_attempt_count < self.max_retry_attempt_count
            && self.needs_retry(exception.status_code())
        {
            self.current_retry_attempt_count += 1;
            return true;
        }
        false
    }

    /// Gets the maximum retry attempt count
    pub fn max_retry_attempt_count(&self) -> u32 {
        self.max_retry_attempt_count
    }

    /// Gets the connection error codes that trigger retries
    pub fn connection_error_codes(&self) -> &HashSet<i32> {
        &self.connection_error_codes
    }

    /// Resets the retry policy state
    pub fn reset(&mut self) {
        self.current_retry_attempt_count = 0;
    }

    /// Updates the request context
    pub fn set_request(&mut self, request: Option<T>) {
        self.request = request;
    }

    /// Gets a reference to the current request
    pub fn request(&self) -> Option<&T> {
        self.request.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock request for testing
    #[derive(Debug, Clone)]
    struct MockCosmosRequest {
        operation_type: OperationType,
    }

    impl MockCosmosRequest {
        fn new(operation_type: OperationType) -> Self {
            Self { operation_type }
        }
    }

    impl CosmosRequest for MockCosmosRequest {
        fn operation_type(&self) -> &OperationType {
            &self.operation_type
        }
    }

    // Mock exception for testing
    struct MockCosmosHttpResponseError {
        status_code: i32,
    }

    impl MockCosmosHttpResponseError {
        fn new(status_code: i32) -> Self {
            Self { status_code }
        }
    }

    impl CosmosHttpResponseError for MockCosmosHttpResponseError {
        fn status_code(&self) -> i32 {
            self.status_code
        }
    }

    #[test]
    fn test_new_policy() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let policy = DefaultRetryPolicy::new(Some(request));

        assert_eq!(policy.max_retry_attempt_count(), 10);
        assert_eq!(policy.current_retry_attempt_count, 0);
        assert_eq!(policy.retry_after_in_milliseconds, 1000);
        assert!(policy.request().is_some());
    }

    #[test]
    fn test_operation_type_classification() {
        // Test read-only operations
        assert!(OperationType::Read.is_read_only_operation());
        assert!(OperationType::Query.is_read_only_operation());
        assert!(OperationType::Head.is_read_only_operation());
        assert!(!OperationType::Read.is_write_operation());

        // Test write operations
        assert!(OperationType::Create.is_write_operation());
        assert!(OperationType::Update.is_write_operation());
        assert!(OperationType::Delete.is_write_operation());
        assert!(!OperationType::Create.is_read_only_operation());
    }

    #[test]
    fn test_needs_retry_with_read_only_operation() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let policy = DefaultRetryPolicy::new(Some(request));

        // Should retry for connection error with read-only operation
        assert!(policy.needs_retry(error_codes::WINDOWS_CONNECTION_TIMED_OUT));
        assert!(policy.needs_retry(error_codes::LINUX_CONNECTION_RESET));

        // Should not retry for non-connection errors
        assert!(!policy.needs_retry(404)); // Not Found
        assert!(!policy.needs_retry(500)); // Internal Server Error
    }

    #[test]
    fn test_needs_retry_with_write_operation() {
        let request = MockCosmosRequest::new(OperationType::Create);
        let policy = DefaultRetryPolicy::new(Some(request));

        // Should NOT retry for connection error with write operation
        assert!(!policy.needs_retry(error_codes::WINDOWS_CONNECTION_TIMED_OUT));
        assert!(!policy.needs_retry(error_codes::LINUX_CONNECTION_RESET));
    }

    #[test]
    fn test_needs_retry_without_request() {
        let policy: DefaultRetryPolicy<MockCosmosRequest> = DefaultRetryPolicy::new(None);

        // Should retry for connection errors when no request context
        assert!(policy.needs_retry(error_codes::WINDOWS_CONNECTION_TIMED_OUT));
        assert!(policy.needs_retry(error_codes::LINUX_CONNECTION_RESET));

        // Should not retry for non-connection errors
        assert!(!policy.needs_retry(404));
    }

    #[test]
    fn test_should_retry_success_cases() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let mut policy = DefaultRetryPolicy::new(Some(request));
        let exception = MockCosmosHttpResponseError::new(error_codes::WINDOWS_CONNECTION_TIMED_OUT);

        // First retry should succeed
        assert!(policy.should_retry(&exception));
        assert_eq!(policy.current_retry_attempt_count, 1);

        // Subsequent retries should succeed up to max attempts
        for i in 2..=10 {
            assert!(policy.should_retry(&exception));
            assert_eq!(policy.current_retry_attempt_count, i);
        }

        // 11th retry should fail (exceeded max attempts)
        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.current_retry_attempt_count, 10);
    }

    #[test]
    fn test_should_retry_with_non_connection_error() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let mut policy = DefaultRetryPolicy::new(Some(request));
        let exception = MockCosmosHttpResponseError::new(404); // Not a connection error

        // Should not retry for non-connection errors
        assert!(!policy.should_retry(&exception));
        assert_eq!(policy.current_retry_attempt_count, 0);
    }

    #[test]
    fn test_reset() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let mut policy = DefaultRetryPolicy::new(Some(request));
        let exception = MockCosmosHttpResponseError::new(error_codes::WINDOWS_CONNECTION_TIMED_OUT);

        // Make some retries
        policy.should_retry(&exception);
        policy.should_retry(&exception);
        assert_eq!(policy.current_retry_attempt_count, 2);

        // Reset the policy
        policy.reset();
        assert_eq!(policy.current_retry_attempt_count, 0);
    }

    #[test]
    fn test_with_config() {
        let request = MockCosmosRequest::new(OperationType::Read);
        let policy = DefaultRetryPolicy::with_config(5, 2000, Some(request));

        assert_eq!(policy.max_retry_attempt_count(), 5);
        assert_eq!(policy.retry_after_in_milliseconds, 2000);
    }

    #[test]
    fn test_connection_error_codes_coverage() {
        let policy: DefaultRetryPolicy<MockCosmosRequest> = DefaultRetryPolicy::new(None);
        let error_codes = policy.connection_error_codes();

        // Verify all expected error codes are present
        assert!(error_codes.contains(&error_codes::WINDOWS_INTERRUPTED_FUNCTION_CALL));
        assert!(error_codes.contains(&error_codes::WINDOWS_CONNECTION_TIMED_OUT));
        assert!(error_codes.contains(&error_codes::LINUX_CONNECTION_RESET));

        // Verify the count matches what we expect
        assert_eq!(error_codes.len(), 16);
    }
}