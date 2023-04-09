use async_trait::async_trait;
use url::Url;

use crate::{
    consumer::EventPosition, event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy, producer::PartitionPublishingOptions,
    PartitionProperties,
};

use super::{
    transport_consumer::TransportConsumer, transport_producer::TransportProducer,
    transport_producer_features::TransportProducerFeatures,
};

#[async_trait]
pub trait TransportClient: Sized {
    type Producer<RP>: TransportProducer
    where
        RP: EventHubsRetryPolicy + Send;
    type Consumer<RP>: TransportConsumer
    where
        RP: EventHubsRetryPolicy + Send;
    type OpenProducerError: std::error::Error;
    type OpenConsumerError: std::error::Error;
    type DisposeError: std::error::Error;

    fn is_closed(&self) -> bool;

    fn service_endpoint(&self) -> &Url;

    async fn get_properties<RP>(
        &mut self,
        retry_policy: RP,
    ) -> Result<EventHubProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send;

    async fn get_partition_properties<RP>(
        &mut self,
        partition_id: &str,
        retry_policy: RP,
    ) -> Result<PartitionProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send;

    async fn create_producer<RP>(
        &mut self,
        partition_id: Option<String>,
        producer_identifier: Option<String>,
        requested_features: TransportProducerFeatures,
        partition_options: PartitionPublishingOptions,
        retry_policy: RP,
    ) -> Result<Self::Producer<RP>, Self::OpenProducerError>
    where
        RP: EventHubsRetryPolicy + Send;

    async fn create_consumer<RP>(
        &mut self,
        consumer_group: String,
        partition_id: String,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
        invalidate_consumer_when_partition_stolen: bool,
        owner_level: Option<i64>,
        prefetch_count: Option<u32>,
    ) -> Result<Self::Consumer<RP>, Self::OpenConsumerError>
    where
        RP: EventHubsRetryPolicy + Send;

    /// Closes the connection to the transport client instance.
    async fn close(
        &mut self,
        // cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError>;

    /// Performs the task needed to clean up resources used by the client,
    /// including ensuring that the client itself has been closed.
    async fn dispose(mut self) -> Result<(), Self::DisposeError> {
        self.close().await?;
        Ok(())
    }
}
