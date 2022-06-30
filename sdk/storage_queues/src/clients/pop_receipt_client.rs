use crate::{operations::*, prelude::*};
use azure_core::{Context, Request, Response};
use azure_storage::core::clients::StorageClient;

#[derive(Debug, Clone)]
pub struct PopReceiptClient {
    queue_client: QueueClient,
    pop_receipt: PopReceipt,
}

impl PopReceiptClient {
    pub(crate) fn new(queue_client: QueueClient, pop_receipt: PopReceipt) -> Self {
        Self {
            queue_client,
            pop_receipt,
        }
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
}
