use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Sender, link::SendError};

use crate::{core::{transport_producer::TransportProducer, transport_producer_features::TransportProducerFeatures}, event_hubs_retry_policy::EventHubsRetryPolicy, producer::{PartitionPublishingOptions, create_batch_options::CreateBatchOptions, send_event_options::SendEventOptions, event_hub_producer_client::MINIMUM_BATCH_SIZE_LIMIT}, util::IntoAzureCoreError, Event, amqp::amqp_message_converter::create_envelope_from_events};

use super::{amqp_connection_scope::AmqpConnectionScope, error::{OpenProducerError, DisposeProducerError, RequestedSizeOutOfRange}, amqp_event_batch::AmqpEventBatch};

pub struct AmqpProducer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,
    pub(crate) initialized_partition_properties: PartitionPublishingOptions,
    pub(crate) retry_policy: RP,
}

#[async_trait]
impl<RP> TransportProducer for AmqpProducer<RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    type MessageBatch = AmqpEventBatch;

    type SendError = SendError;
    type CreateBatchError = RequestedSizeOutOfRange;

    type DisposeError = DisposeProducerError;

    fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateBatchError> {
        let link_max_message_size = self.sender.max_message_size().unwrap_or(u64::MAX);
        let max_size_in_bytes = match options.max_size_in_bytes {
            Some(max_size_in_bytes) => {
                if max_size_in_bytes < MINIMUM_BATCH_SIZE_LIMIT as u64
                    || max_size_in_bytes > link_max_message_size
                {
                    return Err(RequestedSizeOutOfRange {});
                }

                max_size_in_bytes
            }
            // If this field is zero or unset, there is no maximum size imposed by the link endpoint.
            None => link_max_message_size,
        };
        Ok(AmqpEventBatch::new(max_size_in_bytes, options))
    }

    async fn send(&mut self, events: impl Iterator<Item = Event> + ExactSizeIterator + Send, options: SendEventOptions) -> Result<(), Self::SendError> {
        let envelope = create_envelope_from_events(events, options.partition_key);

        // TODO: check size of envelope and make sure it's not too big

        todo!()
    }

    async fn send_batch(&mut self, batch: Self::MessageBatch, options: SendEventOptions) -> Result<(), Self::SendError> {
        todo!()
    }

    // async fn read_initialization_publishing_properties(&mut self) -> Result<PartitionPublishingOptions, ()> { todo!() }


    async fn close(mut self) -> Result<(), Self::DisposeError> {
        self.sender.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }
}
