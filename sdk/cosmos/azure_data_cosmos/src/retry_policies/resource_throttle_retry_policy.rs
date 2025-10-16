use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use azure_core::http::{request::Request, RawResponse, StatusCode};

#[derive(Debug, Clone)]
pub struct ShouldRetryResult {
    pub should_retry: bool,
    pub backoff_time: Duration,
}

impl ShouldRetryResult {
    pub fn no_retry() -> Self {
        Self {
            should_retry: false,
            backoff_time: Duration::ZERO,
        }
    }
    pub fn retry_after(backoff: Duration) -> Self {
        Self {
            should_retry: true,
            backoff_time: backoff,
        }
    }
}

#[async_trait]
pub trait DocumentClientRetryPolicy: Send + Sync {
    async fn should_retry_exception(
        &self,
        err: &azure_core::Error,
    ) -> ShouldRetryResult;

    async fn should_retry_response(
        &self,
        response: &RawResponse,
    ) -> ShouldRetryResult;

    fn on_before_send_request(&self, request: &mut Request);
}

pub struct ResourceThrottleRetryPolicy {
    max_attempt_count: usize,
    backoff_delay_factor: u32,
    max_wait_time: Duration,
    current_attempt_count: AtomicUsize,
    cumulative_retry_delay: Arc<AtomicUsize>, // milliseconds
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
}

#[async_trait]
impl DocumentClientRetryPolicy for ResourceThrottleRetryPolicy {
    async fn should_retry_exception(
        &self,
        err: &azure_core::Error,
    ) -> ShouldRetryResult {
        // When an error occurs, we can only access limited information without consuming it:
        // 1. HTTP status code (if it's an HTTP error)
        // 2. Error message via Display trait
        //
        // To extract headers (like x-ms-substatus), we would need to:
        // - Convert err to ErrorResponse (which consumes it)
        // - This is not possible in retry logic since we need to return the original error
        //
        // Therefore, retry decisions for exceptions are based only on:
        // - HTTP status code (if available)
        // - Retry attempt count
        // - Backoff timing
        
        // Check if the error has an HTTP status code and if it's a valid throttle status
        if let Some(status) = err.http_status() {
            if !self.is_valid_throttle_status_code(status) {
                return ShouldRetryResult::no_retry();
            }
        } else {
            // For non-HTTP errors (network errors, timeouts), don't retry
            // These are typically not transient Cosmos DB throttling issues
            return ShouldRetryResult::no_retry();
        }
        let attempt = self.current_attempt_count.load(Ordering::Relaxed);
        if attempt < self.max_attempt_count {
            let (should_retry, delay) = self.check_if_retry_needed(Some(Duration::from_secs(10)));
            if should_retry {
                self.current_attempt_count.fetch_add(1, Ordering::Relaxed);
                return ShouldRetryResult::retry_after(delay);
            }
        }

        ShouldRetryResult::no_retry()
    }

    async fn should_retry_response(
        &self,
        response: &RawResponse,
    ) -> ShouldRetryResult {
        if !self.is_valid_throttle_status_code(response.status()) {
            return ShouldRetryResult::no_retry();
        }
        let attempt = self.current_attempt_count.load(Ordering::Relaxed);
        if attempt < self.max_attempt_count {
            let (should_retry, delay) = self.check_if_retry_needed(Some(Duration::from_secs(10)));
            if should_retry {
                self.current_attempt_count.fetch_add(1, Ordering::Relaxed);
                return ShouldRetryResult::retry_after(delay);
            }
        }
        ShouldRetryResult::no_retry()
    }

    fn on_before_send_request(&self, _request: &mut Request) {
        // No-op for this policy
    }
}