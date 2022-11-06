use std::env;

use azure_messaging_servicebus::client::service_bus_client::ServiceBusClient;

fn setup_dotenv() {
    dotenv::from_filename("./sdk/messaging_servicebus/tests/.env").ok();
}

#[test]
fn hello_world() {
    setup_dotenv();
    assert_eq!(env::var("HELLO").unwrap(), "hello");
}

#[tokio::test]
async fn client_can_connect_with_connection_string() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let mut client = ServiceBusClient::new(connection_string).await.unwrap();

    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_create_sender() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let sender = client.create_sender(queue).await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_send_single_message() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client.create_sender(queue).await.unwrap();
    sender.send_message("hello world").await.unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_send_multiple_messages() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut sender = client.create_sender(queue).await.unwrap();
    sender
        .send_messages(vec!["hello world", "hello world 2"])
        .await
        .unwrap();

    sender.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_can_create_receiver() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let receiver = client.create_receiver(queue).await.unwrap();

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}

#[tokio::test]
async fn client_receive_messages() {
    setup_dotenv();
    let connection_string = env::var("SERVICE_BUS_CONNECTION_STRING").unwrap();
    let queue = env::var("SERVICE_BUS_QUEUE").unwrap();

    let mut client = ServiceBusClient::new(connection_string).await.unwrap();
    let mut receiver = client.create_receiver(queue).await.unwrap();
    let messages = receiver.receive_messages(1, None).await.unwrap();

    assert_eq!(messages.len(), 1);

    for message in messages {
        receiver.complete_message(message).await.unwrap();
    }

    receiver.dispose().await.unwrap();
    client.dispose().await.unwrap();
}
