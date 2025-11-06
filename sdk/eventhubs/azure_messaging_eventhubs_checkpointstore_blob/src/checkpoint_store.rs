// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Blob Storage implementation of the Event Hubs checkpoint store.
// cspell: ignore rfind sequencenumber ownerid
use azure_core::{
    http::{
        headers::{ETAG, LAST_MODIFIED},
        Etag, NoFormat, RequestContent, StatusCode,
    },
    time::parse_rfc7231,
    Bytes, Result,
};
use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_storage_blob::{
    models::{
        BlobClientSetMetadataOptions, BlobContainerClientListBlobFlatSegmentOptions,
        BlockBlobClientUploadOptions, BlockBlobClientUploadResultHeaders, ListBlobsIncludeItem,
    },
    BlobContainerClient,
};
use futures::TryStreamExt;
use std::{collections::HashMap, sync::Arc};
use time::OffsetDateTime;
use tracing::{debug, info, warn};

/// Azure Blob Storage implementation of the [`CheckpointStore`] trait.
///
/// This implementation stores checkpoint and ownership information in Azure Blob Storage,
/// providing durable persistence for Event Hub event processing state.
#[derive(Clone)]
pub struct BlobCheckpointStore {
    blob_container_client: Arc<BlobContainerClient>,
}

const OWNER_ID: &str = "ownerid";
const OFFSET: &str = "offset";
const SEQUENCE_NUMBER: &str = "sequencenumber";

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

    fn process_storage_response_metadata(
        last_modified: Option<String>,
        etag: Option<String>,
    ) -> Result<(Option<OffsetDateTime>, Option<Etag>)> {
        let lm = match last_modified {
            Some(lm) => Some(parse_rfc7231(lm.as_str())?),
            None => None,
        };
        Ok((lm, etag.map(Etag::from)))
    }

    async fn set_checkpoint_metadata_on_blob(
        &self,
        blob_name: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(Option<OffsetDateTime>, Option<Etag>)> {
        let blob_client = self.blob_container_client.blob_client(blob_name);

        let result = blob_client.set_metadata(metadata.clone(), None).await;
        match result {
            Ok(r) => Ok(Self::process_storage_response_metadata(
                r.headers().get_optional_string(&LAST_MODIFIED),
                r.headers().get_optional_string(&ETAG),
            )?),
            Err(e) => match e.http_status() {
                Some(StatusCode::NotFound) => {
                    info!("Blob {blob_name} not found, creating.");
                    let blob_content = RequestContent::<Bytes, NoFormat>::from(Vec::new());
                    let options = BlockBlobClientUploadOptions {
                        metadata: Some(metadata),
                        ..Default::default()
                    };

                    let upload_result = blob_client
                        .upload(blob_content, true, 0, Some(options))
                        .await;
                    match upload_result {
                        Ok(r) => Ok((r.last_modified()?, r.etag()?.map(Etag::from))),
                        Err(e) => Err(e),
                    }
                }
                _ => Err(e),
            },
        }
    }

    async fn set_ownership_metadata_on_blob(
        &self,
        blob_name: &str,
        metadata: Option<HashMap<String, String>>,
        etag: Option<Etag>,
    ) -> Result<(Option<OffsetDateTime>, Option<Etag>)> {
        let blob_client = self.blob_container_client.blob_client(blob_name);

        if etag.is_some() {
            debug!(
                "{:?} claiming ownership for {} with etag {:?}",
                metadata, blob_name, etag
            );
            let options = BlobClientSetMetadataOptions {
                if_match: etag.map(|e| e.to_string()),
                ..Default::default()
            };
            let result = blob_client
                .set_metadata(metadata.unwrap_or_default(), Some(options))
                .await?;
            return Self::process_storage_response_metadata(
                result.headers().get_optional_string(&LAST_MODIFIED),
                result.headers().get_optional_string(&ETAG),
            );
        }
        debug!("Claiming ownership for {} without etag", blob_name);

        let blob_content = RequestContent::<Bytes, NoFormat>::from(Vec::new());
        let options = BlockBlobClientUploadOptions {
            metadata: metadata.clone(),
            if_none_match: Some("*".to_string()), // Upload without an etag, creating a new blob
            ..Default::default()
        };

        let upload_result = blob_client
            .upload(blob_content, true, 0, Some(options))
            .await;
        match upload_result {
            Ok(r) => Ok((r.last_modified()?, r.etag()?.map(Etag::from))),
            Err(e) => Err(e),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl CheckpointStore for BlobCheckpointStore {
    /// Claims ownership of the specified partitions.
    async fn claim_ownership(&self, ownerships: &[Ownership]) -> Result<Vec<Ownership>> {
        debug!("Claiming ownership for {} partitions", ownerships.len());

        let mut new_ownerships = Vec::new();
        for ownership in ownerships {
            let blob_name = Ownership::get_ownership_name(
                &ownership.fully_qualified_namespace,
                &ownership.event_hub_name,
                &ownership.consumer_group,
                &ownership.partition_id,
            )?;

            let set_metadata_result = self
                .set_ownership_metadata_on_blob(
                    &blob_name,
                    ownership
                        .owner_id
                        .clone()
                        .map(|id| HashMap::<String, String>::from([(OWNER_ID.to_string(), id)])),
                    ownership.etag.clone(),
                )
                .await;
            let (last_modified_time, etag) = match set_metadata_result {
                Ok((last_modified_time, etag)) => (last_modified_time, etag),
                Err(e) if e.http_status() == Some(StatusCode::PreconditionFailed) => {
                    debug!("PreconditionFailed error for blob {}", blob_name);
                    (None, None)
                }
                Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
                    debug!("Blob already exists, skipping");
                    (None, None)
                }
                Err(e) => {
                    warn!("Error claiming ownership for blob {}: {}", blob_name, e);
                    return Err(e);
                }
            };

            if let Some(etag) = etag {
                if !etag.as_ref().is_empty() {
                    let new_ownership = Ownership {
                        etag: Some(etag),
                        last_modified_time,
                        ..ownership.clone()
                    };
                    new_ownerships.push(new_ownership);
                }
            }
        }

        debug!("Returning {} ownerships", new_ownerships.len());
        Ok(new_ownerships)
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
        let mut checkpoints = Vec::new();

        let checkpoint = Checkpoint {
            fully_qualified_namespace: namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            ..Default::default()
        };

        while let Some(blob) = blobs.try_next().await? {
            let blob_body = blob.into_model()?;
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
                        if let Some(additional_properties) = blob
                            .metadata
                            .as_ref()
                            .and_then(|m| m.additional_properties.as_ref())
                        {
                            if let Some(sequence_number) =
                                additional_properties.get(SEQUENCE_NUMBER)
                            {
                                checkpoint.sequence_number = Some(sequence_number.parse()?);
                            }
                            if let Some(offset) = additional_properties.get(OFFSET) {
                                checkpoint.offset = Some(offset.clone());
                            }
                        }
                    }
                }

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
            let blob_body = blob.into_model()?;
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
                        ownership.owner_id = blob
                            .metadata
                            .as_ref()
                            .and_then(|m| m.additional_properties.as_ref())
                            .and_then(|ap| ap.get(OWNER_ID).cloned());
                    }
                }
                if let Some(properties) = &blob.properties {
                    ownership.etag = properties.etag.as_ref().map(|s| Etag::from(s.clone()));
                    ownership.last_modified_time = properties.last_modified;
                }

                ownerships.push(ownership);
            }
        }

        debug!("Found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    /// Updates the checkpoint for a specific partition.
    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()> {
        let blob_name = Checkpoint::get_checkpoint_blob_name(
            &checkpoint.fully_qualified_namespace,
            &checkpoint.event_hub_name,
            &checkpoint.consumer_group,
            &checkpoint.partition_id,
        )?;
        let mut metadata = HashMap::new();
        if let Some(sequence_number) = checkpoint.sequence_number {
            metadata.insert(SEQUENCE_NUMBER.to_string(), sequence_number.to_string());
        }
        if let Some(offset) = checkpoint.offset {
            metadata.insert(OFFSET.to_string(), offset);
        }
        self.set_checkpoint_metadata_on_blob(&blob_name, metadata)
            .await?;
        Ok(())
    }
}
