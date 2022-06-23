use super::*;
use crate::error::{Error, ErrorKind};
use crate::request_options::LeaseId;
use crate::{RequestId, SessionToken};
use chrono::{DateTime, FixedOffset, Utc};
use std::str::FromStr;

pub fn get_option_str_from_headers<'a>(
    headers: &'a Headers,
    key: &HeaderName,
) -> crate::Result<Option<&'a str>> {
    let h = match headers.get(key) {
        Some(h) => h,
        None => return Ok(None),
    };
    Ok(Some(h.as_str()))
}

pub fn get_str_from_headers<'a>(headers: &'a Headers, key: &HeaderName) -> crate::Result<&'a str> {
    get_option_str_from_headers(headers, key)?.ok_or_else(|| {
        Error::with_message(ErrorKind::DataConversion, || {
            format!("could not find '{key:?}' in headers")
        })
    })
}

pub fn get_option_from_headers<T>(headers: &Headers, key: &HeaderName) -> crate::Result<Option<T>>
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
                "failed to parse header '{:?}' as {:?}",
                key,
                std::any::TypeId::of::<T>()
            ),
        )
    })?))
}

pub fn get_from_headers<T>(headers: &Headers, key: &HeaderName) -> crate::Result<T>
where
    T: std::str::FromStr + 'static,
    T::Err: std::error::Error + Send + Sync,
{
    get_str_from_headers(headers, key)?.parse().map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!(
                "failed to parse header '{:?}' as {:?}",
                key,
                std::any::TypeId::of::<T>()
            ),
        )
    })
}

pub fn parse_date_from_str(date: &str, fmt: &str) -> crate::Result<DateTime<FixedOffset>> {
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

pub fn parse_date_from_rfc2822(date: &str) -> crate::Result<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc2822(date).map_err(|e| {
        Error::full(
            ErrorKind::DataConversion,
            e,
            format!("failed to parse date '{}' with as rfc2822", date),
        )
    })
}

pub fn parse_int<F>(s: &str) -> crate::Result<F>
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

pub fn lease_id_from_headers(headers: &Headers) -> crate::Result<LeaseId> {
    get_from_headers(headers, &LEASE_ID)
}

pub fn request_id_from_headers(headers: &Headers) -> crate::Result<RequestId> {
    get_from_headers(headers, &REQUEST_ID)
}

pub fn client_request_id_from_headers_optional(headers: &Headers) -> Option<String> {
    get_option_from_headers(headers, &CLIENT_REQUEST_ID)
        .ok()
        .flatten()
}

pub fn last_modified_from_headers_optional(
    headers: &Headers,
) -> crate::Result<Option<DateTime<Utc>>> {
    get_option_from_headers(headers, &LAST_MODIFIED)
}

pub fn date_from_headers(headers: &Headers) -> crate::Result<DateTime<Utc>> {
    rfc2822_from_headers_mandatory(headers, &DATE)
}

pub fn last_modified_from_headers(headers: &Headers) -> crate::Result<DateTime<Utc>> {
    rfc2822_from_headers_mandatory(headers, &LAST_MODIFIED)
}

pub fn rfc2822_from_headers_mandatory(
    headers: &Headers,
    header_name: &HeaderName,
) -> crate::Result<DateTime<Utc>> {
    let date = get_str_from_headers(headers, header_name)?;
    utc_date_from_rfc2822(date)
}

pub fn utc_date_from_rfc2822(date: &str) -> crate::Result<DateTime<Utc>> {
    let date = parse_date_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn continuation_token_from_headers_optional(
    headers: &Headers,
) -> crate::Result<Option<String>> {
    Ok(get_option_str_from_headers(headers, &CONTINUATION)?.map(String::from))
}

pub fn sku_name_from_headers(headers: &Headers) -> crate::Result<String> {
    Ok(get_str_from_headers(headers, &SKU_NAME)?.to_owned())
}

pub fn account_kind_from_headers(headers: &Headers) -> crate::Result<String> {
    Ok(get_str_from_headers(headers, &ACCOUNT_KIND)?.to_owned())
}

pub fn etag_from_headers_optional(headers: &Headers) -> crate::Result<Option<String>> {
    Ok(get_option_str_from_headers(headers, &ETAG)?.map(String::from))
}

pub fn etag_from_headers(headers: &Headers) -> crate::Result<String> {
    Ok(get_str_from_headers(headers, &ETAG)?.to_owned())
}

pub fn lease_time_from_headers(headers: &Headers) -> crate::Result<u8> {
    get_from_headers(headers, &LEASE_TIME)
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn delete_type_permanent_from_headers(headers: &Headers) -> crate::Result<bool> {
    get_from_headers(headers, &DELETE_TYPE_PERMANENT)
}

#[cfg(feature = "azurite_workaround")]
pub fn delete_type_permanent_from_headers(headers: &Headers) -> crate::Result<Option<bool>> {
    get_option_from_headers(headers, &DELETE_TYPE_PERMANENT)
}

pub fn sequence_number_from_headers(headers: &Headers) -> crate::Result<u64> {
    get_from_headers(headers, &BLOB_SEQUENCE_NUMBER)
}

pub fn session_token_from_headers(headers: &Headers) -> crate::Result<SessionToken> {
    get_str_from_headers(headers, &SESSION_TOKEN).map(ToOwned::to_owned)
}

pub fn server_from_headers(headers: &Headers) -> crate::Result<&str> {
    get_str_from_headers(headers, &SERVER)
}

pub fn version_from_headers(headers: &Headers) -> crate::Result<&str> {
    get_str_from_headers(headers, &VERSION)
}

pub fn request_server_encrypted_from_headers(headers: &Headers) -> crate::Result<bool> {
    get_from_headers(headers, &REQUEST_SERVER_ENCRYPTED)
}

pub fn content_type_from_headers(headers: &Headers) -> crate::Result<&str> {
    get_str_from_headers(headers, &CONTENT_TYPE)
}

pub fn item_count_from_headers(headers: &Headers) -> crate::Result<u32> {
    get_from_headers(headers, &ITEM_COUNT)
}
