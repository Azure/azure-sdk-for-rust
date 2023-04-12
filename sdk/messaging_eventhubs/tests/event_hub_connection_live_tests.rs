use messaging_eventhubs::{
    BasicRetryPolicy, EventHubConnection, EventHubConnectionOptions, EventHubsRetryOptions,
    EventHubsTransportType,
};

mod common;

#[tokio::test]
async fn connection_can_connect_to_event_hubs_using_full_connection_string_over_tcp() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH").unwrap();
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::new(connection_string, None, options)
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
    let connection = EventHubConnection::new(connection_string, event_hub_name, options)
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
    let connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await
        .unwrap();
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_get_event_hub_properties() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubConnectionOptions::default();
    let mut connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await
        .unwrap();
    let options = EventHubsRetryOptions::default();
    let properties = connection
        .get_properties(BasicRetryPolicy::from(options))
        .await
        .unwrap();
    println!("{:?}", properties);
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_get_partition_properties() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubConnectionOptions::default();
    let mut connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await
        .unwrap();
    let options = EventHubsRetryOptions::default();
    let properties = connection
        .get_partition_properties("0", BasicRetryPolicy::from(options))
        .await
        .unwrap();
    println!("{:?}", properties);
    connection.close().await.unwrap();
}

#[tokio::test]
async fn connection_can_get_event_hub_properties_after_idling_for_40_mins() {
    common::setup_dotenv();

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_NAME").unwrap();
    let options = EventHubConnectionOptions::default();
    let mut connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await
        .unwrap();
    let options = EventHubsRetryOptions::default();
    let properties = connection
        .get_properties(BasicRetryPolicy::from(options.clone()))
        .await
        .unwrap();
    println!("{:?}", properties);

    for i in 0..40 {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        println!("{} mins have passed", i+1);
    }

    let properties = connection
        .get_properties(BasicRetryPolicy::from(options))
        .await
        .unwrap();
    println!("{:?}", properties);
    connection.close().await.unwrap();
}
