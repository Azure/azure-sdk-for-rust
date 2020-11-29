use crate::core::prelude::*;
use crate::queue::clients::QueueNameClient;
use crate::queue::HasStorageClient;
use crate::responses::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;
use url::Url;

#[derive(Debug)]
pub struct ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_name_client: &'a QueueNameClient<C>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    pub(crate) fn new(queue_name_client: &'a QueueNameClient<C>) -> ClearMessagesBuilder<'a, C> {
        ClearMessagesBuilder {
            queue_name_client,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C> TimeoutOption for ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> TimeoutSupport for ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = ClearMessagesBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ClearMessagesBuilder {
            queue_name_client: self.queue_name_client,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = ClearMessagesBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ClearMessagesBuilder {
            queue_name_client: self.queue_name_client,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C> ClearMessagesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_name_client(&self) -> &'a QueueNameClient<C> {
        self.queue_name_client
    }

    pub async fn execute(self) -> Result<ClearMessagesResponse, AzureError> {
        let mut url = Url::parse(&format!(
            "{}/{}/messages",
            self.queue_name_client.storage_client().queue_uri(),
            self.queue_name_client.queue_name(),
        ))?;

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
