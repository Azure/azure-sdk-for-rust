use std::time::Duration;

use messaging_eventhubs::producer::{
    event_hub_producer_client::EventHubProducerClient,
    event_hub_producer_client_options::EventHubProducerClientOptions,
    send_event_options::SendEventOptions,
};

mod common;

async fn producer_client_can_send_before_and_after_sleeping(
    duration: Duration,
    partition_id: &str,
) {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubProducerClientOptions::default();
    let mut producer_client =
        EventHubProducerClient::new(connection_string, event_hub_name, options)
            .await
            .unwrap();

    println!("sending first event");
    let options = SendEventOptions::default().with_partition_id(partition_id);
    producer_client
        .send_event("first event", options)
        .await
        .unwrap();
    println!("sent first event");

    println!("sleeping for {} seconds", duration.as_secs());
    tokio::time::sleep(duration).await;

    println!("sending second event");
    let options = SendEventOptions::default().with_partition_id(partition_id);
    producer_client
        .send_event("second event", options)
        .await
        .unwrap();
    println!("sent second event");

    producer_client.close().await.unwrap();
}

// #[tokio::test]
// async fn producer_client_can_send_after_sleeping_for_10_mins() {
//     producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60*10), "0").await;
// }

// #[tokio::test]
// async fn producer_client_can_send_after_sleeping_for_20_mins() {
//     producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60*20), "0").await;
// }

// #[tokio::test]
// async fn producer_client_can_send_after_sleeping_for_30_mins() {
//     producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60*30), "0").await;
// }

#[tokio::test]
async fn producer_client_can_send_after_sleeping() {
    producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60 * 10), "0").await;
    producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60 * 20), "0").await;
    producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60 * 30), "0").await;
    producer_client_can_send_before_and_after_sleeping(Duration::from_secs(60 * 40), "0").await;
}
