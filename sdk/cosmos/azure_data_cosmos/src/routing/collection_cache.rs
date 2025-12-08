// cSpell:ignore smol

use std::sync::Arc;

use moka::future::Cache;
use azure_core::http::{Pipeline, Response};
use crate::{models::ContainerProperties, resource_context::ResourceLink, ReadContainerOptions};
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;

#[derive(Debug)]
pub enum CacheError {
    FetchError(Arc<azure_core::Error>),
}

impl From<Arc<azure_core::Error>> for CacheError {
    fn from(e: Arc<azure_core::Error>) -> Self {
        CacheError::FetchError(e)
    }
}

impl From<CacheError> for azure_core::Error {
    fn from(e: CacheError) -> Self {
        match e {
            CacheError::FetchError(e) => {
                let message = format!("error updating Container Metadata Cache: {}", e);
                azure_core::Error::with_error(azure_core::error::ErrorKind::Other, e, message)
            }
        }
    }
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::FetchError(e) => write!(f, "error fetching latest value: {}", e),
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CacheError::FetchError(e) => Some(&**e),
        }
    }
}


/// A client for working with a specific container in a Cosmos DB account.
///
/// You can get a `Container` by calling [`DatabaseClient::container_client()`](crate::clients::DatabaseClient::container_client()).
#[derive(Clone, Debug)]
pub struct CollectionCache {
    pipeline: Pipeline,
    database_link: Option<ResourceLink>,
    container_properties_cache: Cache<ResourceLink, Arc<ContainerProperties>>,
}

impl CollectionCache {

    pub(crate) fn new(
        pipeline: Pipeline,
    ) -> Self {
        // let link = database_link
        //     .feed(ResourceType::Containers)
        //     .item(container_id);
        // let items_link = link.feed(ResourceType::Documents);
        let container_properties_cache = Cache::new(MAX_CACHE_CAPACITY);
        Self {
            pipeline,
            database_link: None,
            container_properties_cache,
        }
    }

    pub fn set_database_link(&mut self, database_link: ResourceLink) {
        self.database_link = Some(database_link);
    }

    pub async fn read_properties_by_id(
        &self,
        container_id: &str,
        options: Option<ReadContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let container_link = self.database_link
            .clone()
            .unwrap()
            .feed(ResourceType::Containers)
            .item(container_id);

        let cosmos_request =
            CosmosRequest::builder(OperationType::Read, container_link.clone()).build()?;

        self.pipeline
            .send(&options.method_options.context, &mut cosmos_request.into_raw_request(), None)
            .await
            .map(Into::into)
//        self.pipeline
//            .send(cosmos_request, options.method_options.context)
//            .await
    }
    
    pub async fn get_database_link(&self) -> Option<ResourceLink> {
        self.database_link.clone()
    }
}

/// A subset of container properties that are stable and suitable for caching.
pub(crate) struct ContainerMetadata {
    pub resource_id: String,
    pub container_link: ResourceLink,
}

impl ContainerMetadata {
    // We can't use From<ContainerProperties> because we also want the container link.
    pub fn from_properties(
        properties: &ContainerProperties,
        container_link: ResourceLink,
    ) -> azure_core::Result<Self> {
        let resource_id = properties
            .system_properties
            .resource_id
            .clone()
            .ok_or_else(|| {
                azure_core::Error::new(
                    azure_core::error::ErrorKind::Other,
                    "container properties is missing expected value 'resource_id'",
                )
            })?;
        Ok(Self {
            resource_id,
            container_link,
        })
    }
}

/// A cache for container metadata, including properties and routing information.
///
/// The cache can be cloned cheaply, and all clones share the same underlying cache data.
#[derive(Clone)]
pub struct ContainerMetadataCache {
    /// Caches stable container metadata, mapping from container link to metadata.
    container_properties_cache: Cache<ResourceLink, Arc<ContainerMetadata>>,
}

// TODO: Review this value.
// Cosmos has a backend limit of 500 databases and containers per account by default.
// This value affects when Moka will start evicting entries from the cache.
// It could probably be much lower without much impact, but we need to do the research to be sure.
const MAX_CACHE_CAPACITY: u64 = 500;

impl ContainerMetadataCache {
    /// Creates a new `ContainerMetadataCache` with default settings.
    ///
    /// Since the cache is designed to be shared, it is returned inside an `Arc`.
    pub fn new() -> Self {
        let
            container_properties_cache = Cache::new(MAX_CACHE_CAPACITY);
        Self {
            container_properties_cache,
        }
    }

    /// Unconditionally updates the cache with the provided container metadata.
    pub async fn set_container_metadata(&self, metadata: ContainerMetadata) {
        let metadata = Arc::new(metadata);

        self.container_properties_cache
            .insert(metadata.container_link.clone(), metadata)
            .await;
    }

    /// Gets the container metadata from the cache, or initializes it using the provided async function if not present.
    pub async fn get_container_metadata(
        &self,
        key: &ResourceLink,
        init: impl std::future::Future<Output = azure_core::Result<ContainerMetadata>>,
    ) -> Result<Arc<ContainerMetadata>, CacheError> {
        // TODO: Background refresh. We can do background refresh by storing an expiry time in the cache entry.
        // Then, if the entry is stale, we can return the stale entry and spawn a background task to refresh it.
        // There's a little trickiness here in that we can't directly spawn a task because that depends on a specific Async Runtime (tokio, smol, etc).
        // The core SDK has an AsyncRuntime abstraction that we can use to spawn the task.
        Ok(self
            .container_properties_cache
            .try_get_with_by_ref(key, async { init.await.map(Arc::new) })
            .await?)
    }

    /// Removes the container metadata from the cache, forcing a refresh on the next access.
    pub async fn remove_container_metadata(&self, key: &ResourceLink) {
        self.container_properties_cache.invalidate(key).await;
    }
}