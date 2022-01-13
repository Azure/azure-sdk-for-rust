use http::StatusCode;
#[cfg(feature = "enable_hyper")]
use hyper::{self, body, Body};
use std::cmp::PartialEq;

/// A specialized `Result` type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error originating from a pipeline.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("invalid pipeline: last policy is not a TransportPolicy: {0:?}")]
    InvalidTailPolicy(String),
}

/// An error caused by an HTTP header.
#[derive(Debug, thiserror::Error)]
pub enum HttpHeaderError {
    #[error("{0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("{0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
}

/// A general Azure error type.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pipeline error: {0}")]
    Pipeline(#[from] PipelineError),
    #[error("policy error: {0}")]
    Policy(Box<dyn std::error::Error + Send + Sync>),
    #[error("parsing error: {0}")]
    Parsing(#[from] ParsingError),
    #[error("error getting token: {0}")]
    GetToken(Box<dyn std::error::Error + Send + Sync>),
    #[error("http error: {0}")]
    Http(#[from] HttpError),
    #[error("to str error: {0}")]
    ToStr(#[from] http::header::ToStrError),
    #[error("header error: {0}")]
    Header(#[from] HttpHeaderError),
    #[error("header not found: {0}")]
    HeaderNotFound(String),
    #[error("at least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error("error preparing HTTP request: {0}")]
    HttpPrepare(#[from] http::Error),
    #[error(transparent)]
    Stream(#[from] StreamError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[cfg(feature = "enable_hyper")]
type HttpClientError = hyper::Error;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
type HttpClientError = reqwest::Error;

/// An error caused by a failure to parse data.
#[non_exhaustive]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParsingError {
    #[error("unknown variant of {item} found: \"{variant}\"")]
    UnknownVariant { item: &'static str, variant: String },
    #[error("expected token \"{token}\" not found when parsing {item} from \"{full}\"")]
    TokenNotFound {
        item: &'static str,
        token: String,
        full: String,
    },
    #[error("error parsing int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("error parsing uuid: {0}")]
    ParseUuidError(#[from] uuid::Error),
    #[error("error parsing date time: {0}")]
    ParseDateTimeError(#[from] chrono::ParseError),
    #[error("error parsing a float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("error parsing bool: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
}

/// An unexpected value.
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedValue {
    expected: Vec<String>,
    received: String,
}

impl UnexpectedValue {
    pub fn new(expected: String, received: String) -> Self {
        Self {
            expected: vec![expected],
            received,
        }
    }

    pub fn new_multiple(allowed: Vec<String>, received: String) -> Self {
        Self {
            expected: allowed,
            received,
        }
    }
}

/// An error originating from a streaming response.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("error polling stream: {0}")]
    PollError(std::io::Error),
    #[error("error reading stream: {0}")]
    ReadError(HttpClientError),
}

/// An error originating from an HTTP client.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Failed to serialize request body as json: {0}")]
    BodySerializationError(serde_json::Error),
    #[error("HTTP error status (status: {:?}, body: {:?})", status, body)]
    ErrorStatusCode { status: StatusCode, body: String },
    #[error("UTF8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("from UTF8 conversion error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("failed to build request: {0}")]
    BuildRequestError(http::Error),
    #[error("failed to build request: {0}")]
    BuildClientRequestError(HttpClientError),
    #[error("failed to execute request: {0}")]
    ExecuteRequestError(HttpClientError),
    #[error("failed to read response as bytes: {0}")]
    ReadBytesError(HttpClientError),
    #[error("failed to read response as stream: {0}")]
    ReadStreamError(HttpClientError),
    #[error("failed to build response: {0}")]
    BuildResponseError(http::Error),
    #[error("to str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("failed to reset stream: {0}")]
    StreamResetError(StreamError),
}

/// An error caused by invalid permissions.
#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("Permission token not supported in this service ({}). Received token {}, supported tokens {:?}",
        service, received_token, supported_tokens)]
    NonSupportedToken {
        service: String,
        received_token: char,
        supported_tokens: Vec<char>,
    },
}

/// An error caused by failure to traverse a data structure.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TraversingError {
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Multiple node: {0}")]
    MultipleNode(String),
    #[error("Enumeration not matched: {0}")]
    EnumerationNotMatched(String),
    #[error("input string cannot be converted in boolean: {0}")]
    BooleanNotMatched(String),
    #[error("unexpected node type received: expected {0}")]
    UnexpectedNodeTypeError(String),
    #[error("DateTime parse error: {0}")]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("text not found")]
    TextNotFound,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("generic parse error: {0}")]
    GenericParseError(String),
    #[error("parsing error: {0:?}")]
    ParsingError(#[from] ParsingError),
}

/// Extract the headers and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_headers_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(hyper::StatusCode, hyper::HeaderMap, body::Bytes), Error> {
    let res = resp.await.map_err(HttpError::ExecuteRequestError)?;
    let (head, body) = res.into_parts();
    let status = head.status;
    let headers = head.headers;
    let body = body::to_bytes(body)
        .await
        .map_err(HttpError::ReadBytesError)?;
    Ok((status, headers, body))
}

/// Extract the status and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(StatusCode, String), HttpError> {
    let res = resp.await.map_err(HttpError::ExecuteRequestError)?;
    let status = res.status();
    let body = body::to_bytes(res.into_body())
        .await
        .map_err(HttpError::ReadBytesError)?;
    Ok((status, std::str::from_utf8(&body)?.to_owned()))
}

/// Extract the `Location` header, status and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_location_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(http::StatusCode, String, String), HttpError> {
    let res = resp.await.map_err(HttpError::ExecuteRequestError)?;
    let status = res.status();
    let location: String = match res.headers().get("Location") {
        Some(header_value) => header_value.to_str()?.to_owned(),
        _ => "".to_owned(),
    };
    let body = body::to_bytes(res.into_body())
        .await
        .map_err(HttpError::ReadBytesError)?;
    Ok((status, location, std::str::from_utf8(&body)?.to_owned()))
}

/// Extract the HTTP body from a `hyper` HTTP response, and check the response
/// status is 200.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn check_status_extract_body(
    resp: hyper::client::ResponseFuture,
    expected_status_code: hyper::StatusCode,
) -> Result<String, Error> {
    let (status, body) = extract_status_and_body(resp).await?;
    if status == expected_status_code {
        Ok(body)
    } else {
        Err(HttpError::new_unexpected_status_code(expected_status_code, status, &body).into())
    }
}

/// Extract the HTTP body from a `hyper` HTTP response, and check the response
/// status is expected.
#[cfg(feature = "enable_hyper")]
pub async fn check_status_extract_body_2(
    resp: hyper::Response<Body>,
    expected_status: StatusCode,
) -> Result<String, Error> {
    let received_status = resp.status();
    let body = body::to_bytes(resp.into_body())
        .await
        .map_err(HttpError::ReadBytesError)?;
    let s = String::from_utf8(body.to_vec())?;
    debug!("body: {}", s);
    if received_status != expected_status {
        Err(HttpError::new_unexpected_status_code(expected_status, received_status, &s).into())
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn send_fn<T>(_t: T)
    where
        T: Send,
    {
    }

    fn sync_fn<T>(_t: T)
    where
        T: Sync,
    {
    }

    fn error_generator() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_azure_error_send() {
        error_generator().map_err(send_fn).unwrap();
    }

    #[test]
    fn test_azure_error_sync() {
        error_generator().map_err(sync_fn).unwrap();
    }
}
