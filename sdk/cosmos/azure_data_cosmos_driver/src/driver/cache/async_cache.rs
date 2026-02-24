// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Async cache with single-pending-I/O semantics per key.

use super::AsyncLazy;
use async_lock::RwLock;
use std::{collections::HashMap, future::Future, hash::Hash, sync::Arc};

/// A concurrent async cache with single-pending-I/O semantics per key.
/// Not meant to be used as a general purpose cache - but to cache
/// certain metadata like Containers or account properties in combination
/// with retry policies to reach eventual consistent state.
///
/// When multiple callers request the same key simultaneously, only one
/// initialization future runs. Other callers wait for and share that result.
///
/// The cache stores `AsyncLazy<V>` entries (pending computations), not final values.
/// This ensures that concurrent requests for the same missing key share the same
/// initialization - only one I/O operation runs per key.
///
/// Uses `async_lock::RwLock` instead of tokio to remain async-runtime agnostic.
#[derive(Debug)]
pub(crate) struct AsyncCache<K, V> {
    /// The underlying map storing lazy values, protected by an async RwLock.
    map: RwLock<HashMap<K, Arc<AsyncLazy<V>>>>,
}

impl<K, V> AsyncCache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a new empty cache.
    pub(crate) fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    /// Gets a value from the cache, initializing it with the provided future if necessary.
    ///
    /// If the key exists (even if still initializing), waits for that initialization.
    /// If the key doesn't exist, atomically inserts a new `AsyncLazy` and starts
    /// the initialization. Concurrent requests for the same missing key will share
    /// the same `AsyncLazy` and thus the same initialization - only one factory
    /// function runs per key.
    pub(crate) async fn get_or_insert_with<F, Fut>(&self, key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        // Fast path: check if already in cache with read lock
        {
            let read_guard = self.map.read().await;
            if let Some(lazy) = read_guard.get(&key) {
                let lazy_clone = lazy.clone();
                drop(read_guard); // Release read lock before awaiting
                                  // Use get_or_init rather than get() to avoid a panic if
                                  // the entry was just inserted but not yet initialized.
                return lazy_clone.get_or_init(factory).await;
            }
        }

        // Slow path: need to insert a new AsyncLazy
        // Create the lazy FIRST, then atomically insert it
        let lazy = {
            let mut write_guard = self.map.write().await;

            // Double-check: another task might have inserted while we waited
            if let Some(existing) = write_guard.get(&key) {
                existing.clone()
            } else {
                // Insert a new empty AsyncLazy - the key now "exists" in the map
                let new_lazy = Arc::new(AsyncLazy::new());
                write_guard.insert(key, new_lazy.clone());
                new_lazy
            }
        };

        // Now initialize the lazy value (only one caller will actually run factory)
        lazy.get_or_init(factory).await
    }

    /// Gets a cached value if it exists and is initialized.
    ///
    /// Returns `None` if the key doesn't exist or initialization hasn't completed.
    pub(crate) async fn get(&self, key: &K) -> Option<Arc<V>> {
        let read_guard = self.map.read().await;
        read_guard.get(key).and_then(|lazy| lazy.try_get())
    }

    /// Removes an entry from the cache.
    ///
    /// Returns the value if the key existed and was initialized.
    pub(crate) async fn invalidate(&self, key: &K) -> Option<Arc<V>> {
        let mut write_guard = self.map.write().await;
        write_guard.remove(key).and_then(|lazy| lazy.try_get())
    }

    /// Clears all entries from the cache.
    #[cfg(test)]
    pub(crate) async fn clear(&self) {
        let mut write_guard = self.map.write().await;
        write_guard.clear();
    }

    /// Gets a value, optionally forcing a refresh based on a predicate.
    ///
    /// This method enables conditional cache refresh without losing the current value
    /// during the refresh operation. The flow is:
    ///
    /// 1. If the key doesn't exist, call `should_force_refresh(None)`:
    ///    - If true, initialize with factory
    ///    - If false, return None (no value and shouldn't create one)
    ///
    /// 2. If the key exists and has a value, call `should_force_refresh(Some(&value))`:
    ///    - If false, return the existing value
    ///    - If true, start a new async operation to refresh
    ///
    /// The refresh semantics ensure single-pending-I/O per key: if multiple callers
    /// trigger a refresh concurrently, only one refresh runs. Others wait for and
    /// share that result.
    ///
    /// Unlike `invalidate` + `get_or_insert_with`, this approach keeps the old value
    /// available while refresh is in progress, and only replaces it atomically when
    /// the new value is ready.
    #[cfg(test)]
    pub(crate) async fn get_or_refresh_with<F, Fut, P>(
        &self,
        key: K,
        should_force_refresh: P,
        factory: F,
    ) -> Option<Arc<V>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = V>,
        P: FnOnce(Option<&V>) -> bool,
    {
        // First, check if we have an existing entry
        let (initial_lazy, existing_value, is_initializing) = {
            let read_guard = self.map.read().await;
            match read_guard.get(&key) {
                Some(lazy) => {
                    let lazy_clone = lazy.clone();
                    // Get the current value (if initialized) without blocking
                    let current = lazy.try_get();
                    // Detect if initialization is in progress (lazy exists but no value yet)
                    let initializing = current.is_none();
                    (Some(lazy_clone), current, initializing)
                }
                None => (None, None, false),
            }
        };

        // If initialization is in progress (lazy exists but not yet initialized),
        // wait for it to complete rather than starting a new refresh
        if is_initializing {
            if let Some(lazy) = initial_lazy {
                return Some(lazy.get().await);
            }
        }

        // Determine if we should refresh based on the predicate
        // The predicate only sees fully initialized values
        let needs_refresh = should_force_refresh(existing_value.as_ref().map(|v| v.as_ref()));

        if !needs_refresh {
            // No refresh needed - return existing value or None
            return existing_value;
        }

        // Need to refresh - create new lazy and atomically update
        // We use a pattern similar to Java's ConcurrentHashMap.merge():
        // only replace if the lazy we observed is still the current one
        let new_lazy = {
            let mut write_guard = self.map.write().await;

            // Check if the entry still matches what we observed
            match (initial_lazy.as_ref(), write_guard.get(&key)) {
                // Key didn't exist and still doesn't - insert new
                (None, None) => {
                    let lazy = Arc::new(AsyncLazy::new());
                    write_guard.insert(key, lazy.clone());
                    lazy
                }
                // Key didn't exist but now does - another task inserted, use theirs
                (None, Some(current)) => {
                    // Another task already started initialization, wait for it
                    let current_clone = current.clone();
                    drop(write_guard);
                    return Some(current_clone.get().await);
                }
                // Key existed - check if it's still the same lazy
                (Some(initial), Some(current)) => {
                    if Arc::ptr_eq(initial, current) {
                        // Still the same lazy we observed - we can replace
                        let lazy = Arc::new(AsyncLazy::new());
                        write_guard.insert(key, lazy.clone());
                        lazy
                    } else {
                        // Lazy changed - another task already refreshed, use theirs
                        let current_clone = current.clone();
                        drop(write_guard);
                        return Some(current_clone.get().await);
                    }
                }
                // Key existed but was removed - insert new
                (Some(_), None) => {
                    let lazy = Arc::new(AsyncLazy::new());
                    write_guard.insert(key, lazy.clone());
                    lazy
                }
            }
        };

        // Initialize the new lazy value
        Some(new_lazy.get_or_init(factory).await)
    }
}

impl<K, V> Default for AsyncCache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{sleep, Duration};

    async fn cache_len<K, V>(cache: &AsyncCache<K, V>) -> usize
    where
        K: Eq + Hash + Clone,
    {
        cache.map.read().await.len()
    }

    async fn cache_is_empty<K, V>(cache: &AsyncCache<K, V>) -> bool
    where
        K: Eq + Hash + Clone,
    {
        cache.map.read().await.is_empty()
    }

    async fn cache_clear<K, V>(cache: &AsyncCache<K, V>)
    where
        K: Eq + Hash + Clone,
    {
        cache.map.write().await.clear();
    }

    #[tokio::test]
    async fn get_or_insert_caches_value() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        let value = cache
            .get_or_insert_with("key1".to_string(), || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                42
            })
            .await;

        assert_eq!(*value, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Second access should use cached value
        let counter_clone = counter.clone();
        let value2 = cache
            .get_or_insert_with("key1".to_string(), || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                100
            })
            .await;

        assert_eq!(*value2, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn different_keys_different_values() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();

        let v1 = cache
            .get_or_insert_with("key1".to_string(), || async { 1 })
            .await;
        let v2 = cache
            .get_or_insert_with("key2".to_string(), || async { 2 })
            .await;

        assert_eq!(*v1, 1);
        assert_eq!(*v2, 2);
        assert_eq!(cache_len(&cache).await, 2);
    }

    #[tokio::test]
    async fn concurrent_same_key_single_init() {
        let cache = Arc::new(AsyncCache::<String, String>::new());
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let cache_clone = cache.clone();
            let counter_clone = counter.clone();
            handles.push(tokio::spawn(async move {
                cache_clone
                    .get_or_insert_with("shared_key".to_string(), || async move {
                        counter_clone.fetch_add(1, Ordering::SeqCst);
                        sleep(Duration::from_millis(50)).await;
                        "result".to_string()
                    })
                    .await
            }));
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(*result, "result");
        }

        // Only ONE initialization should have occurred - this is the key invariant!
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn get_returns_none_before_insert() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        assert!(cache.get(&"key".to_string()).await.is_none());
    }

    #[tokio::test]
    async fn get_returns_value_after_insert() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        cache
            .get_or_insert_with("key".to_string(), || async { 42 })
            .await;

        assert_eq!(*cache.get(&"key".to_string()).await.unwrap(), 42);
    }

    #[tokio::test]
    async fn invalidate_removes_entry() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        cache
            .get_or_insert_with("key".to_string(), || async { 42 })
            .await;

        let removed = cache.invalidate(&"key".to_string()).await;
        assert_eq!(*removed.unwrap(), 42);
        assert!(cache.get(&"key".to_string()).await.is_none());
        assert!(cache_is_empty(&cache).await);
    }

    #[tokio::test]
    async fn clear_removes_all_entries() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        cache
            .get_or_insert_with("key1".to_string(), || async { 1 })
            .await;
        cache
            .get_or_insert_with("key2".to_string(), || async { 2 })
            .await;

        assert_eq!(cache_len(&cache).await, 2);
        cache_clear(&cache).await;
        assert!(cache_is_empty(&cache).await);
    }

    #[tokio::test]
    async fn refresh_when_key_missing_and_predicate_true() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        let result = cache
            .get_or_refresh_with(
                "key".to_string(),
                |existing| {
                    // No existing value, should initialize
                    assert!(existing.is_none());
                    true
                },
                || async move {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    42
                },
            )
            .await;

        assert_eq!(*result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn no_refresh_when_key_missing_and_predicate_false() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        let result = cache
            .get_or_refresh_with(
                "key".to_string(),
                |existing| {
                    assert!(existing.is_none());
                    false // Don't initialize
                },
                || async move {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    42
                },
            )
            .await;

        assert!(result.is_none());
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        assert!(cache_is_empty(&cache).await);
    }

    #[tokio::test]
    async fn no_refresh_when_value_not_stale() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();

        // Pre-populate cache
        cache
            .get_or_insert_with("key".to_string(), || async { 42 })
            .await;

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = cache
            .get_or_refresh_with(
                "key".to_string(),
                |existing| {
                    // Value exists and is fresh
                    assert_eq!(*existing.unwrap(), 42);
                    false // No refresh needed
                },
                || async move {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    100
                },
            )
            .await;

        assert_eq!(*result.unwrap(), 42); // Still original value
        assert_eq!(counter.load(Ordering::SeqCst), 0); // Factory not called
    }

    #[tokio::test]
    async fn refresh_when_value_is_stale() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();

        // Pre-populate cache with "stale" value
        cache
            .get_or_insert_with("key".to_string(), || async { 42 })
            .await;

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let result = cache
            .get_or_refresh_with(
                "key".to_string(),
                |existing| {
                    // Value is stale
                    assert_eq!(*existing.unwrap(), 42);
                    true // Force refresh
                },
                || async move {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    100
                },
            )
            .await;

        assert_eq!(*result.unwrap(), 100); // New value
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Factory called once
    }

    #[tokio::test]
    async fn concurrent_refresh_single_factory_call() {
        let cache = Arc::new(AsyncCache::<String, String>::new());

        // Pre-populate with stale value
        cache
            .get_or_insert_with("key".to_string(), || async { "stale".to_string() })
            .await;

        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let cache_clone = cache.clone();
            let counter_clone = counter.clone();
            handles.push(tokio::spawn(async move {
                cache_clone
                    .get_or_refresh_with(
                        "key".to_string(),
                        |existing| {
                            // All see stale value and want refresh
                            existing.map(|v| v.as_str() == "stale").unwrap_or(true)
                        },
                        || async move {
                            counter_clone.fetch_add(1, Ordering::SeqCst);
                            sleep(Duration::from_millis(50)).await;
                            "fresh".to_string()
                        },
                    )
                    .await
            }));
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(*result.unwrap(), "fresh");
        }

        // Only ONE factory call despite 10 concurrent requests
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn refresh_replaces_old_lazy_atomically() {
        let cache: AsyncCache<String, i32> = AsyncCache::new();

        // Insert initial value
        cache
            .get_or_insert_with("key".to_string(), || async { 1 })
            .await;

        // Refresh
        cache
            .get_or_refresh_with(
                "key".to_string(),
                |_| true, // Force refresh
                || async { 2 },
            )
            .await;

        // Regular get should return new value
        let value = cache.get(&"key".to_string()).await.unwrap();
        assert_eq!(*value, 2);

        // Another refresh
        cache
            .get_or_refresh_with(
                "key".to_string(),
                |_| true, // Force refresh
                || async { 3 },
            )
            .await;

        let value = cache.get(&"key".to_string()).await.unwrap();
        assert_eq!(*value, 3);
    }

    #[tokio::test]
    async fn fast_path_does_not_panic_on_uninitialized_lazy() {
        // Regression test: a concurrent reader could hit the fast path and find
        // an AsyncLazy that was just inserted but not yet initialized.
        // The fast path must not panic — it should wait or initialize.
        let cache = Arc::new(AsyncCache::<String, String>::new());

        let mut handles = vec![];
        for i in 0..20 {
            let cache_clone = cache.clone();
            handles.push(tokio::spawn(async move {
                cache_clone
                    .get_or_insert_with("race_key".to_string(), || async move {
                        // Simulate slow initialization
                        sleep(Duration::from_millis(10)).await;
                        format!("value-{}", i)
                    })
                    .await
            }));
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        // All tasks should get the same value (single-init semantics)
        let first = &*results[0];
        for result in &results {
            assert_eq!(&**result, first);
        }
    }
}
