use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
    request_server_encrypted_from_headers, RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct PutBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<PutBlobResponse, AzureError> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlobResponse {
            etag,
            last_modified,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
