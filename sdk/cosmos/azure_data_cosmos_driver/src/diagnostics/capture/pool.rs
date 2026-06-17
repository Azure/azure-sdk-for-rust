// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A pool of reusable capture [`EventLog`]s.
//!
//! Renting and returning a log keeps the fast-success "drop" cost to a pair of `Vec::clear`s — no
//! allocation, no free of the backbones — after warm-up. The pool is shared across operations
//! (`Clone` is a cheap [`Arc`] bump) and is touched only at operation boundaries (rent at
//! `op_start`, return at `op_end` / drop), never on the per-attempt hot path, so the brief lock
//! never contends with capture appends.
//!
//! Retention is bounded: the pool keeps at most [`MAX_POOLED`] logs, and a returned log whose
//! combined `Vec` capacity grew beyond [`MAX_RETAINED_ENTRIES`] (e.g. a wide fan-out) is dropped
//! rather than parked, so a single large operation cannot pin memory.

use super::event::EventLog;
use std::sync::{Arc, Mutex};

/// Maximum number of logs parked in the pool at once.
const MAX_POOLED: usize = 64;
/// Maximum retained capacity (spans + attrs entries) of a parked log; larger logs are dropped.
const MAX_RETAINED_ENTRIES: usize = 4096;
/// Initial span capacity of a freshly allocated log (most ops make 1–4 attempts).
const INITIAL_SPANS: usize = 8;
/// Initial attribute capacity of a freshly allocated log.
const INITIAL_ATTRS: usize = 48;

/// A shared, bounded pool of reusable capture [`EventLog`]s.
#[derive(Clone, Debug, Default)]
pub struct LogPool {
    free: Arc<Mutex<Vec<EventLog>>>,
}

impl LogPool {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Rents a cleared log, reusing a pooled one when available.
    pub(crate) fn rent(&self) -> EventLog {
        // Recover from a poisoned lock rather than propagating the panic: a diagnostics buffer
        // pool should never turn one unrelated panic into cascading panics on later operations.
        match self.free.lock().unwrap_or_else(|e| e.into_inner()).pop() {
            Some(mut log) => {
                log.clear();
                log
            }
            None => EventLog::with_capacity(INITIAL_SPANS, INITIAL_ATTRS),
        }
    }

    /// Returns a log to the pool (cleared, capacity retained) unless bounds are exceeded.
    pub(crate) fn give_back(&self, mut log: EventLog) {
        if log.capacity() > MAX_RETAINED_ENTRIES {
            return; // oversized (e.g. wide fan-out) — let it free instead of pinning memory
        }
        log.clear();
        let mut free = self.free.lock().unwrap_or_else(|e| e.into_inner());
        if free.len() < MAX_POOLED {
            free.push(log);
        }
    }

    /// Number of logs currently parked in the pool.
    pub fn pooled(&self) -> usize {
        self.free.lock().unwrap_or_else(|e| e.into_inner()).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::capture::{AttrKey, SpanKind, NO_PARENT};

    #[test]
    fn rent_returns_cleared_log() {
        let pool = LogPool::new();
        let mut log = pool.rent();
        let op = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        log.attr_str(op, AttrKey::OperationName, "read_item");
        pool.give_back(log);
        assert_eq!(pool.pooled(), 1);
        let reused = pool.rent();
        assert!(reused.is_empty());
        assert_eq!(pool.pooled(), 0);
    }

    #[test]
    fn oversized_logs_are_not_retained() {
        let pool = LogPool::new();
        let big = EventLog::with_capacity(MAX_RETAINED_ENTRIES + 1, 0);
        pool.give_back(big);
        assert_eq!(pool.pooled(), 0, "oversized log must not be pooled");
    }

    #[test]
    fn pool_is_bounded() {
        let pool = LogPool::new();
        for _ in 0..(MAX_POOLED + 10) {
            pool.give_back(EventLog::with_capacity(1, 1));
        }
        assert_eq!(pool.pooled(), MAX_POOLED);
    }

    #[test]
    fn clone_shares_the_same_pool() {
        let pool = LogPool::new();
        let clone = pool.clone();
        clone.give_back(EventLog::with_capacity(1, 1));
        assert_eq!(pool.pooled(), 1, "clones share the backing store");
    }
}
