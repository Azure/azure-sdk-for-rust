use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use hyper::header::HeaderMap;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateQueueResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&HeaderMap> for CreateQueueResponse {
    type Error = AzureError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:?}", headers);

        Ok(CreateQueueResponse {
            common_storage_response_headers: headers.try_into()?,
        })
    }
}
