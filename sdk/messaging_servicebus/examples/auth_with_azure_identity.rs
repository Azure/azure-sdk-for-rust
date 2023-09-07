use azure_messaging_servicebus::{
    ServiceBusClient, ServiceBusClientOptions,
    ServiceBusSenderOptions,
};
use azure_identity::DefaultAzureCredential;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The namespace should look like: "<your-namespace>.servicebus.windows.net"
    let namespace = std::env::var("SERVICE_BUS_NAMESPACE")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let credential = DefaultAzureCredential::default();
    let mut client = ServiceBusClient::new_from_credential(
        namespace,
        credential,
        ServiceBusClientOptions::default(),
    )
    .await?;

    // Create a sender for auth purpose only
    let sender = client
        .create_sender(queue_name, ServiceBusSenderOptions::default())
        .await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}
