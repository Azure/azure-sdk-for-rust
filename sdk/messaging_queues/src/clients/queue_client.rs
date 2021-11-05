use crate::requests::*;
use azure_storage::core::clients::StorageClient;
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

    pub(crate) fn url_with_segments<'a, I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.storage_client
            .queue_url_with_segments(Some(self.queue_name.as_str()).into_iter().chain(segments))
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

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::core::prelude::*;
    use crate::queue::clients::AsQueueClient;

    fn get_emulator_client(queue_name: &str) -> Arc<QueueClient> {
        let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
        storage_account.as_queue_client(queue_name)
    }

    #[tokio::test]
    async fn test_create_delete() {
        let queue_name = uuid::Uuid::new_v4().to_string();
        let queue_client = get_emulator_client(&queue_name);

        queue_client
            .create()
            .execute()
            .await
            .expect("create container should succeed");
        queue_client
            .delete()
            .execute()
            .await
            .expect("delete container should succeed");
    }

    #[tokio::test]
    async fn test_put_peek_get_message() {
        let queue_name = uuid::Uuid::new_v4().to_string();
        let queue_client = get_emulator_client(&queue_name);

        queue_client
            .create()
            .execute()
            .await
            .expect("create container should succeed");

        queue_client
            .put_message()
            .execute("Hello")
            .await
            .expect("put message should succeed");

        let mut messages = queue_client
            .peek_messages()
            .execute()
            .await
            .expect("peek messages should succeed");
        assert_eq!(
            messages.messages.pop().expect("message").message_text,
            "Hello"
        );

        let mut messages = queue_client
            .get_messages()
            .execute()
            .await
            .expect("get messages should succeed");
        assert_eq!(
            messages.messages.pop().expect("message").message_text,
            "Hello"
        );

        queue_client
            .delete()
            .execute()
            .await
            .expect("delete container should succeed");
    }
}
