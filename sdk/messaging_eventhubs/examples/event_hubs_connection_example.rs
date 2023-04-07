use messaging_eventhubs::{EventHubConnectionOptions, EventHubConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename("./sdk/messaging_eventhubs/.env");

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING")?;
    let event_hub_name = std::env::var("EVENT_HUB_NAME")?;
    let options = EventHubConnectionOptions::default();
    let connection = EventHubConnection::new(connection_string, event_hub_name, options)
        .await?;
    connection.close().await?;

    Ok(())
}
