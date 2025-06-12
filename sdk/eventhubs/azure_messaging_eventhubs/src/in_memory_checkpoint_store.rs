use crate::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_core::{error::ErrorKind as AzureErrorKind, http::Etag, Error, Result, Uuid};
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, time::SystemTime};
use tracing::{trace, warn};

/// An in-memory checkpoint store for Event Hubs.
/// This store is used to manage checkpoints and ownerships in memory.
/// It is primarily used for testing and development purposes.
/// It implements the `CheckpointStore` trait, allowing it to be used as a checkpoint store.
/// The store is thread-safe and can be used in a multi-threaded environment.
pub struct InMemoryCheckpointStore {
    checkpoints: Arc<Mutex<HashMap<String, Checkpoint>>>,
    ownerships: Arc<Mutex<HashMap<String, Ownership>>>,
}

impl Default for InMemoryCheckpointStore {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! check_non_empty_parameter(
    ($field:expr) => {
        if $field.is_empty() {
            return Err(Error::message(
                AzureErrorKind::Other,
                String::from("Required field ") + stringify!($field) + " is empty",
            ));
        }
    }
);

impl InMemoryCheckpointStore {
    /// Creates a new instance of `InMemoryCheckpointStore`.
    pub fn new() -> Self {
        InMemoryCheckpointStore {
            checkpoints: Arc::new(Mutex::new(HashMap::new())),
            ownerships: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Updates the ownership for a specific partition.
    pub fn update_ownership(&self, ownership: &Ownership) -> Result<Ownership> {
        trace!("Update ownership for partition {}", ownership.partition_id);

        check_non_empty_parameter!(ownership.fully_qualified_namespace);
        check_non_empty_parameter!(ownership.event_hub_name);
        check_non_empty_parameter!(ownership.consumer_group);
        check_non_empty_parameter!(ownership.partition_id);

        let mut store = self.ownerships.lock().unwrap();
        let key = Ownership::get_ownership_name(
            &ownership.fully_qualified_namespace,
            &ownership.event_hub_name,
            &ownership.consumer_group,
            &ownership.partition_id,
        )?;
        trace!("Update ownership for key {}", key);
        if store.contains_key(&key) {
            if ownership.etag != store.get(&key).unwrap().etag {
                warn!("ETag mismatch {}", key);
                return Err(Error::message(
                    AzureErrorKind::Other,
                    format!("ETag mismatch for partition {key}"),
                ));
            }
            store.insert(key.clone(), ownership.clone());
            trace!("Updated ownership for key {}", key);
            Ok(ownership.clone())
        } else {
            trace!("Insert new ownership for key {}", key);
            let mut new_ownership = ownership.clone();
            new_ownership.etag = Some(Etag::from(Uuid::new_v4().to_string()));
            new_ownership.last_modified_time = Some(SystemTime::now());
            store.insert(key.clone(), new_ownership.clone());
            trace!("Inserted new ownership for key {}", key);
            Ok(new_ownership.clone())
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl CheckpointStore for InMemoryCheckpointStore {
    async fn claim_ownership(&self, ownerships: &[Ownership]) -> Result<Vec<Ownership>> {
        trace!("Claim ownership for {} partitions", ownerships.len());
        let mut claimed_ownerships = Vec::new();
        for ownership in ownerships {
            let ownership = self.update_ownership(ownership)?;
            if ownership.etag.is_some() {
                claimed_ownerships.push(ownership);
            }
        }
        Ok(claimed_ownerships)
    }

    #[cfg(feature = "test")]
    async fn update_ownership(&self, ownership: Ownership) -> Result<()> {
        trace!(
            "update_ownership: update ownership for partition {}",
            ownership.partition_id
        );
        let ownership = self.update_ownership(&ownership)?;
        trace!(
            "update_ownership: updated ownership for partition {}",
            ownership.partition_id
        );
        trace!("Update ownership for partition {}", ownership.partition_id);
        Ok(())
    }

    async fn list_checkpoints(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<Vec<Checkpoint>> {
        let store = self.checkpoints.lock().unwrap();
        let prefix =
            Checkpoint::get_checkpoint_blob_prefix_name(namespace, event_hub_name, consumer_group)?;
        trace!("list_checkpoints: list checkpoints for prefix {prefix}");
        let mut checkpoints = Vec::new();
        for (key, value) in store.iter() {
            if key.starts_with(&prefix) {
                checkpoints.push(value.clone());
            }
        }
        checkpoints.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));
        trace!("list_checkpoints: found {} checkpoints", checkpoints.len());
        Ok(checkpoints)
    }

    async fn list_ownerships(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<Vec<Ownership>> {
        let store = self.ownerships.lock().unwrap();

        let prefix =
            Ownership::get_ownership_prefix_name(namespace, event_hub_name, consumer_group)?;
        trace!("list_ownerships: list ownerships for prefix {prefix}");
        let mut ownerships = Vec::new();
        ownerships.extend(
            store
                .iter()
                .filter(|(key, _)| key.starts_with(&prefix))
                .map(|(_, value)| value.clone()),
        );
        ownerships.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));
        trace!("list_ownerships: found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()> {
        trace!(
            "update_checkpoint: update checkpoint for {}",
            checkpoint.partition_id
        );
        let mut checkpoints = self.checkpoints.lock().map_err(|e| {
            Error::message(
                AzureErrorKind::Other,
                format!("Failed to lock checkpoint store: {}", e),
            )
        })?;
        let key = Checkpoint::get_checkpoint_blob_name(
            &checkpoint.fully_qualified_namespace,
            &checkpoint.event_hub_name,
            &checkpoint.consumer_group,
            &checkpoint.partition_id,
        )?;
        trace!("update_checkpoint: insert {checkpoint:?} checkpoint key {key}");
        checkpoints.insert(key, checkpoint);
        Ok(())
    }
}
