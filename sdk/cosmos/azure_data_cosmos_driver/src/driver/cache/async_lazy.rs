// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-value lazy initialization with async support.

use async_lock::RwLock;
use std::{future::Future, sync::Arc};

#[cfg(test)]
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// A lazily initialized value that is computed asynchronously.
///
/// The first caller to request the value runs the initialization future.
/// Subsequent callers wait for that same future to complete and share the result.
/// This ensures single-pending-I/O semantics: only one initialization runs at a time.
///
/// Uses `async_lock::RwLock` instead of tokio to remain async-runtime agnostic.
/// After initialization, concurrent readers share a read lock with no contention.
#[derive(Debug)]
pub(crate) struct AsyncLazy<T> {
    /// The lazily initialized value, protected by an async read-write lock.
    value: RwLock<Option<Arc<T>>>,
}

impl<T> AsyncLazy<T> {
    /// Creates a new uninitialized `AsyncLazy`.
    pub(crate) fn new() -> Self {
        Self {
            value: RwLock::new(None),
        }
    }

    /// Gets the value, initializing it with the provided future if necessary.
    ///
    /// Uses double-checked locking: the fast path acquires only a read lock.
    /// If the value is not yet initialized, a write lock is acquired and the
    /// value is checked again before running the initialization future.
    /// This ensures minimal contention on the common (already-initialized) path.
    pub(crate) async fn get_or_init<F, Fut>(&self, init: F) -> Arc<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        // Fast path: read lock only
        {
            let guard = self.value.read().await;
            if let Some(ref value) = *guard {
                return value.clone();
            }
        }

        // Slow path: acquire write lock and double-check
        let mut guard = self.value.write().await;
        if let Some(ref value) = *guard {
            return value.clone();
        }

        // Initialize
        let value = Arc::new(init().await);
        *guard = Some(value.clone());
        value
    }

    /// Gets the value if it has been initialized, without blocking.
    ///
    /// Returns `None` if initialization has not completed or is in progress.
    pub(crate) fn try_get(&self) -> Option<Arc<T>> {
        // Use try_read to avoid blocking - if write-locked, initialization may be in progress
        self.value.try_read().and_then(|guard| guard.clone())
    }

    /// Gets the value, waiting for initialization to complete.
    ///
    /// If the value is not yet initialized (another task is about to call
    /// [`get_or_init`](Self::get_or_init)), this method yields and retries
    /// until the value becomes available. It will not panic.
    #[cfg(test)]
    pub(crate) async fn get(&self) -> Arc<T> {
        loop {
            {
                let guard = self.value.read().await;
                if let Some(ref value) = *guard {
                    return value.clone();
                }
            }
            // Value not yet initialized — yield and retry so the initializing
            // task can make progress and set the value.
            YieldOnce(false).await;
        }
    }
}

/// Future that yields execution once to the async runtime, then completes.
///
/// This is runtime-agnostic: it returns [`Poll::Pending`] once (scheduling a
/// wake-up via the waker) and [`Poll::Ready`] on the subsequent poll.
#[cfg(test)]
struct YieldOnce(bool);

#[cfg(test)]
impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
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
    }

    #[tokio::test]
    async fn try_get_returns_value_after_init() {
        let lazy = AsyncLazy::new();
        lazy.get_or_init(|| async { 42 }).await;

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

    #[tokio::test]
    async fn get_waits_when_called_before_init_starts() {
        // Reproduces the race: get() is called before get_or_init() has even started.
        // Previously this would panic; now get() should yield and wait.
        let lazy = Arc::new(AsyncLazy::new());
        let lazy_for_get = lazy.clone();
        let lazy_for_init = lazy.clone();

        // Spawn get() first — it should not panic
        let get_handle = tokio::spawn(async move { lazy_for_get.get().await });

        // Yield to let the get() task run and observe None
        tokio::task::yield_now().await;

        // Now start initialization
        let init_handle =
            tokio::spawn(async move { lazy_for_init.get_or_init(|| async { 99 }).await });

        let get_result = get_handle.await.unwrap();
        let init_result = init_handle.await.unwrap();
        assert_eq!(*get_result, 99);
        assert_eq!(*init_result, 99);
    }
}
