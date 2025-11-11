// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::{Arc, Mutex};
use super::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use url::Url;
use azure_core::error::ErrorKind;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::http::headers::{HeaderName, Headers};
use azure_core::time::Duration;
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;

const RETRY_INTERVAL_MS: i64 = 1000;
const MAX_RETRY_COUNT: i32 = 120;
const MAX_SERVICE_UNAVAILABLE_RETRY_COUNT: i32 = 1;

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
    failover_retry_count: i32,
    session_token_retry_count: i32,
    service_unavailable_retry_count: i32,
    is_read_request: bool,
    can_use_multiple_write_locations: bool,
    is_multi_master_write_request: bool,
    location_endpoint: Option<String>,
    retry_context: Option<RetryContext>,
    cosmos_request: Option<Arc<Mutex<CosmosRequest>>>,
    throttling_retry: ResourceThrottleRetryPolicy,
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
            throttling_retry: ResourceThrottleRetryPolicy::new(5, 200, 10)
        }
    }

    fn should_retry_on_session_not_available(
        &mut self,
        cosmos_request: Option<Arc<Mutex<CosmosRequest>>>
    ) -> RetryResult {
        self.session_token_retry_count += 1;

        if !self.enable_endpoint_discovery {
            // If endpoint discovery is disabled, the request cannot be retried anywhere else
            return RetryResult::DoNotRetry;
        }

        if self.can_use_multiple_write_locations {
            // TODO: Update this to .get_applicable_endpoints(cosmos_request)
            let endpoints = self.global_endpoint_manager.preferred_locations.clone();

            if self.session_token_retry_count > endpoints.len() as i32 {
                // When use multiple write locations is true and the request has been tried
                // on all locations, then don't retry the request
                return RetryResult::DoNotRetry;
            } else {
                self.retry_context = Some(RetryContext {
                    retry_location_index: self.session_token_retry_count,
                    retry_request_on_preferred_locations: true,
                    route_to_hub: false,
                });

                return RetryResult::Retry {after: Duration::ZERO};
            }
        } else {
            if self.session_token_retry_count > 1 {
                // When cannot use multiple write locations, then don't retry the request if
                // we have already tried this request on the write location
                return RetryResult::DoNotRetry;
            } else {
                self.retry_context = Some(RetryContext {
                    retry_location_index: 0,
                    retry_request_on_preferred_locations: false,
                    route_to_hub: false,
                });

                return RetryResult::Retry {after: Duration::ZERO};
            }
        }
    }

    async fn should_retry_on_endpoint_failure_async(
        &mut self,
        is_read_request: bool,
        mark_both_read_and_write_as_unavailable: bool,
        force_refresh: bool,
        retry_on_preferred_locations: bool,
        overwrite_endpoint_discovery: bool,
    ) -> RetryResult {
        if self.failover_retry_count > MAX_RETRY_COUNT
            || (!self.enable_endpoint_discovery && !overwrite_endpoint_discovery)
        {
            return RetryResult::DoNotRetry;
        }

        self.failover_retry_count += 1;

        if let Some(ref endpoint) = self.location_endpoint {
            if !overwrite_endpoint_discovery {
                if is_read_request || mark_both_read_and_write_as_unavailable {
                    self.global_endpoint_manager.mark_endpoint_unavailable_for_read(endpoint);
                }
                if !is_read_request || mark_both_read_and_write_as_unavailable {
                    self.global_endpoint_manager.mark_endpoint_unavailable_for_write(endpoint);
                }
            }
        }

        let retry_delay = if !is_read_request {
            if self.failover_retry_count > 1 {
                Duration::milliseconds(RETRY_INTERVAL_MS)
            } else {
                Duration::ZERO
            }
        } else {
            Duration::milliseconds(RETRY_INTERVAL_MS)
        };

        let res = self.global_endpoint_manager.refresh_location_async(force_refresh).await;
        let retry_location_index = if retry_on_preferred_locations {
            0
        } else {
            self.failover_retry_count
        };

        self.retry_context = Some(RetryContext {
            retry_location_index,
            retry_request_on_preferred_locations: retry_on_preferred_locations,
            route_to_hub: false,
        });

        RetryResult::Retry { after: retry_delay }
    }

    fn should_retry_on_unavailable_endpoint_status_codes(&mut self) -> RetryResult {
        self.service_unavailable_retry_count += 1;

        if self.service_unavailable_retry_count > MAX_SERVICE_UNAVAILABLE_RETRY_COUNT {
            return RetryResult::DoNotRetry;
        }

        if !self.can_use_multiple_write_locations && !self.is_read_request
            // && !self.partition_key_range_location_cache.is_partition_level_automatic_failover_enabled() // Add PPAF Support
        {
            return RetryResult::DoNotRetry;
        }

        let available_preferred_locations = self.global_endpoint_manager.preferred_location_count();
        if available_preferred_locations <= 1 {
            return RetryResult::DoNotRetry;
        }

        self.retry_context = Some(RetryContext {
            retry_location_index: self.service_unavailable_retry_count,
            retry_request_on_preferred_locations: true,
            route_to_hub: false,
        });

        RetryResult::Retry { after: Duration::ZERO }
    }

    async fn should_retry_internal_async(
        &mut self,
        status_code: Option<StatusCode>,
        sub_status_code: Option<u32>,
    ) -> Option<RetryResult> {

        // Forbidden - Write forbidden (403.3)
        if status_code == Some(StatusCode::Forbidden) && sub_status_code == Some(3) {

            // TODO: Add Logic For Per Partition Automatic Failover.
            return Some(self.should_retry_on_endpoint_failure_async(
                false, // is_read_request
                false, // mark_both_read_and_write_as_unavailable
                true,  // force_refresh
                false, // retry_on_preferred_locations
                false, // overwrite_endpoint_discovery
            ).await);
        }

        // Request timeout (408)
        if status_code ==  Some(StatusCode::RequestTimeout) {
            // TODO: Handle Request Timeout.
        }

        // Read Session Not Available (404.1022)
        if status_code == Some(StatusCode::NotFound) && sub_status_code == Some(1002) {
            return Some(self.should_retry_on_session_not_available(self.cosmos_request.clone()));
        }

        // Service unavailable (503)
        if status_code == Some(StatusCode::ServiceUnavailable) {
            return Some(self.should_retry_on_unavailable_endpoint_status_codes());
        }

        // Internal server error (500) or Gone - Lease not found (410)
        if (status_code == Some(StatusCode::InternalServerError) && self.is_read_request)
            || (status_code == Some(StatusCode::Gone) && sub_status_code == Some(1022))
        {
            return Some(self.should_retry_on_unavailable_endpoint_status_codes());
        }

        None
    }

    fn extract_headers(err: &azure_core::Error) -> Option<&Headers> {
        if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
            raw_response.as_ref().map(|r| r.headers())
        } else {
            None
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
    async fn should_retry_error(&mut self, err: &azure_core::Error) -> RetryResult {
        let status_code = err.http_status();
        let sub_status_code = ClientRetryPolicy::extract_headers(err).unwrap().get_as(&HeaderName::from("x-ms-substatus")).ok();
        if let Some(result) = self.should_retry_internal_async(status_code, sub_status_code).await {
            return result;
        }

        self.throttling_retry.should_retry_error(err)
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
    async fn should_retry_response(&mut self, response: &RawResponse) -> RetryResult {
        let status_code = response.status();
        let sub_status_code = response.headers().get_as(&HeaderName::from("x-ms-substatus")).ok();
        if let Some(result) = self.should_retry_internal_async(Some(status_code), sub_status_code).await {
            return result;
        }

        self.throttling_retry.should_retry_response(response)
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
                self.should_retry_response(resp).await
            }
            Ok(_) => RetryResult::DoNotRetry,
            Err(err) => self.should_retry_error(err).await,
        }
    }
}
