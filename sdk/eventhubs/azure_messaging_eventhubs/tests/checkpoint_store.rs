// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

//! # Event Hubs SDK for Rust
//!
//!

mod common;
mod in_memory_checkpoint_store;

use azure_core::error::ErrorKind as AzureErrorKind;
use azure_messaging_eventhubs::CheckpointStore;
use in_memory_checkpoint_store::InMemoryCheckpointStore;
use std::sync::Arc;

#[cfg(test)]
mod tests {

    use super::*;
    use azure_messaging_eventhubs::models::{Checkpoint, Ownership};
    use tracing::info;

    #[test]
    fn test_update_ownership() {
        common::setup();
        let store = InMemoryCheckpointStore::new();
        let ownership = Ownership::new(
            "namespace",
            "event_hub",
            "consumer_group",
            "partition_id",
            "owner_id",
            Some("etag".into()),
            None,
        );
        let result = store.update_ownership(&ownership);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_ownership_invalid() {
        common::setup();
        let store = InMemoryCheckpointStore::new();
        let ownership = Ownership::new(
            "namespace",
            "event_hub",
            "consumer_group",
            "partition_id",
            "owner_id",
            Some("etag".into()),
            None,
        );
        let result = store.update_ownership(&ownership);
        assert!(result.is_err());
        assert_eq!(*result.unwrap_err().kind(), AzureErrorKind::Other);
    }

    #[tokio::test]
    async fn test_update_checkpoint() {
        common::setup();
        let store = InMemoryCheckpointStore::new();
        let checkpoint = Checkpoint::new(
            "namespace",
            "event_hub",
            "consumer_group",
            "partition_id",
            None,
            None,
        );
        let result = store.update_checkpoint(checkpoint).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_checkpoints() {
        common::setup();
        let store = InMemoryCheckpointStore::new();
        let checkpoint = Checkpoint::new(
            "namespace",
            "event_hub",
            "consumer_group",
            "partition_id",
            None,
            None,
        );
        info!("Adding checkpoint: {checkpoint:?}");
        store.update_checkpoint(checkpoint).await.unwrap();

        let checkpoints = store
            .list_checkpoints("namespace", "event_hub", "consumer_group", None)
            .await
            .unwrap();

        info!("List checkpoints: {checkpoints:?}");
        assert_eq!(checkpoints.len(), 1);
    }

    fn get_random_name(prefix: &str) -> String {
        format!("{}{}", prefix, azure_core::Uuid::new_v4())
    }

    #[tokio::test]
    async fn checkpoints() -> azure_core::Result<()> {
        common::setup();
        let test_name = get_random_name("checkpoint");

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
        let checkpoints = checkpoint_store
            .list_checkpoints(
                "fully-qualified-namespace",
                "event-hub-name",
                "consumer-group",
                None,
            )
            .await
            .unwrap();
        assert_eq!(checkpoints.len(), 0);

        let checkpoint = Checkpoint::new(
            "ns.servicebus.windows.net",
            "event-hub-name",
            "consumer-group",
            test_name.clone().as_str(),
            Some("offset".to_string()),
            Some(0),
        );

        // Even though we added a checkpoint in one namespace, it doesn't change the older namespace.
        checkpoint_store
            .update_checkpoint(checkpoint.clone())
            .await
            .unwrap();
        let checkpoints = checkpoint_store
            .list_checkpoints(
                "fully-qualified-namespace",
                "event-hub-name",
                "consumer-group",
                None,
            )
            .await
            .unwrap();
        assert_eq!(checkpoints.len(), 0);

        let checkpoints = checkpoint_store
            .list_checkpoints(
                "ns.servicebus.windows.net",
                "event-hub-name",
                "consumer-group",
                None,
            )
            .await;
        assert!(checkpoints.is_ok());
        let checkpoints = checkpoints.unwrap();
        assert_eq!(checkpoints.len(), 1);
        assert_eq!(checkpoints[0].partition_id(), test_name.as_str());
        assert_eq!(checkpoints[0].offset(), Some("offset"));
        assert_eq!(checkpoints[0].sequence_number(), Some(0));
        assert_eq!(checkpoints[0].event_hub_name(), "event-hub-name");
        assert_eq!(checkpoints[0].consumer_group(), "consumer-group");

        Ok(())
    }
}
