// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, Mutex};
use super::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use url::Url;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;

#[derive(Clone, Debug)]
struct RetryContext {
    retry_location_index: i32,
    retry_request_on_preferred_locations: bool,
    route_to_hub: bool,
}

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
pub struct ClientRetryPolicy {
    max_attempt_count: usize,
    backoff_delay_factor: u32,
    max_wait_time: Duration,
    current_attempt_count: usize,
    cumulative_retry_delay: usize,
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    enable_endpoint_discovery: bool,
    is_thin_client_enabled: bool,

    // Mutable state
    failover_retry_count: usize,
    session_token_retry_count: usize,
    service_unavailable_retry_count: usize,
    is_read_request: bool,
    can_use_multiple_write_locations: bool,
    is_multi_master_write_request: bool,
    location_endpoint: Option<String>,
    retry_context: Option<RetryContext>,
    cosmos_request: Option<Arc<Mutex<CosmosRequest>>>,
}

impl ClientRetryPolicy {
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
        max_attempt_count: usize,
        max_wait_time_secs: i64,
        backoff_delay_factor: u32,
        global_endpoint_manager: GlobalEndpointManager,
    ) -> Self {
        Self {
            max_attempt_count,
            backoff_delay_factor,
            max_wait_time: Duration::seconds(max_wait_time_secs),
            current_attempt_count: 0,
            cumulative_retry_delay: 0,
            global_endpoint_manager: Arc::new(global_endpoint_manager),
            enable_endpoint_discovery: false,
            is_thin_client_enabled: false,
            failover_retry_count: 0,
            session_token_retry_count: 0,
            service_unavailable_retry_count: 0,
            is_read_request: false,
            can_use_multiple_write_locations: false,
            is_multi_master_write_request: false,
            location_endpoint: None,
            retry_context: None,
            cosmos_request: None,
        }
    }

    /// Determines if a retry should be attempted based on timing constraints
    ///
    /// This method calculates the retry delay with exponential backoff and checks
    /// if the cumulative wait time would exceed the configured maximum.
    ///
    /// # Arguments
    /// * `retry_after` - Optional duration suggested by the server (from Retry-After header)
    ///
    /// # Returns
    /// A `RetryResult` indicating if the request can be retried.`
    fn check_if_retry_needed(&mut self, retry_after: Option<Duration>) -> RetryResult {
        let retry_delay = retry_after.unwrap_or(Duration::seconds(5)) * self.backoff_delay_factor;

        let cumulative = self.cumulative_retry_delay as u64;
        let cumulative = cumulative + retry_delay.as_seconds_f64() as u64;

        if retry_delay < self.max_wait_time
            && cumulative <= self.max_wait_time.as_seconds_f64() as u64
        {
            self.cumulative_retry_delay = cumulative as usize;
            return RetryResult::Retry { after: retry_delay };
        }
        RetryResult::DoNotRetry
    }

    /// Common retry logic for both errors and responses with exponential backoff
    ///
    /// This method encapsulates the shared retry decision logic used by both
    /// `should_retry_error` and `should_retry_response`. It provides a centralized
    /// place for retry decision-making to ensure consistency across different failure modes.
    fn should_retry_with_backoff(&mut self, retry_after: Option<Duration>) -> RetryResult {
        let attempt = self.current_attempt_count;
        if attempt < self.max_attempt_count {
            tracing::trace!(
                "Current retry attempt: {:?}, backoff time: {:?}",
                attempt,
                retry_after
            );
            let retry_result = self.check_if_retry_needed(retry_after);

            if retry_result.is_retry() {
                self.current_attempt_count += 1;
                return retry_result;
            }
        }

        tracing::trace!(max_attempt_count = self.max_attempt_count, "Exhausted all retry attempts and reached maximum retry. The request will not be retried.");
        RetryResult::DoNotRetry
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
    fn should_retry_error(&mut self, err: &azure_core::Error) -> RetryResult {
        // Check if the error has an HTTP status code and if it's a valid throttle status
        // Early return for invalid or missing status codes
        if err.http_status() == Some(StatusCode::TooManyRequests) {
            // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
            return self.should_retry_with_backoff(Some(Duration::milliseconds(500)));
        }

        RetryResult::DoNotRetry
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
    fn should_retry_response(&mut self, response: &RawResponse) -> RetryResult {
        if response.status() == StatusCode::TooManyRequests {
            // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
            return self.should_retry_with_backoff(Some(Duration::milliseconds(500)));
        }

        RetryResult::DoNotRetry
    }
}


#[async_trait]
impl RetryPolicy for ClientRetryPolicy {

    fn before_send_request(&mut self, request: &mut CosmosRequest) {

        self.is_read_request = request.is_read_only_request();
        self.can_use_multiple_write_locations = self.global_endpoint_manager.can_use_multiple_write_locations(request);

        self.is_multi_master_write_request = !self.is_read_request
            && self.global_endpoint_manager.can_support_multiple_write_locations(
            request.resource_type,
            request.operation_type,
        );

        // Clear previous location-based routing directive
        request.request_context.clear_route_to_location();

        if let Some(ref ctx) = self.retry_context {
            let mut req_ctx = request.request_context.clone();
            if ctx.route_to_hub {
                req_ctx.route_to_location_endpoint(request.resource_link.url(&Url::parse(&self.global_endpoint_manager.get_hub_uri()).unwrap()));
            } else {
                req_ctx.route_to_location_index(ctx.retry_location_index, ctx.retry_request_on_preferred_locations);
            }
        }

        // Resolve the endpoint for the request
        self.location_endpoint = Some(self.global_endpoint_manager.resolve_service_endpoint(request));

        if let Some(ref endpoint) = self.location_endpoint {
            request.request_context.route_to_location_endpoint(request.resource_link.url(&Url::parse(endpoint).unwrap()));
            // request.request_context.location_endpoint_to_route = Some(request.resource_link.url(&endpoint));
        }
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
                self.should_retry_response(resp)
            }
            Ok(_) => RetryResult::DoNotRetry,
            Err(err) => self.should_retry_error(err),
        }
    }
}
