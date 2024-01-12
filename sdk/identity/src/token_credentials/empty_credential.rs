use azure_core::auth::{AccessToken, Secret, TokenCredential};
use std::{str, sync::Arc};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Default)]
pub struct EmptyCredential {}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for EmptyCredential {
    /// Returns an empty string as the token.
    async fn get_token(&self, _scopes: &[&str]) -> azure_core::Result<AccessToken> {
        Ok(AccessToken::new(
            Secret::new(String::new()),
            OffsetDateTime::now_utc() + Duration::hours(1),
        ))
    }
    async fn clear_cache(&self) -> azure_core::Result<()> {
        Ok(())
    }
}

pub fn create_empty_credential() -> Arc<dyn TokenCredential> {
    Arc::new(EmptyCredential::default())
}
