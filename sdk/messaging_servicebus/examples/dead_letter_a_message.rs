use azservicebus::{
    ServiceBusClient, ServiceBusClientOptions, ServiceBusReceiverOptions, SubQueue,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new_from_connection_string(connection_string, ServiceBusClientOptions::default()).await?;

    let mut receiver = client
        .create_receiver_for_queue(&queue_name, Default::default())
        .await?;

    // Try to receive a message with a default max wait time
    let message = receiver.receive_message_with_max_wait_time(None).await?;
    if let Some(message) = message {
        // Dead letter the message
        receiver
            .dead_letter_message(&message, Default::default())
            .await?;
    }
    receiver.dispose().await?;

    // Create a separate deadletter receiver to receive the dead-lettered message
    let options = ServiceBusReceiverOptions {
        sub_queue: SubQueue::DeadLetter,
        ..Default::default()
    };
    let mut dlq_receiver = client
        .create_receiver_for_queue(queue_name, options)
        .await?;
    let message = dlq_receiver
        .receive_message_with_max_wait_time(None)
        .await?;
    if let Some(message) = message {
        // Complete the message
        dlq_receiver.complete_message(&message).await?;
    }
    dlq_receiver.dispose().await?;

    client.dispose().await?;
    Ok(())
}
