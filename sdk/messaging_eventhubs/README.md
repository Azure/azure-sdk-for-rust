# azeventhubs

An unofficial and experimental Azure Event Hubs client library for Rust.

This crate follows a similar structure to that of the [Azure SDK for
.Net](https://github.com/Azure/azure-sdk-for-net/tree/main/sdk/eventhub/Azure.Messaging.EventHubs),
and thus it should be familiar to anyone who has used the dotnet SDK. This crate is still in
development, and not all features are implemented yet.

## Examples

### Event Hub Producer Example

```rust
use azeventhubs::producer::{
EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut producer_client =
        EventHubProducerClient::new_from_connection_string(
            "<CONNECTION_STRING>", // Replace with your connection string
            "<EVENT_HUB_NAME>".to_string(), // Replace with your hub name
            EventHubProducerClientOptions::default()
       ).await?;

    let partition_ids = producer_client.get_partition_ids().await?;

    let event = "Hello, world to first partition!";
    let options = SendEventOptions::new().with_partition_id(&partition_ids[0]);
    producer_client.send_event(event, options).await?;

    producer_client.close().await?;

    Ok(())
}
```

### Event Hub Consumer Example

```rust
use futures_util::StreamExt;
use azeventhubs::consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a consumer client
    let mut consumer_client =
        EventHubConsumerClient::new_from_connection_string(
            EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME,
            "<CONNECTION_STRING>", // Replace with your connection string
            "<EVENT_HUB_NAME>".to_string(), // Replace with your hub name
            EventHubConsumerClientOptions::default(),
        ).await?;

    let partition_ids = consumer_client.get_partition_ids().await?;
    let starting_position = EventPosition::earliest();
    let options = ReadEventOptions::default();

    // Get a stream of events from the first partition
    let mut stream = consumer_client
        .read_events_from_partition(&partition_ids[0], starting_position, options)
        .await?;

    // Receive 30 events
    let mut counter = 0;
    while let Some(event) = stream.next().await {
        let event = event?;
        let body = event.body()?;
        let value = std::str::from_utf8(body)?;
        log::info!("{:?}", value);

        log::info!("counter: {}", counter);
        counter += 1;
        if counter > 30 {
            break;
        }
    }
    // Close the stream
    stream.close().await?;

    // Close the consumer client
    consumer_client.close().await?;

    Ok(())
}
```

## What is implemented and what is not?

| Feature | Supported |
| ------- | --------- |
| Event Hub Connection | Yes |
| Event Hub Producer | Yes |
| Event Hub Consumer | Yes |
| Partition Receiver | Yes |
| Event Hub Buffered Producer | Not yet |
| Event Hub Processor | Not yet |
| Checkpoint Store | Not yet |

## TLS Support

Communication between a client application and an Azure Service Event Hub namespace is encrypted
using Transport Layer Security (TLS). The TLS implementation is exposed to the user through the
corresponding feature flags (please see the feature flag section below). The user should ensure
either the `rustls` or `native-tls` feature is enabled, and one and only one TLS implementation
must be enabled. Enabling both features is **not** supported and will result in an error.

The `native-tls` feature is enabled by default, and it will use the `native-tls` crate to
provide TLS support. The `rustls` feature will use the `rustls` crate and `webpki-roots` crate
to provide TLS support.

## Feature Flags

This crate supports the following feature flags:

| Feature | Description |
| ------- | ----------- |
| `default` | Enables "native-tls" feature |
| `rustls` | Enables the use of the `rustls` crate for TLS support |
| `native-tls` | Enables the use of the `native-tls` crate for TLS support |

## WebAssembly Support

WebAssembly is **NOT** supported yet.

License: MIT
