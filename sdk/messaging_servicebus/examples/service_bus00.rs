use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() {
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

    let queue_name =
        std::env::var("AZURE_QUEUE_NAME").expect("Please set AZURE_QUEUE_NAME env variable first!");

    let policy_name = std::env::var("AZURE_POLICY_NAME")
        .expect("Please set AZURE_POLICY_NAME env variable first!");

    let policy_key =
        std::env::var("AZURE_POLICY_KEY").expect("Please set AZURE_POLICY_KEY env variable first!");

    let http_client = azure_core::default_client();

    let client = Client::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )
    .expect("Failed to create client");

    let message_to_send = "hello, world!";

    client
        .send_message(message_to_send)
        .await
        .expect("Failed to send message while testing receive");

    println!("Sent Message: {}", message_to_send);

    let received_message = client
        .receive_and_delete_message()
        .await
        .expect("Failed to receive message");

    println!("Received Message: {}", received_message);
}
