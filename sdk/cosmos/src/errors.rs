/// A general error having to do with Cosmos.
#[derive(Debug, thiserror::Error)]
pub enum CosmosError {
    /// An error when parsing JSON
    #[error("An error parsing JSON occured: {}", 0)]
    JsonError(#[from] serde_json::Error),
    /// An error when building a request
    #[error("An error in building a request occured: {}", 0)]
    RequestBuilderError(#[from] http::Error),
    /// An http error occured
    #[error("An error occurred when making an http request")]
    HttpRequestError(#[from] azure_core::errors::HttpRequestError),
    /// An unexpected http result
    #[error("An unexpected http error occured: {}", 0)]
    UnexpectedHTTPResult(#[from] azure_core::errors::UnexpectedHTTPResult),
    /// An header failed to parse
    #[error("Failed to parse a header: {}", 0)]
    HeaderError(#[from] azure_core::errors::HeaderError),
    /// A generic catchall error
    #[error("An error occurred: {}", 0)]
    GenericError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<azure_core::errors::AzureError> for CosmosError {
    fn from(error: azure_core::errors::AzureError) -> Self {
        CosmosError::GenericError(error.into())
    }
}
impl From<std::str::Utf8Error> for CosmosError {
    fn from(error: std::str::Utf8Error) -> Self {
        CosmosError::GenericError(error.into())
    }
}

impl From<base64::DecodeError> for CosmosError {
    fn from(error: base64::DecodeError) -> Self {
        CosmosError::GenericError(error.into())
    }
}
