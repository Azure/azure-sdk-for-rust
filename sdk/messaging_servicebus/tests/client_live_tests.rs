use azure_messaging_servicebus::{
    authorization::AzureNamedKeyCredential,
    client::{ServiceBusClient, ServiceBusClientOptions},
    primitives::service_bus_transport_type::ServiceBusTransportType,
};

mod common;
use common::setup_dotenv;

#[tokio::test]
async fn client_can_connect_using_connection_string_over_amqp_tcp() {
    setup_dotenv();

    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut option = ServiceBusClientOptions::default();
    option.transport_type = ServiceBusTransportType::AmqpTcp;

    let mut client = ServiceBusClient::new(&connection_string, option)
        .await
        .unwrap();

    // Create a sender for authentication purpose only.
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
    let sender = client
        .create_sender(queue_name, Default::default())
        .await
        .unwrap();
    sender.dispose().await.unwrap();

    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_connect_using_connection_string_over_amqp_websocket() {
    setup_dotenv();

    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut option = ServiceBusClientOptions::default();
    option.transport_type = ServiceBusTransportType::AmqpWebSocket;

    let mut client = ServiceBusClient::new(&connection_string, option)
        .await
        .unwrap();

    // Create a sender for authentication purpose only.
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();
    let sender = client
        .create_sender(queue_name, Default::default())
        .await
        .unwrap();
    sender.dispose().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_connect_using_named_key_credential() {
    setup_dotenv();
    let namespace = std::env::var("SERVICE_BUS_NAMESPACE").unwrap();
    let key_name = std::env::var("SERVICE_BUS_SAS_KEY_NAME").unwrap();
    let key = std::env::var("SERVICE_BUS_SAS_KEY").unwrap();
    let queue_name = std::env::var("SERVICE_BUS_QUEUE").unwrap();

    let credential = AzureNamedKeyCredential::new(key_name, key);
    let mut client =
        ServiceBusClient::new_with_named_key_credential(namespace, credential, Default::default())
            .await
            .unwrap();

    // Creating sender and receiver will perform CBS authentication first.
    let sender = client
        .create_sender(queue_name.clone(), Default::default())
        .await
        .unwrap();
    let receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await
        .unwrap();

    receiver.dispose().await.unwrap();
    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}
