use crate::core::prelude::*;
use crate::queue::prelude::*;
use crate::responses::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    pop_receipt: Option<Box<dyn PopReceipt>>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    ) -> DeleteMessageBuilder<'a, C> {
        DeleteMessageBuilder {
            queue_name_service,
            pop_receipt: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> PopReceiptRequired for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn pop_receipt(&self) -> &dyn PopReceipt {
        self.pop_receipt.as_deref().unwrap()
    }
}

impl<'a, C> TimeoutOption for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> PopReceiptSupport for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, C>;

    #[inline]
    fn with_pop_receipt(self, pop_receipt: Box<dyn PopReceipt>) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            pop_receipt: Some(pop_receipt),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            pop_receipt: self.pop_receipt,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            pop_receipt: self.pop_receipt,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    pub fn queue_name_service(&self) -> &'a dyn QueueNameService<StorageClient = C> {
        self.queue_name_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteMessageBuilder<'a, C>
where
    C: Client,
{
    pub async fn execute(self) -> Result<DeleteMessageResponse, AzureError> {
        let pop_receipt = self.pop_receipt();

        let mut uri = format!(
            "{}/{}/messages/{}?popreceipt={}",
            self.queue_name_service.storage_client().queue_uri(),
            self.queue_name_service.queue_name(),
            pop_receipt.message_id(),
            utf8_percent_encode(pop_receipt.pop_receipt(), NON_ALPHANUMERIC)
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}{}{}", uri, '&', nm);
        }

        debug!("uri == {}", uri);

        let future_response = self.queue_name_service.storage_client().perform_request(
            &uri,
            &http::Method::DELETE,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _) =
            check_status_extract_headers_and_body(future_response, StatusCode::NO_CONTENT).await?;

        (&headers).try_into()
    }
}
