use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ClearMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for ClearMessagesResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("response == {:?}", response);

        Ok(ClearMessagesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
