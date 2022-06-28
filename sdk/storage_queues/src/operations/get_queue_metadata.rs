use crate::clients::QueueClient;
use azure_core::{error::Error, prelude::*, Context, Method, Response as AzureResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueMetadataBuilder {
    queue_client: QueueClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetQueueMetadataBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        Self {
            queue_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "metadata");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.queue_client
                    .storage_client()
                    .prepare_request(url, Method::GET, None)?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<GetQueueMetadataResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetQueueMetadataBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: Metadata,
}

impl std::convert::TryFrom<AzureResponse> for GetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(GetQueueMetadataResponse {
            common_storage_response_headers: headers.try_into()?,
            metadata: headers.into(),
        })
    }
}
