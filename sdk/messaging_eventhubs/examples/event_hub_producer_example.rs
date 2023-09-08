use azeventhubs::producer::{
    EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename("./sdk/messaging_eventhubs/.env");

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING")?;
    let event_hub_name = std::env::var("EVENT_HUB_NAME")?;
    let options = EventHubProducerClientOptions::default();
    let mut producer_client = EventHubProducerClient::new_from_connection_string(
        connection_string,
        event_hub_name,
        options,
    )
    .await?;

    let partition_ids = producer_client.get_partition_ids().await?;

    for i in 0..300 {
        let event = format!("Hello, world {}!", i);
        let options = SendEventOptions::new().with_partition_id(&partition_ids[0]);
        producer_client.send_event(event, options).await?;
    }

    producer_client.close().await?;

    Ok(())
}
