// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    try_cleanup_probe_candidate, HealthStatus, LocationStateStore, OperationRetryState,
    PartitionKeyRangeId,
};
use crate::{
    driver::{
        cache::AccountMetadataCache,
        routing::{partition_endpoint_state::PartitionFailoverEntry, CosmosEndpoint},
    },
    models::AccountEndpoint,
    options::PartitionFailoverOptions,
};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use url::Url;

type BeforeUpdate = Box<dyn FnOnce(&LocationStateStore)>;

thread_local! {
    static BEFORE_UPDATE: RefCell<Option<BeforeUpdate>> = const { RefCell::new(None) };
}

pub(super) fn before_update(store: &LocationStateStore) {
    let hook = BEFORE_UPDATE.with(|slot| slot.borrow_mut().take());
    if let Some(hook) = hook {
        hook(store);
    }
}

fn with_before_update(hook: impl FnOnce(&LocationStateStore) + 'static, action: impl FnOnce()) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            BEFORE_UPDATE.with(|slot| slot.borrow_mut().take());
        }
    }

    BEFORE_UPDATE.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none());
        *slot = Some(Box::new(hook));
    });
    let _reset = Reset;
    action();
}

fn store_with_probe() -> (LocationStateStore, OperationRetryState) {
    let account = AccountEndpoint::from("https://test.documents.azure.com".parse::<Url>().unwrap());
    let endpoint = CosmosEndpoint::global(account.url().clone());
    let store = LocationStateStore::new(
        Arc::new(AccountMetadataCache::new()),
        account,
        endpoint.clone(),
        Arc::new(|_, _| Box::pin(async { panic!("probe cleanup must not refresh metadata") })),
        false,
        Duration::from_secs(60),
        PartitionFailoverOptions::default(),
        Vec::new(),
        None,
    );
    let key: PartitionKeyRangeId = "0".parse().unwrap();
    store.apply_partition(|current| {
        let mut next = current.clone();
        next.circuit_breaker_overrides.insert(
            key.clone(),
            PartitionFailoverEntry {
                current_endpoint: endpoint.clone(),
                first_failed_endpoint: endpoint.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 10,
                write_failure_count: 5,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::ProbeCandidate,
                failback_jitter: Duration::ZERO,
            },
        );
        next
    });
    let mut retry = OperationRetryState::initial(0, false, Vec::new(), 3, 2);
    retry.partition_key_range_id = Some(key);
    (store, retry)
}

#[test]
fn failure_after_fast_path_check_preserves_state_and_snapshot() {
    let (store, retry) = store_with_probe();
    let version = store.version_for_tests();
    let captured = Arc::new(Mutex::new(None));
    let saved = captured.clone();

    with_before_update(
        move |store| {
            store.apply_partition(|current| {
                let mut next = current.clone();
                let entry = next.circuit_breaker_overrides.get_mut("0").unwrap();
                assert_eq!(entry.health_status, HealthStatus::ProbeCandidate);
                entry.health_status = HealthStatus::Unhealthy;
                entry.read_failure_count = 42;
                next
            });
            *saved.lock().unwrap() = Some(store.snapshot());
        },
        || try_cleanup_probe_candidate(&retry, &store),
    );

    let failed_state = captured
        .lock()
        .unwrap()
        .take()
        .expect("fast path reached the update hook");
    let after = store.snapshot();
    let entry = after.partitions.circuit_breaker_overrides.get("0").unwrap();
    assert_eq!(entry.health_status, HealthStatus::Unhealthy);
    assert_eq!(entry.read_failure_count, 42);
    assert_eq!(
        store.version_for_tests(),
        version + 1,
        "only the failure update may publish"
    );
    assert!(Arc::ptr_eq(&failed_state.partitions, &after.partitions));
}

#[test]
fn unchanged_probe_is_removed_with_one_publication() {
    let (store, retry) = store_with_probe();
    let before = store.snapshot();
    let version = store.version_for_tests();
    try_cleanup_probe_candidate(&retry, &store);
    let after = store.snapshot();
    assert!(after.partitions.circuit_breaker_overrides.is_empty());
    assert_eq!(store.version_for_tests(), version + 1);
    assert!(!Arc::ptr_eq(&before.partitions, &after.partitions));
}
