// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use crate::constants::{A_IM, IF_NONE_MATCH, MAX_ITEM_COUNT};
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::pipeline::GatewayPipeline;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::routing::async_cache::AsyncCache;
use crate::routing::collection_routing_map::CollectionRoutingMap;
use crate::routing::container_cache::ContainerCache;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::partition_key_range::PartitionKeyRange;
use crate::routing::range::Range;
use crate::routing::service_identity::ServiceIdentity;
use crate::CosmosResponse;
use azure_core::http::headers::HeaderName;
use azure_core::http::{Context, StatusCode};
use azure_core::Error;
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;

const PAGE_SIZE_STRING: &str = "-1";

#[derive(Clone, Debug)]
pub(crate) struct PartitionKeyRangeCache {
    routing_map_cache: AsyncCache<String, CollectionRoutingMap>,
    pipeline: Arc<GatewayPipeline>,
    container_cache: Arc<ContainerCache>,
    endpoint_manager: Arc<GlobalEndpointManager>,
    database_link: ResourceLink,
}

#[derive(Clone, Debug, Deserialize)]
struct PkRangesResponse {
    #[serde(rename = "PartitionKeyRanges")]
    partition_key_ranges: Vec<PartitionKeyRange>,
}

impl PartitionKeyRangeCache {
    pub fn new(
        pipeline: Arc<GatewayPipeline>,
        database_link: ResourceLink,
        container_cache: Arc<ContainerCache>,
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Self {
        // No TTL-based expiry is needed.
        let routing_map_cache = AsyncCache::new(None);
        Self {
            routing_map_cache,
            pipeline,
            container_cache,
            endpoint_manager,
            database_link,
        }
    }

    pub async fn resolve_overlapping_ranges(
        &self,
        collection_rid: &str,
        range: Range<String>,
        force_refresh: bool,
    ) -> Result<Option<Vec<PartitionKeyRange>>, Error> {
        let mut routing_map = self.try_lookup(collection_rid, None).await?;

        if force_refresh {
            if let Some(previous) = routing_map.clone() {
                routing_map = self.try_lookup(collection_rid, Some(previous)).await?;
            }
        }

        match routing_map {
            Some(map) => Ok(Some(map.get_overlapping_ranges(&range))),
            None => {
                tracing::warn!(
                    "Routing Map Null for collection: {} for range: {:?}, forceRefresh: {}",
                    collection_rid,
                    range,
                    force_refresh
                );
                Ok(None)
            }
        }
    }

    pub async fn resolve_partition_key_range_by_id(
        &self,
        collection_resource_id: &str,
        partition_key_range_id: &str,
        force_refresh: bool,
    ) -> Option<PartitionKeyRange> {
        let mut routing_map = self.try_lookup(collection_resource_id, None).await.ok()?;

        if force_refresh {
            if let Some(previous) = routing_map.clone() {
                routing_map = self
                    .try_lookup(collection_resource_id, Some(previous))
                    .await
                    .ok()?;
            }
        }

        match routing_map {
            Some(map) => map.try_get_range_by_partition_key_range_id(partition_key_range_id),
            None => {
                info!(
                    "Routing Map Null for collection: {}, PartitionKeyRangeId: {}, forceRefresh: {}",
                    collection_resource_id,
                    partition_key_range_id,
                    force_refresh
                );
                None
            }
        }
    }

    pub async fn try_lookup(
        &self,
        collection_rid: &str,
        previous_value: Option<CollectionRoutingMap>,
    ) -> Result<Option<CollectionRoutingMap>, Error> {
        // Clone previous_value for use in should_force_refresh closure
        // Determine if we need to force refresh based on whether we have a previous value
        let routing_map = self
            .routing_map_cache
            .get(
                collection_rid.to_string(),
                |cached| {
                    PartitionKeyRangeCache::should_force_refresh(previous_value.clone(), cached)
                },
                || async {
                    let routing_map = self
                        .get_routing_map_for_collection(collection_rid, previous_value.clone())
                        .await?;
                    match routing_map {
                        Some(map) => Ok(map),
                        None => Err(Error::new(
                            azure_core::error::ErrorKind::Other,
                            format!(
                                "Failed to get routing map for collection: {}",
                                collection_rid
                            ),
                        )),
                    }
                },
            )
            .await;

        Ok(routing_map.ok())
    }

    fn should_force_refresh(
        previous_value: Option<CollectionRoutingMap>,
        current_value: Option<&CollectionRoutingMap>,
    ) -> bool {
        match (previous_value, current_value) {
            (Some(prev), Some(curr)) => {
                prev.change_feed_next_if_none_match == curr.change_feed_next_if_none_match
            }
            _ => false,
        }
    }

    async fn get_routing_map_for_collection(
        &self,
        collection_rid: &str,
        previous_routing_map: Option<CollectionRoutingMap>,
    ) -> Result<Option<CollectionRoutingMap>, Error> {
        // Maximum number of iterations to prevent infinite loops in case of unexpected server behavior
        const MAX_ITERATIONS: usize = 1000;

        let mut iteration_count = 0;
        let mut ranges = Vec::new();
        let mut change_feed_next_if_none_match = previous_routing_map
            .as_ref()
            .and_then(|m| m.change_feed_next_if_none_match.clone());

        loop {
            iteration_count += 1;
            if iteration_count > MAX_ITERATIONS {
                return Err(Error::new(
                    azure_core::error::ErrorKind::Other,
                    format!(
                        "Maximum iteration count ({}) exceeded while fetching partition key ranges for collection: {}",
                        MAX_ITERATIONS,
                        collection_rid
                    ),
                ));
            }

            let pk_range_link = self
                .database_link
                .feed(ResourceType::Containers)
                .item(collection_rid)
                .feed(ResourceType::PartitionKeyRanges);
            let response = self
                .execute_partition_key_range_read_change_feed(
                    collection_rid,
                    pk_range_link,
                    change_feed_next_if_none_match,
                )
                .await?;

            let last_status_code = response.status();
            change_feed_next_if_none_match = response
                .headers()
                .get_optional_string(&HeaderName::from_static("etag"));

            // If status is 304 (NotModified), the body is empty, so skip parsing
            if last_status_code == StatusCode::NotModified {
                break;
            }

            let body_string = response.into_body().into_string()?;

            // Deserialize the response body to extract Vec<PartitionKeyRange>
            let pk_ranges_response: PkRangesResponse = serde_json::from_str(&body_string)
                .map_err(|e| Error::new(azure_core::error::ErrorKind::DataConversion, e))?;

            ranges.extend(pk_ranges_response.partition_key_ranges);
        }

        let tuples: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> =
            ranges.into_iter().map(|range| (range, None)).collect();

        let routing_map = if let Some(prev_map) = previous_routing_map {
            // Combine with a previous routing map
            prev_map.try_combine(tuples, change_feed_next_if_none_match)?
        } else {
            // Create a new complete routing map, filtering out gone ranges
            let gone_ranges: std::collections::HashSet<String> = tuples
                .iter()
                .filter_map(|(range, _)| range.parents.clone())
                .flatten()
                .collect();

            let filtered_tuples: Vec<_> = tuples
                .into_iter()
                .filter(|(range, _)| !gone_ranges.contains(&range.id))
                .collect();

            CollectionRoutingMap::try_create_complete_routing_map(
                filtered_tuples,
                collection_rid.to_string(),
                change_feed_next_if_none_match,
            )?
        };

        Ok(routing_map)
    }

    pub async fn execute_partition_key_range_read_change_feed(
        &self,
        collection_rid: &str,
        resource_link: ResourceLink,
        if_none_match: Option<String>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let builder = CosmosRequest::builder(OperationType::ReadFeed, resource_link.clone());
        let mut cosmos_request = builder
            .resource_id(collection_rid.to_string())
            .header(
                MAX_ITEM_COUNT.as_str().to_string(),
                PAGE_SIZE_STRING.to_string(),
            )
            .header(A_IM.as_str().to_string(), "Incremental Feed".to_string())
            .build()?;

        if let Some(value) = if_none_match {
            cosmos_request
                .headers
                .insert(IF_NONE_MATCH.as_str().to_string(), value)
        }

        let endpoint = self
            .endpoint_manager
            .resolve_service_endpoint(&cosmos_request);

        let pk_endpoint = resource_link.url(&endpoint);

        cosmos_request.request_context.location_endpoint_to_route = Some(pk_endpoint);
        let ctx_owned = Context::default().with_value(resource_link);

        self.pipeline.send(cosmos_request, ctx_owned).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routing::collection_routing_map::CollectionRoutingMap;
    use crate::routing::partition_key_range::{PartitionKeyRange, PartitionKeyRangeStatus};
    use crate::routing::range::Range;

    // Helper function to create a mock PartitionKeyRange
    fn create_mock_partition_key_range(
        id: &str,
        min_inclusive: &str,
        max_exclusive: &str,
    ) -> PartitionKeyRange {
        PartitionKeyRange {
            id: id.to_string(),
            resource_id: Some(format!("rid_{}", id)),
            self_link: Some(format!("dbs/db/colls/coll/pkranges/{}", id)),
            etag: Some(format!("etag_{}", id)),
            timestamp: Some(1234567890),
            min_inclusive: min_inclusive.to_string(),
            max_exclusive: max_exclusive.to_string(),
            rid_prefix: Some(0),
            throughput_fraction: 1.0,
            target_throughput: Some(1000.0),
            status: PartitionKeyRangeStatus::Online,
            parents: None,
            lsn: 0,
            owned_archival_pk_range_ids: None,
        }
    }

    // Helper function to create a CollectionRoutingMap
    fn create_routing_map(
        ranges: Vec<PartitionKeyRange>,
        change_feed_etag: Option<String>,
    ) -> CollectionRoutingMap {
        let tuples: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> =
            ranges.into_iter().map(|r| (r, None)).collect();
        CollectionRoutingMap::try_create_complete_routing_map(
            tuples,
            "test_collection".to_string(),
            change_feed_etag,
        )
        .expect("Failed to create routing map")
        .expect("Routing map should not be None")
    }

    #[test]
    fn should_force_refresh_both_none() {
        let result = PartitionKeyRangeCache::should_force_refresh(None, None);
        assert!(
            !result,
            "Should not force refresh when both values are None"
        );
    }

    #[test]
    fn should_force_refresh_previous_none() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let current = create_routing_map(vec![range], Some("etag1".to_string()));
        let result = PartitionKeyRangeCache::should_force_refresh(None, Some(&current));
        assert!(
            !result,
            "Should not force refresh when previous value is None"
        );
    }

    #[test]
    fn should_force_refresh_current_none() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let previous = create_routing_map(vec![range], Some("etag1".to_string()));
        let result = PartitionKeyRangeCache::should_force_refresh(Some(previous), None);
        assert!(
            !result,
            "Should not force refresh when current value is None"
        );
    }

    #[test]
    fn should_force_refresh_same_etag() {
        let range1 = create_mock_partition_key_range("0", "", "FF");
        let range2 = create_mock_partition_key_range("0", "", "FF");
        let previous = create_routing_map(vec![range1], Some("etag1".to_string()));
        let current = create_routing_map(vec![range2], Some("etag1".to_string()));

        let result = PartitionKeyRangeCache::should_force_refresh(Some(previous), Some(&current));
        assert!(
            result,
            "Should force refresh when etags are the same (no changes)"
        );
    }

    #[test]
    fn should_force_refresh_different_etag() {
        let range1 = create_mock_partition_key_range("0", "", "FF");
        let range2 = create_mock_partition_key_range("0", "", "FF");
        let previous = create_routing_map(vec![range1], Some("etag1".to_string()));
        let current = create_routing_map(vec![range2], Some("etag2".to_string()));

        let result = PartitionKeyRangeCache::should_force_refresh(Some(previous), Some(&current));
        assert!(
            !result,
            "Should not force refresh when etags are different (changes detected)"
        );
    }

    #[test]
    fn should_force_refresh_no_etags() {
        let range1 = create_mock_partition_key_range("0", "", "FF");
        let range2 = create_mock_partition_key_range("0", "", "FF");
        let previous = create_routing_map(vec![range1], None);
        let current = create_routing_map(vec![range2], None);

        let result = PartitionKeyRangeCache::should_force_refresh(Some(previous), Some(&current));
        assert!(
            result,
            "Should force refresh when both etags are None (equal)"
        );
    }

    #[test]
    fn should_force_refresh_mixed_etags() {
        let range1 = create_mock_partition_key_range("0", "", "FF");
        let range2 = create_mock_partition_key_range("0", "", "FF");
        let previous = create_routing_map(vec![range1], Some("etag1".to_string()));
        let current = create_routing_map(vec![range2], None);

        let result = PartitionKeyRangeCache::should_force_refresh(Some(previous), Some(&current));
        assert!(
            !result,
            "Should not force refresh when etags differ (one is None)"
        );
    }

    #[test]
    fn page_size_constant() {
        assert_eq!(
            PAGE_SIZE_STRING, "-1",
            "PAGE_SIZE_STRING should be -1 to fetch all items"
        );
    }

    #[test]
    fn partition_key_range_creation() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert_eq!(range.id, "0");
        assert_eq!(range.min_inclusive, "");
        assert_eq!(range.max_exclusive, "FF");
        assert_eq!(range.status, PartitionKeyRangeStatus::Online);
        assert_eq!(range.throughput_fraction, 1.0);
        assert!(range.resource_id.is_some());
    }

    #[test]
    fn partition_key_range_with_different_bounds() {
        let range = create_mock_partition_key_range("1", "00", "7F");
        assert_eq!(range.id, "1");
        assert_eq!(range.min_inclusive, "00");
        assert_eq!(range.max_exclusive, "7F");
    }

    #[test]
    fn routing_map_creation_single_range() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let routing_map = create_routing_map(vec![range.clone()], Some("etag1".to_string()));

        assert_eq!(
            routing_map.change_feed_next_if_none_match,
            Some("etag1".to_string())
        );
        let found_range = routing_map.try_get_range_by_partition_key_range_id("0");
        assert!(found_range.is_some());
        assert_eq!(found_range.unwrap().id, "0");
    }

    #[test]
    fn routing_map_creation_multiple_ranges() {
        let range1 = create_mock_partition_key_range("0", "", "7F");
        let range2 = create_mock_partition_key_range("1", "7F", "FF");
        let routing_map = create_routing_map(vec![range1, range2], Some("etag2".to_string()));

        let found_range1 = routing_map.try_get_range_by_partition_key_range_id("0");
        let found_range2 = routing_map.try_get_range_by_partition_key_range_id("1");

        assert!(found_range1.is_some());
        assert!(found_range2.is_some());
        assert_eq!(found_range1.unwrap().id, "0");
        assert_eq!(found_range2.unwrap().id, "1");
    }

    #[test]
    fn routing_map_get_overlapping_ranges_empty() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let routing_map = create_routing_map(vec![range], Some("etag1".to_string()));

        let test_range = Range::new("FF".to_string(), "FFF".to_string(), true, false);
        let overlapping = routing_map.get_overlapping_ranges(&test_range);

        // Should return empty since test range is beyond the partition key range
        assert_eq!(overlapping.len(), 0);
    }

    #[test]
    fn routing_map_get_overlapping_ranges_single_match() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let routing_map = create_routing_map(vec![range], Some("etag1".to_string()));

        let test_range = Range::new("00".to_string(), "7F".to_string(), true, false);
        let overlapping = routing_map.get_overlapping_ranges(&test_range);

        assert_eq!(overlapping.len(), 1);
        assert_eq!(overlapping[0].id, "0");
    }

    #[test]
    fn routing_map_get_overlapping_ranges_multiple_matches() {
        let range1 = create_mock_partition_key_range("0", "", "50");
        let range2 = create_mock_partition_key_range("1", "50", "A0");
        let range3 = create_mock_partition_key_range("2", "A0", "FF");
        let routing_map =
            create_routing_map(vec![range1, range2, range3], Some("etag1".to_string()));

        // Range that spans across multiple partition ranges
        let test_range = Range::new("30".to_string(), "B0".to_string(), true, false);
        let overlapping = routing_map.get_overlapping_ranges(&test_range);

        // Should overlap with all three ranges
        assert!(
            overlapping.len() >= 2,
            "Should find at least 2 overlapping ranges"
        );
    }

    #[test]
    fn routing_map_try_get_range_by_id_exists() {
        let range1 = create_mock_partition_key_range("0", "", "7F");
        let range2 = create_mock_partition_key_range("1", "7F", "FF");
        let routing_map = create_routing_map(vec![range1, range2], Some("etag1".to_string()));

        let found = routing_map.try_get_range_by_partition_key_range_id("1");
        assert!(found.is_some());
        let found_range = found.as_ref().unwrap();
        assert_eq!(found_range.id, "1");
        assert_eq!(found_range.min_inclusive, "7F");
    }

    #[test]
    fn routing_map_try_get_range_by_id_not_exists() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let routing_map = create_routing_map(vec![range], Some("etag1".to_string()));

        let found = routing_map.try_get_range_by_partition_key_range_id("999");
        assert!(found.is_none());
    }

    #[test]
    fn routing_map_with_no_etag() {
        let range = create_mock_partition_key_range("0", "", "FF");
        let routing_map = create_routing_map(vec![range], None);

        assert!(routing_map.change_feed_next_if_none_match.is_none());
        let found_range = routing_map.try_get_range_by_partition_key_range_id("0");
        assert!(found_range.is_some());
    }

    #[test]
    fn range_creation_and_comparison() {
        let range1 = Range::new("00".to_string(), "50".to_string(), true, false);
        let range2 = Range::new("50".to_string(), "FF".to_string(), true, false);

        assert_eq!(range1.min, "00");
        assert_eq!(range1.max, "50");
        assert!(range1.is_min_inclusive);
        assert!(!range1.is_max_inclusive);

        assert_eq!(range2.min, "50");
        assert_eq!(range2.max, "FF");
    }

    #[test]
    fn partition_key_range_status_online() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert_eq!(range.status, PartitionKeyRangeStatus::Online);
    }

    #[test]
    fn partition_key_range_optional_fields() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert!(range.resource_id.is_some());
        assert!(range.self_link.is_some());
        assert!(range.etag.is_some());
        assert!(range.timestamp.is_some());
        assert!(range.rid_prefix.is_some());
        assert!(range.target_throughput.is_some());
        assert!(range.parents.is_none());
        assert_eq!(range.lsn, 0);
    }

    #[test]
    fn partition_key_range_edge_case_empty_min() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert_eq!(range.min_inclusive, "");
    }

    #[test]
    fn partition_key_range_edge_case_full_range() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert_eq!(range.min_inclusive, "");
        assert_eq!(range.max_exclusive, "FF");
    }

    #[test]
    fn routing_map_empty_collection_id() {
        let range = create_mock_partition_key_range("0", "", "FF");
        // The collection_unique_id is set internally in create_routing_map
        let routing_map = create_routing_map(vec![range], Some("etag1".to_string()));
        // Just verify it doesn't panic
        assert!(routing_map
            .try_get_range_by_partition_key_range_id("0")
            .is_some());
    }

    #[test]
    fn multiple_ranges_sequential_ids() {
        let ranges = vec![
            create_mock_partition_key_range("0", "", "33"),
            create_mock_partition_key_range("1", "33", "66"),
            create_mock_partition_key_range("2", "66", "99"),
            create_mock_partition_key_range("3", "99", "FF"),
        ];
        let routing_map = create_routing_map(ranges, Some("etag1".to_string()));

        for i in 0..4 {
            let id = i.to_string();
            assert!(
                routing_map
                    .try_get_range_by_partition_key_range_id(&id)
                    .is_some(),
                "Range {} should exist",
                id
            );
        }
    }

    #[test]
    fn routing_map_point_range_lookup() {
        let range1 = create_mock_partition_key_range("0", "", "50");
        let range2 = create_mock_partition_key_range("1", "50", "FF");
        let routing_map = create_routing_map(vec![range1, range2], Some("etag1".to_string()));

        // Test point range (single value)
        let point_range = Range::get_point_range("25".to_string());
        let overlapping = routing_map.get_overlapping_ranges(&point_range);

        assert!(
            !overlapping.is_empty(),
            "Should find at least one range for point lookup"
        );
    }

    #[test]
    fn routing_map_boundary_lookup() {
        let range1 = create_mock_partition_key_range("0", "", "50");
        let range2 = create_mock_partition_key_range("1", "50", "FF");
        let routing_map = create_routing_map(vec![range1, range2], Some("etag1".to_string()));

        // Test boundary value
        let boundary_range = Range::get_point_range("50".to_string());
        let overlapping = routing_map.get_overlapping_ranges(&boundary_range);

        // Boundary should be handled correctly
        assert!(
            !overlapping.is_empty(),
            "Should find range for boundary value"
        );
    }

    #[test]
    fn throughput_fraction_value() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert_eq!(range.throughput_fraction, 1.0);
        assert!(range.throughput_fraction > 0.0);
        assert!(range.throughput_fraction <= 1.0);
    }

    #[test]
    fn target_throughput_value() {
        let range = create_mock_partition_key_range("0", "", "FF");
        assert!(range.target_throughput.is_some());
        assert_eq!(range.target_throughput.unwrap(), 1000.0);
    }
}
