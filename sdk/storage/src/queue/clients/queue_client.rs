use crate::clients::StorageClient;
use crate::queue::prelude::*;
use crate::queue::requests::*;
use azure_core::Metadata;
use std::borrow::Cow;
use std::sync::Arc;

pub trait AsQueueClient<QN: Into<String>> {
    fn as_queue_client(&self, queue_name: QN) -> Arc<QueueClient>;
}

impl<QN: Into<String>> AsQueueClient<QN> for Arc<StorageClient> {
    fn as_queue_client(&self, queue_name: QN) -> Arc<QueueClient> {
        QueueClient::new(self.clone(), queue_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct QueueClient {
    storage_client: Arc<StorageClient>,
    queue_name: String,
}

impl QueueClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>, queue_name: String) -> Arc<Self> {
        Arc::new(Self {
            storage_client,
            queue_name,
        })
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.storage_client.as_ref()
    }

    pub(crate) fn queue_url(&self) -> Result<url::Url, url::ParseError> {
        self.storage_client()
            .storage_account_client()
            .queue_storage_url()
            .join(&format!("{}/", &self.queue_name))
    }

    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    /// Creates the queue.
    pub fn create(&self) -> CreateQueueBuilder {
        CreateQueueBuilder::new(self)
    }

    /// Deletes the queue.
    pub fn delete(&self) -> DeleteQueueBuilder {
        DeleteQueueBuilder::new(self)
    }

    /// Sets or clears the queue metadata.
    pub fn set_metadata<'a>(&'a self, metadata: &'a Metadata) -> SetQueueMetadataBuilder {
        SetQueueMetadataBuilder::new(self, metadata)
    }

    /// Puts a message in the queue.
    pub fn put_message<'a>(&'a self, body: impl Into<Cow<'a, str>>) -> PutMessageBuilder {
        PutMessageBuilder::new(self, body)
    }

    /// Peeks, without removing, one or more messages.
    pub fn peek_messages(&self) -> PeekMessagesBuilder {
        PeekMessagesBuilder::new(self)
    }

    /// Gets, shadowing them, one or more messages. In order to delete them, call [delete_message]
    /// with the pop receipt before the shadow timeout expires.
    pub fn get_messages(&self) -> GetMessagesBuilder {
        GetMessagesBuilder::new(self)
    }

    /// Deletes one or more previously shadowed messages.
    pub fn delete_message<'a>(&'a self, pop_receipt: &'a dyn PopReceipt) -> DeleteMessageBuilder {
        DeleteMessageBuilder::new(self, pop_receipt)
    }

    /// Removes all messages from the queue.
    pub fn clear_messages(&self) -> ClearMessagesBuilder {
        ClearMessagesBuilder::new(self)
    }
}
