# azure_messaging_eventhubs_checkpointstore_blob

- **Description**: Azure Event Hubs checkpoint store implementation using Azure Blob Storage
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
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
    impl CheckpointStore for BlobCheckpointStore {
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn claim_ownership(&self, ownerships: &[Ownership]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn list_checkpoints(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Checkpoint>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn list_ownerships(&self, namespace: &str, event_hub_name: &str, consumer_group: &str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Ownership>>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn update_checkpoint(&self, checkpoint: Checkpoint) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    }
}
```
