use async_trait::async_trait;
use url::Url;

use crate::{event_hubs_properties::EventHubProperties, PartitionProperties, producer::PartitionPublishingOptions, event_hubs_retry_policy::EventHubsRetryPolicy, consumer::EventPosition};

use super::{transport_consumer::TransportConsumer, transport_producer::TransportProducer, transport_producer_features::TransportProducerFeatures};

#[async_trait]
pub trait TransportClient {
    type Producer: TransportProducer;
    type Consumer: TransportConsumer;
    type OpenProducerError: std::error::Error;
    type OpenConsumerError: std::error::Error;

    fn is_closed(&self) -> bool;

    fn service_endpoint(&self) -> &Url;

    async fn properties(&self) -> EventHubProperties;

    async fn partition_properties(&self, partition_id: &str) -> PartitionProperties;

    async fn create_producer<RP>(
        &mut self,
        partition_id: Option<String>,
        producer_identifier: Option<String>,
        requested_features: TransportProducerFeatures,
        partition_options: PartitionPublishingOptions,
        retry_policy: RP
    ) -> Result<Self::Producer, Self::OpenProducerError>
    where
        RP: EventHubsRetryPolicy + Send;

    async fn create_consumer<RP>(
        &mut self,
        consumer_group: Option<String>,
        partition_id: Option<String>,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
        invalidate_consumer_when_partition_stolen: bool,
        owner_level: Option<i64>,
        prefetch_count: Option<u32>,
    ) -> Result<Self::Consumer, Self::OpenConsumerError>
    where
        RP: EventHubsRetryPolicy + Send;
}
