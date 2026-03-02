// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    get_substatus_code_from_error, get_substatus_code_from_response, is_non_retryable_status_code,
    resource_throttle_retry_policy::ResourceThrottleRetryPolicy, RequestSentExt, RequestSentStatus,
    RetryResult,
};
use crate::constants::{self, SubStatusCode};
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use azure_core::error::ErrorKind;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use std::sync::Arc;
use tracing::error;
use url::Url;

/// An integer indicating the default retry intervals between two retry attempts.
const RETRY_INTERVAL_MS: i64 = 1000;

/// An integer indicating the maximum retry count on endpoint failures.
const MAX_RETRY_COUNT_ON_ENDPOINT_FAILURE: usize = 120;

/// An integer indicating the maximum retry count on connection failures before marking
/// the endpoint unavailable.
const MAX_RETRY_COUNT_ON_CONNECTION_FAILURE: usize = 3;

/// Context information for routing retry attempts to specific endpoints.
#[derive(Clone, Debug)]
struct RetryContext {
    /// Index of the location to route the retry request to
    retry_location_index: usize,

    /// Whether to retry on preferred locations only (true) or all available locations (false)
    retry_request_on_preferred_locations: bool,

    /// Whether to route directly to the hub endpoint instead of using location-based routing
    route_to_hub: bool,
}

/// Retry policy for handling data plane request failures.
#[derive(Debug)]
pub(crate) struct ClientRetryPolicy {
    /// Manages multi-region endpoint routing and failover logic
    global_endpoint_manager: Arc<GlobalEndpointManager>,

    /// An instance of GlobalPartitionEndpointManager that manages partition key range to endpoint mapping
    partition_key_range_location_cache: Arc<GlobalPartitionEndpointManager>,

    /// Whether automatic endpoint discovery is enabled for failover scenarios
    enable_endpoint_discovery: bool,

    /// Counter tracking the number of endpoint failover retry attempts
    failover_retry_count: usize,

    /// Counter tracking the number of session token unavailability retry attempts
    session_token_retry_count: usize,

    /// Counter tracking the number of service unavailable (503) retry attempts
    service_unavailable_retry_count: usize,

    /// Counter tracking the number of consecutive connection failure retry attempts
    /// on the current endpoint before marking it unavailable.
    connection_retry_count: usize,

    /// Whether the current request is a read operation (true) or write operation (false)
    operation_type: Option<OperationType>,

    /// The Cosmos request being processed by the retry policy.
    request: Option<CosmosRequest>,

    /// Whether the account supports writing to multiple locations simultaneously
    can_use_multiple_write_locations: bool,

    /// The resolved endpoint URL for the current or next request attempt
    location_endpoint: Option<Url>,

    /// Context information for routing the next retry attempt to a specific location
    retry_context: Option<RetryContext>,

    /// Regions excluded from routing for the current request
    excluded_regions: Option<Vec<RegionName>>,

    /// Underlying policy for handling resource throttling (429) with exponential backoff
    throttling_retry: ResourceThrottleRetryPolicy,
}

impl ClientRetryPolicy {
    /// Creates a new ClientRetryPolicy instance.
    ///
    /// # Summary
    /// Initializes a retry policy that handles various failure scenarios including session token
    /// mismatches, endpoint failures, service unavailability, and resource throttling. The policy
    /// manages automatic endpoint discovery, multi-region failover, and coordinates with the
    /// GlobalEndpointManager for routing decisions. It wraps a ResourceThrottleRetryPolicy for
    /// handling 429 (TooManyRequests) responses with exponential backoff.
    ///
    /// # Arguments
    /// * `global_endpoint_manager` - The endpoint manager for handling multi-region routing and failover
    ///
    /// # Returns
    /// A new `ClientRetryPolicy` instance configured with default retry limits and throttling behavior
    pub fn new(
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        partition_key_range_location_cache: Arc<GlobalPartitionEndpointManager>,
        excluded_regions: Option<Vec<RegionName>>,
    ) -> Self {
        Self {
            global_endpoint_manager,
            partition_key_range_location_cache,
            enable_endpoint_discovery: true,
            failover_retry_count: 0,
            session_token_retry_count: 0,
            service_unavailable_retry_count: 0,
            connection_retry_count: 0,
            operation_type: None,
            request: None,
            can_use_multiple_write_locations: false,
            location_endpoint: None,
            retry_context: None,
            excluded_regions,
            throttling_retry: ResourceThrottleRetryPolicy::new(5, 200, 10),
        }
    }

    /// Returns whether the current operation is read-only.
    ///
    /// Defaults to `true` if the operation type has not been set, which is the
    /// conservative choice: reads are always safe to retry.
    fn is_read_only(&self) -> bool {
        self.operation_type.is_none_or(|op| op.is_read_only())
    }

    /// Prepares a request before it is sent, configuring routing and endpoint selection.
    ///
    /// # Summary
    /// Performs pre-flight setup for each request attempt by refreshing location cache,
    /// determining request characteristics (read vs write, multi-master support), and
    /// resolving the target endpoint based on retry context. Handles location-based routing
    /// directives, including retry attempts that target specific location indices or the hub
    /// endpoint. Clears previous routing context and configures the request with the
    /// appropriate endpoint URL for the current attempt.
    ///
    /// # Arguments
    /// * `request` - The mutable request to configure before sending
    pub(crate) async fn before_send_request(&mut self, request: &mut CosmosRequest) {
        // Ideally, any request flow should not be blocked by the outcome of refresh_location.
        // There can be three possible cases:
        // a) The refresh_location succeeds when TTL expires.
        // b) The refresh_location is bypassed when TTL hasn't expired.
        // c) The refresh_location operation has failed. In the event of a failure,
        //    the error is logged and the request should not be blocked.
        // Hence, the outcome of the operation is ignored here.
        _ = self.global_endpoint_manager.refresh_location(false).await;
        self.operation_type = Some(request.operation_type);
        self.excluded_regions = request.excluded_regions.clone();
        self.can_use_multiple_write_locations = self
            .global_endpoint_manager
            .can_use_multiple_write_locations(request);

        if self.can_use_multiple_write_locations {
            request
                .headers
                .insert(constants::ALLOW_TENTATIVE_WRITES, "true");
        } else {
            request.headers.remove(constants::ALLOW_TENTATIVE_WRITES);
        }

        // Clear previous location-based routing directive
        request.request_context.clear_route_to_location();

        if let Some(ref ctx) = self.retry_context {
            let mut req_ctx = request.request_context.clone();
            if ctx.route_to_hub {
                req_ctx.route_to_location_endpoint(
                    request
                        .resource_link
                        .url(self.global_endpoint_manager.hub_uri()),
                );
            } else {
                req_ctx.route_to_location_index(
                    ctx.retry_location_index,
                    ctx.retry_request_on_preferred_locations,
                );
            }
            request.request_context = req_ctx;
        }

        // Resolve the endpoint for the request
        self.location_endpoint = Some(
            self.global_endpoint_manager
                .resolve_service_endpoint(request),
        );

        tracing::trace!(
            ?self.location_endpoint,
            "routed request to endpoint"
        );

        if let Some(ref endpoint) = self.location_endpoint {
            request
                .request_context
                .route_to_location_endpoint(endpoint.clone());
        }

        if self
            .partition_key_range_location_cache
            .partition_level_failover_enabled()
            && request.resource_type.is_partitioned()
        {
            self.partition_key_range_location_cache
                .try_add_partition_level_location_override(request);
        }

        self.request = Some(request.clone());
    }

    /// Determines whether a Data Plane request should be retried based on the response or error
    ///
    /// # Summary
    /// Evaluates the result of a request attempt to determine if it should be retried.
    /// Distinguishes between successful responses (2xx), client/server error responses
    /// (4xx/5xx), and transport/network errors. Delegates error responses to
    /// `should_retry_response` and exceptions to `should_retry_error` for detailed
    /// evaluation. Non-error responses (2xx, 3xx) are not retried. This method is
    /// called by the retry framework after each request attempt.
    ///
    /// # Arguments
    /// * `response` - The result of the request attempt (Ok with response or Err with error)
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration }` if the request should be retried with specified delay
    /// - `DoNotRetry` for successful responses or non-retryable failures
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

    /// Determines if a request should be retried when session token is unavailable.
    ///
    /// # Summary
    /// Handles 404.1022 (READ_SESSION_NOT_AVAILABLE) errors by attempting to retry on different
    /// endpoints. For multi-write scenarios, tries all available endpoints before giving up.
    /// For single-write scenarios, retries once on the primary write location. Increments the
    /// session token retry counter and configures retry context for endpoint routing.
    ///
    /// # Arguments
    /// * `cosmos_request` - The original request that failed with session token unavailable
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration::ZERO }` if retry is allowed on a different endpoint
    /// - `DoNotRetry` if endpoint discovery is disabled or all endpoints have been tried
    fn should_retry_on_session_not_available(&mut self) -> RetryResult {
        self.session_token_retry_count += 1;

        // If endpoint discovery is disabled, the request cannot be retried anywhere else
        if !self.enable_endpoint_discovery {
            return RetryResult::DoNotRetry;
        }

        if self.can_use_multiple_write_locations {
            let endpoints = self.global_endpoint_manager.applicable_endpoints(
                self.operation_type.unwrap_or(OperationType::Read),
                self.excluded_regions.as_ref(),
            );
            if self.session_token_retry_count > endpoints.len() {
                // When use multiple write locations is true and the request has been tried on all locations, then don't retry the request.
                RetryResult::DoNotRetry
            } else {
                self.retry_context = Some(RetryContext {
                    retry_location_index: self.session_token_retry_count,
                    retry_request_on_preferred_locations: true,
                    route_to_hub: false,
                });

                RetryResult::Retry {
                    after: Duration::ZERO,
                }
            }
        } else if self.session_token_retry_count > 1 {
            // When cannot use multiple write locations, then don't retry the request if
            // we have already tried this request on the write location
            RetryResult::DoNotRetry
        } else {
            self.retry_context = Some(RetryContext {
                retry_location_index: 0,
                retry_request_on_preferred_locations: false,
                route_to_hub: false,
            });

            RetryResult::Retry {
                after: Duration::ZERO,
            }
        }
    }

    /// Determines if a request should be retried after a connection failure.
    ///
    /// Connection failures mean the request was never sent to the server, so both
    /// reads and writes are safe to retry. The strategy is:
    ///
    /// 1. Retry up to [`MAX_RETRY_COUNT_ON_CONNECTION_FAILURE`] times on the same
    ///    endpoint with a delay — the failure may be transient.
    /// 2. After exhausting local retries, mark the endpoint unavailable for both
    ///    reads and writes, refresh the location cache, and fail over to the next
    ///    available endpoint.
    async fn should_retry_on_connection_failure(&mut self) -> RetryResult {
        self.connection_retry_count += 1;

        if self.connection_retry_count <= MAX_RETRY_COUNT_ON_CONNECTION_FAILURE {
            // Retry on the same endpoint — the connection failure may be transient.
            return RetryResult::Retry {
                after: Duration::milliseconds(RETRY_INTERVAL_MS),
            };
        }

        // Exhausted local retries — mark endpoint unavailable and fail over.
        if let Some(ref endpoint) = self.location_endpoint {
            self.global_endpoint_manager
                .mark_endpoint_unavailable_for_read(endpoint);
            self.global_endpoint_manager
                .mark_endpoint_unavailable_for_write(endpoint);
        }

        self.failover_retry_count += 1;
        if self.failover_retry_count > MAX_RETRY_COUNT_ON_ENDPOINT_FAILURE
            || !self.enable_endpoint_discovery
        {
            return RetryResult::DoNotRetry;
        }

        _ = self.global_endpoint_manager.refresh_location(true).await;

        // Reset connection retry counter for the new endpoint.
        self.connection_retry_count = 0;

        self.retry_context = Some(RetryContext {
            retry_location_index: 0,
            retry_request_on_preferred_locations: true,
            route_to_hub: false,
        });

        RetryResult::Retry {
            after: Duration::ZERO,
        }
    }

    /// Determines if a request should be retried when an endpoint fails.
    ///
    /// # Summary
    /// Handles endpoint failures by marking failed endpoints as unavailable and attempting retry
    /// on alternative endpoints. Refreshes the location cache to get updated endpoint information
    /// and configures retry delays based on request type (write requests get longer delays).
    /// Respects maximum retry limits and endpoint discovery settings. Can mark endpoints as
    /// unavailable for reads, writes, or both depending on the failure scenario.
    ///
    /// # Arguments
    /// * `is_read_request` - Whether this is a read operation
    /// * `mark_both_read_and_write_as_unavailable` - Whether to mark the endpoint unavailable for both operations
    /// * `force_refresh` - Whether to force refresh of the location cache
    /// * `retry_on_preferred_locations` - Whether to retry on preferred locations first
    /// * `overwrite_endpoint_discovery` - Whether to bypass endpoint discovery checks
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration }` with appropriate delay if retry is allowed
    /// - `DoNotRetry` if max retry count exceeded or endpoint discovery disabled
    async fn should_retry_on_endpoint_failure(
        &mut self,
        is_read_request: bool,
        mark_both_read_and_write_as_unavailable: bool,
        force_refresh: bool,
        retry_on_preferred_locations: bool,
        overwrite_endpoint_discovery: bool,
    ) -> RetryResult {
        if self.failover_retry_count > MAX_RETRY_COUNT_ON_ENDPOINT_FAILURE
            || (!self.enable_endpoint_discovery && !overwrite_endpoint_discovery)
        {
            return RetryResult::DoNotRetry;
        }

        self.failover_retry_count += 1;

        if let Some(ref endpoint) = self.location_endpoint {
            if !overwrite_endpoint_discovery {
                if is_read_request || mark_both_read_and_write_as_unavailable {
                    self.global_endpoint_manager
                        .mark_endpoint_unavailable_for_read(endpoint);
                }
                if !is_read_request || mark_both_read_and_write_as_unavailable {
                    self.global_endpoint_manager
                        .mark_endpoint_unavailable_for_write(endpoint);
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

        // Ideally, any request flow should not be blocked by the outcome of refresh_location.
        // There can be three possible cases:
        // a) The refresh_location succeeds when TTL expires.
        // b) The refresh_location is bypassed when TTL hasn't expired.
        // c) The refresh_location operation has failed. In the event of a failure,
        //    the error is logged and the request should not be blocked.
        // Hence, the outcome of the operation is ignored here.
        _ = self
            .global_endpoint_manager
            .refresh_location(force_refresh)
            .await;
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

    /// Determines if a request should be retried for service unavailable status codes.
    ///
    /// # Summary
    /// Handles 503 (ServiceUnavailable), 500 (InternalServerError for reads), and 410 with
    /// LeaseNotFound errors by attempting retry on all applicable endpoints (all regions minus
    /// excluded regions, in preference of preferred regions). Requires multi-write support for
    /// write operations. Configures retry context to route to the next preferred location.
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration::ZERO }` if retry conditions are met
    /// - `DoNotRetry` if all endpoints tried or write without multi-write support
    fn should_retry_on_unavailable_endpoint_status_codes(&mut self) -> RetryResult {
        self.service_unavailable_retry_count += 1;

        if !self.can_use_multiple_write_locations
            && !self
                .operation_type
                .as_ref()
                .is_some_and(|op| op.is_read_only())
        {
            return RetryResult::DoNotRetry;
        }

        // automatic failover support needed to be plugged in.
        if !self.can_use_multiple_write_locations
            && !self.is_read_only()
            && !self
                .partition_key_range_location_cache
                .partition_level_automatic_failover_enabled()
        {
            return RetryResult::DoNotRetry;
        }

        let endpoints = self
            .global_endpoint_manager
            .applicable_endpoints(self.operation_type.unwrap(), self.excluded_regions.as_ref());

        if self.service_unavailable_retry_count > endpoints.len() {
            return RetryResult::DoNotRetry;
        }

        self.retry_context = Some(RetryContext {
            retry_location_index: self.service_unavailable_retry_count,
            retry_request_on_preferred_locations: true,
            route_to_hub: false,
        });

        RetryResult::Retry {
            after: Duration::ZERO,
        }
    }

    /// Routes HTTP status codes to appropriate retry handling logic.
    ///
    /// # Summary
    /// Evaluates HTTP status code and Cosmos DB sub-status code combinations to determine
    /// the appropriate retry strategy. Handles specific scenarios: 403.3 (WriteForbidden)
    /// triggers endpoint failover with cache refresh, 404.1022 (READ_SESSION_NOT_AVAILABLE)
    /// retries on different endpoints, 503 (ServiceUnavailable) attempts preferred location
    /// failover, and 500/408/410 with LeaseNotFound retry on alternative endpoints for reads.
    ///
    /// For read operations, any status code that is not considered non-retryable by
    /// [`is_non_retryable_status_code`] is retried on an alternative endpoint. For write
    /// operations, unhandled status codes are delegated to the throttling policy.
    ///
    /// # Arguments
    /// * `status_code` - The HTTP status code from the response
    /// * `sub_status_code` - The Cosmos DB specific sub-status code
    ///
    /// # Returns
    /// An `Option<RetryResult>`:
    /// - `Some(RetryResult)` if the status code requires special retry handling
    /// - `None` if the status code should be delegated to the throttling policy
    async fn should_retry_on_http_status(
        &mut self,
        status_code: StatusCode,
        sub_status_code: Option<SubStatusCode>,
    ) -> Option<RetryResult> {
        // Forbidden - Write forbidden (403.3)
        if status_code == StatusCode::Forbidden
            && sub_status_code == Some(SubStatusCode::WRITE_FORBIDDEN)
        {
            if self
                .partition_key_range_location_cache
                .partition_level_failover_enabled()
                && self.request.is_some()
                && self
                    .partition_key_range_location_cache
                    .try_mark_endpoint_unavailable_for_partition_key_range(
                        self.request.as_ref().unwrap(),
                    )
            {
                return Some(RetryResult::Retry {
                    after: Duration::ZERO,
                });
            }

            // automatic failover support needed to be plugged in here.
            return Some(
                self.should_retry_on_endpoint_failure(false, false, true, false, false)
                    .await,
            );
        }

        // Read Session Not Available (404.1022)
        if status_code == StatusCode::NotFound
            && sub_status_code == Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
        {
            return Some(self.should_retry_on_session_not_available());
        }

        if self.should_mark_endpoint_unavailable_on_system_resource_unavailable_for_write(
            Some(status_code),
            sub_status_code,
        ) {
            error!(
                "Operation will NOT be retried on local region. \
                     Treating SystemResourceUnavailable (429/3092) as ServiceUnavailable (503). \
                     Status code: 429, sub status code: 3092"
            );

            return Some(
                self.try_mark_endpoint_unavailable_for_pk_range_and_retry_on_service_unavailable(
                    true,
                ),
            );
        }

        // Service unavailable (503)
        if status_code == StatusCode::ServiceUnavailable {
            return Some(
                self.try_mark_endpoint_unavailable_for_pk_range_and_retry_on_service_unavailable(
                    false,
                ),
            );
        }

        // Gone - Lease not found (410.1022) applies to both reads and writes
        if status_code == StatusCode::Gone
            && sub_status_code == Some(SubStatusCode::LEASE_NOT_FOUND)
        {
            return Some(self.should_retry_on_unavailable_endpoint_status_codes());
        }

        // For read operations, retry on any status code that is not explicitly non-retryable.
        // This ensures transient server errors are retried on alternative endpoints.
        if self.is_read_only() && !is_non_retryable_status_code(status_code, sub_status_code) {
            return Some(self.should_retry_on_unavailable_endpoint_status_codes());
        }

        None
    }

    /// Marks endpoint unavailable for partition key range and retries on service unavailable.
    fn try_mark_endpoint_unavailable_for_pk_range_and_retry_on_service_unavailable(
        &mut self,
        is_system_resource_unavailable_for_write: bool,
    ) -> RetryResult {
        self.try_mark_endpoint_unavailable_for_pk_range(is_system_resource_unavailable_for_write);
        self.should_retry_on_unavailable_endpoint_status_codes()
    }

    /// Attempts to mark the endpoint unavailable for the partition key range.
    fn try_mark_endpoint_unavailable_for_pk_range(
        &self,
        is_system_resource_unavailable_for_write: bool,
    ) -> bool {
        if let Some(request) = self.request.as_ref() {
            if is_system_resource_unavailable_for_write
                || self.is_request_eligible_for_per_partition_automatic_failover()
                || self.is_request_eligible_for_partition_level_circuit_breaker()
            {
                return self
                    .partition_key_range_location_cache
                    .try_mark_endpoint_unavailable_for_partition_key_range(request);
            }
        }
        false
    }

    /// Checks if endpoint should be marked unavailable on system resource unavailable for write.
    fn should_mark_endpoint_unavailable_on_system_resource_unavailable_for_write(
        &self,
        status_code: Option<StatusCode>,
        sub_status_code: Option<SubStatusCode>,
    ) -> bool {
        self.can_use_multiple_write_locations
            && status_code == Some(StatusCode::TooManyRequests)
            && sub_status_code == Some(SubStatusCode::SYSTEM_RESOURCE_NOT_AVAILABLE)
    }

    /// Checks if request is eligible for per-partition automatic failover.
    fn is_request_eligible_for_per_partition_automatic_failover(&self) -> bool {
        if let Some(request) = self.request.as_ref() {
            return self
                .partition_key_range_location_cache
                .is_request_eligible_for_per_partition_automatic_failover(request);
        }
        false
    }

    /// Checks if request is eligible for partition-level circuit breaker.
    fn is_request_eligible_for_partition_level_circuit_breaker(&self) -> bool {
        if let Some(request) = self.request.as_ref() {
            return self
                .partition_key_range_location_cache
                .is_request_eligible_for_partition_level_circuit_breaker(request)
                && self
                    .partition_key_range_location_cache
                    .increment_request_failure_counter_and_check_if_partition_can_failover(
                        request,
                    );
        }
        false
    }

    /// Evaluates an error to determine if the request should be retried.
    ///
    /// # Summary
    /// First checks the [`RequestSentStatus`] to handle transport-level errors:
    /// - `NotSent`: retries reads and writes (request never reached server).
    /// - `Sent`/`Unknown` with transport errors (`Timeout`, `Io`): retries reads only.
    ///
    /// For HTTP-level errors, delegates to `should_retry_on_http_status` for
    /// scenario-specific retry logic (403.3, 404.1022, 503, 500, 410), then falls
    /// back to the throttling retry policy for 429 (TooManyRequests).
    ///
    /// # Arguments
    /// * `err` - The error that occurred during the request
    ///
    /// # Returns
    /// A `RetryResult` indicating whether to retry and with what delay
    async fn should_retry_error(&mut self, err: &azure_core::Error) -> RetryResult {
        // Determine whether the request was actually sent to the server.
        // This drives the retry decision for transport-level errors:
        // - NotSent: safe to retry reads and writes (request never reached server)
        // - Sent/Unknown: only retry reads (write may have been applied)
        match err.request_sent_status() {
            RequestSentStatus::NotSent => {
                return self.should_retry_on_connection_failure().await;
            }
            RequestSentStatus::Sent | RequestSentStatus::Unknown => {
                if matches!(err.kind(), ErrorKind::Io) {
                    if self.is_read_only() {
                        return self.should_retry_on_unavailable_endpoint_status_codes();
                    }
                    return RetryResult::DoNotRetry;
                }
            }
        }

        let status_code = err.http_status().unwrap_or(StatusCode::UnknownValue(0));
        let sub_status_code = get_substatus_code_from_error(err);

        if let Some(result) = self
            .should_retry_on_http_status(status_code, sub_status_code)
            .await
        {
            return result;
        }

        self.throttling_retry.should_retry_error(err)
    }

    /// Evaluates an HTTP response to determine if the request should be retried.
    ///
    /// # Summary
    /// Extracts HTTP status code and sub-status code from the response and delegates to
    /// `should_retry_on_http_status` for scenario-specific retry logic. If the response
    /// doesn't match any special retry cases (403.3, 404.1022, 503, 500, 410), falls
    /// back to the throttling retry policy which handles 429 (TooManyRequests) responses
    /// with exponential backoff.
    ///
    /// # Arguments
    /// * `response` - The HTTP response received from the service
    ///
    /// # Returns
    /// A `RetryResult` indicating whether to retry and with what delay
    async fn should_retry_response(&mut self, response: &RawResponse) -> RetryResult {
        let status_code = response.status();
        let sub_status_code = get_substatus_code_from_response(response);

        if let Some(result) = self
            .should_retry_on_http_status(status_code, sub_status_code)
            .await
        {
            return result;
        }

        self.throttling_retry.should_retry_response(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::partition_key::PartitionKey;
    use crate::regions;
    use crate::regions::RegionName;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use azure_core::http::headers::Headers;
    use azure_core::http::ClientOptions;
    use azure_core::Bytes;
    use std::sync::Arc;

    fn create_test_endpoint_manager() -> Arc<GlobalEndpointManager> {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        Arc::new(GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![RegionName::from("West US"), RegionName::from("East US")],
            vec![],
            pipeline,
        ))
    }

    fn create_test_endpoint_manager_no_locations() -> Arc<GlobalEndpointManager> {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        Arc::new(GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![],
            vec![],
            pipeline,
        ))
    }

    fn create_test_endpoint_manager_with_preferred_locations() -> Arc<GlobalEndpointManager> {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );

        Arc::new(GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![
                regions::EAST_ASIA,
                regions::WEST_US,
                regions::NORTH_CENTRAL_US,
            ],
            vec![],
            pipeline,
        ))
    }

    fn create_test_policy() -> ClientRetryPolicy {
        let manager = create_test_endpoint_manager();
        let partition_manager = GlobalPartitionEndpointManager::new(manager.clone(), false, false);
        ClientRetryPolicy::new(manager, partition_manager, None)
    }

    fn create_test_policy_no_locations() -> ClientRetryPolicy {
        let manager = create_test_endpoint_manager_no_locations();
        let partition_manager = GlobalPartitionEndpointManager::new(manager.clone(), false, false);
        ClientRetryPolicy::new(manager, partition_manager, None)
    }

    fn create_test_policy_with_preferred_locations() -> ClientRetryPolicy {
        let manager = create_test_endpoint_manager_with_preferred_locations();
        let partition_manager = GlobalPartitionEndpointManager::new(manager.clone(), false, false);
        ClientRetryPolicy::new(manager, partition_manager, None)
    }

    fn create_test_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        CosmosRequest::builder(OperationType::Read, resource_link.clone())
            .partition_key(PartitionKey::from("test"))
            .build()
            .unwrap()
    }

    fn create_write_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        CosmosRequest::builder(OperationType::Create, resource_link.clone())
            .partition_key(PartitionKey::from("test"))
            .build()
            .unwrap()
    }

    fn create_raw_response(status_code: StatusCode) -> RawResponse {
        let headers = Headers::new();
        RawResponse::from_bytes(status_code, headers, Bytes::new())
    }

    fn create_raw_response_with_substatus(status_code: StatusCode, substatus: u32) -> RawResponse {
        let mut headers = Headers::new();
        headers.insert("x-ms-substatus", substatus.to_string());
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

    fn create_error_with_substatus(status: StatusCode, substatus: u32) -> azure_core::Error {
        let response = create_raw_response_with_substatus(status, substatus);
        azure_core::Error::new(
            azure_core::error::ErrorKind::HttpResponse {
                status: response.status(),
                error_code: None,
                raw_response: Some(Box::new(response)),
            },
            "Test error with substatus",
        )
    }

    #[test]
    fn test_new_policy_initialization() {
        let policy = create_test_policy();
        assert!(policy.enable_endpoint_discovery);
        assert_eq!(policy.failover_retry_count, 0);
        assert_eq!(policy.session_token_retry_count, 0);
        assert_eq!(policy.service_unavailable_retry_count, 0);
        assert!(!policy.can_use_multiple_write_locations);
        assert!(policy.location_endpoint.is_none());
        assert!(policy.retry_context.is_none());
        assert!(policy.operation_type.is_none());
    }

    #[test]
    fn test_retry_context_none_initially() {
        let policy = create_test_policy();
        assert!(policy.retry_context.is_none());
    }

    #[tokio::test]
    async fn test_should_retry_service_unavailable_with_preferred_locations() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
                assert!(policy.retry_context.is_some());
            }
            _ => panic!("Expected retry for ServiceUnavailable with preferred locations"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_service_unavailable_without_preferred_locations() {
        // Even with no preferred locations, applicable_endpoints returns the default endpoint
        let mut policy = create_test_policy_no_locations();
        policy.operation_type = Some(OperationType::Read);
        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
            }
            _ => panic!("Expected retry for ServiceUnavailable (default endpoint available)"),
        }

        // Second attempt should stop — only the default endpoint was available
        let result = policy.should_retry_error(&error).await;
        assert!(
            !result.is_retry(),
            "Expected DoNotRetry after exhausting the single default endpoint"
        );
    }

    #[tokio::test]
    async fn test_should_retry_internal_server_error_for_read() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let error = create_error_with_status(StatusCode::InternalServerError);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
            }
            _ => panic!("Expected retry for InternalServerError on read request"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_internal_server_error_for_write() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Create);
        let error = create_error_with_status(StatusCode::InternalServerError);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::DoNotRetry => {}
            _ => panic!("Expected DoNotRetry for InternalServerError on write request"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_gone_with_lease_not_found() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let error = create_error_with_substatus(
            StatusCode::Gone,
            SubStatusCode::LEASE_NOT_FOUND.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
            }
            _ => panic!("Expected retry for Gone with LeaseNotFound"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_gone_with_lease_not_found_for_write() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Create);
        policy.can_use_multiple_write_locations = true;
        let error = create_error_with_substatus(
            StatusCode::Gone,
            SubStatusCode::LEASE_NOT_FOUND.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
            }
            _ => panic!("Expected retry for Gone with LeaseNotFound on write"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_write_forbidden() {
        let mut policy = create_test_policy();
        policy.operation_type = Some(OperationType::Create);
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());
        let error = create_error_with_substatus(
            StatusCode::Forbidden,
            SubStatusCode::WRITE_FORBIDDEN.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after: _ } => {
                assert_eq!(policy.failover_retry_count, 1);
            }
            _ => panic!("Expected retry for WriteForbidden"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_session_not_available_single_write() {
        let mut policy = create_test_policy();
        policy.enable_endpoint_discovery = true;
        policy.can_use_multiple_write_locations = false;

        let error = create_error_with_substatus(
            StatusCode::NotFound,
            SubStatusCode::READ_SESSION_NOT_AVAILABLE.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.session_token_retry_count, 1);
                assert!(policy.retry_context.is_some());
            }
            _ => panic!("Expected retry for READ_SESSION_NOT_AVAILABLE"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_session_not_available_when_discovery_disabled() {
        let mut policy = create_test_policy();
        policy.enable_endpoint_discovery = false;

        let error = create_error_with_substatus(
            StatusCode::NotFound,
            SubStatusCode::READ_SESSION_NOT_AVAILABLE.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::DoNotRetry => {
                assert_eq!(policy.session_token_retry_count, 1);
            }
            _ => panic!("Expected DoNotRetry when endpoint discovery disabled"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_session_not_available_after_all_endpoints_tried() {
        let mut policy = create_test_policy();
        policy.enable_endpoint_discovery = true;
        policy.can_use_multiple_write_locations = false;
        policy.operation_type = Some(OperationType::Read);
        // create_test_policy has 2 preferred locations, so set count to 2
        // to simulate all endpoints already tried
        policy.session_token_retry_count = 2;

        let error = create_error_with_substatus(
            StatusCode::NotFound,
            SubStatusCode::READ_SESSION_NOT_AVAILABLE.value() as u32,
        );

        let result = policy.should_retry_error(&error).await;
        match result {
            RetryResult::DoNotRetry => {
                assert_eq!(policy.session_token_retry_count, 3);
            }
            _ => panic!("Expected DoNotRetry after all endpoints tried"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_service_unavailable_after_all_endpoints_tried() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        // applicable_endpoints returns 1 (default endpoint) in test setup,
        // so set count to 1 to simulate exhaustion
        policy.service_unavailable_retry_count = 1;

        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::DoNotRetry => {
                assert_eq!(policy.service_unavailable_retry_count, 2);
            }
            _ => panic!("Expected DoNotRetry after all endpoints tried"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_service_unavailable_for_write_without_multi_write() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Create);
        policy.can_use_multiple_write_locations = false;

        let error = create_error_with_status(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_error(&error).await;

        match result {
            RetryResult::DoNotRetry => {}
            _ => panic!("Expected DoNotRetry for write without multi-write support"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_too_many_requests() {
        let mut policy = create_test_policy();
        let error = create_error_with_status(StatusCode::TooManyRequests);

        let result = policy.should_retry_error(&error).await;

        // TooManyRequests should be delegated to throttling policy
        match result {
            RetryResult::Retry { after: _ } => {}
            _ => panic!("Expected retry for TooManyRequests (throttling)"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_response_service_unavailable() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let response = create_raw_response(StatusCode::ServiceUnavailable);

        let result = policy.should_retry_response(&response).await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.service_unavailable_retry_count, 1);
            }
            _ => panic!("Expected retry for ServiceUnavailable response"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_response_too_many_requests() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::TooManyRequests);

        let result = policy.should_retry_response(&response).await;

        // Should be delegated to throttling policy
        match result {
            RetryResult::Retry { after: _ } => {}
            _ => panic!("Expected retry for TooManyRequests response"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_for_error_response() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let response = create_raw_response(StatusCode::ServiceUnavailable);
        let result_with_response: azure_core::Result<RawResponse> = Ok(response);

        let retry_result = policy.should_retry(&result_with_response).await;

        match retry_result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
            }
            _ => panic!("Expected retry for error response"),
        }
    }

    #[tokio::test]
    async fn test_should_not_retry_for_success_response() {
        let mut policy = create_test_policy();
        let response = create_raw_response(StatusCode::Ok);
        let result_with_response: azure_core::Result<RawResponse> = Ok(response);

        let retry_result = policy.should_retry(&result_with_response).await;

        match retry_result {
            RetryResult::DoNotRetry => {}
            _ => panic!("Expected DoNotRetry for success response"),
        }
    }

    #[tokio::test]
    async fn test_should_retry_for_transport_error() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);
        let error = create_error_with_status(StatusCode::ServiceUnavailable);
        let result_with_error: azure_core::Result<RawResponse> = Err(error);

        let retry_result = policy.should_retry(&result_with_error).await;

        match retry_result {
            RetryResult::Retry { after: _ } => {}
            _ => panic!("Expected retry for transport error"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_increments_count() {
        let mut policy = create_test_policy();
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());

        let result = policy
            .should_retry_on_endpoint_failure(true, false, false, false, false)
            .await;

        match result {
            RetryResult::Retry { after: _ } => {
                assert_eq!(policy.failover_retry_count, 1);
                assert!(policy.retry_context.is_some());
            }
            _ => panic!("Expected retry for endpoint failure"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_respects_max_retry_count() {
        let mut policy = create_test_policy();
        policy.failover_retry_count = MAX_RETRY_COUNT_ON_ENDPOINT_FAILURE + 1;

        let result = policy
            .should_retry_on_endpoint_failure(true, false, false, false, false)
            .await;

        match result {
            RetryResult::DoNotRetry => {}
            _ => panic!("Expected DoNotRetry after max failover retries"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_respects_endpoint_discovery_disabled() {
        let mut policy = create_test_policy();
        policy.enable_endpoint_discovery = false;

        let result = policy
            .should_retry_on_endpoint_failure(true, false, false, false, false)
            .await;

        match result {
            RetryResult::DoNotRetry => {}
            _ => panic!("Expected DoNotRetry when endpoint discovery disabled"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_with_overwrite_discovery() {
        let mut policy = create_test_policy();
        policy.enable_endpoint_discovery = false;
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());

        let result = policy
            .should_retry_on_endpoint_failure(true, false, false, false, true)
            .await;

        match result {
            RetryResult::Retry { after: _ } => {
                assert_eq!(policy.failover_retry_count, 1);
            }
            _ => panic!("Expected retry when overwrite_endpoint_discovery is true"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_write_delay() {
        let mut policy = create_test_policy();
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());
        policy.failover_retry_count = 1;

        let result = policy
            .should_retry_on_endpoint_failure(false, false, false, false, false)
            .await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::milliseconds(RETRY_INTERVAL_MS));
                assert_eq!(policy.failover_retry_count, 2);
            }
            _ => panic!("Expected retry with delay for write request"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_first_write_no_delay() {
        let mut policy = create_test_policy();
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());

        let result = policy
            .should_retry_on_endpoint_failure(false, false, false, false, false)
            .await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::ZERO);
                assert_eq!(policy.failover_retry_count, 1);
            }
            _ => panic!("Expected retry with zero delay for first write failover"),
        }
    }

    #[tokio::test]
    async fn test_endpoint_failover_read_always_has_delay() {
        let mut policy = create_test_policy();
        policy.location_endpoint = Some("https://test.documents.azure.com".parse().unwrap());

        let result = policy
            .should_retry_on_endpoint_failure(true, false, false, false, false)
            .await;

        match result {
            RetryResult::Retry { after } => {
                assert_eq!(after, Duration::milliseconds(RETRY_INTERVAL_MS));
            }
            _ => panic!("Expected retry with delay for read request"),
        }
    }

    #[tokio::test]
    async fn test_before_send_request_sets_read_flag() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();

        policy.before_send_request(&mut request).await;

        assert!(policy.operation_type.is_some());
        assert!(policy.operation_type.unwrap().is_read_only());
    }

    #[tokio::test]
    async fn test_before_send_request_sets_write_flag() {
        let mut policy = create_test_policy();
        let mut request = create_write_request();

        policy.before_send_request(&mut request).await;

        assert!(policy.operation_type.is_some());
        assert!(!policy.operation_type.unwrap().is_read_only());
    }

    #[tokio::test]
    async fn test_retry_context_applied_to_request() {
        let mut policy = create_test_policy();
        policy.retry_context = Some(RetryContext {
            retry_location_index: 1,
            retry_request_on_preferred_locations: true,
            route_to_hub: false,
        });
        let mut request = create_test_request();

        policy.before_send_request(&mut request).await;

        // The retry context should be applied to the request
        assert!(policy.location_endpoint.is_some());
    }

    #[test]
    fn test_retry_context_creation() {
        let ctx = RetryContext {
            retry_location_index: 2,
            retry_request_on_preferred_locations: true,
            route_to_hub: false,
        };

        assert_eq!(ctx.retry_location_index, 2);
        assert!(ctx.retry_request_on_preferred_locations);
        assert!(!ctx.route_to_hub);
    }

    #[test]
    fn test_constants_values() {
        assert_eq!(RETRY_INTERVAL_MS, 1000);
        assert_eq!(MAX_RETRY_COUNT_ON_ENDPOINT_FAILURE, 120);
    }

    #[tokio::test]
    async fn read_retries_on_unknown_server_error() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);

        // A non-specific server error (e.g., 502 BadGateway) should be retried for reads
        let error = create_error_with_status(StatusCode::BadGateway);
        let result = policy.should_retry_error(&error).await;

        assert!(
            result.is_retry(),
            "Expected retry for BadGateway on read request"
        );
    }

    #[tokio::test]
    async fn read_does_not_retry_non_retryable_status_codes() {
        for status in [
            StatusCode::BadRequest,
            StatusCode::Unauthorized,
            StatusCode::NotFound,
            StatusCode::MethodNotAllowed,
            StatusCode::Conflict,
            StatusCode::PreconditionFailed,
            StatusCode::PayloadTooLarge,
            StatusCode::Locked,
            constants::RETRY_WITH,
        ] {
            let mut policy = create_test_policy_with_preferred_locations();
            policy.operation_type = Some(OperationType::Read);

            let error = create_error_with_status(status);
            let result = policy.should_retry_error(&error).await;

            assert!(
                !result.is_retry(),
                "Expected DoNotRetry for {status:?} on read request"
            );
        }
    }

    #[tokio::test]
    async fn write_does_not_retry_unknown_server_error() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Create);

        // A non-specific server error should NOT be retried for writes
        let error = create_error_with_status(StatusCode::BadGateway);
        let result = policy.should_retry_error(&error).await;

        assert!(
            !result.is_retry(),
            "Expected DoNotRetry for BadGateway on write request"
        );
    }

    #[tokio::test]
    async fn read_retries_on_forbidden_without_write_forbidden_substatus() {
        let mut policy = create_test_policy_with_preferred_locations();
        policy.operation_type = Some(OperationType::Read);

        // Forbidden without WRITE_FORBIDDEN substatus should be retried for reads
        let error = create_error_with_status(StatusCode::Forbidden);
        let result = policy.should_retry_error(&error).await;

        assert!(
            result.is_retry(),
            "Expected retry for Forbidden (no substatus) on read request"
        );
    }

    fn create_timeout_error(message: &str) -> azure_core::Error {
        azure_core::Error::with_message(azure_core::error::ErrorKind::Io, message.to_string())
    }

    fn create_io_error(message: &str) -> azure_core::Error {
        azure_core::Error::with_message(azure_core::error::ErrorKind::Io, message.to_string())
    }

    #[tokio::test]
    async fn response_timeout_retries_read() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();
        policy.before_send_request(&mut request).await;

        let err = create_timeout_error("response timeout");
        let result = policy.should_retry(&Err(err)).await;
        assert!(
            result.is_retry(),
            "response timeout should retry read requests"
        );
    }

    #[tokio::test]
    async fn response_timeout_does_not_retry_write() {
        let mut policy = create_test_policy();
        let mut request = create_write_request();
        policy.before_send_request(&mut request).await;

        let err = create_timeout_error("response timeout");
        let result = policy.should_retry(&Err(err)).await;
        assert_eq!(
            result,
            RetryResult::DoNotRetry,
            "response timeout should NOT retry write requests"
        );
    }

    #[tokio::test]
    async fn response_timeout_read_uses_service_unavailable_counter() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();
        policy.before_send_request(&mut request).await;

        let err = create_timeout_error("response timeout");
        let result = policy.should_retry(&Err(err)).await;
        assert!(result.is_retry());
        assert_eq!(
            policy.service_unavailable_retry_count, 1,
            "service_unavailable_retry_count should increment on response timeout for reads"
        );
    }

    #[tokio::test]
    async fn unknown_io_error_retries_read() {
        let mut policy = create_test_policy();
        let mut request = create_test_request();
        policy.before_send_request(&mut request).await;

        let err = create_io_error("some unrelated IO error");
        let result = policy.should_retry(&Err(err)).await;
        assert!(
            result.is_retry(),
            "unknown IO errors should retry read requests"
        );
    }

    #[tokio::test]
    async fn unknown_io_error_does_not_retry_write() {
        let mut policy = create_test_policy();
        let mut request = create_write_request();
        policy.before_send_request(&mut request).await;

        let err = create_io_error("some unrelated IO error");
        let result = policy.should_retry(&Err(err)).await;
        assert_eq!(
            result,
            RetryResult::DoNotRetry,
            "unknown IO errors should not retry write requests"
        );
    }
}
