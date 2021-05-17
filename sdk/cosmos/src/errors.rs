/// A general error having to do with Cosmos.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CosmosError {
    #[error(transparent)]
    AzureCoreError(#[from] azure_core::errors::AzureError),
    #[error("Policy error: {}", 0)]
    PolicyError(Box<dyn std::error::Error + Send + Sync>),
    #[error("Header not found: {}", 0)]
    HeaderNotFound(String),
    #[error("Generic error: {}", 0)]
    GenericErrorWithText(String),
    #[error("To str error: {}", 0)]
    ToStrError(#[from] http::header::ToStrError),
    #[error("Parsing error: {}", 0)]
    ParsingError(#[from] azure_core::errors::ParsingError),
    #[error("http error: {}", 0)]
    AzureHttpError(#[from] azure_core::errors::HttpError),
    #[error("stream error: {}", 0)]
    StreamError(#[from] azure_core::errors::StreamError),
    #[error("http error: {}", 0)]
    HttpError(#[from] http::Error),
    #[error("Parse int error: {}", 0)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("uuid error: {}", 0)]
    ParseUuidError(#[from] uuid::Error),
    #[error("Date time parse error: {}", 0)]
    DateTimeParseError(#[from] chrono::format::ParseError),
    #[error("Parse float error: {}", 0)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("JSON error: {}", 0)]
    JsonError(#[from] serde_json::Error),
    #[error("UTF-8 conversion error: {}", 0)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("base64 decode error: {}", 0)]
    DecodeError(#[from] base64::DecodeError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionToDocumentError {
    #[error("Conversion to document failed because at lease one element is raw.")]
    RawElementFound,
}
