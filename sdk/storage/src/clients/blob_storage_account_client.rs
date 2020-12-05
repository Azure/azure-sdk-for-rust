use crate::clients::{ServiceType, StorageAccountClient};
use azure_core::errors::AzureError;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsBlobStorageAccountClient {
    fn as_blob_storage_account_client(self) -> Arc<Box<BlobStorageAccountClient>>;
}

impl AsBlobStorageAccountClient for Arc<Box<StorageAccountClient>> {
    fn as_blob_storage_account_client(self) -> Arc<Box<BlobStorageAccountClient>> {
        BlobStorageAccountClient::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct BlobStorageAccountClient {
    storage_account_client: Arc<Box<StorageAccountClient>>,
}

impl BlobStorageAccountClient {
    pub(crate) fn new(storage_account_client: Arc<Box<StorageAccountClient>>) -> Arc<Box<Self>> {
        Arc::new(Box::new(Self {
            storage_account_client,
        }))
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_account_client.as_ref().as_ref()
    }

    pub fn list_containers(&self) -> crate::container::requests::ListBuilder2 {
        crate::container::requests::ListBuilder2::new(self)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&'a [u8]>,
    ) -> Result<(Request<&'a [u8]>, url::Url), AzureError> {
        self.storage_account_client.prepare_request(
            url,
            method,
            http_header_adder,
            ServiceType::Blob,
            request_body,
        )
    }
}
