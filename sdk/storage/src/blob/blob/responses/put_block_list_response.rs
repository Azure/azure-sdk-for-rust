use crate::headers::content_md5_from_headers;
use crate::ConsistencyMD5;
use azure_core::headers::{
    date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
    request_server_encrypted_from_headers,
};
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockListResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: ConsistencyMD5,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockListResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> crate::Result<PutBlockListResponse> {
        debug!("headers == {:#?}", headers);

        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let content_md5 = content_md5_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockListResponse {
            etag,
            last_modified,
            content_md5,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
