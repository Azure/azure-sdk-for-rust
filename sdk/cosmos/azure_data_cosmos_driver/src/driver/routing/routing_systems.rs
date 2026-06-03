// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure routing systems for account and partition endpoint state.

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use tracing::warn;

use crate::driver::cache::AccountProperties;
use crate::options::Region;

use super::{
    partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverConfig, PartitionFailoverEntry,
    },
    partition_key_range_id::PartitionKeyRangeId,
    AccountEndpointState, CosmosEndpoint, UnavailablePartition, UnavailableReason,
};

/// Builds account endpoint state from account metadata.
///
/// When `preferred_regions` is non-empty, read and write endpoint lists are
/// reordered so that endpoints matching the preferred regions appear first
/// (in preference order). Endpoints whose region is not in the preferred
/// list are appended in their original account-metadata order.
pub(crate) fn build_account_endpoint_state(
    properties: &AccountProperties,
    default_endpoint: CosmosEndpoint,
    previous_generation: Option<u64>,
    gateway20_enabled: bool,
    preferred_regions: &[Region],
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

    if !preferred_regions.is_empty() {
        preferred_read_endpoints =
            reorder_by_preferred_regions(preferred_read_endpoints, preferred_regions);
        preferred_write_endpoints =
            reorder_by_preferred_regions(preferred_write_endpoints, preferred_regions);
    }

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
    gateway20_locations: &[crate::driver::cache::AccountRegion],
    gateway20_enabled: bool,
) -> Vec<CosmosEndpoint> {
    let gateway20_urls = if gateway20_enabled {
        parse_gateway20_locations(gateway20_locations)
    } else {
        HashMap::new()
    };

    let mut endpoints = Vec::with_capacity(standard_locations.len());
    for region in standard_locations {
        let url = region.database_account_endpoint.url().clone();

        let endpoint = gateway20_urls
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

fn parse_gateway20_locations(
    gateway20_locations: &[crate::driver::cache::AccountRegion],
) -> HashMap<crate::options::Region, url::Url> {
    let mut urls = HashMap::new();

    for region in gateway20_locations {
        let url = region.database_account_endpoint.url().clone();

        if url.scheme() != "https" {
            warn!(
                region = %region.name,
                endpoint = %region.database_account_endpoint,
                scheme = url.scheme(),
                "Ignoring non-HTTPS Gateway 2.0 endpoint URL"
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
                        "Duplicate Gateway 2.0 region with conflicting URL; keeping first entry"
                    );
                }
            })
            .or_insert(url);
    }

    urls
}

/// Reorders endpoints so that those matching `preferred_regions` appear first,
/// in the same order as the preference list. Endpoints whose region is not in
/// the preference list (or that have no region, e.g. global endpoints) are
/// appended in their original order.
fn reorder_by_preferred_regions(
    endpoints: Vec<CosmosEndpoint>,
    preferred_regions: &[Region],
) -> Vec<CosmosEndpoint> {
    let mut ordered = Vec::with_capacity(endpoints.len());
    let mut remaining: Vec<Option<CosmosEndpoint>> = endpoints.into_iter().map(Some).collect();

    for region in preferred_regions {
        for slot in remaining.iter_mut() {
            if let Some(ep) = slot {
                if ep.region().is_some_and(|r| r == region) {
                    ordered.push(slot.take().unwrap());
                    break;
                }
            }
        }
    }

    // Append any endpoints not matched by the preferred list.
    for ep in remaining.into_iter().flatten() {
        ordered.push(ep);
    }

    ordered
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
    partition_state.per_partition_automatic_failover_enabled
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
    partition_state.per_partition_circuit_breaker_enabled
        && is_partitioned_resource
        && (is_read_only || account_state.multiple_write_locations_enabled)
        && account_state.preferred_read_endpoints.len() > 1
}

/// Returns `true` if the circuit breaker threshold has been met.
pub(crate) fn can_circuit_breaker_trigger_failover(
    entry: &PartitionFailoverEntry,
    is_read_only: bool,
    config: &PartitionFailoverConfig,
) -> bool {
    if is_read_only {
        entry.read_failure_count >= config.read_failure_threshold
    } else {
        entry.write_failure_count >= config.write_failure_threshold
    }
}

/// Samples a per-entry failback jitter in `[0, unavailability_duration / 2)`.
///
/// The jitter is added to `partition_unavailability_duration` before an
/// `Unhealthy` PPCB entry is allowed to transition to `ProbeCandidate`. This
/// spreads out the failback of partitions that all failed in the same
/// burst, preventing a thundering-herd stampede on the recovering region.
///
/// To remain dependency-free we avoid `rand` and instead seed a SplitMix64
/// finalizer with two sources:
///   * the **full** `SystemTime` epoch-nanosecond value (not just
///     `subsec_nanos`, which caps the modulo at ~1 second and erases the
///     upper half of the intended window whenever
///     `unavailability_duration / 2 > 1 s`);
///   * a hash of the partition key range ID, so that many entries created
///     on the same coarse clock tick (e.g. ~15 ms wall-clock granularity on
///     Windows during a burst) still receive different jitter values
///     instead of all converging on the same fail-back deadline.
///
/// Full statistical randomness is not required — only enough variance to
/// break correlation across many partitions failing within the same
/// recovery window.
fn ppcb_failback_jitter(
    unavailability_duration: Duration,
    pk_range_id: &PartitionKeyRangeId,
) -> Duration {
    let max_jitter_nanos = (unavailability_duration / 2).as_nanos() as u64;
    if max_jitter_nanos == 0 {
        return Duration::ZERO;
    }
    let now_nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    pk_range_id.as_str().hash(&mut hasher);
    let pk_hash = hasher.finish();
    // SplitMix64 finalizer scrambles the combined seed so adjacent inputs
    // (e.g. two pk_range_ids hashed in the same nanosecond) produce
    // uncorrelated outputs across the full [0, max_jitter_nanos) range.
    let mut z = now_nanos ^ pk_hash;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    Duration::from_nanos(z % max_jitter_nanos)
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

    // Caller (apply) guarantees partition_key_range_id is Some; guard defensively.
    let Some(pk_range_id) = &unavail.partition_key_range_id else {
        return new_state;
    };

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
            .circuit_breaker_overrides
            .entry(pk_range_id.clone())
            .or_insert_with(|| PartitionFailoverEntry {
                current_endpoint: failed_endpoint.clone(),
                first_failed_endpoint: failed_endpoint.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: now,
                last_failure_time: now,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: ppcb_failback_jitter(
                    current_state.config.partition_unavailability_duration,
                    pk_range_id,
                ),
            });

        // If probe failed, transition back to Unhealthy and fully reset
        // the entry so that the re-failover path accumulates failures
        // from zero against the original endpoint. Without this reset
        // the old high counters would cause `can_circuit_breaker_trigger_failover`
        // to return true immediately, bypassing threshold-based probing.
        if entry.health_status == HealthStatus::ProbeCandidate {
            entry.health_status = HealthStatus::Unhealthy;
            entry.read_failure_count = 0;
            entry.write_failure_count = 0;
            entry.current_endpoint = entry.first_failed_endpoint.clone();
            entry.failed_endpoints.clear();
            entry.first_failure_time = now;
            entry.last_failure_time = now;
            // Re-sample jitter so the next failback window is offset from
            // any other entries that may have failed on the same tick.
            entry.failback_jitter = ppcb_failback_jitter(
                current_state.config.partition_unavailability_duration,
                pk_range_id,
            );
            return new_state;
        }

        // Reset counters if the counter reset window has elapsed.
        if now.saturating_duration_since(entry.last_failure_time)
            > current_state.config.counter_reset_window
        {
            entry.read_failure_count = 0;
            entry.write_failure_count = 0;
        }

        // Increment appropriate counter.
        if is_read {
            entry.read_failure_count += 1;
        } else {
            entry.write_failure_count += 1;
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
                .circuit_breaker_overrides
                .remove(pk_range_id.as_str());
        }
    } else if is_eligible_for_ppaf(
        current_state,
        account_state,
        is_read,
        is_partitioned_resource,
    ) {
        let entry = new_state
            .failover_overrides
            .entry(pk_range_id.clone())
            .or_insert_with(|| PartitionFailoverEntry {
                current_endpoint: failed_endpoint.clone(),
                first_failed_endpoint: failed_endpoint.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: now,
                last_failure_time: now,
                health_status: HealthStatus::Unhealthy,
                // PPAF entries don't participate in background failback,
                // so jitter is unused.
                failback_jitter: Duration::ZERO,
            });

        entry.last_failure_time = now;

        // PPAF unconditionally moves to the next endpoint.
        // Note: PPAF entries never transition to ProbeCandidate — background
        // failback only applies to PPCB. PPAF overrides persist until the
        // backend signals a change organically.
        if !try_move_next_endpoint(entry, next_endpoints, &failed_endpoint) {
            new_state.failover_overrides.remove(pk_range_id.as_str());
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

/// Transitions expired partition entries from `Unhealthy` to `ProbeCandidate`,
/// producing a new `PartitionEndpointState`.
///
/// Each entry uses its own `failback_jitter` to spread the transition across
/// the failback window, mitigating a thundering-herd effect on the recovering
/// region when many partitions failed at the same time.
///
/// Entries already in `ProbeCandidate` state are not re-transitioned.
/// This is a pure function called by the failback sweep.
pub(crate) fn expire_partition_overrides(
    state: &PartitionEndpointState,
    now: Instant,
    unavailability_duration: Duration,
) -> PartitionEndpointState {
    let mut new_state = state.clone();

    for entry in new_state.circuit_breaker_overrides.values_mut() {
        if entry.health_status == HealthStatus::Unhealthy
            && now.saturating_duration_since(entry.first_failure_time)
                >= unavailability_duration + entry.failback_jitter
        {
            entry.health_status = HealthStatus::ProbeCandidate;
        }
    }

    // Note: failover_overrides (PPAF) are intentionally NOT swept here.
    // PPAF failovers for writes on single-master accounts persist until the
    // backend signals a change (e.g., a new 403/3 from the override region).
    // Only PPCB (circuit breaker) overrides use probe-based failback.

    new_state
}

/// Removes a probe-successful entry from the circuit breaker overrides.
///
/// Called when a PPCB probe request succeeds (the request completed without errors).
/// This causes the partition to return to default routing.
///
/// Note: Only PPCB (circuit breaker) overrides are removed here. PPAF (failover)
/// overrides do not use probe-based failback — they persist until the backend
/// signals a change organically.
pub(crate) fn remove_probe_succeeded_entry(
    state: &PartitionEndpointState,
    partition_key_range_id: &PartitionKeyRangeId,
) -> PartitionEndpointState {
    let mut new_state = state.clone();
    new_state
        .circuit_breaker_overrides
        .remove(partition_key_range_id.as_str());
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
            build_account_endpoint_state(&test_properties(), default_endpoint(), None, false, &[]);
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

        let state = build_account_endpoint_state(&properties, default_endpoint(), None, true, &[]);

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

        let state = build_account_endpoint_state(&properties, default_endpoint(), None, true, &[]);

        assert!(state.preferred_read_endpoints[0].gateway20_url().is_some());
        assert!(state.preferred_write_endpoints[0].gateway20_url().is_some());
    }

    /// Regression guard for the "service stops advertising Gateway 2.0" case.
    ///
    /// When the account previously returned `thinClient*Locations` but a
    /// subsequent metadata refresh omits them (e.g., the service rolled the
    /// account off Gateway 2.0, or thin-client routing was disabled
    /// region-side), the rebuilt endpoint state must drop the
    /// `gateway20_url` on every endpoint so that follow-up requests route
    /// through the standard compute-gateway URL.
    ///
    /// `uses_gateway20(prefer_gateway20=true)` checks
    /// `gateway20_url.is_some()`, so a `None` URL is sufficient to force
    /// the request pipeline back onto the standard gateway transport even
    /// while the operator-level `gateway20_enabled` toggle remains on.
    #[test]
    fn build_state_drops_gateway20_when_thin_client_locations_disappear() {
        // First refresh: account advertises both thin-client read and write
        // endpoints. With `gateway20_enabled=true`, every preferred endpoint
        // must carry a `gateway20_url`.
        let with_g2: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }],
            "thinClientWritableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/" }],
            "thinClientReadableLocations": [{ "name": "westus2", "databaseAccountEndpoint": "https://test-westus2-thin.documents.azure.com:444/" }],
            "enableMultipleWriteLocations": true,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        })).unwrap();

        let initial = build_account_endpoint_state(&with_g2, default_endpoint(), None, true, &[]);
        assert!(
            initial.preferred_read_endpoints[0]
                .gateway20_url()
                .is_some(),
            "initial read endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            initial.preferred_write_endpoints[0]
                .gateway20_url()
                .is_some(),
            "initial write endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            initial.preferred_read_endpoints[0].uses_gateway20(true),
            "initial read endpoint must route through Gateway 2.0 when prefer_gateway20=true"
        );
        assert!(
            initial.preferred_write_endpoints[0].uses_gateway20(true),
            "initial write endpoint must route through Gateway 2.0 when prefer_gateway20=true"
        );

        // Second refresh: same standard `writable`/`readable` endpoints, but
        // the service has stopped returning thin-client locations. Mimics
        // the database-account call no longer advertising Gateway 2.0.
        let without_g2: AccountProperties = serde_json::from_value(serde_json::json!({
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
        })).unwrap();

        // `gateway20_enabled` is still `true` on the store — only the wire
        // payload has changed. The rebuilt endpoints must nevertheless have
        // no `gateway20_url`, forcing fallback to the compute gateway.
        let rebuilt = build_account_endpoint_state(
            &without_g2,
            default_endpoint(),
            Some(initial.generation),
            true,
            &[],
        );
        assert_eq!(rebuilt.generation, initial.generation + 1);
        assert!(
            rebuilt.preferred_read_endpoints[0]
                .gateway20_url()
                .is_none(),
            "read endpoint must lose its Gateway 2.0 URL when the service stops advertising thinClientReadableLocations"
        );
        assert!(
            rebuilt.preferred_write_endpoints[0]
                .gateway20_url()
                .is_none(),
            "write endpoint must lose its Gateway 2.0 URL when the service stops advertising thinClientWritableLocations"
        );
        assert!(
            !rebuilt.preferred_read_endpoints[0].uses_gateway20(true),
            "read request must fall back to the compute gateway even when the operator toggle (prefer_gateway20) is still true"
        );
        assert!(
            !rebuilt.preferred_write_endpoints[0].uses_gateway20(true),
            "write request must fall back to the compute gateway even when the operator toggle (prefer_gateway20) is still true"
        );
    }

    #[test]
    fn mark_and_expire_unavailable_endpoint() {
        let state =
            build_account_endpoint_state(&test_properties(), default_endpoint(), None, false, &[]);
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
            ]
            .into(),
            preferred_write_endpoints: vec![regional_endpoint("eastus")].into(),
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
            ]
            .into(),
            preferred_write_endpoints: vec![
                regional_endpoint("eastus"),
                regional_endpoint("westus"),
            ]
            .into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: true,
            default_endpoint: default_endpoint(),
        }
    }

    fn partition_state_with_ppaf_ppcb_enabled() -> PartitionEndpointState {
        PartitionEndpointState {
            per_partition_automatic_failover_enabled: true,
            per_partition_circuit_breaker_enabled: true,
            ..PartitionEndpointState::default()
        }
    }

    fn pk(s: &str) -> PartitionKeyRangeId {
        s.parse().unwrap()
    }

    fn unavailable_partition(
        pk_range_id: &str,
        region: &str,
        is_read: bool,
    ) -> UnavailablePartition {
        UnavailablePartition {
            partition_key_range_id: Some(pk(pk_range_id)),
            region: Some(Region::new(region.to_string())),
            is_read,
            is_partitioned_resource: true,
        }
    }

    // ── PPAF eligibility tests ────────────────────────────────────────

    #[test]
    fn ppaf_eligible_for_write_on_single_master() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_for_read() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, true, true));
    }

    #[test]
    fn ppaf_not_eligible_for_multi_master() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = multi_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_when_disabled() {
        let ps = PartitionEndpointState {
            per_partition_automatic_failover_enabled: false,
            ..PartitionEndpointState::default()
        };
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    #[test]
    fn ppaf_not_eligible_for_non_partitioned_resource() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(!is_eligible_for_ppaf(&ps, &account, false, false));
    }

    #[test]
    fn ppaf_not_eligible_with_single_read_endpoint() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = AccountEndpointState {
            preferred_read_endpoints: vec![regional_endpoint("eastus")].into(),
            ..single_master_account()
        };
        assert!(!is_eligible_for_ppaf(&ps, &account, false, true));
    }

    // ── PPCB eligibility tests ────────────────────────────────────────

    #[test]
    fn ppcb_eligible_for_read_on_single_master() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        assert!(is_eligible_for_ppcb(&ps, &account, true, true));
    }

    #[test]
    fn ppcb_eligible_for_write_on_multi_master() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = multi_master_account();
        assert!(is_eligible_for_ppcb(&ps, &account, false, true));
    }

    #[test]
    fn ppcb_not_eligible_for_write_on_single_master() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        // Writes on single-master go through PPAF, not PPCB
        assert!(!is_eligible_for_ppcb(&ps, &account, false, true));
    }

    #[test]
    fn ppcb_not_eligible_when_disabled() {
        let ps = PartitionEndpointState {
            per_partition_circuit_breaker_enabled: false,
            ..PartitionEndpointState::default()
        };
        let account = single_master_account();
        assert!(!is_eligible_for_ppcb(&ps, &account, true, true));
    }

    #[test]
    fn ppcb_not_eligible_for_non_partitioned_resource() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
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
            read_failure_count: 1, // below threshold of 10
            write_failure_count: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
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
            read_failure_count: 10, // at default threshold of 10
            write_failure_count: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
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
            read_failure_count: 0,
            write_failure_count: 5, // at threshold of 5
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        };
        assert!(can_circuit_breaker_trigger_failover(&entry, false, &config));
    }

    // ── mark_partition_unavailable tests ──────────────────────────────

    #[test]
    fn mark_partition_unavailable_ppaf_creates_entry() {
        // PPAF: write on single-master with ppaf enabled
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // PPAF should have an entry; PPCB should not
        assert_eq!(result.failover_overrides.len(), 1);
        assert!(result.circuit_breaker_overrides.is_empty());
        let entry = &result.failover_overrides["pk-1"];
        // Should have moved to westus (the other read endpoint)
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));
        assert!(entry
            .failed_endpoints
            .contains(&regional_endpoint("eastus")));
    }

    #[test]
    fn mark_partition_unavailable_ppcb_increments_counter() {
        // PPCB: read on single-master with ppcb enabled
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // PPCB entry should exist
        assert_eq!(result.circuit_breaker_overrides.len(), 1);
        assert!(result.failover_overrides.is_empty());
        let entry = &result.circuit_breaker_overrides["pk-1"];
        // Below threshold (1 failure, threshold is 10) so endpoint should NOT have moved
        assert_eq!(entry.read_failure_count, 1);
        assert_eq!(entry.current_endpoint, regional_endpoint("eastus"));
    }

    #[test]
    fn mark_partition_unavailable_ppcb_exceeds_threshold_moves_endpoint() {
        // Start with an existing PPCB entry that has 9 read failures (one below threshold)
        let mut ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        ps.circuit_breaker_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 9,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        let entry = &result.circuit_breaker_overrides["pk-1"];
        // 10 failures >= default threshold of 10, so endpoint should have moved to westus
        assert_eq!(entry.read_failure_count, 10);
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));
    }

    #[test]
    fn mark_partition_unavailable_all_endpoints_exhausted_removes_entry() {
        // PPAF with all read endpoints already failed
        let mut ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let mut failed = HashSet::new();
        failed.insert(regional_endpoint("westus"));
        ps.failover_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: failed,
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        // Entry removed because all endpoints exhausted
        assert!(result.failover_overrides.is_empty());
    }

    #[test]
    fn mark_partition_unavailable_unknown_region_returns_unchanged() {
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "nonexistent", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        assert!(result.failover_overrides.is_empty());
        assert!(result.circuit_breaker_overrides.is_empty());
    }

    #[test]
    fn mark_partition_unavailable_ppcb_counter_reset_after_window() {
        // Entry with stale last_failure_time beyond counter_reset_window
        let stale_time = Instant::now() - Duration::from_secs(10 * 60);
        let mut ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        ps.circuit_breaker_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("eastus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 100,
                write_failure_count: 100,
                first_failure_time: stale_time,
                last_failure_time: stale_time,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        let entry = &result.circuit_breaker_overrides["pk-1"];
        // Counters should have been reset, then incremented by 1
        assert_eq!(entry.read_failure_count, 1);
        assert_eq!(entry.write_failure_count, 0);
    }

    // ── expire_partition_overrides tests ──────────────────────────────

    #[test]
    fn expire_partition_overrides_transitions_old_entries_to_probe_candidate() {
        let old_time = Instant::now() - Duration::from_secs(60);
        let recent_time = Instant::now();

        let mut state = partition_state_with_ppaf_ppcb_enabled();
        state.failover_overrides.insert(
            pk("old-pk"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: old_time,
                last_failure_time: old_time,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        state.failover_overrides.insert(
            pk("recent-pk"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: recent_time,
                last_failure_time: recent_time,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );
        state.circuit_breaker_overrides.insert(
            pk("old-ppcb"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: old_time,
                last_failure_time: old_time,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let expired = expire_partition_overrides(&state, Instant::now(), Duration::from_secs(30));

        // PPCB old entry transitioned to ProbeCandidate
        assert_eq!(expired.circuit_breaker_overrides.len(), 1);
        assert_eq!(
            expired.circuit_breaker_overrides["old-ppcb"].health_status,
            HealthStatus::ProbeCandidate
        );

        // PPAF entries are NOT transitioned (background failback doesn't apply to PPAF)
        assert_eq!(expired.failover_overrides.len(), 2);
        assert_eq!(
            expired.failover_overrides["old-pk"].health_status,
            HealthStatus::Unhealthy
        );
        assert_eq!(
            expired.failover_overrides["recent-pk"].health_status,
            HealthStatus::Unhealthy
        );
    }

    #[test]
    fn expire_partition_overrides_keeps_all_when_none_expired() {
        let recent = Instant::now();
        let mut state = partition_state_with_ppaf_ppcb_enabled();
        state.failover_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: recent,
                last_failure_time: recent,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let expired = expire_partition_overrides(&state, Instant::now(), Duration::from_secs(60));

        assert_eq!(expired.failover_overrides.len(), 1);
    }

    #[test]
    fn expire_partition_overrides_does_not_retransition_probe_candidate() {
        let old_time = Instant::now() - Duration::from_secs(60);
        let mut state = partition_state_with_ppaf_ppcb_enabled();
        state.circuit_breaker_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: old_time,
                last_failure_time: old_time,
                health_status: HealthStatus::ProbeCandidate,
                failback_jitter: Duration::ZERO,
            },
        );

        let expired = expire_partition_overrides(&state, Instant::now(), Duration::from_secs(30));

        // Already ProbeCandidate, should remain ProbeCandidate (not re-transitioned)
        assert_eq!(
            expired.circuit_breaker_overrides["pk-1"].health_status,
            HealthStatus::ProbeCandidate
        );
    }

    // ── Failback jitter tests (thundering-herd mitigation) ───────────────

    #[test]
    fn ppcb_failback_jitter_stays_within_half_of_unavailability_duration() {
        let unavailability = Duration::from_secs(5);
        let max = unavailability / 2;
        // Sample many times across different partition keys and assert
        // all fall in [0, max).
        for i in 0..100 {
            let pk: PartitionKeyRangeId = format!("pk-{i}").parse().unwrap();
            let j = ppcb_failback_jitter(unavailability, &pk);
            assert!(j < max, "jitter {j:?} exceeded half-window {max:?}");
        }
    }

    #[test]
    fn ppcb_failback_jitter_is_zero_for_zero_duration() {
        let pk: PartitionKeyRangeId = "pk-0".parse().unwrap();
        assert_eq!(ppcb_failback_jitter(Duration::ZERO, &pk), Duration::ZERO);
    }

    #[test]
    fn ppcb_failback_jitter_uses_full_window_above_one_second() {
        // Regression: previously `subsec_nanos()` capped the modulo at ~1 s,
        // so a 5 s unavailability_duration (half-window = 2.5 s) could never
        // produce a jitter > 1 s. Sample many partitions and assert at least
        // one lands above the 1 s ceiling that the old impl could never reach.
        let unavailability = Duration::from_secs(10); // half-window = 5 s
        let one_second = Duration::from_secs(1);
        let mut saw_above_one_second = false;
        for i in 0..1_000 {
            let pk: PartitionKeyRangeId = format!("pk-{i}").parse().unwrap();
            if ppcb_failback_jitter(unavailability, &pk) > one_second {
                saw_above_one_second = true;
                break;
            }
        }
        assert!(
            saw_above_one_second,
            "jitter never exceeded 1 s across 1000 partitions — full window not in use"
        );
    }

    #[test]
    fn ppcb_failback_jitter_decorrelates_simultaneous_failures() {
        // Regression: previously, partitions failing within the same coarse
        // clock tick (e.g. ~15 ms on Windows wall-clock) all got the same
        // `subsec_nanos` seed and therefore the same jitter — defeating the
        // anti-thundering-herd purpose. The pk-hash mix-in must guarantee
        // distinct values across distinct partition keys even within a single
        // tight loop iteration.
        let unavailability = Duration::from_secs(5);
        let mut samples = std::collections::HashSet::new();
        for i in 0..256 {
            let pk: PartitionKeyRangeId = format!("pk-{i}").parse().unwrap();
            samples.insert(ppcb_failback_jitter(unavailability, &pk));
        }
        // Allow a tiny number of collisions (modulo bucket clashes are
        // possible by chance) but require the vast majority to be distinct.
        assert!(
            samples.len() >= 250,
            "expected ≥250 distinct jitter values across 256 partition keys, got {}",
            samples.len()
        );
    }

    #[test]
    fn expire_partition_overrides_respects_failback_jitter() {
        // An entry whose `first_failure_time + unavailability_duration` has just
        // elapsed must NOT transition while the jitter portion is still pending.
        let unavailability = Duration::from_secs(5);
        let jitter = Duration::from_secs(2);
        let first_failure = Instant::now() - unavailability;

        let mut state = partition_state_with_ppaf_ppcb_enabled();
        state.circuit_breaker_overrides.insert(
            pk("jittered-pk"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: first_failure,
                last_failure_time: first_failure,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: jitter,
            },
        );

        // Sweep at exactly `first_failure + unavailability` — jitter not yet elapsed.
        let expired =
            expire_partition_overrides(&state, first_failure + unavailability, unavailability);
        assert_eq!(
            expired.circuit_breaker_overrides["jittered-pk"].health_status,
            HealthStatus::Unhealthy,
            "entry must remain Unhealthy until unavailability + jitter has elapsed"
        );

        // Sweep at `first_failure + unavailability + jitter` — should now flip.
        let expired = expire_partition_overrides(
            &state,
            first_failure + unavailability + jitter,
            unavailability,
        );
        assert_eq!(
            expired.circuit_breaker_overrides["jittered-pk"].health_status,
            HealthStatus::ProbeCandidate,
        );
    }

    #[test]
    fn mark_partition_unavailable_ppcb_samples_jitter() {
        // A new PPCB entry must be created with a non-default jitter that lies
        // within `[0, partition_unavailability_duration / 2)`. This is best-effort:
        // the helper is seeded from `SystemTime` nanos, so we sample multiple
        // times to assert the field is actually populated and bounded.
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);
        let entry = &result.circuit_breaker_overrides["pk-1"];
        let max = ps.config.partition_unavailability_duration / 2;
        assert!(
            entry.failback_jitter < max,
            "jitter {:?} must be < half of unavailability window {:?}",
            entry.failback_jitter,
            max
        );
    }

    #[test]
    fn mark_partition_unavailable_ppaf_uses_zero_jitter() {
        // PPAF entries don't participate in background failback, so they must
        // always have zero jitter (no need to randomize a window that's never
        // consulted).
        let ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let unavail = unavailable_partition("pk-1", "eastus", false);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);
        let entry = &result.failover_overrides["pk-1"];
        assert_eq!(entry.failback_jitter, Duration::ZERO);
    }
    #[test]
    fn probe_failure_transitions_back_to_unhealthy() {
        let mut ps = partition_state_with_ppaf_ppcb_enabled();
        let account = single_master_account();
        let mut failed = HashSet::new();
        failed.insert(regional_endpoint("eastus"));
        ps.circuit_breaker_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: failed,
                read_failure_count: 3,
                write_failure_count: 0,
                first_failure_time: Instant::now() - Duration::from_secs(10),
                last_failure_time: Instant::now() - Duration::from_secs(10),
                health_status: HealthStatus::ProbeCandidate,
                failback_jitter: Duration::ZERO,
            },
        );
        let unavail = unavailable_partition("pk-1", "eastus", true);

        let result = mark_partition_unavailable(&ps, &account, &unavail, true);

        let entry = &result.circuit_breaker_overrides["pk-1"];
        assert_eq!(entry.health_status, HealthStatus::Unhealthy);
        // Counters must be reset so re-failover respects the threshold.
        assert_eq!(entry.read_failure_count, 0);
        assert_eq!(entry.write_failure_count, 0);
        // Endpoint and failed set must be reset for a fresh failover cycle.
        assert_eq!(entry.current_endpoint, regional_endpoint("eastus"));
        assert!(entry.failed_endpoints.is_empty());
    }

    /// Full PPCB cycle: failover → failback → probe failure → re-failover
    /// must respect the threshold again.
    ///
    /// Regression test: previously, a failed probe left old high counters
    /// intact, causing `can_circuit_breaker_trigger_failover` to return
    /// true immediately — bypassing the threshold entirely on re-failover.
    #[test]
    fn ppcb_re_failover_after_probe_failure_respects_threshold() {
        let account = single_master_account();
        let threshold = PartitionFailoverConfig::default().read_failure_threshold;

        // ── Phase 1: Initial failover (eastus → westus) ─────────────
        let mut ps = partition_state_with_ppaf_ppcb_enabled();
        let unavail = unavailable_partition("pk-1", "eastus", true);

        // Simulate (threshold + 1) failures to exceed the threshold.
        for _ in 0..=(threshold as usize) {
            ps = mark_partition_unavailable(&ps, &account, &unavail, true);
        }
        let entry = &ps.circuit_breaker_overrides["pk-1"];
        assert_eq!(entry.read_failure_count, threshold + 1);
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));

        // ── Phase 2: Failback sweep → ProbeCandidate ────────────────
        ps = expire_partition_overrides(
            &ps,
            Instant::now() + Duration::from_secs(60),
            Duration::from_secs(5),
        );
        assert_eq!(
            ps.circuit_breaker_overrides["pk-1"].health_status,
            HealthStatus::ProbeCandidate,
        );

        // ── Phase 3: Probe FAILS → entry resets ─────────────────────
        ps = mark_partition_unavailable(&ps, &account, &unavail, true);
        let entry = &ps.circuit_breaker_overrides["pk-1"];
        assert_eq!(entry.health_status, HealthStatus::Unhealthy);
        // Critical: counters must be 0 so the threshold gates re-failover.
        assert_eq!(entry.read_failure_count, 0);
        assert_eq!(entry.current_endpoint, regional_endpoint("eastus"));
        assert!(!can_circuit_breaker_trigger_failover(
            entry, true, &ps.config
        ));

        // ── Phase 4: Re-failover must accumulate threshold failures ──
        // Failures 1..(threshold-1) stay below the threshold; endpoint must not move.
        for i in 1..threshold {
            ps = mark_partition_unavailable(&ps, &account, &unavail, true);
            let entry = &ps.circuit_breaker_overrides["pk-1"];
            assert_eq!(entry.read_failure_count, i);
            assert_eq!(entry.current_endpoint, regional_endpoint("eastus"));
        }

        // The threshold-th failure meets the threshold → failover to westus.
        ps = mark_partition_unavailable(&ps, &account, &unavail, true);
        let entry = &ps.circuit_breaker_overrides["pk-1"];
        assert_eq!(entry.read_failure_count, threshold);
        assert_eq!(entry.current_endpoint, regional_endpoint("westus"));
    }

    #[test]
    fn remove_probe_succeeded_entry_removes_from_circuit_breaker_only() {
        let mut state = partition_state_with_ppaf_ppcb_enabled();
        state.circuit_breaker_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::ProbeCandidate,
                failback_jitter: Duration::ZERO,
            },
        );
        state.failover_overrides.insert(
            pk("pk-1"),
            PartitionFailoverEntry {
                current_endpoint: regional_endpoint("westus"),
                first_failed_endpoint: regional_endpoint("eastus"),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let result = remove_probe_succeeded_entry(&state, &pk("pk-1"));

        // PPCB entry removed, PPAF entry preserved
        assert!(result.circuit_breaker_overrides.is_empty());
        assert_eq!(result.failover_overrides.len(), 1);
        assert_eq!(
            result.failover_overrides["pk-1"].health_status,
            HealthStatus::Unhealthy
        );
    }

    fn multi_region_properties() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [
                { "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" },
                { "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
                { "name": "westus3", "databaseAccountEndpoint": "https://test-westus3.documents.azure.com:443/" }
            ],
            "readableLocations": [
                { "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" },
                { "name": "westus2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
                { "name": "westus3", "databaseAccountEndpoint": "https://test-westus3.documents.azure.com:443/" }
            ],
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
    fn preferred_regions_reorder_write_endpoints() {
        let preferred = vec![Region::WEST_US_3, Region::EAST_US];
        let state = build_account_endpoint_state(
            &multi_region_properties(),
            default_endpoint(),
            None,
            false,
            &preferred,
        );

        assert_eq!(state.preferred_write_endpoints.len(), 3);
        assert_eq!(
            state.preferred_write_endpoints[0].region().unwrap(),
            &Region::WEST_US_3
        );
        assert_eq!(
            state.preferred_write_endpoints[1].region().unwrap(),
            &Region::EAST_US
        );
        assert_eq!(
            state.preferred_write_endpoints[2].region().unwrap(),
            &Region::WEST_US_2
        );
    }

    #[test]
    fn preferred_regions_reorder_read_endpoints() {
        let preferred = vec![Region::WEST_US_2, Region::WEST_US_3];
        let state = build_account_endpoint_state(
            &multi_region_properties(),
            default_endpoint(),
            None,
            false,
            &preferred,
        );

        assert_eq!(state.preferred_read_endpoints.len(), 3);
        assert_eq!(
            state.preferred_read_endpoints[0].region().unwrap(),
            &Region::WEST_US_2
        );
        assert_eq!(
            state.preferred_read_endpoints[1].region().unwrap(),
            &Region::WEST_US_3
        );
        assert_eq!(
            state.preferred_read_endpoints[2].region().unwrap(),
            &Region::EAST_US
        );
    }

    #[test]
    fn preferred_regions_unknown_regions_are_skipped() {
        let preferred = vec![Region::new("nonexistent"), Region::WEST_US_3];
        let state = build_account_endpoint_state(
            &multi_region_properties(),
            default_endpoint(),
            None,
            false,
            &preferred,
        );

        // westus3 should be first; the nonexistent region is skipped.
        assert_eq!(
            state.preferred_write_endpoints[0].region().unwrap(),
            &Region::WEST_US_3
        );
        assert_eq!(
            state.preferred_write_endpoints[1].region().unwrap(),
            &Region::EAST_US
        );
        assert_eq!(
            state.preferred_write_endpoints[2].region().unwrap(),
            &Region::WEST_US_2
        );
    }

    #[test]
    fn empty_preferred_regions_preserves_original_order() {
        let state = build_account_endpoint_state(
            &multi_region_properties(),
            default_endpoint(),
            None,
            false,
            &[],
        );

        // Original account-metadata order: eastus, westus2, westus3.
        assert_eq!(
            state.preferred_write_endpoints[0].region().unwrap(),
            &Region::EAST_US
        );
        assert_eq!(
            state.preferred_write_endpoints[1].region().unwrap(),
            &Region::WEST_US_2
        );
        assert_eq!(
            state.preferred_write_endpoints[2].region().unwrap(),
            &Region::WEST_US_3
        );
    }
}
