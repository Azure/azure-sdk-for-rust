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
use crate::options::{PartitionFailoverOptions, Region};

use super::{
    partition_endpoint_state::{HealthStatus, PartitionEndpointState, PartitionFailoverEntry},
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
    gateway_v2_enabled: bool,
    preferred_regions: &[Region],
) -> AccountEndpointState {
    let generation = previous_generation.map_or(0, |g| g.saturating_add(1));

    let mut preferred_read_endpoints = build_preferred_endpoints(
        &properties.readable_locations,
        &properties.thin_client_readable_locations,
        gateway_v2_enabled,
    );

    let mut preferred_write_endpoints = build_preferred_endpoints(
        &properties.writable_locations,
        &properties.thin_client_writable_locations,
        gateway_v2_enabled,
    );
    let mut account_write_endpoints = preferred_write_endpoints.clone();

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
    if account_write_endpoints.is_empty() {
        account_write_endpoints.push(default_endpoint.clone());
    }

    AccountEndpointState {
        generation,
        preferred_read_endpoints: preferred_read_endpoints.into(),
        preferred_write_endpoints: preferred_write_endpoints.into(),
        account_write_endpoints: account_write_endpoints.into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: properties.enable_multiple_write_locations,
        default_endpoint,
    }
}

fn build_preferred_endpoints(
    standard_locations: &[crate::driver::cache::AccountRegion],
    gateway_v2_locations: &[crate::driver::cache::AccountRegion],
    gateway_v2_enabled: bool,
) -> Vec<CosmosEndpoint> {
    let gateway_v2_urls = if gateway_v2_enabled {
        parse_gateway_v2_locations(gateway_v2_locations)
    } else {
        HashMap::new()
    };

    let mut endpoints = Vec::with_capacity(standard_locations.len());
    for region in standard_locations {
        let url = region.database_account_endpoint.url().clone();

        let endpoint = gateway_v2_urls
            .get(&region.name)
            .cloned()
            .map(|gateway_v2_url| {
                CosmosEndpoint::regional_with_gateway_v2(
                    region.name.clone(),
                    url.clone(),
                    gateway_v2_url,
                )
            })
            .unwrap_or_else(|| CosmosEndpoint::regional(region.name.clone(), url));

        endpoints.push(endpoint);
    }

    endpoints
}

fn parse_gateway_v2_locations(
    gateway_v2_locations: &[crate::driver::cache::AccountRegion],
) -> HashMap<crate::options::Region, url::Url> {
    let mut urls = HashMap::new();

    for region in gateway_v2_locations {
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
        account_write_endpoints: Arc::clone(&state.account_write_endpoints),
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
        account_write_endpoints: Arc::clone(&state.account_write_endpoints),
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
    config: &PartitionFailoverOptions,
) -> bool {
    if is_read_only {
        entry.read_failure_count >= config.read_failure_threshold() as i32
    } else {
        entry.write_failure_count >= config.write_failure_threshold() as i32
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
                    current_state.config.partition_unavailability_duration(),
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
                current_state.config.partition_unavailability_duration(),
                pk_range_id,
            );
            return new_state;
        }

        // Reset counters if the counter reset window has elapsed.
        if now.saturating_duration_since(entry.last_failure_time)
            > current_state.config.counter_reset_window()
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

/// Inserts or updates the per-partition failover entry after a successful
/// response confirmed the supplied endpoint as the hub.
///
/// Writes into [`PartitionEndpointState::failover_overrides`] — the same
/// per-partition map used by PPAF. On a single-master account the
/// partition's hub region is its write region, so the entry serves both
/// PPAF-write routing and hub-region read routing.
///
/// **Existing-entry semantics**: only `current_endpoint` is updated. The
/// `failed_endpoints` set, `first_failed_endpoint`, `last_failure_time`,
/// and `health_status` are preserved as-is so that:
///
/// 1. A subsequent `403/3` against the new hub can advance through
///    `try_move_next_endpoint` without re-trying endpoints that were
///    already known to be non-hub.
/// 2. Any PPAF state (from a prior write rotation) is not silently
///    wiped by a hub-region read confirmation.
pub(crate) fn cache_hub_region(
    current_state: &PartitionEndpointState,
    pk_range_id: &PartitionKeyRangeId,
    hub_endpoint: &CosmosEndpoint,
) -> PartitionEndpointState {
    if !current_state.per_partition_automatic_failover_enabled {
        return current_state.clone();
    }
    if hub_endpoint.region().is_none() {
        return current_state.clone();
    }

    let mut new_state = current_state.clone();
    let now = Instant::now();

    new_state
        .failover_overrides
        .entry(pk_range_id.clone())
        .and_modify(|entry| {
            // Only update `current_endpoint` — preserve all other fields so
            // PPAF rotation history and failed_endpoints tracking are
            // retained across read-side confirmations.
            entry.current_endpoint = hub_endpoint.clone();
        })
        .or_insert_with(|| PartitionFailoverEntry {
            current_endpoint: hub_endpoint.clone(),
            first_failed_endpoint: hub_endpoint.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: now,
            last_failure_time: now,
            health_status: HealthStatus::Unhealthy,
            // Hub-region entries do not participate in background failback,
            // so the jitter is unused (mirrors PPAF entries).
            failback_jitter: Duration::ZERO,
        });

    new_state
}

/// Advances the per-partition failover entry to the next preferred read
/// endpoint after a `403/3 (WriteForbidden)` response on a hub-region
/// discovery attempt.
///
/// Creates a new entry if none exists (cold cache), or rotates the
/// existing `current_endpoint` to the next un-tried preferred read
/// endpoint (warm cache that found a stale hub). If all preferred reads
/// have been exhausted, the entry is removed so the next attempt falls
/// back to the default selection logic in `resolve_endpoint`.
///
/// Pure function: returns a new `PartitionEndpointState`. The caller is
/// responsible for the CAS swap.
pub(crate) fn advance_hub_region_discovery(
    current_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    pk_range_id: &PartitionKeyRangeId,
    failed_endpoint: &CosmosEndpoint,
) -> PartitionEndpointState {
    if !current_state.per_partition_automatic_failover_enabled {
        return current_state.clone();
    }

    let mut new_state = current_state.clone();
    let now = Instant::now();
    let next_endpoints = &account_state.preferred_read_endpoints;

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
            failback_jitter: Duration::ZERO,
        });

    if try_move_next_endpoint(entry, next_endpoints, failed_endpoint) {
        entry.last_failure_time = now;
    } else {
        // All preferred-read endpoints exhausted — drop the entry so the next
        // attempt falls back to default selection, and the operation aborts via
        // the failover-budget guard once that budget is depleted.
        new_state.failover_overrides.remove(pk_range_id.as_str());
    }

    new_state
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

/// Records an alternate-region hedge win for the given `(partition, primary_region)`
/// pair and, when the consecutive-win count reaches
/// [`PartitionFailoverOptions::consecutive_hedge_win_threshold`], trips the
/// partition by installing an [`HealthStatus::Unhealthy`] entry in
/// [`PartitionEndpointState::circuit_breaker_overrides`].
///
/// - The counter is keyed by `(partition_key_range_id, primary_region)` so that
///   different primary regions accumulate independently.
/// - The trip mirrors the shape produced by [`mark_partition_unavailable`]
///   (same `PartitionFailoverEntry` construction, same jitter sampling), so
///   the existing PPCB failback loop ([`expire_partition_overrides`]) recovers
///   it without any additional plumbing.
/// - Once the trip is installed, the counter for that pair is cleared — the
///   trip itself is the persistent signal; future hedge wins start a fresh
///   count against whichever region happens to be primary next.
///
/// Pure function: caller is responsible for the CAS swap.
pub(crate) fn record_hedge_alternate_win(
    state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    partition_key_range_id: &PartitionKeyRangeId,
    primary_region: Option<&Region>,
) -> PartitionEndpointState {
    let mut new_state = state.clone();
    let key = (partition_key_range_id.clone(), primary_region.cloned());
    let count = new_state
        .consecutive_hedge_wins
        .entry(key.clone())
        .or_insert(0);
    *count = count.saturating_add(1);
    let triggered = *count >= state.config.consecutive_hedge_win_threshold();
    if !triggered {
        return new_state;
    }

    // Threshold reached. Decide whether to skip or proceed based on the
    // existing override entry's health status:
    //
    // * `Unhealthy` — an active PPCB trip is in place and routing already
    //   goes to the alternate region. Hedge feedback shouldn't clobber the
    //   richer state recorded by the real PPCB failure path; clear the
    //   counter and return.
    //
    // * `ProbeCandidate` — failback is in progress: routing is sending
    //   probes back to the original primary. Repeated hedge alternate-wins
    //   prove the primary is *still* unhealthy (the probe lost the race),
    //   so we must re-trip with a fresh `Unhealthy` entry. Mirrors
    //   `mark_partition_unavailable`'s behavior when failures continue
    //   against a probe candidate. Falling through replaces the stale
    //   entry via the `insert` at the end of this function.
    if let Some(existing) = new_state
        .circuit_breaker_overrides
        .get(partition_key_range_id.as_str())
    {
        if existing.health_status == HealthStatus::Unhealthy {
            new_state.consecutive_hedge_wins.remove(&key);
            return new_state;
        }
        // ProbeCandidate — fall through to install a fresh trip.
    }

    // Locate the endpoint matching the primary region. Without an endpoint
    // identity we cannot install a meaningful trip (the override would have
    // nothing to fail away from).
    let primary_endpoint = primary_region.and_then(|region| {
        account_state
            .preferred_read_endpoints
            .iter()
            .chain(account_state.preferred_write_endpoints.iter())
            .find(|e| e.region().is_some_and(|r| r == region))
            .cloned()
    });
    let Some(primary_endpoint) = primary_endpoint else {
        return new_state;
    };

    // Seed the failure counts to the configured thresholds so that
    // [`can_circuit_breaker_trigger_failover`] accepts this entry on the
    // very next routing decision. Without this seeding, an entry with
    // `health_status = Unhealthy` and counts at 0 would be installed in
    // `circuit_breaker_overrides` but never consulted by `resolve_endpoint`
    // (the gate requires `count >= threshold`), so the hedge-driven trip
    // would be observable in state but invisible to routing — defeating
    // the whole point of the hedge-win trip.
    //
    // Both read and write counts are seeded because the same entry serves
    // both code paths in [`can_circuit_breaker_trigger_failover`], and a
    // hedge-driven trip signals partition-level degradation that applies
    // to whichever operation type happens to arrive next.
    let now = Instant::now();
    let mut entry = PartitionFailoverEntry {
        current_endpoint: primary_endpoint.clone(),
        first_failed_endpoint: primary_endpoint.clone(),
        failed_endpoints: Default::default(),
        read_failure_count: state.config.read_failure_threshold() as i32,
        write_failure_count: state.config.write_failure_threshold() as i32,
        first_failure_time: now,
        last_failure_time: now,
        health_status: HealthStatus::Unhealthy,
        failback_jitter: ppcb_failback_jitter(
            state.config.partition_unavailability_duration(),
            partition_key_range_id,
        ),
    };

    // Pick an alternate region. If no alternate exists there is nothing to
    // route to, so leave routing untouched and let the counter continue to
    // accumulate; a future topology change may add one.
    let next_endpoints = &account_state.preferred_read_endpoints;
    if !try_move_next_endpoint(&mut entry, next_endpoints, &primary_endpoint) {
        return new_state;
    }

    new_state.consecutive_hedge_wins.remove(&key);
    new_state
        .circuit_breaker_overrides
        .insert(partition_key_range_id.clone(), entry);
    new_state
}

/// Resets the consecutive-hedge-win counter for the given `(partition,
/// primary_region)` pair.
///
/// A direct primary-region win
/// clears any accumulated count so transient cross-region latency spikes do
/// not stack into a trip over arbitrarily long timescales.
///
/// Intentionally does **not** touch [`PartitionEndpointState::circuit_breaker_overrides`]:
/// an existing trip recovers exclusively via the PPCB failback sweep, not via
/// primary wins (a stale `Unhealthy` entry would route to the alternate, so
/// the primary never gets a chance to "win" against an active trip anyway).
///
/// Pure function: caller is responsible for the CAS swap.
pub(crate) fn record_hedge_primary_win(
    state: &PartitionEndpointState,
    partition_key_range_id: &PartitionKeyRangeId,
    primary_region: Option<&Region>,
) -> PartitionEndpointState {
    let key = (partition_key_range_id.clone(), primary_region.cloned());
    if !state.consecutive_hedge_wins.contains_key(&key) {
        // Fast path: no counter for this pair, no clone needed.
        return state.clone();
    }
    let mut new_state = state.clone();
    new_state.consecutive_hedge_wins.remove(&key);
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
    fn build_state_adds_gateway_v2_endpoint_when_enabled() {
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

        assert!(state.preferred_read_endpoints[0].gateway_v2_url().is_some());
        assert!(state.preferred_write_endpoints[0]
            .gateway_v2_url()
            .is_none());
    }

    #[test]
    fn build_state_adds_gateway_v2_for_write_endpoints_when_present() {
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

        assert!(state.preferred_read_endpoints[0].gateway_v2_url().is_some());
        assert!(state.preferred_write_endpoints[0]
            .gateway_v2_url()
            .is_some());
    }

    /// Regression guard for the "service stops advertising Gateway 2.0" case.
    ///
    /// When the account previously returned `thinClient*Locations` but a
    /// subsequent metadata refresh omits them (e.g., the service rolled the
    /// account off Gateway 2.0, or thin-client routing was disabled
    /// region-side), the rebuilt endpoint state must drop the
    /// `gateway_v2_url` on every endpoint so that follow-up requests route
    /// through the standard compute-gateway URL.
    ///
    /// `uses_gateway_v2(prefer_gateway_v2=true)` checks
    /// `gateway_v2_url.is_some()`, so a `None` URL is sufficient to force
    /// the request pipeline back onto the standard gateway transport even
    /// while the operator-level `gateway_v2_enabled` toggle remains on.
    #[test]
    fn build_state_drops_gateway_v2_when_thin_client_locations_disappear() {
        // First refresh: account advertises both thin-client read and write
        // endpoints. With `gateway_v2_enabled=true`, every preferred endpoint
        // must carry a `gateway_v2_url`.
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
                .gateway_v2_url()
                .is_some(),
            "initial read endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            initial.preferred_write_endpoints[0]
                .gateway_v2_url()
                .is_some(),
            "initial write endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            initial.preferred_read_endpoints[0].uses_gateway_v2(true),
            "initial read endpoint must route through Gateway 2.0 when prefer_gateway_v2=true"
        );
        assert!(
            initial.preferred_write_endpoints[0].uses_gateway_v2(true),
            "initial write endpoint must route through Gateway 2.0 when prefer_gateway_v2=true"
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

        // `gateway_v2_enabled` is still `true` on the store — only the wire
        // payload has changed. The rebuilt endpoints must nevertheless have
        // no `gateway_v2_url`, forcing fallback to the compute gateway.
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
                .gateway_v2_url()
                .is_none(),
            "read endpoint must lose its Gateway 2.0 URL when the service stops advertising thinClientReadableLocations"
        );
        assert!(
            rebuilt.preferred_write_endpoints[0]
                .gateway_v2_url()
                .is_none(),
            "write endpoint must lose its Gateway 2.0 URL when the service stops advertising thinClientWritableLocations"
        );
        assert!(
            !rebuilt.preferred_read_endpoints[0].uses_gateway_v2(true),
            "read request must fall back to the compute gateway even when the operator toggle (prefer_gateway_v2) is still true"
        );
        assert!(
            !rebuilt.preferred_write_endpoints[0].uses_gateway_v2(true),
            "write request must fall back to the compute gateway even when the operator toggle (prefer_gateway_v2) is still true"
        );
    }

    /// Regression guard for the "service starts advertising Gateway 2.0" case.
    ///
    /// When the account previously returned only standard locations but a
    /// subsequent metadata refresh adds `thinClient*Locations` (e.g., the
    /// service rolled the account onto Gateway 2.0), the rebuilt endpoint
    /// state must adopt a `gateway_v2_url` on every endpoint so that follow-up
    /// requests route through Gateway 2.0 while the operator toggle is on.
    #[test]
    fn build_state_adopts_gateway_v2_when_thin_client_locations_appear() {
        // First refresh: account advertises only standard locations. With
        // `gateway_v2_enabled=true` but no thin-client endpoints, no preferred
        // endpoint may carry a `gateway_v2_url`.
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

        let initial =
            build_account_endpoint_state(&without_g2, default_endpoint(), None, true, &[]);
        assert!(
            initial.preferred_read_endpoints[0]
                .gateway_v2_url()
                .is_none(),
            "initial read endpoint must carry no Gateway 2.0 URL"
        );
        assert!(
            initial.preferred_write_endpoints[0]
                .gateway_v2_url()
                .is_none(),
            "initial write endpoint must carry no Gateway 2.0 URL"
        );
        assert!(
            !initial.preferred_read_endpoints[0].uses_gateway_v2(true),
            "initial read endpoint must route through the standard gateway"
        );
        assert!(
            !initial.preferred_write_endpoints[0].uses_gateway_v2(true),
            "initial write endpoint must route through the standard gateway"
        );

        // Second refresh: same standard endpoints, but the service has begun
        // returning thin-client locations. Mimics the database-account call
        // starting to advertise Gateway 2.0.
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

        let rebuilt = build_account_endpoint_state(
            &with_g2,
            default_endpoint(),
            Some(initial.generation),
            true,
            &[],
        );
        assert_eq!(rebuilt.generation, initial.generation + 1);
        assert!(
            rebuilt.preferred_read_endpoints[0]
                .gateway_v2_url()
                .is_some(),
            "read endpoint must gain a Gateway 2.0 URL once the service advertises thinClientReadableLocations"
        );
        assert!(
            rebuilt.preferred_write_endpoints[0]
                .gateway_v2_url()
                .is_some(),
            "write endpoint must gain a Gateway 2.0 URL once the service advertises thinClientWritableLocations"
        );
        assert!(
            rebuilt.preferred_read_endpoints[0].uses_gateway_v2(true),
            "read request must route through Gateway 2.0 once thin-client endpoints are advertised"
        );
        assert!(
            rebuilt.preferred_write_endpoints[0].uses_gateway_v2(true),
            "write request must route through Gateway 2.0 once thin-client endpoints are advertised"
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
            account_write_endpoints: vec![regional_endpoint("eastus")].into(),
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
            account_write_endpoints: vec![regional_endpoint("eastus"), regional_endpoint("westus")]
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
        let config = PartitionFailoverOptions::default();
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
        let config = PartitionFailoverOptions::default();
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
        let config = PartitionFailoverOptions::default();
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
        let max = ps.config.partition_unavailability_duration() / 2;
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
        let threshold = PartitionFailoverOptions::default().read_failure_threshold() as i32;

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

    // ── Hub region cache tests ────────────────────────────────────────

    /// A partition state with PPAF enabled — the new gate that
    /// `cache_hub_region` checks before populating the cache.
    fn partition_state_ppaf_enabled() -> PartitionEndpointState {
        let mut ps = PartitionEndpointState::default();
        ps.per_partition_automatic_failover_enabled = true;
        ps
    }

    /// A partition state with PPAF disabled — `cache_hub_region` is a
    /// no-op against this state.
    fn partition_state_ppaf_disabled() -> PartitionEndpointState {
        PartitionEndpointState::default()
    }

    /// Regression test for the SetCurrent-only update invariant: when
    /// `cache_hub_region` updates an existing entry (e.g., one populated
    /// by a prior PPAF rotation or a prior 403/3 discovery), it must
    /// preserve the `failed_endpoints` set so that a subsequent 403/3
    /// rotation does not re-try already-tried endpoints.
    #[test]
    fn cache_hub_region_updates_existing_entry_preserving_failed_set() {
        let mut ps = partition_state_ppaf_enabled();
        let old_hub = regional_endpoint("westus");
        let new_hub = regional_endpoint("eastus");

        let mut failed = std::collections::HashSet::new();
        failed.insert(regional_endpoint("centralus"));
        let original_failed = failed.clone();
        let original_first_failed = old_hub.clone();
        ps.failover_overrides.insert(
            pk("0"),
            PartitionFailoverEntry {
                current_endpoint: old_hub.clone(),
                first_failed_endpoint: original_first_failed.clone(),
                failed_endpoints: failed,
                read_failure_count: 7,
                write_failure_count: 3,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let result = cache_hub_region(&ps, &pk("0"), &new_hub);
        let entry = result.failover_overrides.get(&pk("0")).unwrap();
        // current_endpoint MUST flip to the new hub.
        assert_eq!(entry.current_endpoint, new_hub);
        // Every other field MUST be preserved (SetCurrent-only semantics).
        assert_eq!(
            entry.failed_endpoints, original_failed,
            "failed_endpoints must be preserved so 403/3 rotation does not re-try already-tried endpoints",
        );
        assert_eq!(
            entry.first_failed_endpoint, original_first_failed,
            "first_failed_endpoint must be preserved so PPAF probe-back logic is not disrupted",
        );
        assert_eq!(
            entry.read_failure_count, 7,
            "read_failure_count must be preserved (no failure happened — this is a 2xx hub confirmation)",
        );
        assert_eq!(
            entry.write_failure_count, 3,
            "write_failure_count must be preserved (no failure happened — this is a 2xx hub confirmation)",
        );
    }

    /// Hub-region caching is a PPAF-only feature. On accounts without
    /// PPAF, `cache_hub_region` is a no-op so the unified cache does
    /// not accumulate state.
    #[test]
    fn cache_hub_region_skips_when_ppaf_disabled() {
        let ps = partition_state_ppaf_disabled();
        let result = cache_hub_region(&ps, &pk("0"), &regional_endpoint("eastus"));
        assert!(
            result.failover_overrides.is_empty(),
            "hub-region cache must not populate when PPAF is disabled on the account",
        );
    }

    // ── Hedge feedback (PPCB §9.5) ────────────────────────────────────

    /// Convenience: produce a partition state with the hedge-win threshold
    /// reduced for tests. Defaults are valid but 5 alternate-wins per test
    /// is noisy; 3 is enough to cover the "below threshold / at threshold"
    /// transition.
    fn partition_state_with_hedge_threshold(threshold: u32) -> PartitionEndpointState {
        let config = PartitionFailoverOptions::builder()
            .with_consecutive_hedge_win_threshold(threshold)
            .build()
            .expect("valid partition failover options");
        let mut state = PartitionEndpointState::new(config);
        state.per_partition_automatic_failover_enabled = true;
        state.per_partition_circuit_breaker_enabled = true;
        state
    }

    #[test]
    fn hedge_alternate_win_increments_counter_below_threshold() {
        let state = partition_state_with_hedge_threshold(3);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        let after = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));

        assert_eq!(
            after.consecutive_hedge_wins[&(pk_range.clone(), Some(primary))],
            1,
            "first hedge win increments to 1",
        );
        assert!(
            after.circuit_breaker_overrides.is_empty(),
            "no trip until threshold reached",
        );
    }

    #[test]
    fn cache_hub_region_skips_non_regional_endpoint() {
        let ps = partition_state_ppaf_enabled();
        let non_regional = default_endpoint();
        assert!(non_regional.region().is_none());

        let result = cache_hub_region(&ps, &pk("0"), &non_regional);
        assert!(result.failover_overrides.is_empty());
    }

    #[test]
    fn advance_hub_region_discovery_rotates_existing_entry() {
        let mut ps = partition_state_ppaf_enabled();
        let account = single_master_account();
        let eastus = regional_endpoint("eastus");
        let westus = regional_endpoint("westus");
        ps.failover_overrides.insert(
            pk("0"),
            PartitionFailoverEntry {
                current_endpoint: eastus.clone(),
                first_failed_endpoint: eastus.clone(),
                failed_endpoints: Default::default(),
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let result = advance_hub_region_discovery(&ps, &account, &pk("0"), &eastus);
        let entry = result.failover_overrides.get(&pk("0")).unwrap();
        assert_eq!(entry.current_endpoint, westus);
    }

    #[test]
    fn advance_hub_region_discovery_removes_entry_when_exhausted() {
        let mut ps = partition_state_ppaf_enabled();
        let account = single_master_account();
        let eastus = regional_endpoint("eastus");
        let westus = regional_endpoint("westus");

        // Pre-populate so the only un-tried endpoint is westus, currently
        // pointing at westus — and the failed-endpoint set already covers eastus.
        let mut failed = std::collections::HashSet::new();
        failed.insert(eastus.clone());
        ps.failover_overrides.insert(
            pk("0"),
            PartitionFailoverEntry {
                current_endpoint: westus.clone(),
                first_failed_endpoint: eastus.clone(),
                failed_endpoints: failed,
                read_failure_count: 0,
                write_failure_count: 0,
                first_failure_time: Instant::now(),
                last_failure_time: Instant::now(),
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::ZERO,
            },
        );

        let result = advance_hub_region_discovery(&ps, &account, &pk("0"), &westus);
        // Both endpoints have failed — entry is removed so default selection
        // applies on the next attempt.
        assert!(!result.failover_overrides.contains_key(&pk("0")));
    }

    /// Hub-region caching is a PPAF-only feature. On accounts without PPAF,
    /// `advance_hub_region_discovery` is a no-op so the unified cache does
    /// not accumulate state. Mirrors `cache_hub_region_skips_when_ppaf_disabled`
    /// for the rotation path — both writers into `failover_overrides` must
    /// share the same gating.
    #[test]
    fn advance_hub_region_discovery_skips_when_ppaf_disabled() {
        let ps = partition_state_ppaf_disabled();
        let account = single_master_account();
        let eastus = regional_endpoint("eastus");

        let result = advance_hub_region_discovery(&ps, &account, &pk("0"), &eastus);
        assert!(
            result.failover_overrides.is_empty(),
            "advance_hub_region_discovery must not populate the cache when PPAF is disabled on the account",
        );
    }

    #[test]
    fn hedge_alternate_win_at_threshold_installs_unhealthy_entry() {
        let mut state = partition_state_with_hedge_threshold(3);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        for _ in 0..3 {
            state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));
        }

        let entry = state
            .circuit_breaker_overrides
            .get(pk_range.as_str())
            .expect("threshold-th win must install a circuit breaker entry");
        assert_eq!(entry.health_status, HealthStatus::Unhealthy);
        assert_eq!(
            entry.first_failed_endpoint,
            regional_endpoint("eastus"),
            "first_failed_endpoint must match the primary region that lost",
        );
        assert_eq!(
            entry.current_endpoint,
            regional_endpoint("westus"),
            "current_endpoint must point at the alternate region after the trip",
        );
        assert!(
            !state
                .consecutive_hedge_wins
                .contains_key(&(pk_range, Some(primary))),
            "counter must be cleared after the trip is installed",
        );
    }

    #[test]
    fn hedge_alternate_win_seeds_failure_counts_to_trigger_failover() {
        // Spec §9.5: the hedge-driven trip must redirect routing on the
        // very next request — not just record the entry in state.
        // `can_circuit_breaker_trigger_failover` gates on
        // `count >= threshold`, so the entry's failure counts must be
        // seeded to the configured thresholds at install time. Without
        // this seeding the trip would be observable in state but
        // invisible to the endpoint resolver.
        let mut state = partition_state_with_hedge_threshold(3);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        for _ in 0..3 {
            state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));
        }

        let entry = state
            .circuit_breaker_overrides
            .get(pk_range.as_str())
            .expect("threshold-th win must install a circuit breaker entry");
        assert_eq!(
            entry.read_failure_count,
            state.config.read_failure_threshold() as i32,
            "read_failure_count must equal the configured threshold so a \
             read attempt's `can_circuit_breaker_trigger_failover` returns true",
        );
        assert_eq!(
            entry.write_failure_count,
            state.config.write_failure_threshold() as i32,
            "write_failure_count must equal the configured threshold so a \
             write attempt's `can_circuit_breaker_trigger_failover` returns true",
        );
        assert!(
            can_circuit_breaker_trigger_failover(
                entry,
                /* is_read_only = */ true,
                &state.config
            ),
            "the freshly-installed hedge-trip entry must immediately pass \
             the failover gate for read operations",
        );
        assert!(
            can_circuit_breaker_trigger_failover(
                entry,
                /* is_read_only = */ false,
                &state.config
            ),
            "the freshly-installed hedge-trip entry must immediately pass \
             the failover gate for write operations",
        );
    }

    #[test]
    fn hedge_primary_win_resets_counter_only() {
        let state = partition_state_with_hedge_threshold(3);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        // Accumulate 2 wins (one below threshold).
        let s1 = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));
        let s2 = record_hedge_alternate_win(&s1, &account, &pk_range, Some(&primary));
        assert_eq!(
            s2.consecutive_hedge_wins[&(pk_range.clone(), Some(primary.clone()))],
            2
        );

        // Primary win clears the counter.
        let s3 = record_hedge_primary_win(&s2, &pk_range, Some(&primary));
        assert!(s3.consecutive_hedge_wins.is_empty());
        assert!(s3.circuit_breaker_overrides.is_empty());

        // A subsequent alternate win starts the count from 1 again — the
        // primary win must have fully reset, not just decremented.
        let s4 = record_hedge_alternate_win(&s3, &account, &pk_range, Some(&primary));
        assert_eq!(s4.consecutive_hedge_wins[&(pk_range, Some(primary))], 1);
        assert!(s4.circuit_breaker_overrides.is_empty());
    }

    #[test]
    fn hedge_primary_win_does_not_touch_existing_trip() {
        // Existing trip + primary win should NOT remove the trip (only the
        // failback loop owns recovery).
        let mut state = partition_state_with_hedge_threshold(1);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));
        assert!(state
            .circuit_breaker_overrides
            .contains_key(pk_range.as_str()));

        let after = record_hedge_primary_win(&state, &pk_range, Some(&primary));
        assert!(
            after
                .circuit_breaker_overrides
                .contains_key(pk_range.as_str()),
            "primary wins must never clear an active PPCB trip; only the failback sweep does",
        );
    }

    #[test]
    fn hedge_counter_keyed_per_partition() {
        let mut state = partition_state_with_hedge_threshold(2);
        let account = multi_master_account();
        let pk_a = pk("pk-A");
        let pk_b = pk("pk-B");
        let primary = Region::new("eastus".to_string());

        // 1 win on pk_a, 1 win on pk_b — neither should trip even though the
        // *total* equals the threshold.
        state = record_hedge_alternate_win(&state, &account, &pk_a, Some(&primary));
        state = record_hedge_alternate_win(&state, &account, &pk_b, Some(&primary));

        assert!(state.circuit_breaker_overrides.is_empty());
        assert_eq!(state.consecutive_hedge_wins.len(), 2);
    }

    #[test]
    fn hedge_counter_keyed_per_primary_region() {
        let mut state = partition_state_with_hedge_threshold(2);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let east = Region::new("eastus".to_string());
        let west = Region::new("westus".to_string());

        // 1 win against eastus-primary, 1 against westus-primary — different
        // keys, neither trips.
        state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&east));
        state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&west));

        assert!(state.circuit_breaker_overrides.is_empty());
        assert_eq!(state.consecutive_hedge_wins.len(), 2);
    }

    #[test]
    fn hedge_alternate_win_skips_trip_when_existing_ppcb_entry_present() {
        // A hard PPCB failure beat hedge feedback to the partition. The hedge
        // win at threshold must NOT clobber the existing entry — but it must
        // still clear its own counter so it doesn't try again on the next win.
        let mut state = partition_state_with_hedge_threshold(1);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        let existing = PartitionFailoverEntry {
            current_endpoint: regional_endpoint("westus"),
            first_failed_endpoint: regional_endpoint("eastus"),
            failed_endpoints: HashSet::new(),
            read_failure_count: 42, // distinctive value to detect clobbering
            write_failure_count: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        };
        state
            .circuit_breaker_overrides
            .insert(pk_range.clone(), existing);

        let after = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));

        let entry = &after.circuit_breaker_overrides[pk_range.as_str()];
        assert_eq!(
            entry.read_failure_count, 42,
            "existing PPCB entry must be preserved verbatim",
        );
        assert!(
            after.consecutive_hedge_wins.is_empty(),
            "counter must be cleared even when the trip is skipped, so we don't loop on every subsequent hedge win",
        );
    }

    #[test]
    fn hedge_alternate_win_re_trips_when_existing_entry_is_probe_candidate() {
        // Regression for PPCB-vs-hedging interaction:
        //
        // After the first hedge-driven trip, the failback sweep transitions
        // the entry from Unhealthy to ProbeCandidate. While the primary is
        // STILL unhealthy, routing sends probes back to it, the probes lose
        // the hedge race to the alternate, and `record_hedge_alternate_win`
        // is invoked again. We must re-trip with a fresh `Unhealthy` entry
        // — not silently clear the counter and leave the stale ProbeCandidate
        // entry in place — otherwise the partition never fails back over
        // and routing keeps probing a degraded primary indefinitely.
        let mut state = partition_state_with_hedge_threshold(1);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let primary = Region::new("eastus".to_string());

        let stale_probe_candidate = PartitionFailoverEntry {
            current_endpoint: regional_endpoint("westus"),
            first_failed_endpoint: regional_endpoint("eastus"),
            failed_endpoints: HashSet::new(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::ProbeCandidate,
            failback_jitter: Duration::ZERO,
        };
        state
            .circuit_breaker_overrides
            .insert(pk_range.clone(), stale_probe_candidate);

        let after = record_hedge_alternate_win(&state, &account, &pk_range, Some(&primary));

        let entry = &after.circuit_breaker_overrides[pk_range.as_str()];
        assert_eq!(
            entry.health_status,
            HealthStatus::Unhealthy,
            "stale ProbeCandidate must be replaced with a fresh Unhealthy entry"
        );
        assert_eq!(
            entry.read_failure_count,
            state.config.read_failure_threshold() as i32,
            "re-trip must seed read_failure_count to the threshold so \
             can_circuit_breaker_trigger_failover accepts the entry on the \
             very next routing decision"
        );
        assert_eq!(
            entry.first_failed_endpoint,
            regional_endpoint("eastus"),
            "re-trip must record the still-unhealthy primary as the first \
             failed endpoint"
        );
        assert_eq!(
            entry.current_endpoint,
            regional_endpoint("westus"),
            "re-trip must point routing at the alternate region"
        );
        assert!(
            after.consecutive_hedge_wins.is_empty(),
            "counter must be cleared after the re-trip is installed"
        );
    }

    #[test]
    fn hedge_alternate_win_no_op_when_primary_region_unknown_to_account() {
        // Pathological case: we record per (partition,
        // primary_region) — but if the primary_region we were told about
        // doesn't match any endpoint in the account snapshot, we cannot
        // install a meaningful trip. Counter still increments (it's keyed
        // on the value, not the endpoint identity), but no trip happens.
        let mut state = partition_state_with_hedge_threshold(1);
        let account = multi_master_account();
        let pk_range = pk("pk-1");
        let unknown_region = Region::new("mars-central-1".to_string());

        state = record_hedge_alternate_win(&state, &account, &pk_range, Some(&unknown_region));

        assert_eq!(
            state.consecutive_hedge_wins[&(pk_range, Some(unknown_region))],
            1,
            "counter still tracks the (partition, primary_region) pair even if we can't trip",
        );
        assert!(
            state.circuit_breaker_overrides.is_empty(),
            "no trip without an endpoint identity to route away from",
        );
    }

    #[test]
    fn hedge_alternate_win_with_none_primary_region_does_not_trip() {
        // Default-endpoint accounts pass primary_region=None. The counter
        // still tracks the pair (None branch), but the lookup for an
        // endpoint matching `None` always fails so no trip is installed.
        let mut state = partition_state_with_hedge_threshold(1);
        let account = multi_master_account();
        let pk_range = pk("pk-1");

        state = record_hedge_alternate_win(&state, &account, &pk_range, None);

        assert_eq!(state.consecutive_hedge_wins[&(pk_range, None)], 1);
        assert!(state.circuit_breaker_overrides.is_empty());
    }
}
