use crate::core::prelude::*;
use crate::queue::clients::QueueNameClient;
use crate::queue::prelude::*;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_name_client: &'a QueueNameClient<C>,
    pop_receipt: Box<dyn PopReceipt>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(
        queue_name_client: &'a QueueNameClient<C>,
        pop_receipt: Box<dyn PopReceipt>,
    ) -> Self {
        DeleteMessageBuilder {
            queue_name_client,
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

impl<'a, C> TimeoutOption for DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> TimeoutSupport for DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteMessageBuilder {
            queue_name_client: self.queue_name_client,
            pop_receipt: self.pop_receipt,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteMessageBuilder {
            queue_name_client: self.queue_name_client,
            pop_receipt: self.pop_receipt,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_name_client(&self) -> &'a QueueNameClient<C> {
        self.queue_name_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client + Clone,
{
    pub async fn execute(self) -> Result<DeleteMessageResponse, AzureError> {
        let pop_receipt = self.pop_receipt();

        let mut url = url::Url::parse(&format!(
            "{}/{}/messages/{}",
            self.queue_name_client.storage_client().queue_uri(),
            self.queue_name_client.queue_name(),
            pop_receipt.message_id()
        ))?;

        url.query_pairs_mut()
            .append_pair("popreceipt", pop_receipt.pop_receipt());

        TimeoutOption::append_pair(&self, &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_name_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::DELETE,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
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
