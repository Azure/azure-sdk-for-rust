use async_lock::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cache entry with TTL tracking
#[derive(Clone, Debug)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// A generic async cache with TTL support.
#[derive(Clone)]
pub struct AsyncCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    ttl: Duration,
}

impl<K, V> AsyncCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Creates a new `AsyncCache` with the specified TTL.
    pub fn new(ttl: Duration) -> Self {
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
    /// * `force_refresh` - If true, bypass cache and compute fresh value even if cached value is valid
    /// * `compute` - Async function to compute the value if not cached or force refresh is requested
    pub async fn get<F, Fut, E>(&self, key: K, force_refresh: bool, compute: F) -> Result<V, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, E>>,
    {
        // Fast path: check if value exists and is not expired (read lock)
        // Skip this if force_refresh is true
        if !force_refresh {
            let store = self.store.read().await;
            if let Some(entry) = store.get(&key) {
                if !entry.is_expired() {
                    return Ok(entry.value.clone());
                }
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
            // force_refresh is true, remove existing entry to ensure fresh computation
            store.remove(&key);
        }

        // Compute new value
        let value = compute().await?;

        // Update cache with new value
        let entry = CacheEntry::new(value.clone(), self.ttl);
        store.insert(key, entry);

        Ok(value)
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
        let cache = AsyncCache::new(Duration::from_secs(60));

        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        // First get - should compute
        let value = cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value1".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Second get - should return cached value
        let value = cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value2".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Not incremented
    }

    #[tokio::test]
    async fn key_expiration() {
        let cache = AsyncCache::new(Duration::from_secs(60));

        // Add entry
        cache
            .get("key1".to_string(), false, || async {
                Ok::<String, &str>("value1".to_string())
            })
            .await
            .unwrap();

        // Manually expire the cache entry by setting expires_at to a past time
        {
            let mut store = cache.store.write().await;
            if let Some(entry) = store.get_mut(&"key1".to_string()) {
                entry.expires_at = Instant::now() - Duration::from_secs(1);
            }
        }

        // Get again - should recompute after expiration
        let value = cache
            .get("key1".to_string(), false, || async {
                Ok::<String, &str>("value2".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value2");
    }

    #[tokio::test]
    async fn key_remove() {
        let cache = AsyncCache::new(Duration::from_secs(60));

        // Add entry
        cache
            .get("key1".to_string(), false, || async {
                Ok::<String, &str>("value1".to_string())
            })
            .await
            .unwrap();

        // Remove
        let removed = cache.remove(&"key1".to_string()).await;
        assert_eq!(removed, Some("value1".to_string()));

        // Verify it's gone
        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value2".to_string())
            })
            .await
            .unwrap();

        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Recomputed
    }

    #[tokio::test]
    async fn force_refresh() {
        let cache = AsyncCache::new(Duration::from_secs(60));

        let compute_count = Arc::new(AtomicUsize::new(0));
        let count_clone = compute_count.clone();

        // First get - should compute
        let value = cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value1".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Second get without force_refresh - should return cached value
        let value = cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value2".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value1");
        assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Not incremented

        // Third get WITH force_refresh - should recompute even though cached value is valid
        let value = cache
            .get("key1".to_string(), true, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value3".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value3");
        assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Incremented due to force_refresh

        // Fourth get without force_refresh - should return newly cached value
        let value = cache
            .get("key1".to_string(), false, || async {
                count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<String, &str>("value4".to_string())
            })
            .await
            .unwrap();

        assert_eq!(value, "value3");
        assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Not incremented
    }
}
