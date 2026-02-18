// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account metadata cache for Cosmos DB driver.

use super::AsyncCache;
use crate::models::{AccountEndpoint, AccountProperties};
use std::sync::Arc;

/// Cache for Cosmos DB account metadata.
///
/// Stores account properties (regions, capabilities) keyed by account endpoint.
/// Uses single-pending-I/O semantics - concurrent requests for the same account
/// share one initialization future.
#[derive(Debug)]
pub(crate) struct AccountMetadataCache {
    cache: AsyncCache<AccountEndpoint, AccountProperties>,
}

impl AccountMetadataCache {
    /// Creates a new empty account metadata cache.
    pub(crate) fn new() -> Self {
        Self {
            cache: AsyncCache::new(),
        }
    }

    /// Gets account properties, fetching them if not cached.
    ///
    /// If the account is not in the cache, calls `fetch_fn` to retrieve
    /// the properties. Concurrent requests for the same endpoint share
    /// the same fetch operation.
    pub(crate) async fn get_or_fetch<F, Fut>(
        &self,
        endpoint: AccountEndpoint,
        fetch_fn: F,
    ) -> Arc<AccountProperties>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = AccountProperties>,
    {
        self.cache.get_or_insert_with(endpoint, fetch_fn).await
    }

    /// Gets cached account properties if available.
    ///
    /// Returns `None` if the account is not in the cache.
    pub(crate) async fn get(&self, endpoint: &AccountEndpoint) -> Option<Arc<AccountProperties>> {
        self.cache.get(endpoint).await
    }

    /// Invalidates the cached properties for an account.
    ///
    /// Returns the previously cached value if it existed.
    pub(crate) async fn invalidate(
        &self,
        endpoint: &AccountEndpoint,
    ) -> Option<Arc<AccountProperties>> {
        self.cache.invalidate(endpoint).await
    }

    /// Clears all cached account metadata.
    pub(crate) async fn clear(&self) {
        self.cache.clear().await;
    }
}

impl Default for AccountMetadataCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::Region;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use url::Url;

    fn test_endpoint(name: &str) -> AccountEndpoint {
        AccountEndpoint::from(
            Url::parse(&format!("https://{name}.documents.azure.com:443/")).unwrap(),
        )
    }

    fn test_properties(region_name: &str) -> AccountProperties {
        AccountProperties::new(
            Region::new(region_name.to_owned()),
            vec![Region::new(region_name.to_owned())],
        )
    }

    #[tokio::test]
    async fn caches_account_properties() {
        let cache = AccountMetadataCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let endpoint = test_endpoint("myaccount");

        let counter_clone = counter.clone();
        let props = cache
            .get_or_fetch(endpoint.clone(), || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                test_properties("westus")
            })
            .await;

        assert_eq!(props.write_region.as_str(), "westus");
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Second access uses cached value
        let counter_clone = counter.clone();
        let props2 = cache
            .get_or_fetch(endpoint, || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                test_properties("eastus")
            })
            .await;

        assert_eq!(props2.write_region.as_str(), "westus");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn different_accounts_cached_separately() {
        let cache = AccountMetadataCache::new();

        let props1 = cache
            .get_or_fetch(test_endpoint("account1"), || async {
                test_properties("westus")
            })
            .await;

        let props2 = cache
            .get_or_fetch(test_endpoint("account2"), || async {
                test_properties("eastus")
            })
            .await;

        assert_eq!(props1.write_region.as_str(), "westus");
        assert_eq!(props2.write_region.as_str(), "eastus");
    }

    #[tokio::test]
    async fn get_returns_none_before_fetch() {
        let cache = AccountMetadataCache::new();
        assert!(cache.get(&test_endpoint("unknown")).await.is_none());
    }

    #[tokio::test]
    async fn invalidate_removes_entry() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint("myaccount");

        cache
            .get_or_fetch(endpoint.clone(), || async { test_properties("westus") })
            .await;

        let removed = cache.invalidate(&endpoint).await;
        assert!(removed.is_some());
        assert!(cache.get(&endpoint).await.is_none());
    }

    #[tokio::test]
    async fn clear_removes_all() {
        let cache = AccountMetadataCache::new();

        cache
            .get_or_fetch(test_endpoint("account1"), || async {
                test_properties("westus")
            })
            .await;
        cache
            .get_or_fetch(test_endpoint("account2"), || async {
                test_properties("eastus")
            })
            .await;

        cache.clear().await;

        assert!(cache.get(&test_endpoint("account1")).await.is_none());
        assert!(cache.get(&test_endpoint("account2")).await.is_none());
    }
}
