use crate::{event_hubs_retry_policy::EventHubsRetryPolicy, event_hubs_retry_options::EventHubsRetryOptions};

pub struct BasicRetryPolicy {

}

impl From<EventHubsRetryOptions> for BasicRetryPolicy {
    fn from(_: EventHubsRetryOptions) -> Self {
        todo!()
    }
}

impl EventHubsRetryPolicy for BasicRetryPolicy {
    fn calculate_try_timeout(&self, attempt_count: u32) -> std::time::Duration {
        todo!()
    }

    fn calculate_retry_delay(
        &self,
        last_error: &dyn std::error::Error,
        attempt_count: u32,
    ) -> Option<std::time::Duration> {
        todo!()
    }
}
