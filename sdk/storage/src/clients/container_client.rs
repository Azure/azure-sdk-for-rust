use crate::blob::prelude::PublicAccess;
use crate::clients::StorageClient;
use crate::container::requests::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsContainerClient<CN: Into<String>> {
    fn as_container_client(&self, container_name: CN) -> Arc<Box<ContainerClient>>;
}

impl<CN: Into<String>> AsContainerClient<CN> for Arc<Box<StorageClient>> {
    fn as_container_client(&self, container_name: CN) -> Arc<Box<ContainerClient>> {
        ContainerClient::new(self.clone(), container_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct ContainerClient {
    storage_client: Arc<Box<StorageClient>>,
    container_name: String,
}

impl ContainerClient {
    pub(crate) fn new(
        storage_client: Arc<Box<StorageClient>>,
        container_name: String,
    ) -> Arc<Box<Self>> {
        Arc::new(Box::new(Self {
            storage_client,
            container_name,
        }))
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.storage_client.as_ref().as_ref()
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

    pub fn acquire_lease(&self, lease_duration: LeaseDuration) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self, lease_duration)
    }

    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self)
    }

    pub fn release_lease(&self, lease_id: LeaseId) -> ReleaseLeaseBuilder {
        ReleaseLeaseBuilder::new(self, lease_id)
    }

    pub fn renew_lease<'a>(&'a self, lease_id: &'a LeaseId) -> RenewLeaseBuilder<'a> {
        RenewLeaseBuilder::new(self, lease_id)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&'a [u8]>,
    ) -> Result<(Request<&'a [u8]>, url::Url), AzureError> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
