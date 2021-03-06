use azure_core::{errors::AzureError, headers::CommonStorageResponseHeaders};
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteFileSystemResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<&Response<Bytes>> for DeleteFileSystemResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("body == {}", std::str::from_utf8(response.body())?);
        debug!("headers == {:?}", response.headers());

        Ok(DeleteFileSystemResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
