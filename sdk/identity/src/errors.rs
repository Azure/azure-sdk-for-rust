//! Errors specific to identity services.
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
/// An unrecognized error response from an identity service.
pub enum Error {
    #[error("Error getting token credentials from Azure CLI: {0}")]
    AzureCliCredentialError(#[from] crate::token_credentials::AzureCliCredentialError),
    #[error("Client secret credentials error: {0}")]
    ClientSecretCredentialError(#[from] crate::token_credentials::ClientSecretCredentialError),
    #[error("Error refreshing token: {0}")]
    RefreshTokenError(#[from] crate::refresh_token::Error),
    #[error("Error response from service: {0}")]
    ErrorResponse(String),

    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// An HTTP error response from the identity service.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ErrorResponse {
    /// An unrecognized error response from an identity service.
    GenericError { error_description: String },
}
