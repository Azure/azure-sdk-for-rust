// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use crate::cosmos_request::CosmosRequest;
use crate::models::{AccountProperties, AccountRegion};
use crate::operation_context::OperationType;
use crate::regions::RegionName;
use crate::resource_context::ResourceType;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
    time::{Duration, SystemTime},
};
use tracing::info;
use url::Url;

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
pub(crate) struct DatabaseAccountLocationsInfo {
    /// User-specified preferred Azure regions for request routing
    pub preferred_locations: Vec<RegionName>,
    /// List of regions where write operations are supported
    pub account_write_locations: Vec<AccountRegion>,
    /// List of regions where read operations are supported
    pub account_read_locations: Vec<AccountRegion>,
    /// Map from location name to write endpoint URL
    pub account_write_endpoints_by_location: HashMap<RegionName, Url>,
    /// Map from location name to read endpoint URL
    pub(crate) account_read_endpoints_by_location: HashMap<RegionName, Url>,
    /// Ordered list of available write endpoint URLs (preferred first, unavailable last)
    pub write_endpoints: Vec<Url>,
    /// Ordered list of available read endpoint URLs
    pub read_endpoints: Vec<Url>,
}

/// Tracks when an endpoint was marked unavailable and for which operations.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LocationUnavailabilityInfo {
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
#[derive(Debug)]
pub(crate) struct LocationCache {
    /// The primary default endpoint URL for the Cosmos DB account
    pub default_endpoint: Url,
    /// Location and endpoint information including preferred regions and available endpoints
    pub locations_info: DatabaseAccountLocationsInfo,
    /// Thread-safe map tracking unavailable endpoints and when they were last checked
    pub location_unavailability_info_map: RwLock<HashMap<Url, LocationUnavailabilityInfo>>,
    /// Client level excluded regions. Empty if no regions are excluded.
    pub client_excluded_regions: Vec<RegionName>,
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
    /// * `excluded_regions` - List of regions to exclude from routing
    ///
    /// # Returns
    /// A new `LocationCache` instance ready for endpoint management
    pub fn new(
        default_endpoint: Url,
        preferred_locations: Vec<RegionName>,
        excluded_regions: Vec<RegionName>,
    ) -> Self {
        Self {
            default_endpoint,
            locations_info: DatabaseAccountLocationsInfo {
                preferred_locations,
                ..Default::default()
            },
            location_unavailability_info_map: RwLock::new(HashMap::new()),
            client_excluded_regions: excluded_regions,
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
    pub fn read_endpoints(&self) -> &[Url] {
        &self.locations_info.read_endpoints
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
    pub fn write_endpoints(&self) -> &[Url] {
        &self.locations_info.write_endpoints
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
        // Build effective preferred locations: preferred regions first, then remaining account regions
        let mut effective_preferred_locations = self.locations_info.preferred_locations.clone();

        // Use HashSet for O(1) lookups instead of O(n) linear search
        let existing: HashSet<RegionName> = effective_preferred_locations.iter().cloned().collect();

        // Extend with read locations not already in preferred locations - O(n)
        for location in &read_locations {
            if !existing.contains(&location.name) {
                effective_preferred_locations.push(location.name.clone());
            }
        }

        self.locations_info.preferred_locations = effective_preferred_locations;

        // Clear existing endpoint lists before repopulating to prevent
        // duplicate accumulation across repeated update() calls (e.g.
        // periodic account-property refreshes).
        self.locations_info.write_endpoints.clear();
        self.locations_info.read_endpoints.clear();

        // Separate write locations into appropriate hashmap and list
        if !write_locations.is_empty() {
            let (account_write_endpoints_by_location, account_write_locations) =
                self.get_endpoints_by_location(write_locations, true);
            self.locations_info.account_write_endpoints_by_location =
                account_write_endpoints_by_location;
            self.locations_info.account_write_locations = account_write_locations;
        }

        // Separate read locations into appropriate hashmap and list
        if !read_locations.is_empty() {
            let (account_read_endpoints_by_location, account_read_locations) =
                self.get_endpoints_by_location(read_locations, false);
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
    pub fn mark_endpoint_unavailable(&mut self, endpoint: &Url, operation: RequestOperation) {
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
                    endpoint.clone(),
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
    pub fn is_endpoint_unavailable(&self, endpoint: &Url, operation: RequestOperation) -> bool {
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
    pub fn resolve_service_endpoint(&self, request: &CosmosRequest) -> Url {
        // Returns service endpoint based on index, if index out of bounds or operation not supported, returns default endpoint
        let location_index = request.request_context.location_index_to_route.unwrap_or(0);
        let mut location_endpoint_to_route = None;
        if !request
            .request_context
            .use_preferred_locations
            .unwrap_or(true)
            || (!request.operation_type.is_read_only()
                && !self.can_support_multiple_write_locations(
                    request.resource_type,
                    request.operation_type,
                ))
        {
            let location_info = &self.locations_info;
            if !location_info.account_write_locations.is_empty() {
                let idx = (location_index) % location_info.account_write_locations.len();
                location_endpoint_to_route = Some(
                    location_info.account_write_locations[idx]
                        .database_account_endpoint
                        .clone(),
                );
            }
        } else {
            let endpoints = self.get_applicable_endpoints(
                request.operation_type,
                request.excluded_regions.as_ref(),
            );

            if !endpoints.is_empty() {
                location_endpoint_to_route =
                    Some(endpoints[location_index % endpoints.len()].clone());
            }
        }

        let endpoint = location_endpoint_to_route.unwrap_or(self.default_endpoint.clone());

        tracing::trace!(
            operation_type = ?request.operation_type,
            resource_link = %request.resource_link,
            ?endpoint,
            "resolved service endpoint"
        );

        endpoint
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
        let endpoints = self.write_endpoints();
        !endpoints.is_empty() && endpoints.len() > 1
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
        self.can_use_multiple_write_locations()
            && (resource_type == ResourceType::Documents
                || (resource_type == ResourceType::StoredProcedures
                    && operation_type == OperationType::Execute))
    }

    /// Returns all endpoints that could handle a specific request.
    ///
    /// # Summary
    /// Retrieves the list of endpoints applicable for the request based on operation type.
    /// Returns read endpoints for read-only operations and write endpoints for write operations.
    /// Used by retry policies to determine available failover endpoints.
    ///
    /// # Arguments
    /// * `operation_type` - The type of Cosmos DB operation
    ///
    /// # Returns
    /// A vector of applicable endpoint URLs
    pub fn get_applicable_endpoints(
        &self,
        operation_type: OperationType,
        excluded_regions: Option<&Vec<RegionName>>,
    ) -> Vec<Url> {
        // Select endpoints based on operation type.
        if operation_type.is_read_only() {
            self.get_preferred_available_endpoints(
                &self.locations_info.account_read_endpoints_by_location,
                RequestOperation::Read,
                &self.default_endpoint,
                excluded_regions,
            )
        } else {
            self.get_preferred_available_endpoints(
                &self.locations_info.account_write_endpoints_by_location,
                RequestOperation::Write,
                &self.default_endpoint,
                excluded_regions,
            )
        }
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
        // Rebuild the preferred-and-availability-ordered endpoint lists so
        // that unavailable endpoints are pushed to the back.  Without this,
        // mark_endpoint_unavailable has no effect on routing order.
        self.locations_info.write_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.account_write_endpoints_by_location,
            RequestOperation::Write,
            &self.default_endpoint,
            None,
        );

        self.locations_info.read_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.account_read_endpoints_by_location,
            RequestOperation::Read,
            &self.default_endpoint,
            None,
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
        is_write: bool,
    ) -> (HashMap<RegionName, Url>, Vec<AccountRegion>) {
        // Separates locations into a hashmap and list
        let mut endpoints_by_location = HashMap::new();
        let mut parsed_locations = Vec::new();

        for location in locations {
            endpoints_by_location.insert(
                location.name.clone(),
                location.database_account_endpoint.clone(),
            );
            if is_write {
                self.locations_info
                    .write_endpoints
                    .push(location.database_account_endpoint.clone());
            } else {
                self.locations_info
                    .read_endpoints
                    .push(location.database_account_endpoint.clone());
            }
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
        endpoints_by_location: &HashMap<RegionName, Url>,
        request: RequestOperation,
        default_endpoint: &Url,
        request_excluded_regions: Option<&Vec<RegionName>>,
    ) -> Vec<Url> {
        let mut endpoints = Vec::new();
        let mut unavailable_endpoints = Vec::new();
        let mut effective_preferred_locations = self.locations_info.preferred_locations.clone();
        // Remove excluded regions from effective preferred locations
        let excluded_regions = request_excluded_regions
            .cloned()
            .unwrap_or_else(|| self.client_excluded_regions.clone());
        effective_preferred_locations
            .retain(|location| !excluded_regions.iter().any(|excluded| excluded == location));

        for location in &effective_preferred_locations {
            // Checks if preferred location exists in endpoints_by_location
            if let Some(endpoint) = endpoints_by_location.get(location) {
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
            endpoints.push(default_endpoint.clone());
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
    use crate::region_proximity::generate_preferred_region_list;
    use crate::regions;
    use crate::resource_context::{ResourceLink, ResourceType};
    use std::{collections::HashSet, vec};

    type TestData = (
        Url,
        Vec<AccountRegion>,
        Vec<AccountRegion>,
        Vec<RegionName>,
        Vec<RegionName>,
    );

    fn create_test_data() -> TestData {
        // Setting up test database account data
        let default_endpoint = "https://default.documents.example.com".parse().unwrap();

        let location_1 = AccountRegion {
            database_account_endpoint: "https://location1.documents.example.com".parse().unwrap(),
            name: RegionName::from("Location 1"),
        };
        let location_2 = AccountRegion {
            database_account_endpoint: "https://location2.documents.example.com".parse().unwrap(),
            name: RegionName::from("Location 2"),
        };
        let location_3 = AccountRegion {
            database_account_endpoint: "https://location3.documents.example.com".parse().unwrap(),
            name: RegionName::from("Location 3"),
        };
        let location_4 = AccountRegion {
            database_account_endpoint: "https://location4.documents.example.com".parse().unwrap(),
            name: RegionName::from("Location 4"),
        };
        let write_locations = Vec::from([location_1.clone(), location_2.clone()]);

        let read_locations = Vec::from([location_1, location_2, location_3, location_4]);

        let preferred_locations: Vec<RegionName> = vec![
            RegionName::from("Location 1"),
            RegionName::from("Location 2"),
        ];
        let excluded_regions: Vec<RegionName> = vec![];

        (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        )
    }

    fn create_test_location_cache() -> LocationCache {
        let (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        ) = create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations, excluded_regions);
        cache.update(write_locations, read_locations).unwrap();
        cache
    }

    fn create_custom_test_location_cache(
        pref_regions: Option<Vec<String>>,
        excl_regions: Option<Vec<String>>,
    ) -> LocationCache {
        let (
            default_endpoint,
            write_locations,
            read_locations,
            mut preferred_locations,
            mut excluded_regions,
        ) = create_test_data();
        if let Some(regions) = pref_regions {
            preferred_locations = regions.into_iter().map(RegionName::from).collect();
        }
        if let Some(regions) = excl_regions {
            excluded_regions = regions.into_iter().map(RegionName::from).collect();
        }

        let mut cache = LocationCache::new(default_endpoint, preferred_locations, excluded_regions);
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
            Url::parse("https://default.documents.example.com").unwrap()
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec![
                RegionName::from("Location 1"),
                RegionName::from("Location 2"),
                RegionName::from("Location 3"),
                RegionName::from("Location 4")
            ]
        );

        // check available write locations
        let actual_account_write_locations: HashSet<_> = cache
            .locations_info
            .account_write_locations
            .iter()
            .cloned()
            .map(|account_region| account_region.name)
            .collect();
        let expected_account_write_locations: HashSet<RegionName> = ["Location 1", "Location 2"]
            .iter()
            .map(|s| RegionName::from(*s))
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
        let expected_account_read_locations: HashSet<RegionName> =
            ["Location 1", "Location 2", "Location 3", "Location 4"]
                .iter()
                .map(|s| RegionName::from(*s))
                .collect();

        assert_eq!(
            actual_account_read_locations,
            expected_account_read_locations
        );

        assert_eq!(
            cache.locations_info.account_write_endpoints_by_location,
            HashMap::from([
                (
                    RegionName::from("Location 1"),
                    Url::parse("https://location1.documents.example.com").unwrap()
                ),
                (
                    RegionName::from("Location 2"),
                    Url::parse("https://location2.documents.example.com").unwrap()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.account_read_endpoints_by_location,
            HashMap::from([
                (
                    RegionName::from("Location 1"),
                    Url::parse("https://location1.documents.example.com").unwrap()
                ),
                (
                    RegionName::from("Location 2"),
                    Url::parse("https://location2.documents.example.com").unwrap()
                ),
                (
                    RegionName::from("Location 3"),
                    Url::parse("https://location3.documents.example.com").unwrap()
                ),
                (
                    RegionName::from("Location 4"),
                    Url::parse("https://location4.documents.example.com").unwrap()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec![
                "https://location1.documents.example.com".parse().unwrap(),
                "https://location2.documents.example.com".parse().unwrap()
            ]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec![
                "https://location1.documents.example.com".parse().unwrap(),
                "https://location2.documents.example.com".parse().unwrap(),
                "https://location3.documents.example.com".parse().unwrap(),
                "https://location4.documents.example.com".parse().unwrap(),
            ]
        );
    }

    #[test]
    fn location_cache_update_with_one_preferred_location() {
        let (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        ) = create_test_data();

        let mut cache = LocationCache::new(
            default_endpoint,
            vec![preferred_locations[0].clone()],
            excluded_regions,
        );

        let _ = cache.update(write_locations, read_locations);

        assert_eq!(
            cache.default_endpoint,
            Url::parse("https://default.documents.example.com").unwrap()
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec![
                preferred_locations[0].clone(),
                RegionName::from("Location 2"),
                RegionName::from("Location 3"),
                RegionName::from("Location 4")
            ]
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec![
                Url::parse("https://location1.documents.example.com").unwrap(),
                Url::parse("https://location2.documents.example.com").unwrap()
            ]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec![
                Url::parse("https://location1.documents.example.com").unwrap(),
                Url::parse("https://location2.documents.example.com").unwrap(),
                Url::parse("https://location3.documents.example.com").unwrap(),
                Url::parse("https://location4.documents.example.com").unwrap()
            ]
        );
    }

    #[test]
    fn mark_read_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        // mark location 1 as unavailable endpoint for read operation
        let unavailable_endpoint = "https://location1.documents.example.com".parse().unwrap();
        let operation = RequestOperation::Read;
        cache.mark_endpoint_unavailable(&unavailable_endpoint, operation);

        // check that endpoint is last option in read endpoints, and it is in the location unavailability info map
        assert_eq!(
            cache
                .get_applicable_endpoints(OperationType::Read, None)
                .last(),
            Some(&unavailable_endpoint)
        );

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        assert!(cache.is_endpoint_unavailable(&unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(&cosmos_request),
            Url::parse("https://location2.documents.example.com").unwrap()
        );
    }

    #[test]
    fn mark_write_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        // mark location 1 as unavailable endpoint for write operation
        let unavailable_endpoint = "https://location1.documents.example.com".parse().unwrap();
        let operation = RequestOperation::Write;
        cache.mark_endpoint_unavailable(&unavailable_endpoint, operation);

        // check that endpoint is last option in write endpoints, and it is in the location unavailability info map
        assert_eq!(
            cache
                .get_applicable_endpoints(OperationType::Create, None)
                .last(),
            Some(&unavailable_endpoint)
        );

        let builder = CosmosRequest::builder(
            OperationType::Create,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        assert!(cache.is_endpoint_unavailable(&unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(&cosmos_request),
            Url::parse("https://location2.documents.example.com").unwrap()
        );
    }

    #[test]
    fn mark_same_endpoint_unavailable() {
        // set up test cache
        let mut cache = create_test_location_cache();

        let endpoint1 = "https://location1.documents.example.com".parse().unwrap();

        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Write);

        let before_marked_unavailable_time = SystemTime::now() - Duration::from_secs(10);

        {
            let mut unavailability_map = cache.location_unavailability_info_map.write().unwrap();
            if let Some(info) = unavailability_map.get_mut(&endpoint1) {
                info.last_check_time = before_marked_unavailable_time;
            }
        }

        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Write);

        assert!(
            cache
                .location_unavailability_info_map
                .read()
                .unwrap()
                .get(&endpoint1)
                .map(|info| info.last_check_time)
                > Some(before_marked_unavailable_time)
        );

        assert_eq!(
            cache
                .location_unavailability_info_map
                .read()
                .unwrap()
                .get(&endpoint1)
                .map(|info| info.unavailable_operation),
            Some(RequestOperation::All)
        );
    }

    #[test]
    fn refresh_stale_endpoints() {
        // create test cache
        let mut cache = create_test_location_cache();

        // mark endpoint 1 and endpoint 2 as unavailable
        let endpoint1 = "https://location1.documents.example.com".parse().unwrap();
        let endpoint2 = "https://location2.documents.example.com".parse().unwrap();
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(&endpoint2, RequestOperation::Read);

        // simulate stale entry
        {
            let mut unavailability_map = cache.location_unavailability_info_map.write().unwrap();
            if let Some(info) = unavailability_map.get_mut(&endpoint1) {
                info.last_check_time = SystemTime::now() - Duration::from_secs(500);
            }
        }

        // refresh stale endpoints
        cache.refresh_stale_endpoints();

        // check that endpoint 1 is marked as available again
        assert!(!cache.is_endpoint_unavailable(&endpoint1, RequestOperation::Read));
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
            Url::parse("https://location1.documents.example.com").unwrap()
        );
    }

    #[test]
    fn resolve_service_endpoint_second_location() {
        // create test cache
        let endpoint1 = "https://location1.documents.example.com".parse().unwrap();
        let mut cache = create_test_location_cache();
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Read);

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        // resolve service endpoint for second location
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location2.documents.example.com").unwrap()
        );
    }

    #[test]
    fn resolve_service_endpoint_request_excluded_regions() {
        let pref_regions: Vec<String> = vec![
            "Location 4".to_string(),
            "Location 3".to_string(),
            "Location 2".to_string(),
            "Location 1".to_string(),
        ];
        // create test cache
        let cache = create_custom_test_location_cache(Some(pref_regions), Some(vec![]));

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder
            .clone()
            .excluded_regions(Some(vec!["Location 4".into()]))
            .build()
            .ok()
            .unwrap();

        // resolve service endpoint - should skip Location 4 and go to Location 3
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location3.documents.example.com").unwrap()
        );

        let cosmos_request = builder
            .excluded_regions(Some(vec![
                "Location 4".into(),
                "Location 3".into(),
                "Location 2".into(),
                "Location 1".into(),
            ]))
            .build()
            .ok()
            .unwrap();

        // resolve service endpoint - should skip all preferred locations and go to default endpoint
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://default.documents.example.com").unwrap()
        );
    }

    #[test]
    fn resolve_service_endpoint_client_excluded_regions() {
        let pref_regions: Vec<String> = vec![
            "Location 4".to_string(),
            "Location 3".to_string(),
            "Location 2".to_string(),
            "Location 1".to_string(),
        ];
        let excl_regions: Vec<String> = vec!["Location 4".to_string()];
        // create test cache
        let cache = create_custom_test_location_cache(Some(pref_regions), Some(excl_regions));

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.build().ok().unwrap();

        // resolve service endpoint - should skip Location 4 and go to Location 3
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location3.documents.example.com").unwrap()
        );
    }

    #[test]
    fn resolve_service_endpoint_excluded_regions_precedence() {
        // verify that request excluded regions take precedence over client excluded regions
        let pref_regions: Vec<String> = vec![
            "Location 4".to_string(),
            "Location 3".to_string(),
            "Location 2".to_string(),
            "Location 1".to_string(),
        ];
        let excl_regions: Vec<String> = vec!["Location 4".to_string()];
        // create test cache
        let cache = create_custom_test_location_cache(Some(pref_regions), Some(excl_regions));

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder
            .clone()
            .excluded_regions(Some(vec!["Location 3".into()]))
            .build()
            .ok()
            .unwrap();

        // resolve service endpoint - should use request excluded regions and only exclude Location 3
        // routing to Location 4 (most preferred region) even if excluded in client excluded regions
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location4.documents.example.com").unwrap()
        );

        // if setting None in request excluded regions, should use client excluded regions
        let cosmos_request = builder.clone().excluded_regions(None).build().ok().unwrap();

        // resolve service endpoint - should skip Location 4 and go to Location 3
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location3.documents.example.com").unwrap()
        );

        // if setting an empty list in request excluded regions, no regions should be excluded
        let cosmos_request = builder.excluded_regions(Some(vec![])).build().ok().unwrap();

        // resolve service endpoint - should not exclude any regions and go to Location 4
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location4.documents.example.com").unwrap()
        );
    }

    #[test]
    fn resolve_service_endpoint_no_preferred_regions() {
        // set no preferred regions in cache, so all regions from account are used
        let pref_regions: Vec<String> = vec![];
        let excl_regions: Vec<String> = vec![];
        // create test cache
        let cache = create_custom_test_location_cache(Some(pref_regions), Some(excl_regions));

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.clone().build().ok().unwrap();

        // resolve service endpoint - should go to first region on the list, which is Location 1
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location1.documents.example.com").unwrap()
        );

        let cosmos_request = builder
            .excluded_regions(Some(vec!["Location 1".into()]))
            .build()
            .ok()
            .unwrap();

        // resolve service endpoint - should skip Location 1 and go to Location 2
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location2.documents.example.com").unwrap()
        );
    }

    #[test]
    fn repeated_update_does_not_duplicate_write_endpoints() {
        // Bug 1: Calling update() multiple times should NOT accumulate
        // duplicate entries in write_endpoints.
        let (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        ) = create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations, excluded_regions);

        // First update — 2 write endpoints expected
        cache
            .update(write_locations.clone(), read_locations.clone())
            .unwrap();
        assert_eq!(cache.locations_info.write_endpoints.len(), 2);

        // Second update (simulates periodic account-property refresh)
        cache
            .update(write_locations.clone(), read_locations.clone())
            .unwrap();
        assert_eq!(
            cache.locations_info.write_endpoints.len(),
            2,
            "write_endpoints should not grow after a second update()"
        );

        // Third update — still 2
        cache.update(write_locations, read_locations).unwrap();
        assert_eq!(
            cache.locations_info.write_endpoints.len(),
            2,
            "write_endpoints should not grow after a third update()"
        );
    }

    #[test]
    fn repeated_update_does_not_duplicate_read_endpoints() {
        // Bug 1: Calling update() multiple times should NOT accumulate
        // duplicate entries in read_endpoints.
        let (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        ) = create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations, excluded_regions);

        // First update — 4 read endpoints expected
        cache
            .update(write_locations.clone(), read_locations.clone())
            .unwrap();
        assert_eq!(cache.locations_info.read_endpoints.len(), 4);

        // Second update
        cache
            .update(write_locations.clone(), read_locations.clone())
            .unwrap();
        assert_eq!(
            cache.locations_info.read_endpoints.len(),
            4,
            "read_endpoints should not grow after a second update()"
        );

        // Third update — still 4
        cache.update(write_locations, read_locations).unwrap();
        assert_eq!(
            cache.locations_info.read_endpoints.len(),
            4,
            "read_endpoints should not grow after a third update()"
        );
    }

    #[test]
    fn single_write_region_stays_single_after_repeated_updates() {
        // Bug 3: A single-master account (1 write region) must keep
        // can_use_multiple_write_locations() == false even after many update() calls.
        let default_endpoint: Url = "https://default.documents.example.com".parse().unwrap();

        let single_write = vec![AccountRegion {
            database_account_endpoint: "https://location1.documents.example.com".parse().unwrap(),
            name: RegionName::from("Location 1"),
        }];

        let read_locations = vec![
            AccountRegion {
                database_account_endpoint: "https://location1.documents.example.com"
                    .parse()
                    .unwrap(),
                name: RegionName::from("Location 1"),
            },
            AccountRegion {
                database_account_endpoint: "https://location2.documents.example.com"
                    .parse()
                    .unwrap(),
                name: RegionName::from("Location 2"),
            },
        ];

        let preferred = vec![
            RegionName::from("Location 1"),
            RegionName::from("Location 2"),
        ];

        let mut cache = LocationCache::new(default_endpoint, preferred, vec![]);

        for i in 0..10 {
            cache
                .update(single_write.clone(), read_locations.clone())
                .unwrap();
            assert_eq!(
                cache.locations_info.write_endpoints.len(),
                1,
                "write_endpoints should stay at 1 after update #{}",
                i + 1
            );
            assert!(
                !cache.can_use_multiple_write_locations(),
                "single-master must report can_use_multiple_write_locations() == false after update #{}", i + 1
            );
        }
    }

    #[test]
    fn mark_write_endpoint_unavailable_reorders_after_refresh() {
        // Bug 2: mark_endpoint_unavailable must cause refresh_endpoints to
        // push the unavailable endpoint to the back of write_endpoints.
        let mut cache = create_test_location_cache();

        let endpoint1: Url = "https://location1.documents.example.com".parse().unwrap();
        let endpoint2: Url = "https://location2.documents.example.com".parse().unwrap();

        // Before marking: endpoint1 is first
        assert_eq!(cache.locations_info.write_endpoints[0], endpoint1);

        // Mark endpoint1 unavailable for writes
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Write);

        // After marking: endpoint2 should be first, endpoint1 last
        assert_eq!(
            cache.locations_info.write_endpoints.first(),
            Some(&endpoint2),
            "available endpoint should be first after marking endpoint1 unavailable"
        );
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&endpoint1),
            "unavailable endpoint should be last after marking endpoint1 unavailable"
        );
    }

    #[test]
    fn mark_read_endpoint_unavailable_reorders_after_refresh() {
        // Bug 2: mark_endpoint_unavailable must cause refresh_endpoints to
        // push the unavailable endpoint to the back of read_endpoints.
        let mut cache = create_test_location_cache();

        let endpoint1: Url = "https://location1.documents.example.com".parse().unwrap();

        // Before marking: endpoint1 is first
        assert_eq!(cache.locations_info.read_endpoints[0], endpoint1);

        // Mark endpoint1 unavailable for reads
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Read);

        // After marking: endpoint1 should be last in read_endpoints
        assert_ne!(
            cache.locations_info.read_endpoints.first(),
            Some(&endpoint1),
            "unavailable endpoint should not be first after marking"
        );
        assert_eq!(
            cache.locations_info.read_endpoints.last(),
            Some(&endpoint1),
            "unavailable endpoint should be last after marking"
        );
    }

    #[test]
    fn mark_unavailable_then_update_preserves_correct_count() {
        // Combined scenario: mark an endpoint unavailable, then call update()
        // again (simulating a refresh cycle). Endpoints should not duplicate
        // and the unavailable endpoint should still be ordered last.
        let (
            default_endpoint,
            write_locations,
            read_locations,
            preferred_locations,
            excluded_regions,
        ) = create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations, excluded_regions);
        cache
            .update(write_locations.clone(), read_locations.clone())
            .unwrap();

        let endpoint1: Url = "https://location1.documents.example.com".parse().unwrap();
        let endpoint2: Url = "https://location2.documents.example.com".parse().unwrap();
        cache.mark_endpoint_unavailable(&endpoint1, RequestOperation::Write);

        // Endpoint count should still be 2, with unavailable endpoint last
        assert_eq!(cache.locations_info.write_endpoints.len(), 2);
        assert_eq!(
            cache.locations_info.write_endpoints.first(),
            Some(&endpoint2),
            "available endpoint should be first after marking endpoint1 unavailable"
        );
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&endpoint1),
            "unavailable endpoint should be last after marking endpoint1 unavailable"
        );

        // Now simulate an account-property refresh
        cache.update(write_locations, read_locations).unwrap();

        // Endpoint count should still be 2 (no duplicates)
        assert_eq!(
            cache.locations_info.write_endpoints.len(),
            2,
            "write_endpoints should not accumulate after update following mark_unavailable"
        );

        // The unavailable endpoint should still be ordered last after the update()
        // because refresh_endpoints() rebuilds lists with availability ordering.
        assert_eq!(
            cache.locations_info.write_endpoints.first(),
            Some(&endpoint2),
            "available endpoint should still be first after update()"
        );
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&endpoint1),
            "previously-unavailable endpoint should still be last after update()"
        );

        // Also verify via get_applicable_endpoints (the public routing API)
        let applicable = cache.get_applicable_endpoints(OperationType::Create, None);
        assert_eq!(
            applicable.first(),
            Some(&endpoint2),
            "get_applicable_endpoints should route to available endpoint first"
        );
        assert_eq!(
            applicable.last(),
            Some(&endpoint1),
            "get_applicable_endpoints should place unavailable endpoint last"
        );
    }

    #[test]
    fn resolve_service_endpoint_effective_preferred_regions() {
        // effective preferred regions should be ordered as preferred regions + remaining regions from account - excluded regions
        // normal region order is Location 1, Location 2, Location 3, Location 4
        let pref_regions: Vec<String> = vec!["Location 4".to_string(), "Location 3".to_string()];
        let excl_regions: Vec<String> = vec![];
        // create test cache
        let cache = create_custom_test_location_cache(Some(pref_regions), Some(excl_regions));

        let builder = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        );

        let cosmos_request = builder.clone().build().ok().unwrap();

        // resolve service endpoint - should go to first region on preferred list, which is Location 4
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location4.documents.example.com").unwrap()
        );

        let cosmos_request = builder
            .excluded_regions(Some(vec!["Location 4".into(), "Location 3".into()]))
            .build()
            .ok()
            .unwrap();

        // resolve service endpoint - should skip Location 4 and Location 3 and go to Location 1
        let endpoint = cache.resolve_service_endpoint(&cosmos_request);
        assert_eq!(
            endpoint,
            Url::parse("https://location1.documents.example.com").unwrap()
        );
    }

    /// Verifies that when preferred regions come from the proximity table (many regions),
    /// the LocationCache correctly filters them to only account-present regions while
    /// preserving the proximity ordering.
    #[test]
    fn preferred_regions_filtered_to_account_regions_in_proximity_order() {
        // Simulate application_region = EAST_US → generates ~96 proximity-sorted regions
        let proximity_list = generate_preferred_region_list(&regions::EAST_US)
            .expect("EAST_US should be a known region")
            .to_vec();

        // Account only has these 3 regions (subset of the 96)
        let account_regions = vec![
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com:443/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com:443/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_EUROPE.clone(),
                database_account_endpoint: "https://test-westeurope.documents.azure.com:443/"
                    .parse()
                    .unwrap(),
            },
        ];

        let default_endpoint: Url = "https://test.documents.azure.com:443/".parse().unwrap();

        let mut cache = LocationCache::new(
            default_endpoint.clone(),
            proximity_list,
            vec![], // no excluded regions
        );
        // Feed account properties (writable = readable for simplicity)
        cache
            .update(account_regions.clone(), account_regions)
            .unwrap();

        let read_endpoints = cache.read_endpoints();

        // Only the 3 account-present regions should appear in the read endpoints,
        // in proximity order: from EAST_US, East US 2 is closest, then West US, then West Europe.
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-eastus2.documents.azure.com/",
                "https://test-westus.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
            "endpoints should be in proximity order from East US"
        );
    }

    /// Verifies that write endpoints also respect proximity ordering from
    /// the preferred regions list.
    #[test]
    fn write_endpoints_respect_proximity_order() {
        let proximity_list = generate_preferred_region_list(&regions::WEST_EUROPE)
            .expect("WEST_EUROPE should be a known region")
            .to_vec();

        // Account has 2 write regions
        let write_regions = vec![
            AccountRegion {
                name: regions::EAST_US.clone(),
                database_account_endpoint: "https://test-eastus.documents.azure.com:443/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::NORTH_EUROPE.clone(),
                database_account_endpoint: "https://test-northeurope.documents.azure.com:443/"
                    .parse()
                    .unwrap(),
            },
        ];
        let read_regions = write_regions.clone();

        let default_endpoint: Url = "https://test.documents.azure.com:443/".parse().unwrap();

        let mut cache = LocationCache::new(default_endpoint, proximity_list, vec![]);
        cache.update(write_regions, read_regions).unwrap();

        let write_endpoints = cache.write_endpoints();

        // From West Europe, North Europe is much closer than East US
        let endpoint_strings: Vec<&str> = write_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-northeurope.documents.azure.com/",
                "https://test-eastus.documents.azure.com/",
            ],
            "write endpoints should be in proximity order from West Europe"
        );
    }

    // --- Proximity + excluded regions interaction tests ---

    /// Helper: creates a LocationCache with EAST_US proximity-sorted preferred regions,
    /// the given account regions, and client-level excluded regions.
    fn create_proximity_cache_with_exclusions(
        account_regions: &[AccountRegion],
        client_excluded: Vec<RegionName>,
    ) -> LocationCache {
        let proximity_list = generate_preferred_region_list(&regions::EAST_US)
            .expect("EAST_US should be a known region")
            .to_vec();
        let default_endpoint: Url = "https://test.documents.azure.com/".parse().unwrap();

        let mut cache = LocationCache::new(default_endpoint, proximity_list, client_excluded);
        cache
            .update(account_regions.to_vec(), account_regions.to_vec())
            .unwrap();
        cache
    }

    fn east_us_account_regions() -> Vec<AccountRegion> {
        vec![
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_EUROPE.clone(),
                database_account_endpoint: "https://test-westeurope.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ]
    }

    /// Excluding the closest proximity region routes to the next-closest.
    #[test]
    fn proximity_with_closest_region_excluded() {
        let cache = create_proximity_cache_with_exclusions(
            &east_us_account_regions(),
            vec![regions::EAST_US_2],
        );

        let read_endpoints = cache.read_endpoints();
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-westus.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
            "West US should be first after East US 2 is excluded"
        );
    }

    /// Excluding multiple regions preserves proximity order among the remaining.
    #[test]
    fn proximity_with_multiple_regions_excluded() {
        let mut regions_list = east_us_account_regions();
        regions_list.push(AccountRegion {
            name: regions::NORTH_EUROPE.clone(),
            database_account_endpoint: "https://test-northeurope.documents.azure.com/"
                .parse()
                .unwrap(),
        });

        let cache = create_proximity_cache_with_exclusions(
            &regions_list,
            vec![regions::EAST_US_2, regions::WEST_US],
        );

        let read_endpoints = cache.read_endpoints();

        // From EAST_US, North Europe (76ms) is closer than West Europe (81ms)
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-northeurope.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
            "remaining regions should preserve proximity order from East US"
        );
    }

    /// Request-level excluded regions override client-level, restoring proximity order.
    #[test]
    fn proximity_request_excluded_overrides_client_excluded() {
        // Client excludes East US 2 (closest)
        let cache = create_proximity_cache_with_exclusions(
            &east_us_account_regions(),
            vec![regions::EAST_US_2],
        );

        // Request excludes West Europe instead — overrides client exclusion,
        // so East US 2 is back and West Europe is removed.
        let request = CosmosRequest::builder(
            OperationType::Read,
            ResourceLink::root(ResourceType::Documents),
        )
        .excluded_regions(Some(vec![regions::WEST_EUROPE]))
        .build()
        .ok()
        .unwrap();

        let endpoint = cache.resolve_service_endpoint(&request);
        assert_eq!(
            endpoint.as_str(),
            "https://test-eastus2.documents.azure.com/",
            "East US 2 should be routed to (request overrides client exclusion)"
        );
    }

    /// Excluding all account regions falls back to the default endpoint.
    #[test]
    fn proximity_exclude_all_falls_back_to_default() {
        let account = vec![
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ];

        let cache = create_proximity_cache_with_exclusions(
            &account,
            vec![regions::EAST_US_2, regions::WEST_US],
        );

        let read_endpoints = cache.read_endpoints();
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec!["https://test.documents.azure.com/"],
            "should fall back to default endpoint when all regions excluded"
        );
    }

    /// Excluding a region not present in the account has no effect on proximity ordering.
    #[test]
    fn proximity_exclude_non_account_region_is_noop() {
        let account = vec![
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ];

        let cache = create_proximity_cache_with_exclusions(
            &account,
            vec![regions::AUSTRALIA_EAST], // not in the account
        );

        let read_endpoints = cache.read_endpoints();
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-eastus2.documents.azure.com/",
                "https://test-westus.documents.azure.com/",
            ],
            "excluding a non-account region should have no effect"
        );
    }
}
