// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::models::{
        BlobClientAcquireLeaseResult, BlobClientBreakLeaseResult, BlobClientChangeLeaseResult,
        BlobClientDownloadResult, BlobClientGetPropertiesResult, BlobClientReleaseLeaseResult,
        BlobClientRenewLeaseResult, BlockBlobClientCommitBlockListResult,
        BlockBlobClientStageBlockResult, BlockBlobClientUploadResult,
    },
    models::{
        AccessTier, BlobClientAcquireLeaseOptions, BlobClientBreakLeaseOptions,
        BlobClientChangeLeaseOptions, BlobClientDeleteOptions, BlobClientDownloadOptions,
        BlobClientGetPropertiesOptions, BlobClientReleaseLeaseOptions, BlobClientRenewLeaseOptions,
        BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlockBlobClientCommitBlockListOptions, BlockBlobClientUploadOptions, BlockList,
        BlockListType, BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlobClientOptions, BlockBlobClient, PageBlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, RequestContent, Response, Url, XmlFormat,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage blob, although that blob may not yet exist.
pub struct BlobClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedBlobClient,
}

impl BlobClient {
    /// Creates a new BlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this blob.
    /// * `blob_name` - The name of the blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let client = GeneratedBlobClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            blob_name.clone(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Returns a new instance of AppendBlobClient.
    ///
    /// # Arguments
    ///
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_append_blob_client(),
        }
    }

    /// Returns a new instance of BlockBlobClient.
    ///
    /// # Arguments
    ///
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_block_blob_client(),
        }
    }

    /// Returns a new instance of PageBlobClient.
    ///
    /// # Arguments
    ///
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_page_blob_client(),
        }
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.client.container_name
    }

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.client.blob_name
    }

    /// Returns all user-defined metadata, standard HTTP properties, and system properties for the blob.
    /// The data returned does not include the content of the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobClientGetPropertiesResult, NoFormat>> {
        self.client.get_properties(options).await
    }

    /// Sets system properties on the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn set_properties(
        &self,
        options: Option<BlobClientSetPropertiesOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_properties(options).await
    }

    /// Downloads a blob from the service, including its metadata and properties.
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<Response<BlobClientDownloadResult, NoFormat>> {
        self.client.download(options).await
    }

    /// Creates a new blob from a data source.
    ///
    /// # Arguments
    ///
    /// * `data` - The blob data to upload.
    /// * `overwrite` - Whether the blob to be uploaded should overwrite the current data. If True, `upload_blob` will overwrite the existing data.
    ///   If False, the operation will fail with ResourceExistsError.
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn upload(
        &self,
        data: RequestContent<Bytes>,
        overwrite: bool,
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let block_blob_client = self.client.get_block_blob_client();

        block_blob_client
            .upload(data, content_length, Some(options))
            .await
    }

    /// Sets user-defined metadata for the specified blob as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the blob. To remove all metadata from the blob, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        options: Option<BlobClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(options).await
    }

    /// Deletes the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete(
        &self,
        options: Option<BlobClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete(options).await
    }

    /// Sets the tier on a blob. Standard tiers are only applicable for Block blobs, while Premium tiers are only applicable
    /// for Page blobs.
    ///
    /// # Arguments
    ///
    /// * `tier` - The tier to be set on the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn set_tier(
        &self,
        tier: AccessTier,
        options: Option<BlobClientSetTierOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_tier(tier, options).await
    }

    /// Requests a new lease on a blob. The lease lock duration can be 15 to 60 seconds, or can be infinite.
    ///
    /// # Arguments
    ///
    /// * `duration` - Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A
    ///   non-infinite lease can be between 15 and 60 seconds.
    /// * `options` - Optional configuration for the request.
    pub async fn acquire_lease(
        &self,
        duration: i32,
        options: Option<BlobClientAcquireLeaseOptions<'_>>,
    ) -> Result<Response<BlobClientAcquireLeaseResult, NoFormat>> {
        self.client.acquire_lease(duration, options).await
    }

    /// Ends a lease and ensures that another client can't acquire a new lease until the current lease
    /// period has expired.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn break_lease(
        &self,
        options: Option<BlobClientBreakLeaseOptions<'_>>,
    ) -> Result<Response<BlobClientBreakLeaseResult, NoFormat>> {
        self.client.break_lease(options).await
    }

    /// Changes the ID of an existing lease to the proposed lease ID.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `proposed_lease_id` - The proposed lease ID for the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn change_lease(
        &self,
        lease_id: String,
        proposed_lease_id: String,
        options: Option<BlobClientChangeLeaseOptions<'_>>,
    ) -> Result<Response<BlobClientChangeLeaseResult, NoFormat>> {
        self.client
            .change_lease(lease_id, proposed_lease_id, options)
            .await
    }

    /// Frees the lease so that another client can immediately acquire a lease
    /// against the blob as soon as the release is complete.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional configuration for the request.
    pub async fn release_lease(
        &self,
        lease_id: String,
        options: Option<BlobClientReleaseLeaseOptions<'_>>,
    ) -> Result<Response<BlobClientReleaseLeaseResult, NoFormat>> {
        self.client.release_lease(lease_id, options).await
    }

    /// Renews the lease on a blob.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional configuration for the request.
    pub async fn renew_lease(
        &self,
        lease_id: String,
        options: Option<BlobClientRenewLeaseOptions<'_>>,
    ) -> Result<Response<BlobClientRenewLeaseResult, NoFormat>> {
        self.client.renew_lease(lease_id, options).await
    }
}
