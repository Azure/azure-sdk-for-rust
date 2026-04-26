// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unified lock-free location state store.

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crossbeam_epoch::{self as epoch, Atomic, Owned};
use futures::future::BoxFuture;

use crate::{
    driver::cache::{AccountMetadataCache, AccountProperties},
    models::AccountEndpoint,
    options::Region,
};

use super::{
    build_account_endpoint_state, expire_unavailable_endpoints, mark_endpoint_unavailable,
    AccountEndpointState, CosmosEndpoint, LocationEffect,
};

/// Placeholder for partition-level state (not yet implemented).
#[derive(Clone, Debug, Default)]
pub(crate) struct PartitionEndpointState;

/// Immutable location snapshot consumed by one operation-loop iteration.
#[derive(Clone, Debug)]
pub(crate) struct LocationSnapshot {
    pub account: Arc<AccountEndpointState>,
    #[allow(dead_code)]
    pub partitions: Arc<PartitionEndpointState>,
}

#[cfg(test)]
impl LocationSnapshot {
    pub(crate) fn for_tests(account: Arc<AccountEndpointState>) -> Self {
        Self {
            account,
            partitions: Arc::new(PartitionEndpointState),
        }
    }
}

type AccountRefreshFn = Arc<
    dyn Fn(
            Option<Arc<AccountProperties>>,
        ) -> BoxFuture<'static, azure_core::Result<AccountProperties>>
        + Send
        + Sync,
>;

/// Unified location state store with lock-free reads and CAS-loop writes.
pub(crate) struct LocationStateStore {
    account: Atomic<AccountEndpointState>,
    partitions: Atomic<PartitionEndpointState>,
    account_metadata_cache: Arc<AccountMetadataCache>,
    account_endpoint: AccountEndpoint,
    account_refresh_fn: AccountRefreshFn,
    default_endpoint: CosmosEndpoint,
    preferred_regions: Vec<Region>,
    gateway20_enabled: bool,
    endpoint_unavailability_ttl: Duration,
    refresh_interval: Duration,
    last_refresh_epoch_ms: AtomicU64,
    /// The etag of the last `AccountProperties` that was synced.
    /// Used to skip the CAS loop when the account metadata hasn't changed.
    last_synced_etag: std::sync::Mutex<String>,
    /// Pointer identity of the last synced `AccountProperties` arc.
    /// When `sync_account_properties` is called with the same `Arc` (i.e.,
    /// the account metadata cache returned the same cached value), the entire
    /// sync is skipped without acquiring any other locks or rebuilding endpoint
    /// lists.
    last_synced_properties: std::sync::Mutex<Option<Arc<AccountProperties>>>,
    /// Monotonic version counter bumped on every successful CAS write.
    account_version: AtomicU64,
    /// Cached snapshot: (version, snapshot). When the version matches
    /// `account_version`, `snapshot()` returns `Arc::clone()` of the cached
    /// arcs (refcount increment only) instead of a full clone.
    cached_snapshot: std::sync::Mutex<(u64, LocationSnapshot)>,
}

impl std::fmt::Debug for LocationStateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocationStateStore")
            .field("account_endpoint", &self.account_endpoint)
            .field("default_endpoint", &self.default_endpoint)
            .field(
                "endpoint_unavailability_ttl",
                &self.endpoint_unavailability_ttl,
            )
            .field("refresh_interval", &self.refresh_interval)
            .finish_non_exhaustive()
    }
}

impl Drop for LocationStateStore {
    fn drop(&mut self) {
        // crossbeam_epoch::Atomic<T> intentionally has no destructor.
        // Detach each field and convert to Owned for immediate deallocation.
        // Safety: &mut self in Drop guarantees exclusive access.
        let account = std::mem::replace(&mut self.account, Atomic::null());
        let partitions = std::mem::replace(&mut self.partitions, Atomic::null());

        unsafe {
            if let Some(account) = account.try_into_owned() {
                drop(account);
            }

            if let Some(partitions) = partitions.try_into_owned() {
                drop(partitions);
            }
        }
    }
}

impl LocationStateStore {
    /// Creates a new location store with a single-endpoint account snapshot.
    pub fn new(
        account_metadata_cache: Arc<AccountMetadataCache>,
        account_endpoint: AccountEndpoint,
        default_endpoint: CosmosEndpoint,
        account_refresh_fn: AccountRefreshFn,
        gateway20_enabled: bool,
        endpoint_unavailability_ttl: Duration,
        preferred_regions: Vec<Region>,
    ) -> Self {
        let account_state = AccountEndpointState::single(default_endpoint.clone());

        let initial_snapshot = LocationSnapshot {
            account: Arc::new(account_state.clone()),
            partitions: Arc::new(PartitionEndpointState),
        };

        Self {
            account: Atomic::new(account_state),
            partitions: Atomic::new(PartitionEndpointState),
            account_metadata_cache,
            account_endpoint,
            account_refresh_fn,
            default_endpoint,
            preferred_regions,
            gateway20_enabled,
            endpoint_unavailability_ttl,
            // TODO(refresh-config): Make refresh interval configurable.
            refresh_interval: Duration::from_secs(5),
            last_refresh_epoch_ms: AtomicU64::new(0),
            last_synced_etag: std::sync::Mutex::new(String::new()),
            last_synced_properties: std::sync::Mutex::new(None),
            account_version: AtomicU64::new(0),
            cached_snapshot: std::sync::Mutex::new((0, initial_snapshot)),
        }
    }

    /// Returns the default endpoint.
    pub fn default_endpoint(&self) -> &CosmosEndpoint {
        &self.default_endpoint
    }

    /// Returns a snapshot of account and partition state.
    ///
    /// Uses a fast path when the version hasn't changed since the last
    /// snapshot: the cached `Arc` is cloned without touching the
    /// epoch-protected pointers. On a version mismatch (slow path),
    /// the current state is loaded under an epoch guard, cloned into
    /// fresh `Arc`s, and cached for subsequent callers.
    pub fn snapshot(&self) -> LocationSnapshot {
        let current_version = self.account_version.load(Ordering::Acquire);

        {
            let cached = self.cached_snapshot.lock().unwrap();
            if cached.0 == current_version {
                return cached.1.clone();
            }
        }

        let guard = epoch::pin();

        let account = {
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current = unsafe { self.account.load(Ordering::Acquire, &guard).deref() };
            Arc::new(current.clone())
        };

        let partitions = {
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current = unsafe { self.partitions.load(Ordering::Acquire, &guard).deref() };
            Arc::new(current.clone())
        };

        let snapshot = LocationSnapshot {
            account,
            partitions,
        };

        let mut cached = self.cached_snapshot.lock().unwrap();
        if cached.0 < current_version {
            *cached = (current_version, snapshot.clone());
        }

        snapshot
    }

    /// Returns the configured endpoint unavailability TTL.
    pub fn endpoint_unavailability_ttl(&self) -> Duration {
        self.endpoint_unavailability_ttl
    }

    /// Returns the latest account snapshot.
    #[allow(dead_code)]
    pub fn account_snapshot(&self) -> Arc<AccountEndpointState> {
        let guard = epoch::pin();
        // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
        let current = unsafe { self.account.load(Ordering::Acquire, &guard).deref() };
        Arc::new(current.clone())
    }

    /// Applies location effects (endpoint unavailability and account refresh).
    pub async fn apply(&self, effects: &[LocationEffect]) {
        for effect in effects {
            match effect {
                LocationEffect::MarkEndpointUnavailable { endpoint, reason } => {
                    let endpoint = endpoint.clone();
                    let reason = reason.clone();
                    self.apply_account(|current| {
                        mark_endpoint_unavailable(current, &endpoint, reason.clone())
                    });
                }
                LocationEffect::MarkPartitionUnavailable(_) => {
                    // TODO(partition-routing): Apply partition-level unavailability.
                }
                LocationEffect::RefreshAccountProperties => {
                    self.refresh_account_properties_if_due().await;
                }
            }
        }
    }

    fn apply_account(&self, mut f: impl FnMut(&AccountEndpointState) -> AccountEndpointState) {
        let guard = epoch::pin();

        loop {
            let current = self.account.load(Ordering::Acquire, &guard);
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current_ref = unsafe { current.deref() };
            let next_state = f(current_ref);

            match self.account.compare_exchange(
                current,
                Owned::new(next_state),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(_) => {
                    // `current` is the old value that was just replaced.
                    unsafe { guard.defer_destroy(current) };
                    self.account_version.fetch_add(1, Ordering::Release);
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    async fn refresh_account_properties_if_due(&self) {
        let now_ms = epoch_millis();
        let refresh_after_ms = self.refresh_interval.as_millis() as u64;
        let last = self.last_refresh_epoch_ms.load(Ordering::Acquire);

        if now_ms.saturating_sub(last) < refresh_after_ms {
            return;
        }

        if self
            .last_refresh_epoch_ms
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        // Capture the previous properties before invalidation so the refresh
        // callback can use them for regional fallback if the primary fails.
        let previous_props = self
            .account_metadata_cache
            .invalidate(&self.account_endpoint)
            .await;

        let refresh_fn = Arc::clone(&self.account_refresh_fn);
        let refreshed = self
            .account_metadata_cache
            .get_or_fetch(self.account_endpoint.clone(), || async move {
                (refresh_fn)(previous_props).await
            })
            .await;

        let Ok(properties) = refreshed else {
            return;
        };

        let default_endpoint = self.default_endpoint.clone();
        self.sync_account_properties(properties, &default_endpoint);
    }

    /// Updates account state from properties using a CAS loop that preserves
    /// existing `unavailable_endpoints` marks set by concurrent operations.
    ///
    /// Skips the CAS loop when the `AccountProperties` etag matches
    /// the last synced value (same server version, properties unchanged).
    pub fn sync_account_properties(
        &self,
        properties: Arc<AccountProperties>,
        default_endpoint: &CosmosEndpoint,
    ) {
        // Fast path: same Arc pointer means identical data — nothing changed.
        {
            let last = self.last_synced_properties.lock().unwrap();
            if last.as_ref().is_some_and(|p| Arc::ptr_eq(p, &properties)) {
                return;
            }
        }

        self.prune_expired_unavailable_endpoints();

        if !properties.etag.is_empty() {
            let last_etag = self.last_synced_etag.lock().unwrap();
            if *last_etag == properties.etag {
                // Etag matches: update the pointer so future calls hit the fast path.
                drop(last_etag);
                *self.last_synced_properties.lock().unwrap() = Some(properties);
                return;
            }
        }

        let default_endpoint = default_endpoint.clone();
        let ttl = self.endpoint_unavailability_ttl;
        self.apply_account(|current| {
            let mut next = build_account_endpoint_state(
                &properties,
                default_endpoint.clone(),
                Some(current.generation),
                self.gateway20_enabled,
                &self.preferred_regions,
            );
            // Carry forward unavailability marks from the current state,
            // filtering out entries that have expired past the configured TTL.
            let now = Instant::now();
            let mut unavailable = current.unavailable_endpoints.clone();
            unavailable.retain(|_, (marked_at, _)| now.saturating_duration_since(*marked_at) < ttl);
            next.unavailable_endpoints = unavailable;
            next
        });

        if !properties.etag.is_empty() {
            let mut last_etag = self.last_synced_etag.lock().unwrap();
            *last_etag = properties.etag.clone();
        }
        *self.last_synced_properties.lock().unwrap() = Some(properties);
    }

    fn prune_expired_unavailable_endpoints(&self) {
        let now = Instant::now();
        let ttl = self.endpoint_unavailability_ttl;
        let snapshot = self.account_snapshot();

        let has_expired = snapshot
            .unavailable_endpoints
            .values()
            .any(|(marked_at, _)| now.saturating_duration_since(*marked_at) >= ttl);

        if !has_expired {
            return;
        }

        self.apply_account(|current| expire_unavailable_endpoints(current, now, ttl));
    }
}

fn epoch_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::{
        driver::routing::{CosmosEndpoint, LocationEffect, UnavailableReason},
        models::AccountEndpoint,
    };

    use super::*;

    fn test_endpoint() -> AccountEndpoint {
        AccountEndpoint::from(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    fn test_refresh_payload() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-1",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
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
    async fn apply_marks_endpoint_unavailable() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            false,
            Duration::from_secs(60),
            Vec::new(),
        );

        store
            .apply(&[LocationEffect::MarkEndpointUnavailable {
                endpoint: default_endpoint.clone(),
                reason: UnavailableReason::TransportError,
            }])
            .await;

        let snapshot = store.snapshot();
        assert_eq!(snapshot.account.unavailable_endpoints.len(), 1);
    }

    #[tokio::test]
    async fn refresh_effect_is_rate_limited() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh_calls = Arc::new(AtomicUsize::new(0));
        let refresh_calls_clone = Arc::clone(&refresh_calls);
        let refresh = Arc::new(move |_previous: Option<Arc<AccountProperties>>| {
            let refresh_calls = Arc::clone(&refresh_calls_clone);
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move {
                    refresh_calls.fetch_add(1, Ordering::SeqCst);
                    Ok(payload)
                });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            Vec::new(),
        );

        store
            .apply(&[LocationEffect::RefreshAccountProperties])
            .await;
        store
            .apply(&[LocationEffect::RefreshAccountProperties])
            .await;

        // The second call should be throttled by refresh_interval.
        assert_eq!(refresh_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn sync_account_properties_prunes_expired_marks_even_when_etag_is_unchanged() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            false,
            Duration::from_secs(60),
            Vec::new(),
        );

        let properties = Arc::new(test_refresh_payload());
        store.sync_account_properties(Arc::clone(&properties), &default_endpoint);

        let expired_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        store.apply_account(|current| {
            let mut next = current.clone();
            next.unavailable_endpoints.insert(
                expired_endpoint.url().clone(),
                (
                    Instant::now() - Duration::from_secs(120),
                    UnavailableReason::TransportError,
                ),
            );
            next
        });

        // Use a different Arc to force a re-sync (same data, different pointer).
        let properties2 = Arc::new(test_refresh_payload());
        store.sync_account_properties(properties2, &default_endpoint);

        let snapshot = store.snapshot();
        assert!(snapshot.account.unavailable_endpoints.is_empty());
    }
}
