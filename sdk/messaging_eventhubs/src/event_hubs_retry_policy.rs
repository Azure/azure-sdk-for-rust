use std::{time::Duration};

pub trait EventHubsRetryPolicy {
    fn calculate_try_timeout(&self, attempt_count: u32) -> Duration;

    fn calculate_retry_delay(
        &self,
        last_error: &dyn std::error::Error,
        attempt_count: u32,
    ) -> Option<Duration>;
}
