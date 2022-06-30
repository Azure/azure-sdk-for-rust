use crate::{operations::*, QueueStoredAccessPolicy};
use azure_core::{prelude::*, Context, Request, Response};
use azure_storage::core::clients::{ServiceType, StorageClient};
use std::{fmt::Debug};

pub trait AsQueueClient<QN: Into<String>> {
    fn queue_client(&self, queue_name: QN) -> QueueClient;
}

impl<QN: Into<String>> AsQueueClient<QN> for StorageClient {
    fn queue_client(&self, queue_name: QN) -> QueueClient {
        QueueClient::new(self.clone(), queue_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct QueueClient {
    storage_client: StorageClient,
    queue_name: String,
}

impl QueueClient {
    pub(crate) fn new(storage_client: StorageClient, queue_name: String) -> Self {
        Self {
            storage_client,
            queue_name,
        }
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.storage_client
            .send(context, request, ServiceType::Queue)
            .await
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        &self.storage_client
    }

    pub(crate) fn url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
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
        CreateQueueBuilder::new(self.clone())
    }

    /// Deletes the queue.
    pub fn delete(&self) -> DeleteQueueBuilder {
        DeleteQueueBuilder::new(self.clone())
    }

    /// Sets or clears the queue metadata.
    ///
    /// Keep in mind that keys present on Azure but not included in the passed
    /// metadata parameter will be deleted. If you want to keep the preexisting
    /// key-value pairs, retrieve them with GetMetadata first and then
    /// update/add to the received Metadata struct. Then pass the Metadata back
    /// to SetQueueMetadata.  If you just want to clear the metadata, just pass
    /// an empty Metadata struct.
    pub fn set_metadata(&self, metadata: Metadata) -> SetQueueMetadataBuilder {
        SetQueueMetadataBuilder::new(self.clone(), metadata)
    }

    /// Get the queue metadata.
    pub fn get_metadata(&self) -> GetQueueMetadataBuilder {
        GetQueueMetadataBuilder::new(self.clone())
    }

    /// Get the queue ACL.
    ///
    /// This call returns all the stored access policies associated to the
    /// current queue.
    pub fn get_acl(&self) -> GetQueueACLBuilder {
        GetQueueACLBuilder::new(self.clone())
    }

    /// Set the queue ACL.
    ///
    /// You can call this function to change or remove already existing stored
    /// access policies by modifying the list returned by `get_acl`.
    ///
    /// While this SDK does not enforce any limit, keep in mind Azure supports a
    /// limited number of stored access policies for each queue.  More info here
    /// [https://docs.microsoft.com/rest/api/storageservices/set-queue-acl#remarks](https://docs.microsoft.com/rest/api/storageservices/set-queue-acl#remarks).
    pub fn set_acl(&self, policies: Vec<QueueStoredAccessPolicy>) -> SetQueueACLBuilder {
        SetQueueACLBuilder::new(self.clone(), policies)
    }

    /// Puts a message in the queue.
    ///
    /// The body will be passed to the `execute` function of the returned
    /// struct.
    pub fn put_message<S: Into<String>>(&self, message: S) -> PutMessageBuilder {
        PutMessageBuilder::new(self.clone(), message.into())
    }

    /// Peeks, without removing, one or more messages.
    pub fn peek_messages(&self) -> PeekMessagesBuilder {
        PeekMessagesBuilder::new(self.clone())
    }

    /// Gets, shadowing them, one or more messages.
    pub fn get_messages(&self) -> GetMessagesBuilder {
        GetMessagesBuilder::new(self.clone())
    }

    /// Removes all messages from the queue.
    pub fn clear_messages(&self) -> ClearMessagesBuilder {
        ClearMessagesBuilder::new(self.clone())
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{core::prelude::*, queue::clients::AsQueueClient};

    fn get_emulator_client(queue_name: &str) -> QueueClient {
        let storage_account = StorageClient::new_emulator_default();
        storage_account.queue_client(queue_name)
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
