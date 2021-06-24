/// An error having to do with Cosmos.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error as defined in the `azure_core` crate
    #[error(transparent)]
    Core(#[from] azure_core::Error),
    /// An error related to parsing
    #[error(transparent)]
    ParsingError(#[from] ParsingError),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Core(azure_core::Error::JsonError(error))
    }
}

impl From<azure_core::StreamError> for Error {
    fn from(error: azure_core::StreamError) -> Self {
        Self::Core(azure_core::Error::StreamError(error))
    }
}

impl From<azure_core::HttpError> for Error {
    fn from(error: azure_core::HttpError) -> Self {
        Self::Core(azure_core::Error::HttpError(error))
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for Error {
    fn from(error: Box<dyn std::error::Error + Sync + Send>) -> Self {
        Self::Core(azure_core::Error::Other(error))
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Self::Core(azure_core::Error::Other(error.into()))
    }
}

/// A parsing error
///
/// Most issues are already defined in `azure_core`
#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error(transparent)]
    Core(azure_core::ParsingError),
    #[error("Resource quota parsing error: {0}")]
    ParseResourceQuotaError(#[from] crate::resource_quota::ResourceQuotaParsingError),
}

impl<T: Into<azure_core::ParsingError>> From<T> for ParsingError {
    fn from(error: T) -> Self {
        Self::Core(error.into())
    }
}
