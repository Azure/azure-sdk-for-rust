// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use crate::constants::{A_IM, IF_NONE_MATCH, MAX_ITEM_COUNT};
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use tracing::info;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use azure_core::Error;
use azure_core::http::{Response, StatusCode};
use azure_core::http::headers::HeaderName;
use serde::Deserialize;
use crate::ReadDatabaseOptions;
use crate::pipeline::CosmosPipeline;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::routing::async_cache::AsyncCache;
use crate::routing::container_cache::ContainerCache;
use crate::routing::collection_routing_map::CollectionRoutingMap;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::partition_key_range::PartitionKeyRange;
use crate::routing::range::Range;
use crate::routing::service_identity::ServiceIdentity;

const PAGE_SIZE_STRING: &str = "-1";

#[async_trait]
pub trait RoutingMapProvider: Send + Sync {
    async fn try_get_partition_key_range_by_id(
        &self,
        collection_resource_id: &str,
        partition_key_range_id: &str,
        force_refresh: bool,
    ) -> Response<Option<PartitionKeyRange>>;
}

#[async_trait]
pub trait CollectionRoutingMapCache: Send + Sync {
    async fn try_lookup(
        &self,
        collection_rid: &str,
        previous_value: Option<Arc<CollectionRoutingMap>>,
        request: Option<Arc<CosmosRequest>>,
    ) -> Response<Option<Arc<CollectionRoutingMap>>>;
}

#[derive(Clone, Debug)]
pub struct PartitionKeyRangeCache {
    routing_map_cache: AsyncCache<String, CollectionRoutingMap>,
    pipeline: Arc<CosmosPipeline>,
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
        pipeline: Arc<CosmosPipeline>,
        database_link: ResourceLink,
        container_cache: Arc<ContainerCache>,
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Self {
        let routing_map_cache = AsyncCache::new(
            Duration::from_secs(300), // Default 5 minutes TTL
        );
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

        let mut routing_map = self.try_lookup(
            collection_rid,
            None,
        ).await?;

        if force_refresh {
            if let Some(previous) = routing_map.clone() {
                routing_map = self.try_lookup(
                    collection_rid,
                    Some(previous),
                ).await?;
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
        let mut routing_map = self.try_lookup(
            collection_resource_id,
            None,
        ).await.ok()?;

        if force_refresh {
            if let Some(previous) = routing_map.clone() {
                routing_map = self.try_lookup(
                    collection_resource_id,
                    Some(previous),
                ).await.ok()?;
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

    async fn try_lookup(
        &self,
        collection_rid: &str,
        previous_value: Option<CollectionRoutingMap>,
    ) -> Result<Option<CollectionRoutingMap>, Error> {
        // Determine if we need to force refresh based on whether we have a previous value
        let should_refresh = previous_value.is_some();
        let routing_map = self.routing_map_cache
            .get(collection_rid.to_string(), should_refresh, || async {
                let routing_map = self.get_routing_map_for_collection(
                    collection_rid,
                    previous_value.as_ref().map(|v| v.clone()),
                ).await?;
                match routing_map {
                    Some(map) => Ok(map),
                    None => Err(Error::new(
                        azure_core::error::ErrorKind::Other,
                        "Failed to get routing map for collection"
                    ))
                }
            })
            .await;

        Ok(routing_map.ok())
    }

    fn should_force_refresh(
        previous_value: Option<CollectionRoutingMap>,
        current_value: Option<CollectionRoutingMap>,
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
        let mut ranges = Vec::new();
        let mut change_feed_next_if_none_match = previous_routing_map
            .as_ref()
            .and_then(|m| m.change_feed_next_if_none_match.clone());

        let mut last_status_code: StatusCode;

        loop {
            let pk_range_link = self.database_link.feed(ResourceType::Containers).item(collection_rid).feed(ResourceType::PartitionKeyRanges);
            let response = self.execute_partition_key_range_read_change_feed(
                collection_rid,
                pk_range_link,
                change_feed_next_if_none_match,
            ).await?;

            last_status_code = response.status();
            change_feed_next_if_none_match = response.headers().get_optional_string(&HeaderName::from_static("etag"));

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

        let tuples: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> = ranges
            .into_iter()
            .map(|range| (range, None))
            .collect();

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
                String::new(),
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
    ) -> azure_core::Result<Response<()>> {

        let options = ReadDatabaseOptions {
            ..Default::default()
        };
        let builder = CosmosRequest::builder(OperationType::ReadFeed, resource_link.clone());
        let mut cosmos_request = builder
            .resource_id(collection_rid.to_string())
            .header(MAX_ITEM_COUNT.as_str().to_string(), PAGE_SIZE_STRING.to_string())
            .header(A_IM.as_str().to_string(), "Incremental Feed".to_string())
            .build()?;

        if (if_none_match).is_some() {
            cosmos_request.headers.insert(IF_NONE_MATCH.as_str().to_string(), if_none_match.unwrap())
        }

        let endpoint = self
            .endpoint_manager
            .resolve_service_endpoint(&cosmos_request);

        let pk_endpoint = resource_link.url(&endpoint);

        cosmos_request.request_context.location_endpoint_to_route = Some(pk_endpoint);
        let ctx_owned = options
            .method_options
            .context
            .with_value(resource_link)
            .into_owned();

        self.pipeline
            .send(cosmos_request, ctx_owned)
            .await
    }
}