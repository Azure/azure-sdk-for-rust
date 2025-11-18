// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use crate::cosmos_request::CosmosRequest;
use crate::models::{AccountProperties, AccountRegion};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, SystemTime},
};
use tracing::info;

const DEFAULT_EXPIRATION_TIME: Duration = Duration::from_secs(5 * 60);

/// Represents the type of operation for endpoint routing and availability tracking.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum RequestOperation {
    /// Read operations (queries, point reads)
    Read,
    /// Write operations (create, update, delete)
    Write,
    /// All operations (both read and write)
    All,
}

impl RequestOperation {
    /// Determines if this operation type includes another operation type.
    ///
    /// # Summary
    /// Checks if the current operation encompasses the specified operation. The `All` operation
    /// includes both `Read` and `Write`, while `Read` and `Write` only include themselves.
    /// Used for endpoint unavailability checks to determine if an endpoint is unavailable for
    /// a specific operation type.
    ///
    /// # Arguments
    /// * `other` - The operation type to check for inclusion
    ///
    /// # Returns
    /// `true` if this operation includes the other operation, `false` otherwise
    pub fn includes(self, other: RequestOperation) -> bool {
        matches!(
            (self, other),
            (RequestOperation::All, _)
                | (_, RequestOperation::All)
                | (RequestOperation::Read, RequestOperation::Read)
                | (RequestOperation::Write, RequestOperation::Write)
        )
    }
}

/// Contains location and endpoint information for a Cosmos DB account.
#[derive(Clone, Default, Debug)]
pub struct DatabaseAccountLocationsInfo {
    /// User-specified preferred Azure regions for request routing
    pub preferred_locations: Vec<Cow<'static, str>>,
    /// List of regions where write operations are supported
    pub account_write_locations: Vec<AccountRegion>,
    /// List of regions where read operations are supported
    pub account_read_locations: Vec<AccountRegion>,
    /// Map from location name to write endpoint URL
    pub account_write_endpoints_by_location: HashMap<String, String>,
    /// Map from location name to read endpoint URL
    pub(crate) account_read_endpoints_by_location: HashMap<String, String>,
    /// Ordered list of available write endpoint URLs (preferred first, unavailable last)
    pub write_endpoints: Vec<String>,
    /// Ordered list of available read endpoint URLs (preferred first, unavailable last)
    pub read_endpoints: Vec<String>,
}

/// Tracks when an endpoint was marked unavailable and for which operations.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationUnavailabilityInfo {
    /// Timestamp when the endpoint was last marked unavailable
    pub last_check_time: SystemTime,
    /// Type of operation(s) for which the endpoint is unavailable
    pub unavailable_operation: RequestOperation,
}

/// Manages location-aware endpoint routing and availability tracking for Cosmos DB requests.
///
/// Maintains endpoint lists for read and write operations, tracks endpoint availability,
/// handles preferred location ordering, and resolves service endpoints based on request
/// characteristics and regional preferences.
#[derive(Default, Debug)]
pub struct LocationCache {
    /// The primary default endpoint URL for the Cosmos DB account
    pub default_endpoint: String,
    /// Location and endpoint information including preferred regions and available endpoints
    pub locations_info: DatabaseAccountLocationsInfo,
    /// Thread-safe map tracking unavailable endpoints and when they were last checked
    pub location_unavailability_info_map: RwLock<HashMap<String, LocationUnavailabilityInfo>>,
}

impl LocationCache {
    /// Creates a new LocationCache with default endpoint and preferred locations.
    ///
    /// # Summary
    /// Initializes a location cache for managing endpoint routing. The cache starts with
    /// empty endpoint lists that will be populated when account properties are read. The
    /// preferred locations determine routing priority order when multiple regions are available.
    ///
    /// # Arguments
    /// * `default_endpoint` - The primary Cosmos DB account endpoint URL
    /// * `preferred_locations` - Ordered list of preferred Azure regions for routing
    ///
    /// # Returns
    /// A new `LocationCache` instance ready for endpoint management
    pub fn new(default_endpoint: String, preferred_locations: Vec<Cow<'static, str>>) -> Self {
        Self {
            default_endpoint,
            locations_info: DatabaseAccountLocationsInfo {
                preferred_locations,
                ..Default::default()
            },
            location_unavailability_info_map: RwLock::new(HashMap::new()),
        }
    }

    /// Returns the list of available read endpoints.
    ///
    /// # Summary
    /// Retrieves a cloned list of read endpoint URLs ordered by preference. Preferred locations
    /// that are available appear first, followed by unavailable endpoints, then the default
    /// endpoint if no preferred locations are configured.
    ///
    /// # Returns
    /// A vector of read endpoint URLs
    pub fn read_endpoints(&self) -> Vec<String> {
        self.locations_info.read_endpoints.clone()
    }

    /// Returns the list of available write endpoints.
    ///
    /// # Summary
    /// Retrieves a cloned list of write endpoint URLs ordered by preference. Preferred locations
    /// that are available appear first, followed by unavailable endpoints, then the default
    /// endpoint if no preferred locations are configured.
    ///
    /// # Returns
    /// A vector of write endpoint URLs
    pub fn write_endpoints(&self) -> Vec<String> {
        self.locations_info.write_endpoints.clone()
    }

    /// Updates location cache with account properties from the service.
    ///
    /// # Summary
    /// Processes account properties fetched from Cosmos DB and updates internal endpoint lists.
    /// Extracts writable and readable regions, builds endpoint mappings, and refreshes the
    /// ordered endpoint lists based on availability and preferences. Called when account
    /// properties are refreshed from the service.
    ///
    /// # Arguments
    /// * `account_properties` - Account metadata including regional endpoint information
    pub fn on_database_account_read(&mut self, account_properties: AccountProperties) {
        let write_regions = account_properties.writable_locations;
        let read_regions = account_properties.readable_locations;
        let _ = &self.update(write_regions, read_regions);
    }

    /// Updates location cache with new write and read regions.
    ///
    /// # Summary
    /// Processes regional endpoint information and updates internal data structures. Converts
    /// region lists into endpoint mappings (location name -> endpoint URL), stores the region
    /// information, and refreshes the ordered endpoint lists based on preferences and availability.
    /// This is the core method for synchronizing cache state with account configuration.
    ///
    /// # Arguments
    /// * `write_locations` - List of regions supporting write operations
    /// * `read_locations` - List of regions supporting read operations
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` if update fails
    pub fn update(
        &mut self,
        write_locations: Vec<AccountRegion>,
        read_locations: Vec<AccountRegion>,
    ) -> Result<(), &'static str> {
        // Separate write locations into appropriate hashmap and list
        if !write_locations.is_empty() {
            let (account_write_endpoints_by_location, account_write_locations) =
                self.get_endpoints_by_location(write_locations);
            self.locations_info.account_write_endpoints_by_location =
                account_write_endpoints_by_location;
            self.locations_info.account_write_locations = account_write_locations;
        }

        // Separate read locations into appropriate hashmap and list
        if !read_locations.is_empty() {
            let (account_read_endpoints_by_location, account_read_locations) =
                self.get_endpoints_by_location(read_locations);
            self.locations_info.account_read_endpoints_by_location =
                account_read_endpoints_by_location;
            self.locations_info.account_read_locations = account_read_locations;
        }

        self.refresh_endpoints();

        Ok(())
    }

    /// Marks an endpoint as unavailable for specific operations.
    ///
    /// # Summary
    /// Records that an endpoint is unavailable for the specified operation type (Read, Write, or All).
    /// Updates the unavailability map with current timestamp and operation type. If the endpoint
    /// is already marked unavailable for a different operation, upgrades to `RequestOperation::All`.
    /// After marking unavailable, refreshes endpoint lists to move the unavailable endpoint to
    /// the end of routing priority. Endpoint will be considered available again after 5 minutes.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint URL to mark unavailable
    /// * `operation` - The operation type (Read, Write, or All) for which endpoint is unavailable
    pub fn mark_endpoint_unavailable(&mut self, endpoint: &str, operation: RequestOperation) {
        let now = SystemTime::now();

        {
            let mut location_unavailability_info_map =
                self.location_unavailability_info_map.write().unwrap();

            if let Some(info) = location_unavailability_info_map.get_mut(endpoint) {
                // update last check time and operation in unavailability_info_map
                info.last_check_time = now;
                if !info.unavailable_operation.includes(operation) {
                    info.unavailable_operation = RequestOperation::All;
                }
            } else {
                // If the endpoint is not in the map, insert it
                location_unavailability_info_map.insert(
                    endpoint.to_string(),
                    LocationUnavailabilityInfo {
                        last_check_time: now,
                        unavailable_operation: operation,
                    },
                );
            }

            info!(
                "Endpoint {} marked unavailable for {:?}",
                endpoint, operation
            );
        }

        self.refresh_endpoints();
    }

    /// Checks if an endpoint is currently unavailable for a specific operation.
    ///
    /// # Summary
    /// Queries the unavailability map to determine if an endpoint should be avoided for the
    /// given operation type. Returns true only if the endpoint is marked unavailable for the
    /// operation AND less than 5 minutes have elapsed since it was marked. After 5 minutes,
    /// endpoints are automatically considered available again.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint URL to check
    /// * `operation` - The operation type to check for
    ///
    /// # Returns
    /// `true` if endpoint is unavailable and not expired, `false` otherwise
    pub fn is_endpoint_unavailable(&self, endpoint: &str, operation: RequestOperation) -> bool {
        let location_unavailability_info_map =
            self.location_unavailability_info_map.read().unwrap();
        if let Some(info) = location_unavailability_info_map.get(endpoint) {
            // Checks if endpoint is unavailable for the given operation
            let elapsed =
                info.last_check_time.elapsed().unwrap_or_default() < DEFAULT_EXPIRATION_TIME;
            info.unavailable_operation.includes(operation) && elapsed
        } else {
            false
        }
    }

    /// Resolves the appropriate service endpoint for a request.
    ///
    /// # Summary
    /// Determines which endpoint should handle the request based on operation type (read vs write),
    /// location index routing preference, multi-write support, and preferred location settings.
    /// For requests not using preferred locations or writes without multi-write support, routes to
    /// write locations directly. Otherwise, selects from ordered read/write endpoint lists based
    /// on location index (with wraparound via modulo). Returns default endpoint if no regions
    /// are configured.
    ///
    /// # Arguments
    /// * `request` - The Cosmos DB request requiring endpoint resolution
    ///
    /// # Returns
    /// The resolved endpoint URL as a String
    pub fn resolve_service_endpoint(&self, request: &CosmosRequest) -> String {
        // Returns service endpoint based on index, if index out of bounds or operation not supported, returns default endpoint
        let location_index = request.request_context.location_index_to_route.unwrap_or(0) as usize;
        let mut location_endpoint_to_route = self.default_endpoint.clone();
        if !request
            .request_context
            .use_preferred_locations
            .unwrap_or(true)
            || (!request.operation_type.is_read_only() && !self.can_use_multiple_write_locations())
        {
            let location_info = &self.locations_info;
            if !location_info.account_write_locations.is_empty() {
                let idx = (location_index) % location_info.account_write_locations.len();
                location_endpoint_to_route = location_info.account_write_locations[idx]
                    .database_account_endpoint
                    .clone();
            }
        } else {
            let endpoints = if request.operation_type.is_read_only() {
                self.read_endpoints()
            } else {
                self.write_endpoints()
            };

            if !endpoints.is_empty() {
                location_endpoint_to_route = endpoints[location_index % endpoints.len()].clone();
            }
        }

        location_endpoint_to_route
    }

    /// Determines if the account supports multiple write locations.
    ///
    /// # Summary
    /// Checks if the account is configured for multi-master writes by verifying that more than
    /// one write endpoint is available. Returns true only when multiple write regions exist,
    /// enabling write operations to be distributed across regions.
    ///
    /// # Returns
    /// `true` if multiple write endpoints are available, `false` otherwise
    pub fn can_use_multiple_write_locations(&self) -> bool {
        !self.write_endpoints().is_empty() && self.write_endpoints().iter().len() > 1
    }

    /// Returns all endpoints that could handle a specific request.
    ///
    /// # Summary
    /// Retrieves the list of endpoints applicable for the request based on operation type.
    /// Currently returns read endpoints for all requests. TODO: Fix to properly distinguish
    /// between read and write requests. Used by retry policies to determine available
    /// failover endpoints.
    ///
    /// # Arguments
    /// * `_request` - The Cosmos DB request (currently unused, pending fix)
    ///
    /// # Returns
    /// A vector of applicable endpoint URLs
    pub fn get_applicable_endpoints(&mut self, _request: &CosmosRequest) -> Vec<String> {
        //TODO: Fix this.
        self.get_preferred_available_endpoints(
            &self.locations_info.account_read_endpoints_by_location,
            RequestOperation::Read,
            &self.default_endpoint,
        )
    }

    /// Refreshes the ordered endpoint lists based on availability and preferences.
    ///
    /// # Summary
    /// Rebuilds the read and write endpoint lists by querying preferred locations against
    /// available endpoints from the account. Orders endpoints by preference with available
    /// endpoints first and unavailable endpoints last. Also removes stale unavailability
    /// entries older than 5 minutes. Called after marking endpoints unavailable or updating
    /// account regions.
    fn refresh_endpoints(&mut self) {
        // Get preferred available endpoints for write and read operations
        self.locations_info.write_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.account_write_endpoints_by_location,
            RequestOperation::Write,
            &self.default_endpoint,
        );

        self.locations_info.read_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.account_read_endpoints_by_location,
            RequestOperation::Read,
            &self.default_endpoint,
        );

        self.refresh_stale_endpoints();
    }

    /// Converts a list of regions into a location-to-endpoint map and region list.
    ///
    /// # Summary
    /// Processes account regions and creates a HashMap for quick lookup of endpoints by
    /// location name, along with preserving the region list for ordered access. Used during
    /// account property updates to organize regional endpoint information.
    ///
    /// # Arguments
    /// * `locations` - List of account regions to process
    ///
    /// # Returns
    /// A tuple of (HashMap<location_name, endpoint_url>, Vec<AccountRegion>)
    fn get_endpoints_by_location(
        &mut self,
        locations: Vec<AccountRegion>,
    ) -> (HashMap<String, String>, Vec<AccountRegion>) {
        // Separates locations into a hashmap and list
        let mut endpoints_by_location: HashMap<String, String> = HashMap::new();
        let mut parsed_locations: Vec<AccountRegion> = Vec::new();

        for location in locations {
            endpoints_by_location.insert(
                location.name.clone(),
                location.database_account_endpoint.clone(),
            );
            parsed_locations.push(location);
        }

        (endpoints_by_location, parsed_locations)
    }

    /// Builds an ordered list of endpoints based on preferences and availability.
    ///
    /// # Summary
    /// Creates a prioritized endpoint list by iterating through preferred locations and checking
    /// their availability. Available endpoints are placed first in order of preference, followed
    /// by unavailable endpoints (for eventual recovery), with the default endpoint as fallback
    /// if no preferred locations exist. This ordering determines request routing priority.
    ///
    /// # Arguments
    /// * `endpoints_by_location` - Map of location names to endpoint URLs
    /// * `request` - Operation type to check for endpoint availability
    /// * `default_endpoint` - Fallback endpoint if no preferred locations match
    ///
    /// # Returns
    /// An ordered vector of endpoint URLs (preferred available, preferred unavailable, default)
    fn get_preferred_available_endpoints(
        &self,
        endpoints_by_location: &HashMap<String, String>,
        request: RequestOperation,
        default_endpoint: &str,
    ) -> Vec<String> {
        let mut endpoints: Vec<String> = Vec::new();
        let mut unavailable_endpoints: Vec<String> = Vec::new();

        for location in &self.locations_info.preferred_locations {
            // Checks if preferred location exists in endpoints_by_location
            if let Some(endpoint) = endpoints_by_location.get(location.as_ref()) {
                // Check if endpoint is available, if not add to unavailable_endpoints
                // If it is then add to endpoints
                if !self.is_endpoint_unavailable(endpoint, request) {
                    endpoints.push(endpoint.clone());
                } else {
                    unavailable_endpoints.push(endpoint.clone());
                }
            }
        }

        // Add unavailable endpoints to end of endpoints lists
        for endpoint in unavailable_endpoints {
            endpoints.push(endpoint);
        }

        // If no preferred locations were found, use the default endpoint
        if endpoints.is_empty() {
            endpoints.push(default_endpoint.to_string());
        }

        endpoints
    }

    /// Removes stale endpoint unavailability entries older than 5 minutes.
    ///
    /// # Summary
    /// Cleans up the unavailability map by removing entries where more than 5 minutes have
    /// elapsed since the endpoint was marked unavailable. This allows endpoints to automatically
    /// recover and be considered available again after the expiration period, enabling retry
    /// attempts without manual intervention.
    fn refresh_stale_endpoints(&mut self) {
        let mut location_unavailability_info_map =
            self.location_unavailability_info_map.write().unwrap();

        // Removes endpoints that have not been checked in the last 5 minutes
        location_unavailability_info_map.retain(|_, info| {
            info.last_check_time.elapsed().unwrap_or_default() <= DEFAULT_EXPIRATION_TIME
        });
    }
}

// Tests for location cache
#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::resource_context::{ResourceLink, ResourceType};
    use std::{collections::HashSet, vec};

    fn create_test_data() -> (
        String,
        Vec<AccountRegion>,
        Vec<AccountRegion>,
        Vec<Cow<'static, str>>,
    ) {
        // Setting up test database account data
        let default_endpoint = "https://default.documents.example.com".to_string();

        let location_1 = AccountRegion {
            database_account_endpoint: "https://location1.documents.example.com".to_string(),
            name: "Location 1".to_string(),
        };
        let location_2 = AccountRegion {
            database_account_endpoint: "https://location2.documents.example.com".to_string(),
            name: "Location 2".to_string(),
        };
        let location_3 = AccountRegion {
            database_account_endpoint: "https://location3.documents.example.com".to_string(),
            name: "Location 3".to_string(),
        };
        let location_4 = AccountRegion {
            database_account_endpoint: "https://location4.documents.example.com".to_string(),
            name: "Location 4".to_string(),
        };
        let write_locations = Vec::from([location_1.clone(), location_2.clone()]);

        let read_locations = Vec::from([location_1, location_2, location_3, location_4]);

        let preferred_locations: Vec<Cow<'static, str>> =
            vec![Cow::Borrowed("Location 1"), Cow::Borrowed("Location 2")];

        (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
        )
    }

    fn create_test_location_cache() -> LocationCache {
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);
        cache.update(write_locations, read_locations).unwrap();
        cache
    }

    #[test]
    fn location_cache_update() {
        // this test also checks refresh_endpoints, get_endpoints_by_location, and get_preferred_available_endpoints methods
        // set up test data
        let cache = create_test_location_cache();

        assert_eq!(
            cache.default_endpoint,
            "https://default.documents.example.com"
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec!["Location 1".to_string(), "Location 2".to_string()]
        );

        // check available write locations
        let actual_account_write_locations: HashSet<_> = cache
            .locations_info
            .account_write_locations
            .iter()
            .cloned()
            .map(|account_region| account_region.name)
            .collect();
        let expected_account_write_locations: HashSet<String> = ["Location 1", "Location 2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            actual_account_write_locations,
            expected_account_write_locations
        );

        // check available read locations
        let actual_account_read_locations: HashSet<_> = cache
            .locations_info
            .account_read_locations
            .iter()
            .cloned()
            .map(|account_region| account_region.name)
            .collect();
        let expected_account_read_locations: HashSet<String> =
            ["Location 1", "Location 2", "Location 3", "Location 4"]
                .iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(
            actual_account_read_locations,
            expected_account_read_locations
        );

        assert_eq!(
            cache.locations_info.account_write_endpoints_by_location,
            HashMap::from([
                (
                    "Location 1".to_string(),
                    "https://location1.documents.example.com".to_string()
                ),
                (
                    "Location 2".to_string(),
                    "https://location2.documents.example.com".to_string()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.account_read_endpoints_by_location,
            HashMap::from([
                (
                    "Location 1".to_string(),
                    "https://location1.documents.example.com".to_string()
                ),
                (
                    "Location 2".to_string(),
                    "https://location2.documents.example.com".to_string()
                ),
                (
                    "Location 3".to_string(),
                    "https://location3.documents.example.com".to_string()
                ),
                (
                    "Location 4".to_string(),
                    "https://location4.documents.example.com".to_string()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec![
                "https://location1.documents.example.com".to_string(),
                "https://location2.documents.example.com".to_string()
            ]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec![
                "https://location1.documents.example.com".to_string(),
                "https://location2.documents.example.com".to_string(),
            ]
        );
    }

    #[test]
    fn location_cache_update_with_one_preferred_location() {
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(
            default_endpoint.to_string(),
            vec![preferred_locations[0].clone()],
        );

        let _ = cache.update(write_locations, read_locations);

        assert_eq!(
            cache.default_endpoint,
            "https://default.documents.example.com"
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec![preferred_locations[0].clone()]
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec!["https://location1.documents.example.com".to_string()]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec!["https://location1.documents.example.com".to_string()]
        );
    }

    #[test]
    fn mark_read_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        // mark location 1 as unavailable endpoint for read operation
        let unavailable_endpoint = "https://location1.documents.example.com";
        let operation = RequestOperation::Read;
        cache.mark_endpoint_unavailable(unavailable_endpoint, operation);

        // check that endpoint is last option in read endpoints and it is in the location unavailability info map
        assert_eq!(
            cache.locations_info.read_endpoints,
            vec![
                "https://location2.documents.example.com".to_string(),
                unavailable_endpoint.to_string()
            ]
        );

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(&cosmos_request),
            "https://location2.documents.example.com".to_string()
        );
    }

    #[test]
    fn mark_write_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        // mark location 1 as unavailable endpoint for write operation
        let unavailable_endpoint = "https://location1.documents.example.com";
        let operation = RequestOperation::Write;
        cache.mark_endpoint_unavailable(unavailable_endpoint, operation);

        // check that endpoint is last option in write endpoints, and it is in the location unavailability info map
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&unavailable_endpoint.to_string())
        );

        let builder = CosmosRequest::builder(
            OperationType::Create,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(&cosmos_request),
            "https://location2.documents.example.com".to_string()
        );
    }

    #[test]
    fn mark_same_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        let endpoint1 = "https://location1.documents.example.com";

        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Write);

        let before_marked_unavailable_time = SystemTime::now() - Duration::from_secs(10);

        {
            let mut unavailability_map = cache.location_unavailability_info_map.write().unwrap();
            if let Some(info) = unavailability_map.get_mut(endpoint1) {
                info.last_check_time = before_marked_unavailable_time;
            }
        }

        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Write);

        assert!(
            cache
                .location_unavailability_info_map
                .read()
                .unwrap()
                .get(endpoint1)
                .map(|info| info.last_check_time)
                > Some(before_marked_unavailable_time)
        );

        assert_eq!(
            cache
                .location_unavailability_info_map
                .read()
                .unwrap()
                .get(endpoint1)
                .map(|info| info.unavailable_operation),
            Some(RequestOperation::All)
        );
    }

    #[test]
    fn refresh_stale_endpoints() {
        // create test cache
        let mut cache = create_test_location_cache();

        // mark endpoint 1 and endpoint 2 as unavailable
        let endpoint1 = "https://location1.documents.example.com";
        let endpoint2 = "https://location2.documents.example.com";
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(endpoint2, RequestOperation::Read);

        // simulate stale entry
        {
            let mut unavailability_map = cache.location_unavailability_info_map.write().unwrap();
            if let Some(info) = unavailability_map.get_mut(endpoint1) {
                info.last_check_time = SystemTime::now() - Duration::from_secs(500);
            }
        }

        // refresh stale endpoints
        cache.refresh_stale_endpoints();

        // check that endpoint 1 is marked as available again
        assert!(!cache.is_endpoint_unavailable(endpoint1, RequestOperation::Read));
    }

    #[test]
    fn resolve_service_endpoint() {
        // create test cache
        let cache = create_test_location_cache();

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        // resolve service endpoint
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            "https://location1.documents.example.com".to_string()
        );
    }

    #[test]
    fn resolve_service_endpoint_second_location() {
        // create test cache
        let endpoint1 = "https://location1.documents.example.com";
        let mut cache = create_test_location_cache();
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        // resolve service endpoint for second location
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            "https://location2.documents.example.com".to_string()
        );
    }
}
