// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use typespec_client_core::http::Request;

/// Retry policy for handling resource throttling (429 TooManyRequests) errors
///
/// This policy implements exponential backoff for 429 status codes, respecting both
/// maximum retry attempts and cumulative wait time limits. It's designed to handle
/// Azure Cosmos DB throttling scenarios where the service limits request rates.
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
        max_wait_time_secs: i64,
        backoff_delay_factor: u32,
    ) -> Self {
        Self {
            max_attempt_count,
            backoff_delay_factor,
            max_wait_time: Duration::seconds(max_wait_time_secs),
            current_attempt_count: AtomicUsize::new(0),
            cumulative_retry_delay: Arc::new(AtomicUsize::new(0)),
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
    fn check_if_retry_needed(&self, retry_after: Option<Duration>) -> RetryResult {
        let retry_delay = retry_after.unwrap_or(Duration::seconds(5)) * self.backoff_delay_factor;

        let cumulative = self.cumulative_retry_delay.load(Ordering::Relaxed) as u64;
        let cumulative = cumulative + retry_delay.as_seconds_f64() as u64;

        if retry_delay < self.max_wait_time
            && cumulative <= self.max_wait_time.as_seconds_f64() as u64
        {
            self.cumulative_retry_delay
                .store(cumulative as usize, Ordering::Relaxed);
            return RetryResult::Retry { after: retry_delay };
        }
        RetryResult::DoNotRetry
    }

    /// Common retry logic for both errors and responses with exponential backoff
    ///
    /// This method encapsulates the shared retry decision logic used by both
    /// `should_retry_error` and `should_retry_response`. It provides a centralized
    /// place for retry decision-making to ensure consistency across different failure modes.
    fn should_retry_with_backoff(&self, retry_after: Option<Duration>) -> RetryResult {
        let attempt = self.current_attempt_count.load(Ordering::Relaxed);
        if attempt < self.max_attempt_count {
            let retry_result = self.check_if_retry_needed(retry_after);

            if matches!(retry_result, RetryResult::Retry { .. }) {
                self.current_attempt_count.fetch_add(1, Ordering::Relaxed);
                return retry_result;
            }
        }

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
    fn should_retry_error(&self, err: &azure_core::Error) -> RetryResult {
        // Check if the error has an HTTP status code and if it's a valid throttle status
        // Early return for invalid or missing status codes
        let status = err
            .http_status()
            .filter(|&s| s == StatusCode::TooManyRequests);
        if status.is_none() {
            // For non-HTTP errors or non-throttle status codes, don't retry
            return RetryResult::DoNotRetry;
        }

        // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
        self.should_retry_with_backoff(Some(Duration::milliseconds(500)))
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
    fn should_retry_response(&self, response: &RawResponse) -> RetryResult {
        if response.status() != StatusCode::TooManyRequests {
            return RetryResult::DoNotRetry;
        }

        // Get the retry_after field from `x-ms-retry-after-ms` header from backend.
        self.should_retry_with_backoff(Some(Duration::milliseconds(500)))
    }
}

#[async_trait]
impl RetryPolicy for ResourceThrottleRetryPolicy {
    /// Hook called before each request is sent (including retries)
    ///
    /// Currently, a no-op for this policy. Future implementations may add
    /// request-specific headers or logging here.
    ///
    /// # Arguments
    /// * `_request` - Mutable reference to the HTTP request being sent
    fn on_before_send_request(&self, _request: &mut Request) {
        // At the moment, this is a No-op for the policy.
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
    async fn should_retry(&self, response: &azure_core::Result<RawResponse>) -> RetryResult {
        match response {
            Ok(resp) => {
                if resp.status().is_server_error() || resp.status().is_client_error() {
                    return self.should_retry_response(resp);
                }

                RetryResult::DoNotRetry
            }
            Err(err) => self.should_retry_error(err),
        }
    }
}
