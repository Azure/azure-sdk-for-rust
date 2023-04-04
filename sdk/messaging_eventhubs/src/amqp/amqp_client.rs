use async_trait::async_trait;
use fe2o3_amqp_management::error::Error as ManagementError;
use url::Url;

use crate::{
    consumer::EventPosition,
    core::{
        transport_client::TransportClient, transport_producer_features::TransportProducerFeatures,
    },
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::PartitionPublishingOptions,
    PartitionProperties, util,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_consumer::AmqpConsumer,
    amqp_management_link::AmqpManagementLink,
    amqp_producer::AmqpProducer,
    error::{OpenConsumerError, OpenProducerError},
};

pub(crate) struct AmqpClient {
    connection_scope: AmqpConnectionScope,
    management_link: AmqpManagementLink,
}

#[async_trait]
impl TransportClient for AmqpClient {
    type Producer = AmqpProducer;
    type Consumer = AmqpConsumer;
    type OpenProducerError = OpenProducerError;
    type OpenConsumerError = OpenConsumerError;
    type ManagementError = ManagementError;

    fn is_closed(&self) -> bool {
        self.connection_scope.is_disposed
    }

    fn service_endpoint(&self) -> &Url {
        &self.connection_scope.service_endpoint
    }

    async fn get_properties(&self) -> Result<EventHubProperties, Self::ManagementError> {
        todo!()
    }

    async fn get_partition_properties(
        &self,
        partition_id: &str,
    ) -> Result<PartitionProperties, Self::ManagementError> {
        todo!()
    }

    async fn create_producer<RP>(
        &mut self,
        partition_id: Option<String>,
        producer_identifier: Option<String>,
        requested_features: TransportProducerFeatures,
        partition_options: PartitionPublishingOptions,
        retry_policy: RP,
    ) -> Result<Self::Producer, Self::OpenProducerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let operation_timeout = retry_policy.calculate_try_timeout(0);
        let fut = self.connection_scope.open_producer_link(partition_id, requested_features, partition_options, producer_identifier);
        let producer = util::time::timeout(operation_timeout, fut).await??;

        Ok(producer)
    }

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
        RP: EventHubsRetryPolicy + Send,
    {
        todo!()
    }
}
