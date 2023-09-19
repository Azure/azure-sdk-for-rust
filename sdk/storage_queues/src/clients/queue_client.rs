use crate::{
    operations::*, PopReceipt, PopReceiptClient, QueueServiceClient, QueueStoredAccessPolicy,
};
use azure_core::{prelude::*, Context, Request, Response};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueClient {
    service_client: QueueServiceClient,
    queue_name: String,
}

impl QueueClient {
    pub(crate) fn new(service_client: QueueServiceClient, queue_name: String) -> Self {
        Self {
            service_client,
            queue_name,
        }
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
    /// key-value pairs, retrieve them with `GetMetadata` first and then
    /// update/add to the received Metadata struct. Then pass the Metadata back
    /// to `SetQueueMetadata`.  If you just want to clear the metadata, just pass
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

    /// Turn into a `PopReceiptClient`.
    pub fn pop_receipt_client(&self, pop_receipt: impl Into<PopReceipt>) -> PopReceiptClient {
        PopReceiptClient::new(self.clone(), pop_receipt.into())
    }

    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn url(&self) -> azure_core::Result<url::Url> {
        let mut url = self.service_client.url()?;
        url.path_segments_mut()
            .expect("invalid base url")
            .push(self.queue_name());
        Ok(url)
    }

    pub(crate) fn messages_url(&self) -> azure_core::Result<url::Url> {
        let mut url = self.url()?;
        url.path_segments_mut()
            .expect("invalid base url")
            .push("messages");
        Ok(url)
    }

    pub(crate) fn finalize_request(
        url: url::Url,
        method: azure_core::Method,
        headers: azure_core::headers::Headers,
        request_body: Option<azure_core::Body>,
    ) -> azure_core::Result<Request> {
        QueueServiceClient::finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.service_client.send(context, request).await
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::clients::QueueServiceClientBuilder;

    fn get_emulator_client(queue_name: &str) -> QueueClient {
        let service_client = QueueServiceClientBuilder::emulator()
            .retry(azure_core::RetryOptions::none())
            .build();
        service_client.queue_client(queue_name)
    }

    #[tokio::test]
    async fn test_create_delete() {
        let queue_name = uuid::Uuid::new_v4().to_string();
        let queue_client = get_emulator_client(&queue_name);

        queue_client
            .create()
            .await
            .expect("create container should succeed");
        queue_client
            .delete()
            .await
            .expect("delete container should succeed");
    }

    #[tokio::test]
    async fn test_put_peek_get_message() {
        let queue_name = uuid::Uuid::new_v4().to_string();
        let queue_client = get_emulator_client(&queue_name);

        queue_client
            .create()
            .await
            .expect("create container should succeed");

        queue_client
            .put_message("Hello")
            .await
            .expect("put message should succeed");

        let mut messages = queue_client
            .peek_messages()
            .await
            .expect("peek messages should succeed");
        assert_eq!(
            messages.messages.pop().expect("message").message_text,
            "Hello"
        );

        let mut messages = queue_client
            .get_messages()
            .await
            .expect("get messages should succeed");
        assert_eq!(
            messages.messages.pop().expect("message").message_text,
            "Hello"
        );

        queue_client
            .delete()
            .await
            .expect("delete container should succeed");
    }
}
