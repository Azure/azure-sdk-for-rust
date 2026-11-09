// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Evaluates routing policy, collects candidates, and selects compatible endpoints.

use std::time::{Duration, Instant};

use crate::{
    clock::Clock,
    models::CosmosOperation,
    pipeline::operation::{
        recovery::state::OperationRetryState,
        routing::{
            self, partition_endpoint_state::HealthStatus, AccountEndpointState, CosmosEndpoint,
            InFlightFailures, LocationSnapshot, RoutingDecision, RoutingFallbackReason,
        },
    },
};

/// Operation constraints evaluated before collecting candidate endpoints.
pub(crate) struct RoutingPolicy {
    pub read_only: bool,
    pub requires_write_endpoint: bool,
    pub distributed_transaction: bool,
    pub prefer_write_endpoints_for_read: bool,
}

impl RoutingPolicy {
    /// Resolves operation intent without consulting mutable routing state.
    pub(crate) fn new(operation: &CosmosOperation) -> Self {
        Self {
            read_only: operation.is_read_only(),
            requires_write_endpoint: operation.operation_type().routes_to_write_endpoints(),
            distributed_transaction: routing::is_distributed_transaction_operation(operation),
            prefer_write_endpoints_for_read: operation.prefers_write_endpoints_for_read(),
        }
    }

    /// Applies deferred failures only to requests that require write-endpoint routing.
    fn in_flight_failures(&self) -> InFlightFailures {
        if self.requires_write_endpoint {
            InFlightFailures::Exclude
        } else {
            InFlightFailures::Ignore
        }
    }
}

/// Candidate ordering plus the eligibility rules for one selection pass.
pub(crate) struct CandidateEndpoints<'a> {
    /// Ordered account endpoints, rotated by the retry position during selection.
    pub endpoints: &'a [CosmosEndpoint],
    /// PPAF's write endpoint for this partition, tried before account-level writers
    /// when a PATCH result-check read prefers write endpoints. Normal filters still apply.
    prioritized_write_endpoint: Option<&'a CosmosEndpoint>,
    /// Filters failed PATCH-check URLs and controls use of unavailable endpoints.
    selection: Selection,
}

/// Candidate filters for ordinary requests and reads that check whether a PATCH succeeded.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Selection {
    /// Prefer healthy endpoints; retain the first unavailable endpoint as a last resort.
    Standard,
    /// Read from a healthy write region to check the PATCH result. Reject failed URLs,
    /// stale regions, and every active unavailability mark, including WriteForbidden.
    WriteRegionForPatchCheck,
    /// No usable write region remains. Try read regions without revisiting failed URLs;
    /// an unavailable reader is still usable as a last resort.
    ReadRegionForPatchCheck,
}

/// Records a selected regional endpoint without choosing its gateway version.
pub(crate) fn select(endpoint: &CosmosEndpoint) -> RoutingDecision {
    RoutingDecision {
        endpoint: endpoint.clone(),
        routing_fallback: None,
    }
}

/// Selects an attempt route without publishing operation-local failure evidence.
///
/// Policy, candidates, selection, and fallback precede partition overrides.
/// PPAF and PPCB retain their distinct exclusion and availability rules.
pub(crate) fn resolve(
    clock: &Clock,
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    // Choose the relevant region list before applying per-attempt failure filters.
    let policy = RoutingPolicy::new(operation);
    let candidates = routing::resolve_candidate_endpoints(location, retry_state, &policy);
    let selected = routing::try_select_endpoint(
        clock,
        operation,
        retry_state,
        &location.account,
        &candidates,
        policy.in_flight_failures(),
        endpoint_unavailability_ttl,
    );

    // An exhausted list does not end the operation: retry policy permits fallback attempts.
    let (selected, routing_fallback) = match selected {
        Some(endpoint) => (endpoint, None),
        None => routing::fallback(
            clock,
            operation,
            retry_state,
            &location.account,
            candidates,
            endpoint_unavailability_ttl,
        ),
    };
    debug_assert!(
        !selected.is_global(),
        "only account-topology fetches may use the global endpoint"
    );

    // A partition may have moved or tripped its circuit breaker independently of the account.
    let selected = if let Some(endpoint) = routing::resolve_partition_overrides(
        clock,
        operation,
        retry_state,
        location,
        policy.in_flight_failures(),
        endpoint_unavailability_ttl,
    ) {
        endpoint
    } else {
        selected
    };

    let mut decision = routing::select(&selected);
    decision.routing_fallback = routing_fallback;
    decision
}

/// Collects account candidates, prioritizing PPAF's partition writer for PATCH result-check reads.
pub(crate) fn resolve_candidate_endpoints<'a>(
    location: &'a LocationSnapshot,
    retry_state: &OperationRetryState,
    policy: &RoutingPolicy,
) -> CandidateEndpoints<'a> {
    // PATCH checks prefer writers; transactions require the account's write region.
    // Other requests follow read/write preferences and session-retry policy.
    let endpoints = if policy.prefer_write_endpoints_for_read {
        &location.account.preferred_write_endpoints[..]
    } else if policy.distributed_transaction {
        &location.account.account_write_endpoints
    } else if policy.read_only
        && (policy.requires_write_endpoint || retry_state.route_reads_to_write_endpoints())
    {
        &location.account.preferred_write_endpoints
    } else if !policy.read_only && retry_state.ppaf_write_retry_allowed {
        // Single-master PPAF discovers a new writer by probing every preferred region.
        &location.account.preferred_read_endpoints
    } else {
        location.account.preferred_endpoints(policy.read_only)
    };

    // PPAF may know a newer writer for this partition than account metadata does.
    // Resolve it against current topology before placing it ahead of account writers.
    let prioritized_write_endpoint = if policy.prefer_write_endpoints_for_read
        && !location.account.multiple_write_locations_enabled
        && location.partitions.per_partition_automatic_failover_enabled
    {
        retry_state
            .partition_key_range_id
            .as_ref()
            .and_then(|id| location.partitions.failover_overrides.get(id))
            .and_then(|entry| {
                routing::current_account_endpoint(&location.account, &entry.current_endpoint)
            })
    } else {
        None
    };

    // A PATCH check must reject unusable writers so it can explicitly fall back to readers.
    CandidateEndpoints {
        endpoints,
        prioritized_write_endpoint,
        selection: if policy.prefer_write_endpoints_for_read {
            Selection::WriteRegionForPatchCheck
        } else {
            Selection::Standard
        },
    }
}

/// Selects the first eligible candidate, rebasing stale retry indices to the current topology.
///
/// Ordinary passes retain the first unavailable endpoint as a last resort.
/// The write-region pass for PATCH result checks returns `None` when no healthy writer remains.
pub(crate) fn try_select_endpoint(
    clock: &Clock,
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    account: &AccountEndpointState,
    candidates: &CandidateEndpoints<'_>,
    in_flight: InFlightFailures,
    endpoint_unavailability_ttl: Duration,
) -> Option<CosmosEndpoint> {
    // Topology changes invalidate the old retry position; start with the new preference order.
    let base_index = if retry_state.location.is_current(account.generation) {
        retry_state.location.index()
    } else {
        0
    };
    let now = clock.now();
    let mut first_unavailable = None;
    let ordered = (0..candidates.endpoints.len())
        .map(|offset| &candidates.endpoints[(base_index + offset) % candidates.endpoints.len()]);

    // Try the partition's prioritized writer first, then rotate through account candidates.
    for candidate in candidates
        .prioritized_write_endpoint
        .into_iter()
        .chain(ordered)
    {
        // Caller exclusions remain binding; deferred failures are skipped unless fallback allows them.
        if retry_state.is_region_excluded(candidate.region(), in_flight) {
            continue;
        }

        // PATCH checks track failed URLs separately because those failures are not published globally.
        if candidates.selection != Selection::Standard
            && retry_state
                .patch_verification_failed_endpoint_urls
                .contains(candidate.url())
        {
            continue;
        }

        // A removed region cannot establish whether the current writer committed the PATCH.
        if candidates.selection == Selection::WriteRegionForPatchCheck
            && !account.contains_region(candidate)
        {
            continue;
        }

        // WriteForbidden still permits ordinary reads, but cannot confirm the PATCH at its writer.
        if account.is_region_available(
            candidate,
            now,
            endpoint_unavailability_ttl,
            operation.is_read_only() && candidates.selection != Selection::WriteRegionForPatchCheck,
        ) {
            return Some(candidate.clone());
        }

        // Ordinary requests and fallback reads may retry a marked endpoint when nothing healthier exists.
        if candidates.selection != Selection::WriteRegionForPatchCheck
            && first_unavailable.is_none()
        {
            first_unavailable = Some(candidate.clone());
        }
    }
    first_unavailable
}

/// Exhausts fallback passes without weakening caller exclusions until every region is excluded.
///
/// PATCH result checks first try readers without revisiting failed URLs.
/// If selection still fails, retry transiently failed regions, then the account's write region.
pub(crate) fn fallback<'a>(
    clock: &Clock,
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    account: &'a AccountEndpointState,
    mut candidates: CandidateEndpoints<'a>,
    endpoint_unavailability_ttl: Duration,
) -> (CosmosEndpoint, Option<RoutingFallbackReason>) {
    let verification_fallback = candidates.selection == Selection::WriteRegionForPatchCheck;
    let mut selected = None;

    // A PATCH result can still be checked in a read region when no usable writer remains.
    if verification_fallback {
        tracing::debug!(
            routing_fallback = "patch_verification_read_write_endpoint_unavailable_or_excluded",
            "PATCH verification read falling back to normal read routing"
        );
        candidates = CandidateEndpoints {
            endpoints: account.preferred_endpoints(operation.is_read_only()),
            prioritized_write_endpoint: None,
            selection: Selection::ReadRegionForPatchCheck,
        };
        selected = routing::try_select_endpoint(
            clock,
            operation,
            retry_state,
            account,
            &candidates,
            // A prior write failure must not rule out a region that can still serve this read.
            InFlightFailures::Ignore,
            endpoint_unavailability_ttl,
        );
    }

    // Retry previously failed regions; PATCH checks also stop filtering failed URLs here.
    let selected = selected.unwrap_or_else(|| {
        candidates.selection = Selection::Standard;
        routing::try_select_endpoint(
            clock,
            operation,
            retry_state,
            account,
            &candidates,
            // In-flight failures may have been transient. If no other regions are
            // available, retry them instead of exhausting the operation here.
            InFlightFailures::Ignore,
            endpoint_unavailability_ttl,
        )
        .unwrap_or_else(|| {
            // When all preferred regions are excluded, make a best-effort request to the account writer.
            if routing::is_distributed_transaction_operation(operation) {
                account.account_write_endpoints.first()
            } else {
                account.preferred_write_endpoints.first()
            }
            .expect("preferred write endpoints are always non-empty")
            .clone()
        })
    });

    // Report reader fallback only when the chosen endpoint is not also an account writer.
    let fallback = (verification_fallback
        && !account
            .preferred_write_endpoints
            .iter()
            .any(|writer| routing::endpoint_matches_region_or_url(writer, &selected)))
    .then_some(RoutingFallbackReason::PatchVerificationReadWriteEndpointUnavailableOrExcluded);
    (selected, fallback)
}

/// Tries the cached hub region, partition circuit breaker, and partition failover in that order.
pub(crate) fn resolve_partition_overrides(
    clock: &Clock,
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    in_flight: InFlightFailures,
    endpoint_unavailability_ttl: Duration,
) -> Option<CosmosEndpoint> {
    let pk_range_id = retry_state.partition_key_range_id.as_ref()?;
    let now = clock.now();

    // A prior session-unavailable response may require reading from this partition's hub region.
    let hub_latch_active = operation.is_read_only()
        && retry_state.hub_region_processing_only
        && location.partitions.per_partition_automatic_failover_enabled;
    if hub_latch_active {
        if let Some(entry) = location.partitions.failover_overrides.get(pk_range_id) {
            if !retry_state.is_region_excluded(entry.current_endpoint.region(), in_flight)
                && location.account.is_region_available(
                    &entry.current_endpoint,
                    now,
                    endpoint_unavailability_ttl,
                    operation.is_read_only(),
                )
            {
                return Some(entry.current_endpoint.clone());
            }
        }
    }

    // PPCB handles reads and multi-write requests; PPAF handles single-write requests.
    // Their eligibility rules are disjoint, so a failed PPCB attempt cannot activate PPAF.
    if let Some(endpoint) = routing::try_partition_circuit_breaker(
        now,
        operation,
        retry_state,
        location,
        in_flight,
        endpoint_unavailability_ttl,
    ) {
        return Some(endpoint);
    }
    if let Some(endpoint) =
        routing::try_partition_failover(operation, retry_state, location, in_flight)
    {
        return Some(endpoint);
    }
    None
}

/// Selects a usable PPCB probe or failover endpoint when the operation is eligible.
///
/// Hub-only reads and reads checking a PATCH result bypass PPCB. This selects
/// an endpoint without updating shared circuit-breaker state.
pub(crate) fn try_partition_circuit_breaker(
    now: Instant,
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    in_flight: InFlightFailures,
    endpoint_unavailability_ttl: Duration,
) -> Option<CosmosEndpoint> {
    // These reads need a specific writer or hub, not a circuit-breaker alternate.
    if (operation.is_read_only()
        && retry_state.hub_region_processing_only
        && location.partitions.per_partition_automatic_failover_enabled)
        || operation.prefers_write_endpoints_for_read()
        || !routing::is_eligible_for_ppcb(&location.partitions, &location.account, operation)
    {
        return None;
    }
    let entry = location
        .partitions
        .circuit_breaker_overrides
        .get(retry_state.partition_key_range_id.as_ref()?)?;

    // Stored overrides can outlive topology changes or a newer request failure.
    let is_usable = |endpoint: &CosmosEndpoint| {
        !retry_state.is_region_excluded(endpoint.region(), in_flight)
            && location.account.contains_region(endpoint)
            && location.account.is_region_available(
                endpoint,
                now,
                endpoint_unavailability_ttl,
                operation.is_read_only(),
            )
    };

    // After the cooldown, first test the original failed region for recovery.
    if entry.health_status == HealthStatus::ProbeCandidate
        && is_usable(&entry.first_failed_endpoint)
    {
        return Some(entry.first_failed_endpoint.clone());
    }

    // Otherwise use the alternate only after this request kind's failure threshold is reached.
    if routing::can_circuit_breaker_trigger_failover(
        entry,
        operation.is_read_only(),
        &location.partitions.config,
    ) && is_usable(&entry.current_endpoint)
    {
        return Some(entry.current_endpoint.clone());
    }
    None
}

/// Selects PPAF's current partition writer for an eligible single-write request.
///
/// Resolves the cached writer against current topology and skips regions this
/// operation already failed against, without updating shared failover state.
pub(crate) fn try_partition_failover(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    in_flight: InFlightFailures,
) -> Option<CosmosEndpoint> {
    // Reads and multi-write requests must not use a single-write partition override.
    if !routing::is_eligible_for_ppaf(
        &location.partitions,
        &location.account,
        operation.is_read_only(),
        operation
            .resource_type()
            .is_partitioned(operation.operation_type()),
    ) {
        return None;
    }
    let entry = location
        .partitions
        .failover_overrides
        .get(retry_state.partition_key_range_id.as_ref()?)?;

    // This may be the only region able to accept the write, so caller exclusions
    // and account-wide marks do not override it; failures from this operation still do.
    routing::current_account_endpoint(&location.account, &entry.current_endpoint)
        .filter(|endpoint| {
            in_flight == InFlightFailures::Ignore
                || !retry_state.has_pending_failure(endpoint.region())
        })
        .cloned()
}

/// Identifies transaction requests that require transaction-specific routing.
pub(crate) fn is_distributed_transaction_operation(operation: &CosmosOperation) -> bool {
    #[cfg(feature = "unstable_dtx")]
    {
        operation.resource_type() == crate::models::ResourceType::DistributedTransactionBatch
    }
    #[cfg(not(feature = "unstable_dtx"))]
    {
        let _ = operation;
        false
    }
}

/// Matches regional identity when known, otherwise the endpoint URL.
pub(crate) fn endpoint_matches_region_or_url(
    left: &CosmosEndpoint,
    right: &CosmosEndpoint,
) -> bool {
    match (left.region(), right.region()) {
        (Some(left), Some(right)) => left == right,
        _ => left.url() == right.url(),
    }
}

/// Resolves a stored endpoint against current account topology.
pub(crate) fn current_account_endpoint<'a>(
    account: &'a AccountEndpointState,
    endpoint: &CosmosEndpoint,
) -> Option<&'a CosmosEndpoint> {
    account
        .preferred_read_endpoints
        .iter()
        .chain(account.preferred_write_endpoints.iter())
        .find(|candidate| routing::endpoint_matches_region_or_url(candidate, endpoint))
}
