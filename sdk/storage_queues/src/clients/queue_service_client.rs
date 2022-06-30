use crate::{operations::*, QueueServiceProperties};
use azure_core::{Context, Request, Response};
use azure_storage::core::clients::{ServiceType, StorageClient};
use std::{fmt::Debug, sync::Arc};

pub trait AsQueueServiceClient {
    fn queue_service_client(&self) -> Arc<QueueServiceClient>;
}

impl AsQueueServiceClient for Arc<StorageClient> {
    fn queue_service_client(&self) -> Arc<QueueServiceClient> {
        QueueServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct QueueServiceClient {
    pub(crate) storage_client: Arc<StorageClient>,
}

impl QueueServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Arc<Self> {
        Arc::new(Self { storage_client })
    }

    pub fn list_queues(&self) -> ListQueuesBuilder {
        ListQueuesBuilder::new(self.clone())
    }

    pub fn get_queue_service_properties(&self) -> GetQueueServicePropertiesBuilder {
        GetQueueServicePropertiesBuilder::new(self.clone())
    }

    /// Set queue service properties.
    ///
    /// More info here:
    /// <https://docs.microsoft.com/rest/api/storageservices/set-queue-service-properties>
    pub fn set_queue_service_properties(
        &self,
        properties: QueueServiceProperties,
    ) -> SetQueueServicePropertiesBuilder {
        SetQueueServicePropertiesBuilder::new(self.clone(), properties)
    }

    pub fn get_queue_service_stats(&self) -> GetQueueServiceStatsBuilder {
        GetQueueServiceStatsBuilder::new(self.clone())
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
}
