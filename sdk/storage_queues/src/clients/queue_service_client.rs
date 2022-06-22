use azure_storage::core::clients::{AsStorageClient, StorageAccountClient, StorageClient};
use std::fmt::Debug;
use std::sync::Arc;

pub trait AsQueueServiceClient {
    fn as_queue_service_client(&self) -> Arc<QueueServiceClient>;
}

impl AsQueueServiceClient for Arc<StorageClient> {
    fn as_queue_service_client(&self) -> Arc<QueueServiceClient> {
        QueueServiceClient::new(self.clone())
    }
}

impl AsQueueServiceClient for Arc<StorageAccountClient> {
    fn as_queue_service_client(&self) -> Arc<QueueServiceClient> {
        self.as_storage_client().as_queue_service_client()
    }
}

#[derive(Debug, Clone)]
pub struct QueueServiceClient {
    storage_client: Arc<StorageClient>,
}

impl QueueServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Arc<Self> {
        Arc::new(Self { storage_client })
    }

    #[must_use]
    pub fn list_queues(&self) -> crate::requests::ListQueuesBuilder {
        crate::requests::ListQueuesBuilder::new(&self.storage_client)
    }

    #[must_use]
    pub fn get_queue_service_properties(
        &self,
    ) -> crate::requests::GetQueueServicePropertiesBuilder {
        crate::requests::GetQueueServicePropertiesBuilder::new(&self.storage_client)
    }

    #[must_use]
    pub fn set_queue_service_properties(
        &self,
    ) -> crate::requests::SetQueueServicePropertiesBuilder {
        crate::requests::SetQueueServicePropertiesBuilder::new(&self.storage_client)
    }

    #[must_use]
    pub fn get_queue_service_stats(&self) -> crate::requests::GetQueueServiceStatsBuilder {
        crate::requests::GetQueueServiceStatsBuilder::new(&self.storage_client)
    }
}
