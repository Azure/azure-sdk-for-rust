// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure, side-effect-free helpers that govern cross-region hedging.
//!
//! These helpers are consumed by `evaluate_transport_result` when it
//! produces `OperationAction::Hedge { secondary_routing }` and by the
//! `execute_hedged()` race loop.

use std::time::Duration;

use crate::{
    driver::{
        pipeline::{
            components::{RoutingDecision, TransportMode},
            hedging_diagnostics::HedgingStrategyConfig,
        },
        routing::AccountEndpointState,
        transport::EndpointKey,
    },
    models::{CosmosOperation, OperationType, ResourceType},
    options::{
        AvailabilityStrategy, HedgeThreshold, HedgingStrategy, OperationOptionsView, Region,
    },
};

/// Default hedge threshold cap used by [`resolve_availability_strategy`]
/// when no operation- / account- / env-level strategy is configured.
///
/// The driver-default threshold is `min(1000ms, request_timeout / 2)`;
/// this constant is the upper bound.
const DEFAULT_THRESHOLD_CAP: Duration = Duration::from_millis(1000);

/// Resource types eligible for cross-region hedging in the current phase.
///
/// Subsequent phases widen this single constant — no other change to
/// [`should_hedge`] is required.
const HEDGEABLE_RESOURCE_TYPES: &[ResourceType] = &[ResourceType::Document];

/// Operation types eligible for cross-region hedging in the current phase.
///
/// Future phases will append feed-style operations
/// (`Query` / `ReadFeed` / `QueryPlan`) and metadata reads.
const HEDGEABLE_OPERATION_TYPES: &[OperationType] = &[OperationType::Read];

/// Returns `true` when the operation is eligible for cross-region hedging.
///
/// `strategy` is the resolved strategy from
/// [`resolve_availability_strategy`]. `None` represents an explicit
/// `AvailabilityStrategy::Disabled` at any layer and short-circuits to
/// `false`.
///
/// `excluded_regions` is the post-resolution `ExcludeRegions` set from the
/// operation's options view; the applicable preferred-region count is
/// computed against the post-filter list.
pub(crate) fn should_hedge(
    strategy: Option<&HedgingStrategy>,
    operation: &CosmosOperation,
    account_state: &AccountEndpointState,
    excluded_regions: &[Region],
) -> bool {
    if strategy.is_none() {
        return false;
    }

    if account_state.preferred_read_endpoints.is_empty() {
        return false;
    }

    if !HEDGEABLE_RESOURCE_TYPES.contains(&operation.resource_type()) {
        return false;
    }

    // Writes are never hedged. The phase also restricts OperationType to
    // `Read`, which is a superset of "not a write", but the explicit
    // `is_read_only()` guard documents the intent and protects against
    // future phase widenings that add non-read OperationTypes (e.g. feed
    // reads) without revisiting this predicate.
    let op = operation.operation_type();
    if !op.is_read_only() {
        return false;
    }
    if !HEDGEABLE_OPERATION_TYPES.contains(&op) {
        return false;
    }

    let applicable = account_state
        .preferred_read_endpoints
        .iter()
        .filter(|ep| match ep.region() {
            // Region-less endpoints (e.g. the global account endpoint)
            // are skipped because the secondary picker in
            // `evaluate_hedge_eligibility` requires `region().is_some()`.
            Some(r) => !excluded_regions.contains(r),
            None => false,
        })
        .count();

    applicable >= 2
}

/// Resolves the effective [`HedgingStrategy`] for a single operation.
///
/// Priority order (highest first):
///
/// 0. Environment-driven master switch — `hedging_enabled` (typically set via
///    `AZURE_COSMOS_HEDGING_ENABLED`). When set, it is the **source of truth**
///    and takes precedence over the programmatic `availability_strategy` in
///    both directions:
///    - `Some(false)` → hedging is disabled (returns `None`) even when an
///      explicit `AvailabilityStrategy::Hedging(..)` is configured.
///    - `Some(true)` → hedging is enabled even when an explicit
///      `AvailabilityStrategy::Disabled` is configured. A programmatic
///      `AvailabilityStrategy::Hedging(..)` still supplies its custom
///      threshold; otherwise the driver default threshold applies.
/// 1. Programmatic operation / account / runtime `availability_strategy`
///    (resolved by [`OperationOptionsView`] before we are called).
/// 2. Driver default — `min(1000ms, request_timeout / 2)`.
///
/// Returns `None` when hedging is disabled — either through the
/// `hedging_enabled` switch resolving to `Some(false)` (layer 0) or an explicit
/// `AvailabilityStrategy::Disabled` with no overriding env switch (layer 1);
/// otherwise returns `Some(_)`. The "single-region account / insufficient
/// regions" case is enforced separately in [`should_hedge`].
pub(crate) fn resolve_availability_strategy(
    view: &OperationOptionsView<'_>,
    request_timeout: Option<Duration>,
) -> Option<HedgingStrategy> {
    // Priority 0 — environment-driven master switch (source of truth). When
    // set, the env switch wins over the programmatic `availability_strategy`
    // in both directions, keeping the two enablement hooks in parity.
    match view.hedging_enabled() {
        // Explicitly disabled — no hedging even if the caller passed an
        // explicit `AvailabilityStrategy::Hedging(..)` at any layer.
        Some(&false) => return None,
        // Explicitly enabled — hedge even if the caller passed an explicit
        // `AvailabilityStrategy::Disabled`. Honor a programmatic
        // `Hedging(..)` strategy's custom threshold when one is present,
        // otherwise fall back to the driver default threshold.
        Some(&true) => {
            return match view.availability_strategy() {
                Some(AvailabilityStrategy::Hedging(s)) => Some(*s),
                _ => Some(HedgingStrategy::new(default_threshold(request_timeout))),
            };
        }
        // Unset — defer to the programmatic strategy below.
        None => {}
    }

    // Priority 1 — code-level strategy.
    match view.availability_strategy() {
        Some(AvailabilityStrategy::Disabled) => return None,
        Some(AvailabilityStrategy::Hedging(s)) => return Some(*s),
        None => {}
    }

    // Priority 2 — driver default.
    Some(HedgingStrategy::new(default_threshold(request_timeout)))
}

/// Computes the driver-default threshold: `min(1000ms, request_timeout / 2)`,
/// falling back to `request_timeout` itself when `t/2 == 0` (sub-ms input),
/// then to `1000ms` only when no timeout is configured.
fn default_threshold(request_timeout: Option<Duration>) -> HedgeThreshold {
    let candidate = match request_timeout {
        Some(t) if !t.is_zero() => (t / 2).min(DEFAULT_THRESHOLD_CAP),
        _ => DEFAULT_THRESHOLD_CAP,
    };

    HedgeThreshold::new(candidate)
        .or_else(|| request_timeout.and_then(HedgeThreshold::new))
        .unwrap_or_else(|| {
            HedgeThreshold::new(DEFAULT_THRESHOLD_CAP)
                .expect("DEFAULT_THRESHOLD_CAP is statically non-zero")
        })
}

/// Outcome of [`evaluate_hedge_eligibility`] — everything the pipeline
/// needs to dispatch [`OperationAction::Hedge`] for a single attempt.
///
/// [`OperationAction::Hedge`]:
/// crate::driver::pipeline::components::OperationAction::Hedge
#[derive(Debug)]
pub(crate) struct HedgeUpgrade {
    /// Routing decision for the alternate-region hedge.
    pub(crate) secondary_routing: RoutingDecision,
    /// The resolved hedge threshold (used to schedule the timer).
    pub(crate) threshold: HedgeThreshold,
    /// Snapshot of the strategy config for `HedgeDiagnostics`.
    pub(crate) strategy_config: HedgingStrategyConfig,
}

/// Evaluates whether the per-attempt transient outcome should be upgraded
/// to a cross-region hedge, returning the materialized [`HedgeUpgrade`]
/// when all eligibility gates hold.
///
/// `primary` is the routing decision for the just-completed attempt; it is
/// used both to honor the same gateway-version preference when constructing
/// the secondary [`RoutingDecision`] and to ensure the secondary targets a
/// **different region** than the primary (the primary is not necessarily
/// `preferred_read_endpoints[0]` once PPCB overrides, probe-candidate
/// routing, location-index advancement, or prior retry state have moved
/// it). `request_timeout` is plumbed through to
/// [`resolve_availability_strategy`] so the driver default
/// (`min(1000ms, request_timeout / 2)`) can be computed.
///
/// Returns `None` when hedging is disabled, the operation is ineligible,
/// or no applicable region distinct from `primary` exists — in all cases
/// the caller falls back to its non-hedged decision (typically
/// `FailoverRetry`).
pub(crate) fn evaluate_hedge_eligibility(
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    account_state: &AccountEndpointState,
    primary: &RoutingDecision,
    request_timeout: Option<Duration>,
) -> Option<HedgeUpgrade> {
    let strategy = resolve_availability_strategy(options, request_timeout)?;

    let user_excluded: Vec<Region> = options
        .excluded_regions()
        .map(|r| r.0.clone())
        .unwrap_or_default();

    if !should_hedge(Some(&strategy), operation, account_state, &user_excluded) {
        return None;
    }

    // Build the applicable list (preferred reads minus user exclusions).
    // Order matches `preferred_read_endpoints`.
    let applicable_regions: Vec<Region> = account_state
        .preferred_read_endpoints
        .iter()
        .filter_map(|ep| ep.region().cloned())
        .filter(|r| !user_excluded.contains(r))
        .collect();

    // Defensive re-check: `should_hedge` already enforced `applicable >= 2`,
    // but the test seam may bypass `should_hedge`.
    if applicable_regions.len() < 2 {
        return None;
    }

    // Pick the first applicable endpoint distinct from `primary` on both
    // region AND endpoint_key. The primary is NOT necessarily
    // `preferred_read_endpoints[0]` (PPCB overrides / probe routing /
    // prior retry state can promote it), so a same-region or same-key
    // alias would double RU with zero availability benefit.
    let primary_region = primary.endpoint.region().cloned();
    let primary_key = primary.endpoint.endpoint_key();
    let secondary_ep = account_state
        .preferred_read_endpoints
        .iter()
        .filter(|ep| ep.region().is_some_and(|r| !user_excluded.contains(r)))
        .find(|ep| ep.region() != primary_region.as_ref() && ep.endpoint_key() != primary_key)?
        .clone();

    // Match the primary's gateway-version preference so a GatewayV2-capable
    // account uses GatewayV2 for both legs (and downgrades cleanly for legacy).
    let prefer_gateway_v2 = matches!(primary.transport_mode, TransportMode::GatewayV2);
    let use_gateway_v2 = secondary_ep.uses_gateway_v2(prefer_gateway_v2);
    let transport_mode = if use_gateway_v2 {
        TransportMode::GatewayV2
    } else {
        TransportMode::Gateway
    };
    let selected_url = secondary_ep.selected_url(use_gateway_v2).clone();
    // Key the connection pool by the URL we will actually dial. For a
    // GatewayV2 leg that is the thin-client proxy authority (host:proxy-port),
    // which differs from the logical G1 endpoint authority — mirror
    // `resolve_endpoint` so the hedge leg shares the main path's G2 pool
    // instead of opening a duplicate one keyed by the G1 host.
    let secondary_endpoint_key = if use_gateway_v2 {
        EndpointKey::try_from(&selected_url).expect("selected URL must have a valid host and port")
    } else {
        secondary_ep.endpoint_key()
    };
    let secondary_routing = RoutingDecision {
        selected_url,
        transport_mode,
        endpoint_key: secondary_endpoint_key,
        endpoint: secondary_ep,
    };

    Some(HedgeUpgrade {
        secondary_routing,
        threshold: strategy.threshold(),
        strategy_config: HedgingStrategyConfig::new(strategy.threshold()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use azure_core::http::StatusCode;
    use url::Url;

    use crate::{
        driver::routing::{AccountEndpointState, CosmosEndpoint},
        models::{
            AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
            CosmosStatus, DatabaseReference, ItemReference, PartitionKey, PartitionKeyDefinition,
            SystemProperties,
        },
        options::{
            AvailabilityStrategy, ExcludedRegions, HedgeThreshold, HedgingStrategy,
            OperationOptions, OperationOptionsBuilder, OperationOptionsView, Region,
        },
    };

    // ───────────────────────── Test fixtures ─────────────────────────

    fn endpoint_for(region: Region) -> CosmosEndpoint {
        let url = Url::parse(&format!(
            "https://acct-{}.documents.azure.com/",
            region.as_str()
        ))
        .expect("valid url");
        CosmosEndpoint::regional(region, url)
    }

    fn account_state_with_regions(regions: &[Region]) -> AccountEndpointState {
        let endpoints: Vec<CosmosEndpoint> = regions.iter().cloned().map(endpoint_for).collect();
        let default = endpoints.first().cloned().unwrap_or_else(|| {
            CosmosEndpoint::global(Url::parse("https://acct.example/").unwrap())
        });
        AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: endpoints.clone().into(),
            preferred_write_endpoints: endpoints.clone().into(),
            account_write_endpoints: endpoints.into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default,
        }
    }

    fn fake_container_reference() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://acct.documents.azure.com/").unwrap(),
            "k",
        );
        let container_props = ContainerProperties {
            id: std::borrow::Cow::Borrowed("c"),
            partition_key: PartitionKeyDefinition::new(vec![std::borrow::Cow::Borrowed("/pk")]),
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(account, "db", "db_rid", "c", "c_rid", &container_props)
    }

    fn read_item_operation() -> CosmosOperation {
        let container = fake_container_reference();
        let item = ItemReference::from_name(&container, PartitionKey::from("pk"), "id");
        CosmosOperation::read_item(item)
    }

    fn create_item_operation() -> CosmosOperation {
        let container = fake_container_reference();
        let item = ItemReference::from_name(&container, PartitionKey::from("pk"), "id");
        CosmosOperation::create_item(item)
    }

    fn read_database_operation() -> CosmosOperation {
        let account = AccountReference::with_master_key(
            Url::parse("https://acct.documents.azure.com/").unwrap(),
            "k",
        );
        let db = DatabaseReference::from_name(account, "db");
        CosmosOperation::read_database(db)
    }

    fn enabled_strategy() -> HedgingStrategy {
        HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(500)).unwrap())
    }

    fn status(code: u16, sub: Option<u32>) -> CosmosStatus {
        let mut s = CosmosStatus::new(StatusCode::from(code));
        if let Some(v) = sub {
            s = s.with_sub_status(v as u16);
        }
        s
    }

    // ───────────────────────── should_hedge ─────────────────────────

    #[test]
    fn should_hedge_read_multi_region() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        assert!(should_hedge(Some(&enabled_strategy()), &op, &state, &[],));
    }

    #[test]
    fn should_hedge_read_single_region() {
        let state = account_state_with_regions(&[Region::EAST_US]);
        let op = read_item_operation();
        assert!(!should_hedge(Some(&enabled_strategy()), &op, &state, &[],));
    }

    #[test]
    fn should_hedge_excluded_to_one_region() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        let excluded = [Region::WEST_US_2];
        assert!(!should_hedge(
            Some(&enabled_strategy()),
            &op,
            &state,
            &excluded,
        ));
    }

    #[test]
    fn should_hedge_no_preferred_regions() {
        let state = account_state_with_regions(&[]);
        let op = read_item_operation();
        assert!(!should_hedge(Some(&enabled_strategy()), &op, &state, &[],));
    }

    #[test]
    fn should_hedge_write_never() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = create_item_operation();
        assert!(!should_hedge(Some(&enabled_strategy()), &op, &state, &[],));
    }

    #[test]
    fn should_hedge_non_document() {
        // Reads against non-Document resource types are excluded in Phase 1.
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_database_operation();
        assert!(!should_hedge(Some(&enabled_strategy()), &op, &state, &[],));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn should_hedge_distributed_transaction_never() {
        // DTX operations carry `ResourceType::DistributedTransactionBatch`, which
        // is not in HEDGEABLE_RESOURCE_TYPES, so neither the write commit nor the
        // read snapshot is hedge-eligible — even though the read reports
        // `is_read_only() == true`. This is the real guard: hedging can fire at
        // the pre-attempt STAGE 2b before the DTX classifier ever runs, so the
        // resource-type exclusion (not the classifier short-circuit) is what
        // keeps DTX out of hedging. Pin it directly.
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let account = AccountReference::with_master_key(
            Url::parse("https://acct.documents.azure.com/").unwrap(),
            "k",
        );
        let write = CosmosOperation::distributed_transaction(
            account.clone(),
            crate::models::DistributedTransactionType::Write,
        );
        let read = CosmosOperation::distributed_transaction(
            account,
            crate::models::DistributedTransactionType::Read,
        );
        assert!(!should_hedge(
            Some(&enabled_strategy()),
            &write,
            &state,
            &[]
        ));
        assert!(read.is_read_only());
        assert!(!should_hedge(Some(&enabled_strategy()), &read, &state, &[]));
    }

    #[test]
    fn should_hedge_disabled_override() {
        // `None` represents Disabled at any layer — short-circuits before
        // should_hedge is even called.
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        assert!(!should_hedge(None, &op, &state, &[]));
    }

    // ───────────────────────── is_final_result ─────────────────────────

    #[test]
    fn is_final_result_success() {
        assert!(status(200, None).is_final_result());
        assert!(status(201, None).is_final_result());
        assert!(status(304, None).is_final_result());
    }

    #[test]
    fn is_final_result_conflict() {
        assert!(status(409, None).is_final_result());
    }

    #[test]
    fn is_final_result_503() {
        assert!(!status(503, None).is_final_result());
    }

    #[test]
    fn is_final_result_404_0() {
        assert!(status(404, None).is_final_result());
        assert!(status(404, Some(0)).is_final_result());
    }

    #[test]
    fn is_final_result_404_1002() {
        assert!(!status(404, Some(1002)).is_final_result());
    }

    #[test]
    fn is_final_result_429() {
        // A generic 429 (no sub-status) and the transient-capacity 3092
        // sub-status stay transient — another region may have capacity.
        assert!(!status(429, None).is_final_result());
        assert!(!status(429, Some(3092)).is_final_result());
    }

    #[test]
    fn is_final_result_429_ru_budget_and_hot_partition_are_final() {
        // RU-budget / hot-partition throttles are account-/partition-wide;
        // racing a second region cannot relieve them, so they are final.
        assert!(status(429, Some(3200)).is_final_result()); // RU_BUDGET_EXCEEDED
        assert!(status(429, Some(3210)).is_final_result()); // RU_BUDGET_EXCEEDED_FOR_MASTER
        assert!(status(429, Some(3214)).is_final_result()); // HOT_PARTITION_KEY_THROTTLED
    }

    #[test]
    fn is_final_result_403_is_final_regardless_of_sub_status() {
        // Most 403s are final for hedging; 403/1008 is topology ownership and must reach retry.
        assert!(status(403, None).is_final_result());
        assert!(status(403, Some(3)).is_final_result()); // WRITE_FORBIDDEN
        assert!(!status(403, Some(1008)).is_final_result()); // DATABASE_ACCOUNT_NOT_FOUND
        assert!(status(403, Some(5)).is_final_result()); // arbitrary unknown sub-status
    }

    #[test]
    fn is_final_result_protocol_and_policy_codes_are_final() {
        // Payload / policy / protocol errors that no alternate region can
        // resolve — racing them just wastes RU and request budget.
        for code in [422_u16, 451, 501, 505] {
            assert!(
                status(code, None).is_final_result(),
                "expected {} to be final",
                code
            );
        }
    }

    #[test]
    fn is_final_result_generic_5xx_remain_retriable() {
        // Generic 500 InternalServerError is left retriable so a hedge
        // against another region can still win — the new final-set
        // expansion intentionally excludes 500 / 502 / 504.
        assert!(!status(500, None).is_final_result());
        assert!(!status(502, None).is_final_result());
        assert!(!status(504, None).is_final_result());
    }

    #[test]
    fn is_final_result_other_client_errors_final() {
        for code in [400_u16, 401, 405, 412, 413] {
            assert!(
                status(code, None).is_final_result(),
                "expected {} to be final",
                code
            );
        }
    }

    // ───────────────────────── resolve_availability_strategy ─────────────────────────

    fn empty_view<'a>(op: &'a OperationOptions) -> OperationOptionsView<'a> {
        OperationOptionsView::new(None, None, None, Some(op))
    }

    #[test]
    fn resolve_returns_driver_default_when_nothing_set() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy(&view, None).expect("driver default is Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_driver_default_uses_half_request_timeout_when_under_cap() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy =
            resolve_availability_strategy(&view, Some(Duration::from_millis(600))).expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(300));
    }

    #[test]
    fn resolve_driver_default_caps_at_1000ms() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy =
            resolve_availability_strategy(&view, Some(Duration::from_secs(30))).expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_operation_disabled_returns_none() {
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let view = empty_view(&op);

        assert!(resolve_availability_strategy(&view, None).is_none());
    }

    #[test]
    fn resolve_operation_hedging_strategy_used_directly() {
        let op_strategy =
            HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(200)).unwrap());
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(op_strategy))
            .build();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy(&view, None).expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(200));
    }

    #[test]
    fn resolve_hedging_enabled_flag_false_returns_none() {
        // `hedging_enabled = Some(false)` at the env layer disables hedging
        // even when the operation has no explicit availability strategy.
        let env = OperationOptionsBuilder::new()
            .with_hedging_enabled(false)
            .build();
        let op = OperationOptions::default();
        let view = OperationOptionsView::new(Some(std::sync::Arc::new(env)), None, None, Some(&op));

        assert!(resolve_availability_strategy(&view, None).is_none());
    }

    #[test]
    fn resolve_hedging_enabled_flag_false_overrides_explicit_strategy() {
        // The env-resolved kill-switch is the source of truth: it disables
        // hedging even when the caller passes an explicit
        // `AvailabilityStrategy::Hedging(..)` at the operation layer.
        let op_strategy =
            HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(200)).unwrap());
        let env = OperationOptionsBuilder::new()
            .with_hedging_enabled(false)
            .build();
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(op_strategy))
            .build();
        let view = OperationOptionsView::new(Some(std::sync::Arc::new(env)), None, None, Some(&op));

        assert!(resolve_availability_strategy(&view, None).is_none());
    }

    #[test]
    fn resolve_hedging_enabled_flag_true_keeps_default_threshold() {
        // Explicitly enabling hedging (without an explicit strategy) keeps the
        // unchanged driver-default threshold of `min(1000ms, timeout / 2)`.
        let env = OperationOptionsBuilder::new()
            .with_hedging_enabled(true)
            .build();
        let op = OperationOptions::default();
        let view = OperationOptionsView::new(Some(std::sync::Arc::new(env)), None, None, Some(&op));

        let strategy = resolve_availability_strategy(&view, None).expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_hedging_enabled_flag_true_overrides_explicit_disabled() {
        // Parity: the env switch is the source of truth in both directions.
        // `hedging_enabled = Some(true)` enables hedging even when the caller
        // passed an explicit `AvailabilityStrategy::Disabled`.
        let env = OperationOptionsBuilder::new()
            .with_hedging_enabled(true)
            .build();
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let view = OperationOptionsView::new(Some(std::sync::Arc::new(env)), None, None, Some(&op));

        let strategy =
            resolve_availability_strategy(&view, None).expect("env=true enables hedging");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_hedging_enabled_flag_true_honors_programmatic_custom_threshold() {
        // When the env switch enables hedging and the caller also configured a
        // programmatic `Hedging(..)` strategy, the custom threshold is honored
        // (env=true means "on", it does not reset an explicit threshold).
        let op_strategy =
            HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(200)).unwrap());
        let env = OperationOptionsBuilder::new()
            .with_hedging_enabled(true)
            .build();
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(op_strategy))
            .build();
        let view = OperationOptionsView::new(Some(std::sync::Arc::new(env)), None, None, Some(&op));

        let strategy = resolve_availability_strategy(&view, None).expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(200));
    }

    #[test]
    fn resolve_env_override_layer_disables_hedging_over_operation() {
        // End-to-end: the top-priority `env_override` kill-switch layer
        // (sourced from `AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE`) must reach the
        // live `resolve_availability_strategy` and beat a per-operation
        // `hedging_enabled = Some(true)` *and* an explicit `Hedging(..)`.
        let op_strategy =
            HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(200)).unwrap());
        let env_override = OperationOptionsBuilder::new()
            .with_hedging_enabled(false)
            .build();
        let op = OperationOptionsBuilder::new()
            .with_hedging_enabled(true)
            .with_availability_strategy(AvailabilityStrategy::Hedging(op_strategy))
            .build();
        let view = OperationOptionsView::new_with_override(
            Some(std::sync::Arc::new(env_override)),
            None,
            None,
            None,
            Some(&op),
        );

        assert!(
            resolve_availability_strategy(&view, None).is_none(),
            "env_override hedging=false must disable hedging through the pipeline resolver",
        );
    }

    #[test]
    fn resolve_env_override_layer_enables_hedging_over_disabled_operation() {
        // Parity in the other direction through the pipeline resolver: an
        // override of `true` enables hedging even when the operation layer
        // explicitly set `Disabled`.
        let env_override = OperationOptionsBuilder::new()
            .with_hedging_enabled(true)
            .build();
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let view = OperationOptionsView::new_with_override(
            Some(std::sync::Arc::new(env_override)),
            None,
            None,
            None,
            Some(&op),
        );

        let strategy = resolve_availability_strategy(&view, None)
            .expect("env_override hedging=true must enable hedging through the pipeline resolver");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    // ───────────────────────── ExcludedRegions integration ─────────────────────────

    #[test]
    fn should_hedge_via_excluded_regions_field() {
        // Sanity-check that the ExcludedRegions type from the public
        // options surface lines up with the &[Region] slice that
        // should_hedge consumes.
        let state =
            account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US]);
        let op = read_item_operation();
        let excluded: ExcludedRegions = [Region::EAST_US, Region::WEST_US_2].into_iter().collect();
        // Two of three regions excluded → only one applicable → false.
        assert!(!should_hedge(
            Some(&enabled_strategy()),
            &op,
            &state,
            &excluded.0,
        ));
    }

    // ───────────────────────── evaluate_hedge_eligibility ─────────────────────────

    fn primary_routing_for(account: &AccountEndpointState) -> RoutingDecision {
        let ep = account
            .preferred_read_endpoints
            .first()
            .cloned()
            .unwrap_or_else(|| account.default_endpoint.clone());
        let url = ep.selected_url(false).clone();
        let endpoint_key = ep.endpoint_key();
        RoutingDecision {
            selected_url: url,
            transport_mode: TransportMode::Gateway,
            endpoint_key,
            endpoint: ep,
        }
    }

    /// Builds a `RoutingDecision` targeting the preferred-read endpoint at
    /// the given index. Used to simulate cases where the primary has been
    /// promoted off `preferred_read_endpoints[0]` by PPCB / probe-candidate
    /// routing / location-index advancement / prior retry state.
    fn primary_routing_for_index(account: &AccountEndpointState, index: usize) -> RoutingDecision {
        let ep = account
            .preferred_read_endpoints
            .get(index)
            .cloned()
            .expect("test fixture must supply enough preferred read endpoints");
        let url = ep.selected_url(false).clone();
        let endpoint_key = ep.endpoint_key();
        RoutingDecision {
            selected_url: url,
            transport_mode: TransportMode::Gateway,
            endpoint_key,
            endpoint: ep,
        }
    }

    #[test]
    fn evaluate_returns_some_for_eligible_read_multi_region() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        let upgrade = evaluate_hedge_eligibility(&op, &view, &state, &primary, None)
            .expect("eligible multi-region read");

        // Secondary pinned to applicable[1] = WEST_US_2.
        assert_eq!(
            upgrade.secondary_routing.endpoint.region(),
            Some(&Region::WEST_US_2)
        );
        // Driver default threshold applies (no env, no request_timeout).
        assert_eq!(upgrade.threshold.get(), Duration::from_millis(1000));
        assert_eq!(
            upgrade.strategy_config,
            HedgingStrategyConfig::new(upgrade.threshold)
        );
    }

    #[test]
    fn evaluate_returns_none_for_single_region_account() {
        let state = account_state_with_regions(&[Region::EAST_US]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        assert!(evaluate_hedge_eligibility(&op, &view, &state, &primary, None).is_none());
    }

    #[test]
    fn evaluate_returns_none_for_write_operation() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = create_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        assert!(evaluate_hedge_eligibility(&op, &view, &state, &primary, None).is_none());
    }

    #[test]
    fn evaluate_returns_none_when_strategy_disabled() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        assert!(evaluate_hedge_eligibility(&op, &view, &state, &primary, None).is_none());
    }

    #[test]
    fn evaluate_returns_none_when_user_exclusion_leaves_one_region() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let mut op_opts = OperationOptions::default();
        op_opts.excluded_regions = Some([Region::WEST_US_2].into_iter().collect());
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        assert!(evaluate_hedge_eligibility(&op, &view, &state, &primary, None).is_none());
    }

    #[test]
    fn evaluate_secondary_routing_uses_secondary_endpoint_url() {
        // Confirm the secondary RoutingDecision is constructed from the
        // alternate endpoint (not the primary): the URL must point at
        // the WEST_US_2 host, and `endpoint.region()` must agree.
        let state =
            account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        let upgrade = evaluate_hedge_eligibility(&op, &view, &state, &primary, None)
            .expect("eligible three-region read");

        let url_str = upgrade.secondary_routing.selected_url.as_str();
        assert!(
            url_str.contains(Region::WEST_US_2.as_str()),
            "secondary URL {} did not contain westus2 region tag",
            url_str
        );
        // GatewayV2 not enabled on the test endpoints — falls back to Gateway.
        assert_eq!(
            upgrade.secondary_routing.transport_mode,
            TransportMode::Gateway
        );
    }

    /// A GatewayV2-capable secondary hedge leg must key the connection pool by
    /// the thin-client proxy authority (`host:proxy-port`) it actually dials,
    /// NOT the logical G1 endpoint authority. Mirrors `resolve_endpoint` so the
    /// hedge leg shares the main path's G2 connection pool instead of opening a
    /// duplicate one keyed by the G1 host.
    #[test]
    fn evaluate_secondary_routing_keys_gateway_v2_leg_by_proxy_authority() {
        fn g2_endpoint_for(region: Region) -> CosmosEndpoint {
            let gateway_url = Url::parse(&format!(
                "https://acct-{}.documents.azure.com/",
                region.as_str()
            ))
            .expect("valid url");
            let gateway_v2_url = Url::parse(&format!(
                "https://proxy-{}.documents.azure.com:10250/",
                region.as_str()
            ))
            .expect("valid url");
            CosmosEndpoint::regional_with_gateway_v2(region, gateway_url, gateway_v2_url)
        }

        let endpoints: Vec<CosmosEndpoint> = [Region::EAST_US, Region::WEST_US_2]
            .into_iter()
            .map(g2_endpoint_for)
            .collect();
        let state = AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: endpoints.clone().into(),
            preferred_write_endpoints: endpoints.clone().into(),
            account_write_endpoints: endpoints.clone().into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: endpoints[0].clone(),
        };

        // Primary on EAST_US over GatewayV2 so `prefer_gateway_v2` propagates
        // to the secondary-leg selection.
        let primary_ep = endpoints[0].clone();
        let primary = RoutingDecision {
            selected_url: primary_ep.selected_url(true).clone(),
            transport_mode: TransportMode::GatewayV2,
            endpoint_key: EndpointKey::try_from(primary_ep.selected_url(true))
                .expect("valid proxy url"),
            endpoint: primary_ep,
        };

        let op = read_item_operation();
        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        let upgrade = evaluate_hedge_eligibility(&op, &view, &state, &primary, None)
            .expect("eligible multi-region GatewayV2 read");
        let secondary = &upgrade.secondary_routing;

        assert_eq!(secondary.transport_mode, TransportMode::GatewayV2);

        // The dialed URL is the WEST_US_2 proxy on port 10250.
        let proxy_url = secondary.endpoint.selected_url(true);
        assert_eq!(secondary.selected_url.as_str(), proxy_url.as_str());
        assert!(secondary.selected_url.as_str().contains(":10250"));

        // The pool key must be derived from that proxy URL, NOT the G1 authority.
        let expected_key = EndpointKey::try_from(proxy_url).expect("valid proxy url");
        assert_eq!(secondary.endpoint_key, expected_key);
        assert_ne!(
            secondary.endpoint_key,
            secondary.endpoint.endpoint_key(),
            "G2 hedge leg must not key by the logical G1 endpoint authority",
        );
    }

    /// Threshold derivation must be stable across repeated calls.
    ///
    /// Calling `evaluate_hedge_eligibility` repeatedly with the same
    /// **configured** `request_timeout` must return the same threshold
    /// every time, regardless of how much wall-clock time has elapsed
    /// between calls. The caller must pass the configured value from
    /// `OperationOptionsView::end_to_end_latency_policy()`, **not** the
    /// remaining time until the deadline — otherwise the threshold would
    /// shrink with elapsed time on retry-driven upgrades.
    #[test]
    fn evaluate_hedge_eligibility_threshold_stable_across_repeated_calls() {
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        let primary = primary_routing_for(&state);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        // Configured timeout = 800ms → expected threshold = 400ms (cap = 1s).
        let configured_timeout = Some(Duration::from_millis(800));

        let first = evaluate_hedge_eligibility(&op, &view, &state, &primary, configured_timeout)
            .expect("first call eligible");
        std::thread::sleep(Duration::from_millis(10));
        let second = evaluate_hedge_eligibility(&op, &view, &state, &primary, configured_timeout)
            .expect("second call eligible");

        assert_eq!(
            first.threshold.get(),
            Duration::from_millis(400),
            "first call must derive threshold from configured timeout",
        );
        assert_eq!(
            second.threshold.get(),
            first.threshold.get(),
            "threshold must be stable across calls: the caller must pass \
             the configured request_timeout (not remaining deadline) so \
             the default does not shrink between attempts",
        );
    }

    /// Alternate selection must skip the primary.
    ///
    /// Before this fix, the alternate was unconditionally
    /// `applicable_regions[1]`. When the primary had been promoted off
    /// `preferred_read_endpoints[0]` (by PPCB overrides, probe-candidate
    /// routing, location-index advancement, or prior retry state), this
    /// could pick the **same region** as the primary — doubling RU/load
    /// with zero availability benefit. The fix selects the first
    /// applicable endpoint whose region/endpoint_key differs from the
    /// primary's, regardless of index.
    #[test]
    fn evaluate_secondary_skips_primary_when_primary_is_not_index_zero() {
        let state =
            account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US]);
        let op = read_item_operation();
        // Primary has been promoted off index 0 → it is now WEST_US_2
        // (index 1). The pre-fix code would have picked WEST_US_2 as
        // alternate (== primary). The fix must pick EAST_US or CENTRAL_US.
        let primary = primary_routing_for_index(&state, 1);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        let upgrade = evaluate_hedge_eligibility(&op, &view, &state, &primary, None)
            .expect("eligible three-region read");

        let secondary_region = upgrade.secondary_routing.endpoint.region().cloned();
        assert_ne!(
            secondary_region.as_ref(),
            Some(&Region::WEST_US_2),
            "alternate must not target the primary's region",
        );
        assert_ne!(
            upgrade.secondary_routing.endpoint.endpoint_key(),
            primary.endpoint.endpoint_key(),
            "alternate must not target the primary's endpoint_key",
        );
        // Implementation chooses the first non-primary applicable region
        // in `preferred_read_endpoints` order → EAST_US (index 0).
        assert_eq!(secondary_region.as_ref(), Some(&Region::EAST_US));
    }

    /// Companion to
    /// `evaluate_secondary_skips_primary_when_primary_is_not_index_zero`.
    ///
    /// When the primary has been promoted all the way to the *last*
    /// preferred-read endpoint (e.g. PPCB tripped every earlier region, or
    /// repeated location-index advancement walked the primary to the tail),
    /// the operation must still be hedged, and the alternate must be the
    /// first applicable region distinct from the promoted primary.
    #[test]
    fn evaluate_hedges_when_primary_is_last_preferred_endpoint() {
        let state =
            account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US]);
        let op = read_item_operation();
        // Primary promoted off index 0 to the tail (index 2 = CENTRAL_US).
        let primary = primary_routing_for_index(&state, 2);

        let op_opts = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        let upgrade = evaluate_hedge_eligibility(&op, &view, &state, &primary, None).expect(
            "request must still be hedged when the primary is not \
             preferred_read_endpoints[0]",
        );

        let secondary_region = upgrade.secondary_routing.endpoint.region().cloned();
        // First applicable region distinct from the primary, in
        // `preferred_read_endpoints` order → EAST_US (index 0).
        assert_eq!(secondary_region.as_ref(), Some(&Region::EAST_US));
        assert_ne!(
            secondary_region.as_ref(),
            primary.endpoint.region(),
            "alternate must differ from the promoted primary region",
        );
        assert_ne!(
            upgrade.secondary_routing.endpoint.endpoint_key(),
            primary.endpoint.endpoint_key(),
            "alternate must differ from the promoted primary endpoint_key",
        );
    }

    /// Degenerate case: only the primary is
    /// applicable (e.g. user excluded every other region, or every other
    /// preferred endpoint aliases to the same region/endpoint_key).
    /// `evaluate_hedge_eligibility` must return `None` rather than build a
    /// same-region hedge.
    #[test]
    fn evaluate_returns_none_when_only_applicable_region_is_primary() {
        let state =
            account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US]);
        let op = read_item_operation();
        // Primary is EAST_US (index 0); user excludes the other two →
        // no distinct alternate exists.
        let primary = primary_routing_for(&state);

        let mut op_opts = OperationOptions::default();
        op_opts.excluded_regions = Some(
            [Region::WEST_US_2, Region::CENTRAL_US]
                .into_iter()
                .collect(),
        );
        let view = OperationOptionsView::new(None, None, None, Some(&op_opts));

        assert!(
            evaluate_hedge_eligibility(&op, &view, &state, &primary, None).is_none(),
            "no distinct alternate exists — must fall back to non-hedged decision",
        );
    }
}
