use super::processor::{
    models::{Checkpoint, Ownership},
    CheckpointStore, ClaimOwnershipOptions, ListCheckpointsOptions, ListOwnershipOptions,
};
//use async_trait::async_trait;
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

    pub fn update_ownership(&self, ownership: &Ownership) -> Result<Ownership> {
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

#[async_trait::async_trait]
impl CheckpointStore for InMemoryCheckpointStore {
    async fn claim_ownership(
        &self,
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
        for (key, value) in store.iter() {
            if key.starts_with(&prefix) {
                ownerships.push(value.clone());
            }
        }
        debug!("list_ownerships: found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()> {
        let mut store = self.checkpoints.lock().map_err(|e| {
            Error::message(
                AzureErrorKind::Other,
                format!("Failed to lock checkpoint store: {}", e),
            )
        })?;
        store.insert(
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
