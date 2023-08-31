use std::{collections::HashMap, marker::PhantomData};

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        amqp_producer::{AmqpProducer, RecoverableAmqpProducer},
    },
    authorization::{event_hub_token_credential::EventHubTokenCredential, AzureNamedKeyCredential, AzureSasCredential},
    core::{BasicRetryPolicy, TransportProducer},
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    EventData, EventHubConnection, EventHubsRetryOptions, PartitionProperties,
};

use super::{
    create_batch_options::CreateBatchOptions, event_data_batch::EventDataBatch,
    event_hub_producer_client_options::EventHubProducerClientOptions,
    send_event_options::SendEventOptions,
};

/// The minimum
pub const MINIMUM_BATCH_SIZE_LIMIT_IN_BYTES: u64 = 24;

/// A client responsible for publishing [`EventData`] to a specific Event Hub, grouped together in
/// batches.  Depending on the options specified when sending, events may be automatically assigned
/// an available partition or may request a specific partition.
///
/// The [`EventHubProducerClient`] publishes immediately, ensuring a deterministic outcome for each
/// send operation, though requires that callers own the responsibility of building and managing
/// batches.
#[derive(Debug)]
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
    /// Creates a new client with a custom retry policy.
    pub fn with_policy<P>() -> EventHubProducerClientBuilder<P>
    where
        P: EventHubsRetryPolicy + Send,
    {
        EventHubProducerClientBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    /// Creates a [`EventHubProducerClient`] using a connection string.
    pub async fn from_connection_string(
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_connection_string(connection_string, event_hub_name, client_options)
            .await
    }

    /// Creates a [`EventHubProducerClient`] using a namespace and a credential.
    pub async fn from_namespace_and_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_namespace_and_credential(
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a [`EventHubProducerClient`] using a namespace and a [`AzureNamedKeyCredential`].
    pub async fn from_namespace_and_named_key_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_namespace_and_named_key_credential(
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a [`EventHubProducerClient`] using a namespace and a [`AzureSasCredential`].
    pub async fn from_namespace_and_sas_credential(
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubProducerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_namespace_and_sas_credential(
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a [`EventHubProducerClient`] using a [`EventHubConnection`].
    pub fn with_connection(
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubProducerClientOptions,
    ) -> Self {
        Self::with_policy().with_connection(connection, client_options)
    }
}

/// A builder for creating a [`EventHubProducerClient`].
///
/// This currently is only used for specifying a custom retry policy.
#[derive(Debug)]
pub struct EventHubProducerClientBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl<RP> EventHubProducerClientBuilder<RP> {
    /// Creates a [`EventHubProducerClient`] using a connection string.
    pub async fn from_connection_string(
        self,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<EventHubProducerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::from_connection_string(
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

    /// Creates a [`EventHubProducerClient`] using a namespace and a credential.
    pub async fn from_namespace_and_credential(
        self,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubProducerClientOptions,
    ) -> Result<EventHubProducerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::from_namespace_and_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential.into(),
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

    /// Creates a [`EventHubProducerClient`] using a namespace and a [`AzureNamedKeyCredential`].
    pub async fn from_namespace_and_named_key_credential(
        self,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubProducerClientOptions,
    ) -> Result<EventHubProducerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::from_namespace_and_named_key_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential,
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

    /// Creates a [`EventHubProducerClient`] using a namespace and a [`AzureSasCredential`].
    pub async fn from_namespace_and_sas_credential(
        self,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubProducerClientOptions,
    ) -> Result<EventHubProducerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::from_namespace_and_sas_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential,
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

    /// Creates a [`EventHubProducerClient`] using a [`EventHubConnection`].
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
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
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
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
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

    /// Creates a new [`EventDataBatch`] with the given options.
    pub async fn create_batch(
        &mut self,
        options: CreateBatchOptions,
    ) -> Result<EventDataBatch, azure_core::Error> {
        let inner = self
            .get_or_create_gateway_producer_mut()
            .await?
            .create_batch(options)?;
        Ok(EventDataBatch { inner })
    }

    /// Sends a single event to the Event Hub.
    pub async fn send_event(
        &mut self,
        event: impl Into<EventData>,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error> {
        self.send_events(std::iter::once(event.into()), options)
            .await
    }

    /// Sends a set of events to the Event Hub.
    pub async fn send_events<E>(
        &mut self,
        events: E,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error>
    where
        E: IntoIterator<Item = EventData>,
        E::IntoIter: ExactSizeIterator + Send,
    {
        let partition_id = options.partition_id();
        let mut producer = self.get_pooled_producer_mut(partition_id).await?;
        producer
            .send(events.into_iter(), options)
            .await
            .map_err(Into::into)
    }

    /// Sends a batch of events to the Event Hub.
    pub async fn send_batch(
        &mut self,
        batch: EventDataBatch,
        options: SendEventOptions,
    ) -> Result<(), azure_core::Error> {
        let partition_id = options.partition_id();
        let mut producer = self.get_pooled_producer_mut(partition_id).await?;
        producer
            .send_batch(batch.inner, options)
            .await
            .map_err(Into::into)
    }

    /// Retrieves information about the Event Hub that the connection is associated with, including
    /// the number of partitions present and their identifiers.
    pub async fn get_event_hub_properties(
        &mut self,
    ) -> Result<EventHubProperties, azure_core::Error> {
        self.connection
            .get_properties(RP::from(self.options.retry_options.clone()))
            .await
    }

    /// Retrieves the set of identifiers for the partitions of an Event Hub.
    pub async fn get_partition_ids(&mut self) -> Result<Vec<String>, azure_core::Error> {
        self.connection
            .get_partition_ids(RP::from(self.options.retry_options.clone()))
            .await
    }

    /// Retrieves information about a specific partition for an Event Hub, including elements that
    /// describe the available events in the partition event stream.
    pub async fn get_partition_properties(
        &mut self,
        partition_id: &str,
    ) -> Result<PartitionProperties, azure_core::Error> {
        self.connection
            .get_partition_properties(partition_id, RP::from(self.options.retry_options.clone()))
            .await
    }

    /// Performs the task needed to clean up resources used by the [`EventHubProducerClient`],
    /// including ensuring that the client itself has been closed.
    ///
    /// This won't close the underlying connection if the connection was shared among multiple
    /// clients.
    pub async fn close(self) -> Result<(), azure_core::Error> {
        let mut result = Ok(());
        for (_, producer) in self.producer_pool {
            let res = producer
                .close()
                .await
                .map_err(Into::into);
            result = result.and(res);
        }

        let res = self.connection.close_if_owned().await;
        result.and(res)
    }
}
