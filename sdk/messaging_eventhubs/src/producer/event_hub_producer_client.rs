use std::{collections::HashMap, marker::PhantomData};

use crate::{
    amqp::{amqp_client::AmqpClient, amqp_producer::AmqpProducer},
    EventHubConnection, event_hubs_retry_policy::EventHubsRetryPolicy,
};

use super::event_hub_producer_client_options::EventHubProducerClientOptions;

pub struct EventHubProducerClient<RP> {
    connection: EventHubConnection<AmqpClient>,
    producer_pool: HashMap<String, AmqpProducer>,
    options: EventHubProducerClientOptions,
    retry_policy_marker: PhantomData<RP>,
}

impl<RP> EventHubProducerClient<RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    async fn get_pooled_producer_mut(&mut self, partition_id: &str) -> Result<&mut AmqpProducer, azure_core::Error> {
        if !self.producer_pool.contains_key(partition_id) {
            let producer_identifier = Some(self.options.identifier.clone().unwrap_or(uuid::Uuid::new_v4().to_string()));
            let requested_features = self.options.create_features();
            let retry_policy = RP::from(self.options.retry_options.clone());
            let partition_options = self.options.get_publishing_options_or_default_for_partition(Some(partition_id));

            self.connection.create_transport_producer(
                Some(partition_id.to_string()),
                producer_identifier,
                requested_features,
                partition_options,
                retry_policy,
            ).await?;
        }

        // This is safe because we just checked that the key exists.
        Ok(self.producer_pool.get_mut(partition_id).unwrap())
    }
}
