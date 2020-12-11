use crate::core::prelude::*;
use crate::queue::clients::QueueClient;
use crate::queue::prelude::*;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    pop_receipt: Box<dyn PopReceipt>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_client: &'a QueueClient<C>, pop_receipt: Box<dyn PopReceipt>) -> Self {
        DeleteMessageBuilder {
            queue_client,
            pop_receipt,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> PopReceiptRequired for DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    fn pop_receipt(&self) -> &dyn PopReceipt {
        self.pop_receipt.as_ref()
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn with_timeout(self, timeout: Timeout) -> Self {
        Self {
            queue_client: self.queue_client,
            pop_receipt: self.pop_receipt,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }

    pub fn with_client_request_id(self, client_request_id: ClientRequestId<'a>) -> Self {
        Self {
            queue_client: self.queue_client,
            pop_receipt: self.pop_receipt,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }

    pub async fn execute(self) -> Result<DeleteMessageResponse, AzureError> {
        let pop_receipt = self.pop_receipt();

        let mut url = url::Url::parse(&format!(
            "{}/{}/messages/{}",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name(),
            pop_receipt.message_id()
        ))?;

        url.query_pairs_mut()
            .append_pair("popreceipt", pop_receipt.pop_receipt());
        AppendToUrlQuery::append_to_url_query(&self.timeout, &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::NO_CONTENT)
            .await?;

        (&headers).try_into()
    }
}
