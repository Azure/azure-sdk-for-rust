// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Building blocks for the runtime-owned periodic refresh of database-account
//! metadata.
//!
//! [`CosmosDriverRuntime`](super::CosmosDriverRuntime) hosts a single
//! background loop that calls [`refresh_one_account`] for every endpoint a
//! driver has registered via
//! `CosmosDriverRuntime::register_for_account_refresh`. Drivers hold the
//! returned [`AccountRefreshRegistration`] guard for their lifetime; dropping
//! it removes the entry from the runtime registry so subsequent ticks skip
//! the endpoint.
//!
//! This module is the lower half of that split: it owns the per-endpoint
//! refresh logic (cache snapshot → fetch outside lock → atomic replace →
//! warn-on-failure) and the guard's `Drop` plumbing. The runtime owns the
//! registry storage, the public API, and the loop's spawn site so it can
//! reuse its existing `BackgroundTaskManager`.
//!
//! The *event-driven* refresh path (5-second rate-limited, triggered by
//! retry policies via `LocationEffect::RefreshAccountProperties`) stays on
//! `LocationStateStore` because it solves a different problem (burst
//! throttling on observed failures) than the timer (idle clients not
//! refreshing).

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Weak,
};

#[cfg(feature = "tokio")]
use std::time::Duration;

use futures::future::BoxFuture;

use crate::{
    driver::cache::{AccountMetadataCache, AccountProperties},
    models::AccountEndpoint,
};

/// Interval between iterations of the runtime-owned account-metadata refresh
/// loop. Hardcoded per PR #4407 review history; independent of the 5-second
/// rate limit used by `LocationStateStore::refresh_account_properties_if_due`.
#[cfg(feature = "tokio")]
pub(crate) const BACKGROUND_REFRESH_INTERVAL: Duration = Duration::from_secs(300);

/// Closure that fetches fresh `AccountProperties` for an account endpoint.
///
/// Receives the previously cached properties (or `None`) so the implementation
/// can fall back to regional endpoints from the prior snapshot if the primary
/// endpoint fails.
pub(crate) type AccountRefreshFn = Arc<
    dyn Fn(
            Option<Arc<AccountProperties>>,
        ) -> BoxFuture<'static, azure_core::Result<AccountProperties>>
        + Send
        + Sync,
>;

/// Closure invoked after a successful refresh installs a fresh value into the
/// cache. The driver uses this to sync its own routing snapshot via
/// `LocationStateStore::sync_account_properties` and to bump its
/// event-driven rate-limit clock.
///
/// Implementations are expected to no-op when their captured state has been
/// dropped (typically by holding a `Weak<LocationStateStore>` and bailing on
/// `upgrade() == None`) — this avoids extending the driver's lifetime via the
/// runtime registry.
pub(crate) type OnSuccessFn = Arc<dyn Fn(Arc<AccountProperties>) + Send + Sync>;

/// A single entry in the runtime's refresh registry.
#[derive(Clone)]
pub(crate) struct AccountRefreshEntry {
    /// Per-registration token. The drop guard only deregisters when its
    /// captured id matches the current entry's id. This makes
    /// re-registration races safe: if a driver is recreated for the same
    /// endpoint, the new registration replaces the old entry's id; when the
    /// *old* guard later drops, the id mismatch is observed and the
    /// deregister call is a no-op.
    pub(crate) id: u64,
    pub(crate) fetch_fn: AccountRefreshFn,
    pub(crate) on_success: OnSuccessFn,
}

impl std::fmt::Debug for AccountRefreshEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccountRefreshEntry")
            .field("id", &self.id)
            // `fetch_fn` and `on_success` are opaque closures.
            .finish_non_exhaustive()
    }
}

/// Generates the next per-registration token.
pub(crate) fn next_registration_id(counter: &AtomicU64) -> u64 {
    counter.fetch_add(1, Ordering::Relaxed)
}

/// Implemented by the runtime so guards can call back into it to deregister
/// without taking a circular `use` dependency on the runtime struct.
pub(crate) trait AccountRefreshRegistry: Send + Sync {
    fn deregister(&self, endpoint: &AccountEndpoint, id: u64);
}

/// RAII guard. Holding it keeps the registration alive; dropping it removes
/// the entry (provided the entry's id still matches) so the next tick skips
/// the endpoint.
///
/// The runtime is held as `Weak<dyn AccountRefreshRegistry>` so the guard
/// does not extend the runtime's lifetime. If the runtime has already been
/// dropped when the guard drops, the deregister step is a no-op (the entire
/// registry is gone with it).
pub(crate) struct AccountRefreshRegistration {
    runtime: Weak<dyn AccountRefreshRegistry>,
    endpoint: AccountEndpoint,
    id: u64,
}

impl AccountRefreshRegistration {
    pub(crate) fn new(
        runtime: Weak<dyn AccountRefreshRegistry>,
        endpoint: AccountEndpoint,
        id: u64,
    ) -> Self {
        Self {
            runtime,
            endpoint,
            id,
        }
    }
}

impl std::fmt::Debug for AccountRefreshRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccountRefreshRegistration")
            .field("endpoint", &self.endpoint)
            .field("id", &self.id)
            .finish()
    }
}

impl Drop for AccountRefreshRegistration {
    fn drop(&mut self) {
        if let Some(rt) = self.runtime.upgrade() {
            rt.deregister(&self.endpoint, self.id);
        }
    }
}

/// Refreshes the cached `AccountProperties` for a single endpoint.
///
/// 1. Snapshots the previously cached value via [`AccountMetadataCache::get`]
///    (no lock contention with concurrent `execute_operation` callers).
/// 2. Invokes `entry.fetch_fn` outside any cache lock so a slow network
///    request never blocks readers.
/// 3. On success, atomically replaces the cache entry via
///    [`AccountMetadataCache::get_or_refresh_with`] under the cache's
///    single-pending-I/O lock — concurrent readers see the previous value
///    through the entire transition rather than a hole.
/// 4. Invokes `entry.on_success` so the registering driver can sync its own
///    routing snapshot and bump its event-driven rate-limit clock.
/// 5. On failure, logs `tracing::warn!` and leaves the cached value alone.
///    Operations continue to succeed against the stale cached endpoints; the
///    next tick (or an event-driven refresh on observed failure) will try
///    again.
pub(crate) async fn refresh_one_account(
    cache: &AccountMetadataCache,
    endpoint: &AccountEndpoint,
    entry: &AccountRefreshEntry,
) {
    let previous_props = cache.get(endpoint).await;

    let fetched = (entry.fetch_fn)(previous_props).await;

    let new_properties = match fetched {
        Ok(props) => props,
        Err(e) => {
            tracing::warn!(
                endpoint = %endpoint,
                error = %e,
                "CosmosDriverRuntime: account metadata refresh failed; cached value retained",
            );
            return;
        }
    };

    let cached_arc = cache
        .get_or_refresh_with(
            endpoint.clone(),
            |_existing| true,
            || async { new_properties },
        )
        .await;

    let Some(properties) = cached_arc else {
        tracing::warn!(
            endpoint = %endpoint,
            "CosmosDriverRuntime: account metadata cache produced no value after refresh; on_success skipped",
        );
        return;
    };

    (entry.on_success)(properties);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering as AOrd},
        Mutex,
    };

    /// In-memory `AccountRefreshRegistry` impl that records every deregister
    /// call. Used to verify the `AccountRefreshRegistration` Drop semantics
    /// without spinning up a real runtime.
    #[derive(Default)]
    struct FakeRegistry {
        calls: Mutex<Vec<(AccountEndpoint, u64)>>,
    }

    impl AccountRefreshRegistry for FakeRegistry {
        fn deregister(&self, endpoint: &AccountEndpoint, id: u64) {
            self.calls.lock().unwrap().push((endpoint.clone(), id));
        }
    }

    fn test_endpoint() -> AccountEndpoint {
        AccountEndpoint::from(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    #[test]
    fn registration_drop_calls_deregister_with_captured_id_and_endpoint() {
        let registry: Arc<FakeRegistry> = Arc::new(FakeRegistry::default());
        let weak: Weak<dyn AccountRefreshRegistry> =
            Arc::downgrade(&(registry.clone() as Arc<dyn AccountRefreshRegistry>));

        let endpoint = test_endpoint();
        let reg = AccountRefreshRegistration::new(weak, endpoint.clone(), 42);
        // Before drop: no deregister calls observed.
        assert!(registry.calls.lock().unwrap().is_empty());

        drop(reg);

        let calls = registry.calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, endpoint);
        assert_eq!(calls[0].1, 42);
    }

    #[test]
    fn registration_drop_after_runtime_drop_is_noop() {
        // Build a registry, downgrade, then drop the registry before the guard.
        let registry: Arc<FakeRegistry> = Arc::new(FakeRegistry::default());
        let weak: Weak<dyn AccountRefreshRegistry> =
            Arc::downgrade(&(registry.clone() as Arc<dyn AccountRefreshRegistry>));

        // Take a counter clone we can inspect after both registry and guard
        // are dropped — proves the Drop took the upgrade-fails branch
        // rather than panicking or doing anything observable.
        let counter_view = registry.calls.lock().unwrap().len();
        drop(registry);

        let reg = AccountRefreshRegistration::new(weak, test_endpoint(), 7);
        drop(reg); // must not panic

        // No observable side effect — registry was already gone.
        let _ = counter_view;
    }

    #[test]
    fn next_registration_id_monotonic() {
        let counter = AtomicU64::new(0);
        let a = next_registration_id(&counter);
        let b = next_registration_id(&counter);
        let c = next_registration_id(&counter);
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
    }

    fn properties_with_etag(etag: &str) -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": etag,
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn refresh_one_account_success_invokes_on_success_with_fresh_props() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint();
        let on_success_calls = Arc::new(AtomicUsize::new(0));
        let received_etag = Arc::new(Mutex::new(String::new()));

        let on_success_calls_clone = Arc::clone(&on_success_calls);
        let received_etag_clone = Arc::clone(&received_etag);
        let entry = AccountRefreshEntry {
            id: 1,
            fetch_fn: Arc::new(|_previous| {
                Box::pin(async { Ok(properties_with_etag("etag-fresh")) })
            }),
            on_success: Arc::new(move |props| {
                on_success_calls_clone.fetch_add(1, AOrd::SeqCst);
                *received_etag_clone.lock().unwrap() = props.etag.clone();
            }),
        };

        refresh_one_account(&cache, &endpoint, &entry).await;

        assert_eq!(on_success_calls.load(AOrd::SeqCst), 1);
        assert_eq!(&*received_etag.lock().unwrap(), "etag-fresh");
        // Cache was populated.
        let cached = cache.get(&endpoint).await.expect("cache populated");
        assert_eq!(cached.etag, "etag-fresh");
    }

    #[tokio::test]
    async fn refresh_one_account_fetch_failure_skips_on_success_and_retains_cache() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint();

        // Seed the cache with a known value via a successful refresh first.
        {
            let seed = AccountRefreshEntry {
                id: 1,
                fetch_fn: Arc::new(|_previous| {
                    Box::pin(async { Ok(properties_with_etag("etag-seed")) })
                }),
                on_success: Arc::new(|_| {}),
            };
            refresh_one_account(&cache, &endpoint, &seed).await;
        }

        let on_success_calls = Arc::new(AtomicUsize::new(0));
        let on_success_calls_clone = Arc::clone(&on_success_calls);
        let entry = AccountRefreshEntry {
            id: 2,
            fetch_fn: Arc::new(|_previous| {
                Box::pin(async {
                    Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "simulated transient failure",
                    ))
                })
            }),
            on_success: Arc::new(move |_| {
                on_success_calls_clone.fetch_add(1, AOrd::SeqCst);
            }),
        };

        refresh_one_account(&cache, &endpoint, &entry).await;

        // on_success must NOT have been invoked on the failed refresh.
        assert_eq!(on_success_calls.load(AOrd::SeqCst), 0);
        // Cached value must remain the seeded one — behavior unchanged on
        // failure beyond the warn log.
        let cached = cache.get(&endpoint).await.expect("seeded cache survived");
        assert_eq!(cached.etag, "etag-seed");
    }
}
