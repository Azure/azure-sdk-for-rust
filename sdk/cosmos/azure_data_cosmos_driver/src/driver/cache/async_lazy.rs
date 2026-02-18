// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-value lazy initialization with async support.

use async_lock::Mutex;
use std::{future::Future, sync::Arc};

/// A lazily initialized value that is computed asynchronously.
///
/// The first caller to request the value runs the initialization future.
/// Subsequent callers wait for that same future to complete and share the result.
/// This ensures single-pending-I/O semantics: only one initialization runs at a time.
///
/// Uses `async_lock::Mutex` instead of tokio to remain async-runtime agnostic.
#[derive(Debug)]
pub(crate) struct AsyncLazy<T> {
    /// The lazily initialized value, protected by an async mutex.
    value: Mutex<Option<Arc<T>>>,
}

impl<T> AsyncLazy<T> {
    /// Creates a new uninitialized `AsyncLazy`.
    pub(crate) fn new() -> Self {
        Self {
            value: Mutex::new(None),
        }
    }

    /// Gets the value, initializing it with the provided future if necessary.
    ///
    /// If the value is already initialized, returns it immediately.
    /// If not, runs the initialization future. Only one future runs at a time;
    /// other callers wait for the same result.
    pub(crate) async fn get_or_init<F, Fut>(&self, init: F) -> Arc<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let mut guard = self.value.lock().await;

        if let Some(ref value) = *guard {
            return value.clone();
        }

        // Initialize
        let value = Arc::new(init().await);
        *guard = Some(value.clone());
        value
    }

    /// Waits for the value to be initialized and returns it.
    ///
    /// If the value is already initialized, returns it immediately.
    /// If initialization is in progress, waits for it to complete.
    ///
    /// # Panics
    ///
    /// Panics if the value was never initialized (i.e., `get_or_init` was never called).
    /// This should only be used when you know the value will be initialized.
    pub(crate) async fn get(&self) -> Arc<T> {
        let guard = self.value.lock().await;
        guard
            .clone()
            .expect("AsyncLazy::get called but value was never initialized")
    }

    /// Gets the value if it has been initialized, without blocking.
    ///
    /// Returns `None` if initialization has not completed or is in progress.
    pub(crate) fn try_get(&self) -> Option<Arc<T>> {
        // Use try_lock to avoid blocking - if locked, initialization may be in progress
        self.value.try_lock().and_then(|guard| guard.clone())
    }

    /// Returns `true` if the value has been initialized.
    pub(crate) fn is_initialized(&self) -> bool {
        self.value
            .try_lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }
}

impl<T> Default for AsyncLazy<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn initializes_once() {
        let lazy = AsyncLazy::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let counter_clone = counter.clone();
        let value = lazy
            .get_or_init(|| async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                42
            })
            .await;

        assert_eq!(*value, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Second call should not reinitialize
        let counter_clone = counter.clone();
        let value2 = lazy
            .get_or_init(|| async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                100 // Different value to prove it's not called
            })
            .await;

        assert_eq!(*value2, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn concurrent_access_single_init() {
        let lazy = Arc::new(AsyncLazy::new());
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let lazy_clone = lazy.clone();
            let counter_clone = counter.clone();
            handles.push(tokio::spawn(async move {
                lazy_clone
                    .get_or_init(|| async move {
                        counter_clone.fetch_add(1, Ordering::SeqCst);
                        // Simulate slow initialization
                        sleep(Duration::from_millis(10)).await;
                        "initialized"
                    })
                    .await
            }));
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(*result, "initialized");
        }

        // Only one initialization should have occurred
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn try_get_returns_none_before_init() {
        let lazy: AsyncLazy<i32> = AsyncLazy::new();
        assert!(lazy.try_get().is_none());
        assert!(!lazy.is_initialized());
    }

    #[tokio::test]
    async fn try_get_returns_value_after_init() {
        let lazy = AsyncLazy::new();
        lazy.get_or_init(|| async { 42 }).await;

        assert!(lazy.is_initialized());
        assert_eq!(*lazy.try_get().unwrap(), 42);
    }

    #[tokio::test]
    async fn get_waits_for_initialization() {
        let lazy = Arc::new(AsyncLazy::new());
        let lazy_clone = lazy.clone();

        // Start initialization in background
        let handle = tokio::spawn(async move {
            lazy_clone
                .get_or_init(|| async {
                    sleep(Duration::from_millis(50)).await;
                    42
                })
                .await
        });

        // Give it time to start
        sleep(Duration::from_millis(10)).await;

        // get() should wait for initialization to complete
        let value = lazy.get().await;
        assert_eq!(*value, 42);

        handle.await.unwrap();
    }
}
