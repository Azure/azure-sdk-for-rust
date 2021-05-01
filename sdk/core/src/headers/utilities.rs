use super::*;
use crate::request_options::LeaseId;
use crate::util::HeaderMapExt;
use crate::*;
use crate::{RequestId, SessionToken};
use chrono::{DateTime, Utc};
use http::header::{HeaderName, DATE, ETAG, LAST_MODIFIED};
#[cfg(feature = "enable_hyper")]
use http::status::StatusCode;
use http::HeaderMap;
#[cfg(feature = "enable_hyper")]
use hyper::{Body, Client, Request};
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

pub fn lease_id_from_headers(headers: &HeaderMap) -> Result<LeaseId, Error> {
    let lease_id = headers
        .get_as_str(LEASE_ID)
        .ok_or_else(|| Error::HeaderNotFound(LEASE_ID.to_owned()))?;
    Ok(LeaseId::from_str(lease_id)?)
}

pub fn request_id_from_headers(headers: &HeaderMap) -> Result<RequestId, Error> {
    let request_id = headers
        .get_as_str(REQUEST_ID)
        .ok_or_else(|| Error::HeaderNotFound(REQUEST_ID.to_owned()))?;
    Ok(Uuid::parse_str(request_id)?)
}

pub fn client_request_id_from_headers_optional(headers: &HeaderMap) -> Option<String> {
    headers.get_as_str(CLIENT_REQUEST_ID).map(|s| s.to_owned())
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
    type Error = Error;

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

pub fn last_modified_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<DateTime<Utc>>, Error> {
    if headers.contains_key(LAST_MODIFIED) {
        Ok(Some(last_modified_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn rfc2822_from_headers_mandatory(
    headers: &HeaderMap,
    header_name: &str,
) -> Result<DateTime<Utc>, Error> {
    let val = headers
        .get(header_name)
        .ok_or_else(|| Error::HeaderNotFound(header_name.to_owned()))?
        .to_str()?;
    let val = DateTime::parse_from_rfc2822(val)?;
    let val = DateTime::from_utc(val.naive_utc(), Utc);

    trace!("header {} == {:?}", header_name, val);
    Ok(val)
}

pub fn string_from_headers_mandatory<'a>(
    headers: &'a HeaderMap,
    header_name: &str,
) -> Result<&'a str, Error> {
    Ok(headers
        .get(header_name)
        .ok_or_else(|| Error::HeaderNotFound(header_name.to_owned()))?
        .to_str()?)
}

pub fn last_modified_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, Error> {
    rfc2822_from_headers_mandatory(headers, LAST_MODIFIED.as_str())
}

pub fn continuation_token_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<String>, Error> {
    if let Some(hc) = headers.get(CONTINUATION) {
        Ok(Some(hc.to_str()?.to_owned()))
    } else {
        Ok(None)
    }
}

#[inline]
pub fn utc_date_from_rfc2822(date: &str) -> Result<DateTime<Utc>, Error> {
    let date = DateTime::parse_from_rfc2822(date)?;
    Ok(DateTime::from_utc(date.naive_utc(), Utc))
}

pub fn date_from_headers(headers: &HeaderMap) -> Result<DateTime<Utc>, Error> {
    let date = headers
        .get(DATE)
        .ok_or_else(|| {
            static D: HeaderName = DATE;
            Error::HeaderNotFound(D.as_str().to_owned())
        })?
        .to_str()?;
    let date = DateTime::parse_from_rfc2822(date)?;
    let date = DateTime::from_utc(date.naive_utc(), Utc);

    trace!("date == {:?}", date);
    Ok(date)
}

pub fn sku_name_from_headers(headers: &HeaderMap) -> Result<String, Error> {
    let sku_name = headers
        .get(SKU_NAME)
        .ok_or_else(|| Error::HeaderNotFound(SKU_NAME.to_owned()))?
        .to_str()?;
    trace!("sku_name == {:?}", sku_name);
    Ok(sku_name.to_owned())
}

pub fn account_kind_from_headers(headers: &HeaderMap) -> Result<String, Error> {
    let account_kind = headers
        .get(ACCOUNT_KIND)
        .ok_or_else(|| Error::HeaderNotFound(ACCOUNT_KIND.to_owned()))?
        .to_str()?;
    trace!("account_kind == {:?}", account_kind);
    Ok(account_kind.to_owned())
}

pub fn etag_from_headers_optional(headers: &HeaderMap) -> Result<Option<String>, Error> {
    if headers.contains_key(ETAG) {
        Ok(Some(etag_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn etag_from_headers(headers: &HeaderMap) -> Result<String, Error> {
    let etag = headers
        .get(ETAG)
        .ok_or_else(|| {
            static E: HeaderName = ETAG;
            Error::HeaderNotFound(E.as_str().to_owned())
        })?
        .to_str()?
        .to_owned();

    trace!("etag == {:?}", etag);
    Ok(etag)
}

pub fn lease_time_from_headers(headers: &HeaderMap) -> Result<u8, Error> {
    let lease_time = headers
        .get(LEASE_TIME)
        .ok_or_else(|| Error::HeaderNotFound(LEASE_TIME.to_owned()))?
        .to_str()?;

    let lease_time = lease_time.parse::<u8>()?;

    trace!("lease_time == {:?}", lease_time);
    Ok(lease_time)
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<bool, Error> {
    let delete_type_permanent = headers
        .get(DELETE_TYPE_PERMANENT)
        .ok_or_else(|| Error::HeaderNotFound(DELETE_TYPE_PERMANENT.to_owned()))?
        .to_str()?;

    let delete_type_permanent = delete_type_permanent.parse::<bool>()?;

    trace!("delete_type_permanent == {:?}", delete_type_permanent);
    Ok(delete_type_permanent)
}

#[cfg(feature = "azurite_workaround")]
pub fn delete_type_permanent_from_headers(headers: &HeaderMap) -> Result<Option<bool>, Error> {
    let delete_type_permanent = headers
        .get(DELETE_TYPE_PERMANENT)
        .map(|delete_type_permanent| -> Result<_, Error> {
            Ok(delete_type_permanent.to_str()?.parse::<bool>()?)
        })
        .transpose()?;

    trace!("delete_type_permanent == {:?}", delete_type_permanent);
    Ok(delete_type_permanent)
}

pub fn sequence_number_from_headers(headers: &HeaderMap) -> Result<u64, Error> {
    let sequence_number = headers
        .get(BLOB_SEQUENCE_NUMBER)
        .ok_or_else(|| Error::HeaderNotFound(BLOB_SEQUENCE_NUMBER.to_owned()))?
        .to_str()?;

    let sequence_number = sequence_number.parse::<u64>()?;

    trace!("sequence_number == {:?}", sequence_number);
    Ok(sequence_number)
}

pub fn session_token_from_headers(headers: &HeaderMap) -> Result<SessionToken, Error> {
    Ok(headers
        .get(SESSION_TOKEN)
        .ok_or_else(|| Error::HeaderNotFound(SESSION_TOKEN.to_owned()))?
        .to_str()?
        .to_owned())
}

pub fn server_from_headers(headers: &HeaderMap) -> Result<&str, Error> {
    Ok(headers
        .get(SERVER)
        .ok_or_else(|| Error::HeaderNotFound(SERVER.to_owned()))?
        .to_str()?)
}

pub fn version_from_headers(headers: &HeaderMap) -> Result<&str, Error> {
    Ok(headers
        .get(VERSION)
        .ok_or_else(|| Error::HeaderNotFound(VERSION.to_owned()))?
        .to_str()?)
}

pub fn request_server_encrypted_from_headers(headers: &HeaderMap) -> Result<bool, Error> {
    let request_server_encrypted = headers
        .get(REQUEST_SERVER_ENCRYPTED)
        .ok_or_else(|| Error::HeaderNotFound(REQUEST_SERVER_ENCRYPTED.to_owned()))?
        .to_str()?;

    let request_server_encrypted = request_server_encrypted.parse::<bool>()?;

    trace!("request_server_encrypted == {:?}", request_server_encrypted);
    Ok(request_server_encrypted)
}

pub fn content_type_from_headers(headers: &HeaderMap) -> Result<&str, Error> {
    Ok(headers
        .get(http::header::CONTENT_TYPE)
        .ok_or_else(|| {
            let header = http::header::CONTENT_TYPE;
            Error::HeaderNotFound(header.as_str().to_owned())
        })?
        .to_str()?)
}

pub fn item_count_from_headers(headers: &HeaderMap) -> Result<u32, AzureError> {
    Ok(headers
        .get(crate::headers::MAX_ITEM_COUNT)
        .ok_or_else(|| AzureError::HeaderNotFound(crate::MAX_ITEM_COUNT.to_owned()))?
        .to_str()?
        .parse()?)
}

#[cfg(feature = "enable_hyper")]
pub async fn perform_http_request(
    client: &Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    req: Request<Body>,
    expected_status: StatusCode,
) -> Result<String, Error> {
    debug!("req == {:?}", req);
    let res = client
        .request(req)
        .await
        .map_err(HttpError::ExecuteRequestError)?;
    check_status_extract_body_2(res, expected_status).await
}
