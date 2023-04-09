use messaging_eventhubs::{
    producer::{
        event_hub_producer_client::EventHubProducerClient,
        event_hub_producer_client_options::EventHubProducerClientOptions,
        send_event_options::SendEventOptions, create_batch_options::CreateBatchOptions,
    },
    EventHubConnection, EventHubConnectionOptions,
};

mod common;

#[tokio::test]
async fn producer_client_can_connect_to_event_hubs_using_full_connection_string_over_tcp() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubProducerClientOptions::default();
    let producer_client = EventHubProducerClient::new(connection_string, None, options)
        .await
        .unwrap();
    producer_client.close().await.unwrap();
}

#[tokio::test]
async fn close_producer_client_does_not_close_shared_connection() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let connection_options = EventHubConnectionOptions::default();
    let mut connection = EventHubConnection::new(connection_string, None, connection_options)
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
    let mut producer_client = EventHubProducerClient::new(connection_string, None, options)
        .await
        .unwrap();

    let event = "Hello, world!";
    let options = SendEventOptions::new().with_partition_id("0");
    producer_client.send_event(event, options).await.unwrap();

    producer_client.close().await.unwrap();
}

#[tokio::test]
async fn producer_client_can_send_without_specifying_partition_id() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubProducerClientOptions::default();
    let mut producer_client = EventHubProducerClient::new(connection_string, None, options)
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

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubProducerClientOptions::default();
    let mut producer_client = EventHubProducerClient::new(connection_string, None, options)
        .await
        .unwrap();

    let event = "Hello, world!";
    let options = CreateBatchOptions::new();
    let mut event_batch = producer_client.create_batch(options).await.unwrap();
    event_batch.try_add(event).unwrap();
    producer_client.send_batch(event_batch, SendEventOptions::default()).await.unwrap();

    producer_client.close().await.unwrap();
}
