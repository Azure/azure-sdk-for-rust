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
    #[serde(rename = "databaseAccountEndpoint")]
    pub database_account_endpoint: String,
    #[serde(rename = "readableLocations")]
    pub read_regions: Vec<AccountRegion>,
    #[serde(rename = "writableLocations")]
    pub write_regions: Vec<AccountRegion>,
    #[serde(rename = "multiRegionWrites")]
    pub enable_multiple_write_locations: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum RequestOperation {
    Read,
    Write,
}

#[derive(Clone)]
pub struct DatabaseAccountLocationsInfo {
    pub preferred_locations: Vec<String>,
    available_write_locations: Vec<String>,
    available_read_locations: Vec<String>,
    available_write_endpoints_by_location: HashMap<String, String>,
    available_read_endpoints_by_location: HashMap<String, String>,
    write_endpoints: Vec<String>,
    read_endpoints: Vec<String>,
}

impl Default for DatabaseAccountLocationsInfo {
    fn default() -> Self {
        DatabaseAccountLocationsInfo {
            preferred_locations: Vec::new(),
            available_write_locations: Vec::new(),
            available_read_locations: Vec::new(),
            available_write_endpoints_by_location: HashMap::new(),
            available_read_endpoints_by_location: HashMap::new(),
            write_endpoints: Vec::new(),
            read_endpoints: Vec::new(),
        }
    }
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
                preferred_locations,
                ..Default::default()
            },
            location_unavailability_info_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn update(
        &mut self,
        write_locations: Option<HashMap<String, String>>,
        read_locations: Option<HashMap<String, String>>,
        preferred_locations: Option<Vec<String>>,
    ) {
        // create locations_info copy
        let mut locations_info_copy = self.locations_info.clone();

        if let Some(preferred_locations) = preferred_locations {
            if !preferred_locations.is_empty() {
                locations_info_copy.preferred_locations = preferred_locations;
            }
        }

        // separate write locations into appropriate hashmap and list
        if let Some(write_locations) = write_locations {
            if !write_locations.is_empty() {
                let (available_write_endpoints_by_location, available_write_locations) =
                    self.get_endpoints_by_location(write_locations);
                locations_info_copy.available_write_endpoints_by_location =
                    available_write_endpoints_by_location;
                locations_info_copy.available_write_locations = available_write_locations;
            }
        }

        // separate read locations into appropriate hashmap and list
        if let Some(read_locations) = read_locations {
            if !read_locations.is_empty() {
                let (available_read_endpoints_by_location, available_read_locations) =
                    self.get_endpoints_by_location(read_locations);
                locations_info_copy.available_read_endpoints_by_location =
                    available_read_endpoints_by_location;
                locations_info_copy.available_read_locations = available_read_locations;
            }
        }

        // get preferred available endpoints for write and read operations
        locations_info_copy.write_endpoints = self.get_preferred_available_endpoints(
            locations_info_copy
                .available_write_endpoints_by_location
                .clone(),
            locations_info_copy.available_write_locations.clone(),
            RequestOperation::Write,
            self.default_endpoint.clone(),
        );

        locations_info_copy.read_endpoints = self.get_preferred_available_endpoints(
            locations_info_copy
                .available_read_endpoints_by_location
                .clone(),
            locations_info_copy.available_read_locations.clone(),
            RequestOperation::Read,
            self.default_endpoint.clone(),
        );

        self.locations_info = locations_info_copy;
    }

    pub fn get_endpoints_by_location(
        &mut self,
        locations: HashMap<String, String>,
    ) -> (HashMap<String, String>, Vec<String>) {
        let mut endpoints_by_location: HashMap<String, String> = HashMap::new();
        let mut parsed_locations: Vec<String> = Vec::new();

        for (location, String) in locations {
            if location != "" {
                endpoints_by_location.insert(location.clone(), String.clone());
                parsed_locations.push(location);
            }
        }

        (endpoints_by_location, parsed_locations)
    }

    pub fn get_preferred_available_endpoints(
        &self,
        endpoints_by_location: HashMap<String, String>,
        locations: Vec<String>,
        request: RequestOperation,
        default_endpoint: String,
    ) -> Vec<String> {
        let mut endpoints: Vec<String> = Vec::new();
        let mut unavailable_endpoints: Vec<String> = Vec::new();

        if !self.locations_info.preferred_locations.is_empty() {
            for location in &self.locations_info.preferred_locations {
                //checks if preferred location exists in endpoints_by_location
                if let Some(endpoint) = endpoints_by_location.get(location) {
                    //check if endpoint is available, if not add to unavailable_endpoints
                    //if it is then add to endpoints
                    if !self.is_endpoint_unavailable(endpoint.to_string(), request.clone()) {
                        endpoints.push(endpoint.clone());
                    } else {
                        unavailable_endpoints.push(endpoint.clone());
                    }
                }
            }
        }

        // Add unavailable endpoints
        for endpoint in unavailable_endpoints {
            endpoints.push(endpoint);
        }

        // If no preferred locations were found, use the default endpoint
        if endpoints.is_empty() {
            endpoints.push(default_endpoint.clone());
        }

        endpoints
    }

    fn mark_endpoint_unavailable(&mut self, endpoint: String, operation: RequestOperation) {
        let now = std::time::SystemTime::now();

        {
            let mut location_unavailability_info_map =
                self.location_unavailability_info_map.lock().unwrap();
            location_unavailability_info_map
                .entry(endpoint)
                .and_modify(|info| {
                    info.last_check_time = now;
                    info.unavailable_operation = operation.clone();
                })
                .or_insert(LocationUnavailabilityInfo {
                    last_check_time: now,
                    unavailable_operation: operation.clone(),
                });
        }

        self.update(None, None, None);
    }

    fn is_endpoint_unavailable(&self, endpoint: String, operation: RequestOperation) -> bool {
        let location_unavailability_info_map =
            self.location_unavailability_info_map.lock().unwrap();
        if let Some(info) = location_unavailability_info_map.get(&endpoint) {
            //this feels counterintuitive, nothing necessarily happens here
            info.unavailable_operation == operation
        } else {
            false
        }
    }

    fn resolve_service_endpoint(&self, operation: RequestOperation) -> String {
        if operation == RequestOperation::Write && !self.locations_info.write_endpoints.is_empty() {
            self.locations_info.write_endpoints[0].clone()
        } else if operation == RequestOperation::Read
            && !self.locations_info.read_endpoints.is_empty()
        {
            self.locations_info.read_endpoints[0].clone()
        } else {
            self.default_endpoint.clone()
        }
    }
}

//tests for location cache

#[cfg(test)]

mod tests {
    use std::{default, hash::Hash};

    use super::*;

    const ACCOUNT_DATA: &str = r#"
{
  "databaseAccountEndpoint": "https://default.documents.azure.com",
  "name": "West US 3",
  "readableLocations": [
    {
      "databaseAccountEndpoint": "https://westus3.documents.azure.com:443/",
      "name": "West US 3"
    },
    {
      "databaseAccountEndpoint": "https://eastus.documents.azure.com:443/",
      "name": "East US"
    },
    {
      "databaseAccountEndpoint": "https://southcentralus.documents.azure.com:443/",
      "name": "South Central US"
    }
  ],
  "writableLocations": [
    {
      "databaseAccountEndpoint": "https://westus3.documents.azure.com:443/",
      "name": "West US 3"
    },
    {
      "databaseAccountEndpoint": "https://eastus.documents.azure.com:443/",
      "name": "East US"
    },
    {
      "databaseAccountEndpoint": "https://southcentralus.documents.azure.com:443/",
      "name": "South Central US"
    }
  ],
  "multiRegionWrites": false
}
"#;

    fn parse_account_properties() -> AccountProperties {
        serde_json::from_str(ACCOUNT_DATA).expect("Failed to parse account properties")
    }

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
        let write_locations =
            HashMap::from([("Location 1".to_string(), location_1.endpoint.clone())]);

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
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let mut cache =
            LocationCache::new(default_endpoint.to_string(), preferred_locations.clone());

        cache.update(
            Some(write_locations.clone()),
            Some(read_locations.clone()),
            Some(preferred_locations.clone()),
        );

        println!(
            "Cache default endpoint: {}, Cache preferred locations: {:#?}, Cache write endpoints: {:#?}, Cache read endpoints: {:#?}, Cache available_write_endpoints by location: {:?}, Cache available_read_endpoints by location: {:?}, write endpoints : {:?}, read endpoints: {:?}",
            cache.default_endpoint.as_str(),
            cache.locations_info.preferred_locations,
            cache.locations_info.available_write_locations,
            cache.locations_info.available_read_locations,
            cache.locations_info.available_write_endpoints_by_location,
            cache.locations_info.available_read_endpoints_by_location,
            cache.locations_info.write_endpoints,
            cache.locations_info.read_endpoints
        );
    }

    #[test]
    fn mark_endpoint_unavailable_test() {
        // set up test data
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();
        let mut cache = LocationCache::new(default_endpoint, preferred_locations.clone());
        cache.update(
            Some(write_locations),
            Some(read_locations),
            Some(preferred_locations),
        );

        // mark location 1 as unavailable endpoint for read operation
        let unavailable_endpoint = "https://location1.documents.azure.com".to_string();
        let operation = RequestOperation::Read;
        cache.mark_endpoint_unavailable(unavailable_endpoint, operation);

        // check that endpoint is no longer in read endpoints, or available_read_endpoints_by_location, and it is in the location unavailablility info map
        println!(
            "Cache read endpoints after marking endpoint unavailable: {:#?}, Cache available read endpoints by location: {:#?}, Cache location unavailability info map: {:#?}",
            cache.locations_info.read_endpoints,
            cache.locations_info.available_read_endpoints_by_location,
            cache.location_unavailability_info_map.lock().unwrap()
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

        cache.update(
            Some(write_locations.clone()),
            Some(read_locations.clone()),
            None,
        );

        println!(
            "Cache default endpoint: {}, Cache preferred locations: {:#?}, Cache write endpoints: {:#?}, Cache read endpoints: {:#?}, Cache available_write_endpoints by location: {:?}, Cache available_read_endpoints by location: {:?}, write endpoints : {:?}, read endpoints: {:?}",
            cache.default_endpoint.as_str(),
            cache.locations_info.preferred_locations,
            cache.locations_info.available_write_locations,
            cache.locations_info.available_read_locations,
            cache.locations_info.available_write_endpoints_by_location,
            cache.locations_info.available_read_endpoints_by_location,
            cache.locations_info.write_endpoints,
            cache.locations_info.read_endpoints
        );
    }

    #[test]
    fn location_cache_update_second_test() {
        let (default_endpoint, write_locations, read_locations, preferred_locations) =
            create_test_data();

        let preferred_locations = vec!["Location 3".to_string(), "Location 4".to_string()];

        let mut cache =
            LocationCache::new(default_endpoint.to_string(), preferred_locations.clone());

        cache.update(
            Some(write_locations.clone()),
            Some(read_locations.clone()),
            Some(preferred_locations.clone()),
        );

        cache.mark_endpoint_unavailable("Location 3".to_string(), RequestOperation::Read);

        println!(
            "Cache default endpoint: {}, Cache preferred locations: {:#?}, Cache write endpoints: {:#?}, Cache read endpoints: {:#?}, Cache available_write_endpoints by location: {:?}, Cache available_read_endpoints by location: {:?}, write endpoints : {:?}, read endpoints: {:?}",
            cache.default_endpoint.as_str(),
            cache.locations_info.preferred_locations,
            cache.locations_info.available_write_locations,
            cache.locations_info.available_read_locations,
            cache.locations_info.available_write_endpoints_by_location,
            cache.locations_info.available_read_endpoints_by_location,
            cache.locations_info.write_endpoints,
            cache.locations_info.read_endpoints
        );
    }
}
