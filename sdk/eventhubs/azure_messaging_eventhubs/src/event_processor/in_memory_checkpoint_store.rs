use super::{
    models::{Checkpoint, Ownership},
    processor::{
        CheckpointStore, ClaimOwnershipOptions, ListCheckpointsOptions, ListOwnershipOptions,
    },
};
//use async_trait::async_trait;
use async_trait::async_trait;
use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::debug;

pub struct InMemoryCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, Checkpoint>>>,
    ownerships: Arc<Mutex<HashMap<String, Ownership>>>,
}

impl InMemoryCheckpointStore {
    pub fn new() -> Self {
        InMemoryCheckpointStore {
            checkpoints: Arc::new(Mutex::new(HashMap::new())),
            ownerships: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn update_ownership(&self, ownership: &Ownership) -> Result<Ownership> {
        if ownership.partition_id.is_empty()
            || ownership.event_hub_name.is_empty()
            || ownership.fully_qualified_namespace.is_empty()
            || ownership.partition_id.is_empty()
        {
            return Err(Error::message(
                AzureErrorKind::Other,
                format!("Ownership is not valid: {:#?}", ownership),
            ));
        }
        let mut store = self.ownerships.lock().unwrap();
        let key = Ownership::get_ownership_name(
            ownership.fully_qualified_namespace.as_str(),
            ownership.event_hub_name.as_str(),
            ownership.consumer_group.as_str(),
            ownership.partition_id.as_str(),
        )?;
        debug!("Update ownership for key {}", key);
        if store.contains_key(&key) {
            if ownership.etag != store.get(&key).unwrap().etag {
                debug!("ETag mismatch {}", key);
                return Err(Error::message(
                    AzureErrorKind::Other,
                    format!("ETag mismatch for partition {}", key),
                ));
            }
            store.insert(key.clone(), ownership.clone());
            Ok(ownership.clone())
        } else {
            Err(Error::message(
                AzureErrorKind::Other,
                format!("Ownership not found for partition {}", key),
            ))
        }
    }
}

#[async_trait]
impl CheckpointStore for InMemoryCheckpointStore {
    async fn claim_ownership<'a>(
        &'a self,
        ownerships: Vec<Ownership>,
        #[allow(unused_variables)] _options: Option<ClaimOwnershipOptions>,
    ) -> Result<Vec<Ownership>> {
        debug!("Claim ownership for {} partitions", ownerships.len());
        let mut claimed_ownerships = Vec::new();
        for ownership in ownerships {
            self.update_ownership(&ownership)?;
            if ownership.etag.is_some() {
                claimed_ownerships.push(ownership);
            }
        }
        Ok(claimed_ownerships)
    }

    async fn list_checkpoints(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        #[allow(unused_variables)] _options: Option<ListCheckpointsOptions>,
    ) -> Result<Vec<Checkpoint>> {
        let store = self.checkpoints.lock().unwrap();
        let prefix =
            Checkpoint::get_checkpoint_blob_prefix_name(namespace, event_hub_name, consumer_group)?;
        debug!("list_checkpoints: list checkpoints for prefix {prefix}");
        let mut checkpoints = Vec::new();
        for (key, value) in store.iter() {
            if key.starts_with(&prefix) {
                checkpoints.push(value.clone());
            }
        }
        debug!("list_checkpoints: found {} checkpoints", checkpoints.len());
        Ok(checkpoints)
    }

    async fn list_ownerships(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        #[allow(unused_variables)] _options: Option<ListOwnershipOptions>,
    ) -> Result<Vec<Ownership>> {
        let store = self.ownerships.lock().unwrap();

        let prefix =
            Ownership::get_ownership_prefix_name(namespace, event_hub_name, consumer_group)?;
        debug!("list_ownerships: list ownerships for prefix {prefix}");
        let mut ownerships = Vec::new();
        ownerships.extend(
            store
                .iter()
                .filter(|(key, _)| key.starts_with(&prefix))
                .map(|(_, value)| value.clone()),
        );
        debug!("list_ownerships: found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()> {
        let mut checkpoints = self.checkpoints.lock().map_err(|e| {
            Error::message(
                AzureErrorKind::Other,
                format!("Failed to lock checkpoint store: {}", e),
            )
        })?;
        checkpoints.insert(
            Checkpoint::get_checkpoint_blob_name(
                checkpoint.fully_qualified_namespace.as_str(),
                checkpoint.event_hub_name.as_str(),
                checkpoint.consumer_group.as_str(),
                checkpoint.partition_id.as_str(),
            )?,
            checkpoint,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::event_processor::models::{Checkpoint, Ownership};

    #[test]
    fn test_update_ownership() {
        let store = InMemoryCheckpointStore::new();
        let ownership = Ownership {
            fully_qualified_namespace: "namespace".to_string(),
            event_hub_name: "event_hub".to_string(),
            consumer_group: "consumer_group".to_string(),
            partition_id: "partition_id".to_string(),
            owner_id: "owner_id".to_string(),
            etag: Some("etag".to_string()),
            ..Default::default()
        };
        let result = store.update_ownership(&ownership);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_ownership_invalid() {
        let store = InMemoryCheckpointStore::new();
        let ownership = Ownership {
            fully_qualified_namespace: "".to_string(),
            event_hub_name: "event_hub".to_string(),
            consumer_group: "consumer_group".to_string(),
            partition_id: "partition_id".to_string(),
            owner_id: "owner_id".to_string(),
            etag: Some("etag".to_string()),
            ..Default::default()
        };
        let result = store.update_ownership(&ownership);
        assert!(result.is_err());
        assert_eq!(*result.unwrap_err().kind(), AzureErrorKind::Other);
    }

    #[tokio::test]
    async fn test_update_checkpoint() {
        let store = InMemoryCheckpointStore::new();
        let checkpoint = Checkpoint {
            fully_qualified_namespace: "namespace".to_string(),
            event_hub_name: "event_hub".to_string(),
            consumer_group: "consumer_group".to_string(),
            partition_id: "partition_id".to_string(),
            ..Default::default()
        };
        let result = store.update_checkpoint(checkpoint).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_checkpoints() {
        let store = InMemoryCheckpointStore::new();
        let checkpoint = Checkpoint {
            fully_qualified_namespace: "namespace".to_string(),
            event_hub_name: "event_hub".to_string(),
            consumer_group: "consumer_group".to_string(),
            partition_id: "partition_id".to_string(),
            ..Default::default()
        };
        store.update_checkpoint(checkpoint).await.unwrap();
        let checkpoints = store
            .list_checkpoints("namespace", "event_hub", "consumer_group", None)
            .await
            .unwrap();
        assert_eq!(checkpoints.len(), 1);
    }

    fn get_random_name(prefix: &str) -> String {
        format!("{}{}", prefix, azure_core::Uuid::new_v4())
    }

    #[tokio::test]
    async fn checkpoints() -> azure_core::Result<()> {
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

        let checkpoint = Checkpoint {
            fully_qualified_namespace: "ns.servicebus.windows.net".to_string(),
            event_hub_name: "event-hub-name".to_string(),
            consumer_group: "consumer-group".to_string(),
            partition_id: test_name.clone(),
            offset: Some("offset".to_string()),
            sequence_number: Some(0),
        };

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

        Ok(())
    }
}
