use std::time::Duration;

/// Retry policy with fixed back-off.
///
/// Retry policy with fixed back-off (with an added random delay up to 256 ms). Each retry will
/// happen at least after the same, configured sleep time. The policy will retry until the maximum number of
/// retries have been reached or the maximum allowed delay has passed (whichever comes first). The
/// wait time is not precise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedRetryPolicy {
    delay: Duration,
    max_retries: u32,
    max_elapsed: Duration,
}

impl FixedRetryPolicy {
    pub(crate) fn new(delay: Duration, max_retries: u32, max_elapsed: Duration) -> Self {
        Self {
            delay: delay.max(Duration::from_millis(10)),
            max_retries,
            max_elapsed,
        }
    }
}

impl super::RetryPolicy for FixedRetryPolicy {
    fn is_expired(&self, time_since_start: Duration, retry_count: u32) -> bool {
        retry_count >= self.max_retries || time_since_start >= self.max_elapsed
    }

    fn sleep_duration(&self, _retry_count: u32) -> Duration {
        let sleep_ms = self.delay.as_millis() as u64 + u64::from(rand::random::<u8>());
        Duration::from_millis(sleep_ms)
    }
}
