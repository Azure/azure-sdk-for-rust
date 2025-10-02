use std::sync::Arc;

use moka::future::Cache;

use crate::{
    models::{ContainerProperties, DatabaseProperties},
    routing::ContainerRoutingMap,
    ResourceId,
};

pub struct ContainerMetadataCache {
    /// Caches a mapping from container ID (the "name") to container properties, including the RID.
    container_properties_cache: Cache<String, Arc<ContainerProperties>>,

    /// Caches a mapping from database ID (the "name") to database properties, including the RID.
    database_properties_cache: Cache<String, Arc<DatabaseProperties>>,

    /// Caches container routing information, mapping from container RID to routing info.
    routing_map_cache: Cache<ResourceId, Arc<ContainerRoutingMap>>,
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
    pub fn new() -> Arc<Self> {
        let container_properties_cache = Cache::new(MAX_CACHE_CAPACITY);
        let database_properties_cache = Cache::new(MAX_CACHE_CAPACITY);
        let routing_map_cache = Cache::new(MAX_CACHE_CAPACITY);
        Arc::new(Self {
            container_properties_cache,
            database_properties_cache,
            routing_map_cache,
        })
    }
}
