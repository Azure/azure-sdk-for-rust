use azure_messaging_servicebus::{
    client::{
        ServiceBusClient, ServiceBusClientOptions,
    },
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
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_connect_using_connection_string_over_amqp_websockets() {
    setup_dotenv();

    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut option = ServiceBusClientOptions::default();
    option.transport_type = ServiceBusTransportType::AmqpWebSockets;

    let mut client = ServiceBusClient::new(&connection_string, option)
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    client.dispose().await.unwrap();
}
