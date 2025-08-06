use std::{collections::HashMap, sync::Mutex, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRegion {
    #[serde(rename = "databaseAccountEndpoint")]
    pub endpoint: String,
    #[serde(rename = "name")]
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

#[derive(Clone, Default)]
pub struct DatabaseAccountLocationsInfo {
    pub preferred_locations: Vec<String>,
    available_write_locations: Vec<String>,
    available_read_locations: Vec<String>,
    available_write_endpoints_by_location: HashMap<String, String>,
    available_read_endpoints_by_location: HashMap<String, String>,
    write_endpoints: Vec<String>,
    read_endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationUnavailabilityInfo {
    pub last_check_time: SystemTime,
    pub unavailable_operation: RequestOperation,
}

pub struct LocationCache {
    pub default_endpoint: String,
    pub locations_info: DatabaseAccountLocationsInfo,
    pub location_unavailability_info_map: Mutex<HashMap<String, LocationUnavailabilityInfo>>,
}

impl LocationCache {
    pub fn new(default_endpoint: String, preferred_locations: Vec<String>) -> Self {
        Self {
            default_endpoint,
            locations_info: DatabaseAccountLocationsInfo {
                preferred_locations: preferred_locations,
                ..Default::default()
            },
            location_unavailability_info_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn update(
        &mut self,
        write_locations: HashMap<String, String>,
        read_locations: HashMap<String, String>,
    ) -> Result<(), &'static str> {
        // Separate write locations into appropriate hashmap and list
        if !write_locations.is_empty() {
            let (available_write_endpoints_by_location, available_write_locations) =
                self.get_endpoints_by_location(write_locations);
            self.locations_info.available_write_endpoints_by_location =
                available_write_endpoints_by_location;
            self.locations_info.available_write_locations = available_write_locations;
        }

        // Separate read locations into appropriate hashmap and list
        if !read_locations.is_empty() {
            let (available_read_endpoints_by_location, available_read_locations) =
                self.get_endpoints_by_location(read_locations);
            self.locations_info.available_read_endpoints_by_location =
                available_read_endpoints_by_location;
            self.locations_info.available_read_locations = available_read_locations;
        }

        self.refresh_endpoints();

        Ok(())
    }

    pub fn mark_endpoint_unavailable(&mut self, endpoint: &str, operation: RequestOperation) {
        let now = std::time::SystemTime::now();

        {
            let mut location_unavailability_info_map =
                self.location_unavailability_info_map.lock().unwrap();

            if let Some(info) = location_unavailability_info_map.get_mut(endpoint) {
                if info.unavailable_operation == operation
                    || info.unavailable_operation == RequestOperation::All
                {
                    // If the endpoint is already marked as unavailable, update last_check_time
                    info.last_check_time = now;
                } else {
                    // If endpoint marked unavailable but with a different operation, update operation type to All
                    info.last_check_time = now;
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
        }

        self.refresh_endpoints();
    }

    pub fn is_endpoint_unavailable(&self, endpoint: &str, operation: RequestOperation) -> bool {
        let location_unavailability_info_map =
            self.location_unavailability_info_map.lock().unwrap();
        if let Some(info) = location_unavailability_info_map.get(endpoint) {
            // Checks if endpoint is unavailable for the given operation
            let operation_type = info.unavailable_operation == operation;
            let elapsed = info.last_check_time.elapsed().unwrap_or_default()
                < std::time::Duration::from_secs(300);

            operation_type && elapsed
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
            self.locations_info.write_endpoints[location_index].clone()
        } else if operation == RequestOperation::Read
            && !self.locations_info.read_endpoints.is_empty()
        {
            self.locations_info.read_endpoints[location_index].clone()
        } else {
            self.default_endpoint.clone()
        }
    }

    fn refresh_endpoints(&mut self) {
        // Get preferred available endpoints for write and read operations
        self.locations_info.write_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.available_write_endpoints_by_location,
            RequestOperation::Write,
            &self.default_endpoint,
        );

        self.locations_info.read_endpoints = self.get_preferred_available_endpoints(
            &self.locations_info.available_read_endpoints_by_location,
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
            self.location_unavailability_info_map.lock().unwrap();

        // Removes endpoints that have not been checked in the last 5 minutes
        location_unavailability_info_map.retain(|_, info| {
            info.last_check_time.elapsed().unwrap_or_default()
                <= std::time::Duration::from_secs(300)
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
        let default_endpoint = "https://default.documents.azure.com".to_string();

        let location_1 = AccountRegion {
            endpoint: "https://location1.documents.azure.com".to_string(),
            region: "Location 1".to_string(),
        };
        let location_2 = AccountRegion {
            endpoint: "https://location2.documents.azure.com".to_string(),
            region: "Location 2".to_string(),
        };
        let location_3 = AccountRegion {
            endpoint: "https://location3.documents.azure.com".to_string(),
            region: "Location 3".to_string(),
        };
        let location_4 = AccountRegion {
            endpoint: "https://location4.documents.azure.com".to_string(),
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

    #[test]
    fn location_cache_update_test() {
        // this test also checks refresh_endpoints, get_endpoints_by_location, and get_preferred_available_endpoints methods
        // set up test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint.to_string(), preferred_locations);

        cache.update(write_locations, read_locations);

        assert_eq!(
            cache.default_endpoint,
            "https://default.documents.azure.com"
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec!["Location 1".to_string(), "Location 2".to_string()]
        );

        // check available write locations
        let actual_available_write_locations: HashSet<_> = cache
            .locations_info
            .available_write_locations
            .iter()
            .cloned()
            .collect();
        let expected_available_write_locations: HashSet<String> = ["Location 1", "Location 2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            actual_available_write_locations,
            expected_available_write_locations
        );

        // check available read locations
        let actual_available_read_locations: HashSet<_> = cache
            .locations_info
            .available_read_locations
            .iter()
            .cloned()
            .collect();
        let expected_available_read_locations: HashSet<String> =
            ["Location 1", "Location 2", "Location 3", "Location 4"]
                .iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(
            actual_available_read_locations,
            expected_available_read_locations
        );

        assert_eq!(
            cache.locations_info.available_write_endpoints_by_location,
            HashMap::from([
                (
                    "Location 1".to_string(),
                    "https://location1.documents.azure.com".to_string()
                ),
                (
                    "Location 2".to_string(),
                    "https://location2.documents.azure.com".to_string()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.available_read_endpoints_by_location,
            HashMap::from([
                (
                    "Location 1".to_string(),
                    "https://location1.documents.azure.com".to_string()
                ),
                (
                    "Location 2".to_string(),
                    "https://location2.documents.azure.com".to_string()
                ),
                (
                    "Location 3".to_string(),
                    "https://location3.documents.azure.com".to_string()
                ),
                (
                    "Location 4".to_string(),
                    "https://location4.documents.azure.com".to_string()
                )
            ])
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec![
                "https://location1.documents.azure.com".to_string(),
                "https://location2.documents.azure.com".to_string()
            ]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec![
                "https://location1.documents.azure.com".to_string(),
                "https://location2.documents.azure.com".to_string(),
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

        cache.update(write_locations, read_locations);

        assert_eq!(
            cache.default_endpoint,
            "https://default.documents.azure.com"
        );

        assert_eq!(
            cache.locations_info.preferred_locations,
            vec![preferred_locations[0].clone()]
        );

        assert_eq!(
            cache.locations_info.write_endpoints,
            vec!["https://location1.documents.azure.com".to_string()]
        );

        assert_eq!(
            cache.locations_info.read_endpoints,
            vec!["https://location1.documents.azure.com".to_string()]
        );
    }

    #[test]
    fn mark_read_endpoint_unavailable_test() {
        // set up test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);
        cache.update(write_locations, read_locations);

        // mark location 1 as unavailable endpoint for read operation
        let unavailable_endpoint = "https://location1.documents.azure.com";
        let operation = RequestOperation::Read;
        cache.mark_endpoint_unavailable(unavailable_endpoint, operation);

        // check that endpoint is last option in read endpoints and it is in the location unavailability info map
        assert_eq!(
            cache.locations_info.read_endpoints.last(),
            Some(&unavailable_endpoint.to_string())
        );

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));
    }

    #[test]
    fn mark_write_endpoint_unavailable_test() {
        // set up test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);
        cache.update(write_locations, read_locations);

        // mark location 1 as unavailable endpoint for write operation
        let unavailable_endpoint = "https://location1.documents.azure.com";
        let operation = RequestOperation::Write;
        cache.mark_endpoint_unavailable(unavailable_endpoint, operation);

        // check that endpoint is last option in write endpoints and it is in the location unavailability info map
        assert_eq!(
            cache.locations_info.write_endpoints.last(),
            Some(&unavailable_endpoint.to_string())
        );

        assert!(cache.is_endpoint_unavailable(unavailable_endpoint, operation));
    }

    #[test]
    fn mark_same_endpoint_unavailable_test() {
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);

        cache.update(write_locations, read_locations);

        let endpoint1 = "https://location1.documents.azure.com";

        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Write);

        assert_eq!(
            cache
                .location_unavailability_info_map
                .lock()
                .unwrap()
                .get(endpoint1)
                .map(|info| info.unavailable_operation),
            Some(RequestOperation::All)
        );
    }

    #[test]
    fn refresh_stale_endpoints_test() {
        // create test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);

        cache.update(write_locations, read_locations);

        // mark endpoint 1 and endpoint 2 as unavailable
        let endpoint1 = "https://location1.documents.azure.com";
        let endpoint2 = "https://location2.documents.azure.com";
        cache.mark_endpoint_unavailable(endpoint1, RequestOperation::Read);
        cache.mark_endpoint_unavailable(endpoint2, RequestOperation::Read);

        // simulate stale entry
        {
            let mut unavailability_map = cache.location_unavailability_info_map.lock().unwrap();
            if let Some(info) = unavailability_map.get_mut(endpoint1) {
                info.last_check_time = SystemTime::now() - std::time::Duration::from_secs(500);
            }
        }

        // refresh stale endpoints
        cache.refresh_stale_endpoints();

        // check that endpoint 1 is marked as available again
        assert!(!cache.is_endpoint_unavailable(endpoint1, RequestOperation::Read));
    }

    #[test]
    fn resolve_service_endpoint_test() {
        // create test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);

        cache.update(write_locations, read_locations);

        // resolve service endpoint
        let endpoint = cache.resolve_service_endpoint(0, RequestOperation::Read);
        assert_eq!(
            endpoint,
            "https://location1.documents.azure.com".to_string()
        );
    }

    #[test]
    fn resolve_service_endpoint_second_location_test() {
        // create test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache = LocationCache::new(default_endpoint, preferred_locations);

        cache.update(write_locations, read_locations);

        // resolve service endpoint for second location
        let endpoint = cache.resolve_service_endpoint(1, RequestOperation::Read);
        assert_eq!(
            endpoint,
            "https://location2.documents.azure.com".to_string()
        );
    }
}
