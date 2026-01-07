// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use std::{
    collections::HashMap,
    sync::RwLock,
};
use tracing::info;
use std::sync::Arc;
use async_trait::async_trait;
use azure_core::Error;
use azure_core::http::{Pipeline, RawResponse, Response, StatusCode};
use azure_core::http::headers::HeaderName;
use serde::Deserialize;
use crate::{ReadContainerOptions, ReadDatabaseOptions};
use crate::pipeline::CosmosPipeline;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::retry_policies::metadata_request_retry_policy::MetadataRequestRetryPolicy;
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
    routing_map_cache: Arc<RwLock<HashMap<String, Arc<CollectionRoutingMap>>>>,
    // authorization_token_provider: Arc<dyn CosmosAuthorizationTokenProvider>,
    pipeline: Arc<CosmosPipeline>,
    container_cache: Arc<ContainerCache>,
    endpoint_manager: Arc<GlobalEndpointManager>,
    database_link: ResourceLink,
}

impl PartitionKeyRangeCache {
    pub fn new(
        // authorization_token_provider: Arc<CosmosAuthorizationTokenProvider>,
        pipeline: Arc<CosmosPipeline>,
        database_link: ResourceLink,
        collection_cache: Arc<ContainerCache>,
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Self {
        Self {
            routing_map_cache: Arc::new(RwLock::new(HashMap::new())),
            // authorization_token_provider,
            pipeline,
            container_cache: collection_cache,
            endpoint_manager,
            database_link,
        }
    }

    pub async fn try_get_overlapping_ranges(
        &self,
        collection_rid: &str,
        range: Range<String>,
        force_refresh: bool,
    ) -> Result<Option<Vec<PartitionKeyRange>>, Error> {
        // let child_trace = trace.start_child(
        //     "Try Get Overlapping Ranges",
        //     TraceComponent::Routing,
        //     TraceLevel::Info,
        // );

        let mut routing_map = self.try_lookup(
            collection_rid,
            None,
        ).await?;

        if force_refresh && routing_map.is_some() {
            let previous = routing_map.clone();
            routing_map = self.try_lookup(
                collection_rid,
                previous,
            ).await?;
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

    pub async fn try_get_partition_key_range_by_id(
        &self,
        collection_resource_id: &str,
        partition_key_range_id: &str,
        force_refresh: bool,
    ) -> Option<PartitionKeyRange> {
        let mut routing_map = self.try_lookup(
            collection_resource_id,
            None,
        ).await.unwrap();

        // if force_refresh && routing_map.is_some() {
        if force_refresh {
            let previous = routing_map.clone();
            routing_map = self.try_lookup(
                collection_resource_id,
                previous,
            ).await.unwrap();
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
        previous_value: Option<Arc<CollectionRoutingMap>>,
    ) -> Result<Option<Arc<CollectionRoutingMap>>, Error> {
        // Check if we need to force refresh
        let should_refresh = {
            let cache = self.routing_map_cache.read().unwrap();
            if let Some(prev) = &previous_value {
                if let Some(current) = cache.get(collection_rid) {
                    Self::should_force_refresh(Some(prev.clone()), Some(current.clone()))
                } else {
                    true
                }
            } else {
                !cache.contains_key(collection_rid)
            }
        };

        if should_refresh {
            // let client_stats = request.as_ref()
            //     .and_then(|r| r.request_context.as_ref())
            //     .and_then(|ctx| ctx.client_request_statistics.clone());

            let routing_map = self.get_routing_map_for_collection(
                collection_rid,
                previous_value.clone(),
                // client_stats,
            ).await?;

            let mut cache = self.routing_map_cache.write().unwrap();
            cache.insert(collection_rid.to_string(), Arc::new(routing_map.unwrap()));
        }

        let cache = self.routing_map_cache.read().unwrap();
        Ok(cache.get(collection_rid).cloned())
    }

    fn should_force_refresh(
        previous_value: Option<Arc<CollectionRoutingMap>>,
        current_value: Option<Arc<CollectionRoutingMap>>,
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
        previous_routing_map: Option<Arc<CollectionRoutingMap>>,
        // client_side_request_statistics: Option<Arc<dyn std::any::Any>>,
    ) -> Result<Option<CollectionRoutingMap>, Error> {
        let mut ranges = Vec::new();
        let mut change_feed_next_if_none_match = previous_routing_map
            .as_ref()
            .and_then(|m| m.change_feed_next_if_none_match.clone());

        // let retry_policy = MetadataRequestRetryPolicy::new(
        //     *self.endpoint_manager.clone()
        // );

        let mut last_status_code = StatusCode::Ok; // HttpStatusCode::OK

        loop {
            let mut headers = HashMap::new();
            headers.insert("x-ms-max-item-count".to_string(), PAGE_SIZE_STRING.to_string());
            headers.insert("a-iam".to_string(), "Incremental feed".to_string());

            if let Some(ref etag) = change_feed_next_if_none_match {
                headers.insert("if-none-match".to_string(), etag.clone());
            }

            let read_container_options = ReadContainerOptions {
                ..Default::default()
            };
            
            // let container_props = self.container_cache.read_properties_by_id(collection_rid, Some(read_container_options)).await?;
            let pk_range_link = self.database_link.feed(ResourceType::Containers).item(collection_rid).feed(ResourceType::PartitionKeyRanges);
            let response = self.execute_partition_key_range_read_change_feed(
                collection_rid,
                pk_range_link,
                // &retry_policy,
            ).await?;

            last_status_code = response.status();
            change_feed_next_if_none_match = response.headers().get_optional_string(&HeaderName::from_static("etag"));

            // Deserialize the response body to extract Vec<PartitionKeyRange>
            if last_status_code == StatusCode::Ok {
                let body_string = response.into_body().into_string()?;
                
                // Cosmos DB wraps partition key ranges in a "PartitionKeyRanges" field
                #[derive(Deserialize)]
                struct PkRangesResponse {
                    #[serde(rename = "PartitionKeyRanges")]
                    partition_key_ranges: Vec<PartitionKeyRange>,
                }
                
                let pk_ranges_response: PkRangesResponse = serde_json::from_str(&body_string)
                    .map_err(|e| Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
                
                ranges.extend(pk_ranges_response.partition_key_ranges);
                break;
            }

            // if last_status_code != StatusCode::Ok { // HttpStatusCode::NotModified
            //     break;
            // }
        }

        let tuples: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> = ranges
            .into_iter()
            .map(|range| (range, None))
            .collect();

        let routing_map = if let Some(prev_map) = previous_routing_map {
            // Combine with previous routing map
            prev_map.try_combine(tuples, change_feed_next_if_none_match)?
        } else {
            // Create new complete routing map, filtering out gone ranges
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
        // retry_policy: &mut MetadataRequestRetryPolicy,
    ) -> azure_core::Result<Response<()>> {

        let options = ReadDatabaseOptions {
            ..Default::default()
        };
        // let resource_link = ResourceLink::root(ResourceType::PartitionKeyRanges);
        let builder = CosmosRequest::builder(OperationType::ReadFeed, resource_link.clone());
        let mut cosmos_request = builder
            .resource_id(collection_rid.to_string())
            .build()?;

        let endpoint = self
            .endpoint_manager
            .resolve_service_endpoint(&cosmos_request);

        // retry_policy.before_send_request(&mut cosmos_request).await;
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