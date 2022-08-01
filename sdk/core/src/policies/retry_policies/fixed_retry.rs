use std::time::Duration;
use time::OffsetDateTime;

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
    max_delay: Duration,
}

impl FixedRetryPolicy {
    pub(crate) fn new(delay: Duration, max_retries: u32, max_delay: Duration) -> Self {
        Self {
            delay,
            max_retries,
            max_delay,
        }
    }
}

impl super::RetryPolicy for FixedRetryPolicy {
    fn is_expired(&self, first_retry_time: &mut Option<OffsetDateTime>, retry_count: u32) -> bool {
        if retry_count > self.max_retries {
            return true;
        }
        let first_retry_time = first_retry_time.get_or_insert_with(OffsetDateTime::now_utc);
        OffsetDateTime::now_utc() > *first_retry_time + self.max_delay
    }

    fn sleep_duration(&self, _retry_count: u32) -> Duration {
        let sleep_ms = self.delay.as_millis() as u64 + u64::from(rand::random::<u8>());
        Duration::from_millis(sleep_ms)
    }
}
