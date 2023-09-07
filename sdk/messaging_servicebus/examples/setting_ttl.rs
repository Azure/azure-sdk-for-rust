use azservicebus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut message = ServiceBusMessage::new("test message 1");
    message.set_time_to_live(std::time::Duration::from_secs(10 * 60))?;
    sender.send_message(message).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}
