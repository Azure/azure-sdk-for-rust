// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobClient, BlobClientOptions};

use crate::{
    generated::{
        clients::BlobClient as GeneratedBlobClient, models::BlobClientDownloadInternalOptions,
    },
    models::{
        BlobClientDownloadIntoResult, BlobClientDownloadOptions, BlobClientDownloadResult,
        BlobClientUploadOptions, BlobClientUploadResult, BlobDownloadProperties, HttpRange,
        StorageErrorCode,
    },
    partitioned_transfer::{self, PartitionedDownloadBehavior},
    AppendBlobClient, BlockBlobClient, PageBlobClient,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        AsyncRawResponse, Etag, NoFormat, Pipeline, RequestContent, StatusCode, Url, UrlExt,
    },
    tracing, Bytes, Result,
};
use std::{ops::Range, sync::Arc};

impl BlobClient {
    /// Creates a new BlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Blob")]
    pub fn new(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if blob_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{blob_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !blob_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

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

    /// Returns a new instance of AppendBlobClient.
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Returns a new instance of BlockBlobClient.
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Returns a new instance of PageBlobClient.
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Creates a new BlobClient targeting a specific blob version.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID of the blob to target.
    pub fn with_version(&self, version_id: &str) -> Result<Self> {
        let mut versioned_endpoint = self.endpoint.clone();
        {
            let mut query_builder = versioned_endpoint.query_builder();
            query_builder.set_pair("versionid", version_id);
            query_builder.build();
        }

        Ok(Self {
            endpoint: versioned_endpoint,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }

    /// Creates a new BlobClient targeting a specific blob snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - The snapshot ID of the blob to target.
    pub fn with_snapshot(&self, snapshot: &str) -> Result<Self> {
        let mut snapshot_endpoint = self.endpoint.clone();
        {
            let mut query_builder = snapshot_endpoint.query_builder();
            query_builder.set_pair("snapshot", snapshot);
            query_builder.build();
        }

        Ok(Self {
            endpoint: snapshot_endpoint,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs. The returned
    /// [`BlobClientDownloadResult::body`] contains the complete blob data, while
    /// [`BlobClientDownloadResult::properties`] and [`BlobClientDownloadResult::headers`]
    /// reflect only the initial response's metadata and properties.
    ///
    /// If the streamed bytes of this blob are to be collected into contiguous memory,
    /// consider instead calling [`BlobClient::download_into`] with a pre-allocated buffer
    /// to avoid unnecessary copies and allocations.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    ///
    /// # Notes
    ///
    /// By default, storage clients create their HTTP transport via
    /// [`azure_core::http::new_http_client()`] with automatic decompression disabled.
    /// If you set a custom transport in [`BlobClientOptions`] without also disabling
    /// automatic decompression, partitioned downloads may not succeed.
    #[tracing::function("Storage.Blob.Blob.download")]
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<BlobClientDownloadResult> {
        let options = options.unwrap_or_default();
        let parallel = options
            .parallel
            .unwrap_or_else(crate::partitioned_transfer::defaults::default_concurrency);
        let partition_size = options
            .partition_size
            .unwrap_or(crate::partitioned_transfer::defaults::DEFAULT_DOWNLOAD_PARTITION_SIZE);
        let range = options.range.clone();
        let inner_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let behavior = BlobClientDownloadBehavior::new(inner_client, options.into());
        let response =
            partitioned_transfer::download(range, parallel, partition_size, Arc::new(behavior))
                .await?;
        BlobClientDownloadResult::from_headers(response)
    }

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs. The downloaded bytes are
    /// written directly into the provided `buffer`.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Destination buffer to write the downloaded blob data into.
    /// * `options` - Optional configuration for the request.
    ///
    /// # Notes
    ///
    /// By default, storage clients create their HTTP transport via
    /// [`azure_core::http::new_http_client()`] with automatic decompression disabled.
    /// If you set a custom transport in [`BlobClientOptions`] without also disabling
    /// automatic decompression, partitioned downloads may not succeed.
    #[tracing::function("Storage.Blob.Blob.download_into")]
    pub async fn download_into(
        &self,
        buffer: &mut [u8],
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<BlobClientDownloadIntoResult> {
        let options = options.unwrap_or_default();
        let parallel = options
            .parallel
            .unwrap_or_else(crate::partitioned_transfer::defaults::default_concurrency);
        let partition_size = options
            .partition_size
            .unwrap_or(crate::partitioned_transfer::defaults::DEFAULT_DOWNLOAD_PARTITION_SIZE);
        let range = options.range.clone();
        let inner_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let behavior = BlobClientDownloadBehavior::new(inner_client, options.into());
        let (_, headers, len) = partitioned_transfer::download_into(
            buffer,
            range,
            parallel,
            partition_size,
            Arc::new(behavior),
        )
        .await?;
        Ok(BlobClientDownloadIntoResult {
            len,
            properties: BlobDownloadProperties::from_headers(&headers)?,
            headers,
        })
    }

    /// Uploads content to a block blob, overwriting any existing blob by default.
    ///
    /// Updating an existing block blob overwrites any existing metadata on the blob. Use [`BlobClientUploadOptions::if_not_exists()`] to fail instead of overwriting.
    /// To perform a partial update of the content of a block blob, use [`BlockBlobClient::stage_block()`] and [`BlockBlobClient::commit_block_list()`] directly.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to upload.
    /// * `options` - Optional parameters for the request.
    pub async fn upload(
        &self,
        content: RequestContent<Bytes, NoFormat>,
        options: Option<BlobClientUploadOptions<'_>>,
    ) -> Result<BlobClientUploadResult> {
        self.block_blob_client().upload(content, options).await
    }

    /// Checks if the blob exists.
    ///
    /// Returns `true` if the blob exists, `false` if the blob does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
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

    /// Begins building a user delegation SAS URL for this blob.
    ///
    /// Returns a [`BlobSasBuilder`](crate::models::sas::BlobSasBuilder) pre-initialized with
    /// this blob's resource info and the requested permissions. Chain optional
    /// setters (e.g. [`protocol`](crate::models::sas::BlobSasBuilder::protocol),
    /// [`ip_range`](crate::models::sas::BlobSasBuilder::ip_range), or response header
    /// overrides), then call [`url()`](crate::models::sas::BlobSasBuilder::url) to produce
    /// the signed URL.
    ///
    /// The returned URL can be passed directly to [`BlobClient::new`] with
    /// `None` for the credential to construct a SAS-authenticated client.
    ///
    /// If this client was built with [`BlobClient::with_snapshot`] or
    /// [`BlobClient::with_version`], the snapshot timestamp or version ID is
    /// forwarded automatically and the SAS is scoped accordingly
    /// (`sr=bs` for snapshots, `sr=bv` for versions).
    ///
    /// # Errors
    ///
    /// Returns an error if `key` is missing any required field, or if both
    /// `snapshot=` and `versionid=` are present in the endpoint query.
    ///
    /// # Examples
    ///
    /// Generate a read-only SAS URL with default settings:
    ///
    /// ```no_run
    /// # use azure_storage_blob::BlobClient;
    /// # use azure_storage_blob::models::sas::{BlobPermissions, UserDelegationKey};
    /// # use time::OffsetDateTime;
    /// # fn example(client: &BlobClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client
    ///     .user_delegation_sas(
    ///         "myaccount",
    ///         &udk,
    ///         BlobPermissions::new().read(),
    ///         OffsetDateTime::now_utc() + time::Duration::hours(1),
    ///     )?
    ///     .url();
    /// # Ok(()) }
    /// ```
    ///
    /// Restrict to HTTPS and limit access by IP:
    ///
    /// ```no_run
    /// # use azure_storage_blob::BlobClient;
    /// # use azure_storage_blob::models::sas::{BlobPermissions, UserDelegationKey};
    /// # use azure_storage_sas::{SasIpRange, SasProtocol};
    /// # use std::net::Ipv4Addr;
    /// # use time::OffsetDateTime;
    /// # fn example(client: &BlobClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client
    ///     .user_delegation_sas(
    ///         "myaccount",
    ///         &udk,
    ///         BlobPermissions::new().read().write(),
    ///         OffsetDateTime::now_utc() + time::Duration::hours(4),
    ///     )?
    ///     .protocol(SasProtocol::Https)
    ///     .ip_range(SasIpRange::Range {
    ///         start: Ipv4Addr::new(10, 0, 0, 1).into(),
    ///         end: Ipv4Addr::new(10, 0, 0, 255).into(),
    ///     })
    ///     .content_type("application/octet-stream")
    ///     .url();
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "sas_builder")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sas_builder")))]
    pub fn user_delegation_sas<'a>(
        &self,
        account_name: &str,
        key: &'a azure_storage_common::models::UserDelegationKey,
        permissions: azure_storage_sas::resource::blob::BlobPermissions,
        expiry: time::OffsetDateTime,
    ) -> Result<crate::sas::BlobSasBuilder<'a>> {
        let segments = crate::sas::helpers::resource_path_segments(&self.endpoint, account_name);

        if segments.len() < 2 {
            return Err(azure_core::Error::with_message(
                ErrorKind::Other,
                "blob endpoint URL must include container and blob name",
            ));
        }

        let container = &segments[0];
        let blob_name = segments[1..].join("/");

        let (snapshot, version_id) = crate::sas::helpers::extract_blob_qualifiers(&self.endpoint)?;
        let mut resource =
            azure_storage_sas::resource::blob::BlobResource::new(container, blob_name);
        if let Some(s) = snapshot {
            resource = resource.snapshot(s);
        }
        if let Some(v) = version_id {
            resource = resource.version(v);
        }

        let inner = azure_storage_sas::SasBuilder::new(account_name, key, expiry)?
            .blob(resource, permissions);
        Ok(crate::sas::BlobSasBuilder::new(
            self.endpoint.clone(),
            inner,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sas_builder")]
    #[test]
    fn user_delegation_sas_attaches_sas_query() {
        use crate::models::sas::{BlobPermissions, UserDelegationKey};
        use time::macros::datetime;

        let url = Url::parse("https://acct.blob.core.windows.net/c1/b1").unwrap();
        let client = BlobClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "acct",
                &udk,
                BlobPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        assert_eq!(sas_url.path(), "/c1/b1");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=b"), "got: {query}");
        assert!(query.contains("sig="), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_path_prefix_endpoint_strips_account_segment() {
        use crate::models::sas::{BlobPermissions, UserDelegationKey};
        use time::macros::datetime;

        // Azurite-style: account name is the first path segment.
        let url = Url::parse("http://127.0.0.1:10000/devstoreaccount1/mycontainer/myblob").unwrap();
        let client = BlobClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "devstoreaccount1",
                &udk,
                BlobPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        // The path should be preserved (including the account prefix).
        assert_eq!(sas_url.path(), "/devstoreaccount1/mycontainer/myblob");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=b"), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_container_named_same_as_account_no_false_skip() {
        use crate::models::sas::{BlobPermissions, UserDelegationKey};
        use time::macros::datetime;

        // Container has the same name as the account — must NOT be skipped.
        let url = Url::parse("https://acct.blob.core.windows.net/acct/myblob").unwrap();
        let client = BlobClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "acct",
                &udk,
                BlobPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        // "acct" is the container, not the account path prefix.
        assert_eq!(sas_url.path(), "/acct/myblob");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=b"), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_custom_domain_does_not_skip() {
        use crate::models::sas::{BlobPermissions, UserDelegationKey};
        use time::macros::datetime;

        // Custom domain (CDN / Front Door) — account name not in host.
        let url = Url::parse("https://cdn.contoso.com/pictures/photo.jpg").unwrap();
        let client = BlobClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "myaccount",
                &udk,
                BlobPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        assert_eq!(sas_url.path(), "/pictures/photo.jpg");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=b"), "got: {query}");
    }
}

struct BlobClientDownloadBehavior<'a> {
    client: GeneratedBlobClient,
    options: BlobClientDownloadInternalOptions<'a>,
}

impl<'a> BlobClientDownloadBehavior<'a> {
    fn new(client: GeneratedBlobClient, options: BlobClientDownloadInternalOptions<'a>) -> Self {
        Self { client, options }
    }
}

#[async_trait]
impl PartitionedDownloadBehavior for BlobClientDownloadBehavior<'_> {
    async fn transfer_range(
        &self,
        range: Option<Range<usize>>,
        etag_lock: Option<Etag>,
    ) -> Result<AsyncRawResponse> {
        let mut opt = self.options.clone();
        opt.range = range.map(HttpRange::from);
        if let Some(etag) = etag_lock {
            opt.if_match = Some(etag);
            opt.if_none_match = None;
            opt.if_modified_since = None;
            opt.if_unmodified_since = None;
            opt.if_tags = None;
        }
        self.client
            .download_internal(Some(opt))
            .await
            .map(AsyncRawResponse::from)
    }
}
