use azure_core::error::{Error, Result};
use azure_core::prelude::*;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueMetadataResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: Metadata,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueMetadataResponse {
    type Error = Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self> {
        let headers = response.headers();

        debug!("headers == {:?}", headers);

        Ok(GetQueueMetadataResponse {
            common_storage_response_headers: headers.try_into()?,
            metadata: headers.into(),
        })
    }
}
