use crate::{clients::PopReceiptClient, prelude::*};
use azure_core::{
    error::Error,
    headers::Headers,
    headers::{rfc1123_from_headers_mandatory, HeaderName},
    prelude::*,
    Method, Response as AzureResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    UpdateMessage,
    client: PopReceiptClient,
    body: String,
    visibility_timeout: VisibilityTimeout,
}

impl UpdateMessageBuilder {
    pub fn into_future(mut self) -> UpdateMessage {
        Box::pin(async move {
            let mut url = self.client.pop_receipt_url()?;

            self.visibility_timeout.append_to_url_query(&mut url);

            // since the format is fixed we just decorate the message with the tags.
            // This could be made optional in the future and/or more
            // stringent.
            let message = format!(
                "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
                self.body
            );

            let mut request = self.client.storage_client().finalize_request(
                url,
                Method::Put,
                Headers::new(),
                Some(message.into()),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct UpdateMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub time_next_visible: OffsetDateTime,
    pub pop_receipt: String,
}

impl std::convert::TryFrom<AzureResponse> for UpdateMessageResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        Ok(UpdateMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
            time_next_visible: rfc1123_from_headers_mandatory(
                headers,
                &HeaderName::from_static("x-ms-time-next-visible"),
            )?,
            pop_receipt: headers.get_as(&HeaderName::from_static("x-ms-popreceipt"))?,
        })
    }
}
