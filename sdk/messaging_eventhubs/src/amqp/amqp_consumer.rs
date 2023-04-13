use std::time::Duration as StdDuration;

use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Receiver, link::RecvError};
use futures_util::{Sink, SinkExt};

use crate::{core::TransportConsumer, event_hubs_retry_policy::EventHubsRetryPolicy, util::sharable::Sharable, ReceivedEvent, consumer::EventPosition};

use super::{amqp_client::AmqpClient, error::{RecoverAndReceiveError, DisposeConsumerError}};

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) track_last_enqueued_event_properties: bool,
    pub(crate) last_received_event: Option<ReceivedEvent>,
    pub(crate) current_event_position: Option<EventPosition>,
    pub(crate) retry_policy: RP,
}

impl<RP> AmqpConsumer<RP> {
    pub async fn dispose(mut self) -> Result<(), DisposeConsumerError> {
        self.receiver.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }

    async fn receive_messages<S>(
        &mut self,
        prefetch_count: u32,
        maximum_messages: Option<u32>,
        buffer: &mut S,
    ) -> Result<(), RecvError>
    where
        S: Sink<ReceivedEvent> + Unpin,
        RecvError: From<S::Error>,
    {
        // Credit mode is manual, need to set credit
        let maximum_messages = maximum_messages.unwrap_or(1);
        if prefetch_count == 0 {
            self.receiver.set_credit(maximum_messages).await?;
        }

        for _ in 0..maximum_messages {
            let delivery = self.receiver.recv().await?;
            self.receiver.accept(&delivery).await?;
            let event = ReceivedEvent::from_raw_amqp_message(delivery.into_message());
            if self.track_last_enqueued_event_properties {
                self.last_received_event = Some(event.clone());
            }

            if event.offset() > i64::MIN {
                self.current_event_position = Some(EventPosition::from_offset(event.offset(), false));
            }

            buffer.send(event).await?;
        }

        Ok(())
    }
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
