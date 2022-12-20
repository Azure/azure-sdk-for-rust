use azure_messaging_servicebus::{
    authorization::AzureNamedKeyCredential, ServiceBusClient, ServiceBusClientOptions,
    ServiceBusSenderOptions,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The namespace should look like: "<your-namespace>.servicebus.windows.net"
    let namespace = std::env::var("SERVICE_BUS_NAMESPACE")?;

    // The SAS key name and SAS key should be obtained from the Azure portal
    // Example SharedAccessKeyName: "RootManageSharedAccessKey"
    let sas_key_name = std::env::var("SERVICE_BUS_SAS_KEY_NAME")?;
    let sas_key = std::env::var("SERVICE_BUS_SAS_KEY")?;

    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let credential = AzureNamedKeyCredential::new(sas_key_name, sas_key);
    let mut client = ServiceBusClient::new_with_named_key_credential(
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
