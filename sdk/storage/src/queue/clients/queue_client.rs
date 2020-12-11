use crate::core::Client;
use crate::queue::clients::QueueAccountClient;
use crate::queue::requests::*;
use crate::queue::PopReceipt;
use crate::HasStorageClient;
use azure_core::Metadata;
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

    pub fn create_queue(&self) -> CreateQueueBuilder<'_, C> {
        CreateQueueBuilder::new(self)
    }

    pub fn delete_queue(&self) -> DeleteQueueBuilder<'_, C> {
        DeleteQueueBuilder::new(self)
    }

    pub fn put_message<'a, MB>(&'a self, message_body: MB) -> PutMessageBuilder<'a, C>
    where
        MB: Into<Cow<'a, str>>,
    {
        PutMessageBuilder::new(self, message_body)
    }

    pub fn get_messages(&self) -> GetMessagesBuilder<'_, C> {
        GetMessagesBuilder::new(self)
    }

    pub fn peek_messages(&self) -> PeekMessagesBuilder<'_, C> {
        PeekMessagesBuilder::new(self)
    }

    pub fn delete_message(&self, pop_receipt: Box<dyn PopReceipt>) -> DeleteMessageBuilder<'_, C> {
        DeleteMessageBuilder::new(self, pop_receipt)
    }

    pub fn clear_messages(&self) -> ClearMessagesBuilder<'_, C> {
        ClearMessagesBuilder::new(self)
    }

    pub fn set_queue_metadata<'a>(
        &'a self,
        metadata: &'a Metadata,
    ) -> SetQueueMetadataBuilder<'a, C> {
        SetQueueMetadataBuilder::new(self, metadata)
    }
}
