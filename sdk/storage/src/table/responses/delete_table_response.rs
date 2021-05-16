use crate::AzureStorageError;
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteTableResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<&Response<Bytes>> for DeleteTableResponse {
    type Error = AzureStorageError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        Ok(DeleteTableResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
