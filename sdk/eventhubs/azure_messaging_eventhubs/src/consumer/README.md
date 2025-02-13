# Consumer Module Overview

This module contains the [`ConsumerClient`] struct and related types, which are used for receiving events from an Event Hub.

The [`ConsumerClient`] provides functionality to establish a connection to an Event Hub, receive events from a specific partition,
and manage the lifecycle of the consumer client.

## Examples

### Creating a new [`ConsumerClient`] instance

```rust no_run
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::ConsumerClient;

#[tokio::main]
async fn main() -> Result<(), azure_core::Error> {
    let my_credential = DefaultAzureCredential::new()?;
    let consumer = ConsumerClient::builder("my_namespace", "my_eventhub", None, my_credential)
        .open()
        .await?;
    Ok(())
}
```

### Opening a connection to the Event Hub

```rust no_run
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::ConsumerClient;

#[tokio::main]
async fn main() -> Result<(), azure_core::Error> {
    let my_credential = DefaultAzureCredential::new()?;
    let result = ConsumerClient::builder("my_namespace", "my_eventhub", None, my_credential)
        .open()
        .await;

    match result {
        Ok(consumer) => {
            // Connection opened successfully
            println!("Connection opened successfully");
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error opening connection: {:?}", err);
        }
    }
    Ok(())
}
```

### Closing the connection to the Event Hub

```rust no_run
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::ConsumerClient;

#[tokio::main]
async fn main() -> Result<(), azure_core::Error> {
    let my_credential = DefaultAzureCredential::new()?;
    let consumer = ConsumerClient::builder("my_namespace", "my_eventhub", None, my_credential)
        .open()
        .await?;

    let result = consumer.close().await;

    match result {
        Ok(()) => {
            // Connection closed successfully
            println!("Connection closed successfully");
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error closing connection: {:?}", err);
        }
    }
    Ok(())
}
```

### Receiving events from a specific partition of the Event Hub

```rust no_run
use async_std::stream::StreamExt;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::ConsumerClient;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_credential = DefaultAzureCredential::new().unwrap();
    let consumer = ConsumerClient::builder("my_namespace", "my_eventhub", None, my_credential)
        .open()
        .await?;
    let partition_id = "0";

    let message_receiver = consumer
        .open_receiver_on_partition(partition_id, None)
        .await?;

    let event_stream = message_receiver.stream_events();

    pin_mut!(event_stream);

    while let Some(event_result) = event_stream.next().await {
        match event_result {
            Ok(event) => {
                // Process the received event
                println!("Received event: {:?}", event);
            }
            Err(err) => {
                // Handle the error
                eprintln!("Error receiving event: {:?}", err);
            }
        }
    }
    Ok(())
}
```
