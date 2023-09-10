use async_trait::async_trait;
use url::Url;

use crate::{
    consumer::EventPosition, event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy, producer::PartitionPublishingOptions,
    PartitionProperties,
};

use super::transport_producer_features::TransportProducerFeatures;

#[async_trait]
pub trait TransportClient: Sized {
    type Producer<RP>
    where
        RP: EventHubsRetryPolicy + Send;
    type Consumer<RP>
    where
        RP: EventHubsRetryPolicy + Send;

    type OpenProducerError: std::error::Error;
    type RecoverProducerError: std::error::Error;
    type OpenConsumerError: std::error::Error;
    type RecoverConsumerError: std::error::Error;
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

    async fn recover_producer<RP>(
        &mut self,
        producer: &mut Self::Producer<RP>,
    ) -> Result<(), Self::RecoverProducerError>
    where
        RP: EventHubsRetryPolicy + Send;

    #[allow(clippy::too_many_arguments)] // TODO: how to reduce the number of arguments?
    async fn create_consumer<RP>(
        &mut self,
        consumer_group: &str,
        partition_id: &str,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
        owner_level: Option<i64>,
        prefetch_count: Option<u32>,
    ) -> Result<Self::Consumer<RP>, Self::OpenConsumerError>
    where
        RP: EventHubsRetryPolicy + Send;

    async fn recover_consumer<RP>(
        &mut self,
        consumer: &mut Self::Consumer<RP>,
    ) -> Result<(), Self::RecoverConsumerError>
    where
        RP: EventHubsRetryPolicy + Send;

    /// Closes the connection to the transport client instance.
    async fn close(
        &mut self,
        // cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError>;

    async fn close_if_owned(
        &mut self,
        // cancellation_token: Option<CancellationToken>,
    ) -> Result<(), Self::DisposeError>;

    fn is_owned(&self) -> bool;

    fn is_shared(&self) -> bool;
}
