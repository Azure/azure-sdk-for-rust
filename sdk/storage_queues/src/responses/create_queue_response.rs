use azure_core::{error::Error, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<CollectedResponse> for CreateQueueResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        Ok(CreateQueueResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
