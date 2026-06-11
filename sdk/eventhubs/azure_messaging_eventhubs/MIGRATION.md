<!-- cspell:words azeventhubs minghuaw checkpointing checkpointstore georeplication prefetch reqwest Entra deserialize servicebus eventhubs eventhub fqdn rustls -->

# Migrating to azure_messaging_eventhubs from azeventhubs

This guide helps you migrate from the community [`azeventhubs`](https://crates.io/crates/azeventhubs) crate (by [minghuaw](https://github.com/minghuaw), latest v0.20.0) to the official [`azure_messaging_eventhubs`](https://crates.io/crates/azure_messaging_eventhubs) crate (v0.15.0) published by the Azure SDK for Rust team.

This is a **cross-library migration**, not a version bump. `azeventhubs` and `azure_messaging_eventhubs` are different crates, written by different authors, with different type names, module layouts, and design philosophies. You are switching libraries, so expect to rewrite client construction, authentication, and the send/receive code rather than just bumping a version in `Cargo.toml`. The good news: the official crate covers everything `azeventhubs` did and adds the pieces it explicitly did not (a processor, checkpoint stores, and Microsoft Entra ID authentication).

## Table of Contents

- [Why Migrate](#why-migrate)
- [At a Glance](#at-a-glance)
- [General Changes](#general-changes)
  - [Crate Name and Cargo.toml](#crate-name-and-cargotoml)
  - [use Statements and Module Layout](#use-statements-and-module-layout)
  - [Authentication](#authentication)
  - [Client Construction](#client-construction)
- [Common Scenarios](#common-scenarios)
  - [Producing Events](#producing-events)
  - [Producing Events in a Batch](#producing-events-in-a-batch)
  - [Consuming Events from a Partition](#consuming-events-from-a-partition)
  - [Scalable Consumption with EventProcessor](#scalable-consumption-with-eventprocessor)
  - [Checkpoint Store Setup (Blob Storage)](#checkpoint-store-setup-blob-storage)
- [Error Handling](#error-handling)
- [Async Runtime and Concurrency](#async-runtime-and-concurrency)
- [Feature Flags](#feature-flags)
- [FAQ and Common Pitfalls](#faq-and-common-pitfalls)
- [Additional Resources](#additional-resources)

## Why Migrate

- **Official, actively maintained crate.** `azure_messaging_eventhubs` is built and supported by the Azure SDK team and follows the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html). The community `azeventhubs` crate served the Rust ecosystem well as the de facto Event Hubs client, but it is unofficial and its own documentation now points users to the official SDK.
- **Microsoft Entra ID authentication.** `azeventhubs` authenticates with connection strings (shared access keys). The official crate integrates with [`azure_identity`](https://crates.io/crates/azure_identity), so you can use `DeveloperToolsCredential` for local development and managed identity in production, with no secrets in your configuration.
- **Scalable consumption with `EventProcessor`.** `azeventhubs` left "Event Hub Processor" and "Checkpoint Store" as unimplemented features. The official crate ships an `EventProcessor` that balances partitions across instances, plus a `CheckpointStore` trait and a blob-backed implementation in the companion crate `azure_messaging_eventhubs_checkpointstore_blob`.
- **Geo-replication support.** The official client negotiates the Event Hubs [geo-replication](https://learn.microsoft.com/azure/event-hubs/geo-replication) capability so producers and consumers cooperate with namespace failover.
- **Built-in resilience.** The AMQP connection layer transparently re-establishes connections, links, and claims-based-security tokens after transient failures, including detecting and surfacing partition reassignment as a dedicated `ConsumerDisconnected` error.
- **Shared Azure SDK foundation.** Transport runs on `azure_core_amqp`, errors integrate with `azure_core::Error`, and tracing uses the standard `tracing` crate, so Event Hubs behaves consistently with the rest of the Azure SDK for Rust.

## At a Glance

| Concept | `azeventhubs` (community) | `azure_messaging_eventhubs` (official) |
| --- | --- | --- |
| Producer type | `EventHubProducerClient` | `ProducerClient` |
| Consumer type | `EventHubConsumerClient` | `ConsumerClient` |
| Construction | `new_from_connection_string(...)` | `ProducerClient::builder().open(host, name, credential)` |
| Authentication | Connection string (SAS key) | Microsoft Entra ID via `azure_identity` credentials |
| Endpoint input | Connection string contains the host | Fully qualified namespace host, for example `my-ns.servicebus.windows.net` |
| Sent event type | `EventData` | `EventData` (with a builder), or anything `Into<EventData>` |
| Received event type | `ReceivedEventData` | `ReceivedEventData` |
| Receive a partition | `read_events_from_partition(...)` -> stream | `open_receiver_on_partition(...).stream_events()` |
| Start position | `EventPosition` | `StartPosition` + `StartLocation` |
| Scalable consumption | Not available | `EventProcessor` + `CheckpointStore` |
| Checkpoint store | Not available | In-memory, or blob via companion crate |
| Error type | Crate-specific error enums | `azure_messaging_eventhubs::EventHubsError` |

## General Changes

### Crate Name and Cargo.toml

Remove `azeventhubs` and add the official crate plus `azure_identity` (for credentials) and `tokio` (the async runtime):

```diff
 [dependencies]
- azeventhubs = "0.20"
+ azure_messaging_eventhubs = "0.15"
+ azure_identity = "1.0"
+ tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

If you want scalable consumption with durable checkpoints in Azure Blob Storage, also add the companion crate (it brings in `azure_storage_blob`, which you use to build the container client):

```diff
 [dependencies]
+ azure_messaging_eventhubs_checkpointstore_blob = "0.9"
+ azure_storage_blob = "1.0"
```

You will also typically want `futures` for the `StreamExt` trait when iterating received events:

```toml
futures = "0.3"
```

### use Statements and Module Layout

The official crate exports its primary types from the crate root, with `models`, `error`, and `processor` submodules for supporting types:

```rust ignore
// Before: azeventhubs
// use azeventhubs::producer::{EventHubProducerClient, EventHubProducerClientOptions, SendEventOptions};
// use azeventhubs::consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions};

// After: azure_messaging_eventhubs
use azure_messaging_eventhubs::{
    ConsumerClient, ProducerClient, SendEventOptions,
    OpenReceiverOptions, StartLocation, StartPosition,
    EventProcessor, CheckpointStore,
};
use azure_messaging_eventhubs::models::EventData;
use azure_messaging_eventhubs::error::ErrorKind;
use azure_identity::DeveloperToolsCredential;
use futures::StreamExt;
```

### Authentication

This is the most significant change. `azeventhubs` parses a connection string (which embeds a shared access key) and the host is part of that string. The official crate takes a fully qualified namespace host and an `azure_identity` credential, so authentication flows through Microsoft Entra ID.

```rust ignore
// Before: azeventhubs - connection string carries both host and key
// let producer = EventHubProducerClient::new_from_connection_string(
//     "Endpoint=sb://my-ns.servicebus.windows.net/;SharedAccessKeyName=...;SharedAccessKey=...",
//     "my-eventhub".to_string(),
//     EventHubProducerClientOptions::default(),
// ).await?;

// After: azure_messaging_eventhubs - host + Entra ID credential, no embedded secret
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;

let credential = DeveloperToolsCredential::new(None)?;
let producer = ProducerClient::builder()
    .open(
        "my-ns.servicebus.windows.net", // fully qualified namespace host
        "my-eventhub",                   // Event Hub name
        credential.clone(),
    )
    .await?;
```

`DeveloperToolsCredential` picks up your Azure CLI login (`az login`) and is appropriate for local development. In production, prefer a managed identity (`ManagedIdentityCredential`) or another specific credential type. See the [`azure_identity`](https://aka.ms/azsdk/rust/identity/docs) documentation for the full set of credentials. If you must keep using a connection string during migration, the recommended replacement for shared access keys is to grant your application identity the **Azure Event Hubs Data Sender** or **Data Receiver** role on the namespace.

### Client Construction

`azeventhubs` constructs clients with associated `new_from_connection_string` functions. The official crate uses a builder that ends in an `.open(...)` call, which establishes the AMQP connection. The builder is where you set options that `azeventhubs` passed through `*Options` structs.

```rust ignore
// Before: azeventhubs
// let consumer = EventHubConsumerClient::new_from_connection_string(
//     "$Default",                 // consumer group
//     "<CONNECTION_STRING>",
//     "my-eventhub".to_string(),
//     EventHubConsumerClientOptions::default(),
// ).await?;

// After: azure_messaging_eventhubs
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ConsumerClient;

let credential = DeveloperToolsCredential::new(None)?;
let consumer = ConsumerClient::builder()
    .with_consumer_group("$Default".to_string())
    .with_application_id("my-app".to_string())
    .open("my-ns.servicebus.windows.net", "my-eventhub", credential.clone())
    .await?;
```

Builder methods available before `.open(...)` include `with_consumer_group`, `with_application_id`, `with_instance_id`, `with_retry_options`, and `with_custom_endpoint` on the consumer; the producer builder offers `with_application_id`, `with_retry_options`, and `with_custom_endpoint`.

## Common Scenarios

The snippets below assume you are inside an `async` function and that errors propagate with `?`. See [Async Runtime and Concurrency](#async-runtime-and-concurrency) for the surrounding `#[tokio::main]` setup.

### Producing Events

In `azeventhubs` you build an `EventData` and call `send_event` with `SendEventOptions`. The official crate keeps the `send_event` name and a `SendEventOptions` struct, but accepts anything convertible into `EventData` (a string, a byte vector, or an `EventData` built with the builder), and partition targeting moves into `SendEventOptions.partition_id`.

```rust ignore
// Before: azeventhubs
// let event = EventData::from("Hello, Event Hub!");
// producer.send_event(event, SendEventOptions::default()).await?;

// After: azure_messaging_eventhubs
use azure_messaging_eventhubs::{models::EventData, ProducerClient, SendEventOptions};

async fn produce(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    // Send a string to a random partition.
    producer.send_event("Hello, Event Hub!", None).await?;

    // Send raw bytes to a specific partition.
    producer
        .send_event(
            vec![2, 4, 8, 16],
            Some(SendEventOptions {
                partition_id: Some("0".to_string()),
            }),
        )
        .await?;

    // Build an event with properties using the EventData builder.
    let event = EventData::builder()
        .with_content_type("text/plain".to_string())
        .with_body("This is some text")
        .add_property("source".to_string(), "sensor-7")
        .build();
    producer.send_event(event, None).await?;

    Ok(())
}
```

### Producing Events in a Batch

The official crate creates a batch from the producer, adds events to it with `try_add_event_data` (which returns `false` when the batch is full rather than erroring), then sends the whole batch in one call.

```rust ignore
// After: azure_messaging_eventhubs
use azure_messaging_eventhubs::{EventDataBatchOptions, ProducerClient};

async fn produce_batch(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some("0".to_string()),
            ..Default::default()
        }))
        .await?;

    // try_add_event_data returns false if the event does not fit in the batch.
    if batch.try_add_event_data("first event", None)? {
        // added successfully
    }
    batch.try_add_event_data(vec![1, 2, 3, 4], None)?;

    producer.send_batch(batch, None).await?;
    Ok(())
}
```

### Consuming Events from a Partition

When you already know which partition to read, open a receiver on it and iterate the event stream. This maps directly to `azeventhubs`'s `read_events_from_partition`. The start position changes from `EventPosition` to a `StartPosition` whose `location` is a `StartLocation` (for example `StartLocation::Earliest`).

```rust ignore
// Before: azeventhubs
// let mut stream = consumer
//     .read_events_from_partition("0", EventPosition::earliest(), ReadEventOptions::default())
//     .await?;
// while let Some(event) = stream.next().await {
//     let event = event?;
//     // ... process event.body() ...
// }

// After: azure_messaging_eventhubs
use azure_messaging_eventhubs::{ConsumerClient, OpenReceiverOptions, StartLocation, StartPosition};
use futures::StreamExt;

async fn consume(consumer: &ConsumerClient) -> Result<(), Box<dyn std::error::Error>> {
    let receiver = consumer
        .open_receiver_on_partition(
            "0".to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let mut stream = receiver.stream_events();
    while let Some(event) = stream.next().await {
        let event = event?; // Result<ReceivedEventData, EventHubsError>
        if let Some(body) = event.event_data().body() {
            println!("received {} bytes", body.len());
        }
        println!("offset {:?}, sequence {:?}", event.offset(), event.sequence_number());
    }

    Ok(())
}
```

`ReceivedEventData` exposes accessors such as `event_data()` (the underlying `EventData`, whose `body()` returns `Option<&[u8]>`), `offset()`, `sequence_number()`, `partition_key()`, `enqueued_time()`, and `raw_amqp_message()`.

### Scalable Consumption with EventProcessor

This capability has no `azeventhubs` equivalent. `EventProcessor` balances partitions across multiple processor instances and records progress in a `CheckpointStore`, so you can run several consumers and let them divide the partitions among themselves. Build it from a `ConsumerClient` and a checkpoint store, run it (typically on a background task), then pull `PartitionClient`s as partitions are assigned to this instance.

```rust ignore
// After: azure_messaging_eventhubs (requires the "in_memory_checkpoint_store" feature for InMemoryCheckpointStore)
use azure_messaging_eventhubs::{
    ConsumerClient, EventProcessor, InMemoryCheckpointStore,
};
use azure_identity::DeveloperToolsCredential;
use futures::StreamExt;
use std::sync::Arc;

async fn process() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let consumer = ConsumerClient::builder()
        .open("my-ns.servicebus.windows.net", "my-eventhub", credential.clone())
        .await?;

    let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;

    // Run the processor's load balancer in the background.
    let runner = processor.clone();
    let handle = tokio::spawn(async move { runner.run().await });

    // Acquire a partition assigned to this instance and process its events.
    let partition_client = processor.next_partition_client().await?;
    println!("processing partition {}", partition_client.get_partition_id());

    let mut stream = partition_client.stream_events().take(100);
    while let Some(event) = stream.next().await {
        let event = event?;
        // ... handle the event ...
        // Record progress so another instance can resume from here.
        partition_client.update_checkpoint(&event).await?;
    }

    processor.shutdown().await?;
    handle.await??;
    Ok(())
}
```

`InMemoryCheckpointStore` is convenient for samples and tests but does not survive a restart. For production, use the blob checkpoint store below.

### Checkpoint Store Setup (Blob Storage)

For durable checkpoints shared across instances, use `azure_messaging_eventhubs_checkpointstore_blob`. You construct an `azure_storage_blob::BlobServiceClient`, derive a container client, and wrap it in a `BlobCheckpointStore`. The container should already exist.

```rust ignore
// After: azure_messaging_eventhubs + azure_messaging_eventhubs_checkpointstore_blob
use azure_core::http::Url;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::BlobServiceClient;

async fn process_with_blob_checkpoints() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;

    // Build the blob container client that will hold checkpoint and ownership blobs.
    let service_url = Url::parse("https://mystorageaccount.blob.core.windows.net")?;
    let service_client = BlobServiceClient::new(service_url, Some(credential.clone()), None)?;
    let container_client = service_client.blob_container_client("eventhubs-checkpoints");

    let checkpoint_store = BlobCheckpointStore::new(container_client);

    let consumer = ConsumerClient::builder()
        .with_consumer_group("$Default".to_string())
        .open("my-ns.servicebus.windows.net", "my-eventhub", credential.clone())
        .await?;

    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;

    // Run the processor; checkpoints now persist in Azure Blob Storage.
    processor.run().await?;
    Ok(())
}
```

The same Entra ID credential authenticates both Event Hubs and Blob Storage. Grant the application identity the **Storage Blob Data Contributor** role on the storage account so it can read and write checkpoint blobs.

## Error Handling

`azure_messaging_eventhubs` defines its own error type, `EventHubsError`, exposed through the crate's `Result<T>` alias (`azure_messaging_eventhubs::Result<T>`). Match on the public `ErrorKind` via the error's `kind` field. Note that `kind` is a field, not a method.

```rust ignore
use azure_messaging_eventhubs::error::ErrorKind;

match consumer.open_receiver_on_partition("0".to_string(), None).await {
    Ok(receiver) => { /* ... */ }
    Err(err) => match err.kind {
        // The broker disconnected this receiver because another consumer attached
        // with the same or higher owner level (epoch). Re-acquire the partition.
        ErrorKind::ConsumerDisconnected(_) => {
            eprintln!("partition reassigned to another consumer");
        }
        // The service rejected a send (for example, a quota was exceeded).
        ErrorKind::SendRejected(details) => {
            eprintln!("send rejected: {details:?}");
        }
        // An error surfaced from azure_core (HTTP, credential, etc.).
        ErrorKind::AzureCore(ref e) => eprintln!("core error: {e}"),
        // An AMQP transport error.
        ErrorKind::AmqpError(ref e) => eprintln!("amqp error: {e:?}"),
        other => eprintln!("other error: {other:?}"),
    },
}
```

`ErrorKind` is `#[non_exhaustive]`, so always include a catch-all arm. `EventHubsError` converts to and from `azure_core::Error`, so it composes with code that works in terms of `azure_core::Result`. When iterating an event stream, each item is a `Result<ReceivedEventData, EventHubsError>`; the `ConsumerDisconnected` variant is the signal that a partition was reassigned and you should re-acquire it through `EventProcessor::next_partition_client`.

## Async Runtime and Concurrency

Like `azeventhubs`, `azure_messaging_eventhubs` is fully async and runs on the [tokio](https://tokio.rs) runtime. All client operations are `async`.

```rust ignore
// Cargo.toml:
//
// [dependencies]
// azure_messaging_eventhubs = "0.15"
// azure_identity = "1.0"
// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
// futures = "0.3"

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let producer = ProducerClient::builder()
        .open("my-ns.servicebus.windows.net", "my-eventhub", credential.clone())
        .await?;

    producer.send_event("Hello, Event Hub!", None).await?;
    producer.close().await?;
    Ok(())
}
```

Clients are `Send + Sync` and can be shared across tasks. The `EventProcessor` is designed to be cloned (typically wrapped in `Arc`) so its load balancer can run on one task while you consume `PartitionClient`s on another, as shown in [Scalable Consumption](#scalable-consumption-with-eventprocessor).

## Feature Flags

| Crate | Feature | Default | Description |
| --- | --- | --- | --- |
| `azure_messaging_eventhubs` | `in_memory_checkpoint_store` | No | Enables `InMemoryCheckpointStore`, a non-durable checkpoint store useful for tests and samples. |
| `azure_messaging_eventhubs` | `default` | Yes | Enables `azure_core_amqp/default`. |

To use the in-memory checkpoint store:

```toml
azure_messaging_eventhubs = { version = "0.15", features = ["in_memory_checkpoint_store"] }
```

The TLS backend is selected through `azure_core`'s transport features (for example a `reqwest`-based stack). If you need a specific TLS configuration, configure it on `azure_core` / the HTTP client as you would for any Azure SDK for Rust crate.

## FAQ and Common Pitfalls

**Where did the connection string go?**
The official crate authenticates with Microsoft Entra ID, not shared access keys. Pass the fully qualified namespace host (for example `my-ns.servicebus.windows.net`, the part after `Endpoint=sb://` in your old connection string) plus an `azure_identity` credential. Assign the **Azure Event Hubs Data Sender** / **Data Receiver** role to your identity to replace key-based access.

**I passed my whole connection string as the host and it failed.**
`open(...)` expects only the host (`my-ns.servicebus.windows.net`), not the full `Endpoint=sb://...;SharedAccessKey=...` string. Strip everything except the namespace host.

**My consumer stopped with a `ConsumerDisconnected` error.**
That is expected when another consumer attaches to the same partition with an equal or higher owner level (epoch), which is exactly how `EventProcessor` reassigns partitions during load balancing. Treat it as a signal to re-acquire a partition via `EventProcessor::next_partition_client` rather than as a fatal error. This has no direct `azeventhubs` analogue because `azeventhubs` had no processor.

**Does the official crate have a buffered producer like the one `azeventhubs` planned?**
Use batches (`create_batch` / `try_add_event_data` / `send_batch`) to group events into a single network request. This gives you explicit control over batching without a separate buffered-producer type.

**How do I get checkpointing that survives restarts?**
`InMemoryCheckpointStore` is in-process only. Add `azure_messaging_eventhubs_checkpointstore_blob` and use `BlobCheckpointStore` so checkpoints and partition ownership persist in Azure Blob Storage and are shared across instances. See [Checkpoint Store Setup](#checkpoint-store-setup-blob-storage).

**`err.kind()` does not compile.**
On `EventHubsError`, `kind` is a public field, not a method. Match on `err.kind`, not `err.kind()`. (This differs from `azure_core::Error`, where `kind()` is a method.)

**Why does `EventData` no longer implement everything it used to?**
The official `EventData` is a distinct type from `azeventhubs`'s. Construct it with `EventData::builder()...build()`, or rely on the `Into<EventData>` conversions for strings and byte vectors when calling `send_event`.

## Additional Resources

- [azure_messaging_eventhubs README](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/README.md)
- [Examples directory](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/examples)
- [Companion crate: azure_messaging_eventhubs_checkpointstore_blob](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs_checkpointstore_blob)
- [API documentation on docs.rs](https://docs.rs/azure_messaging_eventhubs/latest/)
- [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/CHANGELOG.md)
- [azure_identity documentation](https://aka.ms/azsdk/rust/identity/docs)
- [Azure Event Hubs product documentation](https://learn.microsoft.com/azure/event-hubs/)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
</content>
