// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A pool of reusable capture [`EventLogStorage`]s.
//!
//! Renting and returning a storage keeps the fast-success "drop" cost to a pair of `Vec::clear`s
//! — no allocation, no free of the backbones — after warm-up. The pool itself is **not** wrapped
//! in an `Arc`; instead consumers hold an `Arc<LogPool>` so the sharing is explicit at the call
//! site. [`rent`](LogPool::rent) hands out an [`EventLog`] lease that returns its storage on drop,
//! so the pool's brief lock is touched only at operation boundaries (rent / drop), never on the
//! per-attempt hot path.
//!
//! Retention is bounded and sized to the common operation: the pool keeps at most [`MAX_POOLED`]
//! storages, and only retains a returned storage whose combined `Vec` capacity is still at or
//! under [`EventLogStorage::DEFAULT_CAPACITY`]. A storage that had to grow (a wide fan-out) is
//! freed rather than parked, so a single large operation cannot pin oversized buffers in the pool.

use super::event::{EventLog, EventLogStorage};
use std::sync::{Arc, Mutex};

/// Maximum number of storages parked in the pool at once.
const MAX_POOLED: usize = 64;
/// Maximum retained capacity (spans + attrs entries) of a parked storage. Equal to the default
/// capacity, so the pool only retains storages that never had to grow beyond the common-case size.
const MAX_RETAINED_ENTRIES: usize = EventLogStorage::DEFAULT_CAPACITY;

/// A bounded pool of reusable capture [`EventLogStorage`]s.
///
/// Hold this behind an `Arc<LogPool>` and call `rent` to get an [`EventLog`] lease.
#[derive(Debug, Default)]
pub struct LogPool {
    free: Mutex<Vec<EventLogStorage>>,
}

impl LogPool {
    /// Rents an [`EventLog`] lease backed by a cleared storage, reusing a pooled one when available.
    ///
    /// The returned lease owns an `Arc<LogPool>` clone and returns its storage here automatically
    /// when dropped.
    pub(crate) fn rent(self: &Arc<Self>) -> EventLog {
        // Recover from a poisoned lock rather than propagating the panic: a diagnostics buffer
        // pool should never turn one unrelated panic into cascading panics on later operations.
        let storage = self
            .free
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pop()
            .map(|mut s| {
                s.clear();
                s
            })
            .unwrap_or_else(EventLogStorage::with_default_capacity);
        EventLog::new(Arc::clone(self), storage)
    }

    /// Returns a storage to the pool (cleared, capacity retained) unless bounds are exceeded.
    pub(crate) fn give_back(&self, mut storage: EventLogStorage) {
        if storage.capacity() > MAX_RETAINED_ENTRIES {
            return; // grew beyond the common-case size — let it free instead of pinning memory
        }
        storage.clear();
        let mut free = self.free.lock().unwrap_or_else(|e| e.into_inner());
        if free.len() < MAX_POOLED {
            free.push(storage);
        }
    }

    /// Number of storages currently parked in the pool.
    pub fn pooled(&self) -> usize {
        self.free.lock().unwrap_or_else(|e| e.into_inner()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::capture::event::{AttrKey, SpanKind, TimeOffset};

    #[test]
    fn rent_returns_cleared_lease_and_drop_pools_it() {
        let pool = Arc::new(LogPool::default());
        {
            let mut log = pool.rent();
            let op = log.push_span(
                SpanKind::Operation,
                None,
                TimeOffset::ZERO,
                TimeOffset::ZERO,
            );
            log.attr_str(op, AttrKey::OperationName, "read_item");
            assert_eq!(pool.pooled(), 0, "leased storage is not in the pool");
            // `log` drops here, returning its storage to the pool.
        }
        assert_eq!(pool.pooled(), 1, "drop returns the storage");
        let reused = pool.rent();
        assert!(reused.is_empty(), "a reused storage is cleared");
        assert_eq!(pool.pooled(), 0);
    }

    #[test]
    fn oversized_storages_are_not_retained() {
        let pool = LogPool::default();
        let big = EventLogStorage::with_capacity(MAX_RETAINED_ENTRIES + 1, 0);
        pool.give_back(big);
        assert_eq!(pool.pooled(), 0, "oversized storage must not be pooled");
    }

    #[test]
    fn default_capacity_storage_is_retained() {
        let pool = LogPool::default();
        pool.give_back(EventLogStorage::with_default_capacity());
        assert_eq!(pool.pooled(), 1, "a default-sized storage is pooled");
    }

    #[test]
    fn pool_is_bounded() {
        let pool = LogPool::default();
        for _ in 0..(MAX_POOLED + 10) {
            pool.give_back(EventLogStorage::with_capacity(1, 1));
        }
        assert_eq!(pool.pooled(), MAX_POOLED);
    }

    #[test]
    fn shared_arc_pool_is_one_store() {
        let pool = Arc::new(LogPool::default());
        let clone = Arc::clone(&pool);
        clone.give_back(EventLogStorage::with_capacity(1, 1));
        assert_eq!(pool.pooled(), 1, "Arc clones share the backing store");
    }
}
