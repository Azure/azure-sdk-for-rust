<!-- cspell: words -->

# consumer Module Overview

This module contains the `ConsumerClient` struct and related types, which are used for receiving events from an Event Hub.

The `ConsumerClient` provides functionality to establish a connection to an Event Hub, receive events from a specific partition,
and manage the lifecycle of the consumer client.

## Examples

### Creating a new `ConsumerClient` instance

```rust no_run
use azure_messaging_eventhubs::consumer::ConsumerClient;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};

let my_credential = DefaultAzureCredential::new().unwrap();
let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
```

### Opening a connection to the Event Hub

```rust no_run
use azure_messaging_eventhubs::consumer::ConsumerClient;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};

#[tokio::main]
async fn main() {
    let my_credential = DefaultAzureCredential::new().unwrap();
    let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);

    let result = consumer.open().await;

     match result {
        Ok(()) => {
            // Connection opened successfully
            println!("Connection opened successfully");
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error opening connection: {:?}", err);
        }
    }
}
```

### Closing the connection to the Event Hub

```rust no_run
use azure_messaging_eventhubs::consumer::ConsumerClient;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};

#[tokio::main]
async fn main() {
    let my_credential = DefaultAzureCredential::new().unwrap();
    let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);

    consumer.open().await.unwrap();

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
}
```

### Receiving events from a specific partition of the Event Hub

```rust no_run
use azure_messaging_eventhubs::consumer::ConsumerClient;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use async_std::stream::StreamExt;

#[tokio::main]
async fn main() {
    let my_credential = DefaultAzureCredential::new().unwrap();
    let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
    let partition_id = "0";
    let options = None;

    consumer.open().await.unwrap();

    let event_stream = consumer.receive_events_on_partition(partition_id.to_string(), options).await;

    tokio::pin!(event_stream);
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
}
```
