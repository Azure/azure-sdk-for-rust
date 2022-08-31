use crate::clients::QueueClient;
use azure_core::{error::Error, headers::Headers, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    ClearMessages,
    client: QueueClient,
}

impl ClearMessagesBuilder {
    pub fn into_future(mut self) -> ClearMessages {
        Box::pin(async move {
            let url = self.client.url_with_segments(Some("messages"))?;

            let mut request = self.client.storage_client().finalize_request(
                url,
                Method::Delete,
                Headers::new(),
                None,
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
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
