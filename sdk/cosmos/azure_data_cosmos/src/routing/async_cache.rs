use async_lock::RwLock;
use azure_core::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Instant;

/// Cache entry with optional TTL tracking
#[derive(Clone, Debug)]
struct CacheEntry<V> {
    value: V,
    expires_at: Option<Instant>,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Option<Duration>) -> Self {
        Self {
            value,
            expires_at: ttl.map(|d| Instant::now() + d),
        }
    }

    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| Instant::now() >= exp)
            .unwrap_or(false)
    }
}

/// A generic async cache with optional TTL support.
///
/// When created with `new()`, entries expire after the specified TTL.
#[derive(Clone)]
pub(crate) struct AsyncCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    ttl: Option<Duration>,
}

impl<K, V> AsyncCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new `AsyncCache` with an optional TTL.
    ///
    /// # Arguments
    /// * `ttl` - Optional time-to-live for cache entries. If `None`, entries never expire.
    pub fn new(ttl: Option<Duration>) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    /// Gets a value from the cache, or computes it using the provided async function if not present or expired
    ///
    /// When the entry is expired or a force_refresh is requested, the cache is automatically updated
    /// with the newly computed value.
    ///
    /// # Arguments
    /// * `key` - The cache key to look up
    /// * `should_refresh` - Callback function that receives the cached value (if any) and returns true
    ///   if the cache should be refreshed. Receives `Some(&V)` if there's a cached value, or `None` if not.
    /// * `compute` - Async function to compute the value if not cached or refresh is requested
    pub async fn get<F, Fut, E, R>(&self, key: K, should_refresh: R, compute: F) -> Result<V, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, E>>,
        R: FnOnce(Option<&V>) -> bool,
    {
        // First, check what's in the cache and determine if we need to refresh
        let (cached_value, force_refresh) = {
            let store = self.store.read().await;
            match store.get(&key) {
                Some(entry) if !entry.is_expired() => (
                    Some(entry.value.clone()),
                    should_refresh(Some(&entry.value)),
                ),
                Some(entry) => {
                    // Entry exists but is expired - still pass it to should_refresh for inspection
                    (None, should_refresh(Some(&entry.value)))
                }
                None => (None, should_refresh(None)),
            }
        };

        // Fast path: return cached value if it exists and no refresh is needed
        if !force_refresh {
            if let Some(value) = cached_value {
                return Ok(value);
            }
        }

        // Slow path: value missing, expired, or force refresh requested - need to compute (write lock)
        let mut store = self.store.write().await;

        // Double-check after acquiring write lock (another task might have updated it)
        // Only skip recompute if not force_refresh and entry is still valid
        if !force_refresh {
            if let Some(entry) = store.get(&key) {
                if entry.is_expired() {
                    // Remove the entry from the cache.
                    store.remove(&key);
                } else {
                    // Another task updated it while we waited for the lock
                    return Ok(entry.value.clone());
                }
            }
        } else {
            // force_refresh is true, remove the existing entry to ensure fresh computation
            store.remove(&key);
        }

        // Compute new value
        let value = compute().await?;

        // Update cache with new value
        let entry = CacheEntry::new(value.clone(), self.ttl);
        store.insert(key, entry);

        Ok(value)
    }

    /// Inserts a value directly into the cache.
    pub async fn insert(&self, key: K, value: V) {
        let mut store = self.store.write().await;
        let entry = CacheEntry::new(value, self.ttl);
        store.insert(key, entry);
    }

    /// Removes a value from the cache
    /// Returns the removed value if it existed and wasn't expired
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut store = self.store.write().await;

        if let Some(entry) = store.remove(key) {
            if !entry.is_expired() {
                return Some(entry.value);
            }
        }

        None
    }
}

impl<K, V> std::fmt::Debug for AsyncCache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncCache")
            .field("ttl", &self.ttl)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn get_and_compute() {
        let cache = AsyncCache::new(Some(Duration::seconds(60)));

        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        // First get - should compute
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value1".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Second get - should return cached value
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value2".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Not incremented
    }

    #[tokio::test]
    async fn key_expiration() {
        let cache = AsyncCache::new(Some(Duration::seconds(60)));

        // Add entry
        cache
            .get(
                "key1".to_string(),
                |_| false,
                || async { Ok::<String, &str>("value1".to_string()) },
            )
            .await
            .unwrap();

        // Manually expire the cache entry by setting expires_at to a past time
        {
            let mut store = cache.store.write().await;
            if let Some(entry) = store.get_mut(&"key1".to_string()) {
                entry.expires_at = Some(Instant::now() - Duration::seconds(1));
            }
        }

        // Get again - should recompute after expiration
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async { Ok::<String, &str>("value2".to_string()) },
            )
            .await
            .unwrap();

        assert_eq!(value, "value2");
    }

    #[tokio::test]
    async fn key_remove() {
        let cache = AsyncCache::new(Some(Duration::seconds(60)));

        // Add entry
        cache
            .get(
                "key1".to_string(),
                |_| false,
                || async { Ok::<String, &str>("value1".to_string()) },
            )
            .await
            .unwrap();

        // Remove
        let removed = cache.remove(&"key1".to_string()).await;
        assert_eq!(removed, Some("value1".to_string()));

        // Verify it's gone
        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value2".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Recomputed
    }

    #[tokio::test]
    async fn force_refresh() {
        let cache = AsyncCache::new(Some(Duration::seconds(60)));

        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        // First get - should compute
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value1".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Second get without force_refresh - should return cached value
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value2".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Not incremented

        // Third get WITH force_refresh (callback returns true) - should recompute
        let value = cache
            .get(
                "key1".to_string(),
                |_| true,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value3".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value3");
        assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Incremented due to force_refresh

        // Fourth get without force_refresh - should return newly cached value
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value4".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value3");
        assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Not incremented
    }

    #[tokio::test]
    async fn conditional_refresh_based_on_cached_value() {
        let cache = AsyncCache::new(Some(Duration::seconds(60)));

        // First get - cache is empty, should_refresh receives None
        let value = cache
            .get(
                "key1".to_string(),
                |cached| {
                    assert!(cached.is_none()); // No cached value yet
                    false
                },
                || async { Ok::<String, &str>("value1".to_string()) },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");

        // Second get - cache has value, should_refresh receives Some
        let value = cache
            .get(
                "key1".to_string(),
                |cached| {
                    assert_eq!(cached, Some(&"value1".to_string()));
                    false // Don't refresh
                },
                || async { Ok::<String, &str>("value2".to_string()) },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1"); // Still the original value

        // Third, get - conditionally refresh based on cached value content
        let value = cache
            .get(
                "key1".to_string(),
                |cached| {
                    // Refresh only if cached value is "value1"
                    cached.is_some_and(|v| v == "value1")
                },
                || async { Ok::<String, &str>("value3".to_string()) },
            )
            .await
            .unwrap();

        assert_eq!(value, "value3"); // Refreshed because condition was met
    }

    #[tokio::test]
    async fn non_expiring_cache() {
        let cache = AsyncCache::new(None);

        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        // First get - should compute
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value1".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Verify entry has no expiration
        {
            let store = cache.store.read().await;
            let entry = store.get(&"key1".to_string()).unwrap();
            assert!(entry.expires_at.is_none());
            assert!(!entry.is_expired()); // Never expires
        }

        // Second get - should return cached value (never expires)
        let value = cache
            .get(
                "key1".to_string(),
                |_| false,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value2".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Not incremented

        // Force refresh still works on non-expiring cache
        let value = cache
            .get(
                "key1".to_string(),
                |_| true,
                || async {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                    Ok::<String, &str>("value3".to_string())
                },
            )
            .await
            .unwrap();

        assert_eq!(value, "value3");
        assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Incremented due to force_refresh
    }
}
