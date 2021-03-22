use crate::blob_storage::blob::Blob;
use azure_core::errors::AzureError;
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetBlobPropertiesResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl GetBlobPropertiesResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        blob: Blob,
    ) -> Result<GetBlobPropertiesResponse, AzureError> {
        debug!("headers == {:#?}", headers);

        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetBlobPropertiesResponse {
            blob,
            request_id,
            date,
        })
    }
}
