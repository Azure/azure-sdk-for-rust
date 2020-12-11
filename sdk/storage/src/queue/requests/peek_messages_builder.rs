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

#[derive(Debug, Clone)]
pub struct PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    number_of_messages: Option<u32>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_client: &'a QueueClient<C>) -> Self {
        PeekMessagesBuilder {
            queue_client,
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

impl<'a, C> NumberOfMessagesSupport for PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_number_of_messages(self, number_of_messages: u32) -> Self::O {
        Self {
            queue_client: self.queue_client,
            number_of_messages: Some(number_of_messages),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> PeekMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn with_timeout(self, timeout: Timeout) -> Self {
        Self {
            queue_client: self.queue_client,
            number_of_messages: self.number_of_messages,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }

    pub fn with_client_request_id(self, client_request_id: ClientRequestId<'a>) -> Self {
        Self {
            queue_client: self.queue_client,
            number_of_messages: self.number_of_messages,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }

    pub async fn execute(self) -> Result<PeekMessagesResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}/messages",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name()
        ))?;

        url.query_pairs_mut().append_pair("peekonly", "true");
        AppendToUrlQuery::append_to_url_query(&self.timeout, &mut url);
        NumberOfMessagesOption::append_to_url(&self, &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
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
