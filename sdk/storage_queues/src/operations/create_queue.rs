use crate::clients::QueueClient;
use azure_core::{error::Error, prelude::*, Response as AzureResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueBuilder {
    queue_client: QueueClient,
    timeout: Option<Timeout>,
    metadata: Option<Metadata>,
    context: Context,
}

impl CreateQueueBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        CreateQueueBuilder {
            queue_client,
            timeout: None,
            metadata: None,
            context: Context::new(),
        }
    }

    setters! {
        metadata: Metadata => Some(metadata),
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            let mut request = self.queue_client.storage_client().prepare_request(
                url.as_str(),
                http::method::Method::PUT,
                None,
            )?;

            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    request.add_mandatory_header(&m);
                }
            }

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<CreateQueueResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateQueueBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct CreateQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for CreateQueueResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(CreateQueueResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
