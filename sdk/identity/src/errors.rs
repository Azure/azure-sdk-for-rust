//! Errors specific to identity services.
use serde::Deserialize;
use std::fmt;

/// Errors specific to identity services
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error getting credentials from the Azure CLI
    #[error("Error getting token credentials from Azure CLI")]
    AzureCliCredential(#[from] crate::AzureCliCredentialError),
    /// An error getting credentials through the client secrect token credential flow
    #[error("Client secret credentials error")]
    ClientSecretCredential(#[from] crate::ClientSecretCredentialError),
    /// An error getting credentials from the environment
    #[error("Error getting environment credential")]
    EnvironmentCredential(#[from] crate::EnvironmentCredentialError),
    /// An error getting managed identity credentials
    #[error("Error getting managed identity credential")]
    ManagedIdentityCredential(#[from] crate::ManagedIdentityCredentialError),
    /// An error using the default token credential flow
    #[error("Error getting default credential")]
    DefaultAzureCredentialError(#[from] crate::DefaultAzureCredentialError),
    /// An error getting a refresh token
    #[error("Error refreshing token")]
    RefreshToken(#[from] crate::refresh_token::Error),
    /// An error performing the device code flow
    #[error("Error performing the device code flow")]
    DeviceCode(#[from] crate::device_code_flow::DeviceCodeError),
    /// An error performing the device code flow
    #[error("Error performing the device code flow")]
    ClientCredential(#[from] crate::client_credentials_flow::ClientCredentialError),
    /// An unrecognized error response from an identity service.
    #[error("Error response from service: {0}")]
    ErrorResponse(String),
}

/// Error Token
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct ErrorToken {
    error: String,
    error_description: String,
    error_codes: Vec<i64>,
    timestamp: Option<String>,
    trace_id: Option<String>,
    correlation_id: Option<String>,
    suberror: Option<String>,
    claims: Option<String>,
}

impl fmt::Display for ErrorToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "error: {}", self.error)?;
        if let Some(suberror) = &self.suberror {
            writeln!(f, "suberror: {}", suberror)?;
        }
        writeln!(f, "description: {}", self.error_description)
    }
}
