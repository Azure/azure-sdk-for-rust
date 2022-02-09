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
mod imds_managed_identity_credentials;

pub use cli_credentials::*;
pub use client_secret_credentials::*;
pub use default_credentials::*;
pub use environment_credentials::*;
pub use imds_managed_identity_credentials::*;

/// Represents a credential capable of providing an OAuth token.
/// Same as [azure_core::auth::TokenCredential](azure_core::auth::TokenCredential), except a more specific error is returned.
#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync {
    /// A more specific error.
    type Error;
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::auth::TokenResponse, Self::Error>;
}

#[async_trait::async_trait]
impl<Error> azure_core::auth::TokenCredential for dyn TokenCredential<Error = Error>
where
    Error: std::error::Error + Send + Sync + 'static,
{
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::auth::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetToken(Box::new(error)))
    }
}
