use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{
        date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
        request_server_encrypted_from_headers, Headers,
    },
    RequestId,
};
use azure_storage::{headers::consistency_from_headers, ConsistencyCRC64, ConsistencyMD5};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: Option<ConsistencyMD5>,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockBlobResponse {
    pub fn from_headers(headers: &Headers) -> azure_core::Result<PutBlockBlobResponse> {
        debug!("headers == {:#?}", headers);

        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let (content_md5, content_crc64) =
            consistency_from_headers(headers).map_kind(ErrorKind::DataConversion)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockBlobResponse {
            etag,
            last_modified,
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
