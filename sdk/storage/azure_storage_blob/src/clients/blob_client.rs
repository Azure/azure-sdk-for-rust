// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::AppendBlobClient as GeneratedAppendBlobClient,
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::clients::BlockBlobClient as GeneratedBlockBlobClient,
    generated::clients::PageBlobClient as GeneratedPageBlobClient,
    generated::models::{
        BlobClientAcquireLeaseResult, BlobClientBreakLeaseResult, BlobClientChangeLeaseResult,
        BlobClientCreateSnapshotResult, BlobClientDownloadResult, BlobClientGetAccountInfoResult,
        BlobClientGetPropertiesResult, BlobClientReleaseLeaseResult, BlobClientRenewLeaseResult,
        BlockBlobClientUploadResult,
    },
    models::{
        AccessTier, BlobClientAcquireLeaseOptions, BlobClientBreakLeaseOptions,
        BlobClientChangeLeaseOptions, BlobClientCreateSnapshotOptions,
        BlobClientDeleteImmutabilityPolicyOptions, BlobClientDeleteOptions,
        BlobClientDownloadOptions, BlobClientGetAccountInfoOptions, BlobClientGetPropertiesOptions,
        BlobClientGetTagsOptions, BlobClientReleaseLeaseOptions, BlobClientRenewLeaseOptions,
        BlobClientSetImmutabilityPolicyOptions, BlobClientSetLegalHoldOptions,
        BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTagsOptions,
        BlobClientSetTierOptions, BlobClientUndeleteOptions, BlobTags,
        BlockBlobClientUploadOptions, StorageErrorCode,
    },
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlobClientOptions, BlockBlobClient, PageBlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        AsyncResponse, NoFormat, Pipeline, RequestContent, Response, StatusCode, Url, UrlExt,
        XmlFormat,
    },
    time::OffsetDateTime,
    tracing, Bytes, Result,
};
use std::collections::HashMap;
use std::sync::Arc;

/// A client to interact with a specific Azure storage blob, although that blob may not yet exist.
pub struct BlobClient {
    pub(super) client: GeneratedBlobClient,
}

impl GeneratedBlobClient {
    /// Creates a new GeneratedBlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Blob")]
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !blob_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_url} must use https"),
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
            endpoint: blob_url,
            version: options.version,
            pipeline,
        })
    }
}
impl BlobClient {
    /// Creates a new BlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this blob.
    /// * `blob_name` - The name of the blob to interact with.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        blob_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?;
            path_segments.extend([container_name, blob_name]);
        }

        let client = GeneratedBlobClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new BlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedBlobClient::from_url(blob_url, credential, options)?;

        Ok(Self { client })
    }

    /// Returns a new instance of AppendBlobClient.
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            client: GeneratedAppendBlobClient {
                endpoint: self.client.endpoint.clone(),
                pipeline: self.client.pipeline.clone(),
                version: self.client.version.clone(),
                tracer: self.client.tracer.clone(),
            },
        }
    }

    /// Returns a new instance of BlockBlobClient.
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            client: GeneratedBlockBlobClient {
                endpoint: self.client.endpoint.clone(),
                pipeline: self.client.pipeline.clone(),
                version: self.client.version.clone(),
                tracer: self.client.tracer.clone(),
            },
        }
    }

    /// Returns a new instance of PageBlobClient.
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            client: GeneratedPageBlobClient {
                endpoint: self.client.endpoint.clone(),
                pipeline: self.client.pipeline.clone(),
                version: self.client.version.clone(),
                tracer: self.client.tracer.clone(),
            },
        }
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.client.endpoint
    }

    /// Creates a new BlobClient targeting a specific blob version.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID of the blob to target.
    pub fn with_version(&self, version_id: &str) -> Result<Self> {
        let mut versioned_endpoint = self.client.endpoint.clone();
        {
            let mut query_builder = versioned_endpoint.query_builder();
            query_builder.set_pair("versionid", version_id);
            query_builder.build();
        }

        Ok(Self {
            client: GeneratedBlobClient {
                endpoint: versioned_endpoint,
                pipeline: self.client.pipeline.clone(),
                version: self.client.version.clone(),
                tracer: self.client.tracer.clone(),
            },
        })
    }

    /// Creates a new BlobClient targeting a specific blob snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - The snapshot ID of the blob to target.
    pub fn with_snapshot(&self, snapshot: &str) -> Result<Self> {
        let mut snapshot_endpoint = self.client.endpoint.clone();
        {
            let mut query_builder = snapshot_endpoint.query_builder();
            query_builder.set_pair("snapshot", snapshot);
            query_builder.build();
        }

        Ok(Self {
            client: GeneratedBlobClient {
                endpoint: snapshot_endpoint,
                pipeline: self.client.pipeline.clone(),
                version: self.client.version.clone(),
                tracer: self.client.tracer.clone(),
            },
        })
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
    ) -> Result<AsyncResponse<BlobClientDownloadResult>> {
        self.client.download(options).await
    }

    /// Creates a new blob from a data source.
    ///
    /// # Arguments
    ///
    /// * `data` - The blob data to upload.
    /// * `overwrite` - Whether the blob to be uploaded should overwrite the current data. If True, `upload()` will overwrite the existing data.
    ///   If False, the operation will fail with ResourceExistsError.
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn upload(
        &self,
        data: RequestContent<Bytes, NoFormat>,
        overwrite: bool,
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        self.block_blob_client()
            .client
            .upload(data, content_length, Some(options))
            .await
    }

    /// Sets user-defined metadata for the specified blob as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the blob. To remove all metadata from the blob, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `metadata` - A [`HashMap`] containing the metadata key-value pairs to set for the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        metadata: HashMap<String, String>,
        options: Option<BlobClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(&metadata, options).await
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

    /// Sets tags on a blob. Note that each call to this operation replaces all existing tags. To remove
    /// all tags from the blob, call this operation with no tags specified.
    ///
    /// # Arguments
    ///
    /// * `tags` - Name-value pairs associated with the blob as tag. You can create this from a
    ///   [`HashMap<String, String>`] by converting it to [`BlobTags`] and wrapping it into a RequestContent.
    ///   Tags are case-sensitive. The tag set may contain at most 10 tags.  Tag keys must be between 1 and 128 characters,
    ///   and tag values must be between 0 and 256 characters.
    ///   Valid tag key and value characters include: lowercase and uppercase letters, digits (0-9),
    ///   space (' '), plus (+), minus (-), period (.), solidus (/), colon (:), equals (=), underscore (_)
    /// * `options` - Optional configuration for the request.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// use azure_core::http::RequestContent;
    /// use azure_storage_blob::models::BlobTags;
    /// use std::collections::HashMap;
    ///
    /// let mut tags = HashMap::new();
    /// tags.insert("key".to_string(), "value".to_string());
    ///
    /// let blob_tags: BlobTags = tags.into();
    /// let request_content = RequestContent::try_from(blob_tags)?;
    /// blob_client.set_tags(request_content, None).await?;
    /// ```
    pub async fn set_tags(
        &self,
        tags: RequestContent<BlobTags, XmlFormat>,
        options: Option<BlobClientSetTagsOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_tags(tags, options).await
    }

    /// Gets the tags on a blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_tags(
        &self,
        options: Option<BlobClientGetTagsOptions<'_>>,
    ) -> Result<Response<BlobTags, XmlFormat>> {
        self.client.get_tags(options).await
    }

    /// Gets information related to the Storage account in which the blob resides.
    /// This includes the `sku_name` and `account_kind`.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_account_info(
        &self,
        options: Option<BlobClientGetAccountInfoOptions<'_>>,
    ) -> Result<Response<BlobClientGetAccountInfoResult, NoFormat>> {
        self.client.get_account_info(options).await
    }

    /// Checks if the blob exists.
    ///
    /// Returns `true` if the blob exists, `false` if the blob does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.client.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::BlobNotFound.as_ref()
                    || error_code == StorageErrorCode::ContainerNotFound.as_ref() =>
                {
                    Ok(false)
                }
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Sets the immutability policy on the blob.
    ///
    /// # Arguments
    ///
    /// * `expiry` - Specifies the date time when the blob's immutability policy is set to expire.
    /// * `options` - Optional configuration for the request.
    pub async fn set_immutability_policy(
        &self,
        expiry: &OffsetDateTime,
        options: Option<BlobClientSetImmutabilityPolicyOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_immutability_policy(expiry, options).await
    }

    /// Deletes the immutability policy on the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete_immutability_policy(
        &self,
        options: Option<BlobClientDeleteImmutabilityPolicyOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete_immutability_policy(options).await
    }

    /// Sets a legal hold on the blob.
    ///
    /// # Arguments
    ///
    /// * `legal_hold` - Specifies the legal hold status to set on the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn set_legal_hold(
        &self,
        legal_hold: bool,
        options: Option<BlobClientSetLegalHoldOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_legal_hold(legal_hold, options).await
    }

    /// Undeletes a blob that was previously soft-deleted.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn undelete(
        &self,
        options: Option<BlobClientUndeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.undelete(options).await
    }

    /// Creates a read-only snapshot of a blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn create_snapshot(
        &self,
        options: Option<BlobClientCreateSnapshotOptions<'_>>,
    ) -> Result<Response<BlobClientCreateSnapshotResult, NoFormat>> {
        self.client.create_snapshot(options).await
    }
}
