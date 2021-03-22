use crate::core::clients::StorageClient;
use crate::queue_storage::requests::*;
use std::fmt::Debug;
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

    /// Sets or clears the queue metadata. The metadata
    /// will be passed to the `execute` function of the returned struct.
    pub fn set_metadata(&self) -> SetQueueMetadataBuilder {
        SetQueueMetadataBuilder::new(self)
    }

    /// Get the queue metadata.
    pub fn get_metadata(&self) -> GetQueueMetadataBuilder {
        GetQueueMetadataBuilder::new(self)
    }

    /// Get the queue ACL. This call returns
    /// all the stored access policies associated
    /// to the current queue.
    pub fn get_acl(&self) -> GetQueueACLBuilder {
        GetQueueACLBuilder::new(self)
    }

    /// Set the queue ACL. You can call this function
    /// to change or remove already existing stored
    /// access policies by modifying the list returned
    /// by `get_acl`.
    pub fn set_acl(&self) -> SetQueueACLBuilder {
        SetQueueACLBuilder::new(self)
    }

    /// Puts a message in the queue. The body will be passed
    /// to the `execute` function of the returned struct.
    pub fn put_message(&self) -> PutMessageBuilder {
        PutMessageBuilder::new(self)
    }

    /// Peeks, without removing, one or more messages.
    pub fn peek_messages(&self) -> PeekMessagesBuilder {
        PeekMessagesBuilder::new(self)
    }

    /// Gets, shadowing them, one or more messages.
    pub fn get_messages(&self) -> GetMessagesBuilder {
        GetMessagesBuilder::new(self)
    }

    /// Removes all messages from the queue.
    pub fn clear_messages(&self) -> ClearMessagesBuilder {
        ClearMessagesBuilder::new(self)
    }
}
