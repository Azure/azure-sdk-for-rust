# azure_messaging_eventhubs_checkpointstore_blob

- **Description**: Azure Event Hubs checkpoint store implementation using Azure Blob Storage
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`
  - `azure_messaging_eventhubs/default`
  - `azure_storage_blob/default`

```rust
#![crate_name = "azure_messaging_eventhubs_checkpointstore_blob"]
#![crate_type = "lib"]
#![recursion_limit = "128"]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![attr = RecursionLimit {limit:128}]
pub use azure_messaging_eventhubs_checkpointstore_blob::checkpoint_store::BlobCheckpointStore;
pub mod checkpoint_store {
    #[derive(Clone)]
    pub struct BlobCheckpointStore {
    }
    impl BlobCheckpointStore {
        pub fn new(blob_container_client: BlobContainerClient) -> Arc<Self>;
    }
    #[async_trait]
    impl CheckpointStore for BlobCheckpointStore {
        async fn claim_ownership(&self, ownerships: &[Ownership]) -> Result<Vec<Ownership>>;
        async fn list_checkpoints(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> Result<Vec<Checkpoint>>;
        async fn list_ownerships(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> Result<Vec<Ownership>>;
        async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()>;
    }
}
```
