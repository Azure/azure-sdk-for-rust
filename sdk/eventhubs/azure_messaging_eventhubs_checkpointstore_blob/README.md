# Azure Event Hubs Checkpoint Store for Blob Storage

<!-- cspell: ignore myapp -->

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

```rust
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_messaging_eventhubs::processor::EventProcessorBuilder;
use azure_storage_blobs::BlobServiceClient;
use azure_identity::DefaultAzureCredential;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create blob service client
    let credential = Arc::new(DefaultAzureCredential::default());
    let blob_client = BlobServiceClient::new(
        "https://yourstorageaccount.blob.core.windows.net",
        credential.clone()
    );

    // Create checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(
        blob_client,
        "checkpoints".to_string(), // container name
        Some("myapp".to_string())   // optional prefix
    );

    // Create event processor with blob checkpoint store
    let processor = EventProcessorBuilder::new()
        .with_connection_string("your_eventhubs_connection_string")
        .with_consumer_group("$Default")
        .with_checkpoint_store(Arc::new(checkpoint_store))
        .with_event_handler(|event| async move {
            println!("Received event: {:?}", event);
            Ok(())
        })
        .build()
        .await?;

    // Start processing
    processor.start().await?;

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

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE.txt) file for details.
