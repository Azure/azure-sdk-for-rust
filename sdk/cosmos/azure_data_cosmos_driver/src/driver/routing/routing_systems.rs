// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure routing systems for account and partition endpoint state.

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use tracing::warn;

use crate::driver::cache::AccountProperties;

use super::{
    partition_endpoint_state::{
        PartitionEndpointState, PartitionFailoverConfig, PartitionFailoverEntry,
    },
    AccountEndpointState, CosmosEndpoint, UnavailablePartition, UnavailableReason,
};

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
        preferred_read_endpoints,
        preferred_write_endpoints,
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
        let url = match url::Url::parse(&region.database_account_endpoint) {
            Ok(url) => url,
            Err(err) => {
                warn!(
                    region = %region.name,
                    endpoint = %region.database_account_endpoint,
                    error = %err,
                    "Ignoring malformed standard endpoint URL from AccountProperties"
                );
                continue;
            }
        };

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
        let url = match url::Url::parse(&region.database_account_endpoint) {
            Ok(url) => url,
            Err(err) => {
                warn!(
                    region = %region.name,
                    endpoint = %region.database_account_endpoint,
                    error = %err,
                    "Ignoring malformed thin-client endpoint URL from AccountProperties"
                );
                continue;
            }
        };

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

// ── Partition-Level Routing Systems ────────────────────────────────────

/// Returns `true` if the request is eligible for PPAF.
///
/// PPAF applies to writes on single-master accounts when the feature is enabled.
pub(crate) fn is_eligible_for_ppaf(
    partition_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    is_read_only: bool,
    is_partitioned_resource: bool,
) -> bool {
    partition_state.ppaf_enabled
        && is_partitioned_resource
        && !is_read_only
        && !account_state.multiple_write_locations_enabled
        && account_state.preferred_read_endpoints.len() > 1
}

/// Returns `true` if the request is eligible for PPCB.
///
/// PPCB applies to reads (any account) and writes on multi-master accounts
/// when the feature is enabled.
pub(crate) fn is_eligible_for_ppcb(
    partition_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    is_read_only: bool,
    is_partitioned_resource: bool,
) -> bool {
    partition_state.ppcb_enabled
        && is_partitioned_resource
        && (is_read_only || account_state.multiple_write_locations_enabled)
        && account_state.preferred_read_endpoints.len() > 1
}

/// Returns `true` if the circuit breaker threshold has been exceeded.
pub(crate) fn can_circuit_breaker_trigger_failover(
    entry: &PartitionFailoverEntry,
    is_read_only: bool,
    config: &PartitionFailoverConfig,
) -> bool {
    if is_read_only {
        entry.consecutive_read_failures > config.read_failure_threshold
    } else {
        entry.consecutive_write_failures > config.write_failure_threshold
    }
}

/// Marks a partition as unavailable, producing a new `PartitionEndpointState`.
///
/// This is a pure function: it takes the current state and produces a new one.
/// The caller is responsible for the CAS swap.
pub(crate) fn mark_partition_unavailable(
    current_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    unavail: &UnavailablePartition,
    is_partitioned_resource: bool,
) -> PartitionEndpointState {
    let mut new_state = current_state.clone();
    let now = Instant::now();

    let is_read = unavail.is_read;
    let failed_endpoint = match &unavail.region {
        Some(region) => {
            // Find the endpoint matching the failed region
            account_state
                .preferred_read_endpoints
                .iter()
                .chain(account_state.preferred_write_endpoints.iter())
                .find(|e| e.region().is_some_and(|r| r == region))
                .cloned()
        }
        None => None,
    };

    let Some(failed_endpoint) = failed_endpoint else {
        return new_state;
    };

    // Determine which mechanism applies and which endpoint list to use.
    let next_endpoints = &account_state.preferred_read_endpoints;

    if is_eligible_for_ppcb(
        current_state,
        account_state,
        is_read,
        is_partitioned_resource,
    ) {
        let entry = new_state
            .ppcb_overrides
            .entry(unavail.partition_key_range_id.clone())
            .or_insert_with(|| PartitionFailoverEntry {
                current_endpoint: failed_endpoint.clone(),
                first_failed_endpoint: failed_endpoint.clone(),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: now,
                last_failure_time: now,
            });

        // Reset counters if the counter reset window has elapsed.
        if now.saturating_duration_since(entry.last_failure_time)
            > current_state.config.counter_reset_window
        {
            entry.consecutive_read_failures = 0;
            entry.consecutive_write_failures = 0;
        }

        // Increment appropriate counter.
        if is_read {
            entry.consecutive_read_failures += 1;
        } else {
            entry.consecutive_write_failures += 1;
        }
        entry.last_failure_time = now;

        // Check if threshold is exceeded; if not, return without moving endpoint.
        if !can_circuit_breaker_trigger_failover(entry, is_read, &current_state.config) {
            return new_state;
        }

        // Try to move to next endpoint.
        if !try_move_next_endpoint(entry, next_endpoints, &failed_endpoint) {
            // All endpoints exhausted — remove entry to restore default routing.
            new_state
                .ppcb_overrides
                .remove(&unavail.partition_key_range_id);
        }
    } else if is_eligible_for_ppaf(
        current_state,
        account_state,
        is_read,
        is_partitioned_resource,
    ) {
        let entry = new_state
            .ppaf_overrides
            .entry(unavail.partition_key_range_id.clone())
            .or_insert_with(|| PartitionFailoverEntry {
                current_endpoint: failed_endpoint.clone(),
                first_failed_endpoint: failed_endpoint.clone(),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: now,
                last_failure_time: now,
            });

        entry.last_failure_time = now;

        // PPAF unconditionally moves to the next endpoint.
        if !try_move_next_endpoint(entry, next_endpoints, &failed_endpoint) {
            new_state
                .ppaf_overrides
                .remove(&unavail.partition_key_range_id);
        }
    }

    new_state
}

/// Tries to advance a partition failover entry to the next available endpoint.
///
/// Returns `true` if the endpoint was moved, `false` if all endpoints are exhausted.
fn try_move_next_endpoint(
    entry: &mut PartitionFailoverEntry,
    next_endpoints: &[CosmosEndpoint],
    failed_endpoint: &CosmosEndpoint,
) -> bool {
    // If a concurrent CAS already moved the endpoint, count it as success.
    if *failed_endpoint != entry.current_endpoint {
        return true;
    }

    entry.failed_endpoints.insert(failed_endpoint.clone());

    for candidate in next_endpoints {
        if candidate == &entry.current_endpoint {
            continue;
        }
        if entry.failed_endpoints.contains(candidate) {
            continue;
        }
        entry.current_endpoint = candidate.clone();
        return true;
    }

    false
}

/// Removes expired partition override entries, producing a new `PartitionEndpointState`.
///
/// This is a pure function for the background failback loop.
pub(crate) fn expire_partition_overrides(
    state: &PartitionEndpointState,
    now: Instant,
    unavailability_duration: Duration,
) -> PartitionEndpointState {
    let mut new_state = state.clone();

    new_state.ppcb_overrides.retain(|_, entry| {
        now.saturating_duration_since(entry.first_failure_time) < unavailability_duration
    });

    new_state.ppaf_overrides.retain(|_, entry| {
        now.saturating_duration_since(entry.first_failure_time) < unavailability_duration
    });

    new_state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::cache::AccountProperties;
    use crate::options::Region;
    use std::collections::HashSet;

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

    // ── Helpers for partition-level tests ───────────────────────────────

    fn regional_endpoint(name: &str) -> CosmosEndpoint {
        CosmosEndpoint::regional(
            Region::new(name.to_string()),
            url::Url::parse(&format!("https://test-{name}.documents.azure.com:443/")).unwrap(),
        )
    }

    /// Account state with two read endpoints and one write endpoint (single-master).
    fn single_master_account() -> AccountEndpointState {
        AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![
                regional_endpoint("eastus"),
                regional_endpoint("westus"),
            ],
            preferred_write_endpoints: vec![regional_endpoint("eastus")],
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default_endpoint(),
        }
    }

    /// Account state with two read/write endpoints (multi-master).
    fn multi_master_account() -> AccountEndpointState {
        AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![
                regional_endpoint("eastus"),
                regional_endpoint("westus"),
            ],
            preferred_write_endpoints: vec![
                regional_endpoint("eastus"),
                regional_endpoint("westus"),
            ],
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: default_endpoint(),
        }
    }

    fn partition_state_ppaf_ppcb_enabled() -> PartitionEndpointState {
        PartitionEndpointState {
            ppaf_enabled: true,
            ppcb_enabled: true,
            ..PartitionEndpointState::default()
        }
    }

    fn unavailable_partition(
        pk_range_id: &str,
        region: &str,
        is_read: bool,
    ) -> UnavailablePartition {
        UnavailablePartition {
            partition_key_range_id: pk_range_id.to_string(),
            region: Some(Region::new(region.to_string())),
            is_read,
            is_partitioned_resource: true,
        }
    }

    // ── PPAF eligibility tests ────────────────────────────────────────

    #[test]
    fn ppaf_eligible_for_write_on_single_master() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_for_read() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, true, true));
    }

    #[test]
    fn ppaf_not_eligible_for_multi_master() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = multi_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_when_disabled() {
        let ps = PartitionEndpointState {
            ppaf_enabled: false,
            ..PartitionEndpointState::default()
        };
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_for_non_partitioned_resource() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, false));
    }

    #[test]
    fn ppaf_not_eligible_with_single_read_endpoint() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = AccountEndpointState {
            preferred_read_endpoints: vec![regional_endpoint("eastus")],
            ..single_master_account()
        };
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    // ── PPCB eligibility tests ────────────────────────────────────────

    #[test]
    fn ppcb_eligible_for_read_on_single_master() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(is_eligible_for_ppcb(&ps, &account, true, true));
    }

    #[test]
    fn ppcb_eligible_for_write_on_multi_master() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = multi_master_account();
        assert!(is_eligible_for_ppcb(&ps, &account, false, true));
    }

    #[test]
    fn ppcb_not_eligible_for_write_on_single_master() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        // Writes on single-master go through PPAF, not PPCB
        assert!(!is_eligible_for_ppcb(&ps, &account, false, true));
    }

    #[test]
    fn ppcb_not_eligible_when_disabled() {
        let ps = PartitionEndpointState {
            ppcb_enabled: false,
            ..PartitionEndpointState::default()
        };
        let account = single_master_account();
        assert!(!is_eligible_for_ppcb(&ps, &account, true, true));
    }

    #[test]
    fn ppcb_not_eligible_for_non_partitioned_resource() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(!is_eligible_for_ppcb(&ps, &account, true, false));
    }

    // ── Circuit breaker threshold tests ───────────────────────────────

    #[test]
    fn circuit_breaker_not_triggered_below_threshold() {
        let config = PartitionFailoverConfig::default();
        let entry = PartitionFailoverEntry {
            current_endpoint: regional_endpoint("eastus"),
            first_failed_endpoint: regional_endpoint("eastus"),
            failed_endpoints: Default::default(),
            consecutive_read_failures: 1, // below threshold of 2
            consecutive_write_failures: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
        };
        assert!(!can_circuit_breaker_trigger_failover(&entry, true, &config));
    }

    #[test]
    fn circuit_breaker_triggered_above_read_threshold() {
        let config = PartitionFailoverConfig::default();
        let entry = PartitionFailoverEntry {
            current_endpoint: regional_endpoint("eastus"),
            first_failed_endpoint: regional_endpoint("eastus"),
            failed_endpoints: Default::default(),
            consecutive_read_failures: 3, // above threshold of 2
            consecutive_write_failures: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
        };
        assert!(can_circuit_breaker_trigger_failover(&entry, true, &config));
    }

    #[test]
    fn circuit_breaker_triggered_above_write_threshold() {
        let config = PartitionFailoverConfig::default();
        let entry = PartitionFailoverEntry {
            current_endpoint: regional_endpoint("eastus"),
            first_failed_endpoint: regional_endpoint("eastus"),
            failed_endpoints: Default::default(),
            consecutive_read_failures: 0,
            consecutive_write_failures: 6, // above threshold of 5
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
        };
        assert!(can_circuit_breaker_trigger_failover(&entry, false, &config));
    }

    // ── mark_partition_unavailable tests ──────────────────────────────

    #[test]
    fn mark_partition_unavailable_ppaf_creates_entry() {
        // PPAF: write on single-master with ppaf enabled
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // PPAF should have an entry; PPCB should not
        assert_eq!(result.ppaf_overrides.len(), 1);
        assert!(result.ppcb_overrides.is_empty());
        let entry = &result.ppaf_overrides["pk-1"];
        // Should have moved to westus (the other read endpoint)
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));
        assert!(entry
            .failed_endpoints
            .contains(&regional_endpoint("eastus")));
    }

    #[test]
    fn mark_partition_unavailable_ppcb_increments_counter() {
        // PPCB: read on single-master with ppcb enabled
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // PPCB entry should exist
        assert_eq!(result.ppcb_overrides.len(), 1);
        assert!(result.ppaf_overrides.is_empty());
        let entry = &result.ppcb_overrides["pk-1"];
        // Below threshold (1 failure, threshold is 2) so endpoint should NOT have moved
        assert_eq!(entry.consecutive_read_failures, 1);
        assert_eq!(entry.current_endpoint, regional_endpoint("eastus"));
    }

    #[test]
    fn mark_partition_unavailable_ppcb_exceeds_threshold_moves_endpoint() {
        // Start with an existing PPCB entry that has 2 consecutive read failures
        let mut ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        ps.ppcb_overrides.insert(
            "pk-1".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 2,
                consecutive_write_failures: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        let entry = &result.ppcb_overrides["pk-1"];
        // 3 failures > threshold of 2, so endpoint should have moved to westus
        assert_eq!(entry.consecutive_read_failures, 3);
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));
    }

    #[test]
    fn mark_partition_unavailable_all_endpoints_exhausted_removes_entry() {
        // PPAF with all read endpoints already failed
        let mut ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        let mut failed = HashSet::new();
        failed.insert(regional_endpoint("westus"));
        ps.ppaf_overrides.insert(
            "pk-1".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: failed,
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // Entry removed because all endpoints exhausted
        assert!(result.ppaf_overrides.is_empty());
    }

    #[test]
    fn mark_partition_unavailable_unknown_region_returns_unchanged() {
        let ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "nonexistent", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        assert!(result.ppaf_overrides.is_empty());
        assert!(result.ppcb_overrides.is_empty());
    }

    #[test]
    fn mark_partition_unavailable_ppcb_counter_reset_after_window() {
        // Entry with stale last_failure_time beyond counter_reset_window
        let stale_time = Instant::now() - Duration::from_secs(10 * 60);
        let mut ps = partition_state_ppaf_ppcb_enabled();
        let account = single_master_account();
        ps.ppcb_overrides.insert(
            "pk-1".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 100,
                consecutive_write_failures: 100,
                first_failure_time: stale_time,
                last_failure_time: stale_time,
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        let entry = &result.ppcb_overrides["pk-1"];
        // Counters should have been reset, then incremented by 1
        assert_eq!(entry.consecutive_read_failures, 1);
        assert_eq!(entry.consecutive_write_failures, 0);
    }

    // ── expire_partition_overrides tests ──────────────────────────────

    #[test]
    fn expire_partition_overrides_removes_old_entries() {
        let old_time = Instant::now() - Duration::from_secs(60);
        let recent_time = Instant::now();

        let mut state = partition_state_ppaf_ppcb_enabled();
        state.ppaf_overrides.insert(
            "old-pk".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: old_time,
                last_failure_time: old_time,
            },
        );
        state.ppaf_overrides.insert(
            "recent-pk".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: recent_time,
                last_failure_time: recent_time,
            },
        );
        state.ppcb_overrides.insert(
            "old-ppcb".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: old_time,
                last_failure_time: old_time,
            },
        );

        let expired = expire_partition_overrides(&state, Instant::now(), Duration::from_secs(30));

        // Old entries removed, recent entry retained
        assert_eq!(expired.ppaf_overrides.len(), 1);
        assert!(expired.ppaf_overrides.contains_key("recent-pk"));
        assert!(expired.ppcb_overrides.is_empty());
    }

    #[test]
    fn expire_partition_overrides_keeps_all_when_none_expired() {
        let recent = Instant::now();
        let mut state = partition_state_ppaf_ppcb_enabled();
        state.ppaf_overrides.insert(
            "pk-1".to_string(),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                consecutive_read_failures: 0,
                consecutive_write_failures: 0,
                first_failure_time: recent,
                last_failure_time: recent,
            },
        );

        let expired = expire_partition_overrides(&state, Instant::now(), Duration::from_secs(60));

        assert_eq!(expired.ppaf_overrides.len(), 1);
    }
}
