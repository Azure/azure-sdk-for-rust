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
        BlobClientSetMetadataOptions, BlobContainerClientListBlobsOptions,
        BlockBlobClientUploadOptions, ListBlobsIncludeItem,
    },
    BlobContainerClient,
};
use futures::TryStreamExt;
use std::{collections::HashMap, sync::Arc};
use time::OffsetDateTime;
use tracing::{debug, error, info, warn};

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

        let result = blob_client.set_metadata(&metadata, None).await;
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

                    let upload_result = blob_client.upload(blob_content, Some(options)).await;
                    match upload_result {
                        Ok(r) => Ok((r.last_modified, r.etag)),
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
                if_match: etag,
                ..Default::default()
            };
            let metadata_ref = metadata.unwrap_or_default();
            let result = blob_client
                .set_metadata(&metadata_ref, Some(options))
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
            if_none_match: Some(Etag::from("*")), // Upload without an etag, creating a new blob
            ..Default::default()
        };

        let upload_result = blob_client.upload(blob_content, Some(options)).await;
        match upload_result {
            Ok(r) => Ok((r.last_modified, r.etag)),
            Err(e) => Err(e),
        }
    }
}

#[async_trait::async_trait]
impl CheckpointStore for BlobCheckpointStore {
    /// Claims, renews, or releases ownership of the specified partitions.
    ///
    /// See [`CheckpointStore::claim_ownership()`] for the ownership and ETag contract.
    #[tracing::instrument(level = "debug", skip_all, fields(partition_count = ownerships.len()), err)]
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
                    info!(
                        event = "claim-conflict",
                        partition_id = %ownership.partition_id,
                        owner_id = ?ownership.owner_id,
                        etag = ?ownership.etag,
                        blob_name = %blob_name,
                        "Lost ownership claim: precondition (etag) failed"
                    );
                    (None, None)
                }
                Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
                    info!(
                        event = "claim-conflict",
                        partition_id = %ownership.partition_id,
                        owner_id = ?ownership.owner_id,
                        etag = ?ownership.etag,
                        blob_name = %blob_name,
                        "Lost ownership claim: blob already exists"
                    );
                    (None, None)
                }
                Err(e) => {
                    warn!(
                        partition_id = %ownership.partition_id,
                        blob_name = %blob_name,
                        error = %e,
                        "Error claiming ownership for blob"
                    );
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
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            fully_qualified_namespace = %namespace,
            eventhub = %event_hub_name,
            consumer_group = %consumer_group,
        ),
        err,
    )]
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

        let mut blobs =
            self.blob_container_client
                .list_blobs(Some(BlobContainerClientListBlobsOptions {
                    prefix: Some(prefix),
                    include: Some(vec![ListBlobsIncludeItem::Metadata]),
                    ..Default::default()
                }))?;
        let mut checkpoints = Vec::new();

        let checkpoint = Checkpoint {
            fully_qualified_namespace: namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            ..Default::default()
        };

        while let Some(blob) = blobs.try_next().await? {
            let mut checkpoint = checkpoint.clone();
            if let Some(name) = &blob.name {
                checkpoint.partition_id = name
                    .rfind('/')
                    .map(|pos| &name[pos + 1..])
                    .unwrap_or_default()
                    .to_string();
                if let Some(values) = blob.metadata.as_ref().and_then(|m| m.values.as_ref()) {
                    if let Some(sequence_number) = values.get(SEQUENCE_NUMBER) {
                        checkpoint.sequence_number = Some(sequence_number.parse()?);
                    }
                    if let Some(offset) = values.get(OFFSET) {
                        checkpoint.offset = Some(offset.clone());
                    }
                }
            }
            debug!(
                blob_name = ?blob.name,
                partition_id = %checkpoint.partition_id,
                sequence_number = ?checkpoint.sequence_number,
                offset = ?checkpoint.offset,
                "Parsed checkpoint blob"
            );

            checkpoints.push(checkpoint);
        }

        debug!("Found {} checkpoints", checkpoints.len());
        Ok(checkpoints)
    }

    /// Lists all ownerships for the specified Event Hub and consumer group.
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            fully_qualified_namespace = %namespace,
            eventhub = %event_hub_name,
            consumer_group = %consumer_group,
        ),
        err,
    )]
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

        let mut blobs =
            self.blob_container_client
                .list_blobs(Some(BlobContainerClientListBlobsOptions {
                    prefix: Some(prefix),
                    include: Some(vec![ListBlobsIncludeItem::Metadata]),
                    ..Default::default()
                }))?;
        let mut ownerships = Vec::new();

        let ownership = Ownership {
            fully_qualified_namespace: namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            ..Default::default()
        };

        while let Some(blob) = blobs.try_next().await? {
            let mut ownership = ownership.clone();
            if let Some(name) = &blob.name {
                ownership.partition_id = name
                    .rfind('/')
                    .map(|pos| &name[pos + 1..])
                    .unwrap_or_default()
                    .to_string();
                ownership.owner_id = blob
                    .metadata
                    .as_ref()
                    .and_then(|m| m.values.as_ref())
                    .and_then(|v| v.get(OWNER_ID).cloned());
            }
            if let Some(properties) = &blob.properties {
                ownership.etag = properties.etag.clone();
                ownership.last_modified_time = properties.last_modified;
            }
            debug!(
                blob_name = ?blob.name,
                partition_id = %ownership.partition_id,
                owner_id = ?ownership.owner_id,
                etag = ?ownership.etag,
                "Parsed ownership blob"
            );

            ownerships.push(ownership);
        }

        debug!("Found {} ownerships", ownerships.len());
        Ok(ownerships)
    }

    /// Updates the checkpoint for a specific partition.
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            partition_id = %checkpoint.partition_id,
            eventhub = %checkpoint.event_hub_name,
            consumer_group = %checkpoint.consumer_group,
            sequence_number = ?checkpoint.sequence_number,
            offset = ?checkpoint.offset,
        ),
    )]
    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()> {
        debug!(
            partition_id = %checkpoint.partition_id,
            sequence_number = ?checkpoint.sequence_number,
            offset = ?checkpoint.offset,
            "Updating checkpoint"
        );
        let partition_id = checkpoint.partition_id.clone();
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
        if let Err(e) = self
            .set_checkpoint_metadata_on_blob(&blob_name, metadata)
            .await
        {
            error!(
                partition_id = %partition_id,
                blob_name = %blob_name,
                error = %e,
                "Failed to persist checkpoint to blob storage"
            );
            return Err(e);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BlobCheckpointStore, CheckpointStore, Ownership};
    use azure_core::{
        http::{
            headers::{HeaderName, Headers, IF_MATCH},
            AsyncRawResponse, ClientOptions, Method, StatusCode, Transport, Url,
        },
        Bytes, Result,
    };
    use azure_core_test::http::MockHttpClient;
    use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
    use futures::FutureExt;
    use std::sync::Arc;

    #[tokio::test]
    async fn claim_ownership_releases_and_reclaims_without_expiration() -> Result<()> {
        const MODIFIED: &str = "Wed, 01 Apr 2026 00:00:00 GMT";
        const BLOB_NAME: &str =
            "example.servicebus.windows.net/eventhub/consumer-group/ownership/0";
        let mut request_count = 0;
        let mock_client = Arc::new(MockHttpClient::new(move |request| {
            request_count += 1;
            let mut headers = Headers::new();
            headers.insert("last-modified", MODIFIED);
            let owner_header = HeaderName::from_static("x-ms-meta-ownerid");
            let response = match request_count {
                1 => {
                    assert_eq!(request.method(), Method::Put);
                    assert_eq!(
                        request.url().path(),
                        format!("/container/{}", BLOB_NAME.replace('/', "%2F"))
                    );
                    assert_eq!(
                        request
                            .headers()
                            .get_optional_str(&HeaderName::from_static("if-none-match")),
                        Some("*")
                    );
                    assert_eq!(
                        request.headers().get_optional_str(&owner_header),
                        Some("first-owner")
                    );
                    headers.insert("etag", "first-etag");
                    AsyncRawResponse::from_bytes(StatusCode::Created, headers, Bytes::new())
                }
                2 | 4 | 5 => {
                    assert_eq!(request.method(), Method::Put);
                    assert_eq!(
                        request.url().path(),
                        format!("/container/{}", BLOB_NAME.replace('/', "%2F"))
                    );
                    assert_eq!(request.url().query(), Some("comp=metadata"));
                    let (etag, owner) = if request_count == 4 {
                        ("released-etag", Some("second-owner"))
                    } else {
                        ("first-etag", None)
                    };
                    assert_eq!(request.headers().get_optional_str(&IF_MATCH), Some(etag));
                    assert_eq!(request.headers().get_optional_str(&owner_header), owner);
                    if request_count == 5 {
                        // The service rejects a release carrying the previous owner's ETag.
                        headers.insert("x-ms-error-code", "ConditionNotMet");
                        AsyncRawResponse::from_bytes(
                            StatusCode::PreconditionFailed,
                            headers,
                            Bytes::new(),
                        )
                    } else {
                        headers.insert(
                            "etag",
                            if request_count == 2 {
                                "released-etag"
                            } else {
                                "reclaimed-etag"
                            },
                        );
                        AsyncRawResponse::from_bytes(StatusCode::Ok, headers, Bytes::new())
                    }
                }
                3 | 6 => {
                    assert_eq!(request.method(), Method::Get);
                    assert_eq!(request.url().path(), "/container");
                    let query: std::collections::HashMap<_, _> =
                        request.url().query_pairs().collect();
                    assert_eq!(query.get("comp").map(|v| v.as_ref()), Some("list"));
                    assert_eq!(query.get("include").map(|v| v.as_ref()), Some("metadata"));
                    assert_eq!(
                        query.get("prefix").map(|v| v.as_ref()),
                        Some("example.servicebus.windows.net/eventhub/consumer-group/ownership/")
                    );
                    let (etag, metadata) = if request_count == 3 {
                        ("released-etag", "<Metadata />")
                    } else {
                        (
                            "reclaimed-etag",
                            "<Metadata><ownerid>second-owner</ownerid></Metadata>",
                        )
                    };
                    let body = format!(
                        r#"<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
<Blobs><Blob><Name>{BLOB_NAME}</Name><Properties>
<Etag>{etag}</Etag><Last-Modified>{MODIFIED}</Last-Modified><BlobType>BlockBlob</BlobType>
</Properties>{metadata}</Blob></Blobs><NextMarker /></EnumerationResults>"#
                    );
                    AsyncRawResponse::from_bytes(StatusCode::Ok, headers, Bytes::from(body))
                }
                _ => panic!("unexpected request {request_count}"),
            };
            async move { Ok(response) }.boxed()
        }));
        let container_client = BlobContainerClient::new(
            Url::parse("https://example.blob.core.windows.net/container")?,
            None,
            Some(BlobContainerClientOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(mock_client)),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;
        let store = BlobCheckpointStore::new(container_client);
        let ownership = Ownership {
            fully_qualified_namespace: "example.servicebus.windows.net".to_string(),
            event_hub_name: "eventhub".to_string(),
            consumer_group: "consumer-group".to_string(),
            partition_id: "0".to_string(),
            owner_id: Some("first-owner".to_string()),
            ..Default::default()
        };
        let first = store.claim_ownership(&[ownership]).await?;
        assert_eq!(first.len(), 1);
        let mut release = first[0].clone();
        release.owner_id = None;

        let released = store
            .claim_ownership(std::slice::from_ref(&release))
            .await?;
        assert_eq!(released.len(), 1);
        assert_eq!(released[0].owner_id, None);
        assert_eq!(
            released[0].etag.as_ref().map(|etag| etag.as_ref()),
            Some("released-etag")
        );
        assert!(released[0].last_modified_time.is_some());

        let listed = store
            .list_ownerships(
                &release.fully_qualified_namespace,
                &release.event_hub_name,
                &release.consumer_group,
            )
            .await?;
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].owner_id, None);
        assert_eq!(listed[0].etag, released[0].etag);

        // Reclaim immediately using the released record's ETag, with no expiration wait.
        let mut reclaim = listed[0].clone();
        reclaim.owner_id = Some("second-owner".to_string());
        let reclaimed = store.claim_ownership(&[reclaim]).await?;
        assert_eq!(reclaimed.len(), 1);
        assert_eq!(reclaimed[0].owner_id.as_deref(), Some("second-owner"));
        assert_ne!(reclaimed[0].etag, released[0].etag);

        let stale_release = store
            .claim_ownership(std::slice::from_ref(&release))
            .await?;
        assert!(stale_release.is_empty());
        let listed = store
            .list_ownerships(
                &release.fully_qualified_namespace,
                &release.event_hub_name,
                &release.consumer_group,
            )
            .await?;
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].owner_id.as_deref(), Some("second-owner"));
        assert_eq!(listed[0].etag, reclaimed[0].etag);
        Ok(())
    }
}
