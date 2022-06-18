use azure_core::error::Error;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ClearMessagesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<CollectedResponse> for ClearMessagesResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        debug!("response == {:?}", response);

        Ok(ClearMessagesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
