// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, SystemTime},
};
use tracing::info;

const DEFAULT_EXPIRATION_TIME: Duration = Duration::from_secs(5 * 60);

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRegion {
    pub endpoint: String,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountProperties {
    pub database_account_endpoint: String,
    pub read_regions: Vec<AccountRegion>,
    pub write_regions: Vec<AccountRegion>,
    pub enable_multiple_write_locations: bool,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum RequestOperation {
    Read,
    Write,
    All,
}

impl RequestOperation {
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

#[derive(Clone, Default, Debug)]
pub struct DatabaseAccountLocationsInfo {
    pub preferred_locations: Vec<String>,
    account_write_locations: Vec<String>,
    account_read_locations: Vec<String>,
    account_write_endpoints_by_location: HashMap<String, String>,
    account_read_endpoints_by_location: HashMap<String, String>,
    write_endpoints: Vec<String>,
    read_endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationUnavailabilityInfo {
    pub last_check_time: SystemTime,
    pub unavailable_operation: RequestOperation,
}

#[derive(Default, Debug)]
pub struct LocationCache {
    pub default_endpoint: String,
    pub locations_info: DatabaseAccountLocationsInfo,
    pub location_unavailability_info_map: RwLock<HashMap<String, LocationUnavailabilityInfo>>,
}

impl LocationCache {
    pub fn new(default_endpoint: String, preferred_locations: Vec<String>) -> Self {
        Self {
            default_endpoint,
            locations_info: DatabaseAccountLocationsInfo {
                preferred_locations,
                ..Default::default()
            },
            location_unavailability_info_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn read_endpoints(&self) -> Vec<String> {
        self.locations_info.read_endpoints.clone()
    }

    pub fn write_endpoints(&self) -> Vec<String> {
        self.locations_info.write_endpoints.clone()
    }

    pub fn update(
        &mut self,
        write_locations: HashMap<String, String>,
        read_locations: HashMap<String, String>,
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

    pub fn mark_endpoint_unavailable(&mut self, endpoint: &str, operation: RequestOperation) {
        let now = std::time::SystemTime::now();

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

    pub fn resolve_service_endpoint(
        &self,
        location_index: usize,
        operation: RequestOperation,
    ) -> String {
        // Returns service endpoint based on index, if index out of bounds or operation not supported, returns default endpoint
        if operation == RequestOperation::Write && !self.locations_info.write_endpoints.is_empty() {
            self.locations_info
                .write_endpoints
                .get(location_index)
                .cloned()
                .unwrap_or_else(|| self.default_endpoint.clone())
        } else if operation == RequestOperation::Read
            && !self.locations_info.read_endpoints.is_empty()
        {
            self.locations_info
                .read_endpoints
                .get(location_index)
                .cloned()
                .unwrap_or_else(|| self.default_endpoint.clone())
        } else {
            self.default_endpoint.clone()
        }
    }

    pub fn can_use_multiple_write_locations(&mut self) -> bool {
        !self.write_endpoints().is_empty()
    }

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

    fn get_endpoints_by_location(
        &mut self,
        locations: HashMap<String, String>,
    ) -> (HashMap<String, String>, Vec<String>) {
        // Separates locations into a hashmap and list
        let mut endpoints_by_location: HashMap<String, String> = HashMap::new();
        let mut parsed_locations: Vec<String> = Vec::new();

        for (location, endpoint) in locations {
            if !location.is_empty() {
                endpoints_by_location.insert(location.clone(), endpoint.clone());
                parsed_locations.push(location);
            }
        }

        (endpoints_by_location, parsed_locations)
    }

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
            endpoints.push(default_endpoint.to_string());
        }

        endpoints
    }

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
    use std::{collections::HashSet, vec};

    fn create_test_data() -> (
        String,
        HashMap<String, String>,
        HashMap<String, String>,
        Vec<String>,
    ) {
        // Setting up test database account data
        let default_endpoint = "https://default.documents.example.com".to_string();

        let location_1 = AccountRegion {
            endpoint: "https://location1.documents.example.com".to_string(),
            region: "Location 1".to_string(),
        };
        let location_2 = AccountRegion {
            endpoint: "https://location2.documents.example.com".to_string(),
            region: "Location 2".to_string(),
        };
        let location_3 = AccountRegion {
            endpoint: "https://location3.documents.example.com".to_string(),
            region: "Location 3".to_string(),
        };
        let location_4 = AccountRegion {
            endpoint: "https://location4.documents.example.com".to_string(),
            region: "Location 4".to_string(),
        };
        let write_locations = HashMap::from([
            ("Location 1".to_string(), location_1.endpoint.clone()),
            ("Location 2".to_string(), location_2.endpoint.clone()),
        ]);

        let read_locations = HashMap::from([
            ("Location 1".to_string(), location_1.endpoint.clone()),
            ("Location 2".to_string(), location_2.endpoint.clone()),
            ("Location 3".to_string(), location_3.endpoint.clone()),
            ("Location 4".to_string(), location_4.endpoint.clone()),
        ]);

        let preferred_locations = vec!["Location 1".to_string(), "Location 2".to_string()];

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

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(0, RequestOperation::Read),
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

        // check that endpoint is last option in write endpoints and it is in the location unavailability info map
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&unavailable_endpoint.to_string())
        );

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));

        assert_eq!(
            cache.resolve_service_endpoint(0, RequestOperation::Write),
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

        // resolve service endpoint
        let endpoint = cache.resolve_service_endpoint(0, RequestOperation::Read);
        assert_eq!(
            endpoint,
            "https://location1.documents.example.com".to_string()
        );
    }

    #[test]
    fn resolve_service_endpoint_second_location() {
        // create test cache
        let cache = create_test_location_cache();

        // resolve service endpoint for second location
        let endpoint = cache.resolve_service_endpoint(1, RequestOperation::Read);
        assert_eq!(
            endpoint,
            "https://location2.documents.example.com".to_string()
        );
    }

    #[test]
    fn resolve_service_endpoint_default() {
        let cache = create_test_location_cache();

        let endpoint = cache.resolve_service_endpoint(
            cache.locations_info.read_endpoints.len() + 1,
            RequestOperation::Read,
        );
        assert_eq!(
            endpoint,
            "https://default.documents.example.com".to_string()
        );
    }
}
