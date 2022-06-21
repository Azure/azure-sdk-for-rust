use crate::blob::Blob;
use azure_core::headers::Headers;
use azure_core::{
    headers::{date_from_headers, request_id_from_headers},
    RequestId,
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GetBlobPropertiesResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl GetBlobPropertiesResponse {
    pub(crate) fn from_response(
        headers: &Headers,
        blob: Blob,
    ) -> azure_core::Result<GetBlobPropertiesResponse> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetBlobPropertiesResponse {
            blob,
            request_id,
            date,
        })
    }
}
