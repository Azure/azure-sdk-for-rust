use std::env;

use azure_messaging_servicebus::client::service_bus_client::ServiceBusClient;

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
