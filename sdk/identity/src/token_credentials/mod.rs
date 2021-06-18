//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod cli_credentials;
mod client_secret_credentials;
mod default_credentials;
mod environment_credentials;
mod managed_identity_credentials;
mod powershell_credential;

pub use cli_credentials::*;
pub use client_secret_credentials::*;
pub use default_credentials::*;
pub use environment_credentials::*;
pub use managed_identity_credentials::*;
pub use powershell_credential::*;

/// Represents a credential capable of providing an OAuth token.
/// Same as [azure_core::TokenCredential](azure_core::TokenCredential), except a more specific error is returned.
#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync {
    type Error;
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> Result<azure_core::TokenResponse, Self::Error>;
}

#[async_trait::async_trait]
impl<Error> azure_core::TokenCredential for dyn TokenCredential<Error = Error>
where
    Error: std::error::Error + Send + Sync + 'static,
{
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetTokenError(Box::new(error)))
    }
}
