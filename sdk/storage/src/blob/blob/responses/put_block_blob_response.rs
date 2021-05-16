use crate::AzureStorageError;
use azure_core::headers::{
    consistency_from_headers, date_from_headers, etag_from_headers, last_modified_from_headers,
    request_id_from_headers, request_server_encrypted_from_headers,
};
use azure_core::{Consistency, RequestId};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub consistency: Consistency,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<PutBlockBlobResponse, AzureStorageError> {
        debug!("headers == {:#?}", headers);

        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let consistency = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockBlobResponse {
            etag,
            last_modified,
            consistency,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
