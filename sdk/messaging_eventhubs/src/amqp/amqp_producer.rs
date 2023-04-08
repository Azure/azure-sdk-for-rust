use async_trait::async_trait;
use fe2o3_amqp::{session::SessionHandle, Sender};

use crate::{core::{transport_producer::TransportProducer, transport_producer_features::TransportProducerFeatures}, event_hubs_retry_policy::EventHubsRetryPolicy, producer::PartitionPublishingOptions, util::IntoAzureCoreError};

use super::{amqp_connection_scope::AmqpConnectionScope, error::{OpenProducerError, DisposeProducerError}};

pub struct AmqpProducer {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,
    pub(crate) initialized_partition_properties: PartitionPublishingOptions,
}

impl AmqpProducer {

}

#[async_trait]
impl TransportProducer for AmqpProducer {
    type DisposeError = DisposeProducerError;

    async fn dispose(mut self) -> Result<(), Self::DisposeError> {
        self.sender.close().await?;
        self.session_handle.close().await?;
        Ok(())
    }
}
