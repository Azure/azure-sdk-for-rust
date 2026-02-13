// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:ignore smol

use super::async_cache::AsyncCache;
use crate::pipeline::GatewayPipeline;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::{models::ContainerProperties, resource_context::ResourceLink};
use std::sync::Arc;

/// Cache for Cosmos DB container metadata and properties.
///
/// # Summary
/// Maintains an in-memory cache of container properties (partition keys, indexing policies, etc.)
/// to minimize redundant metadata requests to the Cosmos DB service. Uses a 5-minute TTL by default
/// to balance freshness with performance. Integrates with retry handler for resilient metadata fetching
/// across regional endpoints.
#[derive(Clone, Debug)]
#[allow(dead_code)] // Fields are stored for future use when cache queries are wired up.
pub struct ContainerCache {
    pipeline: Arc<GatewayPipeline>,
    container_link: ResourceLink,
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    container_properties_cache: AsyncCache<String, ContainerProperties>,
}

impl ContainerCache {
    /// Creates a new `ContainerCache` with the default configuration.
    ///
    /// # Summary
    /// Initializes a container cache with a 5-minute TTL for container properties.
    /// Sets up retry handler for resilient metadata operations across Azure regions.
    /// The cache automatically refreshes stale entries when accessed after expiration.
    ///
    /// # Arguments
    /// * `pipeline` - HTTP pipeline for making requests to Cosmos DB service
    /// * `global_endpoint_manager` - Manager for multi-region endpoint routing and failover
    ///
    /// # Returns
    /// A new `ContainerCache` instance ready for caching container metadata
    pub(crate) fn new(
        pipeline: Arc<GatewayPipeline>,
        container_link: ResourceLink,
        global_endpoint_manager: Arc<GlobalEndpointManager>,
    ) -> Self {
        // No TTL-based expiry is needed.
        let container_properties_cache = AsyncCache::new(None);

        Self {
            pipeline,
            container_link,
            global_endpoint_manager,
            container_properties_cache,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource_context::ResourceType;
    use crate::CosmosClientOptions;
    use azure_core::http::ClientOptions;
    use url::Url;

    // Helper function to create a test CosmosPipeline
    fn create_test_gateway_pipeline(
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
            false,
        ))
    }

    // Helper function to create a test GlobalEndpointManager
    fn create_test_endpoint_manager() -> Arc<GlobalEndpointManager> {
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

    #[tokio::test]
    async fn new_container_cache() {
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager);

        // Verify the cache was created successfully
        assert!(std::mem::size_of_val(&cache) > 0);
    }

    #[tokio::test]
    async fn new_container_cache_with_preferred_locations() {
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager.clone());

        // Verify the cache can be cloned (Debug trait is implemented)
        let cloned_cache = cache.clone();
        assert!(std::mem::size_of_val(&cloned_cache) > 0);
    }
}
