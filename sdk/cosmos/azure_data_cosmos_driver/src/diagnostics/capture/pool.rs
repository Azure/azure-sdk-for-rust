// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A pool of reusable capture buffers.
//!
//! Renting and returning a buffer keeps the fast-success "drop" cost to a `clear()` plus a
//! `push` — no allocation, no free — after warm-up. The pool is shared across operations
//! (`Clone` is a cheap [`Arc`] bump) and is accessed only at operation boundaries
//! (rent at `op_start`, return at `op_end`/drop), never on the per-attempt hot path, so the
//! brief lock never contends with capture appends.
//!
//! Retention is bounded (SE-6): the pool keeps at most [`MAX_POOLED`] buffers, and a returned
//! buffer whose capacity grew beyond [`MAX_RETAINED_CAPACITY`] (e.g. a wide fan-out) is dropped
//! rather than parked, so a single large operation cannot pin memory.

use std::sync::{Arc, Mutex};

/// Maximum number of buffers parked in the pool at once.
const MAX_POOLED: usize = 64;
/// Maximum retained capacity (bytes) of a parked buffer; larger buffers are dropped on return.
const MAX_RETAINED_CAPACITY: usize = 16 * 1024;
/// Initial capacity of a freshly allocated capture buffer.
const INITIAL_CAPACITY: usize = 256;

/// A shared, bounded pool of reusable capture buffers.
#[derive(Clone, Debug, Default)]
pub struct LogPool {
    free: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl LogPool {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Rents a cleared buffer, reusing a pooled one when available.
    pub(crate) fn rent(&self) -> Vec<u8> {
        match self.free.lock().expect("LogPool mutex poisoned").pop() {
            Some(mut buf) => {
                buf.clear();
                buf
            }
            None => Vec::with_capacity(INITIAL_CAPACITY),
        }
    }

    /// Returns a buffer to the pool (cleared, capacity retained) unless bounds are exceeded.
    pub(crate) fn give_back(&self, mut buf: Vec<u8>) {
        if buf.capacity() > MAX_RETAINED_CAPACITY {
            return; // oversized (e.g. wide fan-out) — let it free instead of pinning memory
        }
        buf.clear();
        let mut free = self.free.lock().expect("LogPool mutex poisoned");
        if free.len() < MAX_POOLED {
            free.push(buf);
        }
    }

    /// Number of buffers currently parked in the pool.
    pub fn pooled(&self) -> usize {
        self.free.lock().expect("LogPool mutex poisoned").len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rent_returns_cleared_buffer() {
        let pool = LogPool::new();
        let mut buf = pool.rent();
        buf.extend_from_slice(b"hello");
        pool.give_back(buf);
        assert_eq!(pool.pooled(), 1);
        let reused = pool.rent();
        assert!(reused.is_empty());
        assert_eq!(pool.pooled(), 0);
    }

    #[test]
    fn oversized_buffers_are_not_retained() {
        let pool = LogPool::new();
        let big = Vec::with_capacity(MAX_RETAINED_CAPACITY + 1);
        pool.give_back(big);
        assert_eq!(pool.pooled(), 0, "oversized buffer must not be pooled");
    }

    #[test]
    fn pool_is_bounded() {
        let pool = LogPool::new();
        for _ in 0..(MAX_POOLED + 10) {
            pool.give_back(Vec::with_capacity(8));
        }
        assert_eq!(pool.pooled(), MAX_POOLED);
    }

    #[test]
    fn clone_shares_the_same_pool() {
        let pool = LogPool::new();
        let clone = pool.clone();
        clone.give_back(Vec::with_capacity(8));
        assert_eq!(pool.pooled(), 1, "clones share the backing store");
    }
}
