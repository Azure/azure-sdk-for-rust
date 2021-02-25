use crate::clients::{ServiceType, StorageAccountClient};
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

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_account_client.as_ref()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.storage_account_client.http_client()
    }

    pub fn get_account_information(
        &self,
    ) -> crate::account::account::requests::GetAccountInformationBuilder {
        crate::account::account::requests::GetAccountInformationBuilder::new(self)
    }

    pub fn list_containers(&self) -> crate::container::requests::ListContainersBuilder {
        crate::container::requests::ListContainersBuilder::new(self)
    }

    pub fn list_queues(&self) -> crate::queue::requests::ListQueuesBuilder {
        crate::queue::requests::ListQueuesBuilder::new(self)
    }

    pub fn get_queue_service_properties(
        &self,
    ) -> crate::queue::requests::GetQueueServicePropertiesBuilder {
        crate::queue::requests::GetQueueServicePropertiesBuilder::new(self)
    }

    pub fn set_queue_service_properties(
        &self,
    ) -> crate::queue::requests::SetQueueServicePropertiesBuilder {
        crate::queue::requests::SetQueueServicePropertiesBuilder::new(self)
    }

    pub fn get_queue_service_stats(&self) -> crate::queue::requests::GetQueueServiceStatsBuilder {
        crate::queue::requests::GetQueueServiceStatsBuilder::new(self)
    }

    pub(crate) fn prepare_request<'a>(
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
