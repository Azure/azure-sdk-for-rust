// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure routing systems for account endpoint state.

use std::time::{Duration, Instant};

use crate::driver::cache::AccountProperties;

use super::{AccountEndpointState, CosmosEndpoint, UnavailableReason};

/// Builds account endpoint state from account metadata.
pub(crate) fn build_account_endpoint_state(
    properties: &AccountProperties,
    default_endpoint: CosmosEndpoint,
    previous_generation: Option<u64>,
) -> AccountEndpointState {
    let generation = previous_generation.map_or(0, |g| g.saturating_add(1));

    let mut preferred_read_endpoints = Vec::with_capacity(properties.readable_locations.len() + 1);
    for region in &properties.readable_locations {
        if let Ok(url) = url::Url::parse(&region.database_account_endpoint) {
            preferred_read_endpoints.push(CosmosEndpoint::regional(region.name.clone(), url));
        }
    }

    let mut preferred_write_endpoints = Vec::with_capacity(properties.writable_locations.len() + 1);
    for region in &properties.writable_locations {
        if let Ok(url) = url::Url::parse(&region.database_account_endpoint) {
            preferred_write_endpoints.push(CosmosEndpoint::regional(region.name.clone(), url));
        }
    }

    if preferred_read_endpoints.is_empty() {
        preferred_read_endpoints.push(default_endpoint.clone());
    }
    if preferred_write_endpoints.is_empty() {
        preferred_write_endpoints.push(default_endpoint.clone());
    }

    AccountEndpointState {
        generation,
        preferred_read_endpoints,
        preferred_write_endpoints,
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: properties.enable_multiple_write_locations,
        default_endpoint,
    }
}

/// Returns a new state with an endpoint marked unavailable.
pub(crate) fn mark_endpoint_unavailable(
    state: &AccountEndpointState,
    endpoint: &CosmosEndpoint,
    reason: UnavailableReason,
) -> AccountEndpointState {
    let mut unavailable = state.unavailable_endpoints.clone();
    unavailable.insert(endpoint.clone(), (Instant::now(), reason));

    AccountEndpointState {
        unavailable_endpoints: unavailable,
        ..state.clone()
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
        unavailable_endpoints: unavailable,
        ..state.clone()
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
        let state = build_account_endpoint_state(&test_properties(), default_endpoint(), None);
        assert_eq!(state.generation, 0);
        assert_eq!(state.preferred_write_endpoints.len(), 1);
        assert_eq!(state.preferred_read_endpoints.len(), 1);
        assert!(state.multiple_write_locations_enabled);
    }

    #[test]
    fn mark_and_expire_unavailable_endpoint() {
        let state = build_account_endpoint_state(&test_properties(), default_endpoint(), None);
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
