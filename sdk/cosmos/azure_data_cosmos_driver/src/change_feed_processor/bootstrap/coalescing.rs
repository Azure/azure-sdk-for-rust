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

type ChangeFeedProcessorBootstrapFuture = Shared<BoxFuture<'static, Result<bool>>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct ChangeFeedProcessorBootstrapCoalescingKey {
    pub(super) namespace: String,
    pub(super) lease_container: String,
    // In-process identity; the shared operation retains this driver.
    pub(super) lease_driver: usize,
    pub(super) mode: Discriminant<ChangeFeedFactoryMode>,
}

struct ChangeFeedProcessorBootstrapCoalescingEntry {
    generation: Arc<()>,
    future: WeakShared<BoxFuture<'static, Result<bool>>>,
}

#[derive(Default)]
pub(crate) struct ChangeFeedProcessorBootstrapCoalescer {
    entries: Mutex<
        HashMap<
            ChangeFeedProcessorBootstrapCoalescingKey,
            ChangeFeedProcessorBootstrapCoalescingEntry,
        >,
    >,
}

impl fmt::Debug for ChangeFeedProcessorBootstrapCoalescer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ChangeFeedProcessorBootstrapCoalescer")
            .finish_non_exhaustive()
    }
}

impl ChangeFeedProcessorBootstrapCoalescer {
    #[cfg(test)]
    pub(super) fn active_count(&self) -> usize {
        self.entries().len()
    }

    fn entries(
        &self,
    ) -> MutexGuard<
        '_,
        HashMap<
            ChangeFeedProcessorBootstrapCoalescingKey,
            ChangeFeedProcessorBootstrapCoalescingEntry,
        >,
    > {
        self.entries.lock().unwrap_or_else(|poisoned| {
            tracing::error!("Recovering poisoned bootstrap coalescing registry");
            poisoned.into_inner()
        })
    }

    pub(super) fn start_or_join<F>(
        self: &Arc<Self>,
        key: ChangeFeedProcessorBootstrapCoalescingKey,
        deadline: Instant,
        make_attempt: F,
    ) -> ChangeFeedProcessorBootstrapFuture
    where
        F: FnOnce() -> BoxFuture<'static, Result<bool>> + Send + 'static,
    {
        let mut entries = self.entries();
        if let Some(future) = entries.get(&key).and_then(|entry| entry.future.upgrade()) {
            drop(entries);
            return future;
        }

        let generation = Arc::new(());
        let cleanup = ChangeFeedProcessorBootstrapCoalescingCleanup {
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
            ChangeFeedProcessorBootstrapCoalescingEntry {
                generation,
                future: future
                    .downgrade()
                    .expect("new bootstrap operation has not been polled"),
            },
        );
        future
    }
}

struct ChangeFeedProcessorBootstrapCoalescingCleanup {
    coalescer: Arc<ChangeFeedProcessorBootstrapCoalescer>,
    key: ChangeFeedProcessorBootstrapCoalescingKey,
    generation: Arc<()>,
}

impl Drop for ChangeFeedProcessorBootstrapCoalescingCleanup {
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
