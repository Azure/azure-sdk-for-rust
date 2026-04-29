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
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
// cSpell:disable
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ReplicationPolicy {
    pub min_replica_set_size: i32,

    // Note: service returns key `maxReplicasetSize` (lowercase 's' in 'set')
    #[serde(rename = "maxReplicasetSize")]
    pub max_replica_set_size: i32,
}

/// User-configured default consistency level for the account.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ConsistencyPolicy {
    pub default_consistency_level: DefaultConsistencyLevel,
}

/// Read preference coefficients used by the service when selecting regions.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
// All fields reflect the JSON contract of the account properties response and
// are kept intentionally even when not yet consumed by driver logic.
#[allow(dead_code)]
pub(crate) struct ReadPolicy {
    pub primary_read_coefficient: i32,

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
    pub media: String,

    /// Root relative path for the addresses endpoint.
    pub addresses: String,

    /// Root relative path for the databases feed.
    #[serde(rename = "_dbs")]
    pub dbs: String,

    /// Regions currently accepting writes for the account.
    pub writable_locations: Vec<AccountRegion>,

    /// Regions from which the account can be read.
    pub readable_locations: Vec<AccountRegion>,

    /// True when multi-master writes are enabled.
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
    pub user_replication_policy: ReplicationPolicy,

    /// Default consistency level configured by the user.
    pub user_consistency_policy: ConsistencyPolicy,

    /// System-managed replication sizing policy.
    pub system_replication_policy: ReplicationPolicy,

    /// Coefficients guiding regional read preference selection.
    pub read_policy: ReadPolicy,

    /// Raw JSON string containing query engine feature/configuration flags.
    pub query_engine_configuration: String,

    /// Regional Gateway 2.0 endpoints accepting writes (thin client mode).
    /// When present, indicates that Gateway 2.0 should be used for the
    /// dataplane transport instead of the standard gateway endpoint.
    #[serde(default)]
    pub thin_client_writable_locations: Vec<AccountRegion>,

    /// Regional Gateway 2.0 endpoints for reads (thin client mode).
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

    /// Returns `true` if Gateway 2.0 (thin client) endpoints are available.
    ///
    /// When thin client locations are present in the account properties,
    /// the driver should use Gateway 2.0 for the dataplane transport.
    pub(crate) fn has_thin_client_endpoints(&self) -> bool {
        !self.thin_client_writable_locations.is_empty()
            || !self.thin_client_readable_locations.is_empty()
    }

    /// Returns thin client (Gateway 2.0) writable locations, if any.
    pub(crate) fn thin_client_writable_regions(&self) -> Vec<Region> {
        self.thin_client_writable_locations
            .iter()
            .map(|loc| loc.name.clone())
            .collect()
    }

    /// Returns thin client (Gateway 2.0) readable locations, if any.
    pub(crate) fn thin_client_readable_regions(&self) -> Vec<Region> {
        self.thin_client_readable_locations
            .iter()
            .map(|loc| loc.name.clone())
            .collect()
    }
}

/// Default minimum interval between metadata refreshes (10 minutes).
///
/// Matches the SDK's `GlobalEndpointManager` TTL and background refresh interval.
const DEFAULT_STALENESS_THRESHOLD: Duration = Duration::from_secs(600);

/// Cache for Cosmos DB account metadata.
///
/// Stores account properties keyed by account endpoint.
#[derive(Debug)]
pub(crate) struct AccountMetadataCache {
    cache: AsyncCache<AccountEndpoint, AccountProperties>,

    /// Tracks the last time each endpoint's metadata was refreshed.
    last_refresh: async_lock::RwLock<std::collections::HashMap<AccountEndpoint, Instant>>,

    /// Minimum interval between refresh attempts to rate-limit requests.
    #[allow(dead_code)] // Used by refresh_if_stale, consumer coming once Driver is used
    staleness_threshold: Duration,

    /// Serializes refresh attempts so that concurrent callers share a single
    /// network fetch instead of each issuing redundant requests.
    #[allow(dead_code)] // Used by refresh_if_stale, consumer coming once Driver is used
    refresh_mutex: async_lock::Mutex<()>,
}

impl AccountMetadataCache {
    /// Creates a new empty account metadata cache.
    pub(crate) fn new() -> Self {
        Self {
            cache: AsyncCache::new(),
            last_refresh: async_lock::RwLock::new(std::collections::HashMap::new()),
            staleness_threshold: DEFAULT_STALENESS_THRESHOLD,
            refresh_mutex: async_lock::Mutex::new(()),
        }
    }

    /// Gets account properties from cache, or fetches and caches them.
    ///
    /// If the fetch fails, the error is propagated and nothing is cached,
    /// so the next call will try fetching again.
    pub(crate) async fn get_or_fetch<F, Fut>(
        &self,
        endpoint: AccountEndpoint,
        fetch_fn: F,
    ) -> azure_core::Result<Arc<AccountProperties>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<AccountProperties>>,
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
            .get_or_insert_with(endpoint.clone(), || async { properties })
            .await;

        // Record the fetch time after caching, so a concurrent thread
        // that loses the race does not reset the staleness clock with
        // a discarded fetch.
        {
            let mut timestamps = self.last_refresh.write().await;
            timestamps.insert(endpoint, Instant::now());
        }

        Ok(result)
    }

    /// Refreshes account properties if they are stale.
    ///
    /// "Stale" means the last refresh was more than the staleness threshold
    /// ago (default 10 minutes, matching the SDK's background refresh interval),
    /// or there is no cached value for the endpoint.
    ///
    /// Uses double-checked locking to ensure that concurrent callers share a
    /// single network fetch: the first caller to acquire the refresh mutex
    /// performs the fetch while subsequent callers wait and re-check staleness,
    /// finding the entry already refreshed.
    ///
    /// When the entry is considered stale, this method attempts to refresh it
    /// using `fetch_fn`. If the fetch fails, the existing cached value (if any)
    /// is preserved and returned. Returns `None` only when there is no cached
    /// value and the entry is not considered stale.
    #[allow(dead_code)] // Consumer coming in cutover
    pub(crate) async fn refresh_if_stale<F, Fut>(
        &self,
        endpoint: AccountEndpoint,
        fetch_fn: F,
    ) -> azure_core::Result<Option<Arc<AccountProperties>>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<AccountProperties>>,
    {
        // First check: fast path without exclusive locking.
        if !self.is_stale(&endpoint).await {
            return Ok(self.cache.get(&endpoint).await);
        }

        // Acquire refresh mutex to serialize concurrent refresh attempts.
        let _guard = self.refresh_mutex.lock().await;

        // Second check: another caller may have refreshed while we waited.
        if !self.is_stale(&endpoint).await {
            return Ok(self.cache.get(&endpoint).await);
        }

        // We are the sole refresher — fetch from the service.
        let cached = self.cache.get(&endpoint).await;
        let properties = match fetch_fn().await {
            Ok(props) => props,
            Err(e) => {
                // On fetch failure, return the existing cached value (if any)
                // so stale data is preferred over no data.
                if cached.is_some() {
                    return Ok(cached);
                }
                return Err(e);
            }
        };

        let endpoint_for_timestamp = endpoint.clone();

        let result = self
            .cache
            .get_or_refresh_with(
                endpoint,
                |_existing| true, // We already determined staleness above
                || async { properties },
            )
            .await;

        // Update the refresh timestamp.
        if result.is_some() {
            let mut timestamps = self.last_refresh.write().await;
            timestamps.insert(endpoint_for_timestamp, Instant::now());
        }

        Ok(result)
    }

    /// Returns `true` if the cached metadata for `endpoint` is stale or absent.
    #[allow(dead_code)] // Used by refresh_if_stale
    async fn is_stale(&self, endpoint: &AccountEndpoint) -> bool {
        let cached = self.cache.get(endpoint).await;
        let timestamps = self.last_refresh.read().await;
        match timestamps.get(endpoint) {
            Some(last) => cached.is_none() || last.elapsed() > self.staleness_threshold,
            None => true,
        }
    }

    /// Invalidates cached account properties for an endpoint.
    pub(crate) async fn invalidate(
        &self,
        endpoint: &AccountEndpoint,
    ) -> Option<Arc<AccountProperties>> {
        self.cache.invalidate(endpoint).await
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

    #[tokio::test]
    async fn refresh_if_stale_returns_cached_value_when_fresh() {
        // A fresh cache entry (within the default 10-minute staleness threshold)
        // should be returned without calling the factory.
        let cache = AccountMetadataCache::new();
        let endpoint = test_endpoint("myaccount");

        // Populate the cache
        cache
            .get_or_fetch(endpoint.clone(), || async { Ok(test_properties("westus")) })
            .await
            .unwrap();

        // Immediately calling refresh_if_stale should NOT refresh (not stale yet)
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        let result = cache
            .refresh_if_stale(endpoint, || async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                Ok(test_properties("eastus"))
            })
            .await
            .unwrap();

        // Should return the cached value without calling the factory
        assert!(result.is_some());
        assert_eq!(result.unwrap().write_region().unwrap().as_str(), "westus");
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn refresh_if_stale_refreshes_when_threshold_exceeded() {
        // Create cache with zero staleness threshold so everything is immediately stale
        let cache = AccountMetadataCache {
            cache: AsyncCache::new(),
            last_refresh: async_lock::RwLock::new(std::collections::HashMap::new()),
            staleness_threshold: Duration::from_secs(0),
            refresh_mutex: async_lock::Mutex::new(()),
        };
        let endpoint = test_endpoint("myaccount");

        // Populate with initial data
        cache
            .get_or_fetch(endpoint.clone(), || async { Ok(test_properties("westus")) })
            .await
            .unwrap();

        // With zero threshold, the data should be considered stale immediately
        let result = cache
            .refresh_if_stale(endpoint, || async { Ok(test_properties("eastus")) })
            .await
            .unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().write_region().unwrap().as_str(), "eastus");
    }

    #[tokio::test]
    async fn refresh_if_stale_returns_cached_on_fetch_failure() {
        // When the fetch fails but a cached value exists, the stale cached
        // value should be returned instead of propagating the error.
        let cache = AccountMetadataCache {
            cache: AsyncCache::new(),
            last_refresh: async_lock::RwLock::new(std::collections::HashMap::new()),
            staleness_threshold: Duration::from_secs(0),
            refresh_mutex: async_lock::Mutex::new(()),
        };
        let endpoint = test_endpoint("myaccount");

        // Populate with initial data
        cache
            .get_or_fetch(endpoint.clone(), || async { Ok(test_properties("westus")) })
            .await
            .unwrap();

        // Fetch fails — should return the stale cached value
        let result = cache
            .refresh_if_stale(endpoint, || async {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "network failure",
                ))
            })
            .await
            .unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().write_region().unwrap().as_str(), "westus");
    }

    #[tokio::test]
    async fn refresh_if_stale_propagates_error_when_no_cached_value() {
        // When the fetch fails and there is no cached value, the error
        // should be propagated to the caller.
        let cache = AccountMetadataCache {
            cache: AsyncCache::new(),
            last_refresh: async_lock::RwLock::new(std::collections::HashMap::new()),
            staleness_threshold: Duration::from_secs(0),
            refresh_mutex: async_lock::Mutex::new(()),
        };
        let endpoint = test_endpoint("myaccount");

        // No prior cached data — fetch fails
        let result = cache
            .refresh_if_stale(endpoint, || async {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "network failure",
                ))
            })
            .await;

        assert!(result.is_err());
    }
}
