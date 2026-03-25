// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key range cache for resolving effective partition keys to range IDs.
//!
//! Uses the driver's operation pipeline to fetch `/pkranges` from the service
//! and caches the resulting [`CollectionRoutingMap`] per collection RID.

use std::sync::Arc;

use super::{collection_routing_map::CollectionRoutingMap, AsyncCache};
use crate::models::partition_key_range::PkRangesResponse;
use crate::models::{
    effective_partition_key::compute_effective_partition_key, ContainerReference, PartitionKey,
};

/// Maximum number of change feed iterations to prevent infinite loops.
const MAX_FETCH_ITERATIONS: usize = 1000;

/// Result of a single partition key range fetch from the service.
///
/// Callers construct this from the HTTP response: parsing the body for ranges,
/// extracting the `etag` header as `continuation`, and checking for HTTP 304 (Not Modified).
#[derive(Debug)]
pub(crate) struct PkRangeFetchResult {
    /// The partition key ranges returned in this page (empty if `not_modified` is true).
    pub ranges: Vec<crate::models::partition_key_range::PartitionKeyRange>,
    /// The continuation token (from the `etag` response header) for the next fetch.
    pub continuation: Option<String>,
    /// True if the server returned HTTP 304 Not Modified.
    pub not_modified: bool,
}

/// Cache that maps collection RIDs to their partition key routing maps.
///
/// When a partition key range ID is needed (for partition-level failover),
/// this cache computes the effective partition key (EPK) from the partition key
/// values and looks up the corresponding range ID in the routing map.
///
/// The routing map is fetched lazily from the service the first time a
/// collection is queried, then cached until invalidated.
#[derive(Debug)]
pub(crate) struct PartitionKeyRangeCache {
    /// Keyed by collection RID.
    cache: AsyncCache<String, CollectionRoutingMap>,
}

impl PartitionKeyRangeCache {
    /// Creates a new empty cache.
    pub fn new() -> Self {
        Self {
            cache: AsyncCache::new(),
        }
    }

    /// Resolves the partition key range ID for a given container and partition key.
    ///
    /// 1. Computes the effective partition key (EPK) from the partition key values.
    /// 2. Looks up the EPK in the cached routing map.
    /// 3. If no routing map is cached, fetches it from the service first.
    ///
    /// Returns `None` if the partition key is empty (cross-partition) or if
    /// the routing map cannot be resolved.
    pub async fn resolve_partition_key_range_id<F, Fut>(
        &self,
        container: &ContainerReference,
        partition_key: &PartitionKey,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<String>
    where
        F: Fn(String, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        if partition_key.is_empty() {
            return None;
        }

        let pk_def = container.partition_key_definition();
        let kind = pk_def.kind();
        let version = pk_def.version();

        let epk = compute_effective_partition_key(partition_key.values(), kind, version);

        let routing_map = self
            .try_lookup(container.rid(), force_refresh, fetch_pk_ranges)
            .await?;

        routing_map
            .get_range_by_effective_partition_key(&epk)
            .map(|r| r.id.clone())
    }

    /// Resolves all partition key ranges that overlap with the given EPK range.
    ///
    /// Returns `None` if the routing map cannot be resolved.
    /// When `force_refresh` is true, the cached routing map is refreshed before lookup.
    pub async fn resolve_overlapping_ranges<F, Fut>(
        &self,
        collection_rid: &str,
        min_inclusive: &str,
        max_exclusive: &str,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<Vec<crate::models::partition_key_range::PartitionKeyRange>>
    where
        F: Fn(String, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let routing_map = self
            .try_lookup(collection_rid, force_refresh, fetch_pk_ranges)
            .await?;

        Some(
            routing_map
                .get_overlapping_ranges(min_inclusive, max_exclusive)
                .into_iter()
                .cloned()
                .collect(),
        )
    }

    /// Resolves a partition key range by its ID.
    ///
    /// Returns `None` if the routing map cannot be resolved or the ID is not found.
    /// When `force_refresh` is true, the cached routing map is refreshed before lookup.
    pub async fn resolve_partition_key_range_by_id<F, Fut>(
        &self,
        collection_rid: &str,
        partition_key_range_id: &str,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<crate::models::partition_key_range::PartitionKeyRange>
    where
        F: Fn(String, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let routing_map = self
            .try_lookup(collection_rid, force_refresh, fetch_pk_ranges)
            .await?;

        routing_map.get_range_by_id(partition_key_range_id).cloned()
    }

    /// Looks up or fetches the routing map for a collection.
    ///
    /// When `force_refresh` is true, the previous routing map's change-feed
    /// continuation token is used for an incremental fetch. If the service
    /// returns 304 Not Modified, the existing map is returned as-is.
    /// Otherwise the new ranges are merged via [`CollectionRoutingMap::try_combine`].
    ///
    /// Returns a routing map for the collection. If the initial fetch fails or
    /// returns invalid ranges, an empty routing map is cached and returned.
    async fn try_lookup<F, Fut>(
        &self,
        collection_rid: &str,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<Arc<CollectionRoutingMap>>
    where
        F: Fn(String, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let rid = collection_rid.to_string();

        if force_refresh {
            // Retrieve the existing routing map for incremental refresh.
            let previous = self.cache.get(&rid).await;
            let prev_continuation = previous
                .as_ref()
                .and_then(|m| m.change_feed_next_if_none_match.clone());

            self.cache
                .get_or_refresh_with(
                    rid.clone(),
                    |existing| {
                        // If there's no existing entry, we must fetch to populate the cache.
                        if existing.is_none() {
                            return true;
                        }
                        // Only refresh if the cached value hasn't been updated
                        // by another concurrent request since we last saw it.
                        existing.map(|m| &m.change_feed_next_if_none_match)
                            == Some(&prev_continuation)
                    },
                    || Self::fetch_and_build_routing_map(rid, previous, fetch_pk_ranges),
                )
                .await
        } else {
            Some(
                self.cache
                    .get_or_insert_with(rid.clone(), || {
                        Self::fetch_and_build_routing_map(rid, None, fetch_pk_ranges)
                    })
                    .await,
            )
        }
    }

    /// Fetches partition key ranges via change-feed loop and builds a routing map.
    ///
    /// This mirrors the SDK's `get_routing_map_for_collection` pattern:
    ///
    /// 1. Start from the previous map's continuation token (or `None` for fresh fetch).
    /// 2. Loop calling `fetch_pk_ranges(collection_rid, continuation)` until the
    ///    service returns 304 Not Modified or `MAX_FETCH_ITERATIONS` is reached.
    /// 3. Accumulate all fetched ranges.
    /// 4. If a previous map exists, merge via [`CollectionRoutingMap::try_combine`];
    ///    otherwise create a fresh routing map.
    async fn fetch_and_build_routing_map<F, Fut>(
        collection_rid: String,
        previous_routing_map: Option<Arc<CollectionRoutingMap>>,
        fetch_pk_ranges: F,
    ) -> CollectionRoutingMap
    where
        F: Fn(String, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let mut all_ranges = Vec::new();
        let mut continuation = previous_routing_map
            .as_ref()
            .and_then(|m| m.change_feed_next_if_none_match.clone());

        for iteration in 0..MAX_FETCH_ITERATIONS {
            let result = match fetch_pk_ranges(collection_rid.clone(), continuation.clone()).await {
                Some(r) => r,
                None => {
                    tracing::warn!(
                        "Failed to fetch partition key ranges from service (iteration {})",
                        iteration
                    );
                    // Preserve the previous routing map during incremental refresh
                    // so a transient failure doesn't discard valid cached routing.
                    if let Some(prev) = previous_routing_map {
                        return (*prev).clone();
                    }

                    return CollectionRoutingMap::empty();
                }
            };

            continuation = result.continuation;

            if result.not_modified {
                break;
            }

            all_ranges.extend(result.ranges);
        }

        // If we exhausted all iterations without a 304/not_modified, warn.
        // This likely means the service is returning an unusually large number
        // of change-feed pages and the routing map may be incomplete.
        if !all_ranges.is_empty() && continuation.is_some() {
            tracing::warn!(
                "Partition key range change-feed loop reached MAX_FETCH_ITERATIONS ({}) \
                 without receiving not_modified; routing map may be incomplete",
                MAX_FETCH_ITERATIONS
            );
        }

        // Incremental refresh: merge new ranges into the previous routing map.
        if let Some(prev) = previous_routing_map {
            if all_ranges.is_empty() {
                // No changes since last fetch (304 on first iteration).
                return (*prev).clone();
            }
            match prev.try_combine(all_ranges, continuation) {
                Ok(Some(map)) => return map,
                Ok(None) => {
                    tracing::warn!(
                        "Incremental routing map merge incomplete; falling back to previous map"
                    );
                    return (*prev).clone();
                }
                Err(e) => {
                    tracing::warn!(
                        "Incremental routing map merge failed: {}; falling back to previous map",
                        e
                    );
                    return (*prev).clone();
                }
            }
        }

        // Full (non-incremental) creation.
        match CollectionRoutingMap::try_create_with_continuation(all_ranges, None, continuation) {
            Ok(Some(map)) => map,
            Ok(None) => {
                tracing::warn!("Partition key range fetch returned empty set");
                CollectionRoutingMap::empty()
            }
            Err(e) => {
                tracing::warn!("Partition key ranges invalid: {}", e);
                CollectionRoutingMap::empty()
            }
        }
    }

    /// Parses a pkranges REST response body into partition key ranges.
    pub(crate) fn parse_pk_ranges_response(
        body: &[u8],
    ) -> Option<Vec<crate::models::partition_key_range::PartitionKeyRange>> {
        let response: PkRangesResponse = serde_json::from_slice(body).ok()?;
        Some(response.partition_key_ranges)
    }

    /// Invalidates the cached routing map for a collection.
    ///
    /// Call this when a partition split is detected (e.g., 410/1002 Gone response).
    pub async fn invalidate(&self, collection_rid: &str) {
        self.cache.invalidate(&collection_rid.to_string()).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::partition_key_range::PartitionKeyRange as PkRange;

    fn test_ranges() -> Vec<PkRange> {
        vec![PkRange {
            id: "0".into(),
            resource_id: None,
            self_link: None,
            etag: None,
            timestamp: None,
            min_inclusive: "".into(),
            max_exclusive: "FF".into(),
            rid_prefix: None,
            throughput_fraction: 0.0,
            target_throughput: None,
            status: Default::default(),
            lsn: 0,
            parents: None,
            owned_archival_pk_range_ids: None,
        }]
    }

    /// Simulates a single-page change feed fetch:
    /// - First call (no continuation): returns all ranges + continuation token.
    /// - Subsequent calls (with continuation): returns 304 Not Modified.
    async fn test_fetch(_rid: String, continuation: Option<String>) -> Option<PkRangeFetchResult> {
        if continuation.is_some() {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(PkRangeFetchResult {
                ranges: test_ranges(),
                continuation: Some("test-etag".to_string()),
                not_modified: false,
            })
        }
    }

    #[tokio::test]
    async fn resolve_returns_range_id() {
        let cache = PartitionKeyRangeCache::new();
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key",
        );
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
            system_properties: Default::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        );
        let pk = PartitionKey::from("hello");

        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, false, test_fetch)
            .await;

        assert!(range_id.is_some());
        assert_eq!(range_id.unwrap(), "0");
    }

    #[tokio::test]
    async fn empty_pk_returns_none() {
        let cache = PartitionKeyRangeCache::new();
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key",
        );
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
            system_properties: Default::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        );
        let pk = PartitionKey::EMPTY;

        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, false, test_fetch)
            .await;

        assert!(range_id.is_none());
    }

    #[tokio::test]
    async fn force_refresh_uses_incremental_merge() {
        let cache = PartitionKeyRangeCache::new();
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key",
        );
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
            system_properties: Default::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        );
        let pk = PartitionKey::from("hello");

        // First call: populates the cache.
        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, false, test_fetch)
            .await;
        assert_eq!(range_id.as_deref(), Some("0"));

        // Second call with force_refresh: the test_fetch callback sees the continuation
        // from the cached map and returns 304 (not_modified), so the existing map is reused.
        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, true, test_fetch)
            .await;
        assert_eq!(range_id.as_deref(), Some("0"));
    }

    #[tokio::test]
    async fn fetch_failure_on_initial_returns_none() {
        let cache = PartitionKeyRangeCache::new();

        // A fetch function that always fails.
        async fn failing_fetch(_rid: String, _cont: Option<String>) -> Option<PkRangeFetchResult> {
            None
        }

        let map = cache.try_lookup("test-rid", false, failing_fetch).await;
        // The cache should store an empty map, so try_lookup returns Some
        // but subsequent EPK lookups will return None.
        assert!(map.is_some());
        let map = map.unwrap();
        assert!(map.ordered_ranges().is_empty());
    }

    #[tokio::test]
    async fn fetch_failure_during_incremental_refresh_preserves_previous_map() {
        let cache = PartitionKeyRangeCache::new();
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key",
        );
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
            system_properties: Default::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        );
        let pk = PartitionKey::from("hello");

        // First call: populate the cache with a valid routing map.
        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, false, test_fetch)
            .await;
        assert_eq!(range_id.as_deref(), Some("0"));

        // A fetch function that always fails (simulating a transient error).
        async fn failing_fetch(_rid: String, _cont: Option<String>) -> Option<PkRangeFetchResult> {
            None
        }

        // Force refresh with a failing fetch should preserve the previous map.
        let range_id = cache
            .resolve_partition_key_range_id(&container, &pk, true, failing_fetch)
            .await;
        assert_eq!(
            range_id.as_deref(),
            Some("0"),
            "Previous routing map should be preserved when incremental fetch fails"
        );
    }
}
