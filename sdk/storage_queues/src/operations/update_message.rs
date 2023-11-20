use crate::{clients::PopReceiptClient, prelude::*, queue_message::QueueMessageSubmit};
use azure_core::{
    error::Error,
    headers::Headers,
    headers::{rfc1123_from_headers_mandatory, HeaderName},
    prelude::*,
    xml::to_xml,
    Method, Response as AzureResponse,
};
use azure_storage::headers::CommonStorageResponseHeaders;
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
            let mut url = self.client.url()?;

            self.visibility_timeout.append_to_url_query(&mut url);

            let message = to_xml(&QueueMessageSubmit {
                message_text: self.body,
            })?;

            let mut request = PopReceiptClient::finalize_request(
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
