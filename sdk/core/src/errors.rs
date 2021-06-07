use http::StatusCode;
#[cfg(feature = "enable_hyper")]
use hyper::{self, body, Body};
#[cfg(feature = "enable_hyper")]
type HttpClientError = hyper::Error;
#[cfg(feature = "enable_reqwest")]
type HttpClientError = reqwest::Error;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Element not found: {0}")]
    ElementNotFound(String),
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("Expected token \"{}\" not found", 0)]
    TokenNotFound(String),
    #[error("Expected split char \'{}\' not found", 0)]
    SplitNotFound(char),
    #[error("Parse int error {0}")]
    ParseIntError(std::num::ParseIntError),
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum AzurePathParseError {
    #[error("Path separator not found")]
    PathSeparatorNotFoundError,
    #[error("Multiple path separators found")]
    MultiplePathSeparatorsFoundError,
    #[error("Missing container name")]
    MissingContainerError,
    #[error("Missing blob name")]
    MissingBlobError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedValue {
    expected: Vec<String>,
    received: String,
}

impl UnexpectedValue {
    pub fn new(expected: String, received: String) -> UnexpectedValue {
        Self {
            expected: vec![expected],
            received,
        }
    }

    pub fn new_multiple(allowed: Vec<String>, received: String) -> Self {
        UnexpectedValue {
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
    #[error("Stream poll error: {0}")]
    PollError(std::io::Error),
    #[error("Stream read error: {0}")]
    ReadError(HttpClientError),
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Failed to serialize request body as json: {0}")]
    BodySerializationError(serde_json::Error),
    #[error(
        "Unexpected HTTP result (expected: {:?}, received: {:?}, body: {:?})",
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
    #[error("From UTF8 conversion error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Failed to build request: {0}")]
    BuildRequestError(http::Error),
    #[error("Failed to build request: {0}")]
    BuildClientRequestError(HttpClientError),
    #[error("Failed to execute request: {0}")]
    ExecuteRequestError(HttpClientError),
    #[error("Failed to read response as bytes: {0}")]
    ReadBytesError(HttpClientError),
    #[error("Failed to read response as stream: {0}")]
    ReadStreamError(HttpClientError),
    #[error("Failed to build response: {0}")]
    BuildResponseError(http::Error),
    #[error("to str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("Failed to reset stream: {0}")]
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
#[derive(Debug, thiserror::Error)]
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

#[derive(Debug, thiserror::Error)]
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
pub enum Error {
    #[error("Error getting token: {0}")]
    GetTokenError(Box<dyn std::error::Error + Send + Sync>),
    #[error("http error: {0}")]
    HttpError(#[from] HttpError),
    #[error("parse bool error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("to str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("Header not found: {0}")]
    HeaderNotFound(String),
    #[error("At least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error(
        "The expected query parameter {} was not found in the provided Url: {:?}",
        expected_parameter,
        url
    )]
    UrlQueryParameterNotFound {
        expected_parameter: String,
        url: url::Url,
    },
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Error preparing HTTP request: {0}")]
    HttpPrepareError(#[from] http::Error),
    #[error("uuid error: {0}")]
    ParseUuidError(#[from] uuid::Error),
    #[error("Chrono parser error: {0}")]
    ChronoParserError(#[from] chrono::ParseError),
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
    #[error("Input string cannot be converted in boolean: {0}")]
    BooleanNotMatched(String),
    #[error("Unexpected node type received: expected {0}")]
    UnexpectedNodeTypeError(String),
    #[error("DateTime parse error: {0}")]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("Text not found")]
    TextNotFound,
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Generic parse error: {0}")]
    GenericParseError(String),
    #[error("Parsing error: {:?}", 0)]
    ParsingError(#[from] ParsingError),
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
