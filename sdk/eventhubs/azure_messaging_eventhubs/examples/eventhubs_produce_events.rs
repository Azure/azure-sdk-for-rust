/// This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient`.
///
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{producer::SendMessageOptions, ProducerClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(eventhub_namespace, eventhub_name, credential, None);

    println!("Created producer client.");

    // Open the client
    client.open().await?;

    // Send a message to an eventhub instance directly. The message will be sent to a random partition.
    client.send("Hello, Event Hub!", None).await?;

    // Send an array of bytes to partition 0 of the eventhubs instance.
    client
        .send(
            vec![2, 4, 8, 16],
            Some(SendMessageOptions {
                partition_id: Some("0".to_string()),
            }),
        )
        .await?;

    println!("Sent messages. Closing client.");

    client.close().await?;
    Ok(())
}
