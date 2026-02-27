//! Concrete (yet unimplemented) GlobalEndpointManager.

use crate::background_task_manager::BackgroundTaskManager;
use crate::constants::ACCOUNT_PROPERTIES_KEY;
use crate::cosmos_request::CosmosRequest;
use crate::models::AccountProperties;
use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::routing::async_cache::AsyncCache;
use crate::routing::location_cache::{LocationCache, RequestOperation};
use azure_core::http::{Context, Pipeline, Response};
use azure_core::time::Duration;
use azure_core::Error;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};
use tracing::info;
use url::Url;

/// Type alias for the account refresh callback function.
pub type OnAccountRefreshCallback = Arc<dyn Fn(&AccountProperties) + Send + Sync>;

/// Default interval (in seconds) at which the background account refresh loop runs.
const BACKGROUND_ACCOUNT_REFRESH_INTERVAL_SECS: i64 = 600;

/// Manages global endpoint routing, failover, and location awareness for Cosmos DB requests.
///
/// This component coordinates multi-region request routing by maintaining location cache state,
/// refreshing account properties, and resolving service endpoints based on request characteristics
/// and availability. It handles endpoint discovery, tracks unavailable endpoints, and supports
/// multi-master write configurations.
pub(crate) struct GlobalEndpointManager {
    /// The primary default endpoint URL for the Cosmos DB account
    default_endpoint: Url,

    /// Thread-safe cache of location information including read/write endpoints and availability status
    location_cache: Mutex<LocationCache>,

    /// HTTP pipeline for making requests to the Cosmos DB service
    pipeline: Pipeline,

    /// Cache for account properties with 600 second TTL to reduce redundant service calls
    account_properties_cache: AsyncCache<&'static str, AccountProperties>,

    /// Optional callback invoked when account properties are refreshed via HTTP call
    on_account_refresh: Mutex<Option<OnAccountRefreshCallback>>,

    /// Flag indicating if the background connection initialization task is active.
    background_account_refresh_active: AtomicBool,

    /// Manages background tasks and signals them to stop when dropped.
    background_task_manager: BackgroundTaskManager,

    /// Background account refresh interval in seconds. Default is 10 minutes.
    background_account_refresh_interval: Duration,
}

impl Debug for GlobalEndpointManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalEndpointManager")
            .field("default_endpoint", &self.default_endpoint)
            .field("location_cache", &self.location_cache)
            .field("pipeline", &self.pipeline)
            .field("account_properties_cache", &self.account_properties_cache)
            .field("on_account_refresh", &"<callback>")
            .finish()
    }
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
    ) -> Arc<Self> {
        let location_cache = Mutex::new(LocationCache::new(
            default_endpoint.clone(),
            preferred_locations.clone(),
            excluded_regions.clone(),
        ));

        let account_properties_cache = AsyncCache::new(
            Some(Duration::seconds(600)), // Default 10 minutes TTL
        );

        let instance = Arc::new(Self {
            default_endpoint,
            location_cache,
            pipeline,
            account_properties_cache,
            on_account_refresh: Mutex::new(None),
            background_account_refresh_active: AtomicBool::new(false),
            background_task_manager: BackgroundTaskManager::new(),
            background_account_refresh_interval: Duration::seconds(
                BACKGROUND_ACCOUNT_REFRESH_INTERVAL_SECS,
            ),
        });
        instance.initialize_and_start_background_account_refresh();
        instance
    }

    /// Sets a callback to be invoked whenever account properties are refreshed via HTTP call.
    ///
    /// # Summary
    /// Registers a callback function that will be called automatically whenever `refresh_location`
    /// fetches new account properties from the service (not when serving from cache). This is useful
    /// for updating partition-level failover configurations when account properties change.
    ///
    /// # Arguments
    /// * `callback` - The callback function to invoke with the refreshed account properties
    pub fn set_on_account_refresh_callback(&self, callback: OnAccountRefreshCallback) {
        let mut guard = self.on_account_refresh.lock().unwrap();
        *guard = Some(callback);
    }

    /// Returns the default hub endpoint URL for the Cosmos DB account.
    ///
    /// # Summary
    /// Retrieves the primary endpoint URL that was configured during manager initialization.
    /// This is the main entry point for the Cosmos DB account and is used as a fallback
    /// when no preferred regional endpoints are available or configured.
    ///
    /// # Returns
    /// The default endpoint URL as a String
    pub fn hub_uri(&self) -> &Url {
        &self.default_endpoint
    }

    /// Returns the list of available read endpoints.
    ///
    /// # Summary
    /// Retrieves all currently available read endpoints from the location cache. The list
    /// includes regional endpoints that can handle read operations and are not marked as
    /// unavailable. Initially empty until account properties are fetched and processed.
    ///
    /// # Returns
    /// A vector of endpoint URLs available for read operations
    #[allow(dead_code)]
    pub fn read_endpoints(&self) -> Vec<Url> {
        self.location_cache
            .lock()
            .unwrap()
            .read_endpoints()
            .to_vec()
    }

    /// Returns the list of available account read endpoints.
    ///
    /// # Summary
    /// Alias for `read_endpoints()` that retrieves all currently available read endpoints
    /// from the location cache. Provides the same functionality with an alternative name
    /// for clarity in account-level operations context.
    ///
    /// # Returns
    /// A vector of endpoint URLs available for read operations
    #[allow(dead_code)]
    pub fn account_read_endpoints(&self) -> Vec<Url> {
        self.location_cache
            .lock()
            .unwrap()
            .read_endpoints()
            .to_vec()
    }

    /// Returns the list of available write endpoints.
    ///
    /// # Summary
    /// Retrieves all currently available write endpoints from the location cache. The list
    /// includes regional endpoints that can handle write operations and are not marked as
    /// unavailable. For multi-master accounts, this may include multiple regions; for
    /// single-master accounts, typically only the write region. Initially empty until
    /// account properties are fetched.
    ///
    /// # Returns
    /// A vector of endpoint URLs available for write operations
    #[allow(dead_code)]
    pub fn write_endpoints(&self) -> Vec<Url> {
        self.location_cache
            .lock()
            .unwrap()
            .write_endpoints()
            .to_vec()
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

    /// Returns all endpoints applicable for handling a specific request.
    ///
    /// # Summary
    /// Retrieves the list of endpoints that could potentially handle the request based
    /// on its operation type (read or write) and current endpoint availability. Used by
    /// retry policies to determine how many alternative endpoints are available for
    /// failover attempts.
    ///
    /// # Arguments
    /// * `request` - The Cosmos DB request to evaluate
    ///
    /// # Returns
    /// A vector of applicable endpoint URLs
    pub fn applicable_endpoints(
        &self,
        operation_type: OperationType,
        excluded_regions: Option<&Vec<RegionName>>,
    ) -> Vec<Url> {
        self.location_cache
            .lock()
            .unwrap()
            .get_applicable_endpoints(operation_type, excluded_regions)
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

        // Flag to track if an HTTP call was made
        let http_call_made = AtomicBool::new(false);

        // When TTL expires or cache is invalidated, the async block executes and updates location cache
        let account_properties = self
            .account_properties_cache
            .get(
                ACCOUNT_PROPERTIES_KEY,
                |_| force_refresh,
                || async {
                    // Fetch latest account properties from service
                    let account_properties: AccountProperties =
                        self.get_database_account().await?.into_body().json()?;

                    // Mark that we're making an HTTP call
                    http_call_made.store(true, Ordering::SeqCst);

                    // Update location cache with the fetched account properties (only on fresh fetch)
                    {
                        let mut cache = self.location_cache.lock().unwrap();
                        cache.on_database_account_read(account_properties.clone());
                    }

                    Ok::<AccountProperties, Error>(account_properties)
                },
            )
            .await?;

        // Invoke the registered callback if an HTTP call was made
        let was_http_call_made = http_call_made.load(Ordering::SeqCst);
        if was_http_call_made {
            // Clone the callback out of the mutex, then drop the lock before invoking it
            // to avoid holding the mutex during arbitrary user code (prevents potential deadlocks).
            let callback = {
                let guard = self.on_account_refresh.lock().unwrap();
                guard.as_ref().map(Arc::clone)
            };

            if let Some(callback) = callback {
                callback(&account_properties);
            }
        }

        Ok(())
    }

    /// Returns a map of write endpoints indexed by location name.
    ///
    /// # Summary
    /// Retrieves a mapping from Azure region names to their corresponding write endpoint URLs.
    /// This provides direct lookup of write endpoints by location, useful for diagnostic
    /// and monitoring scenarios. The map reflects the current account configuration and
    /// may be empty until account properties are fetched.
    ///
    /// # Returns
    /// A HashMap containing the location names with their corresponding write endpoint URLs
    #[allow(dead_code)]
    fn available_write_endpoints_by_location(&self) -> HashMap<RegionName, Url> {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .account_write_endpoints_by_location
            .clone()
    }

    /// Returns a map of read endpoints indexed by location name.
    ///
    /// # Summary
    /// Retrieves a mapping from Azure region names to their corresponding read endpoint URLs.
    /// This provides direct lookup of read endpoints by location, useful for diagnostic
    /// and monitoring scenarios. The map reflects the current account configuration and
    /// may be empty until account properties are fetched.
    ///
    /// # Returns
    /// A HashMap mapping location names to read endpoint URLs
    #[allow(dead_code)]
    fn available_read_endpoints_by_location(&self) -> HashMap<RegionName, Url> {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .account_read_endpoints_by_location
            .clone()
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
    pub async fn get_database_account(&self) -> azure_core::Result<Response<AccountProperties>> {
        let resource_link = ResourceLink::root(ResourceType::DatabaseAccount);
        let builder = CosmosRequest::builder(OperationType::Read, resource_link.clone());
        let mut cosmos_request = builder.build()?;
        let endpoint = self
            .location_cache
            .lock()
            .unwrap()
            .resolve_service_endpoint(&cosmos_request);
        cosmos_request.request_context.location_endpoint_to_route = Some(endpoint);
        let ctx_owned = Context::default().with_value(resource_link);
        self.pipeline
            .send(&ctx_owned, &mut cosmos_request.into_raw_request(), None)
            .await
            .map(Into::into)
    }

    /// Initializes and starts the background account refresh loop.
    ///
    /// # Summary
    /// Atomically checks and sets the `background_account_refresh_active` flag to ensure only
    /// one background refresh task runs at a time. If the flag is already set, the call is a
    /// no-op. Otherwise, it spawns a background task via [`BackgroundTaskManager`] that
    /// periodically refreshes account properties. The spawned task captures a `Weak<Self>`
    /// reference to avoid a reference cycle, allowing the `GlobalEndpointManager` to be
    /// dropped normally, which in turn cancels the background task.
    fn initialize_and_start_background_account_refresh(self: &Arc<Self>) {
        // Atomically try to set from false to true.
        // If it was already true, another thread already started the task.
        if self
            .background_account_refresh_active
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        let weak_self = Arc::downgrade(self);
        // Spawn via BackgroundTaskManager so the task is tracked and will be
        // canceled when the manager (and thus the client) is dropped.
        // We capture a Weak<Self> (not Arc<Self>) to avoid a reference cycle
        // that would prevent the GlobalEndpointManager from ever being dropped.
        self.background_task_manager.spawn(Box::pin(async move {
            Self::initiate_background_account_refresh_loop(weak_self).await;
        }));
    }

    /// Runs the background account refresh loop that periodically updates location information.
    ///
    /// # Summary
    /// Executes an infinite loop that sleeps for the configured refresh interval and then
    /// calls [`refresh_location`](Self::refresh_location) with `force_refresh: true` to
    /// fetch the latest account properties from the service. The loop holds only a
    /// [`Weak`] reference to the `GlobalEndpointManager`; if the manager has been dropped
    /// (i.e., the `Weak` upgrade fails), the loop exits gracefully. Any errors during
    /// refresh are logged but do not terminate the loop.
    ///
    /// # Arguments
    /// * `weak_self` - A weak reference to the owning `GlobalEndpointManager`
    async fn initiate_background_account_refresh_loop(weak_self: Weak<Self>) {
        // Briefly upgrade to read the interval, then release the strong ref
        // so it does not keep Self alive across the sleep.
        let interval = match weak_self.upgrade() {
            Some(strong) => strong.background_account_refresh_interval,
            None => return,
        };

        loop {
            // Use the runtime-agnostic sleep from azure_core
            azure_core::async_runtime::get_async_runtime()
                .sleep(interval)
                .await;

            // Upgrade the Weak ref for this iteration only. If it fails, the
            // manager has been dropped and we should exit.
            let strong = match weak_self.upgrade() {
                Some(s) => s,
                None => {
                    info!("GlobalEndpointManager: background refresh loop exiting because the client has been dropped.");
                    return;
                }
            };

            info!("GlobalEndpointManager: refresh_location() trying to refresh database account.");

            if let Err(e) = strong.refresh_location(true).await {
                tracing::error!("GlobalEndpointManager: initiate_background_account_refresh_loop() - failed to refresh database account. Exception: {}", e);
            }
            // `strong` is dropped here, releasing the temporary strong ref
            // before the next sleep.
        }
    }

    /// Updates the location cache with the given write and read regions.
    ///
    /// This is exposed as `pub(crate)` to allow other modules' tests to populate
    /// endpoints without requiring a live service call to `refresh_location`.
    #[cfg(test)]
    pub(crate) fn update_location_cache(
        &self,
        write_locations: Vec<crate::models::AccountRegion>,
        read_locations: Vec<crate::models::AccountRegion>,
    ) {
        let _ = self
            .location_cache
            .lock()
            .unwrap()
            .update(write_locations, read_locations);
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

    fn create_test_manager() -> Arc<GlobalEndpointManager> {
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

    #[tokio::test]
    async fn test_new_manager_initialization() {
        let manager = create_test_manager();
        assert_eq!(
            manager.hub_uri(),
            &Url::parse("https://test.documents.azure.com/").unwrap()
        );
    }

    #[tokio::test]
    async fn test_hub_uri() {
        let manager = create_test_manager();
        let hub_uri = manager.hub_uri();
        assert_eq!(
            hub_uri,
            &Url::parse("https://test.documents.azure.com/").unwrap()
        );
    }

    #[tokio::test]
    async fn test_resolve_service_endpoint_returns_default() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Read);
        let endpoint = manager.resolve_service_endpoint(&request);
        // Should return default endpoint initially
        assert_eq!(
            endpoint,
            Url::parse("https://test.documents.azure.com/").unwrap()
        );
    }

    #[tokio::test]
    async fn test_read_endpoints_initial_state() {
        let manager = create_test_manager();
        let endpoints = manager.read_endpoints();
        // Initial state may be empty until account properties are loaded
        // Just verify it returns a valid vector and doesn't panic
        let _ = endpoints.len();
    }

    #[tokio::test]
    async fn test_write_endpoints_initial_state() {
        let manager = create_test_manager();
        let endpoints = manager.write_endpoints();
        // Initial state may be empty until account properties are loaded
        // Just verify it returns a valid vector and doesn't panic
        let _ = endpoints.len();
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_for_read() {
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

        // The endpoint should still be in the system but marked unavailable
        let read_endpoints = manager.read_endpoints();
        assert!(!read_endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_mark_endpoint_unavailable_for_write() {
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

        // The endpoint should still be in the system but marked unavailable
        let write_endpoints = manager.write_endpoints();
        assert!(!write_endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_can_use_multiple_write_locations_for_read_request() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Read);

        // Read requests should not use multiple write locations
        assert!(!manager.can_use_multiple_write_locations(&request));
    }

    #[tokio::test]
    async fn test_can_use_multiple_write_locations_for_write_request() {
        let manager = create_test_manager();
        let request = create_test_request(OperationType::Create);

        // Whether this returns true or false depends on account configuration
        // Just verify it doesn't panic
        let _ = manager.can_use_multiple_write_locations(&request);
    }

    #[tokio::test]
    async fn test_can_support_multiple_write_locations_for_documents() {
        let manager = create_test_manager();

        // Documents should potentially support multiple write locations
        // The actual result depends on account configuration
        let _ = manager
            .can_support_multiple_write_locations(ResourceType::Documents, OperationType::Create);
    }

    #[tokio::test]
    async fn test_can_support_multiple_write_locations_for_stored_procedures() {
        let manager = create_test_manager();

        // Stored procedures with Execute operation should potentially support multiple write locations
        let _ = manager.can_support_multiple_write_locations(
            ResourceType::StoredProcedures,
            OperationType::Execute,
        );
    }

    #[tokio::test]
    async fn test_can_support_multiple_write_locations_for_databases() {
        let manager = create_test_manager();

        // Database operations should not support multiple write locations
        let result = manager
            .can_support_multiple_write_locations(ResourceType::Databases, OperationType::Create);

        // Databases don't support multi-write
        assert!(!result);
    }

    #[tokio::test]
    async fn test_applicable_endpoints() {
        let manager = create_test_manager();
        let endpoints = manager.applicable_endpoints(OperationType::Read, None);
        assert!(!endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_applicable_excluded_endpoints() {
        let manager = create_test_manager();
        // Exclude all regions to test behavior - should still return default endpoint
        let excluded_regions: Vec<RegionName> =
            vec![RegionName::from("West US"), RegionName::from("East US")];
        let endpoints = manager.applicable_endpoints(OperationType::Read, Some(&excluded_regions));
        assert!(!endpoints.is_empty());
        let endpoints =
            manager.applicable_endpoints(OperationType::Create, Some(&excluded_regions));
        assert!(!endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_account_read_endpoints() {
        let manager = create_test_manager();
        let endpoints = manager.account_read_endpoints();

        // Should return the same as read_endpoints
        assert_eq!(endpoints, manager.read_endpoints());
    }

    #[tokio::test]
    async fn test_available_write_endpoints_by_location() {
        let manager = create_test_manager();
        let endpoints_map = manager.available_write_endpoints_by_location();

        // Should not panic and return a valid map
        let _ = endpoints_map.len();
    }

    #[tokio::test]
    async fn test_available_read_endpoints_by_location() {
        let manager = create_test_manager();
        let endpoints_map = manager.available_read_endpoints_by_location();

        // Should not panic and return a valid map
        let _ = endpoints_map.len();
    }
}
