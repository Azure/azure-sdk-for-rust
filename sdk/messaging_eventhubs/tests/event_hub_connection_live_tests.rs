use messaging_eventhubs::{EventHubConnection, EventHubConnectionOptions, EventHubsTransportType};

mod common;

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string_over_tcp() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::from_connection_string(connection_string, None, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string_and_event_hub_over_websockets(
) {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let mut options = EventHubConnectionOptions::default();
    options.transport_type = EventHubsTransportType::AmqpWebSockets;
    let connection = EventHubConnection::from_connection_string(connection_string, event_hub_name, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string_and_event_hub_over_tcp()
{
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::from_connection_string(connection_string, event_hub_name, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}
