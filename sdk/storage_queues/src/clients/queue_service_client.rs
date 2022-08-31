use crate::{operations::*, QueueServiceProperties};
use azure_core::{Context, Request, Response};
use azure_storage::clients::{ServiceType, StorageClient};
use std::fmt::Debug;

pub trait AsQueueServiceClient {
    fn queue_service_client(&self) -> QueueServiceClient;
}

impl AsQueueServiceClient for StorageClient {
    fn queue_service_client(&self) -> QueueServiceClient {
        QueueServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct QueueServiceClient {
    pub(crate) storage_client: StorageClient,
}

impl QueueServiceClient {
    pub(crate) fn new(storage_client: StorageClient) -> Self {
        Self { storage_client }
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
