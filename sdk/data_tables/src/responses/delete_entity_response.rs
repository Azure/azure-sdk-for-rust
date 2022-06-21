use azure_core::{error::Error, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<CollectedResponse> for DeleteEntityResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        Ok(DeleteEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
