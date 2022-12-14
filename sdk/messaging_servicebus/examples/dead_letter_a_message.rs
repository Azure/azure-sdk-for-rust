use azure_messaging_servicebus::{ServiceBusClient, ServiceBusClientOptions};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new(connection_string, ServiceBusClientOptions::default()).await?;

    let mut receiver = client.create_receiver_for_queue(queue_name, Default::default()).await?;

    // Try to receive a message with a default max wait time
    let message = receiver.receive_message_with_max_wait_time(None).await?;
    if let Some(message) = message {
        // Dead letter the message
        receiver.dead_letter_message(&message, Default::default()).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
