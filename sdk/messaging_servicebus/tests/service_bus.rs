#![cfg(all(test, feature = "test_e2e"))] // to run this, do: `cargo test --features test_e2e`
extern crate log;

use azure_messaging_servicebus::service_bus::Client;

#[tokio::test]
async fn send_message_test() {
    let mut client = create_client().unwrap();
    client
        .send_message("hello, world!")
        .await
        .expect("Failed to send message");
}

#[tokio::test]
async fn receive_and_delete_message_test() {
    let message_to_send = "hello, world!";
    let mut client = create_client().unwrap();
    client
        .send_message(message_to_send)
        .await
        .expect("Failed to send message while testing receive");

    let received_message = client
        .receive_and_delete_message()
        .await
        .expect("Failed to receive message");

    assert_eq!(message_to_send, received_message);
}

fn create_client() -> Result<Client, azure_core::Error> {
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

    let queue_name =
        std::env::var("AZURE_QUEUE_NAME").expect("Please set AZURE_QUEUE_NAME env variable first!");

    let policy_name = std::env::var("AZURE_POLICY_NAME")
        .expect("Please set AZURE_POLICY_NAME env variable first!");

    let policy_key =
        std::env::var("AZURE_POLICY_KEY").expect("Please set AZURE_POLICY_KEY env variable first!");

    let http_client = azure_core::new_http_client();

    Ok(Client::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )?)
}
