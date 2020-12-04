use crate::core::prelude::*;
use crate::queue::clients::QueueClient;
use crate::queue::responses::*;
use crate::queue::HasStorageClient;
use azure_core::errors::AzureError;
use azure_core::headers::add_header;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueMetadataBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    timeout: Option<Timeout>,
    metadata: &'a Metadata<'a>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a, C> SetQueueMetadataBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_client: &'a QueueClient<C>, metadata: &'a Metadata) -> Self {
        SetQueueMetadataBuilder {
            queue_client,
            timeout: None,
            metadata,
            client_request_id: None,
        }
    }
}

impl<'a, C> SetQueueMetadataBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_client(&self) -> &'a QueueClient<C> {
        self.queue_client
    }

    pub fn metadata(&self) -> &'a Metadata {
        self.metadata
    }

    pub fn timeout(&self) -> &Option<Timeout> {
        &self.timeout
    }

    pub fn with_timeout(self, timeout: Timeout) -> Self {
        Self {
            queue_client: self.queue_client,
            timeout: Some(timeout),
            metadata: self.metadata,
            client_request_id: self.client_request_id,
        }
    }

    pub fn client_request_id(&self) -> &Option<ClientRequestId<'a>> {
        &self.client_request_id
    }

    pub fn with_client_request_id(self, client_request_id: ClientRequestId<'a>) -> Self {
        Self {
            queue_client: self.queue_client,
            timeout: self.timeout,
            metadata: self.metadata,
            client_request_id: Some(client_request_id),
        }
    }

    pub async fn execute(self) -> Result<SetQueueMetadataResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name(),
        ))?;

        url.query_pairs_mut().append_pair("comp", "metadata");
        AppendToUrlQuery::append_to_url_query(self.timeout(), &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_header(self.client_request_id(), request);
                request = AddAsHeader::add_as_header(&self.metadata(), request);
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
