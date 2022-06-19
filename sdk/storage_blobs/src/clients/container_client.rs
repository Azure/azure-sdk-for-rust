use crate::{container::requests::*, prelude::PublicAccess};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    prelude::*,
    HttpClient, Request,
};
use azure_storage::{
    core::clients::{AsStorageClient, StorageAccountClient, StorageClient, StorageCredentials},
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignatureBuilder, BlobSignedResource, SetResources},
        SasToken,
    },
};
use bytes::Bytes;
use http::method::Method;
use std::sync::Arc;

pub trait AsContainerClient<CN: Into<String>> {
    fn as_container_client(&self, container_name: CN) -> Arc<ContainerClient>;
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<StorageClient> {
    fn as_container_client(&self, container_name: CN) -> Arc<ContainerClient> {
        ContainerClient::new(self.clone(), container_name.into())
    }
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<StorageAccountClient> {
    fn as_container_client(&self, container_name: CN) -> Arc<ContainerClient> {
        self.as_storage_client().as_container_client(container_name)
    }
}

#[derive(Debug, Clone)]
pub struct ContainerClient {
    storage_client: Arc<StorageClient>,
    container_name: String,
}

impl ContainerClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>, container_name: String) -> Arc<Self> {
        Arc::new(Self {
            storage_client,
            container_name,
        })
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.storage_client.as_ref()
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.storage_client.storage_account_client().http_client()
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_client.storage_account_client()
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.storage_client.blob_url_with_segments(
            Some(self.container_name.as_str())
                .into_iter()
                .chain(segments),
        )
    }

    pub fn create(&self) -> CreateBuilder {
        CreateBuilder::new(self)
    }

    pub fn delete(&self) -> DeleteBuilder {
        DeleteBuilder::new(self)
    }

    pub fn get_acl(&self) -> GetACLBuilder {
        GetACLBuilder::new(self)
    }

    pub fn set_acl(&self, public_access: PublicAccess) -> SetACLBuilder {
        SetACLBuilder::new(self, public_access)
    }

    pub fn get_properties(&self) -> GetPropertiesBuilder {
        GetPropertiesBuilder::new(self)
    }

    pub fn list_blobs(&self) -> ListBlobsBuilder {
        ListBlobsBuilder::new(self)
    }

    pub fn acquire_lease<LD: Into<LeaseDuration>>(
        &self,
        lease_duration: LD,
    ) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self, lease_duration.into())
    }

    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self)
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: Method,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.storage_client
            .prepare_request(url, method, request_body)
            .map_kind(ErrorKind::DataConversion)
    }

    pub fn shared_access_signature(
        &self,
    ) -> azure_core::Result<BlobSharedAccessSignatureBuilder<(), SetResources, ()>> {
        let canonicalized_resource = format!(
            "/blob/{}/{}",
            self.storage_account_client().account(),
            self.container_name(),
        );

        match self.storage_account_client().storage_credentials() {
            StorageCredentials::Key(ref _account, ref key) => Ok(
                BlobSharedAccessSignatureBuilder::new(key.to_string(), canonicalized_resource)
                    .with_resources(BlobSignedResource::Container),
            ),
            _ => Err(Error::message(ErrorKind::Credential,
                "Shared access signature generation - SAS can be generated only from key and account clients",
            )),
        }
    }

    pub fn generate_signed_container_url<T>(&self, signature: &T) -> azure_core::Result<url::Url>
    where
        T: SasToken,
    {
        let mut url = self.url_with_segments(None)?;
        url.set_query(Some(&signature.token()));
        Ok(url)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{blob::clients::AsBlobClient, core::prelude::*};

    fn get_emulator_client(container_name: &str) -> Arc<ContainerClient> {
        let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();

        storage_account.as_container_client(container_name)
    }

    #[tokio::test]
    async fn test_create_delete() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .execute()
            .await
            .expect("create container should succeed");
        container_client
            .delete()
            .execute()
            .await
            .expect("delete container should succeed");
    }

    #[tokio::test]
    async fn test_list_blobs() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .execute()
            .await
            .expect("create container should succeed");

        let md5 = md5::compute("world");
        container_client
            .as_blob_client("hello.txt")
            .put_block_blob("world")
            .execute()
            .await
            .expect("put block blob should succeed");
        let list = container_client
            .list_blobs()
            .execute()
            .await
            .expect("list blobs should succeed");
        assert_eq!(list.blobs.blobs.len(), 1);
        assert_eq!(list.blobs.blobs[0].name, "hello.txt");
        assert_eq!(
            list.blobs.blobs[0]
                .properties
                .content_md5
                .as_ref()
                .expect("has content_md5")
                .as_slice(),
            &md5.0
        );

        container_client
            .delete()
            .execute()
            .await
            .expect("delete container should succeed");
    }
}
