// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Blob Storage implementation of the Event Hubs checkpoint store.
// cspell: ignore rfind sequencenumber
use async_trait::async_trait;
use azure_core::Result;
use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_storage_blob::{
    models::{BlobContainerClientListBlobFlatSegmentOptions, ListBlobsIncludeItem},
    BlobContainerClient,
};
use futures::TryStreamExt;
use std::{fmt::Debug, sync::Arc};
use tracing::debug;

/// Azure Blob Storage implementation of the [`CheckpointStore`] trait.
///
/// This implementation stores checkpoint and ownership information in Azure Blob Storage,
/// providing durable persistence for Event Hub event processing state.
#[derive(Clone)]
pub struct BlobCheckpointStore {
    blob_container_client: Arc<BlobContainerClient>,
}

impl Debug for BlobCheckpointStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlobCheckpointStore").finish()
    }
}

impl BlobCheckpointStore {
    /// Creates a new blob checkpoint store.
    ///
    /// # Arguments
    ///
    /// * `blob_service_client` - The Azure Blob Storage service client
    pub fn new(blob_container_client: BlobContainerClient) -> Arc<Self> {
        Arc::new(Self {
            blob_container_client: Arc::new(blob_container_client),
        })
    }
}

#[async_trait]
impl CheckpointStore for BlobCheckpointStore {
    /// Claims ownership of the specified partitions.
    async fn claim_ownership(&self, ownerships: &[Ownership]) -> Result<Vec<Ownership>> {
        debug!("Claiming ownership for {} partitions", ownerships.len());

        // For now, return an empty vector indicating no ownerships were claimed
        // This is a minimal implementation to get the package compiling
        Ok(vec![])
    }

    /// Lists all checkpoints for the specified Event Hub and consumer group.
    async fn list_checkpoints(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<Vec<Checkpoint>> {
        debug!(
            "Listing checkpoints for namespace: {}, event_hub: {}, consumer_group: {}",
            namespace, event_hub_name, consumer_group
        );

        let prefix =
            Checkpoint::get_checkpoint_blob_prefix_name(namespace, event_hub_name, consumer_group)?;

        debug!("Using checkpoint prefix: {}", prefix);

        let mut blobs = self.blob_container_client.list_blobs(Some(
            BlobContainerClientListBlobFlatSegmentOptions {
                prefix: Some(prefix),
                include: Some(vec![ListBlobsIncludeItem::Metadata]),
                ..Default::default()
            },
        ))?;
        // For now, return an empty vector since the blob storage API is complex
        // and we need to understand the exact response structure.
        // This provides a working foundation that can be enhanced later.
        let mut checkpoints = Vec::new();

        let checkpoint = Checkpoint {
            fully_qualified_namespace: namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            ..Default::default()
        };

        while let Some(blob) = blobs.try_next().await? {
            let blob_body = blob.into_body().await?;
            debug!("Blob body: {blob_body:?}, {:?}", blob_body.container_name);
            for blob in blob_body.segment.blob_items.iter() {
                let mut checkpoint = checkpoint.clone();
                if let Some(name) = &blob.name {
                    if let Some(name) = &name.content {
                        checkpoint.partition_id = name
                            .rfind('/')
                            .map(|pos| &name[pos + 1..])
                            .unwrap_or_default()
                            .to_string();
                    }
                }

                // // Parse the blob body to extract checkpoint information
                // if let Some(offset) = blob.metadata.get("offset") {
                //     checkpoint.offset = offset.parse().unwrap_or_default();
                // }
                // if let Some(sequence_number) = blob.metadata.get("sequencenumber") {
                //     checkpoint.sequence_number = sequence_number.parse().unwrap_or_default();
                // }

                checkpoints.push(checkpoint);
            }
        }

        debug!("Found {} checkpoints", checkpoints.len());
        Ok(checkpoints)
    }

    /// Lists all ownerships for the specified Event Hub and consumer group.
    async fn list_ownerships(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<Vec<Ownership>> {
        debug!(
            "Listing ownerships for namespace: {namespace}, event_hub: {event_hub_name}, consumer_group: {consumer_group}",
        );

        let prefix =
            Ownership::get_ownership_prefix_name(namespace, event_hub_name, consumer_group)?;

        debug!("Using ownership prefix: {}", prefix);

        let mut blobs = self.blob_container_client.list_blobs(Some(
            BlobContainerClientListBlobFlatSegmentOptions {
                prefix: Some(prefix),
                include: Some(vec![ListBlobsIncludeItem::Metadata]),
                ..Default::default()
            },
        ))?;
        let mut ownerships = Vec::new();

        let ownership = Ownership {
            fully_qualified_namespace: namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            ..Default::default()
        };

        while let Some(blob) = blobs.try_next().await? {
            let blob_body = blob.into_body().await?;
            debug!("Blob body: {blob_body:?}, {:?}", blob_body.container_name);
            for blob in blob_body.segment.blob_items.iter() {
                let mut ownership = ownership.clone();
                if let Some(name) = &blob.name {
                    if let Some(name) = &name.content {
                        ownership.partition_id = name
                            .rfind('/')
                            .map(|pos| &name[pos + 1..])
                            .unwrap_or_default()
                            .to_string();
                    }
                }

                // // Parse the blob body to extract ownership information
                // if let Some(offset) = blob.metadata.get("offset") {
                //     ownership.offset = offset.parse().unwrap_or_default();
                // }
                // if let Some(sequence_number) = blob.metadata.get("sequencenumber") {
                //     ownership.sequence_number = sequence_number.parse().unwrap_or_default();
                // }

                ownerships.push(ownership);
            }
        }

        debug!("Found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    /// Updates the checkpoint for a specific partition.
    async fn update_checkpoint(&self, _checkpoint: Checkpoint) -> Result<()> {
        debug!("Updating checkpoint - minimal implementation");

        // For now, just return Ok
        // This is a minimal implementation to get the package compiling
        Ok(())
    }

    #[cfg(feature = "test")]
    async fn update_ownership(&self, _ownership: Ownership) -> Result<()> {
        unimplemented!("update_ownership is a test hook and is not implemented.");
    }
}
