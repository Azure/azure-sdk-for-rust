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
    /// An error related to parsing
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("conversion to `{0}` failed because at lease one element is raw")]
    ElementIsRaw(String),
    #[error("error parsing authorization token: {0}")]
    AuthorizationTokenParse(#[from] crate::resources::permission::AuthorizationTokenParseError),
    #[error("error parsing permission token: {0}")]
    PermissionTokenParse(#[from] crate::resources::permission::PermissionTokenParseError),
    #[error("error writing the header value: {0}")]
    InvalidHeaderValue(#[from] azure_core::HttpHeaderError),
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

impl From<azure_core::StreamError> for Error {
    fn from(error: azure_core::StreamError) -> Self {
        Self::Core(azure_core::Error::Stream(error))
    }
}

impl From<azure_core::HttpError> for Error {
    fn from(error: azure_core::HttpError) -> Self {
        Self::Core(azure_core::Error::Http(error))
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Self {
        Self::Core(azure_core::Error::HttpPrepare(error))
    }
}

/// A parsing error
///
/// Most issues are already defined in `azure_core`
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Core(azure_core::ParseError),
    #[error("Resource quota parsing error: {0}")]
    ParseResourceQuotaError(#[from] crate::resource_quota::ResourceQuotaParseError),
}

impl<T: Into<azure_core::ParseError>> From<T> for ParseError {
    fn from(error: T) -> Self {
        Self::Core(error.into())
    }
}
