use crate::core::Client;
use crate::queue::clients::QueueNameClient;
use crate::requests;
use crate::HasStorageClient;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueServiceClient<C>
where
    C: Client + Clone,
{
    pub storage_client: C,
}

impl<C> HasStorageClient for QueueServiceClient<C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        &self.storage_client
    }
}

impl<C> QueueServiceClient<C>
where
    C: Client + Clone,
{
    pub fn new(storage_client: C) -> Self {
        Self { storage_client }
    }

    pub fn list_queues(&self) -> requests::ListQueuesBuilder<'_, C> {
        crate::requests::ListQueuesBuilder::new(self)
    }

    pub fn create_queue<'a>(&'a self, queue_name: &'a str) -> requests::CreateQueueBuilder<'a, C> {
        crate::requests::CreateQueueBuilder::new(self, queue_name)
    }

    pub fn into_queue_name_client<QN>(self, queue_name: QN) -> QueueNameClient<C>
    where
        QN: Into<String>,
    {
        QueueNameClient {
            queue_service_client: self,
            queue_name: queue_name.into(),
        }
    }
}

impl<C> From<C> for QueueServiceClient<C>
where
    C: Client + Clone,
{
    fn from(storage_client: C) -> Self {
        Self::new(storage_client)
    }
}
