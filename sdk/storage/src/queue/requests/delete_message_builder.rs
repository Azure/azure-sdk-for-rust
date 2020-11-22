use crate::core::prelude::*;
use crate::queue::prelude::*;
use crate::responses::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    message: Option<&'b Message>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, 'b, C> DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    ) -> DeleteMessageBuilder<'a, 'b, C> {
        DeleteMessageBuilder {
            queue_name_service,
            message: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, 'b, C> MessageRequired for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn message(&self) -> &Message {
        self.message.as_ref().unwrap()
    }
}

impl<'a, 'b, C> TimeoutOption for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, 'b, C> ClientRequestIdOption<'a> for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, 'b, C> MessageSupport<'b> for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, 'b, C>;

    #[inline]
    fn with_message(self, message: &'b Message) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            message: Some(message),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C> TimeoutSupport for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, 'b, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            message: self.message,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, 'b, C> ClientRequestIdSupport<'a> for DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = DeleteMessageBuilder<'a, 'b, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteMessageBuilder {
            queue_name_service: self.queue_name_service,
            message: self.message,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, 'b, C> DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    pub fn queue_name_service(&self) -> &'a dyn QueueNameService<StorageClient = C> {
        self.queue_name_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C> DeleteMessageBuilder<'a, 'b, C>
where
    C: Client,
{
    pub async fn execute(self) -> Result<DeleteMessageResponse, AzureError> {
        let message = self.message();

        let mut uri = format!(
            "{}/{}/messages/{}?popreceipt={}",
            self.queue_name_service.storage_client().queue_uri(),
            self.queue_name_service.queue_name(),
            message.message_id,
            message.pop_receipt
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
