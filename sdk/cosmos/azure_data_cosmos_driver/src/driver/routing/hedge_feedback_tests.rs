// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    advance_hub_region_discovery, cache_hub_region, epoch, mark_partition_unavailable,
    record_hedge_primary_win, tests::build_store_for_ppaf_tests, CosmosEndpoint,
    LocationStateStore, Ordering, PartitionEndpointState, PartitionKeyRangeId,
    PartitionStateUpdate,
};
use crate::{driver::routing::UnavailablePartition, options::Region};
use std::{
    process::Command,
    sync::{atomic::AtomicUsize, Arc, Barrier, Mutex},
    time::{Duration, Instant},
};
use tracing::{Event, Subscriber};
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    Layer,
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
        PartitionStateUpdate::Updated(next)
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
    let publications = AtomicUsize::new(0);
    let version = store.account_version.load(Ordering::Acquire);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                let mut first = true;
                let updated = store.apply_partition(|current| {
                    attempts.fetch_add(1, Ordering::Relaxed);
                    if first {
                        first = false;
                        barrier.wait();
                    }
                    record_hedge_primary_win(current, &"0".parse().unwrap(), Some(&Region::EAST_US))
                });
                if updated {
                    publications.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    assert_eq!(store.account_version.load(Ordering::Acquire), version + 1);
    assert_eq!(attempts.load(Ordering::Relaxed), 3);
    assert_eq!(publications.load(Ordering::Relaxed), 1);
    assert_eq!(store.snapshot().partitions.consecutive_hedge_wins.len(), 99);
}

#[test]
fn disabled_feedback_preserves_existing_counter() {
    let store = store();
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_circuit_breaker_enabled = false;
        PartitionStateUpdate::Updated(next)
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

fn assert_unchanged(
    store: &LocationStateStore,
    mut transition: impl FnMut(&PartitionEndpointState) -> PartitionStateUpdate,
) {
    let before = store.snapshot();
    let version = store.version_for_tests();
    let guard = epoch::pin();
    let pointer = store.partitions.load(Ordering::Acquire, &guard);
    let updated = store.apply_partition(|current| {
        let update = transition(current);
        assert!(matches!(update, PartitionStateUpdate::Unchanged));
        update
    });
    assert!(!updated);
    assert_eq!(store.version_for_tests(), version);
    assert_eq!(pointer, store.partitions.load(Ordering::Acquire, &guard));
    assert!(Arc::ptr_eq(
        &before.partitions,
        &store.snapshot().partitions
    ));
}

struct FeedbackLogVersions {
    store: Arc<LocationStateStore>,
    versions: Arc<Mutex<Vec<u64>>>,
}

impl<S: Subscriber> Layer<S> for FeedbackLogVersions {
    fn on_event(&self, event: &Event<'_>, _context: Context<'_, S>) {
        if event.metadata().target()
            == "azure_data_cosmos_driver::driver::routing::location_state_store"
            && event.fields().any(|field| field.name() == "partition")
        {
            self.versions
                .lock()
                .unwrap()
                .push(self.store.version_for_tests());
        }
    }
}

#[test]
fn hedge_logs_follow_published_updates() {
    // Isolate tracing's global callsite cache from concurrently running tests.
    if std::env::var_os("COSMOS_HEDGE_LOG_TEST_CHILD").is_none() {
        let test = concat!(module_path!(), "::hedge_logs_follow_published_updates");
        let test = test.split_once("::").unwrap().1;
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", test, "--nocapture", "--test-threads=1"])
            .env("COSMOS_HEDGE_LOG_TEST_CHILD", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "isolated log test failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let store = Arc::new(store());
    let versions = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(FeedbackLogVersions {
        store: Arc::clone(&store),
        versions: Arc::clone(&versions),
    });
    let initial_version = store.version_for_tests();
    tracing::subscriber::with_default(subscriber, || {
        let key = "logged".parse().unwrap();
        store.record_primary_win(&key, None);
        store.record_consecutive_hedge_win(&key, None);
        store.record_primary_win(&key, None);
        store.record_primary_win(&key, None);
        store.apply_partition(|current| {
            let mut next = current.clone();
            next.per_partition_circuit_breaker_enabled = false;
            PartitionStateUpdate::Updated(next)
        });
        let existing = "0".parse().unwrap();
        store.record_primary_win(&existing, Some(&Region::EAST_US));
        store.record_consecutive_hedge_win(&existing, Some(&Region::EAST_US));
    });
    assert_eq!(
        *versions.lock().unwrap(),
        vec![initial_version + 1, initial_version + 2]
    );
}

#[test]
fn hub_noop_transitions_preserve_snapshot() {
    let store = store();
    let key = "hub".parse().unwrap();
    let hub = CosmosEndpoint::regional(
        Region::EAST_US,
        "https://test-eastus.documents.azure.com/".parse().unwrap(),
    );
    let account = store.snapshot().account;
    assert_unchanged(&store, |current| cache_hub_region(current, &key, &hub));
    assert_unchanged(&store, |current| {
        advance_hub_region_discovery(current, &account, &key, &hub)
    });

    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_automatic_failover_enabled = true;
        PartitionStateUpdate::Updated(next)
    });
    assert_unchanged(&store, |current| {
        cache_hub_region(current, &key, &account.default_endpoint)
    });
    let version = store.version_for_tests();
    store.apply_partition(|current| cache_hub_region(current, &key, &hub));
    assert_eq!(store.version_for_tests(), version + 1);
    let before = store.snapshot();
    assert_eq!(
        before.partitions.failover_overrides[&key].current_endpoint,
        hub
    );
    assert_unchanged(&store, |current| cache_hub_region(current, &key, &hub));
    assert_eq!(
        before.partitions.failover_overrides[&key].last_failure_time,
        store.snapshot().partitions.failover_overrides[&key].last_failure_time
    );
}

#[test]
fn unavailable_noop_transitions_preserve_snapshot() {
    let store = store();
    let mut account = (*store.snapshot().account).clone();
    let endpoint = CosmosEndpoint::regional(
        Region::EAST_US,
        "https://test-eastus.documents.azure.com/".parse().unwrap(),
    );
    account.preferred_read_endpoints = vec![
        endpoint,
        CosmosEndpoint::regional(
            Region::WEST_US,
            "https://test-westus.documents.azure.com/".parse().unwrap(),
        ),
    ]
    .into();
    let mut unavail = UnavailablePartition {
        partition_key_range_id: None,
        region: Some(Region::EAST_US),
        is_read: true,
        is_partitioned_resource: true,
    };
    assert_unchanged(&store, |current| {
        mark_partition_unavailable(current, &account, &unavail, true)
    });
    unavail.partition_key_range_id = Some("missing".parse().unwrap());
    unavail.region = Some(Region::NORTH_EUROPE);
    assert_unchanged(&store, |current| {
        mark_partition_unavailable(current, &account, &unavail, true)
    });
    unavail.region = None;
    assert_unchanged(&store, |current| {
        mark_partition_unavailable(current, &account, &unavail, true)
    });
    unavail.region = Some(Region::EAST_US);
    assert_unchanged(&store, |current| {
        mark_partition_unavailable(current, &account, &unavail, false)
    });
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_circuit_breaker_enabled = false;
        PartitionStateUpdate::Updated(next)
    });
    assert_unchanged(&store, |current| {
        mark_partition_unavailable(current, &account, &unavail, true)
    });
}

#[test]
fn exhausted_cold_transitions_preserve_snapshot_but_existing_removals_publish() {
    let store = store();
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.per_partition_automatic_failover_enabled = true;
        PartitionStateUpdate::Updated(next)
    });
    let key: PartitionKeyRangeId = "exhausted".parse().unwrap();
    let endpoint = CosmosEndpoint::regional(
        Region::EAST_US,
        "https://test-eastus.documents.azure.com/".parse().unwrap(),
    );
    let mut account = (*store.snapshot().account).clone();
    account.preferred_read_endpoints = vec![endpoint.clone()].into();
    assert_unchanged(&store, |current| {
        advance_hub_region_discovery(current, &account, &key, &endpoint)
    });
    let alternate = CosmosEndpoint::regional(
        Region::WEST_US,
        "https://test-westus.documents.azure.com/".parse().unwrap(),
    );
    account.preferred_read_endpoints = vec![endpoint.clone(), alternate.clone()].into();
    let unavail = UnavailablePartition {
        partition_key_range_id: Some(key.clone()),
        region: Some(Region::EAST_US),
        is_read: false,
        is_partitioned_resource: true,
    };
    for discovery in [false, true] {
        store.apply_partition(|current| {
            let mut next = cache_hub_region(current, &key, &endpoint).apply(current);
            next.failover_overrides
                .get_mut(&key)
                .unwrap()
                .failed_endpoints
                .insert(alternate.clone());
            PartitionStateUpdate::Updated(next)
        });
        let version = store.version_for_tests();
        store.apply_partition(|current| {
            if discovery {
                advance_hub_region_discovery(current, &account, &key, &endpoint)
            } else {
                mark_partition_unavailable(current, &account, &unavail, true)
            }
        });
        assert_eq!(store.version_for_tests(), version + 1);
        assert!(store.snapshot().partitions.failover_overrides.is_empty());
    }
}
