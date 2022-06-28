use crate::{container::operations::*, prelude::PublicAccess};
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    prelude::*,
    Request, Response,
};
use azure_core::{Method, Url};
use azure_storage::{
    core::clients::{
        AsStorageClient, ServiceType, StorageAccountClient, StorageClient, StorageCredentials,
    },
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignatureBuilder, BlobSignedResource, SetResources},
        SasToken,
    },
};
use bytes::Bytes;
use std::sync::Arc;

pub trait AsContainerClient<CN: Into<String>> {
    fn container_client(&self, container_name: CN) -> Arc<ContainerClient>;
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<StorageClient> {
    fn container_client(&self, container_name: CN) -> Arc<ContainerClient> {
        ContainerClient::new(self.clone(), container_name.into())
    }
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<StorageAccountClient> {
    fn container_client(&self, container_name: CN) -> Arc<ContainerClient> {
        self.storage_client().container_client(container_name)
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
        CreateBuilder::new(self.clone())
    }

    pub fn delete(&self) -> DeleteBuilder {
        DeleteBuilder::new(self.clone())
    }

    pub fn get_acl(&self) -> GetACLBuilder {
        GetACLBuilder::new(self.clone())
    }

    pub fn set_acl(&self, public_access: PublicAccess) -> SetACLBuilder {
        SetACLBuilder::new(self.clone(), public_access)
    }

    pub fn get_properties(&self) -> GetPropertiesBuilder {
        GetPropertiesBuilder::new(self.clone())
    }

    pub fn list_blobs(&self) -> ListBlobsBuilder {
        ListBlobsBuilder::new(self.clone())
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

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.storage_client
            .send(context, request, ServiceType::Blob)
            .await
    }

    pub(crate) fn prepare_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.storage_client
            .prepare_request(url, method, headers, request_body)
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
        let storage_account = StorageAccountClient::new_emulator_default().storage_client();

        storage_account.container_client(container_name)
    }

    #[tokio::test]
    async fn test_create_delete() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .into_future()
            .await
            .expect("create container should succeed");
        container_client
            .delete()
            .into_future()
            .await
            .expect("delete container should succeed");
    }

    #[tokio::test]
    async fn test_list_blobs() {
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
            .into_future()
            .await
            .expect("delete container should succeed");
    }
}
