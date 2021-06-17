/// An error having to do with Cosmos.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    AzureCoreError(#[from] azure_core::Error),
    #[error(transparent)]
    ParsingError(ParsingError),
    #[error("http error: {0}")]
    AzureHttpError(#[from] azure_core::HttpError),
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    /// Other errors that can happen but are unlikely to be matched against
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Resource quota parsing error: {0}")]
    ParseResourceQuotaError(#[from] crate::resource_quota::ResourceQuotaParsingError),
    #[error(transparent)]
    Other(azure_core::ParsingError),
}

impl<T: Into<azure_core::ParsingError>> From<T> for ParsingError {
    fn from(error: T) -> Self {
        Self::Other(error.into())
    }
}
