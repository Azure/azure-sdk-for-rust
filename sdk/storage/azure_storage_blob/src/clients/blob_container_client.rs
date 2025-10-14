// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::clients::ContainerClient as GeneratedBlobContainerClient,
    generated::models::{
        ContainerClientAcquireLeaseOptions, ContainerClientAcquireLeaseResult,
        ContainerClientBreakLeaseOptions, ContainerClientBreakLeaseResult,
        ContainerClientChangeLeaseOptions, ContainerClientChangeLeaseResult,
        ContainerClientCreateOptions, ContainerClientDeleteOptions,
        ContainerClientFindBlobsByTagsOptions, ContainerClientGetAccountInfoOptions,
        ContainerClientGetAccountInfoResult, ContainerClientGetPropertiesOptions,
        ContainerClientGetPropertiesResult, ContainerClientListBlobFlatSegmentOptions,
        ContainerClientReleaseLeaseOptions, ContainerClientReleaseLeaseResult,
        ContainerClientRenewLeaseOptions, ContainerClientRenewLeaseResult,
        ContainerClientSetMetadataOptions,
    },
    models::{FilterBlobSegment, ListBlobsFlatSegmentResponse, StorageErrorCode},
    pipeline::StorageHeadersPolicy,
    BlobClient, ContainerClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, PageIterator, Pager, Response, StatusCode, Url, XmlFormat,
    },
    Result,
};
use std::{collections::HashMap, sync::Arc};

/// A client to interact with a specified Azure storage container.
pub struct BlobContainerClient {
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
        options: Option<ContainerClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let mut url = Url::parse(endpoint)?;
        if !url.scheme().starts_with("http") {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{url} must use http(s)"),
            ));
        }

        // Build Container URL, Url crate handles encoding only path params
        url.path_segments_mut()
            .expect("Cannot be base")
            .push(&container_name);

        let client = GeneratedBlobContainerClient::new(url.as_str(), credential, Some(options))?;
        Ok(Self { client })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: String) -> BlobClient {
        // Copy exact logic from new() constructor, but assume container is already contained in URL
        let mut blob_url = self.url().clone();
        // Build Blob URL, Url crate handles encoding only path params
        blob_url.path_segments_mut()
            .expect("Invalid endpoint URL: Cannot append container_name and blob_name to the blob endpoint.")
            .extend([blob_name.clone()]);

        let client = GeneratedBlobClient {
            endpoint: blob_url.clone(),
            pipeline: self.client.pipeline.clone(),
            version: self.client.version.clone(),
            tracer: self.client.tracer.clone(),
        };

        BlobClient {
            endpoint: blob_url,
            client,
        }
    }

    /// Gets the URL of the Storage account this client is connected to.
    pub fn url(&self) -> &Url {
        &self.client.endpoint
    }

    /// Creates a new container under the specified account. If the container with the same name already exists, the operation fails.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn create_container(
        &self,
        options: Option<ContainerClientCreateOptions<'_>>,
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
        options: Option<ContainerClientSetMetadataOptions<'_>>,
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
        options: Option<ContainerClientDeleteOptions<'_>>,
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
        options: Option<ContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<ContainerClientGetPropertiesResult, NoFormat>> {
        self.client.get_properties(options).await
    }

    /// Returns a list of the blobs under the specified container.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub fn list_blobs(
        &self,
        options: Option<ContainerClientListBlobFlatSegmentOptions<'_>>,
    ) -> Result<PageIterator<Response<ListBlobsFlatSegmentResponse, XmlFormat>>> {
        self.client.list_blob_flat_segment(options)
    }

    /// Returns a list of blobs in the container whose tags match a given search expression.
    ///
    /// # Arguments
    ///
    /// * `filter_expression` - The expression to find blobs whose tags matches the specified condition.
    ///   eg.
    /// ```text
    /// "\"yourtagname\"='firsttag' and \"yourtagname2\"='secondtag'"
    /// ```
    ///   To specify a container, eg.
    /// ```text
    /// "@container='containerName' and \"Name\"='C'"
    /// ```
    /// See [`format_filter_expression()`](crate::format_filter_expression) for help with the expected String format.
    /// * `options` - Optional parameters for the request.
    pub async fn find_blobs_by_tags(
        &self,
        filter_expression: &str,
        options: Option<ContainerClientFindBlobsByTagsOptions<'_>>,
    ) -> Result<Response<FilterBlobSegment, XmlFormat>> {
        self.client
            .find_blobs_by_tags(filter_expression, options)
            .await
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
        options: Option<ContainerClientAcquireLeaseOptions<'_>>,
    ) -> Result<Response<ContainerClientAcquireLeaseResult, NoFormat>> {
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
        options: Option<ContainerClientBreakLeaseOptions<'_>>,
    ) -> Result<Response<ContainerClientBreakLeaseResult, NoFormat>> {
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
        options: Option<ContainerClientChangeLeaseOptions<'_>>,
    ) -> Result<Response<ContainerClientChangeLeaseResult, NoFormat>> {
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
        options: Option<ContainerClientReleaseLeaseOptions<'_>>,
    ) -> Result<Response<ContainerClientReleaseLeaseResult, NoFormat>> {
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
        options: Option<ContainerClientRenewLeaseOptions<'_>>,
    ) -> Result<Response<ContainerClientRenewLeaseResult, NoFormat>> {
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
        options: Option<ContainerClientGetAccountInfoOptions<'_>>,
    ) -> Result<Response<ContainerClientGetAccountInfoResult, NoFormat>> {
        self.client.get_account_info(options).await
    }

    /// Returns `true` if the container exists, `false` if the container does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.client.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::ContainerNotFound.as_ref() => Ok(false),
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
