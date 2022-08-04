use super::RetryPolicy;
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
    initial_delay: Duration,
    max_retries: u32,
    max_elapsed: Duration,
    max_delay: Duration,
}

impl ExponentialRetryPolicy {
    pub(crate) fn new(
        initial_delay: Duration,
        max_retries: u32,
        max_elapsed: Duration,
        max_delay: Duration,
    ) -> Self {
        Self {
            initial_delay: initial_delay.max(Duration::from_millis(1)),
            max_retries,
            max_elapsed,
            max_delay: max_delay.max(Duration::from_secs(1)),
        }
    }
}

impl RetryPolicy for ExponentialRetryPolicy {
    fn is_expired(&self, time_since_start: Duration, retry_count: u32) -> bool {
        retry_count >= self.max_retries || time_since_start >= self.max_elapsed
    }

    fn sleep_duration(&self, retry_count: u32) -> Duration {
        let sleep_ms = self.initial_delay.as_millis() as u64 * 2u64.pow(retry_count)
            + u64::from(rand::random::<u8>());
        let sleep_ms = sleep_ms.min(self.max_delay.as_millis().try_into().unwrap_or(u64::MAX));
        Duration::from_millis(sleep_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponentialy_increases_correctly() {
        let options = crate::options::RetryOptions::default();
        let policy = ExponentialRetryPolicy::new(
            options.delay,
            options.max_retries,
            options.max_elapsed,
            options.max_delay,
        );

        let mut elapsed_time = Duration::from_secs(0);
        let mut retry_count = 0;
        let mut durations = vec![];
        while !policy.is_expired(elapsed_time, retry_count) {
            retry_count += 1; // increase at beginning since we only check expiration if we need to retry
            let duration = policy.sleep_duration(retry_count);
            durations.push(duration);
            elapsed_time += duration; // simulate sleep
        }

        let actual = durations
            .into_iter()
            .map(|d| d.as_secs())
            .collect::<Vec<_>>();
        let expected = &[0, 0, 1, 3, 6, 12, 25, 30];
        assert_eq!(
            actual.len(),
            expected.len(),
            "Different number of durations than expected"
        );

        for (&a, &e) in actual.iter().zip(expected.iter()) {
            // Check within one second to account for the jitter
            assert!(
                a == e || a + 1 == e || a == e + 1,
                "actual != expected\nActual: {actual:?}\nExpected: {expected:?}"
            )
        }
    }
}
