use crate::clients::StorageClient;
use azure_core::errors::AzureError;
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

    pub fn list_blobs(&self) -> crate::container::requests::ListBlobBuilder2 {
        crate::container::requests::ListBlobBuilder2::new(self)
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
