// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    epoch, record_hedge_primary_win, tests::build_store_for_ppaf_tests, LocationStateStore,
    Ordering, PartitionKeyRangeId,
};
use crate::options::Region;
use std::{
    sync::{atomic::AtomicUsize, Arc, Barrier},
    time::{Duration, Instant},
};

fn store() -> LocationStateStore {
    let store = build_store_for_ppaf_tests();
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_circuit_breaker_enabled = true;
        for id in 0..100 {
            next.consecutive_hedge_wins.insert(
                (
                    PartitionKeyRangeId::from(id.to_string()),
                    Some(Region::EAST_US),
                ),
                1,
            );
        }
        next
    });
    store
}

#[test]
fn primary_noop_preserves_state_and_cached_snapshot() {
    let store = store();
    for (partition, region) in [("absent", Region::EAST_US), ("0", Region::WEST_US)] {
        let before = store.snapshot();
        let version = store.version_for_tests();
        let guard = epoch::pin();
        let pointer = store.partitions.load(Ordering::Acquire, &guard);
        store.record_primary_win(&partition.parse().unwrap(), Some(&region));
        let after = store.snapshot();
        assert_eq!(
            before.partitions.consecutive_hedge_wins,
            after.partitions.consecutive_hedge_wins
        );
        assert_eq!(store.account_version.load(Ordering::Acquire), version);
        assert_eq!(pointer, store.partitions.load(Ordering::Acquire, &guard));
        assert!(Arc::ptr_eq(&before.partitions, &after.partitions));
    }
    let partition = "0".parse().unwrap();
    store.record_primary_win(&partition, Some(&Region::EAST_US));
    assert_eq!(store.snapshot().partitions.consecutive_hedge_wins.len(), 99);
    let version = store.account_version.load(Ordering::Acquire);
    store.record_primary_win(&partition, Some(&Region::EAST_US));
    assert_eq!(store.snapshot().partitions.consecutive_hedge_wins.len(), 99);
    assert_eq!(store.account_version.load(Ordering::Acquire), version);
}

#[test]
fn competing_resets_recheck_after_cas_failure() {
    let store = store();
    let barrier = Barrier::new(2);
    let attempts = AtomicUsize::new(0);
    let version = store.account_version.load(Ordering::Acquire);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                let mut first = true;
                store.apply_partition(|current| {
                    attempts.fetch_add(1, Ordering::Relaxed);
                    if first {
                        first = false;
                        barrier.wait();
                    }
                    record_hedge_primary_win(current, &"0".parse().unwrap(), Some(&Region::EAST_US))
                });
            });
        }
    });
    assert_eq!(store.account_version.load(Ordering::Acquire), version + 1);
    assert_eq!(attempts.load(Ordering::Relaxed), 3);
    assert_eq!(store.snapshot().partitions.consecutive_hedge_wins.len(), 99);
}

#[test]
fn disabled_feedback_preserves_existing_counter() {
    let store = store();
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_circuit_breaker_enabled = false;
        next
    });
    let before = store.snapshot();
    let version = store.account_version.load(Ordering::Acquire);
    let key = "0".parse().unwrap();
    store.record_primary_win(&key, Some(&Region::EAST_US));
    store.record_consecutive_hedge_win(&key, Some(&Region::EAST_US));
    assert_eq!(store.account_version.load(Ordering::Acquire), version);
    assert!(Arc::ptr_eq(
        &before.partitions,
        &store.snapshot().partitions
    ));
}

#[test]
fn concurrent_alternate_wins_are_not_lost() {
    let store = store();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..16 {
                    store.record_consecutive_hedge_win(&"same".parse().unwrap(), None);
                }
            });
        }
    });
    assert_eq!(
        store.snapshot().partitions.consecutive_hedge_wins[&("same".parse().unwrap(), None)],
        64
    );
}

#[test]
fn idle_failback_preserves_snapshot() {
    let store = store();
    let before = store.snapshot();
    let version = store.account_version.load(Ordering::Acquire);
    store.apply_partition(|current| {
        super::expire_partition_overrides(current, Instant::now(), Duration::from_secs(3600))
    });
    assert_eq!(store.account_version.load(Ordering::Acquire), version);
    assert!(Arc::ptr_eq(
        &before.partitions,
        &store.snapshot().partitions
    ));
}
