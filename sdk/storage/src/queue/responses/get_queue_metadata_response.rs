use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::prelude::*;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: Metadata,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueMetadataResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        debug!("headers == {:?}", headers);

        Ok(GetQueueMetadataResponse {
            common_storage_response_headers: headers.try_into()?,
            metadata: headers.into(),
        })
    }
}
