use azservicebus::{ServiceBusClient, ServiceBusClientOptions, ServiceBusMessage};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // The connection string should look like:
    // "Endpoint=sb://<your-namespace>.servicebus.windows.net/;SharedAccessKeyName=<your-policy>;SharedAccessKey=<your-key>"
    let connection_string = std::env::var("SERVICE_BUS_CONNECTION_STRING")?;
    // The queue must have sessions enabled
    let queue_name = std::env::var("SERVICE_BUS_SESSION_QUEUE")?;
    let session_id = "session1";

    let mut client = ServiceBusClient::new_from_connection_string(
        connection_string,
        ServiceBusClientOptions::default(),
    )
    .await?;

    // Create a sender and send a session message
    let mut sender = client
        .create_sender(&queue_name, Default::default())
        .await?;
    let mut message = ServiceBusMessage::new("Hello World");
    message.set_session_id(String::from(session_id))?;
    sender.send_message(message).await?;

    // Create a receiver and receive the session message
    let mut receiver = client
        .accept_next_session_for_queue(queue_name, Default::default())
        .await?;
    assert_eq!(receiver.session_id(), session_id);
    let received = receiver.receive_message().await?;
    receiver.complete_message(&received).await?;

    sender.dispose().await?;
    receiver.dispose().await?;
    client.dispose().await?;
    Ok(())
}
