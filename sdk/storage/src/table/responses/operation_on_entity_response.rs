use azure_core::{
    headers::{etag_from_headers, CommonStorageResponseHeaders},
    prelude::Etag,
};
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct OperationOnEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
}

impl TryFrom<&Response<Bytes>> for OperationOnEntityResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        println!("{}", std::str::from_utf8(response.body())?);
        println!("headers == {:#?}", response.headers());

        Ok(OperationOnEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: etag_from_headers(response.headers())?.into(),
        })
    }
}
