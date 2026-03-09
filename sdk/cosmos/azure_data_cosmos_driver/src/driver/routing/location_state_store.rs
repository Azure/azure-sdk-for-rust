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
};

use super::{
    build_account_endpoint_state, mark_endpoint_unavailable, AccountEndpointState, CosmosEndpoint,
    LocationEffect,
};

/// Step 3 adds partition-level state; Step 2 keeps this as a placeholder.
#[derive(Clone, Debug, Default)]
pub(crate) struct PartitionEndpointState;

/// Immutable location snapshot consumed by one operation-loop iteration.
#[derive(Clone, Debug)]
pub(crate) struct LocationSnapshot {
    pub account: Arc<AccountEndpointState>,
    #[allow(dead_code)]
    pub partitions: Arc<PartitionEndpointState>,
}

type AccountRefreshFn =
    Arc<dyn Fn() -> BoxFuture<'static, azure_core::Result<AccountProperties>> + Send + Sync>;

/// Unified location state store with lock-free reads and CAS-loop writes.
pub(crate) struct LocationStateStore {
    account: Atomic<AccountEndpointState>,
    partitions: Atomic<PartitionEndpointState>,
    account_metadata_cache: Arc<AccountMetadataCache>,
    account_endpoint: AccountEndpoint,
    account_refresh_fn: AccountRefreshFn,
    default_endpoint: CosmosEndpoint,
    endpoint_unavailability_ttl: Duration,
    refresh_interval: Duration,
    last_refresh_epoch_ms: AtomicU64,
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

impl LocationStateStore {
    /// Creates a new location store with a single-endpoint account snapshot.
    pub fn new(
        account_metadata_cache: Arc<AccountMetadataCache>,
        account_endpoint: AccountEndpoint,
        default_endpoint: CosmosEndpoint,
        account_refresh_fn: AccountRefreshFn,
        endpoint_unavailability_ttl: Duration,
    ) -> Self {
        let account_state = AccountEndpointState::single(default_endpoint.clone());

        Self {
            account: Atomic::new(account_state),
            partitions: Atomic::new(PartitionEndpointState),
            account_metadata_cache,
            account_endpoint,
            account_refresh_fn,
            default_endpoint,
            endpoint_unavailability_ttl,
            // Fixed refresh throttling for Step 2.
            refresh_interval: Duration::from_secs(5),
            last_refresh_epoch_ms: AtomicU64::new(0),
        }
    }

    /// Returns the default endpoint.
    pub fn default_endpoint(&self) -> &CosmosEndpoint {
        &self.default_endpoint
    }

    /// Returns a lock-free snapshot of account and partition state.
    pub fn snapshot(&self) -> LocationSnapshot {
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

        LocationSnapshot {
            account,
            partitions,
        }
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

    /// Applies location effects. Step 2 applies endpoint unavailability and account refresh.
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
                    // Step 2 keeps partition-level logic as a no-op bridge for Step 3.
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
                Ok(old) => {
                    // SAFETY: old pointer is detached after successful exchange.
                    unsafe { guard.defer_destroy(old) };
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

        let _ = self
            .account_metadata_cache
            .invalidate(&self.account_endpoint)
            .await;

        let refresh_fn = Arc::clone(&self.account_refresh_fn);
        let refreshed = self
            .account_metadata_cache
            .get_or_fetch(self.account_endpoint.clone(), || async move {
                (refresh_fn)().await
            })
            .await;

        let Ok(properties) = refreshed else {
            return;
        };

        let default_endpoint = self.default_endpoint.clone();
        self.sync_account_properties(properties.as_ref(), &default_endpoint);
    }

    /// Updates account state from properties using a CAS loop that preserves
    /// existing `unavailable_endpoints` marks set by concurrent operations.
    pub fn sync_account_properties(
        &self,
        properties: &AccountProperties,
        default_endpoint: &CosmosEndpoint,
    ) {
        let default_endpoint = default_endpoint.clone();
        let ttl = self.endpoint_unavailability_ttl;
        self.apply_account(|current| {
            let mut next = build_account_endpoint_state(
                properties,
                default_endpoint.clone(),
                Some(current.generation),
            );
            // Carry forward unavailability marks from the current state,
            // filtering out entries that have expired past the configured TTL.
            let now = Instant::now();
            let mut unavailable = current.unavailable_endpoints.clone();
            unavailable.retain(|_, (marked_at, _)| now.saturating_duration_since(*marked_at) < ttl);
            next.unavailable_endpoints = unavailable;
            next
        });
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
        let refresh = Arc::new(|| {
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
            Duration::from_secs(60),
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
        let refresh = Arc::new(move || {
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
            Duration::from_secs(60),
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
}
