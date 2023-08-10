//! This example shows how to spawn a consumer client to read events from an Event Hub.

use std::sync::Arc;

use azeventhubs::consumer::{
    EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions,
};
use futures_util::lock::Mutex;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = "ENTER YOUR CONNECTION STRING HERE";
    // You can leave event_hub_name as None if your connection string contains the EntityPath
    let event_hub_name = Some(String::from("ENTER YOUR EVENT HUB NAME HERE"));
    let options = EventHubConsumerClientOptions::default();

    // Create a consumer client
    let mut consumer_client = EventHubConsumerClient::from_connection_string(
        EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME,
        connection_string,
        event_hub_name,
        options,
    )
    .await?;

    let partition_ids = consumer_client.get_partition_ids().await?;
    let starting_position = EventPosition::earliest();
    let options = ReadEventOptions::default();

    // You may also move the consumer client into the spawned task and return it from the task at
    // the end.
    let arc = Arc::new(Mutex::new(consumer_client));
    let arc_clone = arc.clone();
    let handle = tokio::spawn(async move {
        // Get a stream of events from the first partition
        let mut consumer_client = arc_clone.lock().await;
        let mut stream = consumer_client
            .read_events_from_partition(&partition_ids[0], starting_position, options)
            .await?;

        // Receive 30 events
        let mut counter = 0;
        while let Some(event) = stream.next().await {
            let event = event?;
            let body = event.body()?;
            let value = std::str::from_utf8(body)?;
            log::info!("{:?}", value);

            log::info!("counter: {}", counter);
            counter += 1;
            if counter > 30 {
                break;
            }
        }

        // Close the stream
        stream.close().await?;

        // Close the consumer client
        Ok::<_, azure_core::error::Error>(())
    });

    handle.await??;

    // Take ownership of the consumer client out of the Arc and Mutex
    let consumer_client = Arc::try_unwrap(arc).unwrap().into_inner();
    consumer_client.close().await?;

    Ok(())
}
