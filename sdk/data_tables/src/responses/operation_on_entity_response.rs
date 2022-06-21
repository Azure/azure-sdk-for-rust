use azure_core::{error::Error, headers::etag_from_headers, CollectedResponse, Etag};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct OperationOnEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Etag,
}

impl TryFrom<CollectedResponse> for OperationOnEntityResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        Ok(OperationOnEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            etag: etag_from_headers(response.headers())?.into(),
        })
    }
}
