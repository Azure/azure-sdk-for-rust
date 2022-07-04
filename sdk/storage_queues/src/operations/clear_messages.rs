use crate::clients::QueueClient;
use azure_core::{error::Error, headers::Headers, prelude::*, Method, Response as AzureResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug)]
pub struct ClearMessagesBuilder {
    queue_client: QueueClient,
    context: Context,
}

impl ClearMessagesBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        ClearMessagesBuilder {
            queue_client,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let url = self.queue_client.url_with_segments(Some("messages"))?;

            let mut request = self.queue_client.storage_client().finalize_request(
                url,
                Method::Delete,
                Headers::new(),
                None,
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<ClearMessagesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ClearMessagesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct ClearMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for ClearMessagesResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(ClearMessagesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
