use super::*;
use crate::error::{Error, ErrorKind, Result};
use crate::request_options::LeaseId;
use crate::{RequestId, SessionToken};

use chrono::{DateTime, FixedOffset, Utc};
use http::header::{DATE, ETAG, LAST_MODIFIED, SERVER};
use http::HeaderMap;
use std::str::FromStr;

pub fn get_option_str_from_headers<'a>(
    headers: &'a HeaderMap,
    key: &str,
) -> Result<Option<&'a str>> {
    let h = match headers.get(key) {
        Some(h) => h,
        None => return Ok(None),
    };
    Ok(Some(h.to_str().map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!("could not convert header '{key}' to string"),
        )
    })?))
}

pub fn get_str_from_headers<'a>(headers: &'a HeaderMap, key: &str) -> Result<&'a str> {
    get_option_str_from_headers(headers, key)?.ok_or_else(|| {
        Error::with_message(ErrorKind::DataConversion, || {
            format!("could not find '{key}' in headers")
        })
    })
}

pub fn get_option_from_headers<T>(headers: &HeaderMap, key: &str) -> Result<Option<T>>
where
    T: std::str::FromStr + 'static,
    T::Err: std::error::Error + Send + Sync,
{
    let h = match get_option_str_from_headers(headers, key)? {
        Some(h) => h,
        None => return Ok(None),
    };

    Ok(Some(h.parse().map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!(
                "failed to parse header '{}' as {:?}",
                key,
                std::any::TypeId::of::<T>()
            ),
        )
    })?))
}

pub fn get_from_headers<T>(headers: &HeaderMap, key: &str) -> Result<T>
where
    T: std::str::FromStr + 'static,
    T::Err: std::error::Error + Send + Sync,
{
    get_str_from_headers(headers, key)?.parse().map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!(
                "failed to parse header '{}' as {:?}",
                key,
                std::any::TypeId::of::<T>()
            ),
        )
    })
}

pub fn parse_date_from_str(date: &str, fmt: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_str(date, fmt).map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!(
                "failed to parse date '{}' with format string {:?}",
                date, fmt
            ),
        )
    })
}

pub fn parse_date_from_rfc2822(date: &str) -> Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc2822(date).map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!("failed to parse date '{}' with as rfc2822", date),
        )
    })
}

pub fn parse_int<F>(s: &str) -> Result<F>
where
    F: FromStr<Err = std::num::ParseIntError>,
{
    FromStr::from_str(s).map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!("failed to parse string '{}' as int", s),
        )
    })
}

pub fn lease_id_from_headers(headers: &HeaderMap) -> Result<LeaseId> {
    get_from_headers(headers, LEASE_ID)
}

pub fn request_id_from_headers(headers: &HeaderMap) -> Result<RequestId> {
    get_from_headers(headers, REQUEST_ID)
}

pub fn client_request_id_from_headers_optional(headers: &HeaderMap) -> Option<String> {
    get_option_from_headers(headers, CLIENT_REQUEST_ID)
        .ok()
        .flatten()
}

pub fn last_modified_from_headers_optional(headers: &HeaderMap) -> Result<Option<DateTime<Utc>>> {
    get_option_from_headers(headers, LAST_MODIFIED.as_str())
}

pub fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>> {
    rfc2822_from_headers_mandatory(headers, DATE.as_str())
}

pub fn last_modified_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>> {
    rfc2822_from_headers_mandatory(headers, LAST_MODIFIED.as_str())
}

pub fn rfc2822_from_headers_mandatory(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>> {
    let date = get_str_from_headers(headers, header_name)?;
    utc_date_from_rfc2822(date)
}

pub fn utc_date_from_rfc2822(date: &str) -> Result<DateTime<Utc>> {
    let date = parse_date_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn continuation_token_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>> {
    Ok(get_option_str_from_headers(headers, CONTINUATION)?.map(String::from))
}

pub fn sku_name_from_headers(headers: &HeaderMap) -> Result<String> {
    Ok(get_str_from_headers(headers, SKU_NAME)?.to_owned())
}

pub fn account_kind_from_headers(headers: &HeaderMap) -> Result<String> {
    Ok(get_str_from_headers(headers, ACCOUNT_KIND)?.to_owned())
}

pub fn etag_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>> {
    Ok(get_option_str_from_headers(headers, ETAG.as_str())?.map(String::from))
}

pub fn etag_from_headers(headers: &HeaderMap) -> Result<String> {
    Ok(get_str_from_headers(headers, ETAG.as_str())?.to_owned())
}

pub fn lease_time_from_headers(headers: &HeaderMap) -> Result<u8> {
    get_from_headers(headers, LEASE_TIME)
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<bool> {
    get_from_headers(headers, DELETE_TYPE_PERMANENT)
}

#[cfg(feature = "azurite_workaround")]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<Option<bool>> {
    get_option_from_headers(headers, DELETE_TYPE_PERMANENT)
}

pub fn sequence_number_from_headers(headers: &HeaderMap) -> Result<u64> {
    get_from_headers(headers, BLOB_SEQUENCE_NUMBER)
}

pub fn session_token_from_headers(headers: &HeaderMap) -> Result<SessionToken> {
    get_str_from_headers(headers, SESSION_TOKEN).map(ToOwned::to_owned)
}

pub fn server_from_headers(headers: &HeaderMap) -> Result<&str> {
    get_str_from_headers(headers, SERVER.as_str())
}

pub fn version_from_headers(headers: &HeaderMap) -> Result<&str> {
    get_str_from_headers(headers, VERSION)
}

pub fn request_server_encrypted_from_headers(headers: &HeaderMap) -> Result<bool> {
    get_from_headers(headers, REQUEST_SERVER_ENCRYPTED)
}

pub fn content_type_from_headers(headers: &HeaderMap) -> Result<&str> {
    get_str_from_headers(headers, http::header::CONTENT_TYPE.as_str())
}

pub fn content_length_from_headers(headers: &HeaderMap) -> Result<i64> {
    get_from_headers(headers, CONTENT_LENGTH)
}

pub fn item_count_from_headers(headers: &HeaderMap) -> Result<u32> {
    get_from_headers(headers, ITEM_COUNT)
}
