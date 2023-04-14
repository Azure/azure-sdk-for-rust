use std::{collections::HashMap, marker::PhantomData};

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        amqp_producer::{AmqpProducer, RecoverableAmqpProducer},
    },
    core::{BasicRetryPolicy, TransportProducer},
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::IntoAzureCoreError,
    Event, EventHubConnection, EventHubsRetryOptions, PartitionProperties,
};

use super::{
    create_batch_options::CreateBatchOptions, event_batch::EventBatch,
    event_hub_producer_client_options::EventHubProducerClientOptions,
    send_event_options::SendEventOptions,
};

pub const MINIMUM_BATCH_SIZE_LIMIT: usize = 24;

pub struct EventHubProducerClient<RP> {
    connection: EventHubConnection<AmqpClient>,
    /// An abstracted Event Hub transport-specific producer that is associated with the
    /// Event Hub gateway rather than a specific partition; intended to perform delegated operations.
    gateway_producer: Option<AmqpProducer<RP>>,
    producer_pool: HashMap<String, AmqpProducer<RP>>,
    options: EventHubProducerClientOptions,
    retry_policy_marker: PhantomData<RP>,
}

impl EventHubProducerClient<BasicRetryPolicy> {
    pub fn with_policy<P>() -> EventHubProducerClientBuilder<P>
    where
        P: EventHubsRetryPolicy + Send,
    {
        EventHubProducerClientBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    pub async fn new(
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new(connection_string, event_hub_name, client_options)
            .await
    }

    pub fn with_connection(
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubProducerClientOptions,
    ) -> Self {
        Self::with_policy().with_connection(connection, client_options)
    }
}

pub struct EventHubProducerClientBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl<RP> EventHubProducerClientBuilder<RP> {
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
            client_options.connection_options.clone(),
        )
        .await?;
        Ok(EventHubProducerClient {
            connection,
            gateway_producer: None,
            producer_pool: HashMap::new(),
            options: client_options,
            retry_policy_marker: PhantomData,
        })
    }

    pub fn with_connection(
        self,
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubProducerClientOptions,
    ) -> EventHubProducerClient<RP> {
        let connection = connection.clone_as_shared();

        EventHubProducerClient {
            connection,
            gateway_producer: None,
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
    async fn get_or_create_gateway_producer_mut(
        &mut self,
    ) -> Result<RecoverableAmqpProducer<'_, RP>, azure_core::Error> {
        let producer_identifier = Some(
            self.options
                .identifier
                .clone()
                .unwrap_or(uuid::Uuid::new_v4().to_string()),
        );
        let requested_features = self.options.create_features();
        let retry_policy = RP::from(self.options.retry_options.clone());

        if self.gateway_producer.is_none() {
            let partition_options = self
                .options
                .get_publishing_options_or_default_for_partition(None);

            let producer = self
                .connection
                .create_transport_producer::<RP>(
                    None,
                    producer_identifier,
                    requested_features,
                    partition_options,
                    retry_policy,
                )
                .await?;
            self.gateway_producer = Some(producer);
        }

        let producer = self.gateway_producer.as_mut().unwrap();
        let client = &mut self.connection.inner;
        Ok(RecoverableAmqpProducer::new(producer, client))
    }

    async fn get_pooled_producer_mut(
        &mut self,
        partition_id: Option<&str>,
    ) -> Result<RecoverableAmqpProducer<'_, RP>, azure_core::Error> {
        let producer_identifier = Some(
            self.options
                .identifier
                .clone()
                .unwrap_or(uuid::Uuid::new_v4().to_string()),
        );
        let requested_features = self.options.create_features();
        let retry_policy = RP::from(self.options.retry_options.clone());

        let partition_id = partition_id.and_then(|id| if id.is_empty() { None } else { Some(id) });

        match partition_id {
            Some(partition_id) => {
                if !self.producer_pool.contains_key(partition_id) {
                    let partition_options = self
                        .options
                        .get_publishing_options_or_default_for_partition(Some(partition_id));

                    let producer = self
                        .connection
                        .create_transport_producer::<RP>(
                            Some(partition_id.to_string()),
                            producer_identifier,
                            requested_features,
                            partition_options,
                            retry_policy,
                        )
                        .await?;
                    self.producer_pool
                        .insert(partition_id.to_string(), producer);
                }

                // This is safe because we just checked that the key exists.
                let producer = self.producer_pool.get_mut(partition_id).unwrap();
                let client = &mut self.connection.inner;
                Ok(RecoverableAmqpProducer::new(producer, client))
            }
            None => self.get_or_create_gateway_producer_mut().await,
        }
    }

    pub async fn create_batch(
        &mut self,
        options: CreateBatchOptions,
    ) -> Result<EventBatch, azure_core::Error> {
        let inner = self
            .get_or_create_gateway_producer_mut()
            .await?
            .create_batch(options)?;
        Ok(EventBatch { inner })
    }

    pub async fn send_event(
        &mut self,
        event: impl Into<Event>,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error> {
        self.send_events(std::iter::once(event.into()), options)
            .await
    }

    pub async fn send_events<E>(
        &mut self,
        events: E,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error>
    where
        E: IntoIterator<Item = Event>,
        E::IntoIter: ExactSizeIterator + Send,
    {
        match self.options.enable_idempotent_partitions {
            true => todo!(),
            false => {
                let partition_id = options.partition_id.as_deref();
                let mut producer = self.get_pooled_producer_mut(partition_id).await?;
                producer
                    .send(events.into_iter(), options)
                    .await
                    .map_err(IntoAzureCoreError::into_azure_core_error)
            }
        }
    }

    pub async fn send_batch(
        &mut self,
        batch: EventBatch,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error> {
        match self.options.enable_idempotent_partitions {
            true => todo!(),
            false => {
                let partition_id = options.partition_id.as_deref();
                let mut producer = self.get_pooled_producer_mut(partition_id).await?;
                producer
                    .send_batch(batch.inner, options)
                    .await
                    .map_err(IntoAzureCoreError::into_azure_core_error)
            }
        }
    }

    pub async fn get_event_hub_properties(
        &mut self,
    ) -> Result<EventHubProperties, azure_core::Error> {
        self.connection
            .get_properties(RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn get_partition_ids(
        &mut self,
    ) -> Result<Vec<String>, azure_core::Error> {
        self.connection
            .get_partition_ids(RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn get_partition_properties(
        &mut self,
        partition_id: &str,
    ) -> Result<PartitionProperties, azure_core::Error> {
        self.connection
            .get_partition_properties(partition_id, RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn close(self) -> Result<(), azure_core::Error> {
        let mut result = Ok(());
        for (_, producer) in self.producer_pool {
            let res = producer
                .dispose()
                .await
                .map_err(IntoAzureCoreError::into_azure_core_error);
            result = result.and(res);
        }

        let res = self.connection.close_if_owned().await;
        result.and(res)
    }
}
