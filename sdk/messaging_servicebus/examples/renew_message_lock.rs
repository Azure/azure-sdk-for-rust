use azservicebus::{ServiceBusClient, ServiceBusClientOptions};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client = ServiceBusClient::new_from_connection_string(
        connection_string,
        ServiceBusClientOptions::default(),
    )
    .await?;
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;

    // Try to receive a message with a default max wait time
    let message = receiver.receive_message_with_max_wait_time(None).await?;
    if let Some(mut message) = message {
        // Renew the message lock
        // A mutable reference is needed because the message locked_until property is updated
        // in place.
        receiver.renew_message_lock(&mut message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
