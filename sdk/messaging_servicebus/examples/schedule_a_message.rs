use azservicebus::{ServiceBusClient, ServiceBusClientOptions, ServiceBusMessage};
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    let queue_name = std::env::var("SERVICE_BUS_QUEUE")?;

    let mut client =
        ServiceBusClient::new_from_connection_string(connection_string, ServiceBusClientOptions::default()).await?;
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;

    // Schedule message by setting the scheduled enqueue time
    let mut message = ServiceBusMessage::new("Message 1");
    let enqueue_time = OffsetDateTime::now_utc() + time::Duration::minutes(1);
    message.set_scheduled_enqueue_time(enqueue_time);
    sender.send_message(message).await?;

    // Schedule message by using the schedule_message method
    let message = ServiceBusMessage::new("Message 2");
    let enqueue_time = OffsetDateTime::now_utc() + time::Duration::minutes(1);
    let _sequence_number = sender.schedule_message(message, enqueue_time).await?;

    // Dispose the sender
    sender.dispose().await?;

    // Wait for one minute
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    // Receive and settle the scheduled messages
    let mut receiver = client
        .create_receiver_for_queue(queue_name, Default::default())
        .await?;
    let received = receiver.receive_messages(2).await?;
    for message in received {
        receiver.complete_message(&message).await?;
    }

    // Dispose the receiver
    receiver.dispose().await?;

    // Dispose the client
    client.dispose().await?;
    Ok(())
}
