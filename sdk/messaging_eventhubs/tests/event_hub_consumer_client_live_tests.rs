#![cfg(all(test, feature = "test_e2e"))]

use azeventhubs::{
    consumer::{
        EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions,
    },
    EventHubsRetryOptions, MaxRetries,
};
use futures_util::StreamExt;

#[macro_use]
mod cfg;

mod common;

cfg_not_wasm32! {
    /// Send `num` events to the event hub to make sure that there are enough events to test the
    /// consumer client.
    ///
    /// The testing event hub is configured with a retention period of only 1 hour, so this is
    /// necessary to make sure that there are events to read.
    async fn prepare_events_on_eventhubs(num: usize, partition: Option<&str>) {
        use azeventhubs::producer::{SendEventOptions, EventHubProducerClientOptions, EventHubProducerClient};

        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, None, options)
                .await
                .unwrap();

        for i in 0..num {
            let event = format!("Events {} prepared for consumer client tests", i);
            let options = match partition {
                Some(p) => SendEventOptions::new().with_partition_id(p),
                None => SendEventOptions::default(),
            };
            producer_client
                .send_event(event, options)
                .await
                .unwrap();
        }

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn event_consumer_can_receive_events_from_partition() {
        common::setup_dotenv();

        prepare_events_on_eventhubs(30, Some("0")).await;

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

        let mut retry_options = EventHubsRetryOptions::default();
        retry_options.max_retries = MaxRetries::try_from(3).unwrap();
        retry_options.try_timeout = std::time::Duration::from_secs(5);
        let mut options = EventHubConsumerClientOptions::default();
        options.retry_options = retry_options;

        let mut consumer = EventHubConsumerClient::new_from_connection_string(
            consumer_group,
            connection_string,
            event_hub_name,
            options,
        )
        .await
        .unwrap();

        let partition_id = "0";
        let starting_position = EventPosition::earliest();
        let options = ReadEventOptions::default();

        let mut stream = consumer
            .read_events_from_partition(partition_id, starting_position, options)
            .await
            .unwrap();

        let mut counter = 0;
        while let Some(event) = stream.next().await {
            let event = event.unwrap();
            let body = event.body().unwrap();
            let value = std::str::from_utf8(body).unwrap();
            log::info!("{:?}", value);

            log::info!("counter: {}", counter);
            counter += 1;
            if counter > 30 {
                break;
            }
        }
        stream.close().await.unwrap();

        consumer.close().await.unwrap();
    }

    #[tokio::test]
    async fn event_consumer_can_receive_events_from_all_partitions() {
        common::setup_dotenv();

        prepare_events_on_eventhubs(30, None).await;

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

        let mut retry_options = EventHubsRetryOptions::default();
        retry_options.max_retries = MaxRetries::try_from(3).unwrap();
        retry_options.try_timeout = std::time::Duration::from_secs(5);
        let mut options = EventHubConsumerClientOptions::default();
        options.retry_options = retry_options;

        let mut consumer = EventHubConsumerClient::new_from_connection_string(
            consumer_group,
            connection_string,
            event_hub_name,
            options,
        )
        .await
        .unwrap();

        let options = ReadEventOptions::default();
        // Some large number that will never be reached but not too large that will take too much
        // memory
        let mut stream = consumer
            .read_events(true, options)
            .await
            .unwrap();

        let mut counter = 0;
        while let Some(Ok(event)) = stream.next().await {
            let body = event.body().unwrap();
            let value = std::str::from_utf8(body).unwrap();
            log::info!("{:?}", value);

            log::info!("counter: {}", counter);
            counter += 1;
            if counter > 30 {
                break;
            }
        }
        stream.close().await.unwrap();

        consumer.close().await.unwrap();
    }

    #[tokio::test]
    async fn spawn_event_consumer_and_receive_events() {
        common::setup_dotenv();

        prepare_events_on_eventhubs(30, None).await;

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

        let mut retry_options = EventHubsRetryOptions::default();
        retry_options.max_retries = MaxRetries::try_from(3).unwrap();
        retry_options.try_timeout = std::time::Duration::from_secs(5);
        let mut options = EventHubConsumerClientOptions::default();
        options.retry_options = retry_options;

        let handle = tokio::spawn(async move {
            let mut consumer = EventHubConsumerClient::new_from_connection_string(
                consumer_group,
                connection_string,
                event_hub_name,
                options,
            )
            .await
            .unwrap();

            let options = ReadEventOptions::default();
            let mut stream = consumer
                .read_events(true, options)
                .await
                .unwrap();

            let mut counter = 0;
            while let Some(Ok(event)) = stream.next().await {
                let body = event.body().unwrap();
                let value = std::str::from_utf8(body).unwrap();
                log::info!("{:?}", value);

                log::info!("counter: {}", counter);
                counter += 1;
                if counter > 30 {
                    break;
                }
            }
            stream.close().await.unwrap();

            consumer.close().await.unwrap();
        });

        handle.await.unwrap();
    }
}
