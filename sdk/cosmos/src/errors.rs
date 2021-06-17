/// An error having to do with Cosmos.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error as defined in the `azure_core` crate
    #[error(transparent)]
    AzureCoreError(#[from] azure_core::Error),
    /// An error related to parsing
    #[error(transparent)]
    ParsingError(ParsingError),
    // TODO forward this to azure_core::Error
    #[allow(missing_docs)]
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),
    /// Other errors that can happen but are unlikely to be matched against
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// A parsing error
///
/// Most issues are already defined in `azure_core`
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

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::AzureCoreError(azure_core::Error::JsonError(error))
    }
}
impl From<azure_core::StreamError> for Error {
    fn from(error: azure_core::StreamError) -> Self {
        Self::AzureCoreError(azure_core::Error::StreamError(error))
    }
}
impl From<azure_core::HttpError> for Error {
    fn from(error: azure_core::HttpError) -> Self {
        Self::AzureCoreError(azure_core::Error::HttpError(error))
    }
}
