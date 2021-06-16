use crate::blob::prelude::PublicAccess;
use crate::container::requests::*;
use crate::core::clients::{StorageAccountClient, StorageClient};
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsContainerClient<CN: Into<String>> {
    fn as_container_client(&self, container_name: CN) -> Arc<ContainerClient>;
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<StorageClient> {
    fn as_container_client(&self, container_name: CN) -> Arc<ContainerClient> {
        ContainerClient::new(self.clone(), container_name.into())
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

    pub(crate) fn url_with_segments<'a, I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
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
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{blob::clients::AsBlobClient, core::prelude::*};
    use url::Url;

    fn get_emulator_client(container_name: &str) -> Arc<ContainerClient> {
        let blob_storage_url =
            Url::parse("http://127.0.0.1:10000").expect("the default local storage emulator URL");
        let queue_storage_url =
            Url::parse("http://127.0.0.1:10001").expect("the default local storage emulator URL");
        let table_storage_url =
            Url::parse("http://127.0.0.1:10002").expect("the default local storage emulator URL");
        let filesystem_url =
            Url::parse("http://127.0.0.1:10004").expect("the default local storage emulator URL");

        let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());
        let storage_account = StorageAccountClient::new_emulator(
            http_client,
            &blob_storage_url,
            &table_storage_url,
            &queue_storage_url,
            &filesystem_url,
        )
        .as_storage_client();

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
