mod cli_credentials;
mod client_secret_credentials;
mod default_credentials;
mod environment_credentials;
mod managed_identity_credentials;
pub use crate::token_credentials::cli_credentials::*;
pub use crate::token_credentials::client_secret_credentials::*;
pub use crate::token_credentials::default_credentials::*;
pub use crate::token_credentials::environment_credentials::*;
pub use crate::token_credentials::managed_identity_credentials::*;
use azure_sdk_core::errors::AzureError;
use chrono::{DateTime, Utc};
use oauth2::AccessToken;

/// Represents an Azure service bearer access token with expiry information.
#[derive(Debug, Clone)]
pub struct TokenResponse {
    /// Get the access token value.
    pub token: AccessToken,
    /// Gets the time when the provided token expires.
    pub expires_on: DateTime<Utc>,
}

impl TokenResponse {
    pub fn new(token: AccessToken, expires_on: DateTime<Utc>) -> Self {
        TokenResponse { token, expires_on }
    }
}
/// Represents a credential capable of providing an OAuth token.
#[async_trait::async_trait]
pub trait TokenCredential {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError>;
}
