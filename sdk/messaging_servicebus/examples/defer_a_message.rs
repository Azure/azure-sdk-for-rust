use azservicebus::{ServiceBusClient, ServiceBusClientOptions};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new_from_connection_string(connection_string, ServiceBusClientOptions::default()).await?;

    // Create a sender and send a message
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;
    sender.send_message("Hello World").await?;
    sender.dispose().await?;

    // Create a receiver
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;

    // Receive and then defer a message
    let message = receiver.receive_message().await?;
    let sequence_number = message.sequence_number();
    receiver.defer_message(&message, None).await?;

    // Receive the deferred message
    let message = receiver.receive_deferred_message(sequence_number).await?;
    if let Some(message) = message {
        receiver.complete_message(message).await?;
    }

    Ok(())
}
