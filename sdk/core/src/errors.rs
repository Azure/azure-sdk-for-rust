use http::StatusCode;
#[cfg(feature = "enable_hyper")]
use hyper::{self, body, Body};
use std::cmp::PartialEq;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("invalid pipeline: last policy is not a TransportPolicy: {0:?}")]
    InvalidTailPolicy(String),
}

#[derive(Debug, thiserror::Error)]
pub enum HTTPHeaderError {
    #[error("{0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("{0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pipeline error: {0}")]
    PipelineError(#[from] PipelineError),
    #[error("policy error: {0}")]
    PolicyError(Box<dyn std::error::Error + Send + Sync>),
    #[error("parsing error: {0}")]
    ParsingError(#[from] ParsingError),
    #[error("error getting token: {0}")]
    GetTokenError(Box<dyn std::error::Error + Send + Sync>),
    #[error("http error: {0}")]
    HttpError(#[from] HttpError),
    #[error("to str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("header not found: {0}")]
    HeaderNotFound(String),
    #[error("at least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error(
        "the expected query parameter {} was not found in the provided Url: {:?}",
        expected_parameter,
        url
    )]
    UrlQueryParameterNotFound {
        expected_parameter: String,
        url: url::Url,
    },
    #[error("error preparing HTTP request: {0}")]
    HttpPrepareError(#[from] http::Error),
    #[error(transparent)]
    StreamError(#[from] StreamError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[cfg(feature = "enable_hyper")]
type HttpClientError = hyper::Error;
#[cfg(feature = "enable_reqwest")]
type HttpClientError = reqwest::Error;

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

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedHTTPResult {
    expected: Vec<StatusCode>,
    received: StatusCode,
    body: String,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("error polling stream: {0}")]
    PollError(std::io::Error),
    #[error("error reading stream: {0}")]
    ReadError(HttpClientError),
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Failed to serialize request body as json: {0}")]
    BodySerializationError(serde_json::Error),
    #[error(
        "unexpected HTTP result (expected: {:?}, received: {:?}, body: {:?})",
        expected,
        received,
        body
    )]
    UnexpectedStatusCode {
        expected: Vec<StatusCode>,
        received: StatusCode,
        body: String,
    },
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

impl HttpError {
    pub fn new_unexpected_status_code(
        expected: StatusCode,
        received: StatusCode,
        body: &str,
    ) -> HttpError {
        HttpError::UnexpectedStatusCode {
            expected: vec![expected],
            received,
            body: body.to_owned(),
        }
    }

    pub fn new_multiple_unexpected_status_code(
        allowed: Vec<StatusCode>,
        received: StatusCode,
        body: &str,
    ) -> HttpError {
        HttpError::UnexpectedStatusCode {
            expected: allowed,
            received,
            body: body.to_owned(),
        }
    }
}
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Not512ByteAlignedError {
    #[error("start range not 512-byte aligned: {0}")]
    StartRange(u64),
    #[error("end range not 512-byte aligned: {0}")]
    EndRange(u64),
}

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

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Parse512AlignedError {
    #[error("split not found")]
    SplitNotFound,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("not 512 byte aligned error: {0}")]
    Not512ByteAlignedError(#[from] Not512ByteAlignedError),
}

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

#[cfg(feature = "mock_transport_framework")]
#[derive(Debug, thiserror::Error)]
pub enum MockFrameworkError {
    #[error("the mock testing framework has not been initialized")]
    UninitializedTransaction(),
    #[error("{0}: {1}")]
    IOError(&'static str, std::io::Error),
    #[error("received request have header {0} but it was not present in the read request")]
    MissingRequestHeader(String),
    #[error("different number of headers in request. Recevied: {0}, Read: {1}")]
    MismatchedRequestHeadersCount(usize, usize),
    #[error("request header {0} value is different. Received: {1}, Read: {2}")]
    MismatchedRequestHeader(String, String, String),
    #[error("mismatched HTTP request method. Received: {0}, Read: {1}")]
    MismatchedRequestHTTPMethod(http::Method, http::Method),
    #[error("mismatched request body. Received: {0:?}, Read: {1:?}")]
    MismatchedRequestBody(Vec<u8>, Vec<u8>),
}

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

    fn error_generator() -> Result<(), Error> {
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
