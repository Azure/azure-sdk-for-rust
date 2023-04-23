#![cfg(all(test, feature = "test_e2e"))]

use futures_util::StreamExt;
use messaging_eventhubs::{
    consumer::{
        EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions,
    },
    EventHubsRetryOptions, MaxRetries,
};

#[macro_use]
mod cfg;

mod common;

cfg_not_wasm32! {
    #[tokio::test]
    async fn event_consumer_can_receive_events_from_partition() {
        common::setup_dotenv();

        // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

        let mut retry_options = EventHubsRetryOptions::default();
        retry_options.max_retries = MaxRetries(3);
        retry_options.try_timeout = std::time::Duration::from_secs(5);
        let mut options = EventHubConsumerClientOptions::default();
        options.retry_options = retry_options;

        let mut consumer = EventHubConsumerClient::from_connection_string(
            consumer_group,
            connection_string,
            event_hub_name,
            options,
        )
        .await
        .unwrap();

        let partition_id = "0";
        let starting_position = EventPosition::earliest();
        let mut options = ReadEventOptions::default();
        options.cache_event_count = 3;

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

        // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

        let mut retry_options = EventHubsRetryOptions::default();
        retry_options.max_retries = MaxRetries(3);
        retry_options.try_timeout = std::time::Duration::from_secs(5);
        let mut options = EventHubConsumerClientOptions::default();
        options.retry_options = retry_options;

        let mut consumer = EventHubConsumerClient::from_connection_string(
            consumer_group,
            connection_string,
            event_hub_name,
            options,
        )
        .await
        .unwrap();

        let mut options = ReadEventOptions::default();
        options.cache_event_count = 3;
        let mut stream = consumer
            .read_events(true, Default::default())
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
}
