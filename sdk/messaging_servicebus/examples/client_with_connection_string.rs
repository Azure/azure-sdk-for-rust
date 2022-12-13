use azure_messaging_servicebus::{
    ServiceBusClient, ServiceBusClientOptions, ServiceBusSenderOptions,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new(&connection_string, ServiceBusClientOptions::default()).await?;

    // Create a sender for authentication purpose only
    let sender = client
        .create_sender(queue_name, ServiceBusSenderOptions::default())
        .await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}
