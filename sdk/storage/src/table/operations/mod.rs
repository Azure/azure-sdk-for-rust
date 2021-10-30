pub mod entity;
pub mod table;

use azure_core::{AppendToUrlQuery, HTTPHeaderError};
use chrono::{DateTime, Utc};
use http::HeaderValue;

/// api version enum
#[derive(Debug, Clone)]
pub enum ApiVersion {
    Version2019_12_12,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::Version2019_12_12
    }
}

impl AsRef<str> for ApiVersion {
    fn as_ref(&self) -> &str {
        match self {
            ApiVersion::Version2019_12_12 => "2019-12-12",
        }
    }
}

/// open data protocol response details level
#[derive(Debug, Clone)]
pub enum OdataMetadataLevel {
    NoMetadata,
    MinimalMetadata,
    FullMetadata,
}

impl AsRef<str> for OdataMetadataLevel {
    fn as_ref(&self) -> &str {
        match self {
            OdataMetadataLevel::NoMetadata => "application/json;odata=nometadata",
            OdataMetadataLevel::MinimalMetadata => "application/json;odata=minimalmetadata",
            OdataMetadataLevel::FullMetadata => "application/json;odata=fullmetadata",
        }
    }
}

/// Sets if the resource should be included in the response.
/// * NO_CONTENT - the response body will be empty and the status code will be 204.
/// * NO_CONTENT - the response body will contain the create table with the specified metadata details and the status code will be 201.
#[derive(Debug, Clone)]
pub enum EchoContent {
    ReturnNoContent,
    ReturnContent,
}

impl AsRef<str> for EchoContent {
    fn as_ref(&self) -> &str {
        match self {
            EchoContent::ReturnNoContent => "return-no-content",
            EchoContent::ReturnContent => "return-content",
        }
    }
}

///The client may specify the ETag for the entity on the request in order to compare to the ETag maintained by the service
/// for the purpose of optimistic concurrency.
pub enum ETag {
    ForceUpdate,
    OnlyIfMatch(String),
}

impl Default for ETag {
    fn default() -> Self {
        Self::ForceUpdate
    }
}

impl AsRef<str> for ETag {
    fn as_ref(&self) -> &str {
        match self {
            ETag::ForceUpdate => "*",
            ETag::OnlyIfMatch(etag) => etag.as_str(),
        }
    }
}

/// A call to a Table service API can include a server timeout interval.
/// If the server timeout interval elapses before the service has finished processing the request, the service returns an error.
///
/// The maximum timeout interval for Table service operations is 30 seconds. The Table service automatically reduces any timeouts larger than 30 seconds to the 30-second maximum.
/// The Table service enforces server timeouts as follows:
/// * Insert, update, and delete operations: The maximum timeout interval is 30 seconds. Thirty seconds is also the default interval for all insert, update, and delete operations.
/// * Query operations: During the timeout interval, a query may execute for up to a maximum of five seconds. If the query does not complete within the five-second interval, the response includes continuation tokens for retrieving remaining items on a subsequent request.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timeout(u8);

impl Timeout {
    ///The timeout parameter is expressed in seconds.
    pub fn new(sec: u8) -> Self {
        Self(sec)
    }
}

impl AppendToUrlQuery for Timeout {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("timeout", &self.0.to_string());
    }
}

impl From<u8> for Timeout {
    fn from(sec: u8) -> Self {
        Self::new(sec)
    }
}

pub fn header_time_value(utc_time: DateTime<Utc>) -> Result<HeaderValue, HTTPHeaderError> {
    const FMT: &str = "%a, %d %h %Y %T GMT";
    HeaderValue::from_str(utc_time.format(FMT).to_string().as_str())
        .map_err(azure_core::HTTPHeaderError::InvalidHeaderValue)
}

pub fn header_value<T: AsRef<str>>(value: &Option<T>) -> Result<HeaderValue, HTTPHeaderError> {
    HeaderValue::from_str(
        value
            .as_ref()
            .ok_or_else(|| {
                azure_core::HTTPHeaderError::HeaderValidationError("header value was none".into())
            })?
            .as_ref(),
    )
    .map_err(azure_core::HTTPHeaderError::InvalidHeaderValue)
}
