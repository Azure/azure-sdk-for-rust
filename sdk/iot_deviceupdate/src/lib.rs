mod client;
pub use client::DeviceUpdateClient;

pub mod device_update;

#[allow(unused_imports)]
use crate::device_update::UpdateOperation;

#[cfg(test)]
mod tests {
    use azure_core::{
        auth::{AccessToken, TokenCredential},
        date, Url,
    };
    use std::sync::Arc;
    use time::OffsetDateTime;

    pub(crate) fn mock_client(server_url: String) -> crate::client::DeviceUpdateClient {
        crate::client::DeviceUpdateClient {
            device_update_url: Url::parse(&server_url).unwrap(),
            token_credential: Arc::new(MockCredential),
        }
    }

    #[derive(Debug)]
    pub(crate) struct MockCredential;

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
        ) -> Result<AccessToken, azure_core::error::Error> {
            Ok(AccessToken::new(
                "TOKEN".to_owned(),
                OffsetDateTime::now_utc() + date::duration_from_days(14),
            ))
        }

        async fn clear_cache(&self) -> azure_core::Result<()> {
            Ok(())
        }
    }
}
