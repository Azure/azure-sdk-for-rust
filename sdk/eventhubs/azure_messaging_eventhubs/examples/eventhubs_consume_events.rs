/// This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient`.
///
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{ConsumerClient, ReceiveOptions, StartLocation, StartPosition};
use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let consumer = ConsumerClient::new(eventhub_namespace, eventhub_name, None, credential, None);

    println!("Created consumer client");
    // Open the client
    consumer.open().await?;

    // Get the partition IDs
    let properties = consumer.get_eventhub_properties().await?;
    println!("EventHub Properties: {:?}", properties);

    // The default is to receive messages from the end of the partition, so specify a start position at the start of the partition.
    let receiver = consumer
        .open_receiver_on_partition(
            properties.partition_ids[0].clone(),
            Some(ReceiveOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                receive_timeout: Some(std::time::Duration::from_secs(5)),
                ..Default::default()
            }),
        )
        .await?;

    println!("Created receiver");

    // Create a stream of events from the receiver
    let receive_stream = receiver.stream_events();

    println!("Created receive stream");

    // Pin the receive stream on the stack so that it can be polled
    pin_mut!(receive_stream);

    // Receive events until the receive_timeout has been reached.
    while let Some(event) = receive_stream.next().await {
        println!("Received: {:?}", event);
    }

    consumer.close().await?;

    Ok(())
}
