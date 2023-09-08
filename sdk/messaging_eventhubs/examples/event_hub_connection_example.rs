use azeventhubs::{EventHubConnection, EventHubConnectionOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename("./sdk/messaging_eventhubs/.env");

    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING_WITH_ENTITY_PATH")?;
    let _event_hub_name = std::env::var("EVENT_HUB_NAME")?;
    let options = EventHubConnectionOptions::default();
    let connection =
        EventHubConnection::new_from_connection_string(connection_string, None, options).await?;
    connection.close().await?;

    Ok(())
}
