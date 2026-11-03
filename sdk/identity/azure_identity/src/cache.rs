// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_lock::RwLock;
use azure_core::credentials::{AccessToken, TokenRequestOptions};
use azure_core::http::{ClientOptions, Url};
use azure_core::time::{Duration, OffsetDateTime};
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, LazyLock, Mutex},
    time::Instant,
};
use tracing::trace;

#[derive(Debug)]
pub(crate) struct TokenCache(RwLock<HashMap<Vec<String>, AccessToken>>);

impl TokenCache {
    pub(crate) fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub(crate) async fn get_token<'a, C, F>(
        &self,
        scopes: &'a [&'a str],
        options: Option<TokenRequestOptions<'a>>,
        callback: C,
    ) -> azure_core::Result<AccessToken>
    where
        C: FnOnce(&'a [&'a str], Option<TokenRequestOptions<'a>>) -> F + Send,
        F: Future<Output = azure_core::Result<AccessToken>> + Send,
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

    fn has_valid_tokens(&self) -> bool {
        self.0
            .try_read()
            .is_none_or(|tokens| tokens.values().any(|token| !should_refresh(token)))
    }
}

impl Default for TokenCache {
    fn default() -> Self {
        TokenCache::new()
    }
}

const MANAGED_IDENTITY_CACHE_CAPACITY: usize = 1024;

static MANAGED_IDENTITY_CACHE: LazyLock<Mutex<SharedManagedIdentityCache>> = LazyLock::new(|| {
    Mutex::new(SharedManagedIdentityCache::new(
        MANAGED_IDENTITY_CACHE_CAPACITY,
    ))
});

pub(crate) struct ManagedIdentityTokenCache {
    // Preserve the existing instance-cache fast path before consulting the process cache.
    pub(crate) local: TokenCache,
    source: &'static str,
    shared: bool,
}

impl ManagedIdentityTokenCache {
    pub(crate) fn new(source: &'static str, options: &ClientOptions) -> Self {
        // Custom transports and policies can change the authentication destination or context.
        let shared = options.transport.is_none()
            && options.per_call_policies.is_empty()
            && options.per_try_policies.is_empty();
        Self {
            local: TokenCache::new(),
            source,
            shared,
        }
    }

    pub(crate) fn get(
        &self,
        url: &Url,
        authentication_context: Option<(&str, &str)>,
    ) -> Option<Arc<TokenCache>> {
        if !self.shared {
            return None;
        }

        let key = ManagedIdentityCacheKey {
            source: self.source,
            url: url.clone(),
            authentication_context: authentication_context
                .map(|(name, value)| (name.to_owned(), value.to_owned())),
        };
        Some(
            MANAGED_IDENTITY_CACHE
                .lock()
                .unwrap_or_else(|err| err.into_inner())
                .get(key),
        )
    }
}

// Don't implement Debug: the authentication context can contain an App Service secret.
#[derive(Clone, Eq, Hash, PartialEq)]
struct ManagedIdentityCacheKey {
    source: &'static str,
    // Includes the identity selector, API version, and resource actually sent to the endpoint.
    url: Url,
    authentication_context: Option<(String, String)>,
}

struct SharedCacheEntry {
    cache: Arc<TokenCache>,
    last_used: Instant,
}

struct SharedManagedIdentityCache {
    entries: HashMap<ManagedIdentityCacheKey, SharedCacheEntry>,
    capacity: usize,
}

impl SharedManagedIdentityCache {
    fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
        }
    }

    fn get(&mut self, key: ManagedIdentityCacheKey) -> Arc<TokenCache> {
        if let Some(entry) = self.entries.get_mut(&key) {
            entry.last_used = Instant::now();
            return entry.cache.clone();
        }

        self.entries.retain(|_, entry| {
            Arc::strong_count(&entry.cache) > 1 || entry.cache.has_valid_tokens()
        });

        if self.entries.len() >= self.capacity {
            let oldest = self
                .entries
                .iter()
                .filter(|(_, entry)| Arc::strong_count(&entry.cache) == 1)
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(key, _)| key.clone());
            if let Some(oldest) = oldest {
                self.entries.remove(&oldest);
            } else {
                // Don't evict an in-use cell and start a second acquisition for the same key.
                return Arc::new(TokenCache::new());
            }
        }

        let cache = Arc::new(TokenCache::new());
        self.entries.insert(
            key,
            SharedCacheEntry {
                cache: cache.clone(),
                last_used: Instant::now(),
            },
        );
        cache
    }
}

fn should_refresh(token: &AccessToken) -> bool {
    token.expires_on <= OffsetDateTime::now_utc() + Duration::seconds(300)
}

#[cfg(test)]
mod tests {
    use super::{
        ManagedIdentityCacheKey, ManagedIdentityTokenCache, SharedManagedIdentityCache, TokenCache,
    };
    use async_lock::Mutex;
    use azure_core::{
        credentials::{AccessToken, Secret, TokenRequestOptions},
        error::ErrorKind,
        http::{policies::TransportPolicy, ClientOptions, Transport, Url},
        time::{Duration, OffsetDateTime},
    };
    use futures::{channel::oneshot, future};
    use std::{sync::Arc, time::Duration as StdDuration};

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
            _: Option<TokenRequestOptions<'_>>,
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
        let expires_on = OffsetDateTime::now_utc() + Duration::seconds(3600);
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

    fn cache_key(resource: &str) -> ManagedIdentityCacheKey {
        let mut url: Url = "http://localhost/metadata/identity/oauth2/token"
            .parse()
            .unwrap();
        url.query_pairs_mut().append_pair("resource", resource);
        ManagedIdentityCacheKey {
            source: "imds",
            url,
            authentication_context: None,
        }
    }

    fn valid_token(value: &str) -> AccessToken {
        AccessToken::new(
            value.to_owned(),
            OffsetDateTime::now_utc() + Duration::hours(1),
        )
    }

    async fn populate(cache: &TokenCache, token: AccessToken) -> azure_core::Result<AccessToken> {
        cache
            .get_token(&[STORAGE_TOKEN_SCOPE], None, |_, _| {
                future::ready(Ok(token))
            })
            .await
    }

    #[tokio::test]
    async fn managed_identity_cache_survives_credential_drop() -> azure_core::Result<()> {
        let url = "http://localhost/cache-survives-credential-drop"
            .parse()
            .unwrap();
        {
            let credential = ManagedIdentityTokenCache::new("drop-test", &ClientOptions::default());
            populate(&credential.get(&url, None).unwrap(), valid_token("first")).await?;
        }

        let credential = ManagedIdentityTokenCache::new("drop-test", &ClientOptions::default());
        let token = populate(&credential.get(&url, None).unwrap(), valid_token("second")).await?;
        assert_eq!(token.token.secret(), "first");
        Ok(())
    }

    #[tokio::test]
    async fn managed_identity_cache_isolates_request_contexts() -> azure_core::Result<()> {
        let base = cache_key("storage");
        let mut keys = vec![base.clone()];
        let mut other_source = base.clone();
        other_source.source = "app-service";
        keys.push(other_source);
        let mut other_endpoint = base.clone();
        other_endpoint.url.set_host(Some("other-host")).unwrap();
        keys.push(other_endpoint);
        keys.push(cache_key("key-vault"));

        for (selector, value) in [
            ("client_id", "first"),
            ("client_id", "second"),
            ("object_id", "first"),
            ("msi_res_id", "first"),
            ("api-version", "different-version"),
        ] {
            let mut key = base.clone();
            key.url.query_pairs_mut().append_pair(selector, value);
            keys.push(key);
        }
        for (name, value) in [
            ("x-identity-header", "first-secret"),
            ("x-identity-header", "second-secret"),
            ("PROGRAMDATA", "first-directory"),
            ("PROGRAMDATA", "second-directory"),
        ] {
            let mut key = base.clone();
            key.authentication_context = Some((name.to_owned(), value.to_owned()));
            keys.push(key);
        }

        let mut shared = SharedManagedIdentityCache::new(keys.len());
        for (index, key) in keys.iter().enumerate() {
            let token = populate(&shared.get(key.clone()), valid_token(&index.to_string())).await?;
            assert_eq!(token.token.secret(), index.to_string());
        }
        for (index, key) in keys.into_iter().enumerate() {
            let token = populate(&shared.get(key), valid_token("unexpected acquisition")).await?;
            assert_eq!(token.token.secret(), index.to_string());
        }
        Ok(())
    }

    #[tokio::test]
    async fn custom_pipelines_keep_instance_caches() -> azure_core::Result<()> {
        let transport = Transport::default();
        let policy = Arc::new(TransportPolicy::new(transport.clone()));
        let options = [
            ClientOptions {
                transport: Some(transport),
                ..Default::default()
            },
            ClientOptions {
                per_call_policies: vec![policy.clone()],
                ..Default::default()
            },
            ClientOptions {
                per_try_policies: vec![policy],
                ..Default::default()
            },
        ];
        let url = "http://localhost/custom-pipeline-cache".parse().unwrap();
        for options in options {
            for value in ["first", "second"] {
                let credential = ManagedIdentityTokenCache::new("custom-pipeline-test", &options);
                let shared = credential.get(&url, None);
                let cache = shared.as_deref().unwrap_or(&credential.local);
                assert_eq!(
                    populate(cache, valid_token(value)).await?.token.secret(),
                    value
                );
                assert_eq!(
                    populate(cache, valid_token("unexpected acquisition"))
                        .await?
                        .token
                        .secret(),
                    value
                );
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn shared_cache_refreshes_and_does_not_cache_errors() -> azure_core::Result<()> {
        let mut shared = SharedManagedIdentityCache::new(2);
        let key = cache_key("refresh");
        populate(
            &shared.get(key.clone()),
            AccessToken::new(
                "refresh-due",
                OffsetDateTime::now_utc() + Duration::minutes(4),
            ),
        )
        .await?;

        shared
            .get(key.clone())
            .get_token(&[STORAGE_TOKEN_SCOPE], None, |_, _| {
                future::ready(Err(azure_core::Error::with_message(
                    ErrorKind::Credential,
                    "acquisition failed",
                )))
            })
            .await
            .expect_err("refresh errors must propagate");

        let token = populate(&shared.get(key.clone()), valid_token("refreshed")).await?;
        assert_eq!(token.token.secret(), "refreshed");
        let token = populate(&shared.get(key), valid_token("unexpected acquisition")).await?;
        assert_eq!(token.token.secret(), "refreshed");
        Ok(())
    }

    #[tokio::test]
    async fn shared_cache_coalesces_without_blocking_other_resources() -> azure_core::Result<()> {
        let mut shared = SharedManagedIdentityCache::new(2);
        let key = cache_key("pending");
        let first_cache = shared.get(key.clone());
        let (started, wait_started) = oneshot::channel();
        let (release, wait_release) = oneshot::channel();
        let first = tokio::spawn(async move {
            first_cache
                .get_token(&[STORAGE_TOKEN_SCOPE], None, |_, _| async {
                    started.send(()).unwrap();
                    wait_release.await.unwrap();
                    Ok(valid_token("first"))
                })
                .await
        });
        wait_started.await.unwrap();

        let second_cache = shared.get(key);
        let second = second_cache.get_token(&[STORAGE_TOKEN_SCOPE], None, |_, _| async {
            panic!("concurrent callers must reuse the first acquisition")
        });
        tokio::pin!(second);
        assert!(futures::poll!(&mut second).is_pending());

        let other = shared.get(cache_key("independent"));
        let token = tokio::time::timeout(
            StdDuration::from_secs(5),
            populate(&other, valid_token("independent")),
        )
        .await
        .expect("another resource must not wait for the pending acquisition")?;
        assert_eq!(token.token.secret(), "independent");

        release.send(()).unwrap();
        assert_eq!(first.await.unwrap()?.token.secret(), "first");
        assert_eq!(second.await?.token.secret(), "first");
        Ok(())
    }

    #[tokio::test]
    async fn shared_cache_prunes_expired_and_evicts_least_recently_used() -> azure_core::Result<()>
    {
        let mut shared = SharedManagedIdentityCache::new(2);
        let first = cache_key("first");
        let second = cache_key("second");
        let third = cache_key("third");
        populate(&shared.get(first.clone()), valid_token("first")).await?;
        populate(
            &shared.get(second.clone()),
            AccessToken::new("expired", OffsetDateTime::now_utc() - Duration::seconds(1)),
        )
        .await?;
        populate(&shared.get(third.clone()), valid_token("third")).await?;
        assert!(!shared.entries.contains_key(&second));

        assert_eq!(
            populate(&shared.get(first.clone()), valid_token("unexpected"))
                .await?
                .token
                .secret(),
            "first"
        );
        populate(&shared.get(second.clone()), valid_token("second")).await?;
        assert!(!shared.entries.contains_key(&third));
        assert!(shared.entries.contains_key(&first));
        assert!(shared.entries.contains_key(&second));
        assert_eq!(shared.entries.len(), 2);
        Ok(())
    }

    #[test]
    fn full_shared_cache_does_not_evict_in_use_entries() {
        let mut shared = SharedManagedIdentityCache::new(1);
        let key = cache_key("in-use");
        let active = shared.get(key.clone());
        let _uncached = shared.get(cache_key("overflow"));
        assert_eq!(shared.entries.len(), 1);
        assert!(Arc::ptr_eq(&active, &shared.get(key)));
    }
}
