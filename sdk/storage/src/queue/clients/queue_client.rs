use crate::core::Client;
use crate::queue::clients::QueueAccountClient;
use crate::queue::PopReceipt;
use crate::requests;
use crate::HasStorageClient;
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueClient<C>
where
    C: Client + Clone,
{
    queue_account_client: QueueAccountClient<C>,
    queue_name: String,
}

impl<C> HasStorageClient for QueueClient<C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        self.queue_account_client.storage_client()
    }
}

impl<C> QueueClient<C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_account_client: QueueAccountClient<C>, queue_name: String) -> Self {
        Self {
            queue_account_client,
            queue_name,
        }
    }

    pub fn queue_name(&self) -> &str {
        self.queue_name.as_ref()
    }

    pub fn queue_account_client(&self) -> &QueueAccountClient<C> {
        &self.queue_account_client
    }

    pub fn create_queue(&self) -> requests::CreateQueueBuilder<'_, C> {
        crate::requests::CreateQueueBuilder::new(self)
    }

    pub fn delete_queue(&self) -> requests::DeleteQueueBuilder<'_, C> {
        crate::requests::DeleteQueueBuilder::new(self)
    }

    pub fn put_message<'a, MB>(&'a self, message_body: MB) -> requests::PutMessageBuilder<'a, C>
    where
        MB: Into<Cow<'a, str>>,
    {
        requests::PutMessageBuilder::new(self, message_body)
    }

    pub fn get_messages(&self) -> requests::GetMessagesBuilder<'_, C> {
        requests::GetMessagesBuilder::new(self)
    }

    pub fn peek_messages(&self) -> requests::PeekMessagesBuilder<'_, C> {
        requests::PeekMessagesBuilder::new(self)
    }

    pub fn delete_message(
        &self,
        pop_receipt: Box<dyn PopReceipt>,
    ) -> requests::DeleteMessageBuilder<'_, C> {
        requests::DeleteMessageBuilder::new(self, pop_receipt)
    }

    pub fn clear_messages(&self) -> requests::ClearMessagesBuilder<'_, C> {
        requests::ClearMessagesBuilder::new(self)
    }
}
