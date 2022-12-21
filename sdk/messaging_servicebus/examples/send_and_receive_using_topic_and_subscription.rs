use azure_messaging_servicebus::{
    ServiceBusClient, ServiceBusClientOptions, ServiceBusReceiverOptions, ServiceBusSenderOptions,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let topic_name = std::env::var("SERVICE_BUS_TOPIC")?;
    let subscription_name = std::env::var("SERVICE_BUS_SUBSCRIPTION")?;

    let mut client =
        ServiceBusClient::new(connection_string, ServiceBusClientOptions::default()).await?;
    let mut sender = client
        .create_sender(&topic_name, ServiceBusSenderOptions::default())
        .await?;
    let mut receiver = client
        .create_receiver_for_subscription(
            topic_name,
            subscription_name,
            ServiceBusReceiverOptions::default(),
        )
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
