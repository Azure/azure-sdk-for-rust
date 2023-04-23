use std::time::Duration;

///A policy governinng retrying of operations.
pub trait EventHubsRetryPolicy: Clone {
    /// Calculates the amount of time before considering the operation to have timed out.
    fn calculate_try_timeout(&self, attempt_count: u32) -> Duration;

    /// Calculates the amount of time to wait before retrying an operation.
    fn calculate_retry_delay(
        &self,
        last_error: &dyn std::error::Error,
        attempt_count: u32,
    ) -> Option<Duration>;
}
