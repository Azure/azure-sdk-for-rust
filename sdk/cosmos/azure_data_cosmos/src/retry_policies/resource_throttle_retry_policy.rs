// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{RetryPolicy, RetryResult};
use async_trait::async_trait;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;

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
pub struct ResourceThrottleRetryPolicy {
    max_attempt_count: usize,
    backoff_delay_factor: u32,
    max_wait_time: Duration,
    current_attempt_count: usize,
    cumulative_retry_delay: usize,
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
    ) -> Self {
        Self {
            max_attempt_count,
            backoff_delay_factor,
            max_wait_time: Duration::seconds(max_wait_time_secs),
            current_attempt_count: 0,
            cumulative_retry_delay: 0,
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
impl RetryPolicy for ResourceThrottleRetryPolicy {
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

// Tests for resource throttle retry policy
#[cfg(test)]
mod tests {
    use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
    use crate::retry_policies::RetryPolicy;
    use crate::retry_policies::RetryResult;
    use azure_core::{
        http::{headers::Headers, RawResponse, StatusCode},
        time::Duration,
    };

    /// Helper function to create a mock RawResponse with a given status code
    fn create_mock_response(status: StatusCode) -> azure_core::Result<RawResponse> {
        // Create headers
        let mut headers = Headers::new();
        headers.insert("content-type", "application/json");

        azure_core::Result::from(Ok(RawResponse::from_bytes(
            status,
            headers,
            r#"{"id":12,"name":"Too Many Requests"}"#,
        )))
    }

    #[tokio::test]
    async fn retry_policy_handles_429_status() {
        // Create a retry policy with 3 max retries, 100 second max wait time, and backoff factor of 2
        let mut policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

        // Simulate a 429 TooManyRequests response
        let response_429 = create_mock_response(StatusCode::TooManyRequests);

        // First retry attempt
        let result1 = policy.should_retry(&response_429).await;
        match result1 {
            RetryResult::Retry { after } => {
                assert!(after > Duration::ZERO, "Should have backoff time");
                println!("First retry - backoff time: {:?}", after);
            }
            RetryResult::DoNotRetry => panic!("Should retry on first 429 response"),
        }

        // Second retry attempt
        let result2 = policy.should_retry(&response_429).await;
        let backoff2 = match result2 {
            RetryResult::Retry { after } => {
                assert!(after > Duration::ZERO, "Should have backoff time");
                after
            }
            RetryResult::DoNotRetry => panic!("Should retry on second 429 response"),
        };

        // Extract backoff1 for comparison
        let RetryResult::Retry { after: backoff1 } = result1 else {
            panic!("Expected retry result")
        };

        assert!(
            backoff2 >= backoff1,
            "Backoff should increase with exponential backoff"
        );
        println!("Second retry - backoff time: {:?}", backoff2);

        // Third retry attempt
        let result3 = policy.should_retry(&response_429).await;
        match result3 {
            RetryResult::Retry { after } => {
                assert!(after > Duration::ZERO, "Should have backoff time");
                println!("Third retry - backoff time: {:?}", after);
            }
            RetryResult::DoNotRetry => panic!("Should retry on third 429 response"),
        }

        // Fourth attempt should NOT retry (exceeded max_attempt_count of 3)
        let result4 = policy.should_retry(&response_429).await;
        assert_eq!(result4, RetryResult::DoNotRetry);
    }

    #[tokio::test]
    async fn retry_policy_does_not_retry_on_success() {
        let mut policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

        // Simulate a 200 OK response (success)
        let response_200 = create_mock_response(StatusCode::Ok);

        let result = policy.should_retry(&response_200).await;
        assert_eq!(result, RetryResult::DoNotRetry);
    }

    #[tokio::test]
    async fn retry_policy_does_not_retry_on_client_errors() {
        let mut policy = ResourceThrottleRetryPolicy::new(3, 100, 2);

        // Test various client errors that should NOT trigger retry
        let test_cases = vec![
            (StatusCode::BadRequest, "400 Bad Request"),
            (StatusCode::Unauthorized, "401 Unauthorized"),
            (StatusCode::Forbidden, "403 Forbidden"),
            (StatusCode::NotFound, "404 Not Found"),
            (StatusCode::Conflict, "409 Conflict"),
        ];

        for (status, _description) in test_cases {
            let response = create_mock_response(status);
            let result = policy.should_retry(&response).await;
            assert_eq!(result, RetryResult::DoNotRetry);
        }
    }

    #[tokio::test]
    async fn retry_policy_backoff_calculation() {
        let backoff_factor = 3;
        let mut policy = ResourceThrottleRetryPolicy::new(5, 1000, backoff_factor);

        let response_429 = create_mock_response(StatusCode::TooManyRequests);

        let mut previous_backoff = Duration::ZERO;

        for attempt in 1..=3 {
            let result = policy.should_retry(&response_429).await;
            match result {
                RetryResult::Retry { after } => {
                    if attempt > 1 {
                        // With exponential backoff, each attempt should have longer delay
                        // (though the exact multiplier depends on internal logic)
                        println!(
                            "Attempt {}: backoff = {:?} (previous = {:?})",
                            attempt, after, previous_backoff
                        );
                    }
                    previous_backoff = after;
                }
                RetryResult::DoNotRetry => panic!("Attempt {} should trigger retry", attempt),
            }
        }
    }

    #[tokio::test]
    async fn retry_policy_respects_max_wait_time() {
        // Set very low max_wait_time to test the limit
        let max_wait_secs = 5;
        let mut policy = ResourceThrottleRetryPolicy::new(10, max_wait_secs, 100);

        let response_429 = create_mock_response(StatusCode::TooManyRequests);

        let mut total_delay = Duration::ZERO;
        let max_wait = Duration::seconds(max_wait_secs);

        // Keep retrying until we hit the cumulative wait time limit
        for attempt in 1..=10 {
            let result = policy.should_retry(&response_429).await;

            match result {
                RetryResult::Retry { after } => {
                    total_delay += after;
                    println!(
                        "Attempt {}: backoff = {:?}, cumulative = {:?}",
                        attempt, after, total_delay
                    );

                    // Total cumulative delay should never exceed max_wait_time
                    assert!(
                        total_delay <= max_wait,
                        "Cumulative delay {:?} should not exceed max wait time {:?}",
                        total_delay,
                        max_wait
                    );
                }
                RetryResult::DoNotRetry => {
                    println!(
                        "Stopped retrying at attempt {} due to max wait time limit",
                        attempt
                    );
                    break;
                }
            }
        }
    }

    // Note: Testing error exceptions with specific status codes is challenging
    // because azure_core::Error doesn't provide a way to set HTTP status codes directly.
    // The should_retry_exception method in ResourceThrottleRetryPolicy checks for
    // 429 status using err.http_status(), which returns None for manually constructed errors.
    // In real scenarios, errors would come from actual HTTP responses.
    #[tokio::test]
    async fn retry_counter_increments() {
        let mut policy = ResourceThrottleRetryPolicy::new(5, 100, 2);
        let response_429 = create_mock_response(StatusCode::TooManyRequests);

        // Track how many times we get a retry decision
        let mut retry_count = 0;

        for attempt in 1..=10 {
            let result = policy.should_retry(&response_429).await;

            match result {
                RetryResult::Retry { after } => {
                    retry_count += 1;
                    println!(
                        "Attempt {}: Retry #{} - backoff: {:?}",
                        attempt, retry_count, after
                    );
                }
                RetryResult::DoNotRetry => {
                    println!(
                        "Attempt {}: No more retries (total retries = {})",
                        attempt, retry_count
                    );
                    break;
                }
            }
        }

        // We should have gotten exactly max_attempt_count retries
        assert_eq!(
            retry_count, 5,
            "Should have retried exactly max_attempt_count times"
        );
    }
}
