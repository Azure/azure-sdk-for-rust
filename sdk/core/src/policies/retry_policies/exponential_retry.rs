use chrono::{DateTime, Local};
use std::time::Duration;

/// Retry policy with exponential back-off.
///
/// Retry policy with exponential back-off (with an added random delay up to 256 ms). Each retry
/// will happen at least after an exponential wait time. So if x is the first retry wait, the
/// second will be x*2, the third x*4 and so on. The policy will retry until the maximum number of
/// retries have been reached or the maximum allowed delay has passed (whichever comes first). The
/// wait time is not precise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExponentialRetryPolicy {
    delay: Duration,
    max_retries: u32,
    max_delay: Duration,
}

impl ExponentialRetryPolicy {
    pub(crate) fn new(delay: Duration, max_retries: u32, max_delay: Duration) -> Self {
        ExponentialRetryPolicy {
            delay,
            max_retries,
            max_delay,
        }
    }
}

impl super::RetryPolicy for ExponentialRetryPolicy {
    fn is_expired(&self, first_retry_time: &mut Option<DateTime<Local>>, retry_count: u32) -> bool {
        if retry_count > self.max_retries {
            return true;
        }

        let first_retry_time = first_retry_time.get_or_insert_with(Local::now);
        let max_delay = chrono::Duration::from_std(self.max_delay)
            .unwrap_or_else(|_| chrono::Duration::max_value());

        Local::now() > *first_retry_time + max_delay
    }

    fn sleep_duration(&self, retry_count: u32) -> Duration {
        let sleep_ms = self.delay.as_millis() as u64 * u64::pow(2u64, retry_count - 1)
            + rand::random::<u8>() as u64;
        Duration::from_millis(sleep_ms)
    }
}
