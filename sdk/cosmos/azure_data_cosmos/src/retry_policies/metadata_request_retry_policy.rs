// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{get_substatus_code_from_error, get_substatus_code_from_response, RetryResult};
use crate::constants::SubStatusCode;
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use std::cmp::max;
use std::sync::Arc;
use tracing::trace;

/// Retry policy for handling metadata request failures.
#[derive(Debug)]
pub struct MetadataRequestRetryPolicy {
    /// An instance of GlobalEndpointManager.
    global_endpoint_manager: Arc<GlobalEndpointManager>,

    /// Defines the throttling retry policy that is used as the underlying retry policy.
    throttling_retry_policy: ResourceThrottleRetryPolicy,

    /// An integer defining the maximum retry count on unavailable endpoint.
    max_unavailable_endpoint_retry_count: i32,

    /// An instance containing the location endpoint where the partition key
    /// range http request will be sent over.
    retry_context: Option<MetadataRetryContext>,

    /// An integer capturing the current retry count on unavailable endpoint.
    unavailable_endpoint_retry_count: i32,
}

/// A helper struct containing the required attributes for metadata retry context.
#[derive(Clone, Debug)]
struct MetadataRetryContext {
    /// An integer defining the current retry location index.
    retry_location_index: i32,

    /// A boolean flag indicating if the request should retry on preferred locations.
    retry_request_on_preferred_locations: bool,
}

impl MetadataRequestRetryPolicy {
    /// Creates a new MetadataRequestRetryPolicy with the specified global endpoint manager.
    ///
    /// # Summary
    /// Initializes a metadata request retry policy that handles transient failures for metadata operations
    /// by retrying requests across multiple endpoints when service unavailable or internal server errors occur.
    /// The policy integrates with the global endpoint manager to route requests to alternative endpoints.
    ///
    /// # Arguments
    /// * `global_endpoint_manager` - The global endpoint manager for routing requests across regions
    ///
    /// # Returns
    /// A new instance of `MetadataRequestRetryPolicy` configured with:
    /// - Maximum unavailable endpoint retries based on preferred location count
    /// - Underlying throttling retry policy for 429 responses
    /// - Initial retry count set to zero
    pub fn new(global_endpoint_manager: GlobalEndpointManager) -> Self {
        Self {
            global_endpoint_manager: Arc::from(global_endpoint_manager.clone()),
            throttling_retry_policy: ResourceThrottleRetryPolicy::new(5, 200, 10),
            max_unavailable_endpoint_retry_count: max(
                global_endpoint_manager.preferred_location_count() as i32,
                1,
            ),
            retry_context: None,
            unavailable_endpoint_retry_count: 0,
        }
    }

    /// Method that is called before a request is sent to allow the retry policy implementation
    /// to modify the state of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request being sent to the service
    pub(crate) async fn before_send_request(&mut self, request: &mut CosmosRequest) {
        let _stat = self.global_endpoint_manager.refresh_location(false).await;

        // Clear the previous location-based routing directive
        request.request_context.clear_route_to_location();

        if let Some(ref ctx) = self.retry_context {
            let mut req_ctx = request.request_context.clone();
            req_ctx.route_to_location_index(
                ctx.retry_location_index,
                ctx.retry_request_on_preferred_locations,
            );
            request.request_context = req_ctx;
        }

        let metadata_location_endpoint = self
            .global_endpoint_manager
            .resolve_service_endpoint(request);

        trace!(
            "MetadataRequestThrottleRetryPolicy: Routing the metadata request to: {:?} for operation type: {:?} and resource type: {:?}.",
            metadata_location_endpoint,
            request.operation_type,
            request.resource_type
        );

        request
            .request_context
            .route_to_location_endpoint(metadata_location_endpoint);
    }

    /// Determines whether an HTTP request should be retried based on the response or error
    ///
    /// This method evaluates the result of an HTTP request attempt and decides whether
    /// the operation should be retried, and if so, how long to wait before the next attempt.
    ///
    /// # Arguments
    ///
    /// * `response` - A reference to the result of the HTTP request attempt. This can be:
    ///   - `Ok(RawResponse)` - A successful HTTP response (which may still indicate an error via status code)
    ///   - `Err(azure_core::Error)` - A network or client-side error
    ///
    /// # Returns
    ///
    /// A `RetryResult` indicating the retry decision.
    pub(crate) async fn should_retry(
        &mut self,
        response: &azure_core::Result<RawResponse>,
    ) -> RetryResult {
        match response {
            Ok(resp) if resp.status().is_server_error() || resp.status().is_client_error() => {
                self.should_retry_response(resp).await
            }
            Ok(_) => RetryResult::DoNotRetry,
            Err(err) => self.should_retry_error(err).await,
        }
    }

    /// Determines whether to retry a metadata operation that failed with an error.
    ///
    /// # Summary
    /// Evaluates the error to determine if it represents a transient failure (service unavailable,
    /// internal server error, lease not found, or database account not found) that can be retried
    /// on an alternative endpoint. Falls back to throttling retry logic for 429 responses.
    ///
    /// # Arguments
    /// * `err` - The error that occurred during the metadata operation
    ///
    /// # Returns
    /// A `RetryResult` indicating whether to retry and the delay duration:
    /// - `Retry { after: Duration::ZERO }` for retryable metadata errors
    /// - Delegates to throttling policy for other errors
    pub async fn should_retry_error(&mut self, err: &azure_core::Error) -> RetryResult {
        let status_code = err.http_status().unwrap_or(StatusCode::UnknownValue(0));
        let sub_status_code = get_substatus_code_from_error(err);

        let retry_result = self.should_retry_with_status_code(status_code, sub_status_code);
        if retry_result.is_retry() {
            return retry_result;
        }

        self.throttling_retry_policy.should_retry_error(err)
    }

    /// Determines whether to retry a metadata operation based on the HTTP response.
    ///
    /// # Summary
    /// Examines the HTTP response status code and sub-status to determine if the failure is transient
    /// (503 service unavailable, 500 internal server error, 410 lease not found, 403 database account
    /// not found) and can be retried on an alternative endpoint. Delegates to throttling policy for
    /// rate limiting (429) responses.
    ///
    /// # Arguments
    /// * `response` - The HTTP response received from the metadata operation
    ///
    /// # Returns
    /// A `RetryResult` indicating whether to retry and the delay duration:
    /// - `Retry { after: Duration::ZERO }` for retryable metadata failures
    /// - Delegates to throttling policy for rate limiting errors
    pub async fn should_retry_response(&mut self, response: &RawResponse) -> RetryResult {
        let status_code = response.status();
        let sub_status_code = get_substatus_code_from_response(&response.clone());

        let retry_result = self.should_retry_with_status_code(status_code, sub_status_code);
        if retry_result.is_retry() {
            return retry_result;
        }

        self.throttling_retry_policy.should_retry_response(response)
    }

    /// Core retry decision logic based on status code and sub-status code.
    ///
    /// # Summary
    /// Determines if a metadata request should be retried based on the combination of HTTP status code
    /// and Cosmos DB sub-status code. Retries are triggered for transient failures: 503 service unavailable,
    /// 500 internal server error, 410 with lease not found, or 403 with database account not found.
    /// If retry is allowed, increments the location index to route the next attempt to a different endpoint.
    ///
    /// # Arguments
    /// * `status_code` - The HTTP status code from the response
    /// * `sub_status_code` - The Cosmos DB specific sub-status code
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration::ZERO }` if the error is retryable and retry count not exceeded
    /// - `DoNotRetry` for non-retryable errors or if max retries exceeded
    fn should_retry_with_status_code(
        &mut self,
        status_code: StatusCode,
        sub_status_code: Option<SubStatusCode>,
    ) -> RetryResult {
        // Check for retryable status codes
        if (status_code == StatusCode::ServiceUnavailable
            || status_code == StatusCode::InternalServerError
            || (status_code == StatusCode::Gone
                && sub_status_code == Some(SubStatusCode::LeaseNotFound))
            || (status_code == StatusCode::Forbidden
                && sub_status_code == Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)))
            && self.increment_retry_index_on_unavailable_endpoint_for_metadata_read()
        {
            return RetryResult::Retry {
                after: Duration::ZERO,
            };
        }

        RetryResult::DoNotRetry
    }

    /// Increments the location index when an unavailable endpoint exception occurs, for any future read requests.
    ///
    /// # Returns
    ///
    /// A boolean flag indicating if the operation was successful.
    fn increment_retry_index_on_unavailable_endpoint_for_metadata_read(&mut self) -> bool {
        self.unavailable_endpoint_retry_count += 1;

        if self.unavailable_endpoint_retry_count > self.max_unavailable_endpoint_retry_count {
            trace!(
                "MetadataRequestThrottleRetryPolicy: Retry count: {} has exceeded the maximum permitted retry count on unavailable endpoint: {}.",
                self.unavailable_endpoint_retry_count,
                self.max_unavailable_endpoint_retry_count
            );
            return false;
        }

        trace!(
            "MetadataRequestThrottleRetryPolicy: Incrementing the metadata retry location index to: {}.",
            self.unavailable_endpoint_retry_count
        );

        self.retry_context = Some(MetadataRetryContext {
            retry_location_index: self.unavailable_endpoint_retry_count,
            retry_request_on_preferred_locations: true,
        });

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::partition_key::PartitionKey;
    use crate::regions;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use azure_core::http::headers::Headers;
    use azure_core::http::ClientOptions;
    use azure_core::Bytes;
    use std::borrow::Cow;

    fn create_test_endpoint_manager() -> GlobalEndpointManager {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![Cow::Borrowed("West US"), Cow::Borrowed("East US")],
            pipeline,
        )
    }

    fn create_test_endpoint_manager_no_locations() -> GlobalEndpointManager {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![],
            pipeline,
        )
    }

    fn create_test_endpoint_manager_with_preferred_locations() -> GlobalEndpointManager {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![
                regions::EAST_ASIA.into(),
                regions::WEST_US.into(),
                regions::NORTH_CENTRAL_US.into(),
            ],
            pipeline,
        )
    }

    fn create_test_policy() -> MetadataRequestRetryPolicy {
        let manager = create_test_endpoint_manager();
        MetadataRequestRetryPolicy::new(manager)
    }

    fn create_test_policy_no_locations() -> MetadataRequestRetryPolicy {
        let manager = create_test_endpoint_manager_no_locations();
        MetadataRequestRetryPolicy::new(manager)
    }

    fn create_test_policy_with_preferred_locations() -> MetadataRequestRetryPolicy {
        let manager = create_test_endpoint_manager_with_preferred_locations();
        MetadataRequestRetryPolicy::new(manager)
    }

    fn create_test_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        let mut request = CosmosRequest::builder(OperationType::Read, resource_link.clone())
            .partition_key(PartitionKey::from("test"))
            .build()
            .unwrap();

        request.request_context.location_endpoint_to_route =
            Some("https://test.documents.azure.com".parse().unwrap());
        request
    }

    fn create_raw_response(status_code: StatusCode) -> RawResponse {
        let headers = Headers::new();
        RawResponse::from_bytes(status_code, headers, Bytes::new())
    }

    fn create_error_with_status(status: StatusCode) -> azure_core::Error {
        let response = create_raw_response(status);
        azure_core::Error::new(
            azure_core::error::ErrorKind::HttpResponse {
                status: response.status(),
                error_code: None,
                raw_response: Some(Box::new(response)),
            },
            "Test error",
        )
    }

    #[test]
    fn test_new_policy_initialization() {
        let policy = create_test_policy_with_preferred_locations();
        assert_eq!(policy.max_unavailable_endpoint_retry_count, 3);
        assert_eq!(policy.unavailable_endpoint_retry_count, 0);
    }

    #[test]
    fn test_retry_context_none_initially() {
        let policy = create_test_policy();
        assert!(policy.retry_context.is_none());
    }

    #[tokio::test]
    async fn test_should_retry_service_unavailable_error() {
        let mut policy = create_test_policy_no_locations();
        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_error(&error).await;
        assert!(result.is_retry());
        if let RetryResult::Retry { after } = result {
            assert_eq!(after, Duration::ZERO);
        }
    }

    #[tokio::test]
    async fn test_should_retry_internal_server_error() {
        let mut policy = create_test_policy_with_preferred_locations();
        let error = create_error_with_status(StatusCode::InternalServerError);

        let result = policy.should_retry_error(&error).await;
        assert!(result.is_retry());
    }

    #[tokio::test]
    async fn test_should_retry_service_unavailable_response() {
        let mut policy = create_test_policy_with_preferred_locations();
        let response = create_raw_response(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_response(&response).await;
        assert!(result.is_retry());
    }

    #[tokio::test]
    async fn test_should_retry_internal_server_error_response() {
        let mut policy = create_test_policy_with_preferred_locations();
        let response = create_raw_response(StatusCode::InternalServerError);

        let result = policy.should_retry_response(&response).await;
        assert!(result.is_retry());
    }

    #[tokio::test]
    async fn test_should_not_retry_ok_response() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::Ok);

        let result = policy.should_retry_response(&response).await;
        assert!(!result.is_retry());
    }

    #[tokio::test]
    async fn test_should_not_retry_created_response() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::Created);

        let result = policy.should_retry_response(&response).await;
        assert!(!result.is_retry());
    }

    #[tokio::test]
    async fn test_increment_retry_index_on_unavailable_endpoint() {
        let mut policy = create_test_policy_with_preferred_locations();
        let initial_count = policy.unavailable_endpoint_retry_count;

        let result = policy.increment_retry_index_on_unavailable_endpoint_for_metadata_read();
        assert!(result);
        assert_eq!(policy.unavailable_endpoint_retry_count, initial_count + 1);
        assert!(policy.retry_context.is_some());
    }

    #[tokio::test]
    async fn test_increment_retry_exceeds_max_count() {
        let mut policy = create_test_policy_no_locations();

        // Exhaust retry attempts
        policy.unavailable_endpoint_retry_count = policy.max_unavailable_endpoint_retry_count + 1;

        let result = policy.increment_retry_index_on_unavailable_endpoint_for_metadata_read();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_retry_context_set_after_increment() {
        let mut policy = create_test_policy_no_locations();

        policy.increment_retry_index_on_unavailable_endpoint_for_metadata_read();

        assert!(policy.retry_context.is_some());
        if let Some(ctx) = &policy.retry_context {
            assert!(ctx.retry_request_on_preferred_locations);
            assert_eq!(
                ctx.retry_location_index,
                policy.unavailable_endpoint_retry_count
            );
        }
    }

    #[tokio::test]
    async fn test_should_retry_with_ok_result() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::Ok);

        let result = policy.should_retry(&Ok(response)).await;
        assert!(!result.is_retry());
    }

    #[tokio::test]
    async fn test_should_retry_with_server_error_result() {
        let mut policy = create_test_policy_no_locations();
        let response = create_raw_response(StatusCode::InternalServerError);

        let result = policy.should_retry(&Ok(response)).await;
        assert!(result.is_retry());
    }

    #[tokio::test]
    async fn test_should_retry_with_error_result() {
        let mut policy = create_test_policy_no_locations();
        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry(&Err(error)).await;
        assert!(result.is_retry());
    }

    #[tokio::test]
    async fn test_should_not_retry_bad_request() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::BadRequest);

        let result = policy.should_retry_response(&response).await;
        // Bad request is not retryable by metadata policy (throttling policy may handle it)
        assert!(!result.is_retry());
    }

    #[tokio::test]
    async fn test_should_not_retry_not_found() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::NotFound);

        let result = policy.should_retry_response(&response).await;
        // Not found is not retryable by metadata policy
        assert!(!result.is_retry());
    }

    #[tokio::test]
    async fn test_multiple_retries_increment_counter() {
        let mut policy = create_test_policy_no_locations();
        // Reset the counter to 0 to allow multiple increments
        policy.unavailable_endpoint_retry_count = 0;
        let initial_count = policy.unavailable_endpoint_retry_count;

        let error1 = create_error_with_status(StatusCode::ServiceUnavailable);
        let _result1 = policy.should_retry_error(&error1).await;
        assert_eq!(policy.unavailable_endpoint_retry_count, initial_count + 1);

        // Can't test second retry as it exceeds max_unavailable_endpoint_retry_count (which is 1)
        // So just verify the first increment worked
    }

    #[tokio::test]
    async fn test_before_send_request_clears_routing() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();

        // Set some routing info
        request.request_context.location_index_to_route = Some(5);

        policy.before_send_request(&mut request).await;

        // After before_send_request, routing should be updated
        assert!(request.request_context.location_endpoint_to_route.is_some());
    }

    #[tokio::test]
    async fn test_retry_context_affects_routing() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();

        // Set up retry context
        policy.retry_context = Some(MetadataRetryContext {
            retry_location_index: 1,
            retry_request_on_preferred_locations: true,
        });

        policy.before_send_request(&mut request).await;

        // Verify the request was updated with retry context
        assert!(request.request_context.location_endpoint_to_route.is_some());
    }

    #[test]
    fn test_policy_debug_format() {
        let policy = create_test_policy();
        let debug_str = format!("{:?}", policy);
        assert!(debug_str.contains("MetadataRequestRetryPolicy"));
    }

    #[test]
    fn test_retry_context_clone() {
        let ctx = MetadataRetryContext {
            retry_location_index: 3,
            retry_request_on_preferred_locations: false,
        };

        let cloned = ctx.clone();
        assert_eq!(ctx.retry_location_index, cloned.retry_location_index);
        assert_eq!(
            ctx.retry_request_on_preferred_locations,
            cloned.retry_request_on_preferred_locations
        );
    }
}
