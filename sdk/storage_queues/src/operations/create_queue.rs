use crate::clients::QueueClient;
use azure_core::{error::Error, headers::Headers, prelude::*, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    CreateQueue,
    client: QueueClient,
    ?metadata: Metadata
}

impl CreateQueueBuilder {
    pub fn into_future(mut self) -> CreateQueue {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            if let Some(metadata) = &self.metadata {
                for m in metadata.iter() {
                    headers.add(m);
                }
            }

            let mut request = QueueClient::finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
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
