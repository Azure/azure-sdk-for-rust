// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key range cache for resolving effective partition keys to range IDs.
//!
//! Uses the driver's operation pipeline to fetch `/pkranges` from the service
//! and caches the resulting [`ContainerRoutingMap`] per container RID.

use std::{collections::HashMap, sync::Arc};

use crate::models::{
    effective_partition_key::EffectivePartitionKey, partition_key_range::PkRangesResponse,
    ContainerReference, PartitionKey,
};

use super::{container_routing_map::ContainerRoutingMap, AsyncCache};

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

/// Cache that maps container RIDs to their partition key routing maps.
///
/// When a partition key range ID is needed (for partition-level failover),
/// this cache computes the effective partition key (EPK) from the partition key
/// values and looks up the corresponding range ID in the routing map.
///
/// The routing map is fetched lazily from the service the first time a
/// container is queried, then cached until invalidated.
#[derive(Debug)]
pub(crate) struct PartitionKeyRangeCache {
    /// Keyed by [`ContainerReference`], which provides the container RID
    /// needed for the `x-ms-expected-rid` header on pkrange changefeed calls.
    cache: AsyncCache<ContainerReference, ContainerRoutingMap>,
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
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        if partition_key.is_empty() {
            return None;
        }

        let pk_def = container.partition_key_definition();
        let kind = pk_def.kind();
        let version = pk_def.version();

        let epk = EffectivePartitionKey::compute(partition_key.values(), kind, version);

        let routing_map = self
            .try_lookup(container, force_refresh, fetch_pk_ranges)
            .await?;

        routing_map
            .get_range_by_effective_partition_key(&epk)
            .map(|r| r.id.clone())
    }

    /// Resolves partition key range IDs for a given container and partition key,
    /// supporting both full and prefix (hierarchical) partition keys.
    ///
    /// For full partition keys (component count == definition path count), returns
    /// a single range ID (same as [`resolve_partition_key_range_id`](Self::resolve_partition_key_range_id)).
    ///
    /// For prefix partition keys on MultiHash containers (fewer components than
    /// the definition), computes the prefix EPK range and returns all overlapping
    /// partition key range IDs, enabling fan-out queries across multiple physical
    /// partitions.
    ///
    /// Returns `None` if the partition key is empty or the routing map cannot be resolved.
    pub async fn resolve_partition_key_range_ids<F, Fut>(
        &self,
        container: &ContainerReference,
        partition_key: &PartitionKey,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<Vec<String>>
    where
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        if partition_key.is_empty() {
            return None;
        }

        let pk_def = container.partition_key_definition();
        let epk_range =
            EffectivePartitionKey::compute_range(partition_key.values(), pk_def).ok()?;

        if epk_range.start == epk_range.end {
            // Full key — point lookup
            let routing_map = self
                .try_lookup(container, force_refresh, fetch_pk_ranges)
                .await?;
            routing_map
                .get_range_by_effective_partition_key(&epk_range.start)
                .map(|r| vec![r.id.clone()])
        } else {
            // Prefix key — overlapping range lookup
            self.resolve_overlapping_ranges(
                container,
                &epk_range.start..&epk_range.end,
                force_refresh,
                fetch_pk_ranges,
            )
            .await
            .map(|ranges| ranges.into_iter().map(|r| r.id).collect())
        }
    }

    /// Resolves all partition key ranges that overlap with the given EPK range.
    ///
    /// Returns `None` if the routing map cannot be resolved.
    /// When `force_refresh` is true, the cached routing map is refreshed before lookup.
    pub async fn resolve_overlapping_ranges<F, Fut>(
        &self,
        container: &ContainerReference,
        epk_range: std::ops::Range<&EffectivePartitionKey>,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<Vec<crate::models::partition_key_range::PartitionKeyRange>>
    where
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let routing_map = self
            .try_lookup(container, force_refresh, fetch_pk_ranges)
            .await?;

        if epk_range.start == epk_range.end {
            // Point range (equality / `IN` predicate resolves to the single EPK
            // `X`). `get_overlapping_ranges` treats `X..X` as an empty
            // `std::ops::Range` and misses the owning partition when `X` sits on
            // a partition's lower boundary. Resolve via the boundary-correct
            // point lookup instead (mirrors `resolve_partition_key_range_ids`).
            return Some(
                routing_map
                    .get_range_by_effective_partition_key(epk_range.start)
                    .cloned()
                    .into_iter()
                    .collect(),
            );
        }

        Some(
            routing_map
                .get_overlapping_ranges(epk_range)
                .into_iter()
                .cloned()
                .collect(),
        )
    }

    /// Resolves the ID of the single partition key range that owns the given
    /// EPK range, or `None` when the range maps to zero or more than one
    /// physical partition (or the routing map cannot be resolved).
    ///
    /// Unlike [`resolve_overlapping_ranges`](Self::resolve_overlapping_ranges),
    /// this clones at most a single range ID rather than every overlapping
    /// range, making it the cheaper choice for callers that only need
    /// single-owner attribution (e.g. PPCB/PPAF first-attempt seeding).
    /// When `force_refresh` is true, the cached routing map is refreshed before lookup.
    pub async fn resolve_single_overlapping_range_id<F, Fut>(
        &self,
        container: &ContainerReference,
        epk_range: std::ops::Range<&EffectivePartitionKey>,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<String>
    where
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let routing_map = self
            .try_lookup(container, force_refresh, fetch_pk_ranges)
            .await?;

        routing_map.single_overlapping_range_id(epk_range)
    }

    /// Resolves a partition key range by its ID.
    ///
    /// Returns `None` if the routing map cannot be resolved or the ID is not found.
    /// When `force_refresh` is true, the cached routing map is refreshed before lookup.
    pub async fn resolve_partition_key_range_by_id<F, Fut>(
        &self,
        container: &ContainerReference,
        partition_key_range_id: &str,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<crate::models::partition_key_range::PartitionKeyRange>
    where
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let routing_map = self
            .try_lookup(container, force_refresh, fetch_pk_ranges)
            .await?;

        routing_map.range(partition_key_range_id).cloned()
    }

    /// Looks up or fetches the routing map for a container.
    ///
    /// When `force_refresh` is true, the previous routing map's change-feed
    /// continuation token is used for an incremental fetch. If the service
    /// returns 304 Not Modified, the existing map is returned as-is.
    /// Otherwise, the new ranges are merged via [`ContainerRoutingMap::try_combine`].
    ///
    /// Returns a routing map for the container. If the initial fetch fails or
    /// returns invalid ranges, the previously cached routing map is preserved
    /// when one exists. Empty routing maps are evicted and returned as `None`.
    pub(crate) async fn try_lookup<F, Fut>(
        &self,
        container: &ContainerReference,
        force_refresh: bool,
        fetch_pk_ranges: F,
    ) -> Option<Arc<ContainerRoutingMap>>
    where
        F: Fn(ContainerReference, Option<String>) -> Fut,
        Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
    {
        let key = container.clone();

        let routing_map = if force_refresh {
            // Retrieve the existing routing map for incremental refresh.
            let previous = self.cache.get(&key).await;
            let prev_continuation = previous
                .as_ref()
                .and_then(|m| m.change_feed_next_if_none_match.clone());

            self.cache
                .get_or_refresh_with(
                    key.clone(),
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
                    || fetch_and_build_routing_map(key.clone(), previous, fetch_pk_ranges),
                )
                .await?
        } else {
            self.cache
                .get_or_insert_with(key.clone(), || {
                    fetch_and_build_routing_map(key.clone(), None, fetch_pk_ranges)
                })
                .await
        };

        if routing_map.ranges().is_empty() {
            self.cache.invalidate_if_same(&key, &routing_map).await;
            return None;
        }

        Some(routing_map)
    }

    /// Invalidates the cached routing map for a container.
    ///
    /// Call this when a partition split is detected (e.g., 410/1002 Gone response).
    pub async fn invalidate(&self, container: &ContainerReference) {
        self.cache.invalidate(container).await;
    }
}

/// Fetches partition key ranges via change-feed loop and builds a routing map.
///
/// This mirrors the SDK's routing-map-for-container pattern:
///
/// 1. Start from the previous map's continuation token (or `None` for fresh fetch).
/// 2. Continue fetching without a client-side iteration cap until the service
///    returns 304 Not Modified.
/// 3. Accumulate all fetched ranges.
/// 4. If a previous map exists, merge via [`ContainerRoutingMap::try_combine`];
///    otherwise create a fresh routing map.
async fn fetch_and_build_routing_map<F, Fut>(
    container: ContainerReference,
    previous_routing_map: Option<Arc<ContainerRoutingMap>>,
    fetch_pk_ranges: F,
) -> ContainerRoutingMap
where
    F: Fn(ContainerReference, Option<String>) -> Fut,
    Fut: std::future::Future<Output = Option<PkRangeFetchResult>>,
{
    let mut all_ranges = HashMap::new();
    let mut continuation = previous_routing_map
        .as_ref()
        .and_then(|m| m.change_feed_next_if_none_match.clone());
    let mut iterations_completed = 0;
    loop {
        let iteration = iterations_completed;
        iterations_completed += 1;

        tracing::trace!(
            iteration,
            has_continuation = continuation.is_some(),
            "Fetching partition key ranges"
        );

        let result = match fetch_pk_ranges(container.clone(), continuation.clone()).await {
            Some(r) => r,
            None => {
                // Falling back to the previously cached map (when one exists)
                // mirrors the merge branch below and avoids regressing the
                // cache to empty on a single transient fetch failure.
                tracing::warn!(
                    "Failed to fetch partition key ranges from service (iteration {}); \
                     falling back to previous routing map if available",
                    iteration
                );
                return previous_routing_map
                    .map(|p| (*p).clone())
                    .unwrap_or_else(ContainerRoutingMap::empty);
            }
        };

        if result.not_modified {
            continuation = result.continuation.or(continuation);
            tracing::trace!(iteration, "Service returned 304 Not Modified");
            break;
        }

        continuation = result.continuation.or(continuation);

        tracing::trace!(
            iteration,
            range_count = result.ranges.len(),
            "Received partition key ranges"
        );
        all_ranges.extend(
            result
                .ranges
                .into_iter()
                .map(|range| (range.id.clone(), range)),
        );
    }

    tracing::debug!(
        iterations = iterations_completed,
        total_ranges = all_ranges.len(),
        "Partition key range fetch loop completed"
    );

    // Incremental refresh: merge new ranges into the previous routing map.
    if let Some(prev) = previous_routing_map {
        if all_ranges.is_empty() {
            let mut unchanged = (*prev).clone();
            unchanged.change_feed_next_if_none_match = continuation;
            return unchanged;
        }
        return match prev.try_combine(all_ranges.into_values().collect(), continuation) {
            Ok(Some(map)) => map,
            Ok(None) => {
                tracing::warn!(
                    "Incremental routing map merge incomplete; falling back to full refresh"
                );
                let refreshed = Box::pin(fetch_and_build_routing_map(
                    container,
                    None,
                    fetch_pk_ranges,
                ))
                .await;
                if refreshed.ranges().is_empty() {
                    (*prev).clone()
                } else {
                    refreshed
                }
            }
            Err(e) => {
                tracing::warn!(
                    "Incremental routing map merge failed: {}; falling back to full refresh",
                    e
                );
                let refreshed = Box::pin(fetch_and_build_routing_map(
                    container,
                    None,
                    fetch_pk_ranges,
                ))
                .await;
                if refreshed.ranges().is_empty() {
                    (*prev).clone()
                } else {
                    refreshed
                }
            }
        };
    }

    // Full (non-incremental) creation.
    match ContainerRoutingMap::try_create(all_ranges.into_values().collect(), None, continuation) {
        Ok(Some(map)) => map,
        Ok(None) => {
            tracing::warn!("Partition key range fetch returned empty set");
            ContainerRoutingMap::empty()
        }
        Err(e) => {
            tracing::warn!("Partition key ranges invalid: {}", e);
            ContainerRoutingMap::empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::partition_key_range::PartitionKeyRange as PkRange;

    fn test_ranges() -> Vec<PkRange> {
        vec![PkRange::new("0".into(), "", "FF")]
    }

    /// Simulates a single-page change feed fetch:
    /// - First call (no continuation): returns all ranges + continuation token.
    /// - Subsequent calls (with continuation): returns 304 Not Modified.
    async fn test_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> Option<PkRangeFetchResult> {
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

    #[test]
    fn parse_pk_ranges_response_test() {
        let body = br#"{
            "PartitionKeyRanges": [
                {"id": "0", "_rid": "rid0", "minInclusive": "", "maxExclusive": "FF"}
            ]
        }"#;
        let ranges = parse_pk_ranges_response(body).unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].id, "0");
    }

    // =========================================================================
    // Tests for resolve_partition_key_range_ids (MultiHash / prefix HPK)
    // =========================================================================

    fn make_container(pk_json: &str) -> ContainerReference {
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key",
        );
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(pk_json).unwrap(),
            system_properties: Default::default(),
        };
        ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        )
    }

    fn routing_map(ranges: Vec<PkRange>, continuation: &str) -> Arc<ContainerRoutingMap> {
        Arc::new(
            ContainerRoutingMap::try_create(ranges, None, Some(continuation.to_string()))
                .unwrap()
                .unwrap(),
        )
    }

    /// A fetch function returning two partition key ranges split at the midpoint "80".
    /// Range "0": ["", "80"), Range "1": ["80", "FF")
    async fn two_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> Option<PkRangeFetchResult> {
        if continuation.is_some() {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(PkRangeFetchResult {
                ranges: vec![
                    PkRange::new("0".into(), "", "80"),
                    PkRange::new("1".into(), "80", "FF"),
                ],
                continuation: Some("test-etag".to_string()),
                not_modified: false,
            })
        }
    }

    #[tokio::test]
    async fn resolve_ids_empty_pk_returns_none() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(
            r#"{"paths":["/tenantId","/userId","/sessionId"],"kind":"MultiHash","version":2}"#,
        );

        let result = cache
            .resolve_partition_key_range_ids(&container, &PartitionKey::EMPTY, false, test_fetch)
            .await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn resolve_ids_full_multihash_returns_single_id() {
        let cache = PartitionKeyRangeCache::new();
        let container =
            make_container(r#"{"paths":["/tenantId","/userId"],"kind":"MultiHash","version":2}"#);
        let pk = PartitionKey::from(("tenant1", "user1"));

        let result = cache
            .resolve_partition_key_range_ids(&container, &pk, false, test_fetch)
            .await;

        assert!(result.is_some());
        let ids = result.unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0], "0"); // single range ["", "FF") contains everything
    }

    #[tokio::test]
    async fn resolve_ids_prefix_multihash_returns_multiple_ids() {
        let cache = PartitionKeyRangeCache::new();
        // 3-path MultiHash container
        let container = make_container(
            r#"{"paths":["/tenantId","/userId","/sessionId"],"kind":"MultiHash","version":2}"#,
        );
        // Prefix key: only 1 of 3 components → prefix EPK range spans multiple ranges
        let pk = PartitionKey::from("tenant1");

        let result = cache
            .resolve_partition_key_range_ids(&container, &pk, false, two_range_fetch)
            .await;

        assert!(result.is_some());
        let ids = result.unwrap();
        // The prefix EPK for "tenant1" is a 32-char hex string starting with a digit 0-3
        // (due to 0x3F mask). With ranges split at "80", the prefix range [epk, epk+"FF")
        // falls entirely within range "0" (["", "80")). So we expect 1 ID.
        // This validates the prefix path is exercised (via resolve_overlapping_ranges).
        assert_eq!(ids, vec!["0".to_string()]);
    }

    #[tokio::test]
    async fn resolve_ids_non_multihash_returns_single_id() {
        let cache = PartitionKeyRangeCache::new();
        // Single-hash container (non-MultiHash)
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let pk = PartitionKey::from("hello");

        let result = cache
            .resolve_partition_key_range_ids(&container, &pk, false, test_fetch)
            .await;

        assert!(result.is_some());
        let ids = result.unwrap();
        assert_eq!(ids, vec!["0".to_string()]);
    }

    #[tokio::test]
    async fn resolve_ids_matches_single_resolve() {
        // Full MultiHash key via resolve_partition_key_range_ids should produce the
        // same result as resolve_partition_key_range_id.
        let cache = PartitionKeyRangeCache::new();
        let container =
            make_container(r#"{"paths":["/tenantId","/userId"],"kind":"MultiHash","version":2}"#);
        let pk = PartitionKey::from(("tenant1", "user1"));

        let single = cache
            .resolve_partition_key_range_id(&container, &pk, false, test_fetch)
            .await;
        let plural = cache
            .resolve_partition_key_range_ids(&container, &pk, false, test_fetch)
            .await;

        assert_eq!(single.as_deref(), Some("0"));
        assert_eq!(plural.as_deref(), Some(vec!["0".to_string()].as_slice()));
    }

    // =========================================================================
    // Tests for resolve_all_partition_key_ranges / resolve_partition_key_ranges_for_key
    // (scenarios matching CosmosDriver public methods)
    // =========================================================================

    #[tokio::test]
    async fn try_lookup_returns_all_ranges() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        let routing_map = cache.try_lookup(&container, false, two_range_fetch).await;

        assert!(routing_map.is_some());
        let ranges = routing_map.unwrap().ranges().to_vec();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].id, "0");
        assert_eq!(ranges[1].id, "1");
    }

    #[tokio::test]
    async fn try_lookup_empty_routing_map_returns_none() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        /// Simulates a service failure: returns no ranges and no continuation.
        async fn empty_fetch(
            _container: ContainerReference,
            _continuation: Option<String>,
        ) -> Option<PkRangeFetchResult> {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation: None,
                not_modified: true,
            })
        }

        let routing_map = cache.try_lookup(&container, false, empty_fetch).await;

        assert!(routing_map.is_none());
    }

    #[tokio::test]
    async fn try_lookup_fetch_failure_returns_none() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        /// Simulates a complete fetch failure (e.g., network error).
        async fn failing_fetch(
            _container: ContainerReference,
            _continuation: Option<String>,
        ) -> Option<PkRangeFetchResult> {
            None
        }

        let routing_map = cache.try_lookup(&container, false, failing_fetch).await;

        assert!(routing_map.is_none());
    }

    #[tokio::test]
    async fn resolve_overlapping_ranges_full_key_returns_single_range() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let pk = PartitionKey::from("hello");

        let pk_def = container.partition_key_definition();
        let epk_range = EffectivePartitionKey::compute_range(pk.values(), pk_def).unwrap();

        // Full key: start == end, so this is a point lookup via get_range_by_effective_partition_key.
        assert_eq!(epk_range.start, epk_range.end);

        // Use try_lookup + manual point lookup (mirrors CosmosDriver::resolve_partition_key_ranges_for_key)
        let routing_map = cache
            .try_lookup(&container, false, test_fetch)
            .await
            .unwrap();
        let range = routing_map.get_range_by_effective_partition_key(&epk_range.start);

        assert!(range.is_some());
        assert_eq!(range.unwrap().id, "0");
    }

    #[tokio::test]
    async fn resolve_overlapping_ranges_prefix_key_returns_multiple() {
        let cache = PartitionKeyRangeCache::new();
        // 3-path MultiHash container with two ranges split at "80"
        let container = make_container(
            r#"{"paths":["/tenantId","/userId","/sessionId"],"kind":"MultiHash","version":2}"#,
        );
        // Prefix key with only 2 of 3 components
        let pk = PartitionKey::from(("tenant1", "user1"));

        let pk_def = container.partition_key_definition();
        let epk_range = EffectivePartitionKey::compute_range(pk.values(), pk_def).unwrap();

        // Prefix key: start != end
        assert_ne!(epk_range.start, epk_range.end);

        let ranges = cache
            .resolve_overlapping_ranges(
                &container,
                &epk_range.start..&epk_range.end,
                false,
                two_range_fetch,
            )
            .await;

        assert!(ranges.is_some());
        // Prefix EPK range may overlap one or both physical ranges depending on hash
        let ranges = ranges.unwrap();
        assert!(!ranges.is_empty());
        // All returned ranges should have valid IDs
        for r in &ranges {
            assert!(!r.id.is_empty());
        }
    }

    #[tokio::test]
    async fn resolve_overlapping_ranges_point_resolves_to_owning_partition_including_boundary() {
        // Option B (issues #4574 / #4638): an equality / `IN` predicate yields a
        // *point* EPK range `X..X`. `get_overlapping_ranges` treats that as an
        // empty `std::ops::Range` and misses the owning partition when `X` sits
        // on a partition's lower boundary, so `resolve_overlapping_ranges` must
        // route points through the boundary-correct point lookup instead.
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        // Point strictly inside range "0" ["", "80").
        let inside = EffectivePartitionKey::from("40");
        let ranges = cache
            .resolve_overlapping_ranges(&container, &inside..&inside, false, two_range_fetch)
            .await
            .unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].id, "0");

        // Point exactly on the boundary "80" == range "1".min_inclusive. This is
        // the case `get_overlapping_ranges(X..X)` would miss (returns empty).
        let boundary = EffectivePartitionKey::from("80");
        let ranges = cache
            .resolve_overlapping_ranges(&container, &boundary..&boundary, false, two_range_fetch)
            .await
            .unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].id, "1");
    }

    #[tokio::test]
    async fn resolve_overlapping_ranges_empty_map_returns_none() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container(
            r#"{"paths":["/tenantId","/userId","/sessionId"],"kind":"MultiHash","version":2}"#,
        );
        let pk = PartitionKey::from("tenant1");

        let pk_def = container.partition_key_definition();
        let epk_range = EffectivePartitionKey::compute_range(pk.values(), pk_def).unwrap();

        /// Returns an empty routing map.
        async fn empty_fetch(
            _container: ContainerReference,
            _continuation: Option<String>,
        ) -> Option<PkRangeFetchResult> {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation: None,
                not_modified: true,
            })
        }

        let ranges = cache
            .resolve_overlapping_ranges(
                &container,
                &epk_range.start..&epk_range.end,
                false,
                empty_fetch,
            )
            .await;

        assert!(ranges.is_none());
    }

    #[tokio::test]
    async fn force_refresh_repopulates_after_empty_cache() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        let call_count = Arc::new(AtomicUsize::new(0));

        // First fetch: returns empty (simulates transient failure)
        let count = call_count.clone();
        let empty_fetch = move |_container: ContainerReference, _continuation: Option<String>| {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Some(PkRangeFetchResult {
                    ranges: vec![],
                    continuation: None,
                    not_modified: true,
                })
            }
        };

        // First lookup rejects the empty routing map and evicts it.
        assert!(cache
            .try_lookup(&container, false, empty_fetch)
            .await
            .is_none());

        // Force refresh with a fetch that returns valid ranges
        let recovering_fetch = |_container: ContainerReference, continuation: Option<String>| async move {
            Some(match continuation {
                Some(continuation) => PkRangeFetchResult {
                    ranges: vec![],
                    continuation: Some(continuation),
                    not_modified: true,
                },
                None => PkRangeFetchResult {
                    ranges: vec![PkRange::new("0".into(), "", "FF")],
                    continuation: Some("etag-2".to_string()),
                    not_modified: false,
                },
            })
        };

        let map2 = cache
            .try_lookup(&container, true, recovering_fetch)
            .await
            .unwrap();
        assert_eq!(map2.ranges().len(), 1);
        assert_eq!(map2.ranges()[0].id, "0");
    }

    #[tokio::test]
    async fn drains_more_than_ten_partition_range_pages() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        const BOUNDARIES: [&str; 12] = [
            "", "10", "20", "30", "40", "50", "60", "70", "80", "90", "A0", "FF",
        ];

        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let call_count = Arc::new(AtomicUsize::new(0));
        let continuations_seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let count = call_count.clone();
        let seen = continuations_seen.clone();
        let paged_fetch = move |_container: ContainerReference, continuation: Option<String>| {
            let count = count.clone();
            let seen = seen.clone();
            async move {
                seen.lock().unwrap().push(continuation.clone());
                let page = count.fetch_add(1, Ordering::SeqCst);
                if page == BOUNDARIES.len() - 1 {
                    return Some(PkRangeFetchResult {
                        ranges: vec![],
                        continuation,
                        not_modified: true,
                    });
                }

                Some(PkRangeFetchResult {
                    ranges: vec![PkRange::new(
                        page.to_string(),
                        BOUNDARIES[page],
                        BOUNDARIES[page + 1],
                    )],
                    continuation: Some(format!("etag-{page}")),
                    not_modified: false,
                })
            }
        };

        let routing_map = cache
            .try_lookup(&container, false, paged_fetch)
            .await
            .unwrap();

        assert_eq!(routing_map.ranges().len(), BOUNDARIES.len() - 1);
        assert_eq!(call_count.load(Ordering::SeqCst), BOUNDARIES.len());
        let expected = std::iter::once(None)
            .chain((0..BOUNDARIES.len() - 1).map(|page| Some(format!("etag-{page}"))))
            .collect::<Vec<_>>();
        assert_eq!(*continuations_seen.lock().unwrap(), expected);
    }

    #[tokio::test]
    async fn redelivered_ranges_are_deduplicated_by_id() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result =
            fetch_and_build_routing_map(container, None, move |_container, continuation| {
                let count = count.clone();
                async move {
                    Some(match count.fetch_add(1, Ordering::SeqCst) {
                        0 => PkRangeFetchResult {
                            ranges: vec![PkRange::new("0".into(), "", "80")],
                            continuation: Some("etag-before-split".to_string()),
                            not_modified: false,
                        },
                        1 => PkRangeFetchResult {
                            ranges: vec![
                                PkRange::new("0".into(), "", "80"),
                                PkRange::new("1".into(), "80", "FF"),
                            ],
                            continuation: Some("etag-after-split".to_string()),
                            not_modified: false,
                        },
                        2 => PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        },
                        call => panic!("unexpected fetch call: {call}"),
                    })
                }
            })
            .await;

        assert_eq!(result.ranges().len(), 2);
        assert_eq!(result.ranges()[0].id, "0");
        assert_eq!(result.ranges()[1].id, "1");
    }

    #[tokio::test]
    async fn immediate_not_modified_preserves_previous_ranges() {
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            |_container, continuation| async move {
                Some(PkRangeFetchResult {
                    ranges: vec![],
                    continuation,
                    not_modified: true,
                })
            },
        )
        .await;

        assert_eq!(result.ranges().len(), 1);
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-previous")
        );
    }

    #[tokio::test]
    async fn not_modified_wins_over_non_empty_payload() {
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            |_container, _continuation| async {
                Some(PkRangeFetchResult {
                    ranges: vec![PkRange::new("ignored".into(), "", "80")],
                    continuation: Some("etag-advanced".to_string()),
                    not_modified: true,
                })
            },
        )
        .await;

        assert_eq!(result.ranges().len(), 1);
        assert_eq!(result.ranges()[0].id, "0");
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-advanced")
        );
    }

    #[tokio::test]
    async fn terminal_page_advances_continuation_without_range_changes() {
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            |_container, _continuation| async {
                Some(PkRangeFetchResult {
                    ranges: vec![],
                    continuation: Some("etag-advanced".to_string()),
                    not_modified: true,
                })
            },
        )
        .await;

        assert_eq!(result.ranges().len(), 1);
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-advanced")
        );
    }

    #[tokio::test]
    async fn real_wire_not_modified_preserves_last_data_etag() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result =
            fetch_and_build_routing_map(container, None, move |_container, _continuation| {
                let count = count.clone();
                async move {
                    Some(if count.fetch_add(1, Ordering::SeqCst) == 0 {
                        PkRangeFetchResult {
                            ranges: test_ranges(),
                            continuation: Some("etag-data".to_string()),
                            not_modified: false,
                        }
                    } else {
                        PkRangeFetchResult {
                            ranges: vec![],
                            continuation: None,
                            not_modified: true,
                        }
                    })
                }
            })
            .await;

        assert_eq!(result.ranges().len(), 1);
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-data")
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn mid_drain_failure_does_not_cache_partial_map() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();
        let failing_fetch = move |_container: ContainerReference, _continuation: Option<String>| {
            let count = count.clone();
            async move {
                (count.fetch_add(1, Ordering::SeqCst) & 1 == 0).then(|| PkRangeFetchResult {
                    ranges: vec![PkRange::new("0".into(), "", "80")],
                    continuation: Some("etag-partial".to_string()),
                    not_modified: false,
                })
            }
        };

        assert!(cache
            .try_lookup(&container, false, failing_fetch.clone())
            .await
            .is_none());
        assert!(cache
            .try_lookup(&container, false, failing_fetch)
            .await
            .is_none());
        assert_eq!(call_count.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn missing_continuation_preserves_previous_map() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            move |_container, continuation| {
                let count = count.clone();
                async move {
                    Some(if count.fetch_add(1, Ordering::SeqCst) == 0 {
                        let mut updated = PkRange::new("0".into(), "", "FF");
                        updated.throughput_fraction = 0.5;
                        PkRangeFetchResult {
                            ranges: vec![updated],
                            continuation: None,
                            not_modified: false,
                        }
                    } else {
                        PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        }
                    })
                }
            },
        )
        .await;

        assert_eq!(result.ranges()[0].id, "0");
        assert_eq!(result.ranges()[0].throughput_fraction, 0.5);
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-previous")
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn unknown_incremental_parent_falls_back_to_full_refresh() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            move |_container, continuation| {
                let count = count.clone();
                async move {
                    Some(match count.fetch_add(1, Ordering::SeqCst) {
                        0 => {
                            let mut child = PkRange::new("child".into(), "", "FF");
                            child.parents = Some(vec!["ghost-parent".to_string()]);
                            PkRangeFetchResult {
                                ranges: vec![child],
                                continuation: Some("etag-child".to_string()),
                                not_modified: false,
                            }
                        }
                        1 | 3 => PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        },
                        2 => PkRangeFetchResult {
                            ranges: vec![
                                PkRange::new("full-left".into(), "", "80"),
                                PkRange::new("full-right".into(), "80", "FF"),
                            ],
                            continuation: Some("etag-full".to_string()),
                            not_modified: false,
                        },
                        call => panic!("unexpected fetch call: {call}"),
                    })
                }
            },
        )
        .await;

        assert_eq!(result.ranges()[0].id, "full-left");
        assert_eq!(result.ranges()[1].id, "full-right");
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-full")
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn failed_full_refresh_after_merge_failure_preserves_previous_map() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            move |_container, continuation| {
                let count = count.clone();
                async move {
                    match count.fetch_add(1, Ordering::SeqCst) {
                        0 => {
                            let mut child = PkRange::new("child".into(), "", "FF");
                            child.parents = Some(vec!["ghost-parent".to_string()]);
                            Some(PkRangeFetchResult {
                                ranges: vec![child],
                                continuation: Some("etag-child".to_string()),
                                not_modified: false,
                            })
                        }
                        1 => Some(PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        }),
                        2 => None,
                        call => panic!("unexpected fetch call: {call}"),
                    }
                }
            },
        )
        .await;

        assert_eq!(result.ranges()[0].id, "0");
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-previous")
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn overlapping_incremental_page_falls_back_to_full_refresh() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let previous = routing_map(test_ranges(), "etag-previous");
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();

        let result = fetch_and_build_routing_map(
            container,
            Some(previous),
            move |_container, continuation| {
                let count = count.clone();
                async move {
                    Some(match count.fetch_add(1, Ordering::SeqCst) {
                        0 => {
                            let mut left = PkRange::new("left".into(), "", "AA");
                            left.parents = Some(vec!["0".to_string()]);
                            let mut right = PkRange::new("right".into(), "80", "FF");
                            right.parents = Some(vec!["0".to_string()]);
                            PkRangeFetchResult {
                                ranges: vec![left, right],
                                continuation: Some("etag-overlap".to_string()),
                                not_modified: false,
                            }
                        }
                        1 | 3 => PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        },
                        2 => PkRangeFetchResult {
                            ranges: vec![
                                PkRange::new("full-left".into(), "", "80"),
                                PkRange::new("full-right".into(), "80", "FF"),
                            ],
                            continuation: Some("etag-full".to_string()),
                            not_modified: false,
                        },
                        call => panic!("unexpected fetch call: {call}"),
                    })
                }
            },
        )
        .await;

        assert_eq!(result.ranges()[0].id, "full-left");
        assert_eq!(result.ranges()[1].id, "full-right");
        assert_eq!(
            result.change_feed_next_if_none_match.as_deref(),
            Some("etag-full")
        );
        assert_eq!(call_count.load(Ordering::SeqCst), 4);
    }

    #[tokio::test]
    async fn per_page_retry_does_not_restart_completed_pages() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let first_page_attempts = Arc::new(AtomicUsize::new(0));
        let second_page_attempts = Arc::new(AtomicUsize::new(0));
        let first = first_page_attempts.clone();
        let second = second_page_attempts.clone();

        let result =
            fetch_and_build_routing_map(container, None, move |_container, continuation| {
                let first = first.clone();
                let second = second.clone();
                async move {
                    Some(match continuation.as_deref() {
                        None => {
                            first.fetch_add(1, Ordering::SeqCst);
                            PkRangeFetchResult {
                                ranges: vec![PkRange::new("0".into(), "", "80")],
                                continuation: Some("etag-1".to_string()),
                                not_modified: false,
                            }
                        }
                        Some("etag-1") => {
                            // The request pipeline retries internally; the drain sees
                            // only the successful page result.
                            second.fetch_add(2, Ordering::SeqCst);
                            PkRangeFetchResult {
                                ranges: vec![PkRange::new("1".into(), "80", "FF")],
                                continuation: Some("etag-2".to_string()),
                                not_modified: false,
                            }
                        }
                        Some("etag-2") => PkRangeFetchResult {
                            ranges: vec![],
                            continuation,
                            not_modified: true,
                        },
                        other => panic!("unexpected continuation: {other:?}"),
                    })
                }
            })
            .await;

        assert_eq!(result.ranges().len(), 2);
        assert_eq!(first_page_attempts.load(Ordering::SeqCst), 1);
        assert_eq!(second_page_attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn empty_routing_map_is_not_cached() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();
        let empty_fetch = move |_container: ContainerReference, _continuation: Option<String>| {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Some(PkRangeFetchResult {
                    ranges: vec![],
                    continuation: None,
                    not_modified: true,
                })
            }
        };

        assert!(cache
            .try_lookup(&container, false, empty_fetch.clone())
            .await
            .is_none());
        assert!(cache
            .try_lookup(&container, false, empty_fetch)
            .await
            .is_none());
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn force_refresh_with_transient_fetch_failure_preserves_previous_map() {
        // A force-refresh that hits a transient fetch failure on iteration 0
        // must NOT regress the cached routing map to empty: the cached map is
        // the only thing keeping routing decisions working until the next
        // successful refresh.
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        let cache = PartitionKeyRangeCache::new();
        let container = make_container(r#"{"paths":["/pk"],"version":2}"#);

        // Seed the cache with two ranges via a successful initial lookup.
        let seeded = cache
            .try_lookup(&container, false, two_range_fetch)
            .await
            .unwrap();
        assert_eq!(seeded.ranges().len(), 2);

        // Force-refresh with a fetcher that fails on the first call.
        let call_count = Arc::new(AtomicUsize::new(0));
        let count = call_count.clone();
        let failing_fetch = move |_container: ContainerReference, _continuation: Option<String>| {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                None
            }
        };

        let refreshed = cache
            .try_lookup(&container, true, failing_fetch)
            .await
            .unwrap();

        assert_eq!(
            call_count.load(Ordering::SeqCst),
            1,
            "failing fetcher should be invoked exactly once before fallback"
        );
        assert_eq!(
            refreshed.ranges().len(),
            2,
            "force-refresh on transient failure must preserve the previously cached map"
        );
        assert_eq!(refreshed.ranges()[0].id, "0");
        assert_eq!(refreshed.ranges()[1].id, "1");

        // The cached entry observed by a subsequent non-refresh lookup must
        // also be the preserved map, not the empty placeholder that would
        // result from the previous bug.
        let after = cache
            .try_lookup(&container, false, |_, _| async {
                panic!("non-refresh lookup must hit cache, not call fetcher")
            })
            .await
            .unwrap();
        assert_eq!(after.ranges().len(), 2);
    }
}
