use azure_core::auth::{TokenCredential, TokenResponse};

pub(crate) struct ServiceBusTokenCredential {}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ServiceBusTokenCredential {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        todo!()
    }
}
