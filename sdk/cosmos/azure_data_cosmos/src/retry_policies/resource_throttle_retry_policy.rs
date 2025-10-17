// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use azure_core::http::{RawResponse, StatusCode};
use typespec_client_core::http::Request;
use super::{IRetryPolicy, ShouldRetryResult};

#[derive(Debug)]
pub struct ResourceThrottleRetryPolicy {
    max_attempt_count: usize,
    backoff_delay_factor: u32,
    max_wait_time: Duration,
    current_attempt_count: AtomicUsize,
    cumulative_retry_delay: Arc<AtomicUsize>,
}

impl ResourceThrottleRetryPolicy {
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

    fn is_valid_throttle_status_code(&self, status_code: StatusCode) -> bool {
        status_code == StatusCode::TooManyRequests
    }

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

    /// Common retry logic for both exceptions and responses
    /// 
    /// This method encapsulates the shared retry decision logic:
    /// - Checks if we haven't exceeded max retry attempts
    /// - Calculates backoff delay
    /// - Increments attempt counter if retrying
    /// 
    /// # Arguments
    /// * `retry_after` - Optional duration to wait before retrying
    /// 
    /// # Returns
    /// `ShouldRetryResult` indicating whether to retry and the backoff time
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
impl IRetryPolicy for ResourceThrottleRetryPolicy {
    fn on_before_send_request(&self, _request: &mut Request) {
        // No-op for this policy
    }

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

        self.should_retry_with_backoff(Some(Duration::from_secs(10)))
    }

    async fn should_retry_response(
        &self,
        response: &RawResponse,
    ) -> ShouldRetryResult {
        if !self.is_valid_throttle_status_code(response.status()) {
            return ShouldRetryResult::no_retry();
        }

        self.should_retry_with_backoff(Some(Duration::from_secs(10)))
    }
}