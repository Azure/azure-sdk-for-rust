use crate::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_core::{
    error::ErrorKind as AzureErrorKind, http::Etag, time::OffsetDateTime, Error, Result, Uuid,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, trace};

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
            return Err(Error::with_message(
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
    ///
    /// Every successful call returns a record with a new ETag and a fresh
    /// `last_modified_time`, for a renewal and for a first claim. A renewal
    /// makes the caller's ETag stale, so the caller must keep the returned
    /// record for its next claim.
    ///
    /// A stale ETag returns an error. [`CheckpointStore::claim_ownership`]
    /// reports the same condition as a lost claim instead.
    pub fn update_ownership(&self, ownership: &Ownership) -> Result<Ownership> {
        if let Some(updated_ownership) = self.try_update_ownership(ownership)? {
            return Ok(updated_ownership);
        }

        let key = Ownership::get_ownership_name(
            &ownership.fully_qualified_namespace,
            &ownership.event_hub_name,
            &ownership.consumer_group,
            &ownership.partition_id,
        )?;
        error!(
            partition_id = %ownership.partition_id,
            expected_etag = ?ownership.etag,
            "ETag mismatch claiming ownership for key {}",
            key
        );
        Err(Error::with_message(
            AzureErrorKind::Other,
            format!("ETag mismatch for partition {key}"),
        ))
    }

    /// Updates the ownership for a specific partition, and reports a lost
    /// claim as `Ok(None)`. An `Err` is a store failure, not a lost claim.
    fn try_update_ownership(&self, ownership: &Ownership) -> Result<Option<Ownership>> {
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

        // A renewal must present the ETag the store holds. A first claim has
        // no record to match against.
        let is_renewal = match store.get(&key) {
            Some(existing) => {
                let actual_etag = existing.etag.clone();
                if ownership.etag != actual_etag {
                    debug!(
                        event = "claim-conflict",
                        partition_id = %ownership.partition_id,
                        expected_etag = ?ownership.etag,
                        actual_etag = ?actual_etag,
                        "Lost ownership claim: ETag mismatch for key {}",
                        key
                    );
                    return Ok(None);
                }
                true
            }
            None => false,
        };

        // A renewal and a first claim share one path, so the two cannot drift
        // apart again. Both rotate the ETag and stamp the current time, the
        // way the blob store does with the values the service returns.
        let mut updated_ownership = ownership.clone();
        updated_ownership.etag = Some(Etag::from(Uuid::new_v4().to_string()));
        updated_ownership.last_modified_time = Some(OffsetDateTime::now_utc());
        store.insert(key.clone(), updated_ownership.clone());

        if is_renewal {
            trace!("Updated ownership for key {}", key);
        } else {
            trace!("Inserted new ownership for key {}", key);
        }
        Ok(Some(updated_ownership))
    }
}

#[cfg(test)]
impl InMemoryCheckpointStore {
    /// Test-only seam: force an ownership's `last_modified_time` directly,
    /// so tests can simulate an expired partition without depending on
    /// `claim_ownership` preserving a stale timestamp.
    pub(crate) fn set_last_modified_time_for_test(
        &self,
        ownership: &Ownership,
        last_modified_time: OffsetDateTime,
    ) -> Result<()> {
        let key = Ownership::get_ownership_name(
            &ownership.fully_qualified_namespace,
            &ownership.event_hub_name,
            &ownership.consumer_group,
            &ownership.partition_id,
        )?;
        let mut store = self.ownerships.lock().unwrap();
        let entry = store.get_mut(&key).ok_or_else(|| {
            Error::with_message(
                AzureErrorKind::Other,
                format!("No ownership found for key {key}"),
            )
        })?;
        entry.last_modified_time = Some(last_modified_time);
        Ok(())
    }
}

#[async_trait::async_trait]
impl CheckpointStore for InMemoryCheckpointStore {
    async fn claim_ownership(&self, ownerships: &[Ownership]) -> Result<Vec<Ownership>> {
        trace!("Claim ownership for {} partitions", ownerships.len());
        let mut claimed_ownerships = Vec::new();
        for ownership in ownerships {
            // A lost claim is not a failure. Skip that partition and keep the
            // claims this batch already made.
            if let Some(claimed) = self.try_update_ownership(ownership)? {
                claimed_ownerships.push(claimed);
            }
        }
        Ok(claimed_ownerships)
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
            error!(
                partition_id = %checkpoint.partition_id,
                error = %e,
                "Checkpoint store lock is poisoned; cannot update checkpoint"
            );
            Error::with_message(
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
