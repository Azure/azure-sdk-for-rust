use std::{collections::VecDeque, marker::PhantomData};

use crate::{
    amqp::{amqp_client::AmqpClient, amqp_consumer::AmqpConsumer},
    consumer::EventPosition,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    util::IntoAzureCoreError,
    BasicRetryPolicy, EventHubConnection, EventHubsRetryOptions, ReceivedEvent,
};

use super::partition_receiver_options::PartitionReceiverOptions;

const DEFAULT_INVALIDATE_CONSUMER_WHEN_PARTITION_STOLEN: bool = false;

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

    pub async fn new(
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new(
                consumer_group,
                partition_id,
                event_position,
                connection_string,
                event_hub_name,
                options,
            )
            .await
    }
}

impl<RP> PartitionReceiverBuilder<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send,
{
    pub async fn new(
        self,
        consumer_group: &str,
        partition_id: &str,
        event_position: EventPosition,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        options: PartitionReceiverOptions,
    ) -> Result<PartitionReceiver<RP>, azure_core::Error> {
        let mut connection = EventHubConnection::new(
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
                DEFAULT_INVALIDATE_CONSUMER_WHEN_PARTITION_STOLEN,
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
    ) -> Result<VecDeque<ReceivedEvent>, azure_core::Error> {
        let mut buffer = VecDeque::with_capacity(max_event_count);
        self.inner_consumer
            .fill_buf_with_timeout(&mut buffer, self.options.maximum_receive_wait_time)
            .await
            .map_err(IntoAzureCoreError::into_azure_core_error)?;
        Ok(buffer)
    }
}
