use azservicebus::{ServiceBusClient, ServiceBusClientOptions};
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client = ServiceBusClient::new_from_connection_string(
        connection_string,
        ServiceBusClientOptions::default(),
    )
    .await?;
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;

    // Schedule a message and then cancel it
    let enqueue_time = OffsetDateTime::now_utc() + time::Duration::minutes(10);
    let sequece_number = sender.schedule_message("Hello World", enqueue_time).await?;
    sender.cancel_scheduled_message(sequece_number).await?;

    sender.dispose().await?;
    client.dispose().await?;
    Ok(())
}
