// cSpell:ignore smol
#![allow(dead_code)]

use super::async_cache::AsyncCache;
use crate::cosmos_request::CosmosRequest;
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::operation_context::OperationType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::{models::ContainerProperties, resource_context::ResourceLink, ReadContainerOptions};
use azure_core::http::{Pipeline, Response};
use azure_core::Error;
use std::time::Duration;

/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone, Debug)]
pub struct CollectionCache {
    pipeline: Pipeline,
    global_endpoint_manager: GlobalEndpointManager,
    container_properties_cache: AsyncCache<String, ContainerProperties>,
    retry_handler: BackOffRetryHandler,
}

impl CollectionCache {
    pub(crate) fn new(pipeline: Pipeline, global_endpoint_manager: GlobalEndpointManager) -> Self {
        let container_properties_cache = AsyncCache::new(
            Duration::from_secs(300), // Default 5 minutes TTL
        );
        let retry_handler = BackOffRetryHandler::new(global_endpoint_manager.clone());

        Self {
            pipeline,
            global_endpoint_manager,
            container_properties_cache,
            retry_handler,
        }
    }

    /// Gets the container metadata from the cache, or initializes it using the provided async function if not present.
    pub async fn resolve_by_id(
        &self,
        container_id: String,
        container_link: ResourceLink,
        options: Option<ReadContainerOptions<'_>>,
    ) -> Result<ContainerProperties, Error> {
        self.container_properties_cache
            .get(container_id, || async {
                let response = self
                    .read_container_properties_by_id(container_link, options)
                    .await?;
                response.into_model()
            })
            .await
    }

    /// Removes the container metadata from the cache, forcing a refresh on the next access.
    pub async fn remove_by_id(&self, container_id: &str) {
        self.container_properties_cache
            .remove(&container_id.to_string())
            .await;
    }

    async fn read_container_properties_by_id(
        &self,
        container_link: ResourceLink,
        options: Option<ReadContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Read, container_link.clone()).build()?;

        let location_endpoint = Some(
            self.global_endpoint_manager
                .resolve_service_endpoint(&cosmos_request),
        );

        if let Some(ref endpoint) = location_endpoint {
            cosmos_request
                .request_context
                .route_to_location_endpoint(cosmos_request.resource_link.url(endpoint));
        }

        let ctx_owned = options
            .method_options
            .context
            .with_value(container_link)
            .into_owned();

        // Prepare a callback delegate to invoke the http request.
        let sender = move |req: &mut CosmosRequest| {
            let mut raw_req = req.clone().into_raw_request();
            let ctx = ctx_owned.clone();
            async move { self.pipeline.send(&ctx, &mut raw_req, None).await }
        };

        // Delegate to the retry handler, providing the sender callback
        let res = self.retry_handler.send(&mut cosmos_request, sender).await;
        res.map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::ClientOptions;
    use std::borrow::Cow;
    use url::Url;

    // Helper function to create a test pipeline
    fn create_test_pipeline() -> Pipeline {
        azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        )
    }

    // Helper function to create a test GlobalEndpointManager
    fn create_test_endpoint_manager() -> GlobalEndpointManager {
        let pipeline = create_test_pipeline();
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        GlobalEndpointManager::new(endpoint, vec![], pipeline)
    }

    // Helper function to create a test GlobalEndpointManager with preferred locations
    fn create_test_endpoint_manager_with_locations() -> GlobalEndpointManager {
        let pipeline = create_test_pipeline();
        let endpoint = Url::parse("https://test.documents.azure.com").unwrap();
        GlobalEndpointManager::new(
            endpoint,
            vec![Cow::Borrowed("East US"), Cow::Borrowed("West US")],
            pipeline,
        )
    }

    #[tokio::test]
    async fn test_remove_by_id() {
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);

        // Test that remove_by_id doesn't panic when removing non-existent items
        cache.remove_by_id("non-existent-container").await;

        // Test passes if no panic occurs
    }

    #[tokio::test]
    async fn test_new_collection_cache() {
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);

        // Verify the cache was created successfully
        assert!(std::mem::size_of_val(&cache) > 0);
    }

    #[tokio::test]
    async fn test_new_collection_cache_with_preferred_locations() {
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager_with_locations();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);

        // Verify the cache can be cloned (Debug trait is implemented)
        let cloned_cache = cache.clone();
        assert!(std::mem::size_of_val(&cloned_cache) > 0);
    }

    #[tokio::test]
    async fn test_remove_by_id_idempotency() {
        // Test that removing the same item multiple times is safe
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);
        let container_id = "test-container";

        // Remove the same ID multiple times
        cache.remove_by_id(container_id).await;
        cache.remove_by_id(container_id).await;
        cache.remove_by_id(container_id).await;

        // Test passes if no panic occurs
    }

    #[tokio::test]
    async fn test_collection_cache_clone() {
        // Test that CollectionCache can be cloned properly
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);

        let cloned_cache = cache.clone();

        // Both should be valid instances
        cache.remove_by_id("test1").await;
        cloned_cache.remove_by_id("test2").await;
    }

    #[tokio::test]
    async fn test_remove_by_id_with_different_ids() {
        // Test removing different container IDs
        let pipeline = create_test_pipeline();
        let global_endpoint_manager = create_test_endpoint_manager();
        let cache = CollectionCache::new(pipeline, global_endpoint_manager);

        cache.remove_by_id("container1").await;
        cache.remove_by_id("container2").await;
        cache.remove_by_id("container-with-dashes").await;
        cache.remove_by_id("container_with_underscores").await;

        // Test passes if no panic occurs
    }
}
