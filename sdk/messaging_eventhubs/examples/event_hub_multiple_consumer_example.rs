use std::sync::{Arc};

use futures_util::StreamExt;
use futures_util::lock::Mutex;
use messaging_eventhubs::consumer::{
    EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions,
};
use messaging_eventhubs::IntoAzureCoreError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename("./sdk/messaging_eventhubs/.env");

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING")?;
    let event_hub_name = std::env::var("EVENT_HUB_NAME")?;
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
            let event = event.unwrap();
            let body = event.body().unwrap();
            let value = std::str::from_utf8(body).unwrap();
            log::info!("{:?}", value);

            log::info!("counter: {}", counter);
            counter += 1;
            if counter > 30 {
                break;
            }
        }

        // Close the stream
        stream
            .close()
            .await
            .map_err(IntoAzureCoreError::into_azure_core_error)?;

        // Close the consumer client
        Ok::<_, azure_core::error::Error>(())
    });

    handle.await??;
    // consumer_client.close().await?;
    let consumer_client = Arc::try_unwrap(arc).unwrap().into_inner();
    consumer_client.close().await?;

    Ok(())
}
