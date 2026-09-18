// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Topology provider adapter backed by the partition key range cache.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use futures::future::BoxFuture;

use crate::{
    driver::cache::{PartitionKeyRangeCache, PkRangeFetchResult},
    models::{ContainerReference, FeedRange},
};

use super::{PartitionRoutingRefresh, ResolvedRange, TopologyProvider};

/// Shares typed fetch failures among callers coalesced by the routing cache.
///
/// The key deliberately matches the cache's physical-container identity. The
/// final active caller removes the entry, so stale generations do not retain
/// errors after their in-flight operations finish.
#[derive(Clone, Debug, Default)]
pub(crate) struct TopologyFetchErrors {
    entries: Arc<Mutex<HashMap<ContainerReference, TopologyFetchState>>>,
}

#[derive(Clone, Debug, Default)]
struct TopologyFetchState {
    active_callers: usize,
    error: Option<crate::error::CosmosError>,
}

struct TopologyFetchRegistration {
    errors: TopologyFetchErrors,
    container: ContainerReference,
}

impl TopologyFetchErrors {
    fn register(&self, container: &ContainerReference) -> TopologyFetchRegistration {
        let mut entries = self
            .entries
            .lock()
            .expect("topology fetch error mutex poisoned");
        entries.entry(container.clone()).or_default().active_callers += 1;
        TopologyFetchRegistration {
            errors: self.clone(),
            container: container.clone(),
        }
    }

    fn clear(&self, container: &ContainerReference) {
        if let Some(state) = self
            .entries
            .lock()
            .expect("topology fetch error mutex poisoned")
            .get_mut(container)
        {
            state.error = None;
        }
    }

    fn record(&self, container: &ContainerReference, error: crate::error::CosmosError) {
        self.entries
            .lock()
            .expect("topology fetch error mutex poisoned")
            .entry(container.clone())
            .or_default()
            .error = Some(error);
    }

    fn get(&self, container: &ContainerReference) -> Option<crate::error::CosmosError> {
        self.entries
            .lock()
            .expect("topology fetch error mutex poisoned")
            .get(container)
            .and_then(|state| state.error.clone())
    }
}

impl Drop for TopologyFetchRegistration {
    fn drop(&mut self) {
        let mut entries = self
            .errors
            .entries
            .lock()
            .expect("topology fetch error mutex poisoned");
        let state = entries
            .get_mut(&self.container)
            .expect("topology fetch registration missing");
        state.active_callers -= 1;
        if state.active_callers == 0 {
            entries.remove(&self.container);
        }
    }
}

/// Adapts [`PartitionKeyRangeCache`] to the [`TopologyProvider`] trait.
///
/// Holds a reference to the cache, the container being queried, and a function
/// that fetches partition key ranges from the service. On each
/// [`resolve_ranges`](TopologyProvider::resolve_ranges) call, it uses the
/// provided [`PartitionRoutingRefresh`](super::PartitionRoutingRefresh) to
/// decide whether to refresh the cache first.
///
/// # Type parameters
///
/// * `F` — `Fn(ContainerReference, Option<String>) -> Fut` that fetches
///   pk-ranges from the service. Passed by reference to the cache so the
///   adapter can call it repeatedly without requiring `Clone`.
pub(crate) struct CachedTopologyProvider<'a, F> {
    cache: &'a PartitionKeyRangeCache,
    container: ContainerReference,
    fetch_pk_ranges: F,
    fetch_errors: TopologyFetchErrors,
}

impl<'a, F> CachedTopologyProvider<'a, F> {
    /// Creates a topology provider backed by the partition key range cache.
    #[cfg(test)]
    pub(crate) fn new(
        cache: &'a PartitionKeyRangeCache,
        container: ContainerReference,
        fetch_pk_ranges: F,
    ) -> Self {
        Self::with_fetch_errors(
            cache,
            container,
            fetch_pk_ranges,
            TopologyFetchErrors::default(),
        )
    }

    pub(crate) fn with_fetch_errors(
        cache: &'a PartitionKeyRangeCache,
        container: ContainerReference,
        fetch_pk_ranges: F,
        fetch_errors: TopologyFetchErrors,
    ) -> Self {
        Self {
            cache,
            container,
            fetch_pk_ranges,
            fetch_errors,
        }
    }
}

impl<F, Fut> TopologyProvider for CachedTopologyProvider<'_, F>
where
    F: Fn(ContainerReference, Option<String>) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = crate::error::Result<Option<PkRangeFetchResult>>> + Send,
{
    fn resolve_ranges<'a>(
        &'a mut self,
        range: &'a FeedRange,
        refresh: PartitionRoutingRefresh,
    ) -> BoxFuture<'a, crate::error::Result<Vec<ResolvedRange>>> {
        let force_refresh = matches!(refresh, PartitionRoutingRefresh::ForceRefresh);
        Box::pin(async move {
            let _registration = self.fetch_errors.register(&self.container);
            let fetch_pk_ranges = |container: ContainerReference, continuation: Option<String>| {
                let result = (self.fetch_pk_ranges)(container.clone(), continuation);
                let fetch_errors = self.fetch_errors.clone();
                async move {
                    match result.await {
                        Ok(result) => {
                            fetch_errors.clear(&container);
                            result
                        }
                        Err(error) => {
                            fetch_errors.record(&container, error);
                            None
                        }
                    }
                }
            };
            let pk_ranges = self
                .cache
                .resolve_overlapping_ranges(
                    &self.container,
                    range.min_inclusive()..range.max_exclusive(),
                    force_refresh,
                    fetch_pk_ranges,
                )
                .await;

            let pk_ranges = match pk_ranges {
                Some(ranges) if !ranges.is_empty() => ranges,
                _ => {
                    if let Some(error) = self.fetch_errors.get(&self.container) {
                        return Err(error);
                    }
                    return Err(crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::CLIENT_TOPOLOGY_RESOLUTION_FAILED)
                        .with_message("failed to resolve partition key ranges from topology cache")
                        .build());
                }
            };

            pk_ranges
                .into_iter()
                .map(|pkr| {
                    Ok(ResolvedRange {
                        partition_key_range_id: pkr.id,
                        range: FeedRange::new(pkr.min_inclusive, pkr.max_exclusive)?,
                    })
                })
                .collect::<crate::error::Result<Vec<_>>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        effective_partition_key::EffectivePartitionKey,
        partition_key_range::PartitionKeyRange as PkRange, ContainerProperties,
    };

    fn make_container() -> ContainerReference {
        make_container_with_rid("c_rid")
    }

    fn make_container_with_rid(rid: &'static str) -> ContainerReference {
        let account = crate::models::AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let props = ContainerProperties {
            id: "c".into(),
            partition_key: serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap(),
            system_properties: Default::default(),
        };
        ContainerReference::new(account, "db", "db_rid", "c", rid, &props)
    }

    async fn single_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> crate::error::Result<Option<PkRangeFetchResult>> {
        Ok(if continuation.is_some() {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(PkRangeFetchResult {
                ranges: vec![PkRange::new("0".into(), "", "FF")],
                continuation: Some("etag-1".to_string()),
                not_modified: false,
            })
        })
    }

    async fn two_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> crate::error::Result<Option<PkRangeFetchResult>> {
        Ok(if continuation.is_some() {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(PkRangeFetchResult {
                ranges: vec![
                    PkRange::new("1".into(), "", "80"),
                    PkRange::new("2".into(), "80", "FF"),
                ],
                continuation: Some("etag-2".to_string()),
                not_modified: false,
            })
        })
    }

    async fn three_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> crate::error::Result<Option<PkRangeFetchResult>> {
        Ok(if continuation.is_some() {
            Some(PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(PkRangeFetchResult {
                ranges: vec![
                    PkRange::new("1".into(), "", "40"),
                    PkRange::new("2".into(), "40", "80"),
                    PkRange::new("3".into(), "80", "FF"),
                ],
                continuation: Some("etag-3".to_string()),
                not_modified: false,
            })
        })
    }

    async fn failing_fetch(
        _container: ContainerReference,
        _continuation: Option<String>,
    ) -> crate::error::Result<Option<PkRangeFetchResult>> {
        Ok(None)
    }

    async fn typed_error_fetch(
        _container: ContainerReference,
        _continuation: Option<String>,
    ) -> crate::error::Result<Option<PkRangeFetchResult>> {
        Err(collection_rid_mismatch_error())
    }

    fn collection_rid_mismatch_error() -> crate::error::CosmosError {
        crate::error::CosmosError::builder()
            .with_status(
                crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)
                    .with_sub_status(crate::models::SubStatusCode::COLLECTION_RID_MISMATCH.value()),
            )
            .with_message("container RID mismatch")
            .build()
    }

    #[tokio::test]
    async fn resolves_normalized_point_to_owning_partition_including_boundary() {
        // Issue #4574: an equality / `IN` predicate's closed point `[X, X]` is
        // normalized to the half-open `[X, successor(X))`, which must resolve to
        // exactly its owning partition. Topology is split at "80"; a point
        // inside the right partition and a point exactly at the "80" boundary
        // both belong to partition "2".
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), two_range_fetch);

        let inside_epk = EffectivePartitionKey::from("C0");
        let inside = FeedRange::new(inside_epk.clone(), inside_epk.successor()).unwrap();
        let ranges = provider
            .resolve_ranges(&inside, PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].partition_key_range_id, "2");

        let boundary_epk = EffectivePartitionKey::from("80");
        let at_boundary = FeedRange::new(boundary_epk.clone(), boundary_epk.successor()).unwrap();
        let ranges = provider
            .resolve_ranges(&at_boundary, PartitionRoutingRefresh::UseCached)
            .await
            .unwrap();
        assert_eq!(ranges.len(), 1);
        assert_eq!(
            ranges[0].partition_key_range_id, "2",
            "a point at the inclusive lower bound belongs to that partition"
        );
    }

    #[tokio::test]
    async fn resolves_single_range_for_full_epk_space() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider =
            CachedTopologyProvider::new(&cache, make_container(), single_range_fetch);

        let ranges = provider
            .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap();

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].partition_key_range_id, "0");
        assert_eq!(ranges[0].range.min_inclusive(), &EffectivePartitionKey::MIN);
        assert_eq!(ranges[0].range.max_exclusive(), &EffectivePartitionKey::MAX);
    }

    #[tokio::test]
    async fn resolves_split_ranges() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), two_range_fetch);

        let ranges = provider
            .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap();

        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].partition_key_range_id, "1");
        assert_eq!(ranges[0].range.min_inclusive(), &EffectivePartitionKey::MIN);
        assert_eq!(
            ranges[0].range.max_exclusive(),
            &EffectivePartitionKey::from("80")
        );
        assert_eq!(ranges[1].partition_key_range_id, "2");
        assert_eq!(
            ranges[1].range.min_inclusive(),
            &EffectivePartitionKey::from("80")
        );
        assert_eq!(ranges[1].range.max_exclusive(), &EffectivePartitionKey::MAX);
    }

    #[tokio::test]
    async fn resolves_partial_epk_range() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), two_range_fetch);

        let left_half = FeedRange::new(
            EffectivePartitionKey::MIN.clone(),
            EffectivePartitionKey::from("80"),
        )
        .unwrap();
        let ranges = provider
            .resolve_ranges(&left_half, PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap();

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].partition_key_range_id, "1");
    }

    #[tokio::test]
    async fn resolves_three_way_split() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), three_range_fetch);

        let ranges = provider
            .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap();

        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].partition_key_range_id, "1");
        assert_eq!(ranges[1].partition_key_range_id, "2");
        assert_eq!(ranges[2].partition_key_range_id, "3");
    }

    #[tokio::test]
    async fn returns_error_when_fetch_fails() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), failing_fetch);

        let err = provider
            .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::ForceRefresh)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("failed to resolve partition key ranges from topology cache"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn preserves_typed_fetch_error() {
        let cache = PartitionKeyRangeCache::new();
        let mut provider = CachedTopologyProvider::new(&cache, make_container(), typed_error_fetch);

        let error = provider
            .resolve_ranges(&FeedRange::full(), PartitionRoutingRefresh::ForceRefresh)
            .await
            .expect_err("typed topology fetch errors must be preserved");

        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );
        assert_eq!(
            error.status().sub_status(),
            Some(crate::models::SubStatusCode::COLLECTION_RID_MISMATCH)
        );
    }

    #[tokio::test]
    async fn coalesced_callers_preserve_typed_fetch_error() {
        let cache = PartitionKeyRangeCache::new();
        let container = make_container();
        let fetch_errors = TopologyFetchErrors::default();
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let make_fetch = || {
            let calls = Arc::clone(&calls);
            move |_container: ContainerReference, _continuation: Option<String>| {
                let calls = Arc::clone(&calls);
                async move {
                    calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                    Err(collection_rid_mismatch_error())
                }
            }
        };
        let mut left = CachedTopologyProvider::with_fetch_errors(
            &cache,
            container.clone(),
            make_fetch(),
            fetch_errors.clone(),
        );
        let mut right = CachedTopologyProvider::with_fetch_errors(
            &cache,
            container,
            make_fetch(),
            fetch_errors,
        );

        let range = FeedRange::full();
        let (left_result, right_result) = tokio::join!(
            left.resolve_ranges(&range, PartitionRoutingRefresh::ForceRefresh),
            right.resolve_ranges(&range, PartitionRoutingRefresh::ForceRefresh),
        );

        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
        for error in [left_result.unwrap_err(), right_result.unwrap_err()] {
            assert_eq!(
                error.status().sub_status(),
                Some(crate::models::SubStatusCode::COLLECTION_RID_MISMATCH)
            );
        }
    }

    #[test]
    fn fetch_errors_are_isolated_by_container_generation() {
        let errors = TopologyFetchErrors::default();
        let old = make_container_with_rid("old_rid");
        let replacement = make_container_with_rid("replacement_rid");
        let old_registration = errors.register(&old);
        let replacement_registration = errors.register(&replacement);

        errors.record(&old, collection_rid_mismatch_error());
        errors.clear(&replacement);

        assert_eq!(
            errors.get(&old).unwrap().status().sub_status(),
            Some(crate::models::SubStatusCode::COLLECTION_RID_MISMATCH)
        );
        assert!(errors.get(&replacement).is_none());

        drop(old_registration);
        drop(replacement_registration);
        assert!(errors.entries.lock().unwrap().is_empty());
    }
}
