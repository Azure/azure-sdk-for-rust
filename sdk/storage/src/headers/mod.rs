use crate::{ConsistencyCRC64, ConsistencyMD5};
use azure_core::error::Error;
use azure_core::headers::{
    client_request_id_from_headers_optional, date_from_headers, request_id_from_headers,
    server_from_headers, version_from_headers, HeaderName, Headers, CONTENT_MD5,
};
use azure_core::RequestId;
use std::convert::TryFrom;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct CommonStorageResponseHeaders {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub date: OffsetDateTime,
    pub server: String,
}

impl TryFrom<&Headers> for CommonStorageResponseHeaders {
    type Error = Error;

    fn try_from(headers: &Headers) -> azure_core::Result<Self> {
        Ok(Self {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?,
            date: date_from_headers(headers)?,
            server: server_from_headers(headers)?,
        })
    }
}

pub const CONTENT_CRC64: HeaderName = HeaderName::from_static("x-ms-content-crc64");
pub const COPY_ID: HeaderName = HeaderName::from_static("x-ms-copy-id");
pub const RENAME_SOURCE: HeaderName = HeaderName::from_static("x-ms-rename-source");

pub fn content_crc64_from_headers(headers: &Headers) -> azure_core::Result<ConsistencyCRC64> {
    headers.get_as(&CONTENT_CRC64)
}

pub fn content_crc64_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<ConsistencyCRC64>> {
    headers.get_optional_as(&CONTENT_CRC64)
}

pub fn content_md5_from_headers(headers: &Headers) -> azure_core::Result<ConsistencyMD5> {
    headers.get_as(&CONTENT_MD5)
}

pub fn content_md5_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<ConsistencyMD5>> {
    headers.get_optional_as(&CONTENT_MD5)
}

pub fn consistency_from_headers(
    headers: &Headers,
) -> azure_core::Result<(Option<ConsistencyMD5>, Option<ConsistencyCRC64>)> {
    let content_crc64 = content_crc64_from_headers_optional(headers)?;
    let content_md5 = content_md5_from_headers_optional(headers)?;
    Ok((content_md5, content_crc64))
}
