/// This sample demonstrates how to send AMQP messages to an Event Hub partition using the `ProducerClient`.
///
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{
    models::{AmqpMessage, AmqpMessageBody},
    ProducerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(eventhub_namespace, eventhub_name, credential, None);

    println!("Created producer client.");

    // Open the client
    client.open().await?;

    // Send a message to an eventhub instance directly. The message will be sent to a random partition.
    client
        .send_message(
            AmqpMessage::builder()
                .with_body(AmqpMessageBody::Value("Hello, Event Hub from AMQP!".into()))
                .build(),
            None,
        )
        .await?;

    // Send an array of bytes to partition 0 of the eventhubs instance.
    client
        .send_message(
            AmqpMessage::builder()
                .with_body(AmqpMessageBody::Binary(vec![vec![2, 13, 8, 16]]))
                .build(),
            None,
        )
        .await?;

    println!("Sent messages. Closing client.");

    client.close().await?;
    Ok(())
}
