use crate::AzureStorageError;
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for DeleteMessageResponse {
    type Error = AzureStorageError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("response == {:?}", response);

        Ok(DeleteMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
