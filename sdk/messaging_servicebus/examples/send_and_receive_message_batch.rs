use azservicebus::{
    CreateMessageBatchOptions, ServiceBusClient, ServiceBusClientOptions, ServiceBusMessage,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new_from_connection_string(connection_string, ServiceBusClientOptions::default()).await?;

    // Create a sender and then send a batch of messages
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;
    let mut batch = sender.create_message_batch(CreateMessageBatchOptions::default())?;

    // Add messages to the batch
    // The three lines below are all equivalent
    batch.try_add_message("Message 1")?;
    batch.try_add_message(ServiceBusMessage::new("Message 2"))?;
    batch.try_add_message(ServiceBusMessage::from("Message 3"))?;

    // Send the batch
    sender.send_message_batch(batch).await?;

    // Create a receiver and then receive the messages
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;
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
