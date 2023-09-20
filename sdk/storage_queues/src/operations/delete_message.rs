use crate::clients::PopReceiptClient;
use azure_core::{error::Error, headers::Headers, Method, Response as AzureResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    DeleteMessage,
    client: PopReceiptClient,
}

impl DeleteMessageBuilder {
    pub fn into_future(mut self) -> DeleteMessage {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut request =
                PopReceiptClient::finalize_request(url, Method::Delete, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            response.try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeleteMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for DeleteMessageResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(DeleteMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
