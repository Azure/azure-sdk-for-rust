use async_trait::async_trait;

use crate::{
    producer::{create_batch_options::CreateBatchOptions, send_event_options::SendEventOptions},
    Event,
};

use super::transport_event_batch::TransportEventBatch;

#[async_trait]
pub trait TransportProducer {
    type MessageBatch: TransportEventBatch;

    type SendError: std::error::Error;
    type CreateBatchError: std::error::Error;

    fn create_batch(
        &self,
        options: CreateBatchOptions,
    ) -> Result<Self::MessageBatch, Self::CreateBatchError>;

    async fn send(
        &mut self,
        events: impl Iterator<Item = Event> + ExactSizeIterator + Send,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError>;

    async fn send_batch(
        &mut self,
        batch: Self::MessageBatch,
        options: SendEventOptions,
    ) -> Result<(), Self::SendError>;

    // async fn read_initialization_publishing_properties(&mut self) -> Result<PartitionPublishingOptions, ()>;
}