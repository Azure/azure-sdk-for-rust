// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use crate::cosmos_request::CosmosRequest;
use crate::models::{ContainerReference, CosmosResponse};
use crate::pipeline::GatewayPipeline;
use crate::resource_context::ResourceType;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;
use azure_core::http::Context;
use std::sync::Arc;

/// Handler for managing transport-level operations with Cosmos DB.
#[derive(Debug, Clone)]
pub(crate) struct ContainerConnection {
    pipeline: Arc<GatewayPipeline>,
    pk_range_cache: Arc<PartitionKeyRangeCache>,
    global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    container_ref: ContainerReference,
}

impl ContainerConnection {
    /// Creates a new [`ContainerConnection`] with the specified dependencies.
    ///
    /// # Arguments
    ///
    /// * `pipeline` - The Cosmos gateway pipeline to use for sending requests.
    /// * `pk_range_cache` - The cache used to resolve partition key ranges.
    pub(crate) fn new(
        pipeline: Arc<GatewayPipeline>,
        pk_range_cache: Arc<PartitionKeyRangeCache>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
        container_ref: ContainerReference,
    ) -> Self {
        Self {
            pipeline,
            pk_range_cache,
            global_partition_endpoint_manager,
            container_ref,
        }
    }

    /// Returns the partition key definition from the eagerly-resolved container reference.
    pub(crate) fn partition_key_definition(
        &self,
    ) -> &azure_data_cosmos_driver::models::PartitionKeyDefinition {
        self.container_ref.partition_key_definition()
    }

    /// Resolves the routing map for this container.
    pub(crate) async fn resolve_routing_map(
        &self,
        force_refresh: bool,
    ) -> Result<
        Option<crate::routing::collection_routing_map::CollectionRoutingMap>,
        azure_core::Error,
    > {
        let collection_rid = self.container_ref.rid();
        let collection_name = self.container_ref.name();
        let routing_map = self
            .pk_range_cache
            .try_lookup(collection_name, collection_rid, None)
            .await?;

        if force_refresh {
            if let Some(previous) = routing_map {
                return self
                    .pk_range_cache
                    .try_lookup(collection_name, collection_rid, Some(previous))
                    .await;
            }
        }

        Ok(routing_map)
    }

    pub async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        if self
            .global_partition_endpoint_manager
            .partition_level_failover_enabled()
            && (cosmos_request.resource_type.is_partitioned()
                || cosmos_request.resource_type == ResourceType::StoredProcedures)
        {
            let pk_def = self.container_ref.partition_key_definition();
            let collection_name = self.container_ref.name();
            let collection_rid = self.container_ref.rid();

            if let Some(pk_range) = cosmos_request.partition_key_range_identity.as_ref() {
                if !pk_range.collection_rid.is_empty() {
                    if let Some(resolved) = self
                        .pk_range_cache
                        .resolve_partition_key_range_by_id(
                            collection_name,
                            &pk_range.collection_rid,
                            &pk_range.partition_key_range_id,
                            false,
                        )
                        .await
                    {
                        cosmos_request.request_context.resolved_partition_key_range =
                            Some(resolved.clone());
                    }
                }
            } else if let Some(partition_key) = cosmos_request.partition_key.as_ref() {
                let routing_map = self
                    .pk_range_cache
                    .try_lookup(collection_name, collection_rid, None)
                    .await?;

                if let Some(routing_map) = routing_map {
                    // Use a safe default version (2) when the service omits the version field,
                    // since get_hashed_partition_key_string only supports version 1 or 2.
                    // PartitionKeyVersion guarantees values 1 or 2; see driver's enum definition.
                    let pk_version = pk_def.version().value() as u8;
                    let epk =
                        partition_key.get_hashed_partition_key_string(pk_def.kind(), pk_version);

                    // First attempt to resolve the partition key range from the
                    // current routing map. If it succeeds, clone immediately so
                    // we release the borrow on routing_map before possibly moving
                    // it into try_lookup for a refresh.
                    match routing_map.get_range_by_effective_partition_key(epk.as_str()) {
                        Ok(pk_range) => {
                            cosmos_request.request_context.resolved_partition_key_range =
                                Some(pk_range.clone());
                        }
                        Err(_) => {
                            // Refresh the routing map and retry.
                            let refreshed_routing_map = self
                                .pk_range_cache
                                .try_lookup(collection_name, collection_rid, Some(routing_map))
                                .await?;

                            if let Some(refreshed_routing_map) = refreshed_routing_map {
                                let pk_range = refreshed_routing_map
                                    .get_range_by_effective_partition_key(epk.as_str())?;
                                cosmos_request.request_context.resolved_partition_key_range =
                                    Some(pk_range.clone());
                            }
                        }
                    }
                }
            }

            cosmos_request.request_context.resolved_collection_rid =
                Some(collection_rid.to_string());
        }

        // Delegate to the retry handler, providing the sender callback
        self.pipeline.send(cosmos_request, context).await
    }
}

#[cfg(test)]
mod tests {
    use crate::cosmos_request::CosmosRequest;
    use crate::operation_context::OperationType;
    use crate::resource_context::{ResourceLink, ResourceType};

    // Helper function to create a test CosmosRequest
    fn create_cosmos_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container")
            .feed(ResourceType::Documents)
            .item("test_doc");
        CosmosRequest::builder(OperationType::Read, resource_link)
            .build()
            .expect("Failed to create CosmosRequest")
    }

    #[test]
    fn cosmos_request_builder_creates_valid_request() {
        let request = create_cosmos_request();
        assert_eq!(request.operation_type, OperationType::Read);
        assert_eq!(request.resource_type, ResourceType::Documents);
    }

    #[test]
    fn cosmos_request_for_different_operations() {
        // Test Read operation
        let read_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container")
            .feed(ResourceType::Documents)
            .item("test_doc");
        let read_request = CosmosRequest::builder(OperationType::Read, read_link)
            .build()
            .unwrap();
        assert_eq!(read_request.operation_type, OperationType::Read);
        assert!(read_request.is_read_only_request());

        // Test Create operation
        let create_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container")
            .feed(ResourceType::Documents);
        let create_request = CosmosRequest::builder(OperationType::Create, create_link)
            .build()
            .unwrap();
        assert_eq!(create_request.operation_type, OperationType::Create);
        assert!(!create_request.is_read_only_request());

        // Test Delete operation
        let delete_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container")
            .feed(ResourceType::Documents)
            .item("doc_to_delete");
        let delete_request = CosmosRequest::builder(OperationType::Delete, delete_link)
            .build()
            .unwrap();
        assert_eq!(delete_request.operation_type, OperationType::Delete);
        assert!(!delete_request.is_read_only_request());

        // Test Query operation
        let query_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container")
            .feed(ResourceType::Documents);
        let query_request = CosmosRequest::builder(OperationType::Query, query_link)
            .build()
            .unwrap();
        assert_eq!(query_request.operation_type, OperationType::Query);
        assert!(query_request.is_read_only_request());
    }
}
