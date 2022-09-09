use azure_core::auth::{TokenCredential, TokenResponse};

/// <summary>
///   Provides a generic token-based credential for a given Service Bus entity instance.
/// </summary>
///
/// <seealso cref="Azure.Core.TokenCredential" />
#[derive(Debug)]
pub(crate) struct ServiceBusTokenCredential<T>
where
    T: TokenCredential,
{
    credential: T,
    // is_shared_access_credential: bool,
}

impl<T> ServiceBusTokenCredential<T>
where
    T: TokenCredential,
{
    pub fn new(credential: T) -> Self {
        Self { credential }
    }

    // /// <summary>
    // ///   Indicates whether the credential is based on an Service Bus
    // ///   shared access policy.
    // /// </summary>
    // ///
    // /// <value><c>true</c> if the credential should be considered a SAS credential; otherwise, <c>false</c>.</value>
    // ///
    // pub fn is_shared_access_credential(&self) -> bool {
    //     self.is_shared_access_credential
    // }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<T> TokenCredential for ServiceBusTokenCredential<T>
where
    T: TokenCredential,
{
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        self.credential.get_token(resource).await
    }
}
