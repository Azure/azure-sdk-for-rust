use crate::blob::{copy_status_from_headers, CopyStatus};
use azure_core::headers::{
    date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
    server_from_headers, version_from_headers,
};
use azure_core::RequestId;
use azure_storage::core::CopyId;
use azure_storage::ConsistencyMD5;
use azure_storage::{core::copy_id_from_headers, headers::content_md5_from_headers_optional};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobFromUrlResponse {
    pub content_md5: Option<ConsistencyMD5>,
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub server: String,
    pub request_id: RequestId,
    pub version: String,
    pub copy_id: CopyId,
    pub copy_status: CopyStatus,
    pub date: DateTime<Utc>,
}

impl TryFrom<&HeaderMap> for CopyBlobFromUrlResponse {
    type Error = crate::Error;
    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:#?}", headers);
        Ok(Self {
            content_md5: content_md5_from_headers_optional(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?.to_owned(),
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}
