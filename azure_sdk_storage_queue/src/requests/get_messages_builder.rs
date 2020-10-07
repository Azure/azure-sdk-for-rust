use crate::prelude::*;
use crate::responses::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_storage_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetMessagesBuilder<'a, C>
where
    C: Client,
{
    queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    number_of_messages: Option<u32>,
    visibility_timeout_seconds: u64,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> GetMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_name_service: &'a dyn QueueNameService<StorageClient = C>,
    ) -> GetMessagesBuilder<'a, C> {
        GetMessagesBuilder {
            queue_name_service,
            number_of_messages: None,
            visibility_timeout_seconds: 30,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> NumberOfMessagesOption for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn number_of_messages(&self) -> Option<u32> {
        self.number_of_messages
    }
}

impl<'a, C> VisibilityTimeoutRequired for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn visibility_timeout_seconds(&self) -> u64 {
        self.visibility_timeout_seconds
    }
}

impl<'a, C> TimeoutOption for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> NumberOfMessagesSupport for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = GetMessagesBuilder<'a, C>;

    #[inline]
    fn with_number_of_messages(self, number_of_messages: u32) -> Self::O {
        GetMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: Some(number_of_messages),
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> VisibilityTimeoutSupport for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = GetMessagesBuilder<'a, C>;

    #[inline]
    fn with_visibility_timeout_seconds(self, visibility_timeout_seconds: u64) -> Self::O {
        GetMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: self.number_of_messages,
            visibility_timeout_seconds,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = GetMessagesBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: self.number_of_messages,
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for GetMessagesBuilder<'a, C>
where
    C: Client,
{
    type O = GetMessagesBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetMessagesBuilder {
            queue_name_service: self.queue_name_service,
            number_of_messages: self.number_of_messages,
            visibility_timeout_seconds: self.visibility_timeout_seconds,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> GetMessagesBuilder<'a, C>
where
    C: Client,
{
    pub fn queue_name_service(&self) -> &'a dyn QueueNameService<StorageClient = C> {
        self.queue_name_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> GetMessagesBuilder<'a, C>
where
    C: Client,
{
    pub async fn execute(self) -> Result<GetMessagesResponse, AzureError> {
        let mut uri = format!(
            "{}/{}/messages",
            self.queue_name_service.storage_client().queue_uri(),
            self.queue_name_service.queue_name()
        );

        uri = format!(
            "{}?{}",
            uri,
            VisibilityTimeoutRequired::to_uri_parameter(&self)
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = NumberOfMessagesOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        debug!("uri == {}", uri);

        let future_response = self.queue_name_service.storage_client().perform_request(
            &uri,
            &http::Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        (&headers, &body as &[u8]).try_into()
    }
}
