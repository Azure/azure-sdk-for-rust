// cSpell:ignore smol

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use url::Url;
use azure_core::http::{Pipeline, Response};
use crate::{models::ContainerProperties, resource_context::ResourceLink, ReadContainerOptions};
use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;

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
    database_link: Arc<RwLock<Option<ResourceLink>>>,
    global_endpoint_manager: GlobalEndpointManager,
    container_properties_cache: Arc<RwLock<HashMap<String, ContainerProperties>>>,
}

impl CollectionCache {

    pub(crate) fn new(
        pipeline: Pipeline,
        global_endpoint_manager: GlobalEndpointManager
    ) -> Self {
        let container_properties_cache = Arc::new(RwLock::new(HashMap::new()));
        Self {
            pipeline,
            database_link: Arc::new(RwLock::new(None)),
            global_endpoint_manager,
            container_properties_cache,
        }
    }

    pub fn set_database_link(&self, database_link: ResourceLink) {
        *self.database_link.write().unwrap() = Some(database_link);
    }

    pub async fn read_properties_by_id(
        &self,
        container_id: &str,
        options: Option<ReadContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let container_link = self.database_link
            .read().unwrap()
            .clone()
            .unwrap()
            .feed(ResourceType::Containers)
            .item(container_id);

        let mut cosmos_request =
            CosmosRequest::builder(OperationType::Read, container_link.clone()).build()?;

        let location_endpoint = Some(
            self.global_endpoint_manager
                .resolve_service_endpoint(&cosmos_request),
        );

        if let Some(ref endpoint) = location_endpoint {
            cosmos_request.request_context.route_to_location_endpoint(
                cosmos_request.resource_link.url(&Url::parse(endpoint)?),
            );
        }

        let ctx_owned = options
            .method_options
            .context
            .with_value(container_link)
            .into_owned();

        self.pipeline
            .send(&ctx_owned, &mut cosmos_request.into_raw_request(), None)
            .await
            .map(Into::into)
    }

    pub async fn get_database_link(&self) -> Option<ResourceLink> {
        self.database_link.read().unwrap().clone()
    }

    /// Gets the container metadata from the cache, or initializes it using the provided async function if not present.
    pub async fn get_container_metadata(
        &self,
        container_id: &str,
    ) -> Result<ContainerProperties, CacheError> {
        // Check if already in cache
        {
            let cache = self.container_properties_cache.read().unwrap();
            if let Some(properties) = cache.get(container_id) {
                return Ok(properties.clone());
            }
        }

        // Not in cache, fetch from service
        let response = self.read_properties_by_id(container_id, None)
            .await
            .map_err(Arc::new)?;

        let properties = serde_json::from_slice::<ContainerProperties>(response.body()).unwrap();

        // Update cache
        self.container_properties_cache
            .write()
            .unwrap()
            .insert(container_id.to_string(), properties.clone());

        Ok(properties)
    }

    /// Removes the container metadata from the cache, forcing a refresh on the next access.
    pub async fn remove_container_metadata(&self, container_id: &str) {
        self.container_properties_cache.write().unwrap().remove(container_id);
    }
}