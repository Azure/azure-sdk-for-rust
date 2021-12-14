use crate::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<&Response<Bytes>> for DeleteEntityResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        Ok(DeleteEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
