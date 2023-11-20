use crate::{operations::*, prelude::*};
use azure_core::{Context, Request, Response};

#[derive(Debug, Clone)]
pub struct PopReceiptClient {
    client: QueueClient,
    pop_receipt: PopReceipt,
}

impl PopReceiptClient {
    pub(crate) fn new(client: QueueClient, pop_receipt: PopReceipt) -> Self {
        Self {
            client,
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
        self.client.send(context, request).await
    }

    pub(crate) fn finalize_request(
        url: url::Url,
        method: azure_core::Method,
        headers: azure_core::headers::Headers,
        request_body: Option<azure_core::Body>,
    ) -> azure_core::Result<Request> {
        QueueClient::finalize_request(url, method, headers, request_body)
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        let mut url = self.client.messages_url()?;

        url.path_segments_mut()
            .expect("invalid base url")
            .push(self.pop_receipt.message_id());

        url.query_pairs_mut()
            .append_pair("popreceipt", self.pop_receipt.pop_receipt());

        Ok(url)
    }
}
