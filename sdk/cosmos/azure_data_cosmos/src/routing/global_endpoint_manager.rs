//! Concrete (yet unimplemented) GlobalEndpointManager.
//! All methods currently use `unimplemented!()` as placeholders per request to keep them blank.

use std::borrow::Cow;
use crate::constants::ACCOUNT_PROPERTIES_KEY;
use crate::cosmos_request::{CosmosRequest, CosmosRequestBuilder};
use crate::models::AccountProperties;
use crate::operation_context::OperationType;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::routing::location_cache::{LocationCache, RequestOperation};
use crate::ReadDatabaseOptions;
use azure_core::http::{Pipeline, Response};
use azure_core::Error;
use moka::future::Cache;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct GlobalEndpointManager {
    default_endpoint: String,
    location_cache: Arc<Mutex<LocationCache>>,
    pipeline: Pipeline,
    account_properties_cache: Cache<&'static str, AccountProperties>,
}

impl GlobalEndpointManager {
    /// Creates a new `GlobalEndpointManager` with a `LocationCache` initialized
    /// from the provided `default_endpoint` and `preferred_locations`.
    ///
    /// Assumptions:
    /// - We use simple fixed intervals for background refresh placeholders until
    ///   a real scheduling implementation is added.
    /// - Account refresh/background flags start as `false`.
    pub fn new(
        default_endpoint: String,
        preferred_locations: Vec<Cow<'static, str>>,
        pipeline: Pipeline,
    ) -> Self {
        let location_cache = Arc::new(Mutex::new(LocationCache::new(
            default_endpoint.clone(),
            preferred_locations.clone(),
        )));
        let account_properties_cache = Cache::builder()
            .max_capacity(1)
            .time_to_live(std::time::Duration::from_secs(600))
            .build();

        Self {
            default_endpoint,
            location_cache,
            pipeline,
            account_properties_cache,
        }
    }

    pub fn get_hub_uri(&self) -> String {
        self.default_endpoint.clone()
    }

    #[allow(dead_code)]
    pub fn read_endpoints(&self) -> Vec<String> {
        self.location_cache.lock().unwrap().read_endpoints()
    }

    #[allow(dead_code)]
    pub fn account_read_endpoints(&self) -> Vec<String> {
        self.location_cache.lock().unwrap().read_endpoints()
    }

    #[allow(dead_code)]
    pub fn write_endpoints(&self) -> Vec<String> {
        self.location_cache.lock().unwrap().write_endpoints()
    }

    pub fn preferred_location_count(&self) -> i32 {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .preferred_locations
            .len() as i32
    }

    pub(crate) fn resolve_service_endpoint(&self, request: &CosmosRequest) -> String {
        self.location_cache
            .lock()
            .unwrap()
            .resolve_service_endpoint(request)
    }

    pub fn get_applicable_endpoints(&self, request: &CosmosRequest) -> Vec<String> {
        self.location_cache
            .lock()
            .unwrap()
            .get_applicable_endpoints(request)
    }

    pub fn mark_endpoint_unavailable_for_read(&self, endpoint: &str) {
        self.location_cache
            .lock()
            .unwrap()
            .mark_endpoint_unavailable(endpoint, RequestOperation::Read)
    }

    pub fn mark_endpoint_unavailable_for_write(&self, endpoint: &str) {
        self.location_cache
            .lock()
            .unwrap()
            .mark_endpoint_unavailable(endpoint, RequestOperation::Write)
    }

    pub fn can_use_multiple_write_locations(&self, request: &CosmosRequest) -> bool {
        !request.is_read_only_request()
            && self
                .can_support_multiple_write_locations(request.resource_type, request.operation_type)
    }

    pub async fn refresh_location_async(&self, force_refresh: bool) -> Result<(), Error> {
        // If force_refresh is true, invalidate the cache to ensure a fresh fetch
        if force_refresh {
            self.account_properties_cache
                .invalidate(&ACCOUNT_PROPERTIES_KEY)
                .await;
        }

        // When TTL expires or cache is invalidated, the async block executes and updates location cache
        let _account_prop = self
            .account_properties_cache
            .try_get_with(ACCOUNT_PROPERTIES_KEY, async {
                // Fetch latest account properties from service
                let account_properties = self
                    .get_database_account(Some(ReadDatabaseOptions {
                        ..Default::default()
                    }))
                    .await?
                    .into_body()?;

                // Update location cache with the fetched account properties (only on fresh fetch)
                {
                    let mut cache = self.location_cache.lock().unwrap();
                    cache.on_database_account_read(account_properties.clone());
                }

                Ok(account_properties)
            })
            .await
            .map_err(|e: Arc<Error>| {
                Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("Failed to fetch account properties: {}", e),
                )
            })?;

        Ok(())
    }

    #[allow(dead_code)]
    fn get_available_write_endpoints_by_location(&self) -> HashMap<String, String> {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .account_write_endpoints_by_location
            .clone()
    }

    #[allow(dead_code)]
    fn get_available_read_endpoints_by_location(&self) -> HashMap<String, String> {
        self.location_cache
            .lock()
            .unwrap()
            .locations_info
            .account_read_endpoints_by_location
            .clone()
    }

    pub(crate) fn can_support_multiple_write_locations(
        &self,
        resource_type: ResourceType,
        operation_type: OperationType,
    ) -> bool {
        let cache = self.location_cache.lock().unwrap();
        cache.can_use_multiple_write_locations()
            && (resource_type == ResourceType::Documents
                || (resource_type == ResourceType::StoredProcedures
                    && operation_type == OperationType::Execute))
    }

    /// Retrieves the Cosmos DB account ("database account") properties.
    ///
    /// # Arguments
    /// * `options` - Optional request options (currently unused for custom
    ///   headers, but the context can carry per-call metadata for tracing or
    ///   cancellation).
    async fn get_database_account(
        &self,
        options: Option<ReadDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<AccountProperties>> {
        let options = options.unwrap_or_default();
        let resource_link = ResourceLink::root(ResourceType::DatabaseAccount);
        let builder = CosmosRequestBuilder::new(
            OperationType::Read,
            ResourceType::DatabaseAccount,
            resource_link.clone(),
        );
        let mut cosmos_request = builder.build()?;
        let endpoint = self
            .location_cache
            .lock()
            .unwrap()
            .resolve_service_endpoint(&cosmos_request)
            .parse()?;
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
