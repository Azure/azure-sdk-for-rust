//! Errors specific to identity services.
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
/// An unrecognized error response from an identity service.
pub enum Error {
    #[error("Error getting token credentials from Azure CLI: {}", 0)]
    AzureCliError(#[from] crate::token_credentials::AzureCliError),
    #[error("Error refreshing token: {}", 0)]
    RefreshTokenError(#[from] crate::refresh_token::Error),
}

/// An HTTP error response from the identity service.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ErrorResponse {
    /// An unrecognized error response from an identity service.
    GenericError { error_description: String },
}
