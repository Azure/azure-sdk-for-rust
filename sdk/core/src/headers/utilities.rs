use super::*;
use crate::error::{Error, ErrorKind};
use crate::request_options::LeaseId;
use crate::{RequestId, SessionToken};
use chrono::{DateTime, FixedOffset, Utc};

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

pub fn lease_id_from_headers(headers: &Headers) -> crate::Result<LeaseId> {
    headers.get_as(&LEASE_ID)
}

pub fn request_id_from_headers(headers: &Headers) -> crate::Result<RequestId> {
    headers.get_as(&REQUEST_ID)
}

pub fn client_request_id_from_headers_optional(headers: &Headers) -> Option<String> {
    headers.get_optional_string(&CLIENT_REQUEST_ID)
}

pub fn last_modified_from_headers_optional(
    headers: &Headers,
) -> crate::Result<Option<DateTime<Utc>>> {
    headers.get_optional_as(&LAST_MODIFIED)
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
    let date = headers.get_str(header_name)?;
    utc_date_from_rfc2822(date)
}

pub fn utc_date_from_rfc2822(date: &str) -> crate::Result<DateTime<Utc>> {
    let date = parse_date_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn continuation_token_from_headers_optional(
    headers: &Headers,
) -> crate::Result<Option<String>> {
    Ok(headers.get_optional_string(&CONTINUATION))
}

pub fn sku_name_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_string(&SKU_NAME)
}

pub fn account_kind_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_string(&ACCOUNT_KIND)
}

pub fn etag_from_headers_optional(headers: &Headers) -> crate::Result<Option<String>> {
    Ok(headers.get_optional_string(&ETAG))
}

pub fn etag_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_string(&ETAG)
}

pub fn lease_time_from_headers(headers: &Headers) -> crate::Result<u8> {
    headers.get_as(&LEASE_TIME)
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn delete_type_permanent_from_headers(headers: &Headers) -> crate::Result<bool> {
    headers.get_as(&DELETE_TYPE_PERMANENT)
}

#[cfg(feature = "azurite_workaround")]
pub fn delete_type_permanent_from_headers(headers: &Headers) -> crate::Result<Option<bool>> {
    headers.get_optional_as(&DELETE_TYPE_PERMANENT)
}

pub fn sequence_number_from_headers(headers: &Headers) -> crate::Result<u64> {
    headers.get_as(&BLOB_SEQUENCE_NUMBER)
}

pub fn session_token_from_headers(headers: &Headers) -> crate::Result<SessionToken> {
    headers.get_string(&SESSION_TOKEN)
}

pub fn server_from_headers(headers: &Headers) -> crate::Result<&str> {
    headers.get_str(&SERVER)
}

pub fn version_from_headers(headers: &Headers) -> crate::Result<&str> {
    headers.get_str(&VERSION)
}

pub fn request_server_encrypted_from_headers(headers: &Headers) -> crate::Result<bool> {
    headers.get_as(&REQUEST_SERVER_ENCRYPTED)
}

pub fn content_type_from_headers(headers: &Headers) -> crate::Result<&str> {
    headers.get_str(&CONTENT_TYPE)
}

pub fn item_count_from_headers(headers: &Headers) -> crate::Result<u32> {
    headers.get_as(&ITEM_COUNT)
}
