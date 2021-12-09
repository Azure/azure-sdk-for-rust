use super::*;
use crate::request_options::LeaseId;
use crate::util::HeaderMapExt;
use crate::*;
use crate::{RequestId, SessionToken};
use chrono::{DateTime, FixedOffset, Utc};
use http::header::{DATE, ETAG, LAST_MODIFIED};
#[cfg(feature = "enable_hyper")]
use http::status::StatusCode;
use http::HeaderMap;
#[cfg(feature = "enable_hyper")]
use hyper::{Body, Client, Request};
use std::str::FromStr;

pub fn lease_id_from_headers(headers: &HeaderMap) -> Result<LeaseId> {
    get_from_headers(headers, LEASE_ID)
}

pub fn request_id_from_headers(headers: &HeaderMap) -> Result<RequestId> {
    get_from_headers(headers, REQUEST_ID)
}

pub fn client_request_id_from_headers_optional(headers: &HeaderMap) -> Option<String> {
    headers.get_as_str(CLIENT_REQUEST_ID).map(|s| s.to_owned())
}

pub fn last_modified_from_headers_optional(headers: &HeaderMap) -> Result<Option<DateTime<Utc>>> {
    get_option_from_headers(headers, LAST_MODIFIED.as_str())
}

pub fn rfc2822_from_headers_mandatory(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>> {
    let date = get_str_from_headers(headers, header_name)?;
    let date = parse_date_from_rfc2822(date)?;
    let date = DateTime::from_utc(date.naive_utc(), Utc);
    Ok(date)
}

pub fn last_modified_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>> {
    rfc2822_from_headers_mandatory(headers, LAST_MODIFIED.as_str())
}

pub fn continuation_token_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>> {
    if let Some(hc) = headers.get(CONTINUATION) {
        Ok(Some(hc.to_str()?.to_owned()))
    } else {
        Ok(None)
    }
}

pub fn utc_date_from_rfc2822(date: &str) -> Result<DateTime<Utc>> {
    let date = parse_date_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>> {
    let date = get_str_from_headers(headers, DATE.as_str())?;
    let date = parse_date_from_rfc2822(date)?;
    let date = DateTime::from_utc(date.naive_utc(), Utc);
    Ok(date)
}

pub fn sku_name_from_headers(headers: &HeaderMap) -> Result<String> {
    let sku_name = get_str_from_headers(headers, SKU_NAME)?;
    Ok(sku_name.to_owned())
}

pub fn account_kind_from_headers(headers: &HeaderMap) -> Result<String> {
    let account_kind = get_str_from_headers(headers, ACCOUNT_KIND)?;
    Ok(account_kind.to_owned())
}

pub fn etag_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>> {
    if headers.contains_key(ETAG) {
        Ok(Some(etag_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn etag_from_headers(headers: &HeaderMap) -> Result<String> {
    get_str_from_headers(headers, ETAG.as_str()).map(ToOwned::to_owned)
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
    get_str_from_headers(headers, SERVER)
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

pub fn item_count_from_headers(headers: &HeaderMap) -> Result<u32> {
    get_from_headers(headers, ITEM_COUNT)
}

#[cfg(feature = "enable_hyper")]
pub async fn perform_http_request(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    req: Request<Body>,
    expected_status: StatusCode,
) -> Result<String> {
    debug!("req == {:?}", req);
    let res = client
        .request(req)
        .await
        .map_err(HttpError::ExecuteRequestError)?;
    check_status_extract_body_2(res, expected_status).await
}

pub fn get_str_from_headers<'a>(headers: &'a HeaderMap, key: &str) -> Result<&'a str> {
    Ok(headers
        .get(key)
        .ok_or_else(|| Error::HeaderNotFound(key.to_owned()))?
        .to_str()?)
}

pub fn get_from_headers<T: std::str::FromStr>(headers: &HeaderMap, key: &str) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: Into<ParsingError>,
{
    get_str_from_headers(headers, key)?
        .parse()
        .map_err(|e: T::Err| Error::ParsingError(e.into()))
}

pub fn get_option_from_headers<T>(headers: &HeaderMap, key: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: Into<ParsingError>,
{
    match headers.get(key) {
        Some(header) => Ok(Some(
            header
                .to_str()?
                .parse()
                .map_err(|e: T::Err| Error::ParsingError(e.into()))?,
        )),
        None => Ok(None),
    }
}

pub fn parse_date_from_str(
    date: &str,
    fmt: &str,
) -> std::result::Result<DateTime<FixedOffset>, ParsingError> {
    DateTime::parse_from_str(date, fmt).map_err(ParsingError::ParseDateTimeError)
}

pub fn parse_date_from_rfc2822(
    date: &str,
) -> std::result::Result<DateTime<FixedOffset>, ParsingError> {
    DateTime::parse_from_rfc2822(date).map_err(ParsingError::ParseDateTimeError)
}

pub fn parse_int<F>(s: &str) -> std::result::Result<F, ParsingError>
where
    F: FromStr<Err = std::num::ParseIntError>,
{
    FromStr::from_str(s).map_err(ParsingError::ParseIntError)
}
