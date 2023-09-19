use crate::clients::QueueClient;
use azure_core::{error::Error, headers::Headers, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    DeleteQueue,
    client: QueueClient,
}

impl DeleteQueueBuilder {
    pub fn into_future(mut self) -> DeleteQueue {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut request =
                QueueClient::finalize_request(url, Method::Delete, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
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
