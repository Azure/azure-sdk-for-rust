# Azure Event Hubs Checkpoint Store for Blob Storage

This crate provides a checkpoint store implementation for Azure Event Hubs using Azure Blob Storage as the backend. It implements the `CheckpointStore` trait from the `azure_messaging_eventhubs` crate, allowing you to persist checkpoints (event positions) to Azure Blob Storage.

## Features

- **Persistent Checkpoints**: Store event processing positions in Azure Blob Storage
- **High Availability**: Leverage Azure Blob Storage's durability and availability
- **Concurrency Support**: Handle multiple processors with proper concurrency control
- **Easy Integration**: Drop-in replacement for other checkpoint store implementations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
azure_messaging_eventhubs_checkpointstore_blob = "0.1.0"
azure_messaging_eventhubs = "0.20.0"
azure_storage_blobs = "0.20.0"
azure_identity = "0.20.0"
```

### Basic Example

This example creates a blob container client on the storage account which will hold the blob checkpoint store, and configures a blob checkpoint store to use that storage client.

It then creates an EventHubs processor client using the blob checkpoint store and starts the processor.

```rust no_run
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor, ProcessorStrategy};
use azure_storage_blob::BlobContainerClient;
use azure_identity::DefaultAzureCredential;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create blob service client
    let credential = DefaultAzureCredential::new()?;
    let blob_client = BlobContainerClient::new(
        "https://yourstorageaccount.blob.core.windows.net",
        "yourcontainername".to_string(),
        credential.clone(),
        None,
    )?;

    // Create checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(blob_client);

    let consumer_client = ConsumerClient::builder()
        .open(
            "my-eventhubs-host-name",
            "my-eventhub-name".to_string(),
            credential.clone(),
        )
        .await?;

    let event_processor = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Greedy)
        .build(
            Arc::new(consumer_client),
            checkpoint_store,
        )
        .await?;

    // Start processing
    tokio::spawn(async move { event_processor.run().await });

    Ok(())
}
```

## Container Structure

The checkpoint store creates blobs in the following structure:

```text
container/
├── prefix/
│   ├── eventhub-name/
│   │   ├── consumer-group/
│   │   │   ├── partition-0
│   │   │   ├── partition-1
│   │   │   └── ...
│   │   └── ownership/
│   │       ├── owner-1.json
│   │       ├── owner-2.json
│   │       └── ...
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `checkpoint_store_basic.rs`: Basic checkpoint operations
- `processor_with_blob_checkpoints.rs`: Complete EventHubs processor setup
