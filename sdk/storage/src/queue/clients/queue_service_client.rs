use crate::core::Client;
use crate::requests;
use crate::HasStorageClient;
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

impl<'a, C> QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    pub fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, C> {
        crate::requests::ListQueuesBuilder::new(self)
    }

    pub fn create_queue(&'a self, queue_name: &'a str) -> requests::CreateQueueBuilder<'a, C> {
        crate::requests::CreateQueueBuilder::new(self, queue_name)
    }
}

impl<'a, C> From<&'a C> for QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    fn from(storage_client: &'a C) -> Self {
        Self {
            storage_client: Cow::Borrowed(storage_client),
        }
    }
}

impl<C> From<C> for QueueServiceClient<'static, C>
where
    C: Client + Clone,
{
    fn from(storage_client: C) -> Self {
        Self {
            storage_client: Cow::Owned(storage_client),
        }
    }
}
