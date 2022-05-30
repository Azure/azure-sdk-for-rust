# Azure SDK for Rust - Azure service bus crate

## The Messaging Service Bus crate

`azure-messaging-servicebus` offers functionality needed to interact with Azure's Service Bus from Rust. As an abstraction over the [Service Bus API](https://docs.microsoft.com/en-us/rest/api/servicebus/), anything that is possible through that Rest API
should also be possible with this crate.

### An example

```rs
// Use the prelude to bring `Client` into scope
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() {
    // Get your service bus namespace, queue name, policy name, and policy key from environment variables
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Please set AZURE_SERVICE_BUS_NAMESPACE env variable first!");

    let queue_name =
        std::env::var("AZURE_QUEUE_NAME").expect("Please set AZURE_QUEUE_NAME env variable first!");

    let policy_name = std::env::var("AZURE_POLICY_NAME")
        .expect("Please set AZURE_POLICY_NAME env variable first!");

    let policy_key =
        std::env::var("AZURE_POLICY_KEY").expect("Please set AZURE_POLICY_KEY env variable first!");

    // Create a new instance of an HTTP client to make calls to the rest API
    let http_client = azure_core::new_http_client();

    // Create a new instance of a client object to group together all we've gathered so far.
    let mut client = Client::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )
    .expect("Failed to create client");

    // Send message to your queue
    let message_to_send = "hello, world!";
    client
        .send_message(message_to_send)
        .await
        .expect("Failed to send message while testing receive");
    println!("Sent Message: {}", message_to_send);

    // Receive and delete a message from your queue
    let received_message = client
        .receive_and_delete_message()
        .await
        .expect("Failed to receive message");
    println!("Received Message: {}", received_message);
}

```