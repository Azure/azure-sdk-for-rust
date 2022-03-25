/// A specialized Result type for Cosmos.
pub type Result<T> = std::result::Result<T, Error>;

/// An error having to do with Cosmos.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error as defined in the `azure_core` crate
    #[error(transparent)]
    Core(#[from] azure_core::Error),
    #[error("error parsing authorization token: {0}")]
    AuthorizationTokenParse(#[from] crate::resources::permission::AuthorizationTokenParseError),
}

impl From<azure_core::error::Error> for Error {
    fn from(err: azure_core::error::Error) -> Self {
        Self::Core(err.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Core(azure_core::Error::Json(error))
    }
}
