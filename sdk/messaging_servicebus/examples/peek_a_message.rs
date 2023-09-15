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

    let peeked = receiver.peek_message(None).await?;
    if let Some(peeked) = peeked {
        let message_body = std::str::from_utf8(peeked.body()?)?;
        println!("Peeked message: {:?}", message_body);
    }

    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
