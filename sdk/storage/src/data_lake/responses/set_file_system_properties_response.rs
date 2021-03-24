use azure_core::{
    errors::AzureError,
    headers::{etag_from_headers, last_modified_from_headers, CommonStorageResponseHeaders},
    prelude::Etag,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct SetFileSystemPropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
    pub last_modified: DateTime<Utc>,
}

impl TryFrom<&Response<Bytes>> for SetFileSystemPropertiesResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        trace!("body == {}", std::str::from_utf8(response.body())?);
        trace!("headers == {:?}", response.headers());

        Ok(SetFileSystemPropertiesResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: Etag::from(etag_from_headers(response.headers())?),
            last_modified: last_modified_from_headers(response.headers())?,
        })
    }
}
