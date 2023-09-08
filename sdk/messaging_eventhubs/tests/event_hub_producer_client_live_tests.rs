#![cfg(all(test, feature = "test_e2e"))]

use azeventhubs::{
    producer::{
        CreateBatchOptions, EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions,
    },
    EventHubConnection, EventHubConnectionOptions,
};

#[macro_use]
mod cfg;

mod common;

cfg_not_wasm32! {
    #[tokio::test]
    async fn producer_client_can_connect_to_event_hubs_using_full_connection_string_over_tcp() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let options = EventHubProducerClientOptions::default();
        let producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, None, options)
                .await
                .unwrap();
        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_connect_using_named_key_credential() {
        use azeventhubs::authorization::{AzureNamedKeyCredential};

        common::setup_dotenv();

        let namespace = std::env::var("EVENT_HUBS_NAMESPACE").unwrap();
        let fqn = format!("{}.servicebus.windows.net", namespace);
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let key_name = std::env::var("EVENT_HUBS_SHARED_ACCESS_KEY_NAME").unwrap();
        let key = std::env::var("EVENT_HUBS_SHARED_ACCESS_KEY").unwrap();
        let options = EventHubProducerClientOptions::default();
        let named_key_credential = AzureNamedKeyCredential::new(key_name, key);

        let mut producer_client = EventHubProducerClient::new_from_named_key_credential(
            fqn,
            event_hub_name,
            named_key_credential,
            options,
        ).await.unwrap();

        let event = "Hello, world to partition 0";
        let options = SendEventOptions::new().with_partition_id("0");
        producer_client.send_event(event, options).await.unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_connect_using_azure_identity_credential() {
        use azure_identity::DefaultAzureCredential;

        common::setup_dotenv();

        let namespace = std::env::var("EVENT_HUBS_NAMESPACE").unwrap();
        let fqn = format!("{}.servicebus.windows.net", namespace);
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let options = EventHubProducerClientOptions::default();
        let default_credential = DefaultAzureCredential::default();

        let mut producer_client = EventHubProducerClient::new_from_credential(
            fqn,
            event_hub_name,
            default_credential,
            options,
        ).await.unwrap();

        let event = "test connect using azure identity";
        let options = SendEventOptions::new().with_partition_id("0");
        producer_client.send_event(event, options).await.unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn close_producer_client_does_not_close_shared_connection() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let connection_options = EventHubConnectionOptions::default();
        let mut connection =
            EventHubConnection::new_from_connection_string(connection_string, None, connection_options)
                .await
                .unwrap();

        let client_options = EventHubProducerClientOptions::default();
        let producer_client_1 =
            EventHubProducerClient::with_connection(&mut connection, client_options.clone());
        let producer_client_2 =
            EventHubProducerClient::with_connection(&mut connection, client_options);
        producer_client_1.close().await.unwrap();
        producer_client_2.close().await.unwrap();

        assert_eq!(connection.is_closed(), false);
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_send_an_event_to_a_partition() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, None, options)
                .await
                .unwrap();

        let event = "Hello, world to partition 0";
        let options = SendEventOptions::new().with_partition_id("0");
        producer_client.send_event(event, options).await.unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_send_without_specifying_partition_id() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, None, options)
                .await
                .unwrap();

        let event = "Hello, world to a random partition";
        producer_client
            .send_event(event, SendEventOptions::default())
            .await
            .unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_create_and_send_event_batch() {
        common::setup_dotenv();

        // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, None, options)
                .await
                .unwrap();

        let options = CreateBatchOptions::new();
        let mut event_batch = producer_client.create_batch(options).await.unwrap();

        let event = "Hello, world to a random partition again and again";
        while let Ok(_) = event_batch.try_add(event) {}
        log::info!("Batch size: {}", event_batch.size_in_bytes());

        producer_client
            .send_batch(event_batch, SendEventOptions::default())
            .await
            .unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_get_event_hub_properties() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, event_hub_name, options)
                .await
                .unwrap();

        let _properties = producer_client.get_event_hub_properties().await.unwrap();

        producer_client.close().await.unwrap();
    }

    #[tokio::test]
    async fn producer_client_can_get_partition_properties() {
        common::setup_dotenv();

        let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
        let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
        let options = EventHubProducerClientOptions::default();
        let mut producer_client =
            EventHubProducerClient::new_from_connection_string(connection_string, event_hub_name, options)
                .await
                .unwrap();

        let _properties = producer_client.get_partition_properties("0").await.unwrap();

        producer_client.close().await.unwrap();
    }
}
