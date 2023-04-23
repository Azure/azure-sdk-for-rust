use messaging_eventhubs::{
    consumer::{EventHubConsumerClient, EventPosition}, primitives::PartitionReceiver,
};

mod common;

#[tokio::test]
async fn partition_receiver_can_receive_events() {
    common::setup_dotenv();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;
    let partition_id = "0";

    let mut receiver = PartitionReceiver::from_connection_string(
        consumer_group,
        partition_id,
        EventPosition::earliest(),
        connection_string,
        event_hub_name,
        Default::default(),
    )
    .await
    .unwrap();

    let batch = receiver
        .recv_batch(3, std::time::Duration::from_secs(10))
        .await
        .unwrap();

    for event in batch {
        let body = event.body().unwrap();
        let value = std::str::from_utf8(body).unwrap();
        log::info!("{:?}", value);
    }

    receiver.close().await.unwrap();
}
