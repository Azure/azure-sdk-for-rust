use std::time::Duration as StdDuration;

use async_trait::async_trait;

#[async_trait]
pub trait TransportConsumer {
    type ReceivedEvent;
    type ReceiveError: std::error::Error;

    fn last_received_event(&self) -> Option<&Self::ReceivedEvent>;

    async fn receive(
        &mut self,
        maximum_event_count: u32,
        maximum_wait_time: Option<StdDuration>,
    ) -> Result<Vec<Self::ReceivedEvent>, Self::ReceiveError>;
}
