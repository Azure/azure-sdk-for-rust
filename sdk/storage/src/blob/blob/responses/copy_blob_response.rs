use crate::blob::blob::{copy_status_from_headers, CopyStatus};
use crate::core::{copy_id_from_headers, CopyId};
use azure_core::headers::{
    client_request_id_from_headers_optional, date_from_headers, etag_from_headers,
    last_modified_from_headers, request_id_from_headers, server_from_headers, version_from_headers,
};
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub version: String,
    pub server: String,
    pub date: DateTime<Utc>,
    pub copy_id: CopyId,
    pub copy_status: CopyStatus,
    pub client_request_id: Option<String>,
}

impl TryFrom<&HeaderMap> for CopyBlobResponse {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        trace!("CopyBlobResponse headers == {:#?}", headers);
        Ok(Self {
            etag: etag_from_headers(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            request_id: request_id_from_headers(headers)?,
            version: version_from_headers(headers)?.to_owned(),
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            copy_id: copy_id_from_headers(headers)?,
            copy_status: copy_status_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
        })
    }
}
