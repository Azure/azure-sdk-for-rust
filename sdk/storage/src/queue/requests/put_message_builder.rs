use crate::queue::responses::*;
use crate::queue::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct PutMessageBuilder<'a> {
    queue_client: &'a QueueClient,
    visibility_timeout: Option<VisibilityTimeout>,
    ttl: Option<MessageTTL>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
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
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
        body: impl AsRef<str>,
    ) -> Result<PutMessageResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.queue_client.queue_url()?.join("messages")?;

        self.visibility_timeout.append_to_url_query(&mut url);
        self.ttl.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {}", url.as_str());

        // since the format is fixed we just decorate the message with the tags.
        // This could be made optional in the future and/or more
        // stringent.
        let message = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            body.as_ref()
        );

        debug!("message about to be posted == {}", message);

        let request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::POST,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(message.into()),
        )?;

        let response = self
            .queue_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, http::status::StatusCode::CREATED)
            .await?;

        Ok((&response).try_into()?)
    }
}
