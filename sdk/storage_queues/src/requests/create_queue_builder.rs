use crate::clients::QueueClient;
use crate::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    metadata: Option<&'a Metadata>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> CreateQueueBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        CreateQueueBuilder {
            queue_client,
            timeout: None,
            metadata: None,
            client_request_id: None,
        }
    }

    setters! {
        metadata: &'a Metadata => Some(metadata),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<CreateQueueResponse> {
        let mut url = self.queue_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        let request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::PUT,
            &|mut request| {
                request.add_optional_header(&self.client_request_id, request);
                if let Some(metadata) = &self.metadata {
                    for m in metadata.iter() {
                        request.add_mandatory_header(&m, request);
                    }
                }
                request
            },
            None,
        )?;

        let response = self
            .queue_client
            .storage_client()
            .storage_account_client()
            .execute_request_check_status(request.0, http::status::StatusCode::CREATED)
            .await?;

        response.try_into()
    }
}
