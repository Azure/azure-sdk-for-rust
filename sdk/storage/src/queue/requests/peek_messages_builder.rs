use crate::core::prelude::*;
use crate::queue::prelude::*;
use crate::responses::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    number_of_messages: Option<u32>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    ) -> PeekMessagesBuilder<'a, C> {
        PeekMessagesBuilder {
            queue_name_service,
            number_of_messages: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> NumberOfMessagesOption for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn number_of_messages(&self) -> Option<u32> {
        self.number_of_messages
    }
}

impl<'a, C> TimeoutOption for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> NumberOfMessagesSupport for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = PeekMessagesBuilder<'a, C>;

    #[inline]
    fn with_number_of_messages(self, number_of_messages: u32) -> Self::O {
        PeekMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: Some(number_of_messages),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = PeekMessagesBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        PeekMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: self.number_of_messages,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = PeekMessagesBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PeekMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: self.number_of_messages,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    pub fn queue_name_service(&self) -> &'a dyn QueueNameService<StorageClient = C> {
        self.queue_name_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client,
{
    pub async fn execute(self) -> Result<PeekMessagesResponse, AzureError> {
        let mut uri = format!(
            "{}/{}/messages?peekonly=true",
            self.queue_name_service.storage_client().queue_uri(),
            self.queue_name_service.queue_name()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}{}{}", uri, '&', nm);
        }
        if let Some(nm) = NumberOfMessagesOption::to_uri_parameter(&self) {
            uri = format!("{}{}{}", uri, '&', nm);
        }

        debug!("uri == {}", uri);

        let perform_request_response = self.queue_name_service.storage_client().perform_request(
            &uri,
            &http::Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(perform_request_response.response_future, StatusCode::OK).await?;

        (&headers, &body as &[u8]).try_into()
    }
}
