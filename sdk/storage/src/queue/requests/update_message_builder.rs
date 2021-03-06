use crate::queue::clients::PopReceiptClient;
use crate::queue::responses::*;
use crate::queue::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct UpdateMessageBuilder<'a> {
    pop_receipt_client: &'a PopReceiptClient,
    visibility_timeout: VisibilityTimeout,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> UpdateMessageBuilder<'a> {
    pub(crate) fn new(
        pop_receipt_client: &'a PopReceiptClient,
        visibility_timeout: impl Into<VisibilityTimeout>,
    ) -> Self {
        UpdateMessageBuilder {
            pop_receipt_client,
            visibility_timeout: visibility_timeout.into(),
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
        new_body: impl AsRef<str>,
    ) -> Result<UpdateMessageResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.pop_receipt_client.pop_receipt_url()?;

        self.visibility_timeout.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {}", url.as_str());

        // since the format is fixed we just decorate the message with the tags.
        // This could be made optional in the future and/or more
        // stringent.
        let message = format!(
            "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
            new_body.as_ref()
        );

        debug!("message about to be put == {}", message);

        let request = self.pop_receipt_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(message.into()),
        )?;

        let response = self
            .pop_receipt_client
            .http_client()
            .execute_request_check_status(request.0, http::status::StatusCode::NO_CONTENT)
            .await?;

        Ok((&response).try_into()?)
    }
}
