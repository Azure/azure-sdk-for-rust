use crate::queue::clients::QueueClient;
use crate::queue::prelude::*;
use crate::queue::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PeekMessagesBuilder<'a> {
    queue_client: &'a QueueClient,
    number_of_messages: Option<NumberOfMessages>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> PeekMessagesBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        PeekMessagesBuilder {
            queue_client,
            number_of_messages: None,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        number_of_messages: NumberOfMessages => Some(number_of_messages),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<PeekMessagesResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.queue_client.queue_url()?.join("messages")?;

        url.query_pairs_mut().append_pair("peekonly", "true");
        self.number_of_messages.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        debug!("url == {}", url);

        let request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::GET,
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
            .execute_request_check_status(request.0, http::status::StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }
}
