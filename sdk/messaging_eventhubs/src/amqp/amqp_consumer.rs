use std::time::Duration as StdDuration;

use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Receiver};

use crate::{core::TransportConsumer, event_hubs_retry_policy::EventHubsRetryPolicy, util::sharable::Sharable, ReceivedEvent};

use super::{amqp_client::AmqpClient, error::RecoverAndReceiveError};

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) last_received_event: Option<ReceivedEvent>,
    pub(crate) retry_policy: RP,
}

pub struct RecoverableAmqpConsumer<'a, RP> {
    consumer: &'a mut AmqpConsumer<RP>,
    client: &'a Sharable<AmqpClient>,
}

#[async_trait]
impl<'a, RP> TransportConsumer for RecoverableAmqpConsumer<'a, RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type ReceivedEvent = ReceivedEvent;
    type ReceiveError = RecoverAndReceiveError;

    fn last_received_event(&self) -> Option<&Self::ReceivedEvent> {
        self.consumer.last_received_event.as_ref()
    }

    async fn receive(
        &mut self,
        maximum_event_count: u32,
        maximum_wait_time: Option<StdDuration>,
    ) -> Result<Vec<Self::ReceivedEvent>, Self::ReceiveError> {
        todo!()
    }
}
