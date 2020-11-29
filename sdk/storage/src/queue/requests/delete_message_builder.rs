use crate::core::prelude::*;
use crate::queue::clients::QueueNameClient;
use crate::queue::prelude::*;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::prelude::*;
use azure_core::{errors::AzureError, No, ToAssign, Yes};
use hyper::StatusCode;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{convert::TryInto, marker::PhantomData};

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    queue_name_client: &'a QueueNameClient<C>,
    p_pop_receipt: PhantomData<PutReceiptSet>,
    pop_receipt: Option<Box<dyn PopReceipt>>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> DeleteMessageBuilder<'a, C, No>
where
    C: Client + Clone,
{
    #[inline]
    pub(crate) fn new(
        queue_name_client: &'a QueueNameClient<C>,
    ) -> DeleteMessageBuilder<'a, C, No> {
        DeleteMessageBuilder {
            queue_name_client,
            p_pop_receipt: PhantomData {},
            pop_receipt: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> PopReceiptRequired for DeleteMessageBuilder<'a, C, Yes>
where
    C: Client + Clone,
{
    #[inline]
    fn pop_receipt(&self) -> &dyn PopReceipt {
        self.pop_receipt.as_deref().unwrap()
    }
}

impl<'a, C, PutReceiptSet> TimeoutOption for DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, PutReceiptSet> ClientRequestIdOption<'a> for DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> PopReceiptSupport for DeleteMessageBuilder<'a, C, No>
where
    C: Client + Clone,
{
    type O = DeleteMessageBuilder<'a, C, Yes>;

    #[inline]
    fn with_pop_receipt(self, pop_receipt: Box<dyn PopReceipt>) -> Self::O {
        DeleteMessageBuilder {
            queue_name_client: self.queue_name_client,
            p_pop_receipt: PhantomData {},
            pop_receipt: Some(pop_receipt),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, PutReceiptSet> TimeoutSupport for DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    type O = DeleteMessageBuilder<'a, C, PutReceiptSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteMessageBuilder {
            queue_name_client: self.queue_name_client,
            p_pop_receipt: PhantomData {},
            pop_receipt: self.pop_receipt,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, PutReceiptSet> ClientRequestIdSupport<'a> for DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    type O = DeleteMessageBuilder<'a, C, PutReceiptSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteMessageBuilder {
            queue_name_client: self.queue_name_client,
            p_pop_receipt: PhantomData {},
            pop_receipt: self.pop_receipt,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C, PutReceiptSet> DeleteMessageBuilder<'a, C, PutReceiptSet>
where
    PutReceiptSet: ToAssign,
    C: Client + Clone,
{
    pub fn queue_name_client(&self) -> &'a QueueNameClient<C> {
        self.queue_name_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteMessageBuilder<'a, C, Yes>
where
    C: Client + Clone,
{
    pub async fn execute(self) -> Result<DeleteMessageResponse, AzureError> {
        let pop_receipt = self.pop_receipt();

        let mut uri = format!(
            "{}/{}/messages/{}?popreceipt={}",
            self.queue_name_client.storage_client().queue_uri(),
            self.queue_name_client.queue_name(),
            pop_receipt.message_id(),
            utf8_percent_encode(pop_receipt.pop_receipt(), NON_ALPHANUMERIC)
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}{}{}", uri, '&', nm);
        }

        debug!("uri == {}", uri);

        let perform_request_response = self.queue_name_client.storage_client().perform_request(
            &uri,
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
