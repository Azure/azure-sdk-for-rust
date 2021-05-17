use http::header::ToStrError;
use http::StatusCode;
#[cfg(feature = "enable_hyper")]
use hyper::{self, body, Body};
use std::io::Error as IOError;
use std::num;
use std::num::ParseIntError;
use std::str;
use std::str::ParseBoolError;
use std::string;
use url::ParseError as URLParseError;

#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Element not found: {}", 0)]
    ElementNotFound(String),
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("Expected token {} not found", 0)]
    TokenNotFound(String),
    #[error("Split {} not found", 0)]
    SplitNotFound(char),
    #[error("Parse int error {}", 0)]
    ParseIntError(ParseIntError),
}

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

impl From<UnexpectedHTTPResult> for AzureError {
    fn from(result: UnexpectedHTTPResult) -> AzureError {
        AzureError::UnexpectedHTTPResult(result)
    }
}

impl UnexpectedHTTPResult {
    pub fn new(expected: StatusCode, received: StatusCode, body: &str) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: vec![expected],
            received,
            body: body.to_owned(),
        }
    }

    pub fn new_multiple(
        allowed: Vec<StatusCode>,
        received: StatusCode,
        body: &str,
    ) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: allowed,
            received,
            body: body.to_owned(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.received
    }
}

impl std::fmt::Display for UnexpectedHTTPResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected HTTP result (expected: {:?}, received: {:?})",
            self.expected, self.received
        )
    }
}

impl std::error::Error for UnexpectedHTTPResult {
    fn description(&self) -> &str {
        "Unexpected HTTP result"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Not512ByteAlignedError {
    #[error("start range not 512-byte aligned: {}", 0)]
    StartRange(u64),
    #[error("end range not 512-byte aligned: {}", 0)]
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
    #[error("parse int error: {}", 0)]
    ParseIntError(#[from] ParseIntError),
    #[error("not 512 byte aligned error: {}", 0)]
    Not512ByteAlignedError(#[from] Not512ByteAlignedError),
}

#[derive(Debug, thiserror::Error)]
pub enum AzureError {
    #[error("{}-{} is not 512 byte aligned", start, end)]
    PageNot512ByteAlignedError { start: u64, end: u64 },
    #[error("{} is not 512 byte aligned", size)]
    Not512ByteAlignedError { size: u64 },
    #[error("Operation not supported. Operation == {}, reason == {}", 0, 1)]
    OperationNotSupported(String, String),
    #[error("base64 decode error: {}", 0)]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("digest length {} bytes instead of 16", 0)]
    DigestNot16BytesLong(u64),
    #[error("CRC64 length {} bytes instead of 8", 0)]
    CRC64Not8BytesLong(u64),
    #[error("parse bool error: {}", 0)]
    ParseBoolError(#[from] ParseBoolError),
    #[error("to str error: {}", 0)]
    ToStrError(#[from] ToStrError),
    #[error("json error: {}", 0)]
    JSONError(#[from] serde_json::Error),
    #[error("Hyper error: {}", 0)]
    HyperError(#[from] Box<dyn std::error::Error + Sync + Send>),
    #[error("Permission error: {}", 0)]
    PermissionError(#[from] PermissionError),
    #[error("IO error: {}", 0)]
    IOError(#[from] IOError),
    #[error("UnexpectedXMLError: {}", 0)]
    UnexpectedXMLError(String),
    #[error("Azure Path parse error: {}", 0)]
    AzurePathParseError(#[from] AzurePathParseError),
    #[error("UnexpectedHTTPResult error: {}", 0)]
    UnexpectedHTTPResult(UnexpectedHTTPResult),
    #[error("UnexpectedValue error: {:?}", 0)]
    UnexpectedValue(UnexpectedValue),
    #[error("Header not found: {}", 0)]
    HeaderNotFound(String),
    #[error("At least one of these headers must be present: {:?}", 0)]
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
    #[error("Traversing error: {}", 0)]
    ResponseParsingError(#[from] TraversingError),
    #[error("Parse int error: {}", 0)]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Parse float error: {}", 0)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Parse error: {}", 0)]
    ParseError(#[from] ParseError),
    #[error("Generic error")]
    GenericError,
    #[error("Generic error: {}", 0)]
    GenericErrorWithText(String),
    #[error("Parsing error: {}", 0)]
    ParsingError(#[from] ParsingError),
    #[error("Input parameters error: {}", 0)]
    InputParametersError(String),
    #[error("URL parse error: {}", 0)]
    URLParseError(#[from] URLParseError),
    #[error("Error preparing HTTP request: {}", 0)]
    HttpPrepareError(#[from] http::Error),
    #[error("uuid error: {}", 0)]
    ParseUuidError(#[from] uuid::Error),
    #[error("Chrono parser error: {}", 0)]
    ChronoParserError(#[from] chrono::ParseError),
    #[error("UTF8 conversion error: {}", 0)]
    UTF8Error(#[from] str::Utf8Error),
    #[error("FromUTF8 error: {}", 0)]
    FromUtf8Error(#[from] string::FromUtf8Error),
    #[error("A required header is missing: {}", 0)]
    MissingHeaderError(String),
    #[error(
        "An expected JSON node is missing: {} of expected type {}",
        value,
        expected_type
    )]
    MissingValueError {
        value: String,
        expected_type: String,
    },
    #[error("Invalid status code: {:?}", 0)]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),
    #[error("Error parsing the transaction response: {:?}", 0)]
    TransactionResponseParseError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum TraversingError {
    #[error("Path not found: {}", 0)]
    PathNotFound(String),
    #[error("Multiple node: {}", 0)]
    MultipleNode(String),
    #[error("Enumeration not matched: {}", 0)]
    EnumerationNotMatched(String),
    #[error("Input string cannot be converted in boolean: {}", 0)]
    BooleanNotMatched(String),
    #[error("Unexpected node type received: expected {}", 0)]
    UnexpectedNodeTypeError(String),
    #[error("DateTime parse error: {}", 0)]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("Text not found")]
    TextNotFound,
    #[error("Parse int error: {}", 0)]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Generic parse error: {}", 0)]
    GenericParseError(String),
    #[error("Parsing error: {:?}", 0)]
    ParsingError(#[from] ParsingError),
}

impl From<()> for AzureError {
    fn from(_: ()) -> AzureError {
        AzureError::GenericError
    }
}

#[cfg(feature = "enable_hyper")]
impl From<hyper::Error> for AzureError {
    fn from(error: hyper::Error) -> AzureError {
        AzureError::HyperError(error.into())
    }
}

#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_headers_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(hyper::StatusCode, hyper::HeaderMap, body::Bytes), AzureError> {
    let res = resp.await?;
    let (head, body) = res.into_parts();
    let status = head.status;
    let headers = head.headers;
    let body = body::to_bytes(body).await?;

    Ok((status, headers, body))
}

#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(StatusCode, String), AzureError> {
    let res = resp.await?;
    let status = res.status();
    let body = body::to_bytes(res.into_body()).await?;
    Ok((status, str::from_utf8(&body)?.to_owned()))
}

#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn extract_location_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(http::StatusCode, String, String), AzureError> {
    let res = resp.await?;
    let status = res.status();
    let location: String = match res.headers().get("Location") {
        Some(header_value) => header_value.to_str()?.to_owned(),
        _ => "".to_owned(),
    };
    let body = body::to_bytes(res.into_body()).await?;
    Ok((status, location, str::from_utf8(&body)?.to_owned()))
}

#[cfg(feature = "enable_hyper")]
#[inline]
pub async fn check_status_extract_body(
    resp: hyper::client::ResponseFuture,
    expected_status_code: hyper::StatusCode,
) -> Result<String, AzureError> {
    let (status, body) = extract_status_and_body(resp).await?;
    if status == expected_status_code {
        Ok(body)
    } else {
        Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            expected_status_code,
            status,
            &body,
        )))
    }
}

#[cfg(feature = "enable_hyper")]
pub async fn check_status_extract_body_2(
    resp: hyper::Response<Body>,
    expected_status: StatusCode,
) -> Result<String, AzureError> {
    let received_status = resp.status();

    let body = body::to_bytes(resp.into_body()).await?;
    let s = String::from_utf8(body.to_vec())?;
    debug!("body: {}", s);
    if received_status != expected_status {
        Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            expected_status,
            received_status,
            &s,
        )))
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

    fn error_generator() -> Result<(), AzureError> {
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

    // This does not compile
    //#[test]
    //fn test_not_send() {
    //    let a = std::rc::Rc::new(100);
    //    send_fn(a);
    //}

    // This does not compile
    //#[test]
    //fn test_not_sync() {
    //    let a = std::cell::Cell::new(500);
    //    sync_fn(a);
    //}
}
