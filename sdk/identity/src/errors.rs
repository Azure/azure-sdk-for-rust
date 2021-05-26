//! Errors specific to identity services.
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error getting token credentials from Azure CLI: {0}")]
    AzureCliCredentialError(#[from] crate::token_credentials::AzureCliCredentialError),
    #[error("Client secret credentials error: {0}")]
    ClientSecretCredentialError(#[from] crate::token_credentials::ClientSecretCredentialError),
    #[error("Error getting environment credential: {0}")]
    EnvironmentCredentialError(#[from] crate::token_credentials::EnvironmentCredentialError),
    #[error("Error getting managed identity credential: {0}")]
    ManagedIdentityCredentialError(
        #[from] crate::token_credentials::ManagedIdentityCredentialError,
    ),
    #[error("Error getting default credential: {0}")]
    DefaultCredentialError(#[from] crate::token_credentials::DefaultCredentialError),
    #[error("Error refreshing token: {0}")]
    RefreshTokenError(#[from] crate::refresh_token::Error),
    /// An unrecognized error response from an identity service.
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
