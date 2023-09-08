#![cfg(all(test, feature = "test_e2e"))]

use std::time::Duration;

use azeventhubs::producer::{
    EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions,
};

#[macro_use]
mod cfg;

mod common;

cfg_not_wasm32! {
    #[tokio::test]
    #[ignore]
    async fn producer_client_send_one_event_per_minute_for_10_mins() {
        common::setup_dotenv();

        // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let partition_id = "0";
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, event_hub_name, options)
                .await
                .unwrap();

        // Send an event to ensure the link to the partition is established.
        let options = SendEventOptions::default().with_partition_id(partition_id);

        // Send an event every minute for 10 minutes.
        for i in 0..10 {
            let duration = Duration::from_secs(60);
            println!("iteration {}", i);
            producer_client
                .send_event(format!("iteration {}", i), options.clone())
                .await
                .unwrap();
            println!("sleeping for {} seconds", duration.as_secs());
            tokio::time::sleep(duration).await;
        }

        producer_client.close().await.unwrap();
    }

    /// This tests whether the the producer client can recover after link/session/connection
    /// are forced to close due to inactivity (for over 30 minutes).
    #[tokio::test]
    #[ignore]
    async fn producer_client_can_recover_and_send_after_sleeping_for_40_mins() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let partition_id = "0";
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, event_hub_name, options)
                .await
                .unwrap();

        // Send an event to ensure the link to the partition is established.
        let options = SendEventOptions::default().with_partition_id(partition_id);
        producer_client
            .send_event("first event", options)
            .await
            .unwrap();

        // Sleep for 40 minutes to force the link to close due to inactivity.
        for _ in 0..8 {
            let duration = Duration::from_secs(5 * 60);
            println!("sleeping for {} seconds", duration.as_secs());
            tokio::time::sleep(duration).await;
        }

        let options = SendEventOptions::default().with_partition_id(partition_id);
        producer_client
            .send_event("second event", options)
            .await
            .unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn producer_client_can_recover_and_get_properties_after_idling_40_mins() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, event_hub_name, options)
                .await
                .unwrap();
        let properties = producer_client.get_event_hub_properties().await.unwrap();
        println!("properties: {:?}", properties);

        // Sleep for 40 minutes to force the link to close due to inactivity.
        for i in 0..40 {
            let duration = Duration::from_secs(60);
            tokio::time::sleep(duration).await;
            println!("iteration {}", i);
        }

        let properties = producer_client.get_event_hub_properties().await.unwrap();
        println!("properties: {:?}", properties);

        producer_client.close().await.unwrap();
    }
}
