use crate::prelude::*;
use crate::responses::*;
use azure_core::prelude::*;

use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PutMessageBuilder<'a> {
    queue_client: &'a QueueClient,
    visibility_timeout: Option<VisibilityTimeout>,
    ttl: Option<MessageTTL>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> PutMessageBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        PutMessageBuilder {
            queue_client,
            visibility_timeout: None,
            ttl: None,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        visibility_timeout: VisibilityTimeout => Some(visibility_timeout),
        ttl: MessageTTL => Some(ttl),
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(&self, body: impl AsRef<str>) -> azure_core::Result<PutMessageResponse> {
        let mut url = self.queue_client.url_with_segments(Some("messages"))?;

        self.visibility_timeout.append_to_url_query(&mut url);
        self.ttl.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        // since the format is fixed we just decorate the message with the tags.
        // This could be made optional in the future and/or more
        // stringent.
        let message = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            body.as_ref()
        );

        let mut request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            http::method::Method::POST,
            Some(message.into()),
        )?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .queue_client
            .storage_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
