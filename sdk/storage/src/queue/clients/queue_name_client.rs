use crate::core::Client;
use crate::queue::clients::QueueServiceClient;
use crate::requests;
use crate::HasStorageClient;
use azure_core::No;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueNameClient<C>
where
    C: Client + Clone,
{
    pub queue_service_client: QueueServiceClient<C>,
    pub queue_name: String,
}

impl<C> HasStorageClient for QueueNameClient<C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        self.queue_service_client.storage_client()
    }
}

impl<C> QueueNameClient<C>
where
    C: Client + Clone,
{
    pub fn queue_name(&self) -> &str {
        self.queue_name.as_ref()
    }

    pub fn put_message(&self) -> requests::PutMessageBuilder<'_, '_, C, No> {
        requests::PutMessageBuilder::new(self)
    }

    pub fn get_messages(&self) -> requests::GetMessagesBuilder<'_, C> {
        requests::GetMessagesBuilder::new(self)
    }

    pub fn peek_messages(&self) -> requests::PeekMessagesBuilder<'_, C> {
        requests::PeekMessagesBuilder::new(self)
    }

    pub fn delete_message(&self) -> requests::DeleteMessageBuilder<'_, C, No> {
        requests::DeleteMessageBuilder::new(self)
    }

    pub fn clear_messages(&self) -> requests::ClearMessagesBuilder<'_, C> {
        requests::ClearMessagesBuilder::new(self)
    }
}
