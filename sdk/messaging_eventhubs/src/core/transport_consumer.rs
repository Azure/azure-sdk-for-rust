use std::time::Duration as StdDuration;

use async_trait::async_trait;
use futures_util::Stream;

#[async_trait]
pub trait TransportConsumer {
    type ReceivedEvent;
    type ReceiveError: std::error::Error;
    type Stream<'s>: Stream<Item=Result<Self::ReceivedEvent, Self::ReceiveError>> where Self: 's;

    fn last_received_event(&self) -> Option<&Self::ReceivedEvent>;

    fn receive(
        &mut self,
        maximum_event_count: Option<u32>,
        maximum_wait_time: Option<StdDuration>,
    ) -> Self::Stream<'_>;
}
