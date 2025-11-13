// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::cmp::max;
use std::sync::Arc;
use super::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use tracing::trace;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use crate::constants::{SubStatusCode, SUB_STATUS};
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::retry_policies::get_substatus_code_from_error;

/// Retry policy for handling resource throttling (429 TooManyRequests) errors
///
/// This policy implements exponential backoff for 429 status codes, respecting both
/// maximum retry attempts and cumulative wait time limits. It's designed to handle
/// Azure Cosmos DB throttling scenarios where the service limits request rates.
/// # Example
/// ```ignore
/// use azure_data_cosmos::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
///
/// // Create a policy with 3 max retries, 100 second max wait, and backoff factor of 2
/// let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);
/// ```
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
    /// Creates a new ResourceThrottleRetryPolicy with the specified configuration
    ///
    /// # Arguments
    /// * `max_attempt_count` - Maximum number of retry attempts before giving up
    /// * `max_wait_time_secs` - Maximum total wait time (in seconds) across all retries
    /// * `backoff_delay_factor` - Multiplier for exponential backoff delay
    ///
    /// # Example
    /// ```ignore
    /// use azure_data_cosmos::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
    ///
    /// // Create a policy with 5 retries, 120 second max wait, and backoff factor of 3
    /// let policy = ResourceThrottleRetryPolicy::new(5, 120, 3);
    /// ```
    pub fn new(
        global_endpoint_manager: GlobalEndpointManager,
    ) -> Self {
        Self {
            global_endpoint_manager: Arc::from(global_endpoint_manager.clone()),
            throttling_retry_policy: ResourceThrottleRetryPolicy::new(5, 200, 10),
            max_unavailable_endpoint_retry_count: 1,
            retry_context: None,
            unavailable_endpoint_retry_count: max(global_endpoint_manager.preferred_locations.len() as i32, 1)
        }
    }

    /// Determines whether to retry an operation that failed with an exception
    ///
    /// Examines the error to check if it contains a 429 (TooManyRequests) HTTP status.
    /// If so, applies the retry logic with exponential backoff. Non-throttle errors
    /// are not retried by this policy.
    ///
    /// # Arguments
    /// * `err` - The error that occurred during the operation
    ///
    /// # Returns
    /// A `RetryResult` indicating if the request can be retried.
    ///
    /// # Note
    /// Currently uses a fixed 500ms base retry delay. Future versions may parse
    /// the `x-ms-retry-after-ms` header from the error context.
    pub async fn should_retry_error(&mut self, err: &azure_core::Error) -> RetryResult {

        let status_code = err.http_status().unwrap();
        let sub_status_code = get_substatus_code_from_error(err).unwrap();

        let retry_result = self.should_retry_with_status_code(status_code,  sub_status_code);
        if (retry_result.is_retry()) {
            return retry_result;
        }

        self.throttling_retry_policy.should_retry_error(err)
    }

    /// Determines whether to retry an operation based on the HTTP response
    ///
    /// Examines the response status code to check if it's 429 (TooManyRequests).
    /// If so, applies the retry logic with exponential backoff. Other status codes
    /// (including success codes) are not retried by this policy.
    ///
    /// # Arguments
    /// * `response` - The HTTP response received from the server
    ///
    /// # Returns
    /// A `RetryResult` indicating if the request can be retried.
    ///
    /// # Note
    /// Currently uses a fixed 500ms base retry delay. Future versions may parse
    /// the `x-ms-retry-after-ms` header from the response headers to respect
    /// server-suggested retry delays.
    pub async fn should_retry_response(&mut self, response: &RawResponse) -> RetryResult {

        let status_code = response.status();
        let sub_status_code = response.headers()
            .get_as(&SUB_STATUS)
            .ok()
            .and_then(|raw: u16| SubStatusCode::try_from(raw).ok())
            .unwrap();

        let retry_result = self.should_retry_with_status_code(status_code,  sub_status_code);
        if (retry_result.is_retry()) {
            return retry_result;
        }

        self.throttling_retry_policy.should_retry_response(response)
    }

    fn should_retry_with_status_code(&mut self, status_code: StatusCode, sub_status_code: SubStatusCode) -> RetryResult {        // Check for retryable status codes
        if status_code == StatusCode::ServiceUnavailable
            || status_code == StatusCode::InternalServerError
            || (status_code == StatusCode::Gone && sub_status_code == SubStatusCode::LeaseNotFound)
            || (status_code == StatusCode::Forbidden && sub_status_code == SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)
        {
            if self.increment_retry_index_on_unavailable_endpoint_for_metadata_read() {
                return RetryResult::Retry { after: Duration::ZERO };
            }
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

#[async_trait]
impl RetryPolicy for MetadataRequestRetryPolicy {
    /// Method that is called before a request is sent to allow the retry policy implementation
    /// to modify the state of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request being sent to the service
    async fn before_send_request(&mut self, request: &mut CosmosRequest) {

        let _stat = self.global_endpoint_manager.refresh_location_async(false).await;

        // Clear the previous location-based routing directive
        request.request_context.clear_route_to_location();

        if let Some(ref ctx) = self.retry_context {
            let mut req_ctx = request.request_context.clone();
            req_ctx.route_to_location_index(ctx.retry_location_index, ctx.retry_request_on_preferred_locations);
            request.request_context = req_ctx;
        }

        let metadata_location_endpoint = self.global_endpoint_manager.resolve_service_endpoint(request);

        trace!(
            "MetadataRequestThrottleRetryPolicy: Routing the metadata request to: {:?} for operation type: {:?} and resource type: {:?}.",
            metadata_location_endpoint,
            request.operation_type,
            request.resource_type
        );

        request.request_context.route_to_location_endpoint(metadata_location_endpoint.parse().unwrap());
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
    async fn should_retry(&mut self, response: &azure_core::Result<RawResponse>) -> RetryResult {
        match response {
            Ok(resp) if resp.status().is_server_error() || resp.status().is_client_error() => {
                self.should_retry_response(resp).await
            }
            Ok(_) => RetryResult::DoNotRetry,
            Err(err) => self.should_retry_error(err).await,
        }
    }
}