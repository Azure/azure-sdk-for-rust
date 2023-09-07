//! This is the same example shown in the crate-level documentation

use azservicebus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
    // which can be found in the Azure portal and should look like
    // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new_from_connection_string(
        "<NAMESPACE-CONNECTION-STRING>",
        ServiceBusClientOptions::default(),
    )
    .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut receiver = client
        .create_receiver_for_queue("<QUEUE-NAME>", ServiceBusReceiverOptions::default())
        .await?;

    // Receive messages from the queue
    // This will wait indefinitely until at least one message is received
    let messages = receiver.receive_messages(3).await?;

    for message in &messages {
        let body = message.body()?;
        println!("Received message: {:?}", std::str::from_utf8(body)?);

        // Complete the message so that it is removed from the queue
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;

    Ok(())
}
