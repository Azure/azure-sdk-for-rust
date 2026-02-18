// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Concrete (yet unimplemented) GlobalEndpointManager.
//! All methods currently use `unimplemented!()` as placeholders per request to keep them blank.

use crate::constants::ACCOUNT_PROPERTIES_KEY;
use crate::cosmos_request::CosmosRequest;
use crate::models::AccountProperties;
use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::routing::async_cache::AsyncCache;
use crate::routing::location_cache::{LocationCache, RequestOperation};
use crate::ReadDatabaseOptions;
use azure_core::http::{Pipeline, Response};
use azure_core::Error;
use std::sync::Mutex;
use std::time::Duration;
use url::Url;

/// Manages global endpoint routing, failover, and location awareness for Cosmos DB requests.
///
/// This component coordinates multi-region request routing by maintaining location cache state,
/// refreshing account properties, and resolving service endpoints based on request characteristics
/// and availability. It handles endpoint discovery, tracks unavailable endpoints, and supports
/// multi-master write configurations.
#[derive(Debug)]
pub struct GlobalEndpointManager {
    /// The primary default endpoint URL for the Cosmos DB account
    default_endpoint: Url,

    /// Thread-safe cache of location information including read/write endpoints and availability status
    location_cache: Mutex<LocationCache>,

    /// HTTP pipeline for making requests to the Cosmos DB service
    pipeline: Pipeline,

    /// Cache for account properties with 600 second TTL to reduce redundant service calls
    account_properties_cache: AsyncCache<&'static str, AccountProperties>,
}

impl GlobalEndpointManager {
    /// Creates a new `GlobalEndpointManager` with a `LocationCache` initialized
    /// from the provided `default_endpoint` and `preferred_locations`.
    ///
    /// # Summary
    /// Initializes the endpoint manager with a default endpoint, preferred regions for routing,
    /// and an HTTP pipeline for communication. Sets up location cache for endpoint management
    /// and account properties cache with 600 second TTL. The manager starts with empty endpoint
    /// lists until the first account properties refresh populates regional endpoints.
    ///
    /// # Arguments
    /// * `default_endpoint` - The primary Cosmos DB account endpoint URL
    /// * `preferred_locations` - Ordered list of preferred Azure regions for request routing
    /// * `excluded_regions` - List of regions to exclude from routing
    /// * `pipeline` - HTTP pipeline for making service requests
    ///
    /// # Returns
    /// A new `GlobalEndpointManager` instance ready for request routing
    pub fn new(
        default_endpoint: Url,
        preferred_locations: Vec<RegionName>,
        excluded_regions: Vec<RegionName>,
        pipeline: Pipeline,
    ) -> Self {
        let location_cache = Mutex::new(LocationCache::new(
            default_endpoint.clone(),
            preferred_locations.clone(),
            excluded_regions.clone(),
        ));

        let account_properties_cache = AsyncCache::new(
            Some(Duration::from_secs(600)), // Default 10 minutes TTL
        );

        Self {
            default_endpoint,
            location_cache,
            pipeline,
            account_properties_cache,
        }
    }

    /// Returns the count of preferred locations configured for routing.
    ///
    /// # Summary
    /// Retrieves the number of preferred Azure regions that were specified during
    /// initialization. This count is used by retry policies to determine failover
    /// behavior and calculate maximum retry attempts across regions.
    ///
    /// # Returns
    /// The number of preferred locations as usize
    pub fn preferred_location_count(&self) -> usize {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .preferred_locations
            .len()
    }

    /// Resolves the appropriate service endpoint URL for a given request.
    ///
    /// # Summary
    /// Determines which endpoint should handle the request based on operation type
    /// (read vs write), resource type, preferred locations, and endpoint availability.
    /// Delegates to the location cache which applies routing logic including regional
    /// preferences and failover to available endpoints.
    ///
    /// # Arguments
    /// * `request` - The Cosmos DB request requiring endpoint resolution
    ///
    /// # Returns
    /// The resolved endpoint URL as a String
    pub(crate) fn resolve_service_endpoint(&self, request: &CosmosRequest) -> Url {
        self.location_cache
            .lock()
            .unwrap()
            .resolve_service_endpoint(request)
    }

    /// Resolves the service endpoint using DOP components instead of a full request.
    ///
    /// This is the component-based equivalent of [`resolve_service_endpoint`] and is
    /// used by the new pipeline orchestration loop.
    pub(crate) fn resolve_endpoint_from_components(
        &self,
        routing: &crate::pipeline::components::RoutingState,
        op: &crate::pipeline::components::OperationInfo,
    ) -> Url {
        let location_index = routing.location_index as usize;
        let use_preferred = routing.use_preferred_locations;
        let cache = self.location_cache.lock().unwrap();

        let mut location_endpoint_to_route = None;
        if !use_preferred
            || (!op.is_read_only()
                && !cache.can_support_multiple_write_locations(op.resource_type, op.operation_type))
        {
            let location_info = &cache.locations_info;
            if !location_info.account_write_locations.is_empty() {
                let idx = location_index % location_info.account_write_locations.len();
                location_endpoint_to_route = Some(
                    location_info.account_write_locations[idx]
                        .database_account_endpoint
                        .clone(),
                );
            }
        } else {
            let endpoints =
                cache.get_applicable_endpoints(op.operation_type, op.excluded_regions.as_ref());
            if !endpoints.is_empty() {
                location_endpoint_to_route =
                    Some(endpoints[location_index % endpoints.len()].clone());
            }
        }

        location_endpoint_to_route.unwrap_or(self.default_endpoint.clone())
    }

    /// Marks an endpoint as unavailable for read operations.
    ///
    /// # Summary
    /// Flags the specified endpoint as unavailable for read requests in the location cache.
    /// This is called by retry policies when read requests fail due to endpoint issues,
    /// preventing subsequent read operations from being routed to the failing endpoint.
    /// The endpoint may still be used for write operations if not separately marked unavailable.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint URL to mark as unavailable for reads
    pub fn mark_endpoint_unavailable_for_read(&self, endpoint: &Url) {
        self.location_cache
            .lock()
            .unwrap()
            .mark_endpoint_unavailable(endpoint, RequestOperation::Read)
    }

    /// Marks an endpoint as unavailable for write operations.
    ///
    /// # Summary
    /// Flags the specified endpoint as unavailable for write requests in the location cache.
    /// This is called by retry policies when write requests fail due to endpoint issues,
    /// preventing subsequent write operations from being routed to the failing endpoint.
    /// The endpoint may still be used for read operations if not separately marked unavailable.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint URL to mark as unavailable for writes
    pub fn mark_endpoint_unavailable_for_write(&self, endpoint: &Url) {
        self.location_cache
            .lock()
            .unwrap()
            .mark_endpoint_unavailable(endpoint, RequestOperation::Write)
    }

    /// Determines if a request can utilize multiple write locations.
    ///
    /// # Summary
    /// Evaluates whether the given request can be routed to multiple write regions based
    /// on the request's operation type and resource type. Returns true only for write
    /// operations on resources that support multi-master writes (documents and stored
    /// procedure executions) when the account is configured for multiple write locations.
    ///
    /// # Arguments
    /// * `request` - The Cosmos DB request to evaluate
    ///
    /// # Returns
    /// `true` if the request can use multiple write locations, `false` otherwise
    pub fn can_use_multiple_write_locations(&self, request: &CosmosRequest) -> bool {
        !request.is_read_only_request()
            && self
                .can_support_multiple_write_locations(request.resource_type, request.operation_type)
    }

    /// Refreshes account properties and location information from the service.
    ///
    /// # Summary
    /// Fetches the latest Cosmos DB account properties including regional endpoint information
    /// and updates the location cache. Uses a Moka cache with 600 second TTL to avoid redundant
    /// service calls. If `force_refresh` is true, invalidates the cache to ensure fresh data.
    /// The location cache is updated only when new data is fetched (TTL expiry or forced refresh),
    /// not when serving cached data.
    ///
    /// # Arguments
    /// * `force_refresh` - If true, invalidates cache and forces fresh fetch from service
    ///
    /// # Returns
    /// `Ok(())` if refresh succeeded, `Err` if fetching account properties failed
    pub async fn refresh_location(&self, force_refresh: bool) -> Result<(), Error> {
        // If force_refresh is true, invalidate the cache to ensure a fresh fetch
        if force_refresh {
            self.account_properties_cache
                .remove(&ACCOUNT_PROPERTIES_KEY)
                .await;
        }

        // When TTL expires or cache is invalidated, the async block executes and updates location cache
        _ = self
            .account_properties_cache
            .get(
                ACCOUNT_PROPERTIES_KEY,
                |_| force_refresh,
                || async {
                    // Fetch latest account properties from service
                    let account_properties: AccountProperties =
                        self.get_database_account().await?.into_body().json()?;

                    // Update location cache with the fetched account properties (only on fresh fetch)
                    {
                        let mut cache = self.location_cache.lock().unwrap();
                        cache.on_database_account_read(account_properties.clone());
                    }

                    Ok::<AccountProperties, Error>(account_properties)
                },
            )
            .await;

        Ok(())
    }

    /// Determines if the account supports multiple write locations for specific resource and operation types.
    ///
    /// # Summary
    /// Evaluates whether multi-master writes are supported based on account configuration and
    /// the specific resource/operation combination. Multi-master writes are supported for
    /// Documents (all operations) and StoredProcedures (Execute operation only). Other resource
    /// types like Databases, Containers, etc., do not support multi-write even in multi-master accounts.
    ///
    /// # Arguments
    /// * `resource_type` - The type of resource being operated on
    /// * `operation_type` - The type of operation being performed
    ///
    /// # Returns
    /// `true` if multi-write is supported for the resource/operation, `false` otherwise
    pub(crate) fn can_support_multiple_write_locations(
        &self,
        resource_type: ResourceType,
        operation_type: OperationType,
    ) -> bool {
        let cache = self.location_cache.lock().unwrap();
        cache.can_support_multiple_write_locations(resource_type, operation_type)
    }

    /// Retrieves the Cosmos DB account ("database account") properties from the service.
    ///
    /// # Summary
    /// Makes an HTTP request to fetch account properties including regional endpoint information,
    /// consistency settings, and multi-master configuration. Uses the default endpoint for the
    /// request and constructs a metadata read operation with appropriate resource link. Called
    /// internally by `refresh_location` when cache needs updating.
    ///
    /// # Returns
    /// `Ok(Response<AccountProperties>)` with account metadata, or `Err` if request failed
    async fn get_database_account(&self) -> azure_core::Result<Response<AccountProperties>> {
        let options = ReadDatabaseOptions {
            ..Default::default()
        };
        let resource_link = ResourceLink::root(ResourceType::DatabaseAccount);
        let builder = CosmosRequest::builder(OperationType::Read, resource_link.clone());
        let mut cosmos_request = builder.build()?;
        let endpoint = self
            .location_cache
            .lock()
            .unwrap()
            .resolve_service_endpoint(&cosmos_request);
        cosmos_request.request_context.location_endpoint_to_route = Some(endpoint);
        let ctx_owned = options
            .method_options
            .context
            .with_value(resource_link)
            .into_owned();
        self.pipeline
            .send(&ctx_owned, &mut cosmos_request.into_raw_request(), None)
            .await
            .map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AccountRegion;
    use crate::partition_key::PartitionKey;

    fn create_test_pipeline() -> Pipeline {
        Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::http::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        )
    }

    fn create_test_manager() -> GlobalEndpointManager {
        GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![RegionName::from("West US"), RegionName::from("East US")],
            vec![],
            create_test_pipeline(),
        )
    }

    fn create_test_request(operation_type: OperationType) -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Documents);
        let mut request = CosmosRequest::builder(operation_type, resource_link.clone())
            .partition_key(PartitionKey::from("test"))
            .build()
            .unwrap();

        request.request_context.location_endpoint_to_route =
            Some("https://test.documents.azure.com".parse().unwrap());
        request
    }

    #[test]
    fn test_new_manager_initialization() {
        let manager = create_test_manager();
        assert_eq!(manager.preferred_location_count(), 2);
    }

    #[test]
    fn test_preferred_location_count() {
        let manager = GlobalEndpointManager::new(
            "https://test.documents.azure.com/".parse().unwrap(),
            vec![
                RegionName::from("West US"),
                RegionName::from("East US"),
                RegionName::from("North Europe"),
            ],
            vec![],
            create_test_pipeline(),
        );
        assert_eq!(manager.preferred_location_count(), 3);
    }

    #[test]
    fn test_preferred_location_count_empty() {
        let manager = GlobalEndpointManager::new(
            "https://test.documents.azure.com".parse().unwrap(),
            vec![],
            vec![],
            create_test_pipeline(),
        );
        assert_eq!(manager.preferred_location_count(), 0);
    }

    #[test]
    fn test_resolve_service_endpoint_returns_default() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Read);
        let endpoint = manager.resolve_service_endpoint(&request);
        // Should return default endpoint initially
        assert_eq!(
            endpoint,
            Url::parse("https://test.documents.azure.com/").unwrap()
        );
    }

    #[test]
    fn test_mark_endpoint_unavailable_for_read() {
        let manager = create_test_manager();
        let endpoint = "https://test.documents.azure.com".parse().unwrap();
        let account_region = AccountRegion {
            name: RegionName::from("West US".to_string()),
            database_account_endpoint: "https://test.documents.azure.com".parse().unwrap(),
        };
        // Populate the location cache's regions
        let _ = manager
            .location_cache
            .lock()
            .unwrap()
            .update(vec![account_region.clone()], vec![account_region]);

        // This should not panic
        manager.mark_endpoint_unavailable_for_read(&endpoint);
    }

    #[test]
    fn test_mark_endpoint_unavailable_for_write() {
        let manager = create_test_manager();
        let endpoint = "https://test.documents.azure.com".parse().unwrap();
        let account_region = AccountRegion {
            name: RegionName::from("West US".to_string()),
            database_account_endpoint: "https://test.documents.azure.com".parse().unwrap(),
        };
        // Populate the location cache's regions
        let _ = manager
            .location_cache
            .lock()
            .unwrap()
            .update(vec![account_region.clone()], vec![account_region]);

        // This should not panic
        manager.mark_endpoint_unavailable_for_write(&endpoint);
    }

    #[test]
    fn test_can_use_multiple_write_locations_for_read_request() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Read);

        // Read requests should not use multiple write locations
        assert!(!manager.can_use_multiple_write_locations(&request));
    }

    #[test]
    fn test_can_use_multiple_write_locations_for_write_request() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Create);

        // Whether this returns true or false depends on account configuration
        // Just verify it doesn't panic
        let _ = manager.can_use_multiple_write_locations(&request);
    }

    #[test]
    fn test_can_support_multiple_write_locations_for_documents() {
        let manager = create_test_manager();

        // Documents should potentially support multiple write locations
        // The actual result depends on account configuration
        let _ = manager
            .can_support_multiple_write_locations(ResourceType::Documents, OperationType::Create);
    }

    #[test]
    fn test_can_support_multiple_write_locations_for_stored_procedures() {
        let manager = create_test_manager();

        // Stored procedures with Execute operation should potentially support multiple write locations
        let _ = manager.can_support_multiple_write_locations(
            ResourceType::StoredProcedures,
            OperationType::Execute,
        );
    }

    #[test]
    fn test_can_support_multiple_write_locations_for_databases() {
        let manager = create_test_manager();

        // Database operations should not support multiple write locations
        let result = manager
            .can_support_multiple_write_locations(ResourceType::Databases, OperationType::Create);

        // Databases don't support multi-write
        assert!(!result);
    }
}
