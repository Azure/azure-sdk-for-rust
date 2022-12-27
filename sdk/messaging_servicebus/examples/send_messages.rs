//! This is the same example shown in the crate-level documentation

use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
    // which can be found in the Azure portal and should look like
    // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new(
        "<NAMESPACE-CONNECTION-STRING>",
        ServiceBusClientOptions::default(),
    )
    .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut sender = client
        .create_sender("<QUEUE-NAME>", ServiceBusSenderOptions::default())
        .await?;

    // Create a batch
    let mut message_batch = sender.create_message_batch(Default::default())?;

    for i in 0..3 {
        // Create a message
        let message = ServiceBusMessage::new(format!("Message {}", i));
        // Try to add the message to the batch
        if let Err(e) = message_batch.try_add_message(message) {
            // If the batch is full, an error will be returned
            println!("Failed to add message {} to batch: {:?}", i, e);
            break;
        }
    }

    // Send the batch of messages to the queue
    match sender.send_message_batch(message_batch).await {
        Ok(()) => println!("Batch sent successfully"),
        Err(e) => println!("Failed to send batch: {:?}", e),
    }

    sender.dispose().await?;
    client.dispose().await?;

    Ok(())
}
