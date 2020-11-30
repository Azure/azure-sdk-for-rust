use crate::core::prelude::*;
use crate::queue::clients::QueueNameClient;
use crate::queue::prelude::*;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_name_client: &'a QueueNameClient<C>,
    number_of_messages: Option<u32>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_name_client: &'a QueueNameClient<C>) -> Self {
        PeekMessagesBuilder {
            queue_name_client,
            number_of_messages: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> NumberOfMessagesOption for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn number_of_messages(&self) -> Option<u32> {
        self.number_of_messages
    }
}

impl<'a, C> TimeoutOption for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> NumberOfMessagesSupport for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_number_of_messages(self, number_of_messages: u32) -> Self::O {
        PeekMessagesBuilder {
            queue_name_client: self.queue_name_client,
            number_of_messages: Some(number_of_messages),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_timeout(self, timeout: u64) -> Self::O {
        PeekMessagesBuilder {
            queue_name_client: self.queue_name_client,
            number_of_messages: self.number_of_messages,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        PeekMessagesBuilder {
            queue_name_client: self.queue_name_client,
            number_of_messages: self.number_of_messages,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_name_client(&self) -> &'a QueueNameClient<C> {
        self.queue_name_client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub async fn execute(self) -> Result<PeekMessagesResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}/messages",
            self.queue_name_client.storage_client().queue_uri(),
            self.queue_name_client.queue_name()
        ))?;

        url.query_pairs_mut().append_pair("peekonly", "true");
        TimeoutOption::append_pair(&self, &mut url);
        NumberOfMessagesOption::append_pair(&self, &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_name_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;

        (&headers, &body as &[u8]).try_into()
    }
}
