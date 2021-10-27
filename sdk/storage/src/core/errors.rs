use std::cmp::PartialEq;

/// A specialized Result type.
pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(#[from] azure_core::Error),
    #[error("Parsing error: {0}")]
    ParsingError(#[from] azure_core::ParsingError),
    #[error("Permission error: {0}")]
    PermissionError(#[from] azure_core::PermissionError),
    #[error("Parse bool error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Connection string error: {0}")]
    ConnectionStringError(#[from] super::connection_string::ConnectionStringError),
    #[error("To str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Date time parse error: {0}")]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("XML error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
    #[error("Traversing error: {0}")]
    TraversingError(#[from] azure_core::TraversingError),
    #[error("XML builder error: {0}")]
    XmlBuilderError(#[from] xml::BuilderError),
    #[error("From UTF-8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Invalid status code: {0:?}")]
    InvalidStatusCode(#[from] http::status::InvalidStatusCode),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("A required header is missing: {0}")]
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
    #[error("uuid error: {0}")]
    ParseUuidError(#[from] uuid::Error),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Header not found: {0}")]
    HeaderNotFound(String),
    #[error("Error parsing the transaction response: {0:?}")]
    TransactionResponseParseError(String),
    #[error("Generic error: {0}")]
    GenericErrorWithText(String),
    #[error("Operation not supported. Operation == {0}, reason == {1}")]
    OperationNotSupported(String, String),
    #[error("UnexpectedXMLError: {0}")]
    UnexpectedXMLError(String),
    #[error("digest length {0} bytes instead of 16")]
    DigestNot16BytesLong(usize),
    #[error("CRC64 length {0} bytes instead of 8")]
    CRC64Not8BytesLong(usize),
    #[error("At least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error("error writing the header value: {0}")]
    InvalidHeaderValue(#[from] azure_core::HTTPHeaderError),
}

impl From<azure_core::HttpError> for Error {
    fn from(error: azure_core::HttpError) -> Self {
        Self::CoreError(azure_core::Error::HttpError(error))
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AzurePathParseError {
    #[error("path separator not found")]
    PathSeparatorNotFoundError,
    #[error("multiple path separators found")]
    MultiplePathSeparatorsFoundError,
    #[error("missing container name")]
    MissingContainerError,
    #[error("missing blob name")]
    MissingBlobError,
}
