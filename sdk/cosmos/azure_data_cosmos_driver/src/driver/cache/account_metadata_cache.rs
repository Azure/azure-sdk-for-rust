// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account metadata cache for Cosmos DB driver.
//!
//! [`AccountProperties`] mirrors the full JSON contract returned by the Cosmos DB
//! account read endpoint. Fields that are not yet consumed by driver logic are
//! kept intentionally to match the service response shape and to ease future
//! feature work.

use super::AsyncCache;
use crate::models::{AccountEndpoint, DefaultConsistencyLevel};
use crate::options::Region;
use futures::lock::Mutex as AsyncMutex;
use serde::Deserialize;
use std::{
    collections::{BTreeSet, HashMap},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

#[derive(Debug)]
struct RefreshGate {
    lock: AsyncMutex<()>,
    requests: Mutex<RefreshRequests>,
    installed_sequence: AtomicU64,
}

#[derive(Debug, Default)]
struct RefreshRequests {
    next_sequence: u64,
    pending: BTreeSet<u64>,
}

struct RefreshReservation {
    gate: Arc<RefreshGate>,
    sequence: u64,
}

impl Drop for RefreshReservation {
    fn drop(&mut self) {
        self.gate
            .requests
            .lock()
            .unwrap()
            .pending
            .remove(&self.sequence);
    }
}

// =============================================================================
// Supporting types for the account JSON contract
// =============================================================================

/// Represents a single regional endpoint for the Cosmos DB account (readable or writable).
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct AccountRegion {
    pub name: Region,

    pub database_account_endpoint: AccountEndpoint,
}

/// Describes replica set sizing characteristics for user/system replication policies.
///
/// The service may omit this object entirely (e.g. the vnext emulator does not
/// send `userReplicationPolicy`) or omit individual fields. To match the
/// defensive behavior of the .NET and Java SDKs, missing values fall back to the
/// standard defaults (`min = 3`, `max = 4`).
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
// cSpell:disable
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ReplicationPolicy {
    #[serde(default = "ReplicationPolicy::default_min_replica_set_size")]
    pub min_replica_set_size: i32,

    // Note: service returns key `maxReplicasetSize` (lowercase 's' in 'set')
    #[serde(
        rename = "maxReplicasetSize",
        default = "ReplicationPolicy::default_max_replica_set_size"
    )]
    pub max_replica_set_size: i32,
}

impl ReplicationPolicy {
    /// Default minimum replica set size used when the service omits the value.
    fn default_min_replica_set_size() -> i32 {
        3
    }

    /// Default maximum replica set size used when the service omits the value.
    fn default_max_replica_set_size() -> i32 {
        4
    }
}

impl Default for ReplicationPolicy {
    fn default() -> Self {
        Self {
            min_replica_set_size: Self::default_min_replica_set_size(),
            max_replica_set_size: Self::default_max_replica_set_size(),
        }
    }
}

/// User-configured default consistency level for the account.
///
/// Defaults to [`DefaultConsistencyLevel::Session`] when the service omits the
/// policy, matching the account creation default.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ConsistencyPolicy {
    #[serde(default = "ConsistencyPolicy::default_consistency_level")]
    pub default_consistency_level: DefaultConsistencyLevel,
}

impl ConsistencyPolicy {
    /// Default consistency level used when the service omits the value.
    fn default_consistency_level() -> DefaultConsistencyLevel {
        DefaultConsistencyLevel::Session
    }
}

impl Default for ConsistencyPolicy {
    fn default() -> Self {
        Self {
            default_consistency_level: Self::default_consistency_level(),
        }
    }
}

/// Read preference coefficients used by the service when selecting regions.
///
/// Defaults to zeroed coefficients when the service omits the policy.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ReadPolicy {
    #[serde(default)]
    pub primary_read_coefficient: i32,

    #[serde(default)]
    pub secondary_read_coefficient: i32,
}

// =============================================================================
// AccountProperties – full JSON contract
// =============================================================================

/// Top-level Cosmos DB DatabaseAccount properties returned by the account read endpoint.
///
/// This struct mirrors the full JSON contract from the service. Fields that are
/// not yet consumed by driver logic are kept intentionally so that the struct can
/// round-trip with `serde` and so that new features can use them without a
/// contract change.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct AccountProperties {
    #[serde(rename = "_self")]
    pub self_link: String,

    /// The id of the respective account.
    pub id: String,

    /// The resource id of the respective account.
    #[serde(rename = "_rid")]
    pub rid: String,

    /// The media type of the respective account.
    #[serde(default)]
    pub media: String,

    /// Root relative path for the addresses endpoint.
    #[serde(default)]
    pub addresses: String,

    /// Root relative path for the databases feed.
    #[serde(rename = "_dbs", default)]
    pub dbs: String,

    /// Regions currently accepting writes for the account.
    #[serde(default)]
    pub writable_locations: Vec<AccountRegion>,

    /// Regions from which the account can be read.
    #[serde(default)]
    pub readable_locations: Vec<AccountRegion>,

    /// True when multi-master writes are enabled.
    #[serde(default)]
    pub enable_multiple_write_locations: bool,

    /// Indicates if continuous backup (PITR) is enabled.
    #[serde(default)]
    pub continuous_backup_enabled: bool,

    /// Enables synchronous commit across N regions.
    #[serde(default)]
    pub enable_n_region_synchronous_commit: bool,

    /// Allows failover at per-partition granularity.
    #[serde(default)]
    pub enable_per_partition_failover_behavior: bool,

    /// User replication settings (min/max replica set sizes).
    #[serde(default)]
    pub user_replication_policy: ReplicationPolicy,

    /// Default consistency level configured by the user.
    #[serde(default)]
    pub user_consistency_policy: ConsistencyPolicy,

    /// System-managed replication sizing policy.
    #[serde(default)]
    pub system_replication_policy: ReplicationPolicy,

    /// Coefficients guiding regional read preference selection.
    #[serde(default)]
    pub read_policy: ReadPolicy,

    /// Raw JSON string containing query engine feature/configuration flags.
    #[serde(default)]
    pub query_engine_configuration: String,

    /// Regional Gateway 2.0 endpoints accepting writes.
    /// When present, indicates that Gateway 2.0 should be used for the
    /// dataplane transport instead of the standard gateway endpoint.
    #[serde(default)]
    pub thin_client_writable_locations: Vec<AccountRegion>,

    /// Regional Gateway 2.0 endpoints for reads.
    /// When present, indicates that Gateway 2.0 should be used for the
    /// dataplane transport instead of the standard gateway endpoint.
    #[serde(default)]
    pub thin_client_readable_locations: Vec<AccountRegion>,

    /// Server-assigned version tag. Changes when the account metadata is updated.
    #[serde(rename = "_etag", default)]
    pub etag: String,
}

// Convenience accessors for the account properties JSON contract. Some may not
// yet be used by driver logic but are kept intentionally for future use.
#[allow(dead_code)]
impl AccountProperties {
    /// Returns the first writable [`AccountRegion`], if any.
    pub(crate) fn write_account_region(&self) -> Option<&AccountRegion> {
        self.writable_locations.first()
    }

    /// Returns the first write region, if any.
    pub(crate) fn write_region(&self) -> Option<Region> {
        self.writable_locations.first().map(|loc| loc.name.clone())
    }

    /// Returns readable regions derived from the account metadata.
    pub(crate) fn readable_regions(&self) -> Vec<Region> {
        self.readable_locations
            .iter()
            .map(|loc| loc.name.clone())
            .collect()
    }

    /// Returns `true` if Gateway 2.0 endpoints are available.
    ///
    /// When Gateway 2.0 locations are present in the account properties,
    /// the driver should use Gateway 2.0 for the dataplane transport.
    pub(crate) fn has_gateway_v2_endpoints(&self) -> bool {
        !self.thin_client_writable_locations.is_empty()
            || !self.thin_client_readable_locations.is_empty()
    }

    /// Returns Gateway 2.0 writable locations, if any.
    pub(crate) fn gateway_v2_writable_regions(&self) -> Vec<Region> {
        self.thin_client_writable_locations
            .iter()
            .map(|loc| loc.name.clone())
            .collect()
    }

    /// Returns Gateway 2.0 readable locations, if any.
    pub(crate) fn gateway_v2_readable_regions(&self) -> Vec<Region> {
        self.thin_client_readable_locations
            .iter()
            .map(|loc| loc.name.clone())
            .collect()
    }
}

/// Cache for Cosmos DB account metadata.
///
/// Stores account properties keyed by account endpoint. Freshness is owned
/// by the periodic background loop in
/// [`LocationStateStore::start_account_refresh_loop`](crate::driver::routing::LocationStateStore::start_account_refresh_loop),
/// which atomically replaces cache entries via [`Self::refresh_with`].
/// The per-operation hot path uses [`Self::get_or_fetch`] for a cheap
/// fast-path lookup with no staleness check or extra locking.
#[derive(Debug)]
pub(crate) struct AccountMetadataCache {
    cache: AsyncCache<AccountEndpoint, AccountProperties>,
    refresh_gates: Mutex<HashMap<AccountEndpoint, Arc<RefreshGate>>>,
}

impl AccountMetadataCache {
    /// Creates a new empty account metadata cache.
    pub(crate) fn new() -> Self {
        Self {
            cache: AsyncCache::new(),
            refresh_gates: Mutex::new(HashMap::new()),
        }
    }

    /// Gets account properties from cache, or fetches and caches them.
    ///
    /// If the fetch fails, the error is propagated and nothing is cached,
    /// so the next call will try fetching again.
    ///
    /// **Does NOT honor any staleness threshold** — once a value is cached
    /// it is returned forever. Periodic re-fetch is handled by
    /// [`LocationStateStore::start_account_refresh_loop`](crate::driver::routing::LocationStateStore::start_account_refresh_loop)
    /// which calls [`Self::get_or_refresh_with`] to atomically replace the
    /// entry on a timer.
    pub(crate) async fn get_or_fetch<F, Fut>(
        &self,
        endpoint: AccountEndpoint,
        fetch_fn: F,
    ) -> crate::error::Result<Arc<AccountProperties>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<AccountProperties>>,
    {
        // Fast path: return cached value.
        if let Some(cached) = self.cache.get(&endpoint).await {
            return Ok(cached);
        }

        // Fetch from the service – propagate errors without caching them.
        let properties = fetch_fn().await?;

        // Cache the successfully fetched properties.
        let result = self
            .cache
            .get_or_insert_with(endpoint, || async { properties })
            .await;

        Ok(result)
    }

    /// Returns the currently cached account properties for an endpoint, if
    /// any. Does NOT trigger a fetch — callers that want to populate on miss
    /// should use [`Self::get_or_fetch`] instead.
    pub(crate) async fn get(&self, endpoint: &AccountEndpoint) -> Option<Arc<AccountProperties>> {
        self.cache.get(endpoint).await
    }

    /// Serializes refresh fetch/install for one account endpoint across every
    /// driver sharing this cache. Different account endpoints remain
    /// independent, and operation hot paths continue reading the old value
    /// while a refresh is in progress.
    pub(crate) async fn refresh_with<F, Fut, C, CFut, K>(
        &self,
        endpoint: AccountEndpoint,
        refresh_fn: F,
        consume_fn: C,
        commit_fn: K,
    ) -> crate::error::Result<()>
    where
        F: FnOnce(Option<Arc<AccountProperties>>) -> Fut,
        Fut: std::future::Future<Output = crate::error::Result<AccountProperties>>,
        C: FnOnce(Arc<AccountProperties>) -> CFut,
        CFut: std::future::Future<Output = ()>,
        K: FnOnce(bool),
    {
        let gate = {
            let mut gates = self.refresh_gates.lock().unwrap();
            Arc::clone(gates.entry(endpoint.clone()).or_insert_with(|| {
                Arc::new(RefreshGate {
                    lock: AsyncMutex::new(()),
                    requests: Mutex::new(RefreshRequests::default()),
                    installed_sequence: AtomicU64::new(0),
                })
            }))
        };
        let sequence = {
            let mut requests = gate.requests.lock().unwrap();
            requests.next_sequence += 1;
            let sequence = requests.next_sequence;
            requests.pending.insert(sequence);
            sequence
        };
        let _reservation = RefreshReservation {
            gate: Arc::clone(&gate),
            sequence,
        };
        let _guard = gate.lock.lock().await;
        if sequence < gate.installed_sequence.load(Ordering::Acquire) {
            let properties = self
                .cache
                .get(&endpoint)
                .await
                .expect("a newer refresh sequence must have installed a value");
            consume_fn(properties).await;
            commit_fn(false);
            return Ok(());
        }
        let previous = self.cache.get(&endpoint).await;
        let properties = refresh_fn(previous).await?;
        let properties = if sequence < gate.installed_sequence.load(Ordering::Acquire) {
            self.cache
                .get(&endpoint)
                .await
                .expect("a newer refresh sequence must have installed a value")
        } else {
            let properties = self
                .cache
                .get_or_refresh_with(endpoint, |_| true, || async { properties })
                .await
                .expect("refresh factory always produces account properties");
            gate.installed_sequence.store(sequence, Ordering::Release);
            properties
        };
        consume_fn(properties).await;
        // Prevent a newer live request from being registered between this
        // check and the caller's local timestamp commit.
        let requests = gate.requests.lock().unwrap();
        let latest = gate.installed_sequence.load(Ordering::Acquire) == sequence
            && !requests.pending.iter().any(|pending| *pending > sequence);
        commit_fn(latest);
        Ok(())
    }
}

impl Default for AccountMetadataCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn test_endpoint(name: &str) -> AccountEndpoint {
        AccountEndpoint::from(
            url::Url::parse(&format!("https://{name}.documents.azure.com:443/")).unwrap(),
        )
    }

    /// Builds a minimal [`AccountProperties`] from JSON with the given region
    /// used for both the writable and readable location.
    fn test_properties(region_name: &str) -> AccountProperties {
        let endpoint = format!("https://test-{region_name}.documents.azure.com:443/");
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": region_name, "databaseAccountEndpoint": endpoint }],
            "readableLocations": [{ "name": region_name, "databaseAccountEndpoint": endpoint }],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .expect("test JSON is valid")
    }

    #[tokio::test]
    async fn caches_account_properties() {
        let cache = AccountMetadataCache::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let endpoint = test_endpoint("myaccount");

        let counter_clone = counter.clone();
        let props = cache
            .get_or_fetch(endpoint.clone(), || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(test_properties("westus"))
            })
            .await
            .unwrap();

        assert_eq!(props.write_region().unwrap().as_str(), "westus");
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Second access uses cached value
        let counter_clone = counter.clone();
        let props2 = cache
            .get_or_fetch(endpoint, || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(test_properties("eastus"))
            })
            .await
            .unwrap();

        assert_eq!(props2.write_region().unwrap().as_str(), "westus");
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn different_accounts_cached_separately() {
        let cache = AccountMetadataCache::new();

        let props1 = cache
            .get_or_fetch(test_endpoint("account1"), || async {
                Ok(test_properties("westus"))
            })
            .await
            .unwrap();

        let props2 = cache
            .get_or_fetch(test_endpoint("account2"), || async {
                Ok(test_properties("eastus"))
            })
            .await
            .unwrap();

        assert_eq!(props1.write_region().unwrap().as_str(), "westus");
        assert_eq!(props2.write_region().unwrap().as_str(), "eastus");
    }

    #[tokio::test]
    async fn cancelled_newer_waiter_does_not_suppress_active_refresh() {
        let cache = Arc::new(AccountMetadataCache::new());
        let endpoint = test_endpoint("cancelled-waiter");
        let started = Arc::new(tokio::sync::Notify::new());
        let release = Arc::new(tokio::sync::Notify::new());
        let active_committed = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let active = {
            let cache = Arc::clone(&cache);
            let endpoint = endpoint.clone();
            let started = Arc::clone(&started);
            let release = Arc::clone(&release);
            let committed = Arc::clone(&active_committed);
            tokio::spawn(async move {
                cache
                    .refresh_with(
                        endpoint,
                        move |_| async move {
                            started.notify_one();
                            release.notified().await;
                            Ok(test_properties("westus"))
                        },
                        |_| async {},
                        move |latest| committed.store(latest, Ordering::SeqCst),
                    )
                    .await
            })
        };
        started.notified().await;

        let waiter = {
            let cache = Arc::clone(&cache);
            let endpoint = endpoint.clone();
            tokio::spawn(async move {
                cache
                    .refresh_with(
                        endpoint,
                        |_| async { Ok(test_properties("eastus")) },
                        |_| async {},
                        |_| {},
                    )
                    .await
            })
        };
        tokio::task::yield_now().await;
        let gate = Arc::clone(cache.refresh_gates.lock().unwrap().get(&endpoint).unwrap());
        assert_eq!(gate.requests.lock().unwrap().next_sequence, 2);

        waiter.abort();
        let _ = waiter.await;

        release.notify_one();
        active.await.unwrap().unwrap();
        assert!(active_committed.load(Ordering::SeqCst));
        assert_eq!(
            cache
                .get(&endpoint)
                .await
                .unwrap()
                .write_region()
                .unwrap()
                .as_str(),
            "westus"
        );
    }

    #[tokio::test]
    async fn superseded_sequence_uses_cache_without_fallible_fetch() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint("superseded");
        cache
            .refresh_with(
                endpoint.clone(),
                |_| async { Ok(test_properties("westus")) },
                |_| async {},
                |_| {},
            )
            .await
            .unwrap();
        let gate = Arc::clone(cache.refresh_gates.lock().unwrap().get(&endpoint).unwrap());
        // Simulate an older waiter acquiring the non-FIFO mutex after sequence
        // 3 already installed. Its next allocated sequence is 2.
        gate.requests.lock().unwrap().next_sequence = 1;
        gate.installed_sequence.store(3, Ordering::Release);
        let fetches = Arc::new(AtomicUsize::new(0));
        let consumed = Arc::new(AtomicUsize::new(0));
        let committed = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let fetches_clone = Arc::clone(&fetches);
        let consumed_clone = Arc::clone(&consumed);
        let committed_clone = Arc::clone(&committed);

        cache
            .refresh_with(
                endpoint,
                move |_| async move {
                    fetches_clone.fetch_add(1, Ordering::SeqCst);
                    Err(crate::error::CosmosError::builder()
                        .with_message("superseded fetch must not run")
                        .build())
                },
                move |properties| async move {
                    assert_eq!(properties.write_region().unwrap().as_str(), "westus");
                    consumed_clone.fetch_add(1, Ordering::SeqCst);
                },
                move |latest| committed_clone.store(latest, Ordering::SeqCst),
            )
            .await
            .unwrap();

        assert!(!committed.load(Ordering::SeqCst));
        assert_eq!(fetches.load(Ordering::SeqCst), 0);
        assert_eq!(consumed.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn get_returns_none_before_fetch() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint("myaccount");

        assert!(cache.cache.get(&endpoint).await.is_none());
    }

    #[tokio::test]
    async fn invalidate_removes_entry() {
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint("myaccount");

        cache
            .get_or_fetch(endpoint.clone(), || async { Ok(test_properties("westus")) })
            .await
            .unwrap();

        let removed = cache.cache.invalidate(&endpoint).await;
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().write_region().unwrap().as_str(), "westus");
        assert!(cache.cache.get(&endpoint).await.is_none());
    }

    #[tokio::test]
    async fn clear_removes_all() {
        let cache = AccountMetadataCache::new();

        cache
            .get_or_fetch(test_endpoint("account1"), || async {
                Ok(test_properties("westus"))
            })
            .await
            .unwrap();
        cache
            .get_or_fetch(test_endpoint("account2"), || async {
                Ok(test_properties("eastus"))
            })
            .await
            .unwrap();

        cache.cache.clear().await;

        assert!(cache.cache.get(&test_endpoint("account1")).await.is_none());
        assert!(cache.cache.get(&test_endpoint("account2")).await.is_none());
    }

    #[test]
    fn deserialize_full_account_payload() {
        let json = r#"{
            "_self": "",
            "id": "testaccount",
            "_rid": "testaccount.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [
                { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }
            ],
            "readableLocations": [
                { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
                { "name": "East US 2", "databaseAccountEndpoint": "https://test-eastus2.documents.azure.com:443/" }
            ],
            "enableMultipleWriteLocations": false,
            "continuousBackupEnabled": false,
            "enableNRegionSynchronousCommit": false,
            "enablePerPartitionFailoverBehavior": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{\"allowNewKeywords\":true}"
        }"#;

        let props: AccountProperties = serde_json::from_str(json).expect("deserialize");
        assert_eq!(props.id, "testaccount");
        // Region normalizes "West US 2" -> "westus2"
        assert_eq!(props.write_region().unwrap().as_str(), "westus2");
        assert_eq!(props.readable_regions().len(), 2);
        assert_eq!(props.writable_locations.len(), 1);
        assert_eq!(props.readable_locations.len(), 2);
        assert_eq!(props.user_replication_policy.min_replica_set_size, 3);
        assert_eq!(
            props.user_consistency_policy.default_consistency_level,
            DefaultConsistencyLevel::Session
        );
        assert!(!props.enable_multiple_write_locations);
    }

    #[test]
    fn write_region_is_none_when_empty() {
        let props: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "",
            "_rid": "",
            "media": "",
            "addresses": "",
            "_dbs": "",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 0, "maxReplicasetSize": 0 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 0, "maxReplicasetSize": 0 },
            "readPolicy": { "primaryReadCoefficient": 0, "secondaryReadCoefficient": 0 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap();

        assert!(props.write_region().is_none());
        assert!(props.readable_regions().is_empty());
    }

    #[test]
    fn replication_policy_defaults_when_missing() {
        // The vnext emulator omits `userReplicationPolicy` (and may omit other
        // policy objects). Deserialization must succeed and fall back to the
        // standard defaults (min = 3, max = 4), matching the .NET and Java SDKs.
        let json = r#"{
            "_self": "",
            "id": "emulator",
            "_rid": "emulator.documents.azure.com",
            "writableLocations": [
                { "name": "South Central US", "databaseAccountEndpoint": "https://localhost:8081/" }
            ],
            "readableLocations": [
                { "name": "South Central US", "databaseAccountEndpoint": "https://localhost:8081/" }
            ]
        }"#;

        let props: AccountProperties = serde_json::from_str(json).expect("deserialize");

        // Missing replication policies fall back to the standard defaults.
        assert_eq!(props.user_replication_policy, ReplicationPolicy::default());
        assert_eq!(props.user_replication_policy.min_replica_set_size, 3);
        assert_eq!(props.user_replication_policy.max_replica_set_size, 4);
        assert_eq!(
            props.system_replication_policy,
            ReplicationPolicy::default()
        );

        // Other omitted fields fall back to their defaults.
        assert_eq!(
            props.user_consistency_policy.default_consistency_level,
            DefaultConsistencyLevel::Session
        );
        assert_eq!(props.read_policy, ReadPolicy::default());
        assert!(props.media.is_empty());
        assert!(props.query_engine_configuration.is_empty());
        assert!(!props.enable_multiple_write_locations);

        // Present fields are still honored.
        assert_eq!(props.write_region().unwrap().as_str(), "southcentralus");
    }

    #[test]
    fn replication_policy_defaults_individual_fields() {
        // A partial replication policy object should default the missing field
        // rather than failing to deserialize.
        let policy: ReplicationPolicy =
            serde_json::from_str(r#"{ "minReplicaSetSize": 1 }"#).expect("deserialize");
        assert_eq!(policy.min_replica_set_size, 1);
        assert_eq!(policy.max_replica_set_size, 4);

        let policy: ReplicationPolicy = serde_json::from_str("{}").expect("deserialize");
        assert_eq!(policy, ReplicationPolicy::default());
    }

    #[test]
    fn error_envelope_service_unavailable_does_not_parse_as_account_properties() {
        // Shape returned by the gateway for HTTP 503.
        let body = r#"{"code":"ServiceUnavailable","message":"Service is currently unavailable."}"#;
        let err = serde_json::from_str::<AccountProperties>(body)
            .expect_err("503 error envelope must not deserialize as AccountProperties");
        // Confirms the pre-fix surface: serde fails on missing AccountProperties fields.
        assert!(
            err.to_string().contains("_self") || err.to_string().contains("missing field"),
            "expected serde to fail on the missing AccountProperties fields, got: {err}"
        );
    }

    #[test]
    fn error_envelope_unauthorized_does_not_parse_as_account_properties() {
        // Shape returned for HTTP 401 (AAD InvalidToken / TokenExpired / RBAC propagation race).
        let body = r#"{"code":"Unauthorized","message":"The input authorization token can't serve the request."}"#;
        serde_json::from_str::<AccountProperties>(body)
            .expect_err("401 error envelope must not deserialize as AccountProperties");
    }

    #[test]
    fn error_envelope_forbidden_does_not_parse_as_account_properties() {
        // Shape returned for HTTP 403 (data-plane RBAC, WriteForbidden, firewall block, etc.).
        let body = r#"{"code":"Forbidden","message":"Request is blocked by your Cosmos DB account firewall settings."}"#;
        serde_json::from_str::<AccountProperties>(body)
            .expect_err("403 error envelope must not deserialize as AccountProperties");
    }

    #[test]
    fn error_envelope_too_many_requests_does_not_parse_as_account_properties() {
        // Shape returned by the gateway for HTTP 429.
        let body = r#"{"code":"TooManyRequests","message":"Request rate is large."}"#;
        serde_json::from_str::<AccountProperties>(body)
            .expect_err("429 error envelope must not deserialize as AccountProperties");
    }

    #[test]
    fn plain_text_error_body_does_not_parse_as_account_properties() {
        // Some proxies / LBs / fault injectors emit plain-text non-JSON bodies on non-2xx.
        let body = "Service Unavailable - Injected fault";
        serde_json::from_str::<AccountProperties>(body)
            .expect_err("plain-text body must not deserialize as AccountProperties");
    }
}
