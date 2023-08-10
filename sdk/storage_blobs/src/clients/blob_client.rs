use crate::{
    blob::operations::*,
    options::{BA512Range, Snapshot, Tags},
    prelude::*,
};
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    prelude::*,
    Body, Method, Request, Response, StatusCode,
};
use azure_storage::{
    prelude::*,
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignature, BlobSignedResource},
        SasToken,
    },
    StorageCredentials,
};
use futures::StreamExt;
use time::OffsetDateTime;
use url::Url;

/// A client for handling blobs
///
/// For a full list of operations available on blobs, check out [the Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/operations-on-blobs).
#[derive(Debug, Clone)]
pub struct BlobClient {
    container_client: ContainerClient,
    blob_name: String,
}

impl BlobClient {
    pub(crate) fn new(container_client: ContainerClient, blob_name: String) -> Self {
        Self {
            container_client,
            blob_name,
        }
    }

    pub fn from_sas_url(url: &Url) -> azure_core::Result<Self> {
        let container_client = ContainerClient::from_sas_url(url)?;
        // TODO: this currently only works for cloud locations Public and China
        let path: Vec<_> = url.path().split_terminator('/').skip(2).collect();
        if path.is_empty() {
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                || "unable to find blob path",
            ))
        } else {
            let path = path.join("/");
            Ok(container_client.blob_client(path))
        }
    }

    /// Stream a blob in chunks.
    ///
    /// By default, blobs are downloaded in 1MB chunks to reduce the impact of
    /// intermittent network issues while downloading large blobs.
    pub fn get(&self) -> GetBlobBuilder {
        GetBlobBuilder::new(self.clone())
    }

    /// Return an entire blob.
    pub async fn get_content(&self) -> azure_core::Result<Vec<u8>> {
        let mut blob = Vec::new();
        // NOTE: this uses the default chunk size of 1MB, which enables the
        // pipeline to handle intermitent connection failures with retry, rather
        // than restarting the whole blob on a failure.
        let mut stream = self.get().into_stream();
        while let Some(value) = stream.next().await {
            let data = value?.data.collect().await?;
            blob.extend(&data);
        }
        Ok(blob)
    }

    /// Get all user-defined metadata, standard HTTP properties, and system properties for the blob.
    pub fn get_properties(&self) -> GetPropertiesBuilder {
        GetPropertiesBuilder::new(self.clone())
    }

    /// Set blob properties.
    ///
    /// Several properties are cleared from the blob if not passed.
    /// Consider calling `set_from_blob_properties` with existing blob properties.
    pub fn set_properties(&self) -> SetPropertiesBuilder {
        SetPropertiesBuilder::new(self.clone())
    }

    /// Get all user-defined metadata for the blob.
    pub fn get_metadata(&self) -> GetMetadataBuilder {
        GetMetadataBuilder::new(self.clone())
    }

    /// Set all user-defined metadata of the blob
    pub fn set_metadata(&self) -> SetMetadataBuilder {
        SetMetadataBuilder::new(self.clone())
    }

    /// Set the access tier on the blob.
    pub fn set_blob_tier(&self, access_tier: AccessTier) -> SetBlobTierBuilder {
        SetBlobTierBuilder::new(self.clone(), access_tier)
    }

    /// Set an expiry time on an existing blob.
    ///
    /// This operation is only allowed on Hierarchical Namespace enabled
    /// accounts.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/set-blob-expiry>
    pub fn set_blob_expiry(&self, blob_expiry: BlobExpiry) -> SetBlobExpiryBuilder {
        SetBlobExpiryBuilder::new(self.clone(), blob_expiry)
    }

    /// Creates a new page blob.
    pub fn put_page_blob(&self, length: u128) -> PutPageBlobBuilder {
        PutPageBlobBuilder::new(self.clone(), length)
    }

    /// Creates a new append blob.
    pub fn put_append_blob(&self) -> PutAppendBlobBuilder {
        PutAppendBlobBuilder::new(self.clone())
    }

    /// Creates a new block blob, or update the content of an existing block blob.
    pub fn put_block_blob(&self, body: impl Into<Body>) -> PutBlockBlobBuilder {
        PutBlockBlobBuilder::new(self.clone(), body.into())
    }

    /// Copy the blob to a destination within the storage account.
    pub fn copy(&self, copy_source: Url) -> CopyBlobBuilder {
        CopyBlobBuilder::new(self.clone(), copy_source)
    }

    /// Copy the blob to a destination within the storage account synchronously.
    pub fn copy_from_url(&self, copy_source: Url) -> CopyBlobFromUrlBuilder {
        CopyBlobFromUrlBuilder::new(self.clone(), copy_source)
    }

    /// Create a lease on the blob to lock for write and delete operations.
    pub fn acquire_lease<LD: Into<LeaseDuration>>(
        &self,
        lease_duration: LD,
    ) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self.clone(), lease_duration.into())
    }

    /// End the lease but ensure that another client cannot acquire a new lease until the current lease period has expired.
    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self.clone())
    }

    /// Delete the blob.
    pub fn delete(&self) -> DeleteBlobBuilder {
        DeleteBlobBuilder::new(self.clone())
    }

    /// Delete a snapshot of the blob.
    pub fn delete_snapshot(&self, snapshot: Snapshot) -> DeleteBlobSnapshotBuilder {
        DeleteBlobSnapshotBuilder::new(self.clone(), snapshot)
    }

    /// Delete the blob at a specific version.
    pub fn delete_version_id(&self, version_id: VersionId) -> DeleteBlobVersionBuilder {
        DeleteBlobVersionBuilder::new(self.clone(), version_id)
    }

    /* Operations specific to certain blob types */

    /// Creates a new block to be committed as part of a block blob.
    pub fn put_block(
        &self,
        block_id: impl Into<BlockId>,
        body: impl Into<Body>,
    ) -> PutBlockBuilder {
        PutBlockBuilder::new(self.clone(), block_id.into(), body.into())
    }

    /// Creates a new block to be committed as part of a block blob, from a URL.
    pub fn put_block_url(
        &self,
        block_id: impl Into<BlockId>,
        copy_source: Url,
    ) -> PutBlockUrlBuilder {
        PutBlockUrlBuilder::new(self.clone(), block_id.into(), copy_source)
    }

    /// Retrieve the list of blocks that have been uploaded as part of a block blob.
    pub fn get_block_list(&self) -> GetBlockListBuilder {
        GetBlockListBuilder::new(self.clone())
    }

    /// Retrieve the user-defined tags for the specified blob, version, or snapshot.
    pub fn get_tags(&self) -> GetTagsBuilder {
        GetTagsBuilder::new(self.clone())
    }

    /// Set user-defined tags for the specified blob, version, or snapshot.
    pub fn set_tags(&self, tags: impl Into<Tags>) -> SetTagsBuilder {
        SetTagsBuilder::new(self.clone(), tags.into())
    }

    /// Write a block blob by specifying the list of block IDs that make up the blob.
    ///
    /// In order to be written as part of a blob, a block must have been successfully written to the server in a prior Put Block operation.
    pub fn put_block_list(&self, block_list: BlockList) -> PutBlockListBuilder {
        PutBlockListBuilder::new(self.clone(), block_list)
    }

    /// Write a range of pages to a page blob.
    pub fn put_page(&self, ba512_range: BA512Range, content: impl Into<Body>) -> PutPageBuilder {
        PutPageBuilder::new(self.clone(), ba512_range, content.into())
    }

    /// Return the list of valid page ranges for a page blob or snapshot of a page blob.
    pub fn get_page_ranges(&self) -> GetPageRangesBuilder {
        GetPageRangesBuilder::new(self.clone())
    }

    /// Commits a new block of data to the end of an existing append blob.
    pub fn append_block(&self, body: impl Into<Body>) -> AppendBlockBuilder {
        AppendBlockBuilder::new(self.clone(), body.into())
    }

    /// Clear range of pages in a page blob.
    pub fn clear_page(&self, ba512_range: BA512Range) -> ClearPageBuilder {
        ClearPageBuilder::new(self.clone(), ba512_range)
    }

    /// Create a shared access signature.
    pub fn shared_access_signature(
        &self,
        permissions: BlobSasPermissions,
        expiry: OffsetDateTime,
    ) -> azure_core::Result<BlobSharedAccessSignature> {
        match self.container_client.credentials() {
            StorageCredentials::Key(account, ref key) => {

        let canonicalized_resource = format!(
            "/blob/{}/{}/{}",
            account,
            self.container_client.container_name(),
            self.blob_name()
        );
                Ok(
                BlobSharedAccessSignature::new(key.to_string(), canonicalized_resource, permissions, expiry, BlobSignedResource::Blob)
            )},
            _ => Err(Error::message(ErrorKind::Credential,
                "Shared access signature generation - SAS can be generated only from key and account clients",
            )),
        }
    }

    /// Create a signed blob url
    pub fn generate_signed_blob_url<T>(&self, signature: &T) -> azure_core::Result<url::Url>
    where
        T: SasToken,
    {
        let mut url = self.url()?;
        url.set_query(Some(&signature.token()));
        Ok(url)
    }

    /// Check whether blob exists.
    pub async fn exists(&self) -> azure_core::Result<bool> {
        match self.get_properties().await {
            Ok(_) => Ok(true),
            Err(err)
                if err
                    .as_http_error()
                    .map(|e| e.status() == StatusCode::NotFound)
                    .unwrap_or_default() =>
            {
                Ok(false)
            }
            Err(err) => Err(err),
        }
    }

    /// Create a blob snapshot
    pub fn snapshot(&self) -> SnapshotBlobBuilder {
        SnapshotBlobBuilder::new(self.clone())
    }

    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    /// Turn into a `BlobLeaseClient`
    pub fn blob_lease_client(&self, lease_id: LeaseId) -> BlobLeaseClient {
        BlobLeaseClient::new(self.clone(), lease_id)
    }

    pub fn container_client(&self) -> &ContainerClient {
        &self.container_client
    }

    /// Full URL for the blob.
    pub fn url(&self) -> azure_core::Result<url::Url> {
        let blob_name = self
            .blob_name()
            .strip_prefix('/')
            .unwrap_or_else(|| self.blob_name());
        let url = format!("{}/{}", self.container_client().url()?, blob_name);
        Ok(url::Url::parse(&url)?)
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.container_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.container_client.send(context, request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_url() -> azure_core::Result<()> {
        let path = "my/complex/nested/path/here";
        let container = "mycontainer";
        let account = "accountname";

        let example = format!("https://{account}.blob.core.windows.net/{container}/{path}?token=1");
        let url = Url::parse(&example)?;
        let blob_client = BlobClient::from_sas_url(&url)?;

        assert_eq!(blob_client.blob_name(), path);
        assert_eq!(blob_client.container_client().container_name(), container);

        let creds = blob_client.container_client.credentials();
        assert!(matches!(creds, StorageCredentials::SASToken(_)));

        let url = Url::parse("https://accountname.blob.core.windows.net/mycontainer/myblob")?;
        assert!(BlobClient::from_sas_url(&url).is_err(), "missing token");

        let url = Url::parse("https://accountname.blob.core.windows.net/mycontainer?token=1")?;
        assert!(BlobClient::from_sas_url(&url).is_err(), "missing path");

        let url = Url::parse("https://accountname.blob.core.windows.net/?token=1")?;
        assert!(BlobClient::from_sas_url(&url).is_err(), "missing container");

        let example =
            format!("https://{account}.blob.core.chinacloudapi.cn/{container}/{path}?token=1");
        let url = Url::parse(&example)?;
        let blob_client = BlobClient::from_sas_url(&url)?;

        assert_eq!(blob_client.blob_name(), path);
        assert_eq!(blob_client.container_client().container_name(), container);

        Ok(())
    }

    struct FakeSas {
        token: String,
    }
    impl SasToken for FakeSas {
        fn token(&self) -> String {
            self.token.clone()
        }
    }

    fn build_url(container_name: &str, blob_name: &str, sas: &FakeSas) -> url::Url {
        let blob_client = ClientBuilder::emulator().blob_client(container_name, blob_name);
        blob_client
            .generate_signed_blob_url(sas)
            .expect("build url failed")
    }

    #[test]
    fn test_generate_url() {
        let sas = FakeSas {
            token: "fake_token".to_owned(),
        };

        let url = build_url("a", "b", &sas);
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:10000/devstoreaccount1/a/b?fake_token"
        );

        let url = build_url("a", "b/c/d", &sas);
        assert_eq!(
            url.as_str(),
            "http://127.0.0.1:10000/devstoreaccount1/a/b/c/d?fake_token"
        );
    }
}
