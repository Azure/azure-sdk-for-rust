use crate::clients::QueueClient;
use crate::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;

use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueMetadataBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> GetQueueMetadataBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        Self {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<GetQueueMetadataResponse> {
        let mut url = self.queue_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "metadata");

        self.timeout.append_to_url_query(&mut url);

        trace!("url == {}", url);

        let request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request.add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .queue_client
            .storage_client()
            .storage_account_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
