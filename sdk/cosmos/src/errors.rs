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
    #[error("http error: {0}")]
    CoreHttpError(#[from] azure_core::HttpError),
    #[error("stream error: {0}")]
    StreamError(#[from] azure_core::StreamError),
    #[error("http error: {0}")]
    HttpError(#[from] http::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    /// Other errors that can happen but are unlikely to be matched against
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// A parsing error
///
/// Most issues are already defined in `azure_core`
#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error(transparent)]
    Core(#[from] azure_core::ParsingError),
    #[error("Resource quota parsing error: {0}")]
    ParseResourceQuotaError(#[from] crate::resource_quota::ResourceQuotaParsingError),
}
