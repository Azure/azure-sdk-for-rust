// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_lock::RwLock;
use azure_core::credentials::{AccessToken, TokenRequestOptions};
use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::trace;

#[derive(Debug)]
pub(crate) struct TokenCache(RwLock<HashMap<Vec<String>, AccessToken>>);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) trait MaybeSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> MaybeSend for T {}

#[cfg(target_arch = "wasm32")]
pub(crate) trait MaybeSend {}
#[cfg(target_arch = "wasm32")]
impl<T> MaybeSend for T {}

impl TokenCache {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub(crate) async fn get_token<'a, C, F>(
        &self,
        scopes: &'a [&'a str],
        options: Option<TokenRequestOptions>,
        callback: C,
    ) -> azure_core::Result<AccessToken>
    where
        C: FnOnce(&'a [&'a str], Option<TokenRequestOptions>) -> F + MaybeSend,
        F: Future<Output = azure_core::Result<AccessToken>> + MaybeSend,
    {
        let token_cache = self.0.read().await;
        let scopes_owned = scopes.iter().map(ToString::to_string).collect::<Vec<_>>();
        if let Some(token) = token_cache.get(&scopes_owned) {
            if !should_refresh(token) {
                trace!("returning cached token");
                return Ok(token.clone());
            }
        }

        // otherwise, drop the read lock and get a write lock to refresh the token
        drop(token_cache);
        let mut token_cache = self.0.write().await;

        // check again in case another thread refreshed the token while we were
        // waiting on the write lock
        if let Some(token) = token_cache.get(&scopes_owned) {
            if !should_refresh(token) {
                trace!("returning token that was updated while waiting on write lock");
                return Ok(token.clone());
            }
        }

        trace!("token cache miss");
        let token = callback(scopes, options).await?;
        token_cache.insert(scopes_owned, token.clone());
        Ok(token)
    }
}

impl Default for TokenCache {
    fn default() -> Self {
        TokenCache::new()
    }
}

fn should_refresh(token: &AccessToken) -> bool {
    token.expires_on <= OffsetDateTime::now_utc() + Duration::from_secs(300)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_lock::Mutex;
    use azure_core::credentials::Secret;
    use std::time::Duration;
    use time::OffsetDateTime;

    #[derive(Debug)]
    struct MockCredential {
        token: AccessToken,
        get_token_call_count: Mutex<usize>,
    }

    impl MockCredential {
        fn new(token: AccessToken) -> Self {
            Self {
                token,
                get_token_call_count: Mutex::new(0),
            }
        }

        async fn get_token(
            &self,
            scopes: &[&str],
            _: Option<TokenRequestOptions>,
        ) -> azure_core::Result<AccessToken> {
            // Include an incrementing counter in the token to track how many times the token has been refreshed
            let mut call_count = self.get_token_call_count.lock().await;
            *call_count += 1;
            Ok(AccessToken {
                token: Secret::new(format!(
                    "{}-{}:{}",
                    scopes.join(" "),
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
        let resource1 = &[STORAGE_TOKEN_SCOPE];
        let resource2 = &[IOTHUB_TOKEN_SCOPE];
        let secret_string = "test-token";
        let expires_on = OffsetDateTime::now_utc() + Duration::from_secs(3600);
        let access_token = AccessToken::new(Secret::new(secret_string), expires_on);

        let mock_credential = MockCredential::new(access_token);

        let cache = TokenCache::new();

        // Test that querying a token for the same resource twice returns the same (cached) token on the second call
        let token1 = cache
            .get_token(resource1, None, |s, o| mock_credential.get_token(s, o))
            .await?;
        let token2 = cache
            .get_token(resource1, None, |s, o| mock_credential.get_token(s, o))
            .await?;

        let expected_token = format!("{}-{}:1", resource1.join(" "), secret_string);
        assert_eq!(token1.token.secret(), expected_token);
        assert_eq!(token2.token.secret(), expected_token);

        // Test that querying a token for a second resource returns a different token, as the cache is per-resource.
        // Also test that the same token is the returned (cached) on a second call.
        let token3 = cache
            .get_token(resource2, None, |s, o| mock_credential.get_token(s, o))
            .await?;
        let token4 = cache
            .get_token(resource2, None, |s, o| mock_credential.get_token(s, o))
            .await?;
        let expected_token = format!("{}-{}:2", resource2.join(" "), secret_string);
        assert_eq!(token3.token.secret(), expected_token);
        assert_eq!(token4.token.secret(), expected_token);

        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_expired_token() -> azure_core::Result<()> {
        let resource = &[STORAGE_TOKEN_SCOPE];
        let access_token = "test-token";
        let expires_on = OffsetDateTime::now_utc();
        let token_response = AccessToken::new(Secret::new(access_token), expires_on);

        let mock_credential = MockCredential::new(token_response);

        let cache = TokenCache::new();

        // Test that querying an expired token returns a new token
        for i in 1..5 {
            let token = cache
                .get_token(resource, None, |s, o| mock_credential.get_token(s, o))
                .await?;
            assert_eq!(
                token.token.secret(),
                format!("{}-{}:{}", resource.join(" "), access_token, i)
            );
        }

        Ok(())
    }
}
