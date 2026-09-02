// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dual container cache for Cosmos DB driver.
//!
//! Maintains two lookup indices — by name and by RID — so that a resolved
//! [`ContainerReference`] can be retrieved efficiently regardless of which
//! identifier the caller has. Name-addressed references are cross-populated
//! into both indices to keep them in sync; RID-addressed references are stored
//! only in the by-RID index, since they carry no database name to form a safe
//! by-name key.

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
    /// Builds a name key from a container reference, or `None` if the container
    /// was addressed by RID (no database name is available to key on).
    fn from_container(c: &ContainerReference) -> Option<Self> {
        Some(Self {
            account_endpoint: c.account().endpoint().as_str().to_owned(),
            db_name: c.database_name()?.to_owned(),
            container_name: c.name().to_owned(),
        })
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
/// (`account + container_rid`). A name-addressed reference fetched or inserted
/// via either index is cross-populated into the other automatically, with the
/// by-RID copy re-addressed by RID so each index always hands back a reference
/// matching how it was looked up; a RID-addressed reference is stored only in
/// the by-RID index, because it has no database name to form a safe by-name key.
///
/// Uses single-pending-I/O semantics per key — concurrent requests for the
/// same container share one fetch operation.
#[derive(Debug)]
pub(crate) struct ContainerCache {
    by_name: AsyncCache<ContainerNameKey, crate::error::Result<ContainerReference>>,
    by_rid: AsyncCache<ContainerRidKey, crate::error::Result<ContainerReference>>,
    refreshes: AsyncCache<ContainerNameKey, crate::error::Result<ContainerReference>>,
}

impl ContainerCache {
    /// Creates a new empty dual container cache.
    pub(crate) fn new() -> Self {
        Self {
            by_name: AsyncCache::new(),
            by_rid: AsyncCache::new(),
            refreshes: AsyncCache::new(),
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
    ) -> crate::error::Result<Arc<ContainerReference>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<ContainerReference>>,
    {
        let key = ContainerNameKey {
            account_endpoint: account_endpoint.to_owned(),
            db_name: db_name.to_owned(),
            container_name: container_name.to_owned(),
        };
        self.get_or_fetch_impl(&self.by_name, key, fetch_fn).await
    }

    /// Refreshes a name-addressed container if the cache still contains the
    /// caller's observed RID.
    ///
    /// Concurrent refreshes share one service request while the previous value
    /// remains readable. If another caller already installed a newer RID, that
    /// value wins without invoking `fetch_fn`. A failed refresh leaves the
    /// previous valid entry untouched.
    pub(crate) async fn refresh_by_name_if_same<F, Fut>(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
        observed_rid: &str,
        fetch_fn: F,
    ) -> crate::error::Result<Arc<ContainerReference>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<ContainerReference>>,
    {
        let key = ContainerNameKey {
            account_endpoint: account_endpoint.to_owned(),
            db_name: db_name.to_owned(),
            container_name: container_name.to_owned(),
        };
        let previous = self.by_name.get(&key).await;
        if let Some(Ok(container)) = previous.as_deref() {
            if container.rid() != observed_rid {
                return Ok(Arc::new(container.clone()));
            }
        }

        let resolved = self
            .refreshes
            .get_or_insert_with(key.clone(), fetch_fn)
            .await;

        let result = match resolved.as_ref() {
            Ok(container) => {
                let current = self.by_name.get(&key).await;
                if let Some(Ok(current)) = current.as_deref() {
                    if current.rid() != observed_rid {
                        self.refreshes.invalidate_if_same(&key, &resolved).await;
                        return Ok(Arc::new(current.clone()));
                    }
                }

                let replacement = Arc::new(Ok(container.clone()));
                let replaced = if let Some(current) = current.as_ref() {
                    self.by_name
                        .replace_if_same(&key, current, replacement)
                        .await
                } else {
                    false
                };
                if replaced {
                    self.put(container.clone()).await;
                    Ok(Arc::new(container.clone()))
                } else {
                    let container = container.clone();
                    self.get_or_fetch_impl(&self.by_name, key.clone(), || async { Ok(container) })
                        .await
                }
            }
            Err(error) => Err(error.clone()),
        };
        self.refreshes.invalidate_if_same(&key, &resolved).await;
        result
    }

    /// Looks up a container by RID, fetching if not cached.
    ///
    /// On a cache miss, calls `fetch_fn` to resolve the container from the
    /// service. The resolved (RID-addressed) reference is populated into the
    /// by-RID cache; the by-name cache is left untouched because a RID-addressed
    /// reference has no database name to key on.
    pub(crate) async fn get_or_fetch_by_rid<F, Fut>(
        &self,
        account_endpoint: &str,
        container_rid: &str,
        fetch_fn: F,
    ) -> crate::error::Result<Arc<ContainerReference>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<ContainerReference>>,
    {
        let key = ContainerRidKey {
            account_endpoint: account_endpoint.to_owned(),
            container_rid: container_rid.to_owned(),
        };
        self.get_or_fetch_impl(&self.by_rid, key, fetch_fn).await
    }

    /// Returns a cached container looked up by name, or `None` if not cached.
    pub(crate) async fn get_by_name(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
    ) -> Option<Arc<ContainerReference>> {
        let key = ContainerNameKey {
            account_endpoint: account_endpoint.to_owned(),
            db_name: db_name.to_owned(),
            container_name: container_name.to_owned(),
        };
        self.get_from(&self.by_name, &key).await
    }

    /// Returns a cached container looked up by RID, or `None` if not cached.
    #[allow(dead_code)] // Used in tests; will be called from production code once lookup-by-RID is wired up.
    pub(crate) async fn get_by_rid(
        &self,
        account_endpoint: &str,
        container_rid: &str,
    ) -> Option<Arc<ContainerReference>> {
        let key = ContainerRidKey {
            account_endpoint: account_endpoint.to_owned(),
            container_rid: container_rid.to_owned(),
        };
        self.get_from(&self.by_rid, &key).await
    }

    /// Core fetch-or-lookup logic shared by both key variants.
    ///
    /// Checks the cache for an existing entry, calls the factory on a miss,
    /// cross-populates on success, and invalidates on error.
    async fn get_or_fetch_impl<K, F, Fut>(
        &self,
        cache: &AsyncCache<K, crate::error::Result<ContainerReference>>,
        key: K,
        fetch_fn: F,
    ) -> crate::error::Result<Arc<ContainerReference>>
    where
        K: Eq + std::hash::Hash + Clone,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<ContainerReference>>,
    {
        if let Some(cached) = self.get_from(cache, &key).await {
            return Ok(cached);
        }

        let resolved = cache.get_or_insert_with(key.clone(), fetch_fn).await;

        match resolved.as_ref() {
            Ok(container) => {
                self.put(container.clone()).await;
                Ok(Arc::new(container.clone()))
            }
            Err(error) => {
                cache.invalidate(&key).await;
                // The cached `crate::error::CosmosError` is `Clone` (cheap Arc
                // refcount bump), so the typed payload propagates directly.
                Err(error.clone())
            }
        }
    }

    /// Reads a cached value from one of the underlying caches.
    async fn get_from<K>(
        &self,
        cache: &AsyncCache<K, crate::error::Result<ContainerReference>>,
        key: &K,
    ) -> Option<Arc<ContainerReference>>
    where
        K: Eq + std::hash::Hash + Clone,
    {
        cache
            .get(key)
            .await
            .and_then(|entry| entry.as_ref().as_ref().ok().map(|c| Arc::new(c.clone())))
    }

    /// Inserts a known-resolved container reference into both caches.
    ///
    /// If an entry already exists under either key, the existing entry is
    /// preserved (first-write-wins). RID-addressed references are inserted only
    /// into the by-RID cache, since they carry no database name to key on.
    ///
    /// The by-RID index always stores a RID-addressed reference, so a by-RID
    /// lookup that hits an entry cross-populated by a name-based resolution
    /// still yields RID-based paths and signing rather than silently falling
    /// back to name addressing.
    pub(crate) async fn put(&self, container: ContainerReference) {
        let name_key = ContainerNameKey::from_container(&container);
        let rid_key = ContainerRidKey::from_container(&container);

        if let Some(name_key) = name_key {
            let container_for_name = container.clone();
            self.by_name
                .get_or_insert_with(name_key, || async { Ok(container_for_name) })
                .await;
        }
        let container_for_rid = container.into_rid_addressed();
        self.by_rid
            .get_or_insert_with(rid_key, || async { Ok(container_for_rid) })
            .await;
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
        SystemProperties,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::{
        sync::Notify,
        time::{sleep, Duration},
    };
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    const ACCOUNT_ENDPOINT: &str = "https://myaccount.documents.azure.com/";

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container(db: &str, container: &str) -> ContainerReference {
        test_container_with_rid(db, container, &format!("{db}_{container}_rid"))
    }

    fn test_container_with_rid(
        db: &str,
        container: &str,
        container_rid: &str,
    ) -> ContainerReference {
        ContainerReference::new(
            test_account(),
            db.to_owned(),
            format!("{db}_rid"),
            container.to_owned(),
            container_rid.to_owned(),
            &test_container_props(),
        )
    }

    fn test_container_by_rid(db_rid: &str, container_rid: &str) -> ContainerReference {
        ContainerReference::new_by_rid(
            test_account(),
            db_rid.to_owned(),
            "testcontainer".to_owned(),
            container_rid.to_owned(),
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

    #[tokio::test]
    async fn refresh_by_name_replaces_only_observed_generation() {
        let cache = ContainerCache::new();
        let old = test_container_with_rid("mydb", "mycoll", "old_rid");
        let replacement = test_container_with_rid("mydb", "mycoll", "new_rid");
        cache.put(old).await;

        let resolved = cache
            .refresh_by_name_if_same(ACCOUNT_ENDPOINT, "mydb", "mycoll", "old_rid", || async {
                Ok(replacement)
            })
            .await
            .expect("refresh should succeed");

        assert_eq!(resolved.rid(), "new_rid");
        assert_eq!(
            cache
                .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
                .await
                .expect("name entry should remain cached")
                .rid(),
            "new_rid",
        );
        assert!(cache
            .get_by_rid(ACCOUNT_ENDPOINT, "old_rid")
            .await
            .is_some());
        assert!(cache
            .get_by_rid(ACCOUNT_ENDPOINT, "new_rid")
            .await
            .is_some());
    }

    #[tokio::test]
    async fn refresh_by_name_preserves_newer_winner() {
        let cache = ContainerCache::new();
        cache
            .put(test_container_with_rid("mydb", "mycoll", "new_rid"))
            .await;
        let calls = AtomicUsize::new(0);

        let resolved = cache
            .refresh_by_name_if_same(ACCOUNT_ENDPOINT, "mydb", "mycoll", "old_rid", || async {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(test_container_with_rid("mydb", "mycoll", "other_rid"))
            })
            .await
            .expect("newer winner should be returned");

        assert_eq!(resolved.rid(), "new_rid");
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn failed_refresh_restores_observed_generation() {
        let cache = ContainerCache::new();
        cache
            .put(test_container_with_rid("mydb", "mycoll", "old_rid"))
            .await;

        let error = crate::error::CosmosError::builder()
            .with_message("refresh failed")
            .build();
        let result = cache
            .refresh_by_name_if_same(ACCOUNT_ENDPOINT, "mydb", "mycoll", "old_rid", || async {
                Err(error)
            })
            .await;

        assert!(result.is_err());
        assert_eq!(
            cache
                .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
                .await
                .expect("old generation should be restored")
                .rid(),
            "old_rid",
        );
    }

    #[tokio::test]
    async fn failed_concurrent_refresh_keeps_observed_generation_readable() {
        let cache = Arc::new(ContainerCache::new());
        cache
            .put(test_container_with_rid("mydb", "mycoll", "old_rid"))
            .await;
        let calls = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());

        let owner = {
            let cache = Arc::clone(&cache);
            let calls = Arc::clone(&calls);
            let started = Arc::clone(&started);
            let release = Arc::clone(&release);
            tokio::spawn(async move {
                cache
                    .refresh_by_name_if_same(
                        ACCOUNT_ENDPOINT,
                        "mydb",
                        "mycoll",
                        "old_rid",
                        || async move {
                            calls.fetch_add(1, Ordering::SeqCst);
                            started.notify_one();
                            release.notified().await;
                            Err(crate::error::CosmosError::builder()
                                .with_message("refresh failed")
                                .build())
                        },
                    )
                    .await
            })
        };

        started.notified().await;
        assert_eq!(
            cache
                .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
                .await
                .expect("old generation should remain readable during refresh")
                .rid(),
            "old_rid",
        );

        let waiter = {
            let cache = Arc::clone(&cache);
            let calls = Arc::clone(&calls);
            tokio::spawn(async move {
                cache
                    .refresh_by_name_if_same(
                        ACCOUNT_ENDPOINT,
                        "mydb",
                        "mycoll",
                        "old_rid",
                        || async move {
                            calls.fetch_add(1, Ordering::SeqCst);
                            Err(crate::error::CosmosError::builder()
                                .with_message("unexpected second refresh")
                                .build())
                        },
                    )
                    .await
            })
        };
        sleep(Duration::from_millis(10)).await;
        release.notify_one();

        assert!(owner.await.unwrap().is_err());
        assert!(waiter.await.unwrap().is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            cache
                .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
                .await
                .expect("old generation should remain cached after refresh failure")
                .rid(),
            "old_rid",
        );
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

    // --- get_or_fetch_by_rid ---

    #[tokio::test]
    async fn fetch_by_rid_caches_without_name_index() {
        let cache = ContainerCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let container = test_container_by_rid("db_rid", "coll_rid");
        let container_clone = container.clone();
        let counter_clone = counter.clone();

        let resolved = cache
            .get_or_fetch_by_rid(ACCOUNT_ENDPOINT, "coll_rid", || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(container_clone)
            })
            .await
            .unwrap();

        assert!(resolved.is_by_rid());
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Retrievable by RID.
        assert!(cache
            .get_by_rid(ACCOUNT_ENDPOINT, "coll_rid")
            .await
            .is_some());

        // A second fetch is served from cache, not the factory.
        let counter2 = counter.clone();
        cache
            .get_or_fetch_by_rid(ACCOUNT_ENDPOINT, "coll_rid", || async move {
                counter2.fetch_add(1, Ordering::SeqCst);
                Ok(test_container_by_rid("db_rid", "coll_rid"))
            })
            .await
            .unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn rid_lookup_after_name_resolution_stays_rid_addressed() {
        // Resolving by name cross-populates the by-RID index. A later by-RID
        // lookup must still yield a RID-addressed reference, otherwise the
        // caller silently falls back to name-based paths and signing despite
        // having explicitly addressed the container by RID.
        let cache = ContainerCache::new();
        let container = test_container("mydb", "mycoll");
        let rid = container.rid().to_owned();

        let by_name = cache
            .get_or_fetch_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll", || async move {
                Ok(container)
            })
            .await
            .unwrap();
        assert!(!by_name.is_by_rid());
        assert_eq!(by_name.database_name(), Some("mydb"));

        let by_rid = cache
            .get_or_fetch_by_rid(ACCOUNT_ENDPOINT, &rid, || async {
                panic!("by-RID lookup should be served from the cross-populated cache")
            })
            .await
            .unwrap();

        assert!(by_rid.is_by_rid());
        assert_eq!(by_rid.database_name(), None);
        assert_eq!(by_rid.name_based_path(), None);
        assert_eq!(by_rid.base_path(), by_rid.rid_based_path());
        // The name-addressed entry is untouched by the re-addressing.
        assert_eq!(
            cache
                .get_by_name(ACCOUNT_ENDPOINT, "mydb", "mycoll")
                .await
                .unwrap()
                .database_name(),
            Some("mydb")
        );
    }

    #[tokio::test]
    async fn put_rid_only_skips_name_cache() {
        let cache = ContainerCache::new();
        let container = test_container_by_rid("db_rid", "coll_rid");

        cache.put(container).await;

        // The by-RID index is populated, the by-name index is not (no db name).
        assert!(cache
            .get_by_rid(ACCOUNT_ENDPOINT, "coll_rid")
            .await
            .is_some());
        assert!(cache
            .get_by_name(ACCOUNT_ENDPOINT, "db_rid", "testcontainer")
            .await
            .is_none());
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

        assert_eq!(r1.database_name(), Some("db1"));
        assert_eq!(r2.database_name(), Some("db2"));
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

    // --- clear ---

    #[tokio::test]
    async fn clear_removes_all() {
        let cache = ContainerCache::new();

        cache.put(test_container("db", "coll1")).await;
        cache.put(test_container("db", "coll2")).await;

        cache.by_name.clear().await;
        cache.by_rid.clear().await;
        cache.refreshes.clear().await;

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
