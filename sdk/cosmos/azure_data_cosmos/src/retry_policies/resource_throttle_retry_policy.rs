// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use azure_core::http::{RawResponse, StatusCode};
use typespec_client_core::http::Request;
use super::{RetryPolicy, ShouldRetryResult};

/// Retry policy for handling resource throttling (429 TooManyRequests) errors
///
/// This policy implements exponential backoff for 429 status codes, respecting both
/// maximum retry attempts and cumulative wait time limits. It's designed to handle
/// Azure Cosmos DB throttling scenarios where the service limits request rates.
///
/// # Features
/// - Exponential backoff with configurable delay factor
/// - Maximum retry attempt limit
/// - Cumulative wait time tracking to prevent excessive delays
/// - Thread-safe retry counter using atomic operations
///
/// # Example
/// ```
/// use azure_data_cosmos::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
///
/// // Create a policy with 3 max retries, 100 second max wait, and backoff factor of 2
/// let policy = ResourceThrottleRetryPolicy::new(3, 100, 2);
/// ```
#[derive(Debug)]
pub struct ResourceThrottleRetryPolicy {
    max_attempt_count: usize,
    backoff_delay_factor: u32,
    max_wait_time: Duration,
    current_attempt_count: AtomicUsize,
    cumulative_retry_delay: Arc<AtomicUsize>,
}

impl ResourceThrottleRetryPolicy {
    /// Creates a new ResourceThrottleRetryPolicy with the specified configuration
    ///
    /// # Arguments
    /// * `max_attempt_count` - Maximum number of retry attempts before giving up
    /// * `max_wait_time_secs` - Maximum total wait time (in seconds) across all retries
    /// * `backoff_delay_factor` - Multiplier for exponential backoff delay
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
    ///
    /// // Create a policy with 5 retries, 120 second max wait, and backoff factor of 3
    /// let policy = ResourceThrottleRetryPolicy::new(5, 120, 3);
    /// ```
    pub fn new(
        max_attempt_count: usize,
        max_wait_time_secs: u64,
        backoff_delay_factor: u32,
    ) -> Self {
        Self {
            max_attempt_count,
            backoff_delay_factor,
            max_wait_time: Duration::from_secs(max_wait_time_secs),
            current_attempt_count: AtomicUsize::new(0),
            cumulative_retry_delay: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Checks if the given HTTP status code indicates resource throttling
    ///
    /// # Arguments
    /// * `status_code` - The HTTP status code to check
    ///
    /// # Returns
    /// `true` if the status code is 429 (TooManyRequests), `false` otherwise
    fn is_valid_throttle_status_code(&self, status_code: StatusCode) -> bool {
        status_code == StatusCode::TooManyRequests
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
    /// A tuple of (should_retry: bool, retry_delay: Duration)
    /// - `should_retry`: true if retry is allowed within timing constraints
    /// - `retry_delay`: The calculated delay duration (with backoff applied)
    fn check_if_retry_needed(
        &self,
        retry_after: Option<Duration>,
    ) -> (bool, Duration) {
        let mut retry_delay = retry_after.unwrap_or(Duration::from_secs(0));
        if self.backoff_delay_factor > 1 {
            retry_delay *= self.backoff_delay_factor;
        }
        let cumulative = self.cumulative_retry_delay.load(Ordering::Relaxed) as u64;
        let new_cumulative = cumulative + retry_delay.as_millis() as u64;
        if retry_delay < self.max_wait_time && new_cumulative <= self.max_wait_time.as_millis() as u64 {
            if retry_delay == Duration::ZERO {
                retry_delay = Duration::from_secs(5);
            }
            self.cumulative_retry_delay.store(new_cumulative as usize, Ordering::Relaxed);
            return (true, retry_delay);
        }
        (false, Duration::ZERO)
    }

    /// Common retry logic for both exceptions and responses with exponential backoff
    ///
    /// This method encapsulates the shared retry decision logic used by both
    /// `should_retry_exception` and `should_retry_response`. It provides a centralized
    /// place for retry decision making to ensure consistency across different failure modes.
    ///
    /// # Retry Logic
    /// 1. Checks if current attempt count is below `max_attempt_count`
    /// 2. Calculates exponential backoff delay using `backoff_delay_factor`
    /// 3. Verifies cumulative delay doesn't exceed `max_wait_time`
    /// 4. Increments attempt counter atomically if retry is approved
    ///
    /// # Arguments
    /// * `retry_after` - Optional duration to wait before retrying (from server header or default)
    ///
    /// # Returns
    /// `ShouldRetryResult` containing:
    /// - `should_retry`: true if retry is approved, false otherwise
    /// - `backoff_time`: The calculated delay duration (0 if not retrying)
    fn should_retry_with_backoff(&self, retry_after: Option<Duration>) -> ShouldRetryResult {
        let attempt = self.current_attempt_count.load(Ordering::Relaxed);
        if attempt < self.max_attempt_count {
            let (should_retry, delay) = self.check_if_retry_needed(retry_after);
            if should_retry {
                self.current_attempt_count.fetch_add(1, Ordering::Relaxed);
                return ShouldRetryResult::retry_after(delay);
            }
        }
        ShouldRetryResult::no_retry()
    }
}

#[async_trait]
impl RetryPolicy for ResourceThrottleRetryPolicy {
    /// Hook called before each request is sent (including retries)
    ///
    /// Currently a no-op for this policy. Future implementations may add
    /// request-specific headers or logging here.
    ///
    /// # Arguments
    /// * `_request` - Mutable reference to the HTTP request being sent
    fn on_before_send_request(&self, _request: &mut Request) {
        // At the moment, this is a No-op for the policy.
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
    /// `ShouldRetryResult` with:
    /// - `should_retry`: true if error is 429 and retry limits not exceeded
    /// - `backoff_time`: Calculated delay with exponential backoff
    ///
    /// # Note
    /// Currently uses a fixed 500ms base retry delay. Future versions may parse
    /// the `x-ms-retry-after-ms` header from the error context.
    async fn should_retry_exception(
        &self,
        err: &azure_core::Error,
    ) -> ShouldRetryResult {

        // Check if the error has an HTTP status code and if it's a valid throttle status
        // Early return for invalid or missing status codes
        let status = err.http_status().filter(|&s| self.is_valid_throttle_status_code(s));
        if status.is_none() {
            // For non-HTTP errors or non-throttle status codes, don't retry
            return ShouldRetryResult::no_retry();
        }

        // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
        self.should_retry_with_backoff(Some(Duration::from_millis(500)))
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
    /// `ShouldRetryResult` with:
    /// - `should_retry`: true if status is 429 and retry limits not exceeded
    /// - `backoff_time`: Calculated delay with exponential backoff
    ///
    /// # Note
    /// Currently uses a fixed 500ms base retry delay. Future versions may parse
    /// the `x-ms-retry-after-ms` header from the response headers to respect
    /// server-suggested retry delays.
    async fn should_retry_response(
        &self,
        response: &RawResponse,
    ) -> ShouldRetryResult {
        if !self.is_valid_throttle_status_code(response.status()) {
            return ShouldRetryResult::no_retry();
        }

        // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
        self.should_retry_with_backoff(Some(Duration::from_millis(500)))
    }
}