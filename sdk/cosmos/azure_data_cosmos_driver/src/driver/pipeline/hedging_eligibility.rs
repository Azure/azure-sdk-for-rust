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
            Some(r) => !excluded_regions.contains(r),
            None => true,
        })
        .count();

    applicable >= 2
}

/// Resolves the effective [`HedgingStrategy`] for a single operation.
///
/// Priority order (highest first):
///
/// 1. Operation / account / runtime `availability_strategy` (resolved by
///    [`OperationOptionsView`] before we are called).
/// 2. Driver default — `min(1000ms, request_timeout / 2)`.
///
/// Returns `None` only when an explicit `AvailabilityStrategy::Disabled`
/// at layer 1 turns hedging off; otherwise always returns `Some(_)`.
/// The "single-region account / insufficient regions" case is enforced
/// separately in [`should_hedge`].
pub(crate) fn resolve_availability_strategy(
    view: &OperationOptionsView<'_>,
    request_timeout: Option<Duration>,
) -> Option<HedgingStrategy> {
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
/// falling back to `1000ms` when `request_timeout` is `None` or zero.
fn default_threshold(request_timeout: Option<Duration>) -> HedgeThreshold {
    let candidate = match request_timeout {
        Some(t) if !t.is_zero() => (t / 2).min(DEFAULT_THRESHOLD_CAP),
        _ => DEFAULT_THRESHOLD_CAP,
    };

    // `candidate` can only be zero if `request_timeout` is `Some(1ns)`
    // (which floors to zero after `/2`). In that degenerate case, fall
    // back to the cap so the newtype invariant holds — the caller is
    // already running with an unrealistic timeout.
    HedgeThreshold::new(candidate).unwrap_or_else(|| {
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
/// used to honor the same gateway-version preference when constructing the
/// secondary [`RoutingDecision`]. `request_timeout` is plumbed through to
/// [`resolve_availability_strategy`] so the driver default
/// (`min(1000ms, request_timeout / 2)`) can be computed.
///
/// Returns `None` when hedging is disabled, the operation is ineligible,
/// or no alternate region can be selected — in all cases the caller falls
/// back to its non-hedged decision (typically `FailoverRetry`).
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
    // Order matches `preferred_read_endpoints`; index 1 is the alternate.
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

    let secondary_region = applicable_regions[1].clone();
    let secondary_ep = account_state
        .preferred_read_endpoints
        .iter()
        .find(|ep| ep.region() == Some(&secondary_region))?
        .clone();

    // Match the primary's gateway-version preference so a Gateway20-capable
    // account uses Gateway20 for both legs (and downgrades cleanly for legacy).
    let prefer_gateway20 = matches!(primary.transport_mode, TransportMode::Gateway20);
    let use_gateway20 = secondary_ep.uses_gateway20(prefer_gateway20);
    let transport_mode = if use_gateway20 {
        TransportMode::Gateway20
    } else {
        TransportMode::Gateway
    };
    let secondary_routing = RoutingDecision {
        selected_url: secondary_ep.selected_url(use_gateway20).clone(),
        transport_mode,
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
            preferred_write_endpoints: endpoints.into(),
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
            s = s.with_sub_status(v);
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
        assert!(!status(429, None).is_final_result());
    }

    #[test]
    fn is_final_result_403_transient() {
        // Per §7.2 row, 403 (with or without sub-status) is transient
        // for hedging purposes.
        assert!(!status(403, None).is_final_result());
        assert!(!status(403, Some(3)).is_final_result());
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
        RoutingDecision {
            selected_url: url,
            transport_mode: TransportMode::Gateway,
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
        // Gateway20 not enabled on the test endpoints — falls back to Gateway.
        assert_eq!(
            upgrade.secondary_routing.transport_mode,
            TransportMode::Gateway
        );
    }

    /// Spec D1 regression — §5.2 threshold derivation.
    ///
    /// Calling `evaluate_hedge_eligibility` repeatedly with the same
    /// **configured** `request_timeout` must return the same threshold
    /// every time, regardless of how much wall-clock time has elapsed
    /// between calls. The caller's contract (per `execute_operation_pipeline`
    /// STAGE 2b / 5b) is to pass the configured value from
    /// `OperationOptionsView::end_to_end_latency_policy()`, **not** the
    /// remaining time until the deadline. Before the D1 fix, the call
    /// sites passed `deadline.map(|d| d.saturating_duration_since(now))`,
    /// which would shrink with elapsed time on STAGE 5b retry-driven
    /// upgrades and silently violate the spec's `min(1000ms, configured / 2)`
    /// formula.
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
            "threshold must be stable across calls — D1 regression: \
             caller must pass configured request_timeout (not remaining \
             deadline) so the §5.2 default does not shrink between \
             STAGE 2b and STAGE 5b",
        );
    }
}
