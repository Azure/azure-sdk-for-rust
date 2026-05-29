// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-shard observation state used to add pre-failure context to error logs.
//!
//! The SDK exposes a `shard_id: u64` per request via the diagnostics context's
//! `transport_shard()`. The driver itself maintains shard / h2-connection state
//! internally and does not surface "time since this h2 connection opened",
//! "last 10 stream-ids handled by this shard", or pool-wide connection-open
//! timing to callers.
//!
//! Rather than reaching into the driver, this module reconstructs the same
//! information from outside, using only what the perf binary can see on every
//! request:
//!
//! - **`observed_age_ms`** — wall-clock time since the perf binary *first*
//!   saw this `shard_id`. The driver creates a fresh `shard_id` for every
//!   new h2 connection it opens, so first-observation time is a tight upper
//!   bound on the connection's true age (it cannot be older than perf's
//!   first observation of it).
//! - **`ms_since_last_success_on_shard`** — wall-clock time since the perf
//!   binary last observed a successful request finish on this `shard_id`.
//! - **`request_seq_on_shard` / `recent_request_seqs_on_shard`** — a
//!   monotonic per-shard request counter maintained by perf. **Not** the
//!   real HTTP/2 stream-id (hyper/h2 do not surface those on the
//!   client-side `SendRequest` API), but functionally equivalent for "did
//!   this shard appear to stall after a specific request number" triage.
//! - **`other_shards_opened_within_100ms`** — whether perf observed any
//!   *other* `shard_id` for the first time within 100ms before the failing
//!   request. Reconstructs the pool's connection-storm signal from
//!   outside.
//!
//! The single `Mutex` on the hot path is unavoidable while we maintain a
//! shared `HashMap` of shards, but the critical section is small (a few
//! ring-buffer pushes and a single `HashMap::entry`); this is acceptable
//! overhead for a diagnostics-only path.

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Serialize;

/// How many recent per-shard sequence numbers to retain for the ring buffer.
const RECENT_SEQS_PER_SHARD: usize = 10;

/// Window for "other shards opened recently" — see [`ShardSnapshot`].
const OTHER_SHARDS_WINDOW: Duration = Duration::from_millis(100);

/// Upper bound on the pool-wide "shards first observed" history kept in
/// memory. Entries older than [`SHARD_OPEN_HISTORY_TTL`] are pruned lazily
/// on every observation.
const SHARD_OPEN_HISTORY_CAP: usize = 256;

/// How long to retain shard first-observation entries before pruning. Must
/// be comfortably larger than [`OTHER_SHARDS_WINDOW`].
const SHARD_OPEN_HISTORY_TTL: Duration = Duration::from_secs(60);

#[derive(Debug)]
struct ShardState {
    first_seen: Instant,
    last_success: Option<Instant>,
    next_seq: u64,
    recent_seqs: VecDeque<u64>,
}

impl ShardState {
    fn new(now: Instant) -> Self {
        Self {
            first_seen: now,
            last_success: None,
            next_seq: 0,
            recent_seqs: VecDeque::with_capacity(RECENT_SEQS_PER_SHARD),
        }
    }
}

#[derive(Debug, Default)]
struct ObserverState {
    shards: HashMap<u64, ShardState>,
    /// `(shard_id, first_seen)` ordered by `first_seen` ascending.
    shard_open_history: VecDeque<(u64, Instant)>,
}

/// Tracks per-shard observation state across the perf binary.
#[derive(Debug, Default)]
pub struct ShardObserver {
    state: Mutex<ObserverState>,
}

/// Snapshot of a single shard's state at the moment of an observed request.
///
/// All `ms_since_*` fields measure wall-clock elapsed time from the recorded
/// event to "now" (the observation time). `recent_request_seqs_on_shard`
/// is ordered oldest-to-newest and *includes* the current request's seq.
#[derive(Clone, Debug, Serialize)]
pub struct ShardSnapshot {
    /// The SDK-assigned shard id (from `transport_shard.shard_id()`).
    pub shard_id: u64,
    /// Wall-clock ms since perf first observed this `shard_id`.
    pub observed_age_ms: u64,
    /// Wall-clock ms since perf last observed a successful finish on this
    /// `shard_id`. `None` if no success has been observed yet (e.g., the
    /// shard has only ever failed, or this was its first request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_since_last_success_on_shard: Option<u64>,
    /// Monotonic sequence number assigned to this request on this shard.
    pub request_seq_on_shard: u64,
    /// Up to the most recent [`RECENT_SEQS_PER_SHARD`] sequence numbers on
    /// this shard, oldest-to-newest. Includes the current request.
    pub recent_request_seqs_on_shard: Vec<u64>,
    /// Whether any **other** shard was first observed within the last
    /// [`OTHER_SHARDS_WINDOW`] (100 ms by default).
    pub other_shards_opened_within_100ms: bool,
    /// Ids of those other shards (sorted, deduped). Empty when
    /// `other_shards_opened_within_100ms` is `false`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub other_shards_opened_recently: Vec<u64>,
}

impl ShardObserver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Observes a request that terminated on `shard_id` with `success`
    /// outcome and returns the resulting snapshot.
    pub fn observe(&self, shard_id: u64, success: bool) -> ShardSnapshot {
        self.observe_at(shard_id, success, Instant::now())
    }

    fn observe_at(&self, shard_id: u64, success: bool, now: Instant) -> ShardSnapshot {
        // Safe to ignore poisoning: the critical section only performs
        // collection mutations that cannot panic.
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());

        let mut newly_seen = false;
        let entry = state.shards.entry(shard_id).or_insert_with(|| {
            newly_seen = true;
            ShardState::new(now)
        });

        let first_seen = entry.first_seen;
        let ms_since_last_success = entry
            .last_success
            .map(|t| now.saturating_duration_since(t).as_millis() as u64);

        let seq = entry.next_seq;
        entry.next_seq = entry.next_seq.saturating_add(1);
        if entry.recent_seqs.len() == RECENT_SEQS_PER_SHARD {
            entry.recent_seqs.pop_front();
        }
        entry.recent_seqs.push_back(seq);
        let recent_seqs: Vec<u64> = entry.recent_seqs.iter().copied().collect();

        if success {
            entry.last_success = Some(now);
        }

        if newly_seen {
            if state.shard_open_history.len() == SHARD_OPEN_HISTORY_CAP {
                state.shard_open_history.pop_front();
            }
            state.shard_open_history.push_back((shard_id, now));
        }

        // Prune entries that fell out of the retention window.
        while let Some(&(_, t)) = state.shard_open_history.front() {
            if now.saturating_duration_since(t) > SHARD_OPEN_HISTORY_TTL {
                state.shard_open_history.pop_front();
            } else {
                break;
            }
        }

        let mut other_shards: Vec<u64> = state
            .shard_open_history
            .iter()
            .filter(|(id, t)| {
                *id != shard_id && now.saturating_duration_since(*t) <= OTHER_SHARDS_WINDOW
            })
            .map(|(id, _)| *id)
            .collect();
        other_shards.sort_unstable();
        other_shards.dedup();
        let other_shards_opened_within_100ms = !other_shards.is_empty();

        ShardSnapshot {
            shard_id,
            observed_age_ms: now.saturating_duration_since(first_seen).as_millis() as u64,
            ms_since_last_success_on_shard: ms_since_last_success,
            request_seq_on_shard: seq,
            recent_request_seqs_on_shard: recent_seqs,
            other_shards_opened_within_100ms,
            other_shards_opened_recently: other_shards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_observation_has_no_prior_success_and_seq_zero() {
        let obs = ShardObserver::new();
        let now = Instant::now();
        let snap = obs.observe_at(42, false, now);
        assert_eq!(snap.shard_id, 42);
        assert_eq!(snap.request_seq_on_shard, 0);
        assert_eq!(snap.recent_request_seqs_on_shard, vec![0]);
        assert!(snap.ms_since_last_success_on_shard.is_none());
        assert_eq!(snap.observed_age_ms, 0);
        assert!(!snap.other_shards_opened_within_100ms);
    }

    #[test]
    fn recent_seqs_ring_keeps_last_ten_oldest_to_newest() {
        let obs = ShardObserver::new();
        let now = Instant::now();
        for i in 0..15 {
            obs.observe_at(7, false, now + Duration::from_millis(i));
        }
        let snap = obs.observe_at(7, false, now + Duration::from_millis(100));
        // 15 prior observations + 1 final = 16 total. Ring holds the
        // last 10, so 6..=15.
        assert_eq!(snap.request_seq_on_shard, 15);
        assert_eq!(
            snap.recent_request_seqs_on_shard,
            vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    #[test]
    fn ms_since_last_success_resets_on_success_and_accumulates_on_failure() {
        let obs = ShardObserver::new();
        let t0 = Instant::now();
        obs.observe_at(1, true, t0);
        let snap1 = obs.observe_at(1, false, t0 + Duration::from_millis(50));
        assert_eq!(snap1.ms_since_last_success_on_shard, Some(50));

        // Another failure does not reset the success timestamp.
        let snap2 = obs.observe_at(1, false, t0 + Duration::from_millis(120));
        assert_eq!(snap2.ms_since_last_success_on_shard, Some(120));

        // A success resets the clock; the next failure measures from there.
        obs.observe_at(1, true, t0 + Duration::from_millis(200));
        let snap3 = obs.observe_at(1, false, t0 + Duration::from_millis(225));
        assert_eq!(snap3.ms_since_last_success_on_shard, Some(25));
    }

    #[test]
    fn other_shards_opened_within_window_are_reported() {
        let obs = ShardObserver::new();
        let t0 = Instant::now();
        // Shard 1 opens at t0.
        obs.observe_at(1, true, t0);
        // Shard 2 opens 30ms later.
        obs.observe_at(2, true, t0 + Duration::from_millis(30));
        // Shard 3 opens 60ms after t0 (30ms after shard 2).
        obs.observe_at(3, true, t0 + Duration::from_millis(60));

        // Failure on shard 3 at t0+80: shards 1 (80ms ago) and 2 (50ms ago)
        // — shard 1 is outside the 100ms window? 80ms <= 100ms, so it IS
        // inside. Shard 2 at 50ms ago is inside. Both reported.
        let snap = obs.observe_at(3, false, t0 + Duration::from_millis(80));
        assert!(snap.other_shards_opened_within_100ms);
        assert_eq!(snap.other_shards_opened_recently, vec![1, 2]);

        // Same failure 200ms after t0: shard 1 first-seen 200ms ago (out),
        // shard 2 first-seen 170ms ago (out), nobody else opened.
        let snap_late = obs.observe_at(3, false, t0 + Duration::from_millis(200));
        assert!(!snap_late.other_shards_opened_within_100ms);
        assert!(snap_late.other_shards_opened_recently.is_empty());
    }

    #[test]
    fn other_shards_excludes_self_and_dedupes() {
        let obs = ShardObserver::new();
        let t0 = Instant::now();
        // The "current" shard's own first-observation should not be
        // counted under "other shards".
        let snap = obs.observe_at(99, false, t0);
        assert!(!snap.other_shards_opened_within_100ms);
        assert!(snap.other_shards_opened_recently.is_empty());

        // Re-observing shard 99 doesn't put it in the recent-opens list
        // (it's not "newly seen").
        let snap2 = obs.observe_at(99, false, t0 + Duration::from_millis(5));
        assert!(snap2.other_shards_opened_recently.is_empty());
    }
}
