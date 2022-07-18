mod client;
pub use client::DeviceUpdateClient;

pub mod device_update;

#[allow(unused_imports)]
use crate::device_update::UpdateOperation;

#[cfg(test)]
mod tests {
    use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};
    use azure_identity::AutoRefreshingTokenCredential;
    use chrono::{Duration, Utc};
    use std::sync::Arc;

    pub(crate) fn mock_client() -> crate::client::DeviceUpdateClient {
        crate::client::DeviceUpdateClient {
            device_update_url: url::Url::parse(&mockito::server_url()).unwrap(),
            endpoint: "".to_string(),
            token_credential: AutoRefreshingTokenCredential::new(Arc::new(MockCredential)),
        }
    }

    pub(crate) struct MockCredential;

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _resource: &str,
        ) -> Result<TokenResponse, azure_core::error::Error> {
            Ok(TokenResponse::new(
                AccessToken::new("TOKEN".to_owned()),
                Utc::now() + Duration::days(14),
            ))
        }
    }
}
