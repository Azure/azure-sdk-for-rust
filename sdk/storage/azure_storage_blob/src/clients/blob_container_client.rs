// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::clients::BlobContainerClient as GeneratedBlobContainerClient,
    generated::models::{
        BlobContainerClientAcquireLeaseOptions, BlobContainerClientAcquireLeaseResult,
        BlobContainerClientBreakLeaseOptions, BlobContainerClientBreakLeaseResult,
        BlobContainerClientChangeLeaseOptions, BlobContainerClientChangeLeaseResult,
        BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
        BlobContainerClientFindBlobsByTagsOptions, BlobContainerClientGetAccessPolicyOptions,
        BlobContainerClientGetAccountInfoOptions, BlobContainerClientGetAccountInfoResult,
        BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
        BlobContainerClientListBlobFlatSegmentOptions, BlobContainerClientReleaseLeaseOptions,
        BlobContainerClientReleaseLeaseResult, BlobContainerClientRenewLeaseOptions,
        BlobContainerClientRenewLeaseResult, BlobContainerClientSetAccessPolicyOptions,
        BlobContainerClientSetMetadataOptions, SignedIdentifiers,
    },
    models::{FilterBlobSegment, ListBlobsFlatSegmentResponse, StorageErrorCode},
    pipeline::StorageHeadersPolicy,
    BlobClient, BlobContainerClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pager, Pipeline, RequestContent, Response, StatusCode, Url, XmlFormat,
    },
    tracing, Result,
};
use std::{collections::HashMap, sync::Arc};

/// A client to interact with a specified Azure storage container, although that container may not yet exist.
pub struct BlobContainerClient {
    pub(super) client: GeneratedBlobContainerClient,
}

impl GeneratedBlobContainerClient {
    /// Creates a new GeneratedBlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn from_url(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            ));
            vec![auth_policy]
        } else {
            Vec::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: container_url,
            version: options.version,
            pipeline,
        })
    }
}

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?;
            path_segments.extend([container_name]);
        }

        let client = GeneratedBlobContainerClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_url(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedBlobContainerClient::from_url(container_url, credential, options)?;

        Ok(Self { client })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as container URL has already been validated on client construction.
            .expect("Invalid endpoint URL: Cannot append blob_name to the blob endpoint.")
            .extend([blob_name]);

        let client = GeneratedBlobClient {
            endpoint: blob_url,
            pipeline: self.client.pipeline.clone(),
            version: self.client.version.clone(),
            tracer: self.client.tracer.clone(),
        };

        BlobClient { client }
    }

    /// Gets the URL of the resource this client is configured for.
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
    /// * `metadata` - A [`HashMap`] containing the metadata key-value pairs to set for the container.
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        metadata: HashMap<String, String>,
        options: Option<BlobContainerClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(&metadata, options).await
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
    ) -> Result<Pager<ListBlobsFlatSegmentResponse, XmlFormat, String>> {
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
        options: Option<BlobContainerClientFindBlobsByTagsOptions<'_>>,
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

    /// Checks if the container exists.
    ///
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

    /// Sets the permissions for the specified container. The permissions indicate whether blobs in a
    /// container may be accessed publicly.
    ///
    /// # Arguments
    ///
    /// * `container_acl` - The access control list for the container. You can create this from a
    ///   [`HashMap<String, AccessPolicy>`] by converting it to [`SignedIdentifiers`] and wrapping it into a RequestContent.
    /// * `options` - Optional configuration for the request.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// use azure_core::http::RequestContent;
    /// use azure_storage_blob::models::{AccessPolicy, SignedIdentifiers};
    /// use std::collections::HashMap;
    /// use typespec_client_core::time::OffsetDateTime;
    ///
    /// let mut policies = HashMap::new();
    /// policies.insert("some_policy_id".to_string(), AccessPolicy {
    ///     start: Some(OffsetDateTime::now_utc()),
    ///     expiry: Some(OffsetDateTime::now_utc() + Duration::from_secs(10)),
    ///     permission: Some("rwd".to_string()),
    /// });
    ///
    /// let signed_identifiers: SignedIdentifiers = policies.into();
    /// let request_content = RequestContent::try_from(signed_identifiers)?;
    /// container_client.set_access_policy(request_content, None).await?;
    /// ```
    pub async fn set_access_policy(
        &self,
        container_acl: RequestContent<SignedIdentifiers, XmlFormat>,
        options: Option<BlobContainerClientSetAccessPolicyOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_access_policy(container_acl, options).await
    }

    /// Gets the permissions for the specified container. The permissions indicate whether container data
    /// may be accessed publicly.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_access_policy(
        &self,
        options: Option<BlobContainerClientGetAccessPolicyOptions<'_>>,
    ) -> Result<Response<SignedIdentifiers, XmlFormat>> {
        self.client.get_access_policy(options).await
    }
}
