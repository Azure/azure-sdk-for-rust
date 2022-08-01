use crate::{clients::*, container::operations::*, prelude::PublicAccess};
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    prelude::*,
    Body, Method, Request, Response, Url,
};
use azure_storage::{
    core::clients::{ServiceType, StorageClient, StorageCredentials},
    prelude::BlobSasPermissions,
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignature, BlobSignedResource},
        SasToken,
    },
};
use time::OffsetDateTime;

pub trait AsContainerClient {
    fn container_client(&self, container_name: impl Into<String>) -> ContainerClient;
}

impl AsContainerClient for StorageClient {
    fn container_client(&self, container_name: impl Into<String>) -> ContainerClient {
        ContainerClient::new(self.clone(), container_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct ContainerClient {
    storage_client: StorageClient,
    container_name: String,
}

impl ContainerClient {
    pub(crate) fn new(storage_client: StorageClient, container_name: String) -> Self {
        Self {
            storage_client,
            container_name,
        }
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

    pub fn container_lease_client(&self, lease_id: LeaseId) -> ContainerLeaseClient {
        ContainerLeaseClient::new(self.clone(), lease_id)
    }

    pub fn blob_client<BN: Into<String>>(&self, blob_name: BN) -> BlobClient {
        BlobClient::new(self.clone(), blob_name.into())
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn storage_client(&self) -> &StorageClient {
        &self.storage_client
    }

    pub fn shared_access_signature(
        &self,
        permissions: BlobSasPermissions,
        expiry: OffsetDateTime,
    ) -> azure_core::Result<BlobSharedAccessSignature> {
        let canonicalized_resource = format!(
            "/blob/{}/{}",
            self.storage_client().account(),
            self.container_name(),
        );

        match self.storage_client().storage_credentials() {
            StorageCredentials::Key(_, key) => Ok(
                BlobSharedAccessSignature::new(key.to_string(), canonicalized_resource, permissions, expiry, BlobSignedResource::Container),
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
        let mut url = self.url()?;
        url.set_query(Some(&signature.token()));
        Ok(url)
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        self.storage_client
            .blob_url_with_segments(Some(self.container_name.as_str()).into_iter())
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

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.storage_client
            .finalize_request(url, method, headers, request_body)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{blob::clients::AsBlobClient, core::prelude::*};

    fn get_emulator_client(container_name: &str) -> ContainerClient {
        let storage_account = StorageClient::new_emulator_default();

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
