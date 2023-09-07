use async_lock::RwLock;
use azure_core::auth::{TokenCredential, TokenResponse};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

fn is_expired(token: &TokenResponse) -> bool {
    token.expires_on < OffsetDateTime::now_utc() + Duration::from_secs(20)
}

#[derive(Clone)]
/// Wraps a `TokenCredential` and handles token refresh on token expiry.
pub struct AutoRefreshingTokenCredential {
    credential: Arc<dyn TokenCredential>,
    // Tokens are specific to a resource, so we cache tokens by resource.
    token_cache: Arc<RwLock<HashMap<String, TokenResponse>>>,
}

impl std::fmt::Debug for AutoRefreshingTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AutoRefreshingTokenCredential")
            .field("credential", &"TokenCredential")
            .finish()
    }
}

impl AutoRefreshingTokenCredential {
    /// Create a new `AutoRefreshingTokenCredential` around the provided base provider.
    pub fn new(provider: Arc<dyn TokenCredential>) -> Self {
        Self {
            credential: provider,
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AutoRefreshingTokenCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        // if the current cached token for this resource is good, return it.
        let token_cache = self.token_cache.read().await;
        if let Some(token) = token_cache.get(resource) {
            if !is_expired(token) {
                return Ok(token.clone());
            }
        }

        // otherwise, drop the read lock and get a write lock to refresh the token
        drop(token_cache);
        let mut token_cache = self.token_cache.write().await;

        // check again in case another thread refreshed the token while we were
        // waiting on the write lock
        if let Some(token) = token_cache.get(resource) {
            if !is_expired(token) {
                return Ok(token.clone());
            }
        }

        let token = self.credential.get_token(resource).await?;

        // NOTE: we do not check to see if the token is expired here, as at
        // least one credential, `AzureCliCredential`, specifies the token is
        // immediately expired after it is returned, which indicates the token
        // should always be refreshed upon use.
        token_cache.insert(resource.to_string(), token.clone());
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::auth::AccessToken;
    use azure_core::auth::TokenCredential;
    use std::sync::Mutex;

    struct MockCredential {
        token: TokenResponse,
        get_token_call_count: Mutex<usize>,
    }

    impl MockCredential {
        fn new(token: TokenResponse) -> Self {
            Self {
                token,
                get_token_call_count: Mutex::new(0),
            }
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for MockCredential {
        async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
            // Include an incrementing counter in the token to track how many times the token has been refreshed
            let mut call_count = self.get_token_call_count.lock().unwrap();
            *call_count += 1;
            Ok(TokenResponse {
                token: AccessToken::new(format!(
                    "{}-{}:{}",
                    resource,
                    self.token.token.secret(),
                    *call_count
                )),
                expires_on: self.token.expires_on,
            })
        }
    }

    const STORAGE_TOKEN_SCOPE: &str = "https://storage.azure.com/";
    const IOTHUB_TOKEN_SCOPE: &str = "https://iothubs.azure.net";

    #[tokio::test]
    async fn test_get_token_different_resources() -> azure_core::Result<()> {
        let resource1 = STORAGE_TOKEN_SCOPE;
        let resource2 = IOTHUB_TOKEN_SCOPE;
        let token_value = "test-token";
        let access_token = AccessToken::new(token_value);
        let expires_on = OffsetDateTime::now_utc() + Duration::from_secs(300);
        let token_response = TokenResponse::new(access_token, expires_on);

        let mock_credential = MockCredential::new(token_response);
        let auto_refreshing_credential =
            AutoRefreshingTokenCredential::new(Arc::new(mock_credential));

        // Test that querying a token for the same resource twice returns the same (cached) token on the second call
        let token1 = auto_refreshing_credential.get_token(resource1).await?;
        let token2 = auto_refreshing_credential.get_token(resource1).await?;
        let expected_token = format!("{}-{}:1", resource1, token_value);
        assert_eq!(token1.token.secret(), expected_token);
        assert_eq!(token2.token.secret(), expected_token);

        // Test that querying a token for a second resource returns a different token, as the cache is per-resource.
        // Also test that the same token is the returned (cached) on a second call.
        let token3 = auto_refreshing_credential.get_token(resource2).await?;
        let token4 = auto_refreshing_credential.get_token(resource2).await?;
        let expected_token = format!("{}-{}:2", resource2, token_value);
        assert_eq!(token3.token.secret(), expected_token);
        assert_eq!(token4.token.secret(), expected_token);

        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_expired_token() -> azure_core::Result<()> {
        let resource = STORAGE_TOKEN_SCOPE;
        let token_value = "test-token";
        let access_token = AccessToken::new(token_value);
        let expires_on = OffsetDateTime::now_utc();
        let token_response = TokenResponse::new(access_token, expires_on);

        let mock_credential = MockCredential::new(token_response);
        let auto_refreshing_credential =
            AutoRefreshingTokenCredential::new(Arc::new(mock_credential));

        // Test that querying an expired token returns a new token
        for i in 1..5 {
            let token = auto_refreshing_credential.get_token(resource).await?;
            assert_eq!(
                token.token.secret(),
                format!("{}-{}:{}", resource, token_value, i)
            );
        }

        Ok(())
    }
}
