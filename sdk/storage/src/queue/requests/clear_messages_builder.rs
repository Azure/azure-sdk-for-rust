use crate::queue::clients::QueueClient;
use crate::queue::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ClearMessagesBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> ClearMessagesBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        ClearMessagesBuilder {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<ClearMessagesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.queue_client.queue_url()?.join("messages")?;

        self.timeout.append_to_url_query(&mut url);

        debug!("url == {}", url);

        let request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .queue_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, http::status::StatusCode::NO_CONTENT)
            .await?;

        Ok((&response).try_into()?)
    }
}
