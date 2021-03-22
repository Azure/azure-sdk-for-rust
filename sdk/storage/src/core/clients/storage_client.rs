use crate::core::clients::{ServiceType, StorageAccountClient};
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsStorageClient {
    fn as_storage_client(&self) -> Arc<StorageClient>;
}

impl AsStorageClient for Arc<StorageAccountClient> {
    fn as_storage_client(&self) -> Arc<StorageClient> {
        StorageClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct StorageClient {
    storage_account_client: Arc<StorageAccountClient>,
}

impl StorageClient {
    pub(crate) fn new(storage_account_client: Arc<StorageAccountClient>) -> Arc<Self> {
        Arc::new(Self {
            storage_account_client,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_account_client.as_ref()
    }

    #[allow(dead_code)]
    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.storage_account_client.http_client()
    }

    #[cfg(feature = "storage_account")]
    pub fn get_account_information(
        &self,
    ) -> crate::storage_account::account::requests::GetAccountInformationBuilder {
        crate::storage_account::account::requests::GetAccountInformationBuilder::new(self)
    }

    #[cfg(feature = "blob_storage")]
    pub fn list_containers(&self) -> crate::container::requests::ListContainersBuilder {
        crate::container::requests::ListContainersBuilder::new(self)
    }

    #[cfg(feature = "queue_storage")]
    pub fn list_queues(&self) -> crate::queue_storage::requests::ListQueuesBuilder {
        crate::queue_storage::requests::ListQueuesBuilder::new(self)
    }

    #[cfg(feature = "queue_storage")]
    pub fn get_queue_service_properties(
        &self,
    ) -> crate::queue_storage::requests::GetQueueServicePropertiesBuilder {
        crate::queue_storage::requests::GetQueueServicePropertiesBuilder::new(self)
    }

    #[cfg(feature = "queue_storage")]
    pub fn set_queue_service_properties(
        &self,
    ) -> crate::queue_storage::requests::SetQueueServicePropertiesBuilder {
        crate::queue_storage::requests::SetQueueServicePropertiesBuilder::new(self)
    }

    #[cfg(feature = "queue_storage")]
    pub fn get_queue_service_stats(
        &self,
    ) -> crate::queue_storage::requests::GetQueueServiceStatsBuilder {
        crate::queue_storage::requests::GetQueueServiceStatsBuilder::new(self)
    }

    #[allow(dead_code)]
    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.storage_account_client.prepare_request(
            url,
            method,
            http_header_adder,
            ServiceType::Blob,
            request_body,
        )
    }
}
