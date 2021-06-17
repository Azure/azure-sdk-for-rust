/// A general error having to do with Cosmos.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AzureCoreError(#[from] azure_core::Error),
    #[error(transparent)]
    ParsingError(ParsingError),
    #[error("Header not found: {0}")]
    HeaderNotFound(String),
    #[error("To str error: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("http error: {0}")]
    AzureHttpError(#[from] azure_core::HttpError),
    #[error("stream error: {0}")]
    StreamError(#[from] azure_core::StreamError),
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("base64 decode error: {0}")]
    DecodeError(#[from] base64::DecodeError),
    /// Other errors that can happen but are unlikely to be matched against
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Resource quota parsing error: {0}")]
    ParseResourceQuotaError(#[from] crate::resource_quota::ResourceQuotaParsingError),
    #[error("parsing error: {0}")]
    Other(azure_core::ParsingError),
}

impl<T: Into<azure_core::ParsingError>> From<T> for ParsingError {
    fn from(error: T) -> Self {
        Self::Other(error.into())
    }
}
