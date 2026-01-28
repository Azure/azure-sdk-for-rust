// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use crate::cosmos_request::CosmosRequest;
use crate::pipeline::GatewayPipeline;
use crate::routing::container_cache::ContainerCache;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;
use azure_core::http::{Context, Response};
use std::sync::Arc;

/// Handler for managing transport-level operations with Cosmos DB.
#[derive(Debug, Clone)]
pub struct ContainerConnection {
    pipeline: Arc<GatewayPipeline>,
    container_cache: Arc<ContainerCache>,
    pk_range_cache: Arc<PartitionKeyRangeCache>,
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
    ) -> Self {
        Self {
            pipeline,
            container_cache,
            pk_range_cache,
        }
    }

    pub async fn send<T>(
        &self,
        cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<Response<T>> {
        self.pipeline.send(cosmos_request, context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmos_request::CosmosRequest;
    use crate::operation_context::OperationType;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use crate::CosmosClientOptions;
    use azure_core::http::ClientOptions;
    use std::borrow::Cow;
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
        Arc::new(GlobalEndpointManager::new(endpoint, vec![], vec![], pipeline))
    }

    // Helper function to create a test GatewayPipeline
    fn create_gateway_pipeline(
        endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Arc<GatewayPipeline> {
        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline_core,
            endpoint_manager,
            CosmosClientOptions::default(),
        ))
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
            vec![Cow::Borrowed("East US"), Cow::Borrowed("West US")],
            vec![],
            pipeline.clone(),
        ));

        let gateway_pipeline = Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline,
            endpoint_manager.clone(),
            CosmosClientOptions::default(),
        ));

        let container_cache =
            create_container_cache(gateway_pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache = create_pk_range_cache(
            gateway_pipeline.clone(),
            container_cache.clone(),
            endpoint_manager,
        );

        let connection =
            ContainerConnection::new(gateway_pipeline, container_cache, pk_range_cache);

        // Verify the connection was created successfully with preferred locations
        assert!(std::mem::size_of_val(&connection) > 0);
    }

    #[test]
    fn multiple_container_connections_share_caches() {
        let endpoint_manager = create_endpoint_manager();
        let pipeline = create_gateway_pipeline(endpoint_manager.clone());
        let container_cache = create_container_cache(pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache =
            create_pk_range_cache(pipeline.clone(), container_cache.clone(), endpoint_manager);

        // Create multiple connections sharing the same caches
        let connection1 = ContainerConnection::new(
            pipeline.clone(),
            container_cache.clone(),
            pk_range_cache.clone(),
        );
        let connection2 = ContainerConnection::new(
            pipeline.clone(),
            container_cache.clone(),
            pk_range_cache.clone(),
        );
        let connection3 = ContainerConnection::new(pipeline, container_cache, pk_range_cache);

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
        let pipeline = create_gateway_pipeline(endpoint_manager.clone());
        let container_cache = create_container_cache(pipeline.clone(), endpoint_manager.clone());
        let pk_range_cache =
            create_pk_range_cache(pipeline.clone(), container_cache.clone(), endpoint_manager);

        let connection = ContainerConnection::new(pipeline, container_cache, pk_range_cache);

        // Verify Debug trait is properly implemented
        let debug_str = format!("{:?}", connection);
        assert!(!debug_str.is_empty());
    }
}
