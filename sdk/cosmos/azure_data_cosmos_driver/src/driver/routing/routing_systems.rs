// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure routing systems for account endpoint state.

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use tracing::warn;

use crate::driver::cache::AccountProperties;

use super::{AccountEndpointState, CosmosEndpoint, UnavailableReason};

/// Builds account endpoint state from account metadata.
///
/// TODO: Accept `preferred_locations: &[Region]` to reorder endpoint lists
/// based on user configuration (derived from `application_region` via
/// `generate_preferred_region_list` in `azure_data_cosmos`). Wire this when
/// operations in `azure_data_cosmos` are migrated to the driver's
/// `execute_operation` API — that cross-crate change is the natural point
/// to thread preferred regions through `DriverOptions` → `LocationStateStore`.
pub(crate) fn build_account_endpoint_state(
    properties: &AccountProperties,
    default_endpoint: CosmosEndpoint,
    previous_generation: Option<u64>,
    gateway20_enabled: bool,
) -> AccountEndpointState {
    let generation = previous_generation.map_or(0, |g| g.saturating_add(1));

    let mut preferred_read_endpoints = build_preferred_endpoints(
        &properties.readable_locations,
        &properties.thin_client_readable_locations,
        gateway20_enabled,
    );

    let mut preferred_write_endpoints = build_preferred_endpoints(
        &properties.writable_locations,
        &properties.thin_client_writable_locations,
        gateway20_enabled,
    );

    if preferred_read_endpoints.is_empty() {
        preferred_read_endpoints.push(default_endpoint.clone());
    }
    if preferred_write_endpoints.is_empty() {
        preferred_write_endpoints.push(default_endpoint.clone());
    }

    AccountEndpointState {
        generation,
        preferred_read_endpoints: preferred_read_endpoints.into(),
        preferred_write_endpoints: preferred_write_endpoints.into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: properties.enable_multiple_write_locations,
        default_endpoint,
    }
}

fn build_preferred_endpoints(
    standard_locations: &[crate::driver::cache::AccountRegion],
    thin_client_locations: &[crate::driver::cache::AccountRegion],
    gateway20_enabled: bool,
) -> Vec<CosmosEndpoint> {
    let thin_client_urls = if gateway20_enabled {
        parse_thin_client_locations(thin_client_locations)
    } else {
        HashMap::new()
    };

    let mut endpoints = Vec::with_capacity(standard_locations.len());
    for region in standard_locations {
        let url = region.database_account_endpoint.url().clone();

        let endpoint = thin_client_urls
            .get(&region.name)
            .cloned()
            .map(|gateway20_url| {
                CosmosEndpoint::regional_with_gateway20(
                    region.name.clone(),
                    url.clone(),
                    gateway20_url,
                )
            })
            .unwrap_or_else(|| CosmosEndpoint::regional(region.name.clone(), url));

        endpoints.push(endpoint);
    }

    endpoints
}

fn parse_thin_client_locations(
    thin_client_locations: &[crate::driver::cache::AccountRegion],
) -> HashMap<crate::options::Region, url::Url> {
    let mut urls = HashMap::new();

    for region in thin_client_locations {
        let url = region.database_account_endpoint.url().clone();

        if url.scheme() != "https" {
            warn!(
                region = %region.name,
                endpoint = %region.database_account_endpoint,
                scheme = url.scheme(),
                "Ignoring non-HTTPS thin-client endpoint URL"
            );
            continue;
        }

        urls.entry(region.name.clone())
            .and_modify(|existing| {
                if existing != &url {
                    warn!(
                        region = %region.name,
                        existing_url = %existing,
                        new_url = %url,
                        "Duplicate thin-client region with conflicting URL; keeping first entry"
                    );
                }
            })
            .or_insert(url);
    }

    urls
}

/// Returns a new state with an endpoint marked unavailable.
pub(crate) fn mark_endpoint_unavailable(
    state: &AccountEndpointState,
    endpoint: &CosmosEndpoint,
    reason: UnavailableReason,
) -> AccountEndpointState {
    let mut unavailable = state.unavailable_endpoints.clone();
    unavailable.insert(endpoint.url().clone(), (Instant::now(), reason));

    AccountEndpointState {
        generation: state.generation,
        preferred_read_endpoints: Arc::clone(&state.preferred_read_endpoints),
        preferred_write_endpoints: Arc::clone(&state.preferred_write_endpoints),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: state.multiple_write_locations_enabled,
        default_endpoint: state.default_endpoint.clone(),
    }
}

/// Returns a new state with expired endpoint unavailability removed.
#[allow(dead_code)] // Spec-defined system function; used in tests and future steps.
pub(crate) fn expire_unavailable_endpoints(
    state: &AccountEndpointState,
    now: Instant,
    expiry_duration: Duration,
) -> AccountEndpointState {
    if state.unavailable_endpoints.is_empty() {
        return state.clone();
    }

    let mut unavailable = state.unavailable_endpoints.clone();
    unavailable
        .retain(|_, (marked_at, _)| now.saturating_duration_since(*marked_at) < expiry_duration);

    AccountEndpointState {
        generation: state.generation,
        preferred_read_endpoints: Arc::clone(&state.preferred_read_endpoints),
        preferred_write_endpoints: Arc::clone(&state.preferred_write_endpoints),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: state.multiple_write_locations_enabled,
        default_endpoint: state.default_endpoint.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::cache::AccountProperties;

    fn default_endpoint() -> CosmosEndpoint {
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    fn test_properties() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }],
            "enableMultipleWriteLocations": true,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    #[test]
    fn build_state_uses_metadata_locations() {
        let state =
            build_account_endpoint_state(&test_properties(), default_endpoint(), None, false);
        assert_eq!(state.generation, 0);
        assert_eq!(state.preferred_write_endpoints.len(), 1);
        assert_eq!(state.preferred_read_endpoints.len(), 1);
        assert!(state.multiple_write_locations_enabled);
    }

    #[test]
    fn build_state_adds_gateway20_endpoint_when_enabled() {
        let properties: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }],
            "thinClientReadableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2-thin.documents.azure.com:444/" }],
            "enableMultipleWriteLocations": true,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap();

        let state = build_account_endpoint_state(&properties, default_endpoint(), None, true);

        assert!(state.preferred_read_endpoints[0].gateway20_url().is_some());
        assert!(state.preferred_write_endpoints[0].gateway20_url().is_none());
    }

    #[test]
    fn build_state_adds_gateway20_for_write_endpoints_when_present() {
        let properties: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }],
            "thinClientReadableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2-thin.documents.azure.com:444/" }],
            "thinClientWritableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/" }],
            "enableMultipleWriteLocations": true,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap();

        let state = build_account_endpoint_state(&properties, default_endpoint(), None, true);

        assert!(state.preferred_read_endpoints[0].gateway20_url().is_some());
        assert!(state.preferred_write_endpoints[0].gateway20_url().is_some());
    }

    #[test]
    fn mark_and_expire_unavailable_endpoint() {
        let state =
            build_account_endpoint_state(&test_properties(), default_endpoint(), None, false);
        let endpoint = state.preferred_read_endpoints[0].clone();
        let marked =
            mark_endpoint_unavailable(&state, &endpoint, UnavailableReason::TransportError);
        assert_eq!(marked.unavailable_endpoints.len(), 1);

        let expired = expire_unavailable_endpoints(
            &marked,
            Instant::now() + Duration::from_secs(61),
            Duration::from_secs(60),
        );
        assert!(expired.unavailable_endpoints.is_empty());
    }
}
