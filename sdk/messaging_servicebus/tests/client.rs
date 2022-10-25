use std::env;

use azure_messaging_servicebus::client::service_bus_client::ServiceBusClient;
use fe2o3_amqp::{sasl_profile::SaslProfile, Connection};

fn setup_dotenv() {
    dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
}

#[test]
fn hello_world() {
    setup_dotenv();
    println!("{:?}", env::var("HELLO").unwrap());
}

#[tokio::test]
async fn client_can_connect_with_connection_string() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    println!("{:?}", connection_string);
    let mut client = ServiceBusClient::new(connection_string).await.unwrap();

    client.dispose().await.unwrap();
}

#[tokio::test]
async fn anonymous_amqp_connection() {
    let addr = "amqps://fe2o3-amqp-example.servicebus.windows.net:5671";
    let mut connection = Connection::builder()
        .container_id("test")
        .alt_tls_establishment(true)
        .sasl_profile(SaslProfile::Anonymous)
        .open(addr)
        .await
        .unwrap();

    connection.close().await.unwrap();
}
