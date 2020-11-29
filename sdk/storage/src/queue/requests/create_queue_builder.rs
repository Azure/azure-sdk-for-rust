use crate::core::prelude::*;
use crate::queue::clients::QueueServiceClient;
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
    queue_service: &'a QueueServiceClient<'a, C>,
    queue_name: &'a str,
    timeout: Option<u64>,
    metadata: Option<&'a HashMap<&'a str, &'a str>>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
    pub(crate) fn new(
        queue_service: &'a QueueServiceClient<'a, C>,
        queue_name: &'a str,
    ) -> CreateQueueBuilder<'a, C> {
        CreateQueueBuilder {
            queue_service,
            queue_name,
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
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    #[inline]
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

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        CreateQueueBuilder {
            queue_service: self.queue_service,
            queue_name: self.queue_name,
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

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        CreateQueueBuilder {
            queue_service: self.queue_service,
            queue_name: self.queue_name,
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
            queue_service: self.queue_service,
            queue_name: self.queue_name,
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
    pub fn queue_service(&self) -> &'a QueueServiceClient<'a, C> {
        self.queue_service
    }

    pub fn queue_name(&self) -> &'a str {
        self.queue_name
    }

    pub async fn execute(self) -> Result<CreateQueueResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.queue_service.storage_client().queue_uri(),
            self.queue_name(),
        ))?;

        TimeoutOption::append_pair(&self, &mut url);

        debug!("uri == {}", url);

        let perform_request_response = self.queue_service.storage_client().perform_request(
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
