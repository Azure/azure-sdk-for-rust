use crate::{ConsistencyCRC64, ConsistencyMD5};
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::headers::{
    client_request_id_from_headers_optional, date_from_headers, request_id_from_headers,
    server_from_headers, version_from_headers, Headers,
};
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct CommonStorageResponseHeaders {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

impl TryFrom<&Headers> for CommonStorageResponseHeaders {
    type Error = Error;

    fn try_from(headers: &Headers) -> azure_core::Result<Self> {
        Ok(Self {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
        })
    }
}

pub const CONTENT_CRC64: &str = "x-ms-content-crc64";
pub const CONTENT_MD5: &str = "Content-MD5";
pub const COPY_ID: &str = "x-ms-copy-id";
pub const RENAME_SOURCE: &str = "x-ms-rename-source";

pub fn content_crc64_from_headers(headers: &Headers) -> azure_core::Result<ConsistencyCRC64> {
    content_crc64_from_headers_optional(headers)?.ok_or_else(|| {
        Error::with_message(ErrorKind::DataConversion, || {
            format!("header not found: {}", CONTENT_CRC64)
        })
    })
}

pub fn content_crc64_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<ConsistencyCRC64>> {
    headers
        .get(CONTENT_CRC64)
        .map(|content_crc64| {
            let content_crc64 = content_crc64.as_str();
            ConsistencyCRC64::decode(content_crc64).with_context(ErrorKind::DataConversion, || {
                format!("failed to decode content_crc64 from headers: {content_crc64}")
            })
        })
        .transpose()
}

pub fn content_md5_from_headers(headers: &Headers) -> azure_core::Result<ConsistencyMD5> {
    content_md5_from_headers_optional(headers)?.ok_or_else(|| {
        Error::with_message(ErrorKind::DataConversion, || {
            format!("header not found: {}", CONTENT_MD5)
        })
    })
}

pub fn content_md5_from_headers_optional(
    headers: &Headers,
) -> azure_core::Result<Option<ConsistencyMD5>> {
    headers
        .get(CONTENT_MD5)
        .map(|content_md5| {
            let content_md5 = content_md5.as_str();
            ConsistencyMD5::decode(content_md5).with_context(ErrorKind::DataConversion, || {
                format!("failed to decode content_md5 from headers: {content_md5}")
            })
        })
        .transpose()
}

pub fn consistency_from_headers(
    headers: &Headers,
) -> azure_core::Result<(Option<ConsistencyMD5>, Option<ConsistencyCRC64>)> {
    let content_crc64 = content_crc64_from_headers_optional(headers)?;
    let content_md5 = content_md5_from_headers_optional(headers)?;
    Ok((content_md5, content_crc64))
}
