use crate::{table::prelude::*, AzureStorageError};
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct CreateTableResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub table: Table,
}

impl TryFrom<&Response<Bytes>> for CreateTableResponse {
    type Error = AzureStorageError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("{}", std::str::from_utf8(response.body())?);
        debug!("headers == {:#?}", response.headers());

        Ok(CreateTableResponse {
            common_storage_response_headers: response.headers().try_into()?,
            table: serde_json::from_slice(response.body())?,
        })
    }
}
