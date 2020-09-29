use crate::requests;
use crate::{HasStorageClient, IntoQueueServiceClient, QueueService, WithQueueServiceClient};
use azure_sdk_storage_core::Client;
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    pub storage_client: Cow<'a, C>,
}

impl<'a, C> HasStorageClient for QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        self.storage_client.as_ref()
    }
}

impl<'a, C> WithQueueServiceClient<'a> for C
where
    C: Client + 'a + Clone,
{
    type QueueServiceClient = QueueServiceClient<'a, C>;

    fn with_queue_service_client(&'a self) -> Self::QueueServiceClient {
        QueueServiceClient {
            storage_client: Cow::Borrowed(self),
        }
    }
}

impl<C> IntoQueueServiceClient for C
where
    C: Client + 'static + Clone,
{
    type QueueServiceClient = QueueServiceClient<'static, C>;

    fn into_queue_service_client(self) -> Self::QueueServiceClient {
        QueueServiceClient {
            storage_client: Cow::Owned(self),
        }
    }
}

impl<'a, C> QueueService for QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, Self::StorageClient> {
        crate::requests::ListQueuesBuilder::new(self)
    }
}
