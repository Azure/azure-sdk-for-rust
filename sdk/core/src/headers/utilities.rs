use super::*;
use crate::errors::*;
use crate::request_options::LeaseId;
use crate::util::HeaderMapExt;
use crate::{Consistency, RequestId, SessionToken};
use chrono::{DateTime, Utc};
use http::header::{HeaderName, DATE, ETAG, LAST_MODIFIED};
#[cfg(feature = "enable_hyper")]
use http::status::StatusCode;
use http::HeaderMap;
#[cfg(feature = "enable_hyper")]
use hyper::{Body, Client, Request};
use std::str::FromStr;
use uuid::Uuid;

use std::convert::TryFrom;

pub fn lease_id_from_headers(headers: &HeaderMap) -> Result<LeaseId, AzureError> {
    let lease_id = headers
        .get_as_str(LEASE_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(LEASE_ID.to_owned()))?;
    Ok(LeaseId::from_str(lease_id)?)
}

pub fn request_id_from_headers(headers: &HeaderMap) -> Result<RequestId, AzureError> {
    let request_id = headers
        .get_as_str(REQUEST_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(REQUEST_ID.to_owned()))?;
    Ok(Uuid::parse_str(request_id)?)
}

pub fn client_request_id_from_headers_optional(headers: &HeaderMap) -> Option<String> {
    headers.get_as_str(CLIENT_REQUEST_ID).map(|s| s.to_owned())
}

pub fn content_md5_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<[u8; 16]>, AzureError> {
    if headers.contains_key(CONTENT_MD5) {
        Ok(Some(content_md5_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub struct CommonStorageResponseHeaders {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
    pub date: DateTime<Utc>,
    pub server: String,
}

impl TryFrom<&HeaderMap> for CommonStorageResponseHeaders {
    type Error = AzureError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
        })
    }
}

pub fn content_md5_from_headers(headers: &HeaderMap) -> Result<[u8; 16], AzureError> {
    let content_md5 = headers
        .get(CONTENT_MD5)
        .ok_or_else(|| AzureError::HeaderNotFound(CONTENT_MD5.to_owned()))?
        .to_str()?;

    let content_md5_vec = base64::decode(&content_md5)?;

    if content_md5_vec.len() != 16 {
        return Err(AzureError::DigestNot16BytesLong(
            content_md5_vec.len() as u64
        ));
    }
    let mut content_md5 = [0; 16];
    content_md5.copy_from_slice(&content_md5_vec[0..16]);

    trace!("content_md5 == {:?}", content_md5);
    Ok(content_md5)
}

pub fn content_crc64_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<[u8; 8]>, AzureError> {
    if headers.contains_key(CONTENT_CRC64) {
        Ok(Some(content_crc64_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn content_crc64_from_headers(headers: &HeaderMap) -> Result<[u8; 8], AzureError> {
    let content_crc64 = headers
        .get(CONTENT_CRC64)
        .ok_or_else(|| AzureError::HeaderNotFound(CONTENT_CRC64.to_owned()))?
        .to_str()?;

    let content_crc64_vec = base64::decode(&content_crc64)?;

    if content_crc64_vec.len() != 8 {
        return Err(AzureError::CRC64Not8BytesLong(
            content_crc64_vec.len() as u64
        ));
    }
    let mut content_crc64 = [0; 8];
    content_crc64.copy_from_slice(&content_crc64_vec[0..8]);

    trace!("content_crc64 == {:?}", content_crc64);
    Ok(content_crc64)
}

pub fn consistency_from_headers(headers: &HeaderMap) -> Result<Consistency, AzureError> {
    if let Some(content_crc64) = content_crc64_from_headers_optional(headers)? {
        return Ok(Consistency::Crc64(content_crc64));
    } else if let Some(content_md5) = content_md5_from_headers_optional(headers)? {
        return Ok(Consistency::Md5(content_md5));
    }

    Err(AzureError::HeadersNotFound(vec![
        CONTENT_CRC64.to_owned(),
        CONTENT_MD5.to_owned(),
    ]))
}

pub fn last_modified_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<DateTime<Utc>>, AzureError> {
    if headers.contains_key(LAST_MODIFIED) {
        Ok(Some(last_modified_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn rfc2822_from_headers_mandatory(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>, AzureError> {
    let val = headers
        .get(header_name)
        .ok_or_else(|| AzureError::HeaderNotFound(header_name.to_owned()))?
        .to_str()?;
    let val = DateTime::parse_from_rfc2822(val)?;
    let val = DateTime::from_utc(val.naive_utc(), Utc);

    trace!("header {} == {:?}", header_name, val);
    Ok(val)
}

pub fn string_from_headers_mandatory<'a>(
    headers: &'a HeaderMap,
    header_name: &str,
) -> Result<&'a str, AzureError> {
    Ok(headers
        .get(header_name)
        .ok_or_else(|| AzureError::HeaderNotFound(header_name.to_owned()))?
        .to_str()?)
}

pub fn last_modified_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, AzureError> {
    rfc2822_from_headers_mandatory(headers, LAST_MODIFIED.as_str())
}

pub fn continuation_token_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<String>, AzureError> {
    if let Some(hc) = headers.get(CONTINUATION) {
        Ok(Some(hc.to_str()?.to_owned()))
    } else {
        Ok(None)
    }
}

#[inline]
pub fn utc_date_from_rfc2822(date: &str) -> Result<DateTime<Utc>, AzureError> {
    let date = DateTime::parse_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, AzureError> {
    let date = headers
        .get(DATE)
        .ok_or_else(|| {
            static D: HeaderName = DATE;
            AzureError::HeaderNotFound(D.as_str().to_owned())
        })?
        .to_str()?;
    let date = DateTime::parse_from_rfc2822(date)?;
    let date = DateTime::from_utc(date.naive_utc(), Utc);

    trace!("date == {:?}", date);
    Ok(date)
}

pub fn sku_name_from_headers(headers: &HeaderMap) -> Result<String, AzureError> {
    let sku_name = headers
        .get(SKU_NAME)
        .ok_or_else(|| AzureError::HeaderNotFound(SKU_NAME.to_owned()))?
        .to_str()?;
    trace!("sku_name == {:?}", sku_name);
    Ok(sku_name.to_owned())
}

pub fn account_kind_from_headers(headers: &HeaderMap) -> Result<String, AzureError> {
    let account_kind = headers
        .get(ACCOUNT_KIND)
        .ok_or_else(|| AzureError::HeaderNotFound(ACCOUNT_KIND.to_owned()))?
        .to_str()?;
    trace!("account_kind == {:?}", account_kind);
    Ok(account_kind.to_owned())
}

pub fn etag_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>, AzureError> {
    if headers.contains_key(ETAG) {
        Ok(Some(etag_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn etag_from_headers(headers: &HeaderMap) -> Result<String, AzureError> {
    let etag = headers
        .get(ETAG)
        .ok_or_else(|| {
            static E: HeaderName = ETAG;
            AzureError::HeaderNotFound(E.as_str().to_owned())
        })?
        .to_str()?
        .to_owned();

    trace!("etag == {:?}", etag);
    Ok(etag)
}

pub fn lease_time_from_headers(headers: &HeaderMap) -> Result<u8, AzureError> {
    let lease_time = headers
        .get(LEASE_TIME)
        .ok_or_else(|| AzureError::HeaderNotFound(LEASE_TIME.to_owned()))?
        .to_str()?;

    let lease_time = lease_time.parse::<u8>()?;

    trace!("lease_time == {:?}", lease_time);
    Ok(lease_time)
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<bool, AzureError> {
    let delete_type_permanent = headers
        .get(DELETE_TYPE_PERMANENT)
        .ok_or_else(|| AzureError::HeaderNotFound(DELETE_TYPE_PERMANENT.to_owned()))?
        .to_str()?;

    let delete_type_permanent = delete_type_permanent.parse::<bool>()?;

    trace!("delete_type_permanent == {:?}", delete_type_permanent);
    Ok(delete_type_permanent)
}

#[cfg(feature = "azurite_workaround")]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<Option<bool>, AzureError> {
    let delete_type_permanent = headers
        .get(DELETE_TYPE_PERMANENT)
        .map(|delete_type_permanent| -> Result<_, AzureError> {
            Ok(delete_type_permanent.to_str()?.parse::<bool>()?)
        })
        .transpose()?;

    trace!("delete_type_permanent == {:?}", delete_type_permanent);
    Ok(delete_type_permanent)
}

pub fn sequence_number_from_headers(headers: &HeaderMap) -> Result<u64, AzureError> {
    let sequence_number = headers
        .get(BLOB_SEQUENCE_NUMBER)
        .ok_or_else(|| AzureError::HeaderNotFound(BLOB_SEQUENCE_NUMBER.to_owned()))?
        .to_str()?;

    let sequence_number = sequence_number.parse::<u64>()?;

    trace!("sequence_number == {:?}", sequence_number);
    Ok(sequence_number)
}

pub fn session_token_from_headers(headers: &HeaderMap) -> Result<SessionToken, AzureError> {
    Ok(headers
        .get(SESSION_TOKEN)
        .ok_or_else(|| AzureError::HeaderNotFound(SESSION_TOKEN.to_owned()))?
        .to_str()?
        .to_owned())
}

pub fn server_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(SERVER)
        .ok_or_else(|| AzureError::HeaderNotFound(SERVER.to_owned()))?
        .to_str()?)
}

pub fn version_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(VERSION)
        .ok_or_else(|| AzureError::HeaderNotFound(VERSION.to_owned()))?
        .to_str()?)
}

pub fn request_server_encrypted_from_headers(headers: &HeaderMap) -> Result<bool, AzureError> {
    let request_server_encrypted = headers
        .get(REQUEST_SERVER_ENCRYPTED)
        .ok_or_else(|| AzureError::HeaderNotFound(REQUEST_SERVER_ENCRYPTED.to_owned()))?
        .to_str()?;

    let request_server_encrypted = request_server_encrypted.parse::<bool>()?;

    trace!("request_server_encrypted == {:?}", request_server_encrypted);
    Ok(request_server_encrypted)
}

pub fn content_type_from_headers(headers: &HeaderMap) -> Result<&str, AzureError> {
    Ok(headers
        .get(http::header::CONTENT_TYPE)
        .ok_or_else(|| {
            let header = http::header::CONTENT_TYPE;
            AzureError::HeaderNotFound(header.as_str().to_owned())
        })?
        .to_str()?)
}

#[cfg(feature = "enable_hyper")]
pub async fn perform_http_request(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    req: Request<Body>,
    expected_status: StatusCode,
) -> Result<String, AzureError> {
    debug!("req == {:?}", req);
    let res = client.request(req).await?;
    check_status_extract_body_2(res, expected_status).await
}
