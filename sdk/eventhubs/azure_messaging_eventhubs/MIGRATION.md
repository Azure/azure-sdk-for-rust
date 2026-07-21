<!-- cspell:words azeventhubs checkpointing minghuaw -->

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
| Construction | `new_from_connection_string(...)` | `ProducerClient::builder().open(host, name, credential)`, or `.open_with_connection_string(...)` |
| Authentication | Connection string (SAS key) | Microsoft Entra ID via `azure_identity` credentials (recommended), or a connection string / SAS |
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

Remove `azeventhubs` and add the official crate plus `azure_identity`, which supplies the credential types the new client authenticates with:

```diff
 [dependencies]
- azeventhubs = "0.20"
+ azure_messaging_eventhubs = "0.15"
+ azure_identity = "1"
```

Both crates are async and run on [tokio](https://tokio.rs), so your existing runtime dependency carries over unchanged.

If you want scalable consumption with durable checkpoints in Azure Blob Storage, also add the companion crate. You build the container client yourself, so you need `azure_storage_blob` and `azure_core` (for the `Url` type) as direct dependencies too:

```diff
 [dependencies]
+ azure_messaging_eventhubs_checkpointstore_blob = "0.9"
+ azure_storage_blob = "1"
+ azure_core = "1"
```

You will also typically want `futures` for the `StreamExt` trait when iterating received events:

```toml
futures = "0.3"
```

### use Statements and Module Layout

The official crate exports its primary types from the crate root, with `models`, `error`, and `processor` submodules for supporting types:

```rust ignore use_statements
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

```rust ignore authentication
// Before (azeventhubs): connection string carries both host and key
// let producer = EventHubProducerClient::new_from_connection_string(
//     "Endpoint=sb://my-ns.servicebus.windows.net/;SharedAccessKeyName=...;SharedAccessKey=...",
//     "my-eventhub".to_string(),
//     EventHubProducerClientOptions::default(),
// ).await?;

// After (azure_messaging_eventhubs): host + Entra ID credential, no embedded secret
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

`DeveloperToolsCredential` picks up your Azure CLI login (`az login`) and is appropriate for local development. In production, prefer a managed identity (`ManagedIdentityCredential`) or another specific credential type. See the [`azure_identity`](https://aka.ms/azsdk/rust/identity/docs) documentation for the full set of credentials. To replace key-based access, grant your application identity the **Azure Event Hubs Data Sender** or **Data Receiver** role on the namespace.

#### Keeping a Connection String

You do not have to change the authentication model and the client code in one step. Both builders also accept an Event Hubs connection string, which lets you port the API surface first and move to Entra ID afterwards:

```rust ignore connection_string
use azure_messaging_eventhubs::ProducerClient;

async fn open_with_sas() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = std::env::var("EVENTHUBS_CONNECTION_STRING")?;

    // The Event Hub name is optional when the connection string carries an
    // `EntityPath`; if you pass both, they must agree.
    let producer = ProducerClient::builder()
        .open_with_connection_string(&connection_string, Some("my-eventhub"))
        .await?;

    producer.close().await?;
    Ok(())
}
```

`ConsumerClient::builder()` exposes the same method. When the connection string carries a `SharedAccessKeyName` and `SharedAccessKey`, the client signs and refreshes SAS tokens for you. When it carries a pre-formed `SharedAccessSignature`, that token is used as-is and cannot be refreshed, so the broker drops the link once the token expires. Entra ID remains the recommendation for production because it avoids storing a secret in your configuration.

### Client Construction

`azeventhubs` constructs clients with associated `new_from_connection_string` functions. The official crate uses a builder that ends in an `.open(...)` call, which establishes the AMQP connection. The builder is where you set options that `azeventhubs` passed through `*Options` structs.

```rust ignore client_construction
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
    // Note: ConsumerClient::open takes the event hub name as an owned String,
    // whereas ProducerClient::open takes &str.
    .open("my-ns.servicebus.windows.net", "my-eventhub".to_string(), credential.clone())
    .await?;
```

Builder methods available before `.open(...)` include `with_consumer_group`, `with_application_id`, `with_instance_id`, `with_retry_options`, and `with_custom_endpoint` on the consumer; the producer builder offers `with_application_id`, `with_retry_options`, and `with_custom_endpoint`.

## Common Scenarios

The snippets below assume you are inside an `async` function and that errors propagate with `?`. See [Async Runtime and Concurrency](#async-runtime-and-concurrency) for the surrounding `#[tokio::main]` setup.

### Producing Events

In `azeventhubs` you build an `EventData` and call `send_event` with `SendEventOptions`. The official crate keeps the `send_event` name and a `SendEventOptions` struct, but accepts anything convertible into `EventData` (a string, a byte vector, or an `EventData` built with the builder), and partition targeting moves into `SendEventOptions.partition_id`.

```rust ignore producing
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

```rust ignore batch
// After: azure_messaging_eventhubs
use azure_messaging_eventhubs::{EventDataBatchOptions, ProducerClient};

async fn produce_batch(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    // Build the options from a closure so an overflow batch can target the same
    // partition. EventDataBatchOptions is not Clone.
    let batch_options = || EventDataBatchOptions {
        partition_id: Some("0".to_string()),
        ..Default::default()
    };

    let batch = producer.create_batch(Some(batch_options())).await?;

    // try_add_event_data returns false when the event does not fit, which means
    // the batch is full. Never ignore it, or the event is silently dropped.
    if !batch.try_add_event_data("first event", None)? {
        return Err("event does not fit in an empty batch".into());
    }

    if !batch.try_add_event_data(vec![1, 2, 3, 4], None)? {
        // The batch is full. Send what you have, start a new batch with the same
        // options, and add the event that did not fit to it.
        producer.send_batch(batch, None).await?;

        let batch = producer.create_batch(Some(batch_options())).await?;
        if !batch.try_add_event_data(vec![1, 2, 3, 4], None)? {
            return Err("event does not fit in an empty batch".into());
        }
        producer.send_batch(batch, None).await?;
        return Ok(());
    }

    producer.send_batch(batch, None).await?;
    Ok(())
}
```

### Consuming Events from a Partition

When you already know which partition to read, open a receiver on it and iterate the event stream. This maps directly to `azeventhubs`'s `read_events_from_partition`. The start position changes from `EventPosition` to a `StartPosition` whose `location` is a `StartLocation` (for example `StartLocation::Earliest`).

```rust ignore consuming
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

```rust ignore processor
// After: azure_messaging_eventhubs (requires the "in_memory_checkpoint_store" feature for InMemoryCheckpointStore)
use azure_messaging_eventhubs::{
    error::ErrorKind, processor::PartitionClient, ConsumerClient, EventProcessor,
    InMemoryCheckpointStore,
};
use azure_identity::DeveloperToolsCredential;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::sync::Arc;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

async fn process() -> Result<(), BoxError> {
    let credential = DeveloperToolsCredential::new(None)?;
    let consumer = ConsumerClient::builder()
        .open("my-ns.servicebus.windows.net", "my-eventhub".to_string(), credential.clone())
        .await?;

    let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
    // `build` returns an Arc<EventProcessor>, so clone the handle to share it.
    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;

    // Run the processor's load balancer in the background.
    let runner = processor.clone();
    let mut handle = tokio::spawn(async move { runner.run().await });

    // Watch the load balancer while consuming. If `run()` stops early, waiting
    // on the next assignment would otherwise block forever.
    let result = tokio::select! {
        runner_result = &mut handle => runner_result?.map_err(BoxError::from),
        consume_result = process_partitions(&processor) => consume_result,
    };

    // Shut the processor down on either outcome, so the load balancer always
    // stops and the task is joined.
    processor.shutdown().await?;
    if !handle.is_finished() {
        handle.await??;
    }
    result
}

async fn process_partitions(processor: &EventProcessor) -> Result<(), BoxError> {
    // Partitions are assigned over time and each stream is long-lived, so keep
    // taking new assignments while the partitions you already hold keep running.
    // `stream_events()` is not `Send`, so drive them on this task with
    // `FuturesUnordered` rather than `tokio::spawn`.
    let mut partitions = FuturesUnordered::new();

    loop {
        tokio::select! {
            partition_client = processor.next_partition_client() => {
                partitions.push(process_partition(partition_client?));
            }
            // Surface failures from partitions that finished, for example after
            // a reassignment.
            Some(finished) = partitions.next(), if !partitions.is_empty() => {
                finished?;
            }
        }
    }
}

async fn process_partition(partition_client: Arc<PartitionClient>) -> Result<(), BoxError> {
    println!("processing partition {}", partition_client.get_partition_id());

    let mut stream = partition_client.stream_events();
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                // ... handle the event ...
                // Record progress so another instance can resume from here.
                partition_client.update_checkpoint(&event).await?;
            }
            // Load balancing moved this partition to another instance. Stop
            // using this client and let the task end; the processor hands out
            // the next assignment through `next_partition_client`. This is a
            // normal reassignment, not a failure.
            Err(err) if matches!(err.kind, ErrorKind::ConsumerDisconnected(_)) => break,
            Err(err) => return Err(err.into()),
        }
    }

    Ok(())
}
```

`InMemoryCheckpointStore` is convenient for samples and tests but does not survive a restart. For production, use the blob checkpoint store below.

### Checkpoint Store Setup (Blob Storage)

For durable checkpoints shared across instances, use `azure_messaging_eventhubs_checkpointstore_blob`. You construct an `azure_storage_blob::BlobServiceClient`, derive a container client, and wrap it in a `BlobCheckpointStore`. The container should already exist.

```rust ignore checkpoint_store
// After: azure_messaging_eventhubs + azure_messaging_eventhubs_checkpointstore_blob
use azure_core::http::Url;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::BlobServiceClient;
use std::sync::Arc;

// Returns the configured processor; run and consume it as shown above.
async fn build_processor_with_blob_checkpoints(
) -> Result<Arc<EventProcessor>, Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;

    // Build the blob container client that will hold checkpoint and ownership blobs.
    let service_url = Url::parse("https://mystorageaccount.blob.core.windows.net")?;
    let service_client = BlobServiceClient::new(service_url, Some(credential.clone()), None)?;
    let container_client = service_client.blob_container_client("eventhubs-checkpoints");

    let checkpoint_store = BlobCheckpointStore::new(container_client);

    let consumer = ConsumerClient::builder()
        .with_consumer_group("$Default".to_string())
        .open("my-ns.servicebus.windows.net", "my-eventhub".to_string(), credential.clone())
        .await?;

    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;

    // Only the checkpoint store changed. Run the processor and consume its
    // partition clients exactly as in the previous scenario; the checkpoints
    // that `update_checkpoint` writes now persist in Azure Blob Storage.
    Ok(processor)
}
```

The same Entra ID credential authenticates both Event Hubs and Blob Storage. Grant the application identity the **Storage Blob Data Contributor** role on the storage account so it can read and write checkpoint blobs.

## Error Handling

`azure_messaging_eventhubs` defines its own error type, `EventHubsError`, exposed through the crate's `Result<T>` alias (`azure_messaging_eventhubs::Result<T>`). Match on the public `ErrorKind` via the error's `kind` field. Note that `kind` is a field, not a method.

Match where the error actually surfaces. `ConsumerDisconnected` is reported by the receive path, so it arrives as a stream item rather than from the call that opened the receiver:

```rust ignore error_handling
use azure_messaging_eventhubs::error::ErrorKind;
use futures::StreamExt;

let receiver = consumer.open_receiver_on_partition("0".to_string(), None).await?;
let mut stream = receiver.stream_events();

while let Some(event) = stream.next().await {
    match event {
        Ok(_event) => { /* ... process the event ... */ }
        Err(err) => match err.kind {
            // The broker disconnected this receiver because another consumer
            // attached with the same or higher owner level (epoch). Stop using
            // this receiver and re-acquire the partition.
            ErrorKind::ConsumerDisconnected(_) => {
                eprintln!("partition reassigned to another consumer");
                break;
            }
            // An error surfaced from azure_core (HTTP, credential, etc.).
            ErrorKind::AzureCore(ref e) => eprintln!("core error: {e}"),
            // An AMQP transport error.
            ErrorKind::AmqpError(ref e) => eprintln!("amqp error: {e:?}"),
            other => eprintln!("other error: {other:?}"),
        },
    }
}
```

`SendRejected` comes from the send path instead, so match it on the producer call:

```rust ignore send_rejected
use azure_messaging_eventhubs::error::ErrorKind;

if let Err(err) = producer.send_event("Hello, Event Hub!", None).await {
    match err.kind {
        // The service rejected the send, for example because a quota was exceeded.
        ErrorKind::SendRejected(ref details) => eprintln!("send rejected: {details:?}"),
        other => eprintln!("other error: {other:?}"),
    }
}
```

`ErrorKind` is `#[non_exhaustive]`, so always include a catch-all arm. `EventHubsError` converts to and from `azure_core::Error`, so it composes with code that works in terms of `azure_core::Result`. When iterating an event stream, each item is a `Result<ReceivedEventData, EventHubsError>`; the `ConsumerDisconnected` variant is the signal that a partition was reassigned and you should re-acquire it through `EventProcessor::next_partition_client`.

## Async Runtime and Concurrency

Like `azeventhubs`, `azure_messaging_eventhubs` is fully async and runs on the [tokio](https://tokio.rs) runtime. All client operations are `async`.

```rust ignore async_runtime
// Cargo.toml:
//
// [dependencies]
// azure_messaging_eventhubs = "0.15"
// azure_identity = "1"
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

Clients are `Send + Sync` and can be shared across tasks. `EventProcessorBuilder::build` already returns an `Arc<EventProcessor>`, so you clone that handle rather than wrapping the processor yourself. That lets the load balancer run on one task while you consume `PartitionClient`s on another, as shown in [Scalable Consumption](#scalable-consumption-with-eventprocessor).

## Feature Flags

| Crate | Feature | Default | Description |
| --- | --- | --- | --- |
| `azure_messaging_eventhubs` | `in_memory_checkpoint_store` | No | Enables `InMemoryCheckpointStore`, a non-durable checkpoint store useful for tests and samples. |
| `azure_messaging_eventhubs` | `default` | Yes | Enables `azure_core_amqp/default`. |

To use the in-memory checkpoint store:

```toml
azure_messaging_eventhubs = { version = "0.15", features = ["in_memory_checkpoint_store"] }
```

Event Hubs traffic runs over AMQP, not HTTP, so the TLS backend comes from the AMQP stack rather than from `azure_core`'s HTTP transport features. The `default` feature enables `azure_core_amqp/default`, which selects `fe2o3-amqp/native-tls`. To use a different backend, disable default features and enable the one you want on `azure_core_amqp` directly.

## FAQ and Common Pitfalls

**Where did the connection string go?**
It is still supported. Call `open_with_connection_string(...)` on either builder to keep your existing connection string, which is the quickest way to port the code without also changing how you authenticate. The official crate recommends Microsoft Entra ID instead: pass the fully qualified namespace host (for example `my-ns.servicebus.windows.net`, the part after `Endpoint=sb://` in your old connection string) plus an `azure_identity` credential to `open(...)`, and assign the **Azure Event Hubs Data Sender** / **Data Receiver** role to your identity to replace key-based access. See [Keeping a Connection String](#keeping-a-connection-string).

**I passed my whole connection string as the host and it failed.**
`open(...)` expects only the host (`my-ns.servicebus.windows.net`), not the full `Endpoint=sb://...;SharedAccessKey=...` string. Strip everything except the namespace host, or call `open_with_connection_string(...)` instead, which parses the full string for you.

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

- [azure_messaging_eventhubs README](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/eventhubs/azure_messaging_eventhubs/README.md)
- [Examples directory](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/examples)
- [Companion crate: azure_messaging_eventhubs_checkpointstore_blob](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs_checkpointstore_blob)
- [API documentation on docs.rs](https://docs.rs/azure_messaging_eventhubs/latest/)
- [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/eventhubs/azure_messaging_eventhubs/CHANGELOG.md)
- [azure_identity documentation](https://aka.ms/azsdk/rust/identity/docs)
- [Azure Event Hubs product documentation](https://learn.microsoft.com/azure/event-hubs/)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
