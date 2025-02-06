/// This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient`.
/// It also demonstrates how to send events to all partitions using a batch sender.
///
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{
    producer::SendMessageOptions, EventDataBatchOptions, ProducerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(eventhub_namespace, eventhub_name, credential, None);

    // Open the client
    client.open().await?;

    // Get the partition IDs
    let properties = client.get_eventhub_properties().await?;
    println!("EventHub Properties: {:?}", properties);

    // Create a message to send
    let message = "Hello, Event Hub!";

    // Send a message to an eventhub instance directly.
    client.send(message, None).await?;

    // Send an array of bytes to partition 0 of the eventhubs instance.
    client
        .send(
            vec![2, 4, 8, 16],
            Some(SendMessageOptions {
                partition_id: Some("0".to_string()),
            }),
        )
        .await?;

    // Send the message to each partition using a batch sender.
    for partition_id in properties.partition_ids {
        let mut batch = client
            .create_batch(Some(EventDataBatchOptions {
                partition_id: Some(partition_id.clone()),
                ..Default::default()
            }))
            .await?;
        if batch.try_add_event_data(message, None)? {
            println!("Message sent to partition: {}", partition_id);
        }
        client.submit_batch(&batch, None).await?;
    }

    Ok(())
}
