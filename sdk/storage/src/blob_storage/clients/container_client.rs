use crate::blob_storage::prelude::PublicAccess;
use crate::container::requests::*;
use crate::core::clients::{StorageAccountClient, StorageClient};
use azure_core::errors::AzureError;
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
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
