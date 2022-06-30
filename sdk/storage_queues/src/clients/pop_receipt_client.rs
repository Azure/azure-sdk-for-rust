use crate::{operations::*, prelude::*};
use azure_core::{Context, Request, Response};
use azure_storage::core::clients::StorageClient;


pub trait AsPopReceiptClient {
    /// Implement this trait to convert the calling client into a
    /// `PopReceiptClient`. This trait is used to make sure the
    /// returned client is wrapped in an `Arc` to avoid
    /// unnecessary copying while keeping the clients
    /// type signature simple (without lifetimes).
    fn pop_receipt_client(&self, pop_receipt: impl Into<PopReceipt>) -> PopReceiptClient;
}

impl AsPopReceiptClient for QueueClient {
    /// Pass a valid `PopReceipt` to a `QueueClient`
    /// to obtain a `PopReceiptClient` back. The `PopReceiptClient`
    /// can then delete or update the message
    /// referenced by the passed `PopReceipt`.
    fn pop_receipt_client(&self, pop_receipt: impl Into<PopReceipt>) -> PopReceiptClient {
        PopReceiptClient::new(self.clone(), pop_receipt)
    }
}

#[derive(Debug, Clone)]
pub struct PopReceiptClient {
    queue_client: QueueClient,
    pop_receipt: PopReceipt,
}

impl PopReceiptClient {
    pub(crate) fn new(queue_client: QueueClient, pop_receipt: impl Into<PopReceipt>) -> Self {
        Self {
            queue_client,
            pop_receipt: pop_receipt.into(),
        }
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.queue_client.send(context, request).await
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.queue_client.storage_client()
    }

    pub(crate) fn pop_receipt_url(&self) -> azure_core::Result<url::Url> {
        let mut url = self.queue_client.url_with_segments(
            ["messages", self.pop_receipt.message_id()]
                .iter()
                .map(std::ops::Deref::deref),
        )?;

        url.query_pairs_mut()
            .append_pair("popreceipt", self.pop_receipt.pop_receipt());

        Ok(url)
    }

    /// Updates the message.
    ///
    /// The message must not have been made visible again
    /// or this call would fail.
    pub fn update(
        &self,
        body: impl Into<String>,
        visibility_timeout: impl Into<VisibilityTimeout>,
    ) -> UpdateMessageBuilder {
        UpdateMessageBuilder::new(self.clone(), body.into(), visibility_timeout.into())
    }

    /// Deletes the message.
    ///
    /// The message must not have been made visible again
    /// or this call would fail.
    pub fn delete(&self) -> DeleteMessageBuilder {
        DeleteMessageBuilder::new(self.clone())
    }
}
