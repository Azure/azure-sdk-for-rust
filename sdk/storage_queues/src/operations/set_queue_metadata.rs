use crate::clients::QueueClient;
use azure_core::{error::Error, prelude::*, Context, Response as AzureResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueMetadataBuilder {
    queue_client: QueueClient,
    metadata: Metadata,
    timeout: Option<Timeout>,
    context: Context,
}

impl SetQueueMetadataBuilder {
    pub(crate) fn new(queue_client: QueueClient, metadata: Metadata) -> Self {
        SetQueueMetadataBuilder {
            queue_client,
            metadata,
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

            let mut request = self.queue_client.storage_client().prepare_request(
                url.as_str(),
                http::method::Method::PUT,
                None,
            )?;
            for m in self.metadata.iter() {
                request.add_mandatory_header(&m);
            }

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<SetQueueMetadataResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetQueueMetadataBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct SetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueMetadataResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
