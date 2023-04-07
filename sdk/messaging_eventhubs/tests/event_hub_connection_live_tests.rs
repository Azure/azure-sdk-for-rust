use messaging_eventhubs::{EventHubConnection, EventHubConnectionOptions};

mod common;

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::new(connection_string, None, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string_and_event_hub() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}
