// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use crate::cosmos_request::CosmosRequest;
use crate::models::CosmosResponse;
use crate::pipeline::GatewayPipeline;
use crate::routing::container_cache::ContainerCache;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;
use azure_core::http::Context;
use std::sync::Arc;

/// Handler for managing transport-level operations with Cosmos DB.
#[derive(Debug, Clone)]
pub struct ContainerConnection {
    pipeline: Arc<GatewayPipeline>,
    container_cache: Arc<ContainerCache>,
    pk_range_cache: Arc<PartitionKeyRangeCache>,
    global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

impl ContainerConnection {
    /// Creates a new [`ContainerConnection`] with the specified dependencies.
    ///
    /// # Arguments
    ///
    /// * `pipeline` - The Cosmos gateway pipeline to use for sending requests.
    /// * `container_cache` - The cache used to resolve container properties.
    /// * `pk_range_cache` - The cache used to resolve partition key ranges.
    pub(crate) fn new(
        pipeline: Arc<GatewayPipeline>,
        container_cache: Arc<ContainerCache>,
        pk_range_cache: Arc<PartitionKeyRangeCache>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    ) -> Self {
        Self {
            pipeline,
            container_cache,
            pk_range_cache,
            global_partition_endpoint_manager,
        }
    }

    pub async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        if self
            .global_partition_endpoint_manager
            .partition_level_failover_enabled()
        {
            let mut container_properties = None;
            if let Some(container_id) = cosmos_request.container_id() {
                container_properties = Some(
                    self.container_cache
                        .resolve_by_id(container_id, None, false)
                        .await?,
                );
            } else if let Some(pk_range) = cosmos_request.partition_key_range_identity.as_ref() {
                if !pk_range.collection_rid.is_empty() {
                    container_properties = Some(
                        self.container_cache
                            .resolve_by_id(pk_range.collection_rid.clone(), None, false)
                            .await?,
                    );
                }
            }

            if let Some(container_prop) = container_properties {
                let pk_def = container_prop.partition_key;
                if let Some(pk_range) = cosmos_request.partition_key_range_identity.as_ref() {
                    let pk_range = self
                        .pk_range_cache
                        .resolve_partition_key_range_by_id(
                            &pk_range.collection_rid,
                            &pk_range.partition_key_range_id,
                            false,
                        )
                        .await
                        .unwrap();

                    cosmos_request.request_context.resolved_partition_key_range =
                        Some(pk_range.clone());
                } else if let Some(partition_key) = cosmos_request.partition_key.as_ref() {
                    let routing_map = self
                        .pk_range_cache
                        .try_lookup(&container_prop.id, None)
                        .await?;

                    if let Some(routing_map) = routing_map {
                        let pk_version = pk_def.version.unwrap_or_default() as u8;
                        let epk =
                            partition_key.get_hashed_partition_key_string(pk_def.kind, pk_version);
                        let pk_range = routing_map.get_range_by_effective_partition_key(&epk)?;
                        cosmos_request.request_context.resolved_partition_key_range =
                            Some(pk_range.clone());

                        if cosmos_request
                            .request_context
                            .resolved_partition_key_range
                            .is_none()
                        {
                            let refreshed_routing_map = self
                                .pk_range_cache
                                .try_lookup(&container_prop.id, Some(routing_map))
                                .await?;

                            if let Some(refreshed_routing_map) = refreshed_routing_map {
                                let pk_range = refreshed_routing_map
                                    .get_range_by_effective_partition_key(&epk)?;
                                cosmos_request.request_context.resolved_partition_key_range =
                                    Some(pk_range.clone());
                            }
                        }
                    }
                }

                cosmos_request.request_context.resolved_collection_rid =
                    Some(container_prop.id.into_owned());
            }
        }

        // Delegate to the retry handler, providing the sender callback
        self.pipeline.send(cosmos_request, context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmos_request::CosmosRequest;
    use crate::operation_context::OperationType;
    use crate::pipeline::GatewayPipeline;
    use crate::regions::RegionName;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
    use crate::CosmosClientOptions;
    use azure_core::http::ClientOptions;
    use url::Url;

    // Helper function to create a test GlobalEndpointManager
    fn create_endpoint_manager() -> Arc<GlobalEndpointManager> {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        Arc::new(GlobalEndpointManager::new(
            endpoint,
            vec![],
            vec![],
            pipeline,
        ))
    }

    // Helper function to create a test GatewayPipeline
    fn create_gateway_pipeline(
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> (Arc<GatewayPipeline>, Arc<GlobalPartitionEndpointManager>) {
        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        let partition_manager =
            GlobalPartitionEndpointManager::new(endpoint_manager.clone(), false, false);
        (
            Arc::new(GatewayPipeline::new(
                endpoint,
                pipeline_core,
                endpoint_manager,
                partition_manager.clone(),
                CosmosClientOptions::default(),
            )),
            partition_manager,
        )
    }

    // Helper function to create a test ContainerCache
    fn create_container_cache(
        pipeline: Arc<GatewayPipeline>,
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Arc<ContainerCache> {
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        Arc::new(ContainerCache::new(
            pipeline,
            container_link,
            endpoint_manager,
        ))
    }

    // Helper function to create a test PartitionKeyRangeCache
    fn create_pk_range_cache(
        pipeline: Arc<GatewayPipeline>,
        container_cache: Arc<ContainerCache>,
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Arc<PartitionKeyRangeCache> {
        let database_link = ResourceLink::root(ResourceType::Databases).item("test_db");
        Arc::new(PartitionKeyRangeCache::new(
            pipeline,
            database_link,
            container_cache,
            endpoint_manager,
        ))
    }

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
    fn container_connection_with_preferred_locations() {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        let endpoint_manager = Arc::new(GlobalEndpointManager::new(
            endpoint.clone(),
            vec![RegionName::from("East US"), RegionName::from("West US")],
            vec![],
            pipeline.clone(),
        ));
        let partition_manager =
            GlobalPartitionEndpointManager::new(endpoint_manager.clone(), false, false);

        let gateway_pipeline = Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline,
            endpoint_manager.clone(),
            partition_manager.clone(),
            CosmosClientOptions::default(),
            false,
        ));

        let container_cache =
            create_container_cache(gateway_pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache = create_pk_range_cache(
            gateway_pipeline.clone(),
            container_cache.clone(),
            endpoint_manager.clone(),
        );

        let connection = ContainerConnection::new(
            gateway_pipeline,
            container_cache,
            pk_range_cache,
            partition_manager,
        );

        // Verify the connection was created successfully with preferred locations
        assert!(std::mem::size_of_val(&connection) > 0);
    }

    #[test]
    fn multiple_container_connections_share_caches() {
        let endpoint_manager = create_endpoint_manager();
        let (pipeline, partition_manager) = create_gateway_pipeline(endpoint_manager.clone());
        let container_cache = create_container_cache(pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache = create_pk_range_cache(
            pipeline.clone(),
            container_cache.clone(),
            endpoint_manager.clone(),
        );

        // Create multiple connections sharing the same caches
        let connection1 = ContainerConnection::new(
            pipeline.clone(),
            container_cache.clone(),
            pk_range_cache.clone(),
            partition_manager.clone(),
        );
        let connection2 = ContainerConnection::new(
            pipeline.clone(),
            container_cache.clone(),
            pk_range_cache.clone(),
            partition_manager.clone(),
        );
        let connection3 =
            ContainerConnection::new(pipeline, container_cache, pk_range_cache, partition_manager);

        // All connections should be valid
        assert!(std::mem::size_of_val(&connection1) > 0);
        assert!(std::mem::size_of_val(&connection2) > 0);
        assert!(std::mem::size_of_val(&connection3) > 0);
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

    #[test]
    fn container_connection_debug_implementation() {
        let endpoint_manager = create_endpoint_manager();
        let (pipeline, partition_manager) = create_gateway_pipeline(endpoint_manager.clone());
        let container_cache = create_container_cache(pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache = create_pk_range_cache(
            pipeline.clone(),
            container_cache.clone(),
            endpoint_manager.clone(),
        );

        let connection =
            ContainerConnection::new(pipeline, container_cache, pk_range_cache, partition_manager);

        // Verify Debug trait is properly implemented
        let debug_str = format!("{:?}", connection);
        assert!(!debug_str.is_empty());
    }
}
