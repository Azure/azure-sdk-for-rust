use std::marker::PhantomData;

use crate::{
    amqp::{
        amqp_client::AmqpClient,
        amqp_consumer::{AmqpConsumer, EventStream},
    },
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    BasicRetryPolicy, EventHubConnection, EventHubsRetryOptions,
};

use super::{EventHubConsumeClientOptions, EventPosition, ReadEventOptions};

pub struct EventHubConsumerClient<RP> {
    connection: EventHubConnection<AmqpClient>,
    // consumer_pool: HashMap<String, AmqpConsumer<RP>>,
    retry_policy_marker: PhantomData<RP>,
    options: EventHubConsumeClientOptions,
    consumer_group: String,
    invalidate_consumer_when_partition_is_stolen: bool,
}

impl EventHubConsumerClient<BasicRetryPolicy> {
    pub const DEFAULT_CONSUMER_GROUP_NAME: &'static str = "$Default";

    pub fn with_policy<P>() -> EventHubConsumerClientBuilder<P>
    where
        P: EventHubsRetryPolicy + Send,
    {
        EventHubConsumerClientBuilder {
            _retry_policy_marker: PhantomData,
        }
    }

    pub async fn new(
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumeClientOptions,
    ) -> Result<Self, azure_core::Error> {
        Self::with_policy()
            .new(
                consumer_group,
                connection_string,
                event_hub_name,
                client_options,
            )
            .await
    }

    pub fn with_connection(
        consumer_group: impl Into<String>,
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubConsumeClientOptions,
    ) -> Self {
        Self::with_policy().with_connection(consumer_group, connection, client_options)
    }
}

pub struct EventHubConsumerClientBuilder<RP> {
    _retry_policy_marker: PhantomData<RP>,
}

impl<RP> EventHubConsumerClientBuilder<RP> {
    pub async fn new(
        self,
        consumer_group: impl Into<String>,
        connection_string: impl Into<String>,
        event_hub_name: impl Into<Option<String>>,
        client_options: EventHubConsumeClientOptions,
    ) -> Result<EventHubConsumerClient<RP>, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let connection = EventHubConnection::new(
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
            invalidate_consumer_when_partition_is_stolen: false,
        })
    }

    pub fn with_connection(
        self,
        consumer_group: impl Into<String>,
        connection: &mut EventHubConnection<AmqpClient>,
        client_options: EventHubConsumeClientOptions,
    ) -> EventHubConsumerClient<RP> {
        EventHubConsumerClient {
            connection: connection.clone_as_shared(),
            retry_policy_marker: PhantomData,
            options: client_options,
            consumer_group: consumer_group.into(),
            invalidate_consumer_when_partition_is_stolen: false,
        }
    }
}

impl<RP> EventHubConsumerClient<RP>
where
    RP: EventHubsRetryPolicy + From<EventHubsRetryOptions> + Send + Unpin,
{
    pub async fn get_event_hub_properties(
        &mut self,
    ) -> Result<EventHubProperties, azure_core::Error> {
        self.connection
            .get_properties(RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn get_partition_ids(&mut self) -> Result<Vec<String>, azure_core::Error> {
        self.connection
            .get_partition_ids(RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn get_partition_properties(
        &mut self,
        partition_id: &str,
    ) -> Result<crate::PartitionProperties, azure_core::Error> {
        self.connection
            .get_partition_properties(partition_id, RP::from(self.options.retry_options.clone()))
            .await
    }

    pub async fn read_events_from_partition(
        &mut self,
        partition_id: &str,
        starting_position: EventPosition,
        read_event_options: ReadEventOptions,
    ) -> Result<EventStream<'_, AmqpConsumer<RP>>, azure_core::Error> {
        let consumer = self
            .connection
            .create_transport_consumer(
                &self.consumer_group,
                partition_id,
                self.options.identifier.clone(),
                starting_position,
                RP::from(self.options.retry_options.clone()),
                read_event_options.track_last_enqueued_event_properties,
                self.invalidate_consumer_when_partition_is_stolen,
                read_event_options.owner_level,
                Some(read_event_options.prefetch_count),
            )
            .await?;

        println!("Created consumer");

        let event_stream = EventStream::with_consumer(
            &mut self.connection.inner,
            consumer,
            read_event_options.cache_event_count,
            read_event_options.maximum_wait_time,
        );
        Ok(event_stream)
    }

    pub async fn close(self) -> Result<(), azure_core::Error> {
        self.connection.close_if_owned().await
    }
}
