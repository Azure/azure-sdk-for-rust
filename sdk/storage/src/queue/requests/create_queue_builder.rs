use crate::core::prelude::*;
use crate::queue::clients::QueueClient;
use crate::queue::responses::*;
use crate::queue::HasStorageClient;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    timeout: Option<u64>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_client: &'a QueueClient<C>) -> Self {
        CreateQueueBuilder {
            queue_client,
            timeout: None,
            metadata: None,
            client_request_id: None,
        }
    }
}

impl<'a, C> TimeoutOption for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> MetadataOption<'a> for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    fn metadata(&self) -> Option<&'a HashMap<&'a str, &'a str>> {
        self.metadata
    }
}

impl<'a, C> TimeoutSupport for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_timeout(self, timeout: u64) -> Self::O {
        CreateQueueBuilder {
            queue_client: self.queue_client,
            timeout: Some(timeout),
            metadata: self.metadata,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CreateQueueBuilder {
            queue_client: self.queue_client,
            timeout: self.timeout,
            metadata: self.metadata,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C> MetadataSupport<'a> for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_metadata(self, metadata: &'a HashMap<&'a str, &'a str>) -> Self::O {
        CreateQueueBuilder {
            queue_client: self.queue_client,
            timeout: self.timeout,
            metadata: Some(metadata),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_client(&self) -> &'a QueueClient<C> {
        self.queue_client
    }

    pub async fn execute(self) -> Result<CreateQueueResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name(),
        ))?;

        TimeoutOption::append_to_url(&self, &mut url);

        debug!("uri == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = MetadataOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::CREATED)
            .await?;

        (&headers).try_into()
    }
}
