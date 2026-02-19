// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dual container cache for Cosmos DB driver.
//!
//! Maintains two lookup indices — by name and by RID — so that a resolved
//! [`ContainerReference`] can be retrieved efficiently regardless of which
//! identifier the caller has. When a reference is fetched or inserted,
//! both caches are cross-populated to keep them in sync.

use super::AsyncCache;
use crate::models::ContainerReference;
use std::sync::Arc;

// =============================================================================
// Cache key types
// =============================================================================

/// Key for looking up a container by user-provided names.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ContainerNameKey {
    account_endpoint: String,
    db_name: String,
    container_name: String,
}

impl ContainerNameKey {
    fn from_container(c: &ContainerReference) -> Self {
        Self {
            account_endpoint: c.account().endpoint().as_str().to_owned(),
            db_name: c.database_name().to_owned(),
            container_name: c.name().to_owned(),
        }
    }
}

/// Key for looking up a container by its internal RID.
///
/// Container RIDs are unique within a Cosmos DB account, so the
/// database RID is not required in the key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ContainerRidKey {
    account_endpoint: String,
    container_rid: String,
}

impl ContainerRidKey {
    fn from_container(c: &ContainerReference) -> Self {
        Self {
            account_endpoint: c.account().endpoint().as_str().to_owned(),
            container_rid: c.rid().to_owned(),
        }
    }
}

// =============================================================================
// ContainerCache
// =============================================================================

/// Dual-index cache for resolved Cosmos DB container references.
///
/// Stores fully-resolved [`ContainerReference`] values and indexes them by
/// both name (`account + db_name + container_name`) and RID
/// (`account + container_rid`). When a reference is fetched or inserted via
/// either index, the other index is cross-populated automatically.
///
/// Uses single-pending-I/O semantics per key — concurrent requests for the
/// same container share one fetch operation.
#[derive(Debug)]
pub(crate) struct ContainerCache {
    by_name: AsyncCache<ContainerNameKey, azure_core::Result<ContainerReference>>,
    by_rid: AsyncCache<ContainerRidKey, azure_core::Result<ContainerReference>>,
}

impl ContainerCache {
    /// Creates a new empty dual container cache.
    pub(crate) fn new() -> Self {
        Self {
            by_name: AsyncCache::new(),
            by_rid: AsyncCache::new(),
        }
    }

    /// Looks up a container by name, fetching if not cached.
    ///
    /// On a cache miss, calls `fetch_fn` to resolve the container from the
    /// service. The resolved reference is then populated into both the
    /// by-name and by-RID caches.
    pub(crate) async fn get_or_fetch_by_name<F, Fut>(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
        fetch_fn: F,
    ) -> azure_core::Result<Arc<ContainerReference>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<ContainerReference>>,
    {
        if let Some(cached) = self
            .get_by_name(account_endpoint, db_name, container_name)
            .await
        {
            return Ok(cached);
        }

        let name_key = ContainerNameKey {
            account_endpoint: account_endpoint.to_owned(),
            db_name: db_name.to_owned(),
            container_name: container_name.to_owned(),
        };

        let resolved = self
            .by_name
            .get_or_insert_with(name_key.clone(), fetch_fn)
            .await;

        match resolved.as_ref() {
            Ok(container) => {
                self.put(container.clone()).await;
                Ok(Arc::new(container.clone()))
            }
            Err(error) => {
                self.by_name.invalidate(&name_key).await;
                Err(azure_core::Error::with_message(
                    error.kind().clone(),
                    error.to_string(),
                ))
            }
        }
    }

    /// Looks up a container by RID, fetching if not cached.
    ///
    /// On a cache miss, calls `fetch_fn` to resolve the container from the
    /// service. The resolved reference is then cross-populated into the
    /// by-name cache. Concurrent requests for the same RID share one fetch.
    #[cfg(test)]
    pub(crate) async fn get_or_fetch_by_rid<F, Fut>(
        &self,
        account_endpoint: &str,
        container_rid: &str,
        fetch_fn: F,
    ) -> azure_core::Result<Arc<ContainerReference>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<ContainerReference>>,
    {
        if let Some(cached) = self.get_by_rid(account_endpoint, container_rid).await {
            return Ok(cached);
        }

        let rid_key = ContainerRidKey {
            account_endpoint: account_endpoint.to_owned(),
            container_rid: container_rid.to_owned(),
        };

        let resolved = self
            .by_rid
            .get_or_insert_with(rid_key.clone(), fetch_fn)
            .await;

        match resolved.as_ref() {
            Ok(container) => {
                self.put(container.clone()).await;
                Ok(Arc::new(container.clone()))
            }
            Err(error) => {
                self.by_rid.invalidate(&rid_key).await;
                Err(azure_core::Error::with_message(
                    error.kind().clone(),
                    error.to_string(),
                ))
            }
        }
    }

    /// Returns a cached container looked up by name, or `None` if not cached.
    pub(crate) async fn get_by_name(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
    ) -> Option<Arc<ContainerReference>> {
        let name_key = ContainerNameKey {
            account_endpoint: account_endpoint.to_owned(),
            db_name: db_name.to_owned(),
            container_name: container_name.to_owned(),
        };
        self.by_name
            .get(&name_key)
            .await
            .and_then(|entry| entry.as_ref().as_ref().ok().map(|c| Arc::new(c.clone())))
    }

    /// Returns a cached container looked up by RID, or `None` if not cached.
    #[cfg(test)]
    pub(crate) async fn get_by_rid(
        &self,
        account_endpoint: &str,
        container_rid: &str,
    ) -> Option<Arc<ContainerReference>> {
        let rid_key = ContainerRidKey {
            account_endpoint: account_endpoint.to_owned(),
            container_rid: container_rid.to_owned(),
        };
        self.by_rid
            .get(&rid_key)
            .await
            .and_then(|entry| entry.as_ref().as_ref().ok().map(|c| Arc::new(c.clone())))
    }

    /// Inserts a known-resolved container reference into both caches.
    ///
    /// If an entry already exists under either key, the existing entry is
    /// preserved (first-write-wins).
    pub(crate) async fn put(&self, container: ContainerReference) {
        let name_key = ContainerNameKey::from_container(&container);
        let rid_key = ContainerRidKey::from_container(&container);
        let container_for_rid = container.clone();

        self.by_name
            .get_or_insert_with(name_key, || async { Ok(container) })
            .await;
        self.by_rid
            .get_or_insert_with(rid_key, || async { Ok(container_for_rid) })
            .await;
    }

    /// Invalidates a container from both caches.
    pub(crate) async fn invalidate(&self, container: &ContainerReference) {
        let name_key = ContainerNameKey::from_container(container);
        let rid_key = ContainerRidKey::from_container(container);
        self.by_name.invalidate(&name_key).await;
        self.by_rid.invalidate(&rid_key).await;
    }

    /// Clears all cached container metadata from both indices.
    #[cfg(test)]
    pub(crate) async fn clear(&self) {
        self.by_name.clear().await;
        self.by_rid.clear().await;
    }
}

impl Default for ContainerCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, PartitionKeyDefinition,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    const ACCOUNT_ENDPOINT: &str = "https://myaccount.documents.azure.com/";

    fn test_container_props() -> ContainerProperties {
        ContainerProperties::new("testcontainer", PartitionKeyDefinition::new(["/pk"]))
    }

    fn test_container(db: &str, container: &str) -> ContainerReference {
        ContainerReference::new(
            test_account(),
            db.to_owned(),
            format!("{db}_rid"),
            container.to_owned(),
            format!("{db}_{container}_rid"),
            &test_container_props(),
        )
    }

    // --- get_or_fetch_by_name ---

    #[tokio::test]
    async fn fetch_by_name_caches_and_cross_populates_rid() {
        let cache = ContainerCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let container = test_container("mydb", "mycoll");
        let container_clone = container.clone();
        let counter_clone = counter.clone();

        let resolved = cache
            .get_or_fetch_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll", || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(container_clone)
            })
            .await
            .unwrap();

        assert_eq!(resolved.name(), "mycoll");
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Should be retrievable by name
        let by_name = cache.get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll").await;
        assert!(by_name.is_some());
        assert_eq!(by_name.unwrap().name(), "mycoll");

        // Should be cross-populated and retrievable by RID
        let by_rid = cache.get_by_rid(ACCOUNT_ENDPOINT, container.rid()).await;
        assert!(by_rid.is_some());
        assert_eq!(by_rid.unwrap().name(), "mycoll");
    }

    #[tokio::test]
    async fn fetch_by_name_deduplicates() {
        let cache = ContainerCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let container = test_container("mydb", "mycoll");

        let c1 = container.clone();
        let counter1 = counter.clone();
        cache
            .get_or_fetch_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll", || async move {
                counter1.fetch_add(1, Ordering::SeqCst);
                Ok(c1)
            })
            .await
            .unwrap();

        // Second fetch should use cache, not call factory
        let c2 = container.clone();
        let counter2 = counter.clone();
        let resolved = cache
            .get_or_fetch_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll", || async move {
                counter2.fetch_add(1, Ordering::SeqCst);
                Ok(c2)
            })
            .await
            .unwrap();

        assert_eq!(resolved.name(), "mycoll");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    // --- get_or_fetch_by_rid ---

    #[tokio::test]
    async fn fetch_by_rid_caches_and_cross_populates_name() {
        let cache = ContainerCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let container = test_container("mydb", "mycoll");
        let container_rid = container.rid().to_owned();
        let container_clone = container.clone();
        let counter_clone = counter.clone();

        let resolved = cache
            .get_or_fetch_by_rid(ACCOUNT_ENDPOINT, &container_rid, || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(container_clone)
            })
            .await
            .unwrap();

        assert_eq!(resolved.name(), "mycoll");
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Should be cross-populated and retrievable by name
        let by_name = cache.get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll").await;
        assert!(by_name.is_some());
        assert_eq!(by_name.unwrap().rid(), container_rid);

        // Should also be retrievable by RID
        let by_rid = cache.get_by_rid(ACCOUNT_ENDPOINT, &container_rid).await;
        assert!(by_rid.is_some());
    }

    // --- put ---

    #[tokio::test]
    async fn put_populates_both_caches() {
        let cache = ContainerCache::new();
        let container = test_container("mydb", "mycoll");
        let rid = container.rid().to_owned();

        cache.put(container).await;

        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
            .await
            .is_some());
        assert!(cache.get_by_rid(ACCOUNT_ENDPOINT, &rid).await.is_some());
    }

    // --- different containers ---

    #[tokio::test]
    async fn different_containers_cached_separately() {
        let cache = ContainerCache::new();

        let c1 = test_container("db1", "coll1");
        let c2 = test_container("db1", "coll2");

        cache.put(c1).await;
        cache.put(c2).await;

        let r1 = cache
            .get_by_name(ACCOUNT_ENDPOINT, "db1", "coll1")
            .await
            .unwrap();
        let r2 = cache
            .get_by_name(ACCOUNT_ENDPOINT, "db1", "coll2")
            .await
            .unwrap();

        assert_eq!(r1.name(), "coll1");
        assert_eq!(r2.name(), "coll2");
    }

    #[tokio::test]
    async fn same_container_different_databases() {
        let cache = ContainerCache::new();

        let c1 = test_container("db1", "coll");
        let c2 = test_container("db2", "coll");

        cache.put(c1).await;
        cache.put(c2).await;

        let r1 = cache
            .get_by_name(ACCOUNT_ENDPOINT, "db1", "coll")
            .await
            .unwrap();
        let r2 = cache
            .get_by_name(ACCOUNT_ENDPOINT, "db2", "coll")
            .await
            .unwrap();

        assert_eq!(r1.database_name(), "db1");
        assert_eq!(r2.database_name(), "db2");
    }

    // --- get returns none ---

    #[tokio::test]
    async fn get_by_name_returns_none_before_fetch() {
        let cache = ContainerCache::new();
        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "db", "unknown")
            .await
            .is_none());
    }

    #[tokio::test]
    async fn get_by_rid_returns_none_before_fetch() {
        let cache = ContainerCache::new();
        assert!(cache
            .get_by_rid(ACCOUNT_ENDPOINT, "unknown_rid")
            .await
            .is_none());
    }

    // --- invalidation ---

    #[tokio::test]
    async fn invalidate_removes_from_both_caches() {
        let cache = ContainerCache::new();
        let container = test_container("mydb", "mycoll");
        let rid = container.rid().to_owned();

        cache.put(container.clone()).await;

        // Verify present in both
        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
            .await
            .is_some());
        assert!(cache.get_by_rid(ACCOUNT_ENDPOINT, &rid).await.is_some());

        // Invalidate
        cache.invalidate(&container).await;

        // Both should be gone
        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
            .await
            .is_none());
        assert!(cache.get_by_rid(ACCOUNT_ENDPOINT, &rid).await.is_none());
    }

    // --- clear ---

    #[tokio::test]
    async fn clear_removes_all() {
        let cache = ContainerCache::new();

        cache.put(test_container("db", "coll1")).await;
        cache.put(test_container("db", "coll2")).await;

        cache.clear().await;

        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "db", "coll1")
            .await
            .is_none());
        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "db", "coll2")
            .await
            .is_none());
    }
}
