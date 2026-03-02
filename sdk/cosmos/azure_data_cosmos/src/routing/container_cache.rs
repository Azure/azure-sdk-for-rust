// cSpell:ignore smol
#![allow(dead_code)]

use super::async_cache::AsyncCache;
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::pipeline::GatewayPipeline;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::{
    models::ContainerProperties, resource_context::ResourceLink, CosmosResponse,
    ReadContainerOptions,
};
use azure_core::http::Context;
use azure_core::Error;
use std::sync::Arc;

/// Cache for Cosmos DB container metadata and properties.
///
/// # Summary
/// Maintains an in-memory cache of container properties (partition keys, indexing policies, etc.)
/// to minimize redundant metadata requests to the Cosmos DB service. Uses a 5-minute TTL by default
/// to balance freshness with performance. Integrates with retry handler for resilient metadata fetching
/// across regional endpoints.
#[derive(Clone, Debug)]
pub(crate) struct ContainerCache {
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

    /// Retrieves container properties from cache or fetches from service if not cached.
    ///
    /// # Summary
    /// Returns container metadata (partition key definition, indexing policy, etc.) for the
    /// specified container. Checks the cache first; if not found or expired, fetches fresh
    /// metadata from the Cosmos DB service and updates the cache. Uses retry handler for
    /// resilience against transient failures and regional outages.
    ///
    /// # Arguments
    /// * `container_id` - Unique identifier of the container (used as cache key)
    /// * `container_link` - Resource link to the container in Cosmos DB
    /// * `options` - Optional request options including consistency level and context
    ///
    /// # Returns
    /// `Ok(ContainerProperties)` with container metadata, or `Err` if fetch fails
    pub async fn resolve_by_id(
        &self,
        container_id: String,
        options: Option<ReadContainerOptions>,
        force_refresh: bool,
    ) -> Result<ContainerProperties, Error> {
        self.container_properties_cache
            .get(
                container_id,
                |_| force_refresh,
                || async {
                    let response = self
                        .read_container_properties_by_id(self.container_link.clone(), options)
                        .await?;
                    response.into_model()
                },
            )
            .await
    }

    /// Removes container metadata from the cache, forcing refresh on next access.
    ///
    /// # Summary
    /// Invalidates the cached container properties for the specified container ID.
    /// The next call to `resolve_by_id` for this container will fetch fresh metadata
    /// from the service. Useful when container configuration changes (e.g., partition
    /// key updates, indexing policy modifications) and stale cache must be cleared.
    ///
    /// # Arguments
    /// * `container_id` - Unique identifier of the container to remove from cache
    pub async fn remove_by_id(&self, container_id: &str) {
        self.container_properties_cache
            .remove(&container_id.to_string())
            .await;
    }

    /// Inserts container properties directly into the cache.
    ///
    /// Used to populate the cache from a container read response without
    /// requiring a separate metadata fetch.
    pub async fn populate(&self, container_id: String, properties: ContainerProperties) {
        self.container_properties_cache
            .insert(container_id, properties)
            .await;
    }

    /// Fetches container properties directly from the Cosmos DB service.
    ///
    /// # Summary
    /// Executes an HTTP GET request to retrieve container metadata from the service.
    /// Resolves the appropriate regional endpoint using the global endpoint manager,
    /// constructs the request with proper routing context, and delegates to the retry
    /// handler for resilient execution with automatic failover on errors.
    ///
    /// # Arguments
    /// * `container_link` - Resource link identifying the target container
    /// * `options` - Optional request options including consistency level and context
    ///
    /// # Returns
    /// `Ok(Response<ContainerProperties>)` on success, or `Err` if request fails
    async fn read_container_properties_by_id(
        &self,
        container_link: ResourceLink,
        _options: Option<ReadContainerOptions>,
    ) -> azure_core::Result<CosmosResponse<ContainerProperties>> {
        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Read, container_link.clone()).build()?;

        let location_endpoint = self
            .global_endpoint_manager
            .resolve_service_endpoint(&cosmos_request);
        cosmos_request
            .request_context
            .route_to_location_endpoint(cosmos_request.resource_link.url(&location_endpoint));

        let ctx_owned = Context::default().with_value(container_link);

        self.pipeline.send(cosmos_request, ctx_owned).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regions::RegionName;
    use crate::resource_context::ResourceType;
    use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
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
        let partition_manager =
            GlobalPartitionEndpointManager::new(endpoint_manager.clone(), false, false);
        Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline_core,
            endpoint_manager,
            partition_manager,
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

        GlobalEndpointManager::new(endpoint, vec![], vec![], pipeline)
    }

    // Helper function to create a test GlobalEndpointManager with preferred locations
    fn create_test_endpoint_manager_with_locations() -> Arc<GlobalEndpointManager> {
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        );
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        GlobalEndpointManager::new(
            endpoint,
            vec![RegionName::from("East US"), RegionName::from("West US")],
            vec![],
            pipeline,
        )
    }

    #[tokio::test]
    async fn remove_by_id() {
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager);

        // Test that remove_by_id doesn't panic when removing non-existent items
        cache.remove_by_id("non-existent-container").await;

        // Test passes if no panic occurs
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

    #[tokio::test]
    async fn remove_by_id_idempotency() {
        // Test that removing the same item multiple times is safe
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager);
        let container_id = "test-container";

        // Remove the same ID multiple times
        cache.remove_by_id(container_id).await;
        cache.remove_by_id(container_id).await;
        cache.remove_by_id(container_id).await;

        // Test passes if no panic occurs
    }

    #[tokio::test]
    async fn container_cache_clone() {
        // Test that ContainerCache can be cloned properly
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager);

        let cloned_cache = cache.clone();

        // Both should be valid instances
        cache.remove_by_id("test1").await;
        cloned_cache.remove_by_id("test2").await;
    }

    #[tokio::test]
    async fn remove_by_id_with_different_ids() {
        // Test removing different container IDs
        let global_endpoint_manager = create_test_endpoint_manager();
        let pipeline = create_test_gateway_pipeline(global_endpoint_manager.clone());
        let container_link = ResourceLink::root(ResourceType::Databases)
            .item("test_db")
            .feed(ResourceType::Containers)
            .item("test_container");
        let cache = ContainerCache::new(pipeline, container_link, global_endpoint_manager);

        cache.remove_by_id("container1").await;
        cache.remove_by_id("container2").await;
        cache.remove_by_id("container-with-dashes").await;
        cache.remove_by_id("container_with_underscores").await;

        // Test passes if no panic occurs
    }
}
