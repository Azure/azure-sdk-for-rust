use azure_core::auth::{TokenCredential, TokenResponse};

use super::shared_access_credential::SharedAccessCredential;

/// <summary>
///   Provides a generic token-based credential for a given Service Bus entity instance.
/// </summary>
///
/// <seealso cref="Azure.Core.TokenCredential" />
#[derive(Debug)]
pub enum ServiceBusTokenCredential<TC>
where
    TC: TokenCredential,
{
    SharedAccessCredential(SharedAccessCredential),
    Other(TC),
}

impl From<SharedAccessCredential> for ServiceBusTokenCredential<SharedAccessCredential> {
    fn from(source: SharedAccessCredential) -> Self {
        Self::SharedAccessCredential(source)
    }
}

impl<TC> ServiceBusTokenCredential<TC>
where
    TC: TokenCredential,
{
    /// <summary>
    ///   Indicates whether the credential is based on an Service Bus
    ///   shared access policy.
    /// </summary>
    ///
    /// <value><c>true</c> if the credential should be considered a SAS credential; otherwise,
    /// <c>false</c>.</value>
    ///
    pub fn is_shared_access_credential(&self) -> bool {
        match self {
            ServiceBusTokenCredential::SharedAccessCredential(_) => true,
            ServiceBusTokenCredential::Other(_) => false,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<TC> TokenCredential for ServiceBusTokenCredential<TC>
where
    TC: TokenCredential,
{
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        match self {
            ServiceBusTokenCredential::SharedAccessCredential(credential) => {
                credential.get_token(resource).await
            }
            ServiceBusTokenCredential::Other(credential) => credential.get_token(resource).await,
        }
    }
}
