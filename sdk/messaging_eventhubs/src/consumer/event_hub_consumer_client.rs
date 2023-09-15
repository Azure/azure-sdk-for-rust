use std::marker::PhantomData;

use crate::{
    amqp::amqp_consumer::EventStream,
    authorization::{event_hub_token_credential::EventHubTokenCredential, AzureNamedKeyCredential, AzureSasCredential},
    core::BasicRetryPolicy,
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    EventHubConnection, EventHubsRetryOptions,
};

use super::{EventHubConsumerClientOptions, EventPosition, ReadEventOptions};

/// A client responsible for reading [`crate::EventData`] from a specific Event Hub
/// as a member of a specific consumer group.
///
/// A consumer may be exclusive, which asserts ownership over associated partitions for the consumer
/// group to ensure that only one consumer from that group is reading the from the partition.
/// These exclusive consumers are sometimes referred to as "Epoch Consumers."
///
/// A consumer may also be non-exclusive, allowing multiple consumers from the same consumer
/// group to be actively reading events from a given partition. These non-exclusive consumers are
/// sometimes referred to as "Non-Epoch Consumers."
#[derive(Debug)]
pub struct EventHubConsumerClient<RP> {
    connection: EventHubConnection,
    retry_policy_marker: PhantomData<RP>,
    options: EventHubConsumerClientOptions,
    consumer_group: String,
}

impl EventHubConsumerClient<BasicRetryPolicy> {
    /// The name of the default consumer group in the Event Hubs service.
    pub const DEFAULT_CONSUMER_GROUP_NAME: &'static str = "$Default";

    /// Creates a new [`EventHubConsumerClientBuilder`] with a custom retry policy.
    pub fn with_policy<P>() -> EventHubConsumerClientBuilder<P>
    where
        P: EventHubsRetryPolicy + Send,
    {
        EventHubConsumerClientBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    /// Creates a new [`EventHubConsumerClient`] from a connection string.
    pub async fn new_from_connection_string(
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_connection_string(
                consumer_group,
                connection_string,
                event_hub_name,
                client_options,
            )
            .await
    }

    /// Creates a new [`EventHubConsumerClient`] from a connection string.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_connection_string` instead"
    )]
    pub async fn from_connection_string(
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_connection_string(consumer_group, connection_string, event_hub_name, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a credential.
    pub async fn new_from_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_credential(
                consumer_group,
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a credential.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_credential` instead"
    )]
    pub async fn from_namespace_and_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureNamedKeyCredential`].
    pub async fn new_from_named_key_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_named_key_credential(
                consumer_group,
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureNamedKeyCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_named_key_credential` instead"
    )]
    pub async fn from_namespace_and_named_key_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_named_key_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureSasCredential`].
    pub async fn new_from_sas_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_sas_credential(
                consumer_group,
                fully_qualified_namespace,
                event_hub_name,
                credential,
                client_options,
            )
            .await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureSasCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_sas_credential` instead"
    )]
    pub async fn from_namespace_and_sas_credential(
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_sas_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from an existing connection.
    pub fn with_connection(
        consumer_group: impl Into<String>,
        connection: &mut EventHubConnection,
        client_options: EventHubConsumerClientOptions,
    ) -> Self {
        Self::with_policy().with_connection(consumer_group, connection, client_options)
    }
}

/// A builder for creating an [`EventHubConsumerClient`].
#[derive(Debug)]
pub struct EventHubConsumerClientBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl<RP> EventHubConsumerClientBuilder<RP> {
    /// Creates a new [`EventHubConsumerClient`] from an existing connection.
    pub async fn new_from_connection_string(
        self,
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new_from_connection_string(
            connection_string.into(),
            event_hub_name.into(),
            client_options.connection_options.clone(),
        )
        .await?;
        Ok(EventHubConsumerClient {
            connection,
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
        })
    }

    /// Creates a new [`EventHubConsumerClient`] from an existing connection.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_connection_string` instead"
    )]
    pub async fn from_connection_string(
        self,
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        self.new_from_connection_string(consumer_group, connection_string, event_hub_name, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and credential.
    pub async fn new_from_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new_from_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential.into(),
            client_options.connection_options.clone(),
        )
        .await?;
        Ok(EventHubConsumerClient {
            connection,
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
        })
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and credential.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_credential` instead"
    )]
    pub async fn from_namespace_and_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        self.new_from_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureNamedKeyCredential`].
    pub async fn new_from_named_key_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new_from_named_key_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential,
            client_options.connection_options.clone(),
        )
        .await?;
        Ok(EventHubConsumerClient {
            connection,
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
        })
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureNamedKeyCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_named_key_credential` instead"
    )]
    pub async fn from_namespace_and_named_key_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureNamedKeyCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        self.new_from_named_key_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureSasCredential`].
    pub async fn new_from_sas_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new_from_sas_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential,
            client_options.connection_options.clone(),
        )
        .await?;
        Ok(EventHubConsumerClient {
            connection,
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
        })
    }

    /// Creates a new [`EventHubConsumerClient`] from a namespace and a [`AzureSasCredential`].
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_sas_credential` instead"
    )]
    pub async fn from_namespace_and_sas_credential(
        self,
        consumer_group: impl Into<String>,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: AzureSasCredential,
        client_options: EventHubConsumerClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        self.new_from_sas_credential(consumer_group, fully_qualified_namespace, event_hub_name, credential, client_options).await
    }

    /// Creates a new [`EventHubConsumerClient`] from an existing [`EventHubConnection`].
    pub fn with_connection(
        self,
        consumer_group: impl Into<String>,
        connection: &mut EventHubConnection,
        client_options: EventHubConsumerClientOptions,
    ) -> EventHubConsumerClient<RP> {
        EventHubConsumerClient {
            connection: connection.clone_as_shared(),
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
        }
    }
}

impl<RP> EventHubConsumerClient<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send + Unpin,
{
    /// Retrieves information about the Event Hub instance the client is associated with, including
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

    /// Retrieves information about a specific partition for an Event Hub, including elements that describe the available
    /// events in the partition event stream.
    pub async fn get_partition_properties(
        &mut self,
        partition_id: &str,
    ) -> Result<crate::PartitionProperties, azure_core::Error> {
        self.connection
            .get_partition_properties(partition_id, RP::from(self.options.retry_options.clone()))
            .await
    }

    /// Reads events from the requested partition as an `Stream`, allowing events to be `.await`ed
    /// as they become available on the partition.
    pub async fn read_events_from_partition(
        &mut self,
        partition_id: &str,
        starting_position: EventPosition,
        read_event_options: ReadEventOptions,
    ) -> Result<EventStream<'_, RP>, azure_core::Error> {
        let consumer = self
            .connection
            .create_transport_consumer(
                &self.consumer_group,
                partition_id,
                self.options.identifier.clone(),
                starting_position,
                RP::from(self.options.retry_options.clone()),
                read_event_options.track_last_enqueued_event_properties,
                read_event_options.owner_level,
                Some(read_event_options.prefetch_count),
            )
            .await?;

        let event_stream = EventStream::with_consumer(
            &mut self.connection.inner,
            consumer,
        );
        Ok(event_stream)
    }

    /// Reads events from all partitions as an `Stream`, allowing events to be `.await`ed
    /// as they become available on the partition.
    pub async fn read_events(
        &mut self,
        start_reading_at_earliest_event: bool,
        read_event_options: ReadEventOptions,
    ) -> Result<EventStream<'_, RP>, azure_core::Error>
    where
        RP: 'static,
    {
        let starting_position = match start_reading_at_earliest_event {
            true => EventPosition::earliest(),
            false => EventPosition::latest(),
        };

        let retry_policy = RP::from(self.options.retry_options.clone());
        let partitions = self.connection.get_partition_ids(retry_policy).await?;

        // Create one consumer per partition
        let mut consumers = Vec::with_capacity(partitions.len());
        for partition in partitions {
            let retry_policy = RP::from(self.options.retry_options.clone());
            let consumer = self
                .connection
                .create_transport_consumer(
                    &self.consumer_group,
                    &partition,
                    self.options.identifier.clone(),
                    starting_position.clone(),
                    retry_policy,
                    read_event_options.track_last_enqueued_event_properties,
                    read_event_options.owner_level,
                    Some(read_event_options.prefetch_count),
                )
                .await?;
            consumers.push(consumer);
        }

        // Create an event stream that will read from all consumers
        let retry_policy = RP::from(self.options.retry_options.clone());
        let event_stream = EventStream::with_multiple_consumers(
            &mut self.connection.inner,
            consumers,
            retry_policy,
        );
        Ok(event_stream)
    }

    /// Closes the consumer.
    ///
    /// The underlying connection will be closed if the consumer is the last one using it.
    pub async fn close(self) -> Result<(), azure_core::Error> {
        self.connection.close_if_owned().await
    }
}
