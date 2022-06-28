use crate::clients::QueueClient;
use azure_core::{
    error::Error, headers::Headers, prelude::*, Context, Method, Response as AzureResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteQueueBuilder {
    queue_client: QueueClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl DeleteQueueBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        DeleteQueueBuilder {
            queue_client,
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
            let mut url = self.queue_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            let mut request = self.queue_client.storage_client().prepare_request(
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

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<DeleteQueueResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteQueueBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for DeleteQueueResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(DeleteQueueResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
