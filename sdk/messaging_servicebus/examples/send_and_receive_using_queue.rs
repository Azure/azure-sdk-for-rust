use azure_messaging_servicebus::{ServiceBusClient, ServiceBusClientOptions};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new(connection_string, ServiceBusClientOptions::default()).await?;

    // Create a sender and receiver
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;

    // Send one message
    sender.send_message("Hello World").await?;
    // Receive the message and settle it
    let received = receiver.receive_message().await?;
    receiver.complete_message(&received).await?;

    // Send an iterator of messages
    let messages = vec!["Message 1", "Message 2", "Message 3"];
    sender.send_messages(messages).await?;
    // Receive the messages and settle them
    // This will wait indefinitely until at least one message is received
    let received = receiver.receive_messages(3).await?;
    for message in received {
        receiver.complete_message(&message).await?;
    }

    sender.dispose().await?;
    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
