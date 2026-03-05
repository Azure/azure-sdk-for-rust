// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key range cache for Cosmos DB driver.
//!
//! Caches [`CollectionRoutingMap`] instances keyed by collection RID, providing
//! efficient partition key range lookups for routing operations.

use super::AsyncCache;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

/// Represents a partition key range returned by the Cosmos DB service.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct PartitionKeyRange {
    /// The partition key range ID.
    #[serde(rename = "id")]
    pub id: String,

    /// The resource ID of the partition key range.
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// Minimum inclusive effective partition key value.
    #[serde(rename = "minInclusive")]
    pub min_inclusive: String,

    /// Maximum exclusive effective partition key value.
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: String,

    /// Parent partition key range IDs (present after a split).
    #[serde(rename = "parents", default, skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
}

/// A routing map for a collection's partition key ranges.
///
/// Stores partition key ranges in sorted order for efficient lookup by
/// effective partition key value. Built from the service response to a
/// partition key ranges read-feed request.
#[derive(Clone, Debug)]
pub(crate) struct CollectionRoutingMap {
    /// Lookup from partition key range ID to the range.
    range_by_id: HashMap<String, PartitionKeyRange>,

    /// Partition key ranges sorted by `min_inclusive`.
    ordered_ranges: Vec<PartitionKeyRange>,

    /// Collection resource ID this routing map belongs to.
    collection_rid: String,

    /// ETag for change feed continuation.
    pub change_feed_next_if_none_match: Option<String>,
}

/// Minimum inclusive effective partition key value (empty string).
const MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "";
/// Maximum exclusive effective partition key value.
const MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "FF";

impl CollectionRoutingMap {
    /// Tries to create a complete routing map from a set of partition key ranges.
    ///
    /// Returns `Ok(None)` if the ranges do not cover the full key space.
    pub fn try_create_complete(
        ranges: Vec<PartitionKeyRange>,
        collection_rid: String,
        change_feed_next_if_none_match: Option<String>,
    ) -> azure_core::Result<Option<Self>> {
        let range_by_id: HashMap<String, PartitionKeyRange> =
            ranges.iter().map(|r| (r.id.clone(), r.clone())).collect();

        let mut ordered_ranges = ranges;
        ordered_ranges.sort_by(|a, b| a.min_inclusive.cmp(&b.min_inclusive));

        if !Self::is_complete_set(&ordered_ranges) {
            return Ok(None);
        }

        Ok(Some(Self {
            range_by_id,
            ordered_ranges,
            collection_rid,
            change_feed_next_if_none_match,
        }))
    }

    /// Returns the collection resource ID.
    pub fn collection_rid(&self) -> &str {
        &self.collection_rid
    }

    /// Returns the ordered list of partition key ranges.
    pub fn ordered_ranges(&self) -> &[PartitionKeyRange] {
        &self.ordered_ranges
    }

    /// Gets a partition key range by its ID.
    pub fn get_range_by_id(&self, id: &str) -> Option<&PartitionKeyRange> {
        self.range_by_id.get(id)
    }

    /// Gets the partition key range that contains the given effective partition key value.
    pub fn get_range_by_effective_partition_key(
        &self,
        effective_partition_key: &str,
    ) -> azure_core::Result<&PartitionKeyRange> {
        if self.ordered_ranges.is_empty() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "no partition key ranges available",
            ));
        }

        if effective_partition_key == MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY {
            return Ok(&self.ordered_ranges[0]);
        }

        // Binary search for the range whose min_inclusive <= effective_partition_key
        let index = match self
            .ordered_ranges
            .binary_search_by(|probe| probe.min_inclusive.as_str().cmp(effective_partition_key))
        {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };

        Ok(&self.ordered_ranges[index])
    }

    /// Validates that the ordered ranges form a complete set covering the entire key space.
    fn is_complete_set(ordered_ranges: &[PartitionKeyRange]) -> bool {
        if ordered_ranges.is_empty() {
            return false;
        }

        let first = &ordered_ranges[0];
        let last = &ordered_ranges[ordered_ranges.len() - 1];

        if first.min_inclusive != MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY
            || last.max_exclusive != MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY
        {
            return false;
        }

        for i in 1..ordered_ranges.len() {
            if ordered_ranges[i - 1].max_exclusive != ordered_ranges[i].min_inclusive {
                return false;
            }
        }

        true
    }
}

/// Cache for partition key range routing maps.
///
/// Stores [`CollectionRoutingMap`] instances keyed by collection RID.
/// Follows the same error-handling pattern as [`super::AccountMetadataCache`]:
/// errors are propagated without caching, so the next call retries.
#[derive(Debug)]
pub(crate) struct PartitionKeyRangeCache {
    cache: AsyncCache<String, CollectionRoutingMap>,
}

impl PartitionKeyRangeCache {
    /// Creates a new empty partition key range cache.
    pub(crate) fn new() -> Self {
        Self {
            cache: AsyncCache::new(),
        }
    }

    /// Gets a routing map from cache, or fetches and caches it.
    ///
    /// If the fetch fails, the error is propagated and nothing is cached.
    pub(crate) async fn get_or_fetch<F, Fut>(
        &self,
        collection_rid: String,
        fetch_fn: F,
    ) -> azure_core::Result<Arc<CollectionRoutingMap>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = azure_core::Result<CollectionRoutingMap>>,
    {
        // Fast path: return cached value.
        if let Some(cached) = self.cache.get(&collection_rid).await {
            return Ok(cached);
        }

        // Fetch from the service.
        let routing_map = fetch_fn().await?;

        // Cache the successfully fetched routing map.
        Ok(self
            .cache
            .get_or_insert_with(collection_rid, || async { routing_map })
            .await)
    }

    /// Invalidates a cached routing map for a collection.
    pub(crate) async fn invalidate(
        &self,
        collection_rid: &str,
    ) -> Option<Arc<CollectionRoutingMap>> {
        self.cache.invalidate(&collection_rid.to_owned()).await
    }
}

impl Default for PartitionKeyRangeCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_range(id: &str, min: &str, max: &str) -> PartitionKeyRange {
        PartitionKeyRange {
            id: id.to_string(),
            resource_id: Some(format!("rid_{id}")),
            min_inclusive: min.to_string(),
            max_exclusive: max.to_string(),
            parents: None,
        }
    }

    #[test]
    fn create_complete_routing_map() {
        let ranges = vec![
            create_test_range("0", "", "AA"),
            create_test_range("1", "AA", "FF"),
        ];

        let map = CollectionRoutingMap::try_create_complete(
            ranges,
            "col1".to_string(),
            Some("etag1".to_string()),
        )
        .unwrap();

        assert!(map.is_some());
        let map = map.unwrap();
        assert_eq!(map.ordered_ranges().len(), 2);
        assert_eq!(map.collection_rid(), "col1");
    }

    #[test]
    fn incomplete_ranges_return_none() {
        let ranges = vec![create_test_range("0", "", "AA")];

        let map =
            CollectionRoutingMap::try_create_complete(ranges, "col1".to_string(), None).unwrap();

        assert!(map.is_none());
    }

    #[test]
    fn get_range_by_effective_partition_key() {
        let ranges = vec![
            create_test_range("0", "", "33"),
            create_test_range("1", "33", "66"),
            create_test_range("2", "66", "FF"),
        ];

        let map = CollectionRoutingMap::try_create_complete(ranges, "col1".to_string(), None)
            .unwrap()
            .unwrap();

        let range = map.get_range_by_effective_partition_key("").unwrap();
        assert_eq!(range.id, "0");

        let range = map.get_range_by_effective_partition_key("33").unwrap();
        assert_eq!(range.id, "1");

        let range = map.get_range_by_effective_partition_key("50").unwrap();
        assert_eq!(range.id, "1");

        let range = map.get_range_by_effective_partition_key("66").unwrap();
        assert_eq!(range.id, "2");
    }

    #[test]
    fn get_range_by_id() {
        let ranges = vec![
            create_test_range("0", "", "50"),
            create_test_range("1", "50", "FF"),
        ];

        let map = CollectionRoutingMap::try_create_complete(ranges, "col1".to_string(), None)
            .unwrap()
            .unwrap();

        assert!(map.get_range_by_id("0").is_some());
        assert!(map.get_range_by_id("1").is_some());
        assert!(map.get_range_by_id("999").is_none());
    }

    #[tokio::test]
    async fn cache_returns_cached_value() {
        let cache = PartitionKeyRangeCache::new();
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let counter_clone = counter.clone();
        let result = cache
            .get_or_fetch("rid1".to_string(), || async move {
                counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Ok(CollectionRoutingMap::try_create_complete(
                    vec![
                        create_test_range("0", "", "50"),
                        create_test_range("1", "50", "FF"),
                    ],
                    "rid1".to_string(),
                    None,
                )
                .unwrap()
                .unwrap())
            })
            .await
            .unwrap();

        assert_eq!(result.ordered_ranges().len(), 2);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);

        // Second call should use cache
        let counter_clone = counter.clone();
        cache
            .get_or_fetch("rid1".to_string(), || async move {
                counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                panic!("should not be called");
            })
            .await
            .unwrap();

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn invalidate_removes_entry() {
        let cache = PartitionKeyRangeCache::new();

        cache
            .get_or_fetch("rid1".to_string(), || async {
                Ok(CollectionRoutingMap::try_create_complete(
                    vec![
                        create_test_range("0", "", "50"),
                        create_test_range("1", "50", "FF"),
                    ],
                    "rid1".to_string(),
                    None,
                )
                .unwrap()
                .unwrap())
            })
            .await
            .unwrap();

        let removed = cache.invalidate("rid1").await;
        assert!(removed.is_some());

        // Should be gone from cache
        assert!(cache.cache.get(&"rid1".to_string()).await.is_none());
    }
}
