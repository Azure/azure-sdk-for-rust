use std::{collections::VecDeque, marker::PhantomData, time::Duration as StdDuration};

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        amqp_consumer::{receive_event_batch, AmqpConsumer},
    },
    authorization::event_hub_token_credential::EventHubTokenCredential,
    consumer::EventPosition,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::IntoAzureCoreError,
    BasicRetryPolicy, EventHubConnection, EventHubsRetryOptions, ReceivedEvent,
};

use super::partition_receiver_options::PartitionReceiverOptions;

pub struct PartitionReceiver<RP> {
    connection: EventHubConnection<AmqpClient>,
    inner_consumer: AmqpConsumer<RP>,
    options: PartitionReceiverOptions,
}

pub struct PartitionReceiverBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl PartitionReceiver<BasicRetryPolicy> {
    pub fn with_policy<RP>() -> PartitionReceiverBuilder<RP>
    where
        RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
    {
        PartitionReceiverBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    pub async fn from_connection_string(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_connection_string(
                consumer_group,
                partition_id,
                event_position,
                connection_string,
                event_hub_name,
                options,
            )
            .await
    }

    pub async fn from_namespace_and_credential(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        fully_qualified_namespace: impl Into<String>,
        event_hub_name: impl Into<String>,
        credential: impl Into<EventHubTokenCredential>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .from_namespace_and_credential(
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
}

impl<RP> PartitionReceiverBuilder<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
{
    pub async fn from_connection_string(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        let mut connection = EventHubConnection::from_connection_string(
            connection_string.into(),
            event_hub_name.into(),
            options.connection_options.clone(),
        )
        .await?;

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
        let mut connection = EventHubConnection::from_namespace_and_credential(
            fully_qualified_namespace.into(),
            event_hub_name.into(),
            credential.into(),
            options.connection_options.clone(),
        )
        .await?;

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
    pub async fn recv_batch(
        &mut self,
        max_event_count: usize,
        max_wait_time: impl Into<Option<StdDuration>>,
    ) -> Result<impl Iterator<Item = ReceivedEvent> + ExactSizeIterator, azure_core::Error> {
        let mut buffer = VecDeque::with_capacity(max_event_count);
        let max_wait_time = max_wait_time.into();
        let max_wait_time = max_wait_time.map(|t| t.max(self.options.maximum_receive_wait_time));
        match receive_event_batch(
            &mut self.connection.inner,
            &mut self.inner_consumer,
            &mut buffer,
            max_wait_time,
        )
        .await
        {
            Some(result) => result.map_err(IntoAzureCoreError::into_azure_core_error)?,
            None => {
                // Return an empty buffer
            }
        }
        Ok(buffer.into_iter())
    }
}

impl<RP> PartitionReceiver<RP> {
    pub async fn close(self) -> Result<(), azure_core::Error> {
        self.inner_consumer
            .close()
            .await
            .map_err(IntoAzureCoreError::into_azure_core_error)?;
        self.connection.close_if_owned().await?;
        Ok(())
    }
}
