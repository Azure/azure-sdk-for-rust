use super::*;
use crate::prelude::Continuation;
use crate::request_options::LeaseId;
use crate::{date, RequestId, SessionToken};
use time::OffsetDateTime;

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
) -> crate::Result<Option<OffsetDateTime>> {
    headers
        .get_optional_str(&LAST_MODIFIED)
        .map(date::parse_rfc1123)
        .transpose()
}

pub fn date_from_headers(headers: &Headers) -> crate::Result<OffsetDateTime> {
    rfc1123_from_headers_mandatory(headers, &DATE)
}

pub fn last_modified_from_headers(headers: &Headers) -> crate::Result<OffsetDateTime> {
    rfc1123_from_headers_mandatory(headers, &LAST_MODIFIED)
}

pub fn rfc1123_from_headers_mandatory(
    headers: &Headers,
    header_name: &HeaderName,
) -> crate::Result<OffsetDateTime> {
    let date = headers.get_str(header_name)?;
    date::parse_rfc1123(date)
}

pub fn continuation_token_from_headers_optional(
    headers: &Headers,
) -> crate::Result<Option<Continuation>> {
    Ok(headers
        .get_optional_string(&CONTINUATION)
        .map(Continuation::from))
}

pub fn sku_name_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&SKU_NAME)
}

pub fn account_kind_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&ACCOUNT_KIND)
}

pub fn etag_from_headers_optional(headers: &Headers) -> crate::Result<Option<String>> {
    Ok(headers.get_optional_string(&ETAG))
}

pub fn etag_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&ETAG)
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
    headers.get_as(&SESSION_TOKEN)
}

pub fn server_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&SERVER)
}

pub fn version_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&VERSION)
}

pub fn request_server_encrypted_from_headers(headers: &Headers) -> crate::Result<bool> {
    headers.get_as(&REQUEST_SERVER_ENCRYPTED)
}

pub fn content_type_from_headers(headers: &Headers) -> crate::Result<String> {
    headers.get_as(&CONTENT_TYPE)
}

pub fn item_count_from_headers(headers: &Headers) -> crate::Result<u32> {
    headers.get_as(&ITEM_COUNT)
}
