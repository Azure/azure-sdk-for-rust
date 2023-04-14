use futures_util::StreamExt;
use messaging_eventhubs::{consumer::{EventHubConsumerClient, EventPosition, ReadEventOptions, EventHubConsumeClientOptions}, EventHubsRetryOptions, MaxRetries};

mod common;

#[tokio::test]
async fn event_consumer_can_receive_from_partition() {
    common::setup_dotenv();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;

    let mut retry_options = EventHubsRetryOptions::default();
    retry_options.max_retries = MaxRetries(2);
    retry_options.try_timeout = std::time::Duration::from_secs(5);
    let mut options = EventHubConsumeClientOptions::default();
    options.retry_options = retry_options;

    let mut consumer = EventHubConsumerClient::new(
        consumer_group,
        connection_string,
        event_hub_name,
        options,
    ).await.unwrap();

    let partition_id = "0";
    let starting_position = EventPosition::earliest();
    let mut options = ReadEventOptions::default();
    options.cache_event_count = Some(3);
    options.maximum_wait_time = Some(std::time::Duration::from_secs(1));

    let mut stream = consumer.read_events_from_partition(partition_id, starting_position, options).await.unwrap();

    while let Some(event) = stream.next().await {
        println!("{:?}", event.unwrap().body().unwrap());
    }

    consumer.close().await.unwrap();
}
