// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-region hedging availability strategy for Cosmos DB requests.
//!
//! When a request takes longer than a configurable threshold, the SDK sends
//! a parallel (hedged) request to the next preferred region. If the hedged
//! request also exceeds a step interval, additional hedged requests are sent
//! to subsequent regions until a non-transient response is received or all
//! regions are exhausted.

use crate::cosmos_request::CosmosRequest;
use crate::resource_context::ResourceType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use azure_core::http::{RawResponse, StatusCode};
use std::sync::Arc;
use std::time::Duration;

/// Availability strategy that controls how the SDK hedges requests across regions
/// during periods of high latency or regional unavailability.
///
/// By default, no availability strategy is set, meaning requests are sent to a single
/// region and retried according to the retry policy. When a cross-region hedging strategy
/// is configured, the SDK sends parallel requests to additional regions after the initial
/// request exceeds a latency threshold, returning the first successful response.
///
/// # Examples
///
/// ```rust
/// use azure_data_cosmos::AvailabilityStrategy;
/// use std::time::Duration;
///
/// // Hedge after 500ms, then every 100ms to subsequent regions
/// let strategy = AvailabilityStrategy::cross_region_hedging(
///     Duration::from_millis(500),
///     Some(Duration::from_millis(100)),
/// );
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AvailabilityStrategy {
    /// Cross-region hedging strategy that sends parallel requests to additional
    /// regions when the primary request exceeds a latency threshold.
    CrossRegionHedging(CrossRegionHedgingStrategy),

    /// Disables any client-level availability strategy for specific requests.
    /// Use this to opt out of hedging on a per-request basis when a client-level
    /// strategy is configured.
    Disabled,
}

impl AvailabilityStrategy {
    /// Creates a cross-region hedging strategy.
    ///
    /// After a request's duration exceeds `threshold`, the SDK sends a hedged request
    /// to the next preferred region. If `threshold_step` is provided and positive, subsequent
    /// hedged requests are sent at that interval until a response is received or all
    /// regions are exhausted.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Duration after which the first hedged request is sent.
    ///   Must be greater than zero.
    /// * `threshold_step` - Optional interval between subsequent hedged requests after
    ///   the first hedge. If `None`, only one hedge is sent.
    ///
    /// # Panics
    ///
    /// Panics if `threshold` is zero.
    pub fn cross_region_hedging(
        threshold: Duration,
        threshold_step: Option<Duration>,
    ) -> AvailabilityStrategy {
        AvailabilityStrategy::CrossRegionHedging(CrossRegionHedgingStrategy::new(
            threshold,
            threshold_step,
            false,
        ))
    }

    /// Creates a cross-region hedging strategy with multi-region write hedging enabled.
    ///
    /// In addition to hedging read requests, this variant also hedges write requests
    /// on accounts with multi-region writes enabled. Note that this may cause
    /// additional 409 (Conflict) or 412 (Precondition Failed) errors, which
    /// applications must be prepared to handle.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Duration after which the first hedged request is sent.
    ///   Must be greater than zero.
    /// * `threshold_step` - Optional interval between subsequent hedged requests.
    ///
    /// # Panics
    ///
    /// Panics if `threshold` is zero.
    pub fn cross_region_hedging_with_writes(
        threshold: Duration,
        threshold_step: Option<Duration>,
    ) -> AvailabilityStrategy {
        AvailabilityStrategy::CrossRegionHedging(CrossRegionHedgingStrategy::new(
            threshold,
            threshold_step,
            true,
        ))
    }
}

/// Configuration for cross-region hedging.
///
/// This struct holds the parameters that control when and how the SDK sends
/// hedged (parallel) requests to additional regions.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CrossRegionHedgingStrategy {
    /// Duration after which the first hedged request is sent.
    pub threshold: Duration,

    /// Interval between subsequent hedged requests after the first hedge.
    /// If `None`, only one hedge is sent.
    pub threshold_step: Option<Duration>,

    /// Whether to hedge write requests on multi-region write accounts.
    pub enable_multi_write_region_hedge: bool,
}

impl CrossRegionHedgingStrategy {
    /// Creates a new `CrossRegionHedgingStrategy`.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Duration after which the first hedged request is sent.
    /// * `threshold_step` - Optional interval between subsequent hedged requests.
    /// * `enable_multi_write_region_hedge` - Whether to hedge writes on multi-write accounts.
    ///
    /// # Panics
    ///
    /// Panics if `threshold` is zero.
    fn new(
        threshold: Duration,
        threshold_step: Option<Duration>,
        enable_multi_write_region_hedge: bool,
    ) -> Self {
        assert!(
            !threshold.is_zero(),
            "hedging threshold must be greater than zero"
        );
        Self {
            threshold,
            threshold_step,
            enable_multi_write_region_hedge,
        }
    }

    /// Determines if the given request should be hedged.
    ///
    /// Hedging is only applicable for document point operations:
    /// - Read operations are always eligible.
    /// - Write operations are eligible only when multi-region write hedging
    ///   is enabled AND the account supports multi-region writes.
    pub(crate) fn should_hedge(
        &self,
        request: &CosmosRequest,
        global_endpoint_manager: &GlobalEndpointManager,
    ) -> bool {
        // Only hedge document point operations
        if request.resource_type != ResourceType::Documents {
            return false;
        }

        // Reads are always eligible for hedging
        if request.operation_type.is_read_only() {
            return true;
        }

        // Writes only when multi-write hedging is enabled and the account supports it
        if self.enable_multi_write_region_hedge {
            return global_endpoint_manager.can_support_multiple_write_locations(
                request.resource_type,
                request.operation_type,
            );
        }

        false
    }
}

/// Determines whether a response status code indicates a final (non-transient) result.
///
/// Final results include:
/// - All 1xx, 2xx, and 3xx status codes
/// - Client errors that are not transient: 400, 405, 409, 412, 413, 401
/// - 404 with no sub-status code (document not found after consistency enforcement)
pub(crate) fn is_final_result(status_code: StatusCode) -> bool {
    // All informational, success, and redirect codes are final
    if status_code.is_informational() || status_code.is_success() || status_code.is_redirection() {
        return true;
    }

    // Non-transient client errors
    matches!(
        status_code,
        StatusCode::BadRequest
            | StatusCode::MethodNotAllowed
            | StatusCode::Conflict
            | StatusCode::PreconditionFailed
            | StatusCode::PayloadTooLarge
            | StatusCode::Unauthorized
            | StatusCode::NotFound
    )
}

/// Internal result from a hedged request, carrying the response and metadata
/// about whether it is a final (non-transient) result.
pub(crate) struct HedgingResponse {
    /// Whether this response is non-transient and should be returned immediately.
    pub is_non_transient: bool,
    /// The raw HTTP response.
    pub response: azure_core::Result<RawResponse>,
    /// The index of the region that produced this response.
    pub region_index: usize,
}

/// Executes the cross-region hedging strategy.
///
/// Sends the request to the primary region first. If no response arrives within
/// the threshold, sends parallel requests to subsequent regions. Returns the first
/// non-transient response, cancelling remaining requests.
///
/// # Arguments
///
/// * `strategy` - The hedging configuration (threshold, step, etc.)
/// * `request` - The original request to hedge
/// * `global_endpoint_manager` - Endpoint manager for resolving regional endpoints
/// * `sender` - Async closure that sends a request and returns the response
pub(crate) async fn execute_hedging<Sender, Fut>(
    strategy: &CrossRegionHedgingStrategy,
    request: &CosmosRequest,
    global_endpoint_manager: &Arc<GlobalEndpointManager>,
    sender: Sender,
) -> azure_core::Result<RawResponse>
where
    Sender: Fn(CosmosRequest) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send + 'static,
{
    let is_read = request.operation_type.is_read_only();

    // Get all applicable regions for this operation
    let hedge_regions = global_endpoint_manager
        .applicable_endpoints(request.operation_type, request.excluded_regions.as_ref());

    // If only one region available, no hedging possible
    if hedge_regions.len() <= 1 {
        return sender(request.clone()).await;
    }

    let runtime = azure_core::async_runtime::get_async_runtime();
    let total_regions = hedge_regions.len();

    // We'll use a channel to collect results from hedged requests
    let (tx, mut rx) = futures::channel::mpsc::unbounded::<HedgingResponse>();

    // Track spawned tasks for cancellation
    let mut spawned_tasks = Vec::new();

    for request_number in 0..total_regions {
        let await_time = if request_number == 0 {
            // First request: send immediately, then wait threshold before hedging
            Duration::ZERO
        } else if request_number == 1 {
            strategy.threshold
        } else {
            // Subsequent hedges use threshold_step, or skip if not configured
            match strategy.threshold_step {
                Some(step) if !step.is_zero() => step,
                _ => break, // No more hedging if threshold_step is not set
            }
        };

        // Wait the configured delay before sending this request
        if !await_time.is_zero() {
            let sleep_duration =
                azure_core::time::Duration::milliseconds(await_time.as_millis() as i64);
            runtime.sleep(sleep_duration).await;

            // Check if we already have a final response while we were waiting
            match rx.try_next() {
                Ok(Some(resp)) if resp.is_non_transient => {
                    // Cancel all outstanding tasks by dropping them
                    drop(spawned_tasks);
                    return resp.response;
                }
                _ => {}
            }
        }

        // Clone the request and configure it to target region at `request_number`
        let mut hedged_request = request.clone();

        if request_number > 0 {
            // For hedged requests, set excluded_regions to all regions EXCEPT the target
            let excluded: Vec<_> = hedge_regions
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != request_number)
                .filter_map(|(_, endpoint)| {
                    // Try to find the region name for this endpoint
                    find_region_name_for_endpoint(global_endpoint_manager, endpoint, is_read)
                })
                .collect();

            if !excluded.is_empty() {
                hedged_request.excluded_regions = Some(excluded);
            }
        }

        let tx_clone = tx.clone();
        let region_index = request_number;

        // Spawn the request as a background task
        let send_fut = sender(hedged_request);
        let task_future: azure_core::async_runtime::TaskFuture = Box::pin(async move {
            let result = send_fut.await;
            let is_non_transient = match &result {
                Ok(resp) => is_final_result(resp.status()),
                Err(_) => false,
            };
            let _ = tx_clone.unbounded_send(HedgingResponse {
                is_non_transient,
                response: result,
                region_index,
            });
        });
        spawned_tasks.push(runtime.spawn(task_future));
    }

    // Drop the sender side so rx completes when all tasks are done
    drop(tx);

    // Wait for results and return the first non-transient one
    let mut last_response: Option<HedgingResponse> = None;

    use futures::StreamExt;
    while let Some(hedging_resp) = rx.next().await {
        if hedging_resp.is_non_transient {
            // Cancel remaining tasks
            drop(spawned_tasks);

            tracing::debug!(
                target: "azure_data_cosmos::availability_strategy",
                region_index = hedging_resp.region_index,
                "hedging completed with non-transient response"
            );

            return hedging_resp.response;
        }
        last_response = Some(hedging_resp);
    }

    // All tasks completed without a non-transient response; return the last response
    match last_response {
        Some(resp) => resp.response,
        None => Err(azure_core::Error::new(
            azure_core::error::ErrorKind::Other,
            "cross-region hedging completed without producing a response",
        )),
    }
}

/// Attempts to find the region name for a given endpoint URL.
fn find_region_name_for_endpoint(
    global_endpoint_manager: &GlobalEndpointManager,
    endpoint: &url::Url,
    is_read: bool,
) -> Option<crate::regions::RegionName> {
    let endpoints_by_region = if is_read {
        global_endpoint_manager.available_read_endpoints_by_location()
    } else {
        global_endpoint_manager.available_write_endpoints_by_location()
    };

    endpoints_by_region
        .into_iter()
        .find(|(_, url)| url == endpoint)
        .map(|(name, _)| name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_region_hedging_creation() {
        let strategy = AvailabilityStrategy::cross_region_hedging(
            Duration::from_millis(500),
            Some(Duration::from_millis(100)),
        );
        match strategy {
            AvailabilityStrategy::CrossRegionHedging(s) => {
                assert_eq!(s.threshold, Duration::from_millis(500));
                assert_eq!(s.threshold_step, Some(Duration::from_millis(100)));
                assert!(!s.enable_multi_write_region_hedge);
            }
            _ => panic!("expected CrossRegionHedging variant"),
        }
    }

    #[test]
    fn cross_region_hedging_with_writes_creation() {
        let strategy = AvailabilityStrategy::cross_region_hedging_with_writes(
            Duration::from_millis(300),
            None,
        );

        match strategy {
            AvailabilityStrategy::CrossRegionHedging(s) => {
                assert_eq!(s.threshold, Duration::from_millis(300));
                assert!(s.threshold_step.is_none());
                assert!(s.enable_multi_write_region_hedge);
            }
            _ => panic!("expected CrossRegionHedging variant"),
        }
    }

    #[test]
    fn disabled_strategy() {
        let strategy = AvailabilityStrategy::Disabled;
        assert!(matches!(strategy, AvailabilityStrategy::Disabled));
    }

    #[test]
    #[should_panic(expected = "hedging threshold must be greater than zero")]
    fn zero_threshold_panics() {
        AvailabilityStrategy::cross_region_hedging(Duration::ZERO, None);
    }

    #[test]
    fn is_final_result_success_codes() {
        assert!(is_final_result(StatusCode::Ok));
        assert!(is_final_result(StatusCode::Created));
        assert!(is_final_result(StatusCode::NoContent));
        assert!(is_final_result(StatusCode::NotModified));
    }

    #[test]
    fn is_final_result_non_transient_errors() {
        assert!(is_final_result(StatusCode::BadRequest));
        assert!(is_final_result(StatusCode::Unauthorized));
        assert!(is_final_result(StatusCode::MethodNotAllowed));
        assert!(is_final_result(StatusCode::Conflict));
        assert!(is_final_result(StatusCode::PreconditionFailed));
        assert!(is_final_result(StatusCode::PayloadTooLarge));
    }

    #[test]
    fn is_final_result_not_found() {
        // 404 is treated as final (non-transient) for hedging
        assert!(is_final_result(StatusCode::NotFound));
    }

    #[test]
    fn is_final_result_transient_errors() {
        assert!(!is_final_result(StatusCode::TooManyRequests));
        assert!(!is_final_result(StatusCode::InternalServerError));
        assert!(!is_final_result(StatusCode::ServiceUnavailable));
    }
}
