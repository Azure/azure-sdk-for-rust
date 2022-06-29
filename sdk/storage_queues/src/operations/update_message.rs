use crate::{clients::PopReceiptClient, prelude::*};
use azure_core::{
    error::Error,
    headers::Headers,
    headers::{rfc2822_from_headers_mandatory, HeaderName},
    prelude::*,
    Context, Method, Response as AzureResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct UpdateMessageBuilder {
    pop_receipt_client: PopReceiptClient,
    body: String,
    visibility_timeout: VisibilityTimeout,
    timeout: Option<Timeout>,
    context: Context,
}

impl UpdateMessageBuilder {
    pub(crate) fn new(
        pop_receipt_client: PopReceiptClient,
        body: String,
        visibility_timeout: VisibilityTimeout,
    ) -> Self {
        UpdateMessageBuilder {
            pop_receipt_client,
            body,
            visibility_timeout,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.pop_receipt_client.pop_receipt_url()?;

            self.visibility_timeout.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            // since the format is fixed we just decorate the message with the tags.
            // This could be made optional in the future and/or more
            // stringent.
            let message = format!(
                "<QueueMessage><MessageText>{}</MessageText></QueueMessage>",
                self.body
            );

            let mut request = self.pop_receipt_client.storage_client().finalize_request(
                url,
                Method::Put,
                Headers::new(),
                Some(message.into()),
            )?;

            let response = self
                .pop_receipt_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<UpdateMessageResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for UpdateMessageBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct UpdateMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub time_next_visible: DateTime<Utc>,
    pub pop_receipt: String,
}

impl std::convert::TryFrom<AzureResponse> for UpdateMessageResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        Ok(UpdateMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
            time_next_visible: rfc2822_from_headers_mandatory(
                headers,
                &HeaderName::from_static("x-ms-time-next-visible"),
            )?,
            pop_receipt: headers.get_string(&HeaderName::from_static("x-ms-popreceipt"))?,
        })
    }
}
