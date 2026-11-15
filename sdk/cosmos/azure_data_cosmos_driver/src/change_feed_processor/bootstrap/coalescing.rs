// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    change_feed_processor::bootstrap::{bounded, catch_bootstrap_panic},
    models::ChangeFeedFactoryMode,
    Result,
};
use futures::{
    future::{BoxFuture, Shared, WeakShared},
    FutureExt,
};
use std::{
    collections::HashMap,
    fmt,
    mem::Discriminant,
    sync::{Arc, Mutex, MutexGuard},
    time::Instant,
};

type AttemptFuture = Shared<BoxFuture<'static, Result<bool>>>;

/// Identifies compatible in-process attempts that may share one future; not a distributed lock.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct AttemptKey {
    pub(super) namespace: String,
    pub(super) lease_container: String,
    // In-process identity; the shared operation retains this driver.
    pub(super) lease_driver: usize,
    pub(super) mode: Discriminant<ChangeFeedFactoryMode>,
}

/// Weak reference to an in-flight attempt plus a generation marker that protects replacement entries.
struct AttemptEntry {
    generation: Arc<()>,
    future: WeakShared<BoxFuture<'static, Result<bool>>>,
}

/// Driver-local registry for joining compatible bootstrap attempts without caching completed results.
#[derive(Default)]
pub(crate) struct BootstrapCoalescer {
    entries: Mutex<HashMap<AttemptKey, AttemptEntry>>,
}

impl fmt::Debug for BootstrapCoalescer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BootstrapCoalescer")
            .finish_non_exhaustive()
    }
}

impl BootstrapCoalescer {
    #[cfg(test)]
    pub(super) fn active_count(&self) -> usize {
        self.entries().len()
    }

    fn entries(&self) -> MutexGuard<'_, HashMap<AttemptKey, AttemptEntry>> {
        self.entries.lock().unwrap_or_else(|poisoned| {
            tracing::error!("Recovering poisoned bootstrap coalescing registry");
            poisoned.into_inner()
        })
    }

    pub(super) fn start_or_join<F>(
        self: &Arc<Self>,
        key: AttemptKey,
        deadline: Instant,
        make_attempt: F,
    ) -> AttemptFuture
    where
        F: FnOnce() -> BoxFuture<'static, Result<bool>> + Send + 'static,
    {
        let mut entries = self.entries();
        if let Some(future) = entries.get(&key).and_then(|entry| entry.future.upgrade()) {
            drop(entries);
            return future;
        }

        let generation = Arc::new(());
        let cleanup = AttemptCleanup {
            coalescer: Arc::clone(self),
            key: key.clone(),
            generation: Arc::clone(&generation),
        };
        let future = async move {
            let result = catch_bootstrap_panic(bounded(
                Box::pin(async move { make_attempt().await }),
                deadline,
            ))
            .await;
            drop(cleanup);
            result
        }
        .boxed()
        .shared();
        entries.insert(
            key,
            AttemptEntry {
                generation,
                future: future
                    .downgrade()
                    .expect("new bootstrap operation has not been polled"),
            },
        );
        future
    }
}

/// Removes only its own registry generation when the shared attempt completes or is dropped.
struct AttemptCleanup {
    coalescer: Arc<BootstrapCoalescer>,
    key: AttemptKey,
    generation: Arc<()>,
}

impl Drop for AttemptCleanup {
    fn drop(&mut self) {
        let mut entries = self.coalescer.entries();
        // A dead weak entry may have been replaced before its old future finished dropping.
        if entries
            .get(&self.key)
            .is_some_and(|entry| Arc::ptr_eq(&entry.generation, &self.generation))
        {
            entries.remove(&self.key);
        }
    }
}

#[cfg(test)]
mod tests;
