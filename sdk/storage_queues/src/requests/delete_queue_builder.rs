use crate::clients::QueueClient;
use crate::responses::*;
use azure_core::prelude::*;

use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteQueueBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> DeleteQueueBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        DeleteQueueBuilder {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self) -> azure_core::Result<DeleteQueueResponse> {
        let mut url = self.queue_client.url_with_segments(None)?;

        self.timeout.append_to_url_query(&mut url);

        let mut request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            http::method::Method::DELETE,
            None,
        )?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .queue_client
            .storage_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
