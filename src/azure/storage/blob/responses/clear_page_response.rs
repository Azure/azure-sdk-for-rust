use azure::core::errors::AzureError;
use azure::core::{
    date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers, sequence_number_from_headers, RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ClearPageResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub sequence_number: u64,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl ClearPageResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<ClearPageResponse, AzureError> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let sequence_number = sequence_number_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(ClearPageResponse {
            etag,
            last_modified,
            sequence_number,
            request_id,
            date,
        })
    }
}
