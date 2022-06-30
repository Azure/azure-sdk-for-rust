use crate::{blob::operations::*, prelude::*, BA512Range};
use azure_core::Method;
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    prelude::*,
    Request, Response,
};
use azure_storage::core::{
    clients::StorageCredentials,
    prelude::*,
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignatureBuilder, BlobSignedResource, SetResources},
        SasToken,
    },
};
use bytes::Bytes;
use futures::StreamExt;
use std::sync::Arc;
use url::Url;

pub trait AsBlobClient<BN: Into<String>> {
    fn blob_client(&self, blob_name: BN) -> Arc<BlobClient>;
}

impl<BN: Into<String>> AsBlobClient<BN> for Arc<ContainerClient> {
    fn blob_client(&self, blob_name: BN) -> Arc<BlobClient> {
        BlobClient::new(self.clone(), blob_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct BlobClient {
    container_client: Arc<ContainerClient>,
    blob_name: String,
}

impl BlobClient {
    pub(crate) fn new(container_client: Arc<ContainerClient>, blob_name: String) -> Arc<Self> {
        Arc::new(Self {
            container_client,
            blob_name,
        })
    }

    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    #[allow(dead_code)]
    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.container_client.storage_client()
    }

    #[allow(dead_code)]
    pub(crate) fn container_client(&self) -> &ContainerClient {
        self.container_client.as_ref()
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let blob_name_with_segments = self.blob_name.split('/').into_iter().chain(segments);
        self.container_client
            .url_with_segments(blob_name_with_segments)
    }

    // stream blob downloads
    //
    // By default, blobs are downloaded in 1MB chunks to reduce the impact of
    // intermittent network issues while downloading large blobs.
    pub fn get(&self) -> GetBlobBuilder {
        GetBlobBuilder::new(self.clone())
    }

    // helper function that returns the entire blob stream
    pub async fn get_content(&self) -> azure_core::Result<Vec<u8>> {
        let mut blob = Vec::new();
        // NOTE: this uses the default chunk size of 1MB, which enables the
        // pipeline to handle intermitent connection failures with retry, rather
        // than restarting the whole blob on a failure.
        let mut stream = self.get().into_stream();
        while let Some(value) = stream.next().await {
            let data = value?.data;
            blob.extend(&data);
        }
        Ok(blob)
    }

    pub fn get_properties(&self) -> GetPropertiesBuilder {
        GetPropertiesBuilder::new(self.clone())
    }

    /// Creates a builder for setting blob properties.
    ///
    /// Several properties are cleared from the blob if not passed.
    /// Consider calling `set_from_blob_properties` with existing blob properties.
    pub fn set_properties(&self) -> SetPropertiesBuilder {
        SetPropertiesBuilder::new(self.clone())
    }

    pub fn get_metadata(&self) -> GetMetadataBuilder {
        GetMetadataBuilder::new(self.clone())
    }

    pub fn set_metadata(&self) -> SetMetadataBuilder {
        SetMetadataBuilder::new(self.clone())
    }

    pub fn set_blob_tier(&self) -> SetBlobTierBuilder {
        SetBlobTierBuilder::new(self.clone())
    }

    pub fn update_page(
        &self,
        ba512_range: BA512Range,
        content: impl Into<Bytes>,
    ) -> UpdatePageBuilder {
        UpdatePageBuilder::new(self.clone(), ba512_range, content.into())
    }

    pub fn get_page_ranges(&self) -> GetPageRangesBuilder {
        GetPageRangesBuilder::new(self.clone())
    }

    pub fn delete(&self) -> DeleteBlobBuilder {
        DeleteBlobBuilder::new(self.clone())
    }

    pub fn delete_snapshot(&self, snapshot: Snapshot) -> DeleteBlobSnapshotBuilder {
        DeleteBlobSnapshotBuilder::new(self.clone(), snapshot)
    }

    pub fn delete_version_id(&self, version_id: VersionId) -> DeleteBlobVersionBuilder {
        DeleteBlobVersionBuilder::new(self.clone(), version_id)
    }

    pub fn copy(&self, copy_source: Url) -> CopyBlobBuilder {
        CopyBlobBuilder::new(self.clone(), copy_source)
    }

    pub fn copy_from_url(&self, copy_source: Url) -> CopyBlobFromUrlBuilder {
        CopyBlobFromUrlBuilder::new(self.clone(), copy_source)
    }

    pub fn put_page_blob(&self, length: u128) -> PutPageBlobBuilder {
        PutPageBlobBuilder::new(self.clone(), length)
    }

    pub fn put_append_blob(&self) -> PutAppendBlobBuilder {
        PutAppendBlobBuilder::new(self.clone())
    }

    pub fn get_block_list(&self) -> GetBlockListBuilder {
        GetBlockListBuilder::new(self.clone())
    }

    pub fn put_block_list(&self, block_list: BlockList) -> PutBlockListBuilder {
        PutBlockListBuilder::new(self.clone(), block_list)
    }

    pub fn put_block_blob(&self, body: impl Into<Bytes>) -> PutBlockBlobBuilder {
        PutBlockBlobBuilder::new(self.clone(), body.into())
    }

    pub fn append_block(&self, body: impl Into<Bytes>) -> AppendBlockBuilder {
        AppendBlockBuilder::new(self.clone(), body.into())
    }

    pub fn put_block(
        &self,
        block_id: impl Into<BlockId>,
        body: impl Into<Bytes>,
    ) -> PutBlockBuilder {
        PutBlockBuilder::new(self.clone(), block_id.into(), body.into())
    }

    pub fn clear_page(&self, ba512_range: BA512Range) -> ClearPageBuilder {
        ClearPageBuilder::new(self.clone(), ba512_range)
    }

    pub fn acquire_lease<LD: Into<LeaseDuration>>(
        &self,
        lease_duration: LD,
    ) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self.clone(), lease_duration.into())
    }

    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self.clone())
    }

    pub fn shared_access_signature(
        &self,
    ) -> azure_core::Result<BlobSharedAccessSignatureBuilder<(), SetResources, ()>> {
        let canonicalized_resource = format!(
            "/blob/{}/{}/{}",
            self.container_client.storage_client().account(),
            self.container_client.container_name(),
            self.blob_name()
        );

        match self.storage_client().storage_credentials() {
            StorageCredentials::Key(ref _account, ref key) => Ok(
                BlobSharedAccessSignatureBuilder::new(key.to_string(), canonicalized_resource)
                    .with_resources(BlobSignedResource::Blob),
            ),
            _ => Err(Error::message(ErrorKind::Credential,
                "Shared access signature generation - SAS can be generated only from key and account clients",
            )),
        }
    }

    pub fn generate_signed_blob_url<T>(&self, signature: &T) -> azure_core::Result<url::Url>
    where
        T: SasToken,
    {
        let mut url = self.url_with_segments(None)?;
        url.set_query(Some(&signature.token()));
        Ok(url)
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Bytes>,
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

    pub async fn exists(&self) -> azure_core::Result<bool> {
        let result = self.get_properties().into_future().await.map(|_| true);
        if let Err(err) = result {
            if let ErrorKind::HttpResponse { status, .. } = err.kind() {
                return Ok(status != &404u16);
            } else {
                return Err(err);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::AsBlobClient;

    struct FakeSas {
        token: String,
    }
    impl SasToken for FakeSas {
        fn token(&self) -> String {
            self.token.clone()
        }
    }

    fn build_url(container_name: &str, blob_name: &str, sas: &FakeSas) -> url::Url {
        let storage_account = StorageClient::new_emulator_default();
        storage_account
            .container_client(container_name)
            .blob_client(blob_name)
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

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::blob::clients::AsBlobClient;

    fn get_emulator_client(container_name: &str) -> Arc<ContainerClient> {
        let storage_account = StorageClient::new_emulator_default();
        storage_account.container_client(container_name)
    }

    #[tokio::test]
    async fn test_get_properties() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .into_future()
            .await
            .expect("create container should succeed");

        let md5 = md5::compute("world");
        container_client
            .blob_client("hello.txt")
            .put_block_blob("world")
            .into_future()
            .await
            .expect("put block blob should succeed");
        let properties = container_client
            .blob_client("hello.txt")
            .get_properties()
            .into_future()
            .await
            .expect("get properties should succeed");
        assert_eq!(properties.blob.name, "hello.txt");
        assert_eq!(
            properties
                .blob
                .properties
                .content_md5
                .expect("has content_md5")
                .as_slice(),
            &md5.0
        );

        container_client
            .delete()
            .into_future()
            .await
            .expect("delete container should succeed");
    }
}
