use azure_core::error::Error;
use azure_core::{prelude::*, CollectedResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: Metadata,
}

impl std::convert::TryFrom<CollectedResponse> for GetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        debug!("headers == {:?}", headers);

        Ok(GetQueueMetadataResponse {
            common_storage_response_headers: headers.try_into()?,
            metadata: headers.into(),
        })
    }
}
