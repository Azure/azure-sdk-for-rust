use std::{collections::HashMap, marker::PhantomData};

use crate::{
    amqp::{amqp_client::AmqpClient, amqp_producer::AmqpProducer},
    core::{transport_producer::TransportProducer, basic_retry_policy::BasicRetryPolicy},
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::IntoAzureCoreError,
    EventHubConnection, EventHubsRetryOptions,
};

use super::event_hub_producer_client_options::EventHubProducerClientOptions;

pub const MINIMUM_BATCH_SIZE_LIMIT: usize = 24;

pub struct EventHubProducerClient<RP> {
    connection: EventHubConnection<AmqpClient>,
    producer_pool: HashMap<String, AmqpProducer<RP>>,
    options: EventHubProducerClientOptions,
    retry_policy_marker: PhantomData<RP>,
}

impl EventHubProducerClient<BasicRetryPolicy> {
    pub fn with_policy<P>() -> WithCustomRetryPolicy<P>
    where
        P: EventHubsRetryPolicy + Send,
    {
        WithCustomRetryPolicy {
            _retry_policy_marker: PhantomData,
        }
    }

    pub async fn new(
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new(connection_string, event_hub_name, client_options).await
    }

    pub fn with_connection(
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubProducerClientOptions
    ) -> Self {
        Self::with_policy()
            .with_connection(connection, client_options)
    }
}

pub struct WithCustomRetryPolicy<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl<RP> WithCustomRetryPolicy<RP> {
    pub async fn new(
        self,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<EventHubProducerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new(
            connection_string.into(),
            event_hub_name.into(),
            client_options.connection_options.clone()
        ).await?;
        Ok(EventHubProducerClient {
            connection,
            producer_pool: HashMap::new(),
            options: client_options,
            retry_policy_marker: PhantomData,
        })
    }

    pub fn with_connection(
        self,
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubProducerClientOptions
    ) -> EventHubProducerClient<RP> {
        let connection = connection.clone_as_shared();

        EventHubProducerClient {
            connection,
            producer_pool: HashMap::new(),
            options: client_options,
            retry_policy_marker: PhantomData,
        }
    }
}

impl<RP> EventHubProducerClient<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
{
    async fn get_pooled_producer_mut(
        &mut self,
        partition_id: &str,
    ) -> Result<&mut AmqpProducer<RP>, azure_core::Error> {
        if !self.producer_pool.contains_key(partition_id) {
            let producer_identifier = Some(
                self.options
                    .identifier
                    .clone()
                    .unwrap_or(uuid::Uuid::new_v4().to_string()),
            );
            let requested_features = self.options.create_features();
            let retry_policy = RP::from(self.options.retry_options.clone());
            let partition_options = self
                .options
                .get_publishing_options_or_default_for_partition(Some(partition_id));

            // let producer = self.connection
            //     .create_transport_producer::<RP>(
            //         Some(partition_id.to_string()),
            //         producer_identifier,
            //         requested_features,
            //         partition_options,
            //         retry_policy,
            //     )
            //     .await?;
        }

        // This is safe because we just checked that the key exists.
        Ok(self.producer_pool.get_mut(partition_id).unwrap())
    }

    pub async fn close(self) -> Result<(), azure_core::Error> {
        let mut result = Ok(());
        for (_, producer) in self.producer_pool {
            let res = producer
                .close()
                .await
                .map_err(IntoAzureCoreError::into_azure_core_error);
            result = result.and(res);
        }

        let res = self.connection.close_if_owned().await;
        result.and(res)
    }
}
