use futures_util::StreamExt;
use messaging_eventhubs::{
    consumer::{
        EventHubConsumeClientOptions, EventHubConsumerClient, EventPosition, ReadEventOptions,
    },
    EventHubsRetryOptions, MaxRetries,
};

mod common;

#[tokio::test]
async fn event_consumer_can_receive_infinite_events_from_partition_for_10_mins() {
    common::setup_dotenv();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

    let mut retry_options = EventHubsRetryOptions::default();
    retry_options.max_retries = MaxRetries(3);
    retry_options.try_timeout = std::time::Duration::from_secs(5);
    let mut options = EventHubConsumeClientOptions::default();
    options.retry_options = retry_options;

    let mut consumer =
        EventHubConsumerClient::new(consumer_group, connection_string, event_hub_name, options)
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
    stream.dispose().await.unwrap();

    consumer.close().await.unwrap();
}