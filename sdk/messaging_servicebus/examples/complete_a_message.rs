use azure_messaging_servicebus::{ServiceBusClient, ServiceBusClientOptions};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new(connection_string, ServiceBusClientOptions::default()).await?;

    // Create a sender and send a message
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;
    sender.send_message("Hello World").await?;
    sender.dispose().await?;

    // Create a receiver and receive a message
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;
    let message = receiver.receive_message().await?;

    // Complete the message
    receiver.complete_message(&message).await?;

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
