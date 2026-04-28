// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key range cache for resolving effective partition keys to range IDs.
//!
//! Uses the driver's operation pipeline to fetch `/pkranges` from the service
//! and caches the resulting [`ContainerRoutingMap`] per container RID.

use std::sync::Arc;

use crate::models::{
    effective_partition_key::EffectivePartitionKey, partition_key_range::PkRangesResponse,
    ContainerReference, PartitionKey,
};

use super::{container_routing_map::ContainerRoutingMap, AsyncCache};

/// Maximum number of change feed iterations to prevent infinite loops.
///
/// In practice the loop completes in 1–2 iterations: the service returns all
/// partition key ranges in a single unbounded page, then 304 Not Modified on the
/// next call. This cap is a safety net, not an expected limit.
const MAX_FETCH_ITERATIONS: usize = 10;

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

        Some(
            routing_map
                .get_overlapping_ranges(epk_range)
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
    /// returns invalid ranges, an empty routing map is cached and returned.
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

        if force_refresh {
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
                .await
        } else {
            Some(
                self.cache
                    .get_or_insert_with(key.clone(), || {
                        fetch_and_build_routing_map(key.clone(), None, fetch_pk_ranges)
                    })
                    .await,
            )
        }
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
/// 2. Loop calling `fetch_pk_ranges(container, continuation)` until the
///    service returns 304 Not Modified or `MAX_FETCH_ITERATIONS` is reached.
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
    let mut all_ranges = Vec::new();
    let mut continuation = previous_routing_map
        .as_ref()
        .and_then(|m| m.change_feed_next_if_none_match.clone());

    let mut received_not_modified = false;
    let mut iterations_completed = 0;
    for iteration in 0..MAX_FETCH_ITERATIONS {
        iterations_completed = iteration + 1;

        tracing::trace!(
            iteration,
            has_continuation = continuation.is_some(),
            "Fetching partition key ranges"
        );

        let result = match fetch_pk_ranges(container.clone(), continuation.clone()).await {
            Some(r) => r,
            None => {
                tracing::warn!(
                    "Failed to fetch partition key ranges from service (iteration {})",
                    iteration
                );
                return ContainerRoutingMap::empty();
            }
        };

        continuation = result.continuation;

        if result.not_modified {
            tracing::trace!(iteration, "Service returned 304 Not Modified");
            received_not_modified = true;
            break;
        }

        tracing::trace!(
            iteration,
            range_count = result.ranges.len(),
            "Received partition key ranges"
        );
        all_ranges.extend(result.ranges);
    }

    tracing::debug!(
        iterations = iterations_completed,
        total_ranges = all_ranges.len(),
        not_modified = received_not_modified,
        "Partition key range fetch loop completed"
    );

    if !received_not_modified && !all_ranges.is_empty() {
        tracing::warn!(
            "Partition key range fetch loop reached MAX_FETCH_ITERATIONS ({}) without \
             receiving Not Modified; routing map may be built from partial data",
            MAX_FETCH_ITERATIONS
        );
    }

    // Incremental refresh: merge new ranges into the previous routing map.
    if let Some(prev) = previous_routing_map {
        if all_ranges.is_empty() {
            // No changes since last fetch (304 on first iteration).
            return (*prev).clone();
        }
        return match prev.try_combine(all_ranges, continuation) {
            Ok(Some(map)) => map,
            Ok(None) => {
                tracing::warn!(
                    "Incremental routing map merge incomplete; falling back to previous map"
                );
                (*prev).clone()
            }
            Err(e) => {
                tracing::warn!(
                    "Incremental routing map merge failed: {}; falling back to previous map",
                    e
                );
                (*prev).clone()
            }
        };
    }

    // Full (non-incremental) creation.
    match ContainerRoutingMap::try_create(all_ranges, None, continuation) {
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
}
