use crate::core::clients::StorageClient;
use crate::queue::prelude::*;
use crate::queue::requests::*;
use azure_core::HttpClient;
use std::sync::Arc;

pub trait AsPopReceiptClient {
    /// Implement this trait to convert the calling client into a
    /// `PopReceiptClient`. This trait is used to make sure the
    /// returned client is wrapped in an `Arc` to avoid
    /// unnecessary copying while keeping the clients
    /// type signature simple (without lifetimes).
    fn as_pop_receipt_client(&self, pop_receipt: impl Into<PopReceipt>) -> Arc<PopReceiptClient>;
}

impl AsPopReceiptClient for Arc<QueueClient> {
    /// Pass a valid `PopReceipt` to a `QueueClient`
    /// to obtain a `PopReceiptClient` back. The `PopReceiptClient`
    /// can then delete or update the message
    /// referenced by the passed `PopReceipt`.
    fn as_pop_receipt_client(&self, pop_receipt: impl Into<PopReceipt>) -> Arc<PopReceiptClient> {
        PopReceiptClient::new(self.clone(), pop_receipt)
    }
}

#[derive(Debug, Clone)]
pub struct PopReceiptClient {
    queue_client: Arc<QueueClient>,
    pop_receipt: PopReceipt,
}

impl PopReceiptClient {
    pub(crate) fn new(
        queue_client: Arc<QueueClient>,
        pop_receipt: impl Into<PopReceipt>,
    ) -> Arc<Self> {
        Arc::new(Self {
            queue_client,
            pop_receipt: pop_receipt.into(),
        })
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.queue_client.storage_client()
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.queue_client
            .storage_client()
            .storage_account_client()
            .http_client()
    }

    pub(crate) fn pop_receipt_url(&self) -> Result<url::Url, url::ParseError> {
        let mut url = self.queue_client.url_with_segments(
            ["messages", self.pop_receipt.message_id()]
                .iter()
                .map(std::ops::Deref::deref),
        )?;

        url.query_pairs_mut()
            .append_pair("popreceipt", self.pop_receipt.pop_receipt());

        Ok(url)
    }

    /// Deletes the message. The message must not have been
    /// made visible again or this call would fail.
    pub fn delete(&self) -> DeleteMessageBuilder {
        DeleteMessageBuilder::new(self)
    }

    /// Updates the message.
    /// The message must not have been
    /// made visible again or this call would fail.
    pub fn update(&self, visibility_timeout: impl Into<VisibilityTimeout>) -> UpdateMessageBuilder {
        UpdateMessageBuilder::new(self, visibility_timeout)
    }
}
