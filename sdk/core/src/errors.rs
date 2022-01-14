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
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("invalid header name")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error("to str error")]
    ToStr(#[source] http::header::ToStrError),
}

/// A general Azure error type.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pipeline error")]
    Pipeline(#[from] PipelineError),
    #[error("policy error")]
    Policy(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("parse error")]
    Parse(#[from] ParseError),
    #[error("error getting token")]
    GetToken(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("http error")]
    Http(#[from] HttpError),
    #[error("header error")]
    Header(#[from] HttpHeaderError),
    #[error("header not found: {0}")]
    HeaderNotFound(String),
    #[error("at least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error("error preparing HTTP request")]
    HttpPrepare(#[source] http::Error),
    #[error(transparent)]
    Stream(#[from] StreamError),
    #[error("Other error")]
    Other(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[cfg(feature = "enable_hyper")]
type HttpClientError = hyper::Error;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
type HttpClientError = reqwest::Error;

/// An error caused by a failure to parse data.
#[non_exhaustive]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("unknown variant of {item} found: \"{variant}\"")]
    UnknownVariant { item: &'static str, variant: String },
    #[error("expected token \"{token}\" not found when parsing {item} from \"{full}\"")]
    TokenNotFound {
        item: &'static str,
        token: String,
        full: String,
    },
    #[error("error parsing int")]
    Int(#[from] std::num::ParseIntError),
    #[error("error parsing uuid")]
    Uuid(#[from] uuid::Error),
    #[error("error parsing date time")]
    DateTime(#[from] chrono::ParseError),
    #[error("error parsing a float")]
    Float(#[from] std::num::ParseFloatError),
    #[error("error parsing bool")]
    Bool(#[from] std::str::ParseBoolError),
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
    Poll(std::io::Error),
    #[error("error reading stream: {0}")]
    Read(HttpClientError),
}

/// An error originating from an HTTP client.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("HTTP error status (status: {:?}, body: {:?})", status, body)]
    StatusCode { status: StatusCode, body: String },
    #[error("UTF8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("failed to build request")]
    BuildClientRequest(#[source] HttpClientError),
    #[error("failed to execute request")]
    ExecuteRequest(#[source] HttpClientError),
    #[error("failed to read response as bytes")]
    ReadBytes(#[source] HttpClientError),
    #[error("failed to build response")]
    BuildResponse(#[source] http::Error),
    #[error("failed to reset stream")]
    StreamReset(#[source] StreamError),
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
    #[error("DateTime parse error")]
    DateTimeParse(#[from] chrono::format::ParseError),
    #[error("text not found")]
    TextNotFound,
    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse error")]
    Parse(#[from] ParseError),
}

/// Extract the headers and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_headers_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(hyper::StatusCode, hyper::HeaderMap, body::Bytes), Error> {
    let res = resp.await.map_err(HttpError::ExecuteRequest)?;
    let (head, body) = res.into_parts();
    let status = head.status;
    let headers = head.headers;
    let body = body::to_bytes(body).await.map_err(HttpError::ReadBytes)?;
    Ok((status, headers, body))
}

/// Extract the status and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(StatusCode, String), HttpError> {
    let res = resp.await.map_err(HttpError::ExecuteRequest)?;
    let status = res.status();
    let body = body::to_bytes(res.into_body())
        .await
        .map_err(HttpError::ReadBytes)?;
    Ok((status, std::str::from_utf8(&body)?.to_owned()))
}

/// Extract the `Location` header, status and body from a `hyper` HTTP response.
#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_location_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(http::StatusCode, String, String), HttpError> {
    let res = resp.await.map_err(HttpError::ExecuteRequest)?;
    let status = res.status();
    let location: String = match res.headers().get("Location") {
        Some(header_value) => header_value.to_str()?.to_owned(),
        _ => "".to_owned(),
    };
    let body = body::to_bytes(res.into_body())
        .await
        .map_err(HttpError::ReadBytes)?;
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
        .map_err(HttpError::ReadBytes)?;
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
