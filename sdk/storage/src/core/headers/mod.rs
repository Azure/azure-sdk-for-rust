use crate::{ConsistencyCRC64, ConsistencyMD5};
use azure_core::error::{Error, ErrorKind, Result, ResultExt};
use azure_core::headers::*;
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct CommonStorageResponseHeaders {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

impl TryFrom<&HeaderMap> for CommonStorageResponseHeaders {
    type Error = Error;

    fn try_from(headers: &HeaderMap) -> Result<Self> {
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

pub fn content_crc64_from_headers(headers: &HeaderMap) -> Result<ConsistencyCRC64> {
    let content_crc64 = headers
        .get(CONTENT_CRC64)
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("header not found: {}", CONTENT_CRC64)
            })
        })?
        .to_str()
        .context(
            ErrorKind::DataConversion,
            "failed to convert content_crc64 header value to string",
        )?;

    let content_crc64 = ConsistencyCRC64::decode(content_crc64)
        .with_context(ErrorKind::DataConversion, || {
            format!("failed to decode content_crc64 from headers: {content_crc64}")
        })?;
    trace!("content_crc64 == {:?}", content_crc64);
    Ok(content_crc64)
}

pub fn content_crc64_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<ConsistencyCRC64>> {
    if headers.contains_key(CONTENT_CRC64) {
        Ok(Some(content_crc64_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn content_md5_from_headers(headers: &HeaderMap) -> Result<ConsistencyMD5> {
    let content_md5 = headers
        .get(CONTENT_MD5)
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("header not found: {}", CONTENT_CRC64)
            })
        })?
        .to_str()
        .context(
            ErrorKind::DataConversion,
            "failed to convert content_md5 header to string",
        )?;

    let content_md5 = ConsistencyMD5::decode(content_md5)?;
    trace!("content_md5 == {:?}", content_md5);
    Ok(content_md5)
}

pub fn content_md5_from_headers_optional(headers: &HeaderMap) -> Result<Option<ConsistencyMD5>> {
    if headers.contains_key(CONTENT_MD5) {
        Ok(Some(content_md5_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn consistency_from_headers(
    headers: &HeaderMap,
) -> Result<(Option<ConsistencyMD5>, Option<ConsistencyCRC64>)> {
    let content_crc64 = content_crc64_from_headers_optional(headers)?;
    let content_md5 = content_md5_from_headers_optional(headers)?;
    Ok((content_md5, content_crc64))
}
