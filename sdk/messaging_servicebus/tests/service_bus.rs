#![cfg(all(test, feature = "test_e2e"))] // to run this, do: `cargo test --features test_e2e`
extern crate log;

use azure_messaging_servicebus::service_bus::Client;
use std::time::Duration;

#[tokio::test]
async fn send_message_test() {
    let client = create_client().unwrap();
    client
        .send_message("hello, world!")
        .await
        .expect("Failed to send message");
}

#[tokio::test]
async fn receive_and_delete_message_test() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can receive something
    client
        .receive_and_delete_message()
        .await
        .expect("Failed to receive message");
}

#[tokio::test]
async fn peek_lock_message_test() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can receive something
    client
        .peek_lock_message(None)
        .await
        .expect("Failed to receive message");
}

#[tokio::test]
async fn peek_lock_message2_test() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can receive something
    client
        .peek_lock_message2(None)
        .await
        .expect("Failed to receive message");
}

#[tokio::test]
async fn delete_message_test() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can delete something
    client
        .peek_lock_message2(None)
        .await
        .expect("Failed to receive message")
        .delete_message()
        .await
        .expect("Failed to delete message");
}

#[tokio::test]
async fn renew_message_lock_test() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can receive something
    client
        .peek_lock_message2(Some(Duration::from_secs(60)))
        .await
        .expect("Failed to receive message")
        .renew_message_lock()
        .await
        .expect("Failed to renew message's lock");
}

#[tokio::test]
async fn unlock_message() {
    let client = create_client().unwrap();
    send_message_test(); // send message to ensure we can receive something
    client
        .peek_lock_message2(None)
        .await
        .expect("Failed to receive message")
        .unlock_message()
        .await
        .expect("Failed to unlock message's lock");
}

fn create_client() -> azure_core::Result<Client> {
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

    let queue_name =
        std::env::var("AZURE_QUEUE_NAME").expect("Please set AZURE_QUEUE_NAME env variable first!");

    let policy_name = std::env::var("AZURE_POLICY_NAME")
        .expect("Please set AZURE_POLICY_NAME env variable first!");

    let policy_key =
        std::env::var("AZURE_POLICY_KEY").expect("Please set AZURE_POLICY_KEY env variable first!");

    let http_client = azure_core::new_http_client();

    Client::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )
}
