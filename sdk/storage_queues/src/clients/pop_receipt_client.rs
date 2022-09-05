use crate::{operations::*, prelude::*};
use azure_core::{Context, Request, Response};

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

    pub(crate) fn finalize_request(
        &self,
        url: url::Url,
        method: azure_core::Method,
        headers: azure_core::headers::Headers,
        request_body: Option<azure_core::Body>,
    ) -> azure_core::Result<Request> {
        self.queue_client
            .finalize_request(url, method, headers, request_body)
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
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
