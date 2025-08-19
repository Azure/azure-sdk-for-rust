// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobContainerClient as GeneratedBlobContainerClient,
    generated::models::{
        BlobContainerClientAcquireLeaseResult, BlobContainerClientBreakLeaseResult,
        BlobContainerClientChangeLeaseResult, BlobContainerClientGetAccountInfoResult,
        BlobContainerClientGetPropertiesResult, BlobContainerClientReleaseLeaseResult,
        BlobContainerClientRenewLeaseResult,
    },
    models::{
        BlobContainerClientAcquireLeaseOptions, BlobContainerClientBreakLeaseOptions,
        BlobContainerClientChangeLeaseOptions, BlobContainerClientCreateOptions,
        BlobContainerClientDeleteOptions, BlobContainerClientGetAccountInfoOptions,
        BlobContainerClientGetPropertiesOptions, BlobContainerClientListBlobFlatSegmentOptions,
        BlobContainerClientReleaseLeaseOptions, BlobContainerClientRenewLeaseOptions,
        BlobContainerClientSetMetadataOptions, ListBlobsFlatSegmentResponse,
    },
    pipeline::StorageHeadersPolicy,
    BlobClient, BlobContainerClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, PageIterator, Pager, Response, Url, XmlFormat,
    },
    Result,
};
use std::collections::HashMap;
use std::sync::Arc;

/// A client to interact with a specified Azure storage container.
pub struct BlobContainerClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedBlobContainerClient,
}

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let client = GeneratedBlobContainerClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            Some(options),
        )?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: String) -> BlobClient {
        BlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_blob_client(blob_name),
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

    /// Creates a new container under the specified account. If the container with the same name already exists, the operation fails.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn create_container(
        &self,
        options: Option<BlobContainerClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.create(options).await
    }

    /// Sets user-defined metadata for the specified container as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the container. To remove all metadata from the container, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata headers.
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        metadata: HashMap<String, String>,
        options: Option<BlobContainerClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(metadata, options).await
    }

    /// Marks the specified container for deletion. The container and any blobs contained within are later deleted during garbage collection.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete_container(
        &self,
        options: Option<BlobContainerClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete(options).await
    }

    /// Returns all user-defined metadata and system properties for the specified container.
    /// The data returned does not include the container's list of blobs.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetPropertiesResult, NoFormat>> {
        self.client.get_properties(options).await
    }

    /// Returns a list of the blobs under the specified container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub fn list_blobs(
        &self,
        options: Option<BlobContainerClientListBlobFlatSegmentOptions<'_>>,
    ) -> Result<PageIterator<Response<ListBlobsFlatSegmentResponse, XmlFormat>>> {
        self.client.list_blob_flat_segment(options)
    }

    /// Requests a new lease on a container. The lease lock duration can be 15 to 60 seconds, or can be infinite.
    ///
    /// # Arguments
    ///
    /// * `duration` - Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A
    ///   non-infinite lease can be between 15 and 60 seconds.
    /// * `options` - Optional configuration for the request.
    pub async fn acquire_lease(
        &self,
        duration: i32,
        options: Option<BlobContainerClientAcquireLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientAcquireLeaseResult, NoFormat>> {
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
        options: Option<BlobContainerClientBreakLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientBreakLeaseResult, NoFormat>> {
        self.client.break_lease(options).await
    }

    /// Changes the ID of an existing lease to the proposed lease ID.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `proposed_lease_id` - The proposed lease ID for the container.
    /// * `options` - Optional configuration for the request.
    pub async fn change_lease(
        &self,
        lease_id: String,
        proposed_lease_id: String,
        options: Option<BlobContainerClientChangeLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientChangeLeaseResult, NoFormat>> {
        self.client
            .change_lease(lease_id, proposed_lease_id, options)
            .await
    }

    /// Frees the lease so that another client can immediately acquire a lease
    /// against the container as soon as the release is complete.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional configuration for the request.
    pub async fn release_lease(
        &self,
        lease_id: String,
        options: Option<BlobContainerClientReleaseLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientReleaseLeaseResult, NoFormat>> {
        self.client.release_lease(lease_id, options).await
    }

    /// Renews the lease on a container.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - A lease ID for the source path. The source path must have an active lease and the
    ///   lease ID must match.
    /// * `options` - Optional configuration for the request.
    pub async fn renew_lease(
        &self,
        lease_id: String,
        options: Option<BlobContainerClientRenewLeaseOptions<'_>>,
    ) -> Result<Response<BlobContainerClientRenewLeaseResult, NoFormat>> {
        self.client.renew_lease(lease_id, options).await
    }

    /// Gets information related to the Storage account in which the container resides.
    /// This includes the `sku_name` and `account_kind`.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_account_info(
        &self,
        options: Option<BlobContainerClientGetAccountInfoOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetAccountInfoResult, NoFormat>> {
        self.client.get_account_info(options).await
    }
}
