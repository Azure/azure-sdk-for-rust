use crate::core::Client;
use crate::queue::clients::QueueClient;
use crate::requests;
use crate::HasStorageClient;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueAccountClient<C>
where
    C: Client + Clone,
{
    storage_client: C,
}

impl<C> HasStorageClient for QueueAccountClient<C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        &self.storage_client
    }
}

impl<C> QueueAccountClient<C>
where
    C: Client + Clone,
{
    pub fn new(storage_client: C) -> Self {
        Self { storage_client }
    }

    pub fn list_queues(&self) -> requests::ListQueuesBuilder<'_, C> {
        crate::requests::ListQueuesBuilder::new(self)
    }

    pub fn into_queue_client<QN>(self, queue_name: QN) -> QueueClient<C>
    where
        QN: Into<String>,
    {
        QueueClient::new(self, queue_name.into())
    }
}

impl<C> From<C> for QueueAccountClient<C>
where
    C: Client + Clone,
{
    fn from(storage_client: C) -> Self {
        Self::new(storage_client)
    }
}
