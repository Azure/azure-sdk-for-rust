use std::{collections::VecDeque, marker::PhantomData, time::Duration as StdDuration};

use crate::{
    amqp::amqp_consumer::{single::receive_event_batch, AmqpConsumer},
    authorization::event_hub_token_credential::EventHubTokenCredential,
    consumer::EventPosition,
    core::BasicRetryPolicy,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    EventHubConnection, EventHubsRetryOptions, ReceivedEventData,
};

use super::partition_receiver_options::PartitionReceiverOptions;

/// Allows reading events from a specific partition of an Event Hub, and in the context of a
/// specific consumer group, to be read with a greater level of control over communication with the
/// Event Hubs service than is offered by other event consumers.
#[derive(Debug)]
pub struct PartitionReceiver<RP> {
    connection: EventHubConnection,
    inner_consumer: AmqpConsumer<RP>,
    options: PartitionReceiverOptions,
}

/// A builder for a [`PartitionReceiver`].
#[derive(Debug)]
pub struct PartitionReceiverBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl PartitionReceiver<BasicRetryPolicy> {
    /// Creates a new [`PartitionReceiverBuilder`] with a custom retry policy.
    pub fn with_policy<RP>() -> PartitionReceiverBuilder<RP>
    where
        RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
    {
        PartitionReceiverBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    /// Creates a new [`PartitionReceiver`] from a connection string.
    pub async fn new_from_connection_string(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_connection_string(
                consumer_group,
                partition_id,
                event_position,
                connection_string,
                event_hub_name,
                options,
            )
            .await
    }

    /// Creates a new [`PartitionReceiver`] from a connection string.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_connection_string` instead"
    )]
    pub async fn from_connection_string(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_connection_string(
            consumer_group,
            partition_id,
            event_position,
            connection_string,
            event_hub_name,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from a namespace and a credential.
    pub async fn new_from_credential(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new_from_credential(
                consumer_group,
                partition_id,
                event_position,
                fully_qualified_namespace,
                event_hub_name,
                credential,
                options,
            )
            .await
    }

    /// Creates a new [`PartitionReceiver`] from a namespace and a credential.
    #[deprecated(since = "0.14.1", note = "Please use `new_from_credential` instead")]
    pub async fn from_namespace_and_credential(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::new_from_credential(
            consumer_group,
            partition_id,
            event_position,
            fully_qualified_namespace,
            event_hub_name,
            credential,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from an existing [`EventHubConnection`].
    pub async fn with_conneciton(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection: EventHubConnection,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .with_connection(
                consumer_group,
                partition_id,
                event_position,
                connection,
                options,
            )
            .await
    }
}

impl<RP> PartitionReceiverBuilder<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
{
    /// Creates a new [`PartitionReceiver`] from a connection string.
    pub async fn new_from_connection_string(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        let connection = EventHubConnection::new_from_connection_string(
            connection_string.into(),
            event_hub_name.into(),
            options.connection_options.clone(),
        )
        .await?;

        self.with_connection(
            consumer_group,
            partition_id,
            event_position,
            connection,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from a connection string.
    #[deprecated(
        since = "0.14.1",
        note = "Please use `new_from_connection_string` instead"
    )]
    pub async fn from_connection_string(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        self.new_from_connection_string(
            consumer_group,
            partition_id,
            event_position,
            connection_string,
            event_hub_name,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from a namespace and a credential.
    #[allow(clippy::too_many_arguments)] // TODO: how to reduce the number of arguments?
    pub async fn new_from_credential(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        let connection = EventHubConnection::new_from_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential.into(),
            options.connection_options.clone(),
        )
        .await?;

        self.with_connection(
            consumer_group,
            partition_id,
            event_position,
            connection,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from a namespace and a credential.
    #[allow(clippy::too_many_arguments)] // TODO: how to reduce the number of arguments?
    #[deprecated(since = "0.14.1", note = "Please use `new_from_credential` instead")]
    pub async fn from_namespace_and_credential(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        self.new_from_credential(
            consumer_group,
            partition_id,
            event_position,
            fully_qualified_namespace,
            event_hub_name,
            credential,
            options,
        )
        .await
    }

    /// Creates a new [`PartitionReceiver`] from an existing [`EventHubConnection`].
    pub async fn with_connection(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        mut connection: EventHubConnection,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        let consumer_identifier = options.identifier.clone();
        let retry_policy = RP::from(options.retry_options.clone());
        let inner_consumer = connection
            .create_transport_consumer(
                consumer_group,
                partition_id,
                consumer_identifier,
                event_position,
                retry_policy,
                options.track_last_enqueued_event_properties,
                options.owner_level,
                Some(options.prefetch_count),
            )
            .await?;

        Ok(PartitionReceiver {
            connection,
            inner_consumer,
            options,
        })
    }
}

impl<RP> PartitionReceiver<RP>
where
    RP: EventHubsRetryPolicy + Send,
{
    /// Receives a batch of events from the Event Hub partition.
    pub async fn recv_batch(
        &mut self,
        max_event_count: usize,
        max_wait_time: impl Into<Option<StdDuration>>,
    ) -> Result<impl Iterator<Item = ReceivedEventData> + ExactSizeIterator, azure_core::Error>
    {
        let mut buffer = VecDeque::with_capacity(max_event_count);
        let max_wait_time = max_wait_time.into();
        let max_wait_time = max_wait_time.map(|t| t.max(self.options.maximum_receive_wait_time));
        receive_event_batch(
            &mut self.connection.inner,
            &mut self.inner_consumer,
            &mut buffer,
            max_wait_time,
        )
        .await?;
        Ok(buffer.into_iter())
    }
}

impl<RP> PartitionReceiver<RP> {
    /// Closes the [`PartitionReceiver`].
    pub async fn close(self) -> Result<(), azure_core::Error> {
        self.inner_consumer.close().await?;
        self.connection.close_if_owned().await?;
        Ok(())
    }
}
