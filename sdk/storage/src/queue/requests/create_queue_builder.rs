use crate::core::prelude::*;
use crate::queue::clients::QueueClient;
use crate::queue::responses::*;
use crate::queue::HasStorageClient;
use azure_core::errors::AzureError;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_client: &'a QueueClient<C>,
    timeout: Option<Timeout>,
    metadata: Option<&'a Metadata>,
    client_request_id: Option<ClientRequestId<'a>>,
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

impl<'a, C> CreateQueueBuilder<'a, C>
where
    C: Client + Clone,
{
    setters! {
        metadata: &'a Metadata => Some(metadata),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(self) -> Result<CreateQueueResponse, AzureError> {
        let mut url = url::Url::parse(&format!(
            "{}/{}",
            self.queue_client.storage_client().queue_uri(),
            self.queue_client.queue_name(),
        ))?;

        AppendToUrlQuery::append_to_url_query(&self.timeout, &mut url);

        debug!("uri == {}", url);

        let perform_request_response = self.queue_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header(&self.metadata, request);
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
