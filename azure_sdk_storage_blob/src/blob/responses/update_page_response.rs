use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    content_md5_from_headers, date_from_headers, etag_from_headers, last_modified_from_headers,
    request_id_from_headers, request_server_encrypted_from_headers, sequence_number_from_headers,
    RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePageResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: [u8; 16],
    pub sequence_number: u64,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl UpdatePageResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<UpdatePageResponse, AzureError> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers)?;
        let sequence_number = sequence_number_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(UpdatePageResponse {
            etag,
            last_modified,
            content_md5,
            sequence_number,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
