use std::collections::HashMap;

use url::Url;

pub struct AccountRegion {
    pub region: String,
    pub endpoint: Url,
}

pub enum RequestOperation {
    Read,
    Write,
}

#[derive(Clone)]
pub struct DatabaseAccountLocationsInfo {
    pub preferred_locations: Vec<String>,
    available_write_locations: Vec<String>,
    available_read_locations: Vec<String>,
    available_write_endpoints_by_location: HashMap<String, Url>,
    available_read_endpoints_by_location: HashMap<String, Url>,
    write_endpoints: Vec<Url>,
    read_endpoints: Vec<Url>,
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

pub struct LocationCache {
    pub default_endpoint: Url,
    pub locations_info: DatabaseAccountLocationsInfo,
}

impl LocationCache {
    pub fn new(default_endpoint: Url, preferred_locations: Vec<String>) -> Self {
        Self {
            default_endpoint,
            locations_info: DatabaseAccountLocationsInfo {
                preferred_locations,
                ..Default::default()
            },
        }
    }

    pub fn update_location_cache(
        &mut self,
        write_locations: HashMap<String, Url>,
        read_locations: HashMap<String, Url>,
        preferred_locations: Vec<String>,
    ) {
        let mut locations_info_copy = self.locations_info.clone();

        if !preferred_locations.is_empty() {
            locations_info_copy.preferred_locations = preferred_locations;
        }

        if !write_locations.is_empty() {
            let (available_write_endpoints_by_location, available_write_locations) =
                self.get_endpoints_by_location(write_locations);
            locations_info_copy.available_write_endpoints_by_location =
                available_write_endpoints_by_location;
            locations_info_copy.available_write_locations = available_write_locations;
        }

        if !read_locations.is_empty() {
            let (available_read_endpoints_by_location, available_read_locations) =
                self.get_endpoints_by_location(read_locations);
            locations_info_copy.available_read_endpoints_by_location =
                available_read_endpoints_by_location;
            locations_info_copy.available_read_locations = available_read_locations;
        }

        //call get_preferred_available_endpoints

        self.locations_info = locations_info_copy;
    }

    pub fn get_endpoints_by_location(
        &mut self,
        locations: HashMap<String, Url>,
    ) -> (HashMap<String, Url>, Vec<String>) {
        let mut endpoints_by_location: HashMap<String, Url> = HashMap::new();
        let mut parsed_locations: Vec<String> = Vec::new();

        for (location, url) in locations {
            if location != "" {
                endpoints_by_location.insert(location.clone(), url.clone());
                parsed_locations.push(location);
            }
        }

        (endpoints_by_location, parsed_locations)
    }

    pub fn get_preferred_available_endpoints(
        &mut self,
        endpoints_by_location: HashMap<String, Url>,
        locations: Vec<String>,
        request: RequestOperation,
        default_endpoint: Url,
    ) -> Vec<Url> {
        let mut endpoints: Vec<Url> = Vec::new();
        let mut unavailable_endpoints: Vec<Url> = Vec::new();

        if !self.locations_info.preferred_locations.is_empty() {
            for location in &self.locations_info.preferred_locations {
                if let Some(endpoint) = endpoints_by_location.get(location) {
                    //check if endpoint is available, if not add to unavailable_endpoints

                    //if it is then add to endpoints
                }
            }
        }

        for location in locations {
            if let Some(endpoint) = endpoints_by_location.get(&location) {
                endpoints.push(endpoint.clone());
            }
        }

        endpoints
    }
}
