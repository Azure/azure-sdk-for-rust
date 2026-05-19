// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pure, side-effect-free helpers that govern cross-region hedging.
//!
//! See [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md):
//!
//! - §5.1 — [`should_hedge`] decision matrix.
//! - §5.2 — default-on activation and the
//!   `min(1000ms, request_timeout / 2)` driver default threshold.
//! - §6.3 — [`build_secondary_excluded_regions`] alternate-region pinning.
//! - §7.1 — [`is_final_result`] response-classification predicate.
//! - §11.3.1 — [`resolve_availability_strategy`] precedence chain.
//!
//! These functions are consumed by `evaluate_transport_result`
//! (Transport Pipeline Spec §3.4) when it produces
//! `OperationAction::Hedge { secondary_routing }` and by the
//! `execute_hedged()` race loop. Both call sites land in Part 4 of
//! `docs/HEDGING_IMPLEMENTATION_PLAN.md`; once Part 4b wired the
//! evaluator + STAGE 7 dispatch the module-level `dead_code` allow
//! was removed.

use std::time::Duration;

use crate::{
    driver::{
        pipeline::{
            components::{RoutingDecision, TransportMode},
            hedging_diagnostics::HedgingStrategyConfig,
        },
        routing::AccountEndpointState,
    },
    models::{CosmosOperation, CosmosStatus, OperationType, ResourceType},
    options::{
        env_parsing, AvailabilityStrategy, HedgeThreshold, HedgingStrategy, OperationOptionsView,
        Region,
    },
};

/// Default hedge threshold cap used by [`resolve_availability_strategy`]
/// when no operation- / account- / env-level strategy is configured.
///
/// Per spec §5.2 the driver-default threshold is
/// `min(1000ms, request_timeout / 2)`; this constant is the upper bound.
const DEFAULT_THRESHOLD_CAP: Duration = Duration::from_millis(1000);

/// Phase 1 allowed `ResourceType` set per spec §5.1 footnote.
///
/// Subsequent phases widen this single constant — no other change to
/// [`should_hedge`] is required.
const PHASE_ONE_RESOURCE_TYPES: &[ResourceType] = &[ResourceType::Document];

/// Phase 1 allowed `OperationType` set per spec §5.1 footnote.
///
/// Phase 2 will append feed-style operations
/// (`Query` / `ReadFeed` / `QueryPlan`) and metadata reads.
const PHASE_ONE_OPERATION_TYPES: &[OperationType] = &[OperationType::Read];

/// Returns `true` when the status is a **final** (non-transient) outcome
/// per spec §7.1: any 1xx/2xx/3xx, the explicitly non-transient client
/// errors (`400`, `401`, `405`, `409`, `412`, `413`), or `404` with no
/// sub-status. Everything else — including `404/1002`, `408`, `429`,
/// `503`, and `403` regardless of sub-status — is treated as transient
/// so that the racing hedge gets a chance to win.
pub(crate) fn is_final_result(status: &CosmosStatus) -> bool {
    let code: u16 = status.status_code().into();
    if code < 400 {
        return true;
    }

    let sub = status.sub_status().map(|s| s.value()).unwrap_or(0);
    matches!(code, 400 | 401 | 405 | 409 | 412 | 413) || (code == 404 && sub == 0)
}

/// Returns `true` when the operation is eligible for cross-region
/// hedging per spec §5.1.
///
/// `strategy` is the resolved strategy from
/// [`resolve_availability_strategy`]. `None` represents either an
/// explicit `AvailabilityStrategy::Disabled` at any layer or the
/// `AZURE_COSMOS_HEDGING_DISABLED=true` env-var kill switch — both
/// short-circuit to `false` (Row 1).
///
/// `excluded_regions` is the post-resolution `ExcludeRegions` set from
/// the operation's options view; the applicable preferred-region count
/// is computed against the post-filter list (Row 5).
pub(crate) fn should_hedge(
    strategy: Option<&HedgingStrategy>,
    operation: &CosmosOperation,
    account_state: &AccountEndpointState,
    excluded_regions: &[Region],
) -> bool {
    // Row 1 — strategy resolved?
    if strategy.is_none() {
        return false;
    }

    // Row 2 — non-empty preferred-region list?
    if account_state.preferred_read_endpoints.is_empty() {
        return false;
    }

    // Row 3 — phase-allowed ResourceType?
    if !PHASE_ONE_RESOURCE_TYPES.contains(&operation.resource_type()) {
        return false;
    }

    // Row 4 — writes are never hedged. Phase 1 also restricts
    // OperationType to `Read`, which is a superset of "not a write",
    // but keeping the explicit `is_read_only()` guard documents the
    // intent and protects against future phase widenings that add
    // non-read OperationTypes (e.g. feed reads) without revisiting
    // this predicate.
    let op = operation.operation_type();
    if !op.is_read_only() {
        return false;
    }
    if !PHASE_ONE_OPERATION_TYPES.contains(&op) {
        return false;
    }

    // Row 5 — ≥ 2 applicable preferred read endpoints after
    // ExcludeRegions filtering.
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

/// Resolves the effective [`HedgingStrategy`] for a single operation per
/// spec §11.3.1.
///
/// Priority order (highest first):
///
/// 1. Operation / account / runtime `availability_strategy` (resolved by
///    [`OperationOptionsView`] before we are called).
/// 2. `AZURE_COSMOS_HEDGING_DISABLED=true` env-var kill switch
///    (short-circuits to `None`).
/// 3. `AZURE_COSMOS_HEDGING_THRESHOLD_MS` env var (threshold override,
///    still wrapped in the driver-default `HedgingStrategy`).
/// 4. Driver default — `min(1000ms, request_timeout / 2)` per §5.2.
///
/// Returns `None` only when an explicit `AvailabilityStrategy::Disabled`
/// at layers 1–2 turns hedging off; otherwise always returns `Some(_)`.
/// The "single-region account / insufficient regions" case (priority 5)
/// is enforced separately in [`should_hedge`].
pub(crate) fn resolve_availability_strategy(
    view: &OperationOptionsView<'_>,
    request_timeout: Option<Duration>,
) -> Option<HedgingStrategy> {
    resolve_availability_strategy_with(view, request_timeout, |name| std::env::var(name))
}

/// Test seam for [`resolve_availability_strategy`] that lets unit tests
/// inject env-var lookups without mutating process state.
pub(crate) fn resolve_availability_strategy_with(
    view: &OperationOptionsView<'_>,
    request_timeout: Option<Duration>,
    env_var: impl Fn(&str) -> Result<String, std::env::VarError> + Copy,
) -> Option<HedgingStrategy> {
    // Priority 1 — code-level strategy.
    match view.availability_strategy() {
        Some(AvailabilityStrategy::Disabled) => return None,
        Some(AvailabilityStrategy::Hedging(s)) => return Some(*s),
        None => {}
    }

    // Priority 2 — env-var kill switch.
    if env_parsing::parse_hedging_disabled_from_env_with(env_var) {
        return None;
    }

    // Priority 3 — env-var threshold override (still produces a strategy).
    if let Some(env_threshold) = env_parsing::parse_hedging_threshold_from_env_with(env_var) {
        if let Some(t) = HedgeThreshold::new(env_threshold) {
            return Some(HedgingStrategy::new(t));
        }
    }

    // Priority 4 — driver default per §5.2.
    Some(HedgingStrategy::new(default_threshold(request_timeout)))
}

/// Computes the §5.2 driver-default threshold:
/// `min(1000ms, request_timeout / 2)`, falling back to `1000ms` when
/// `request_timeout` is `None` or zero.
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

/// Computes the `ExcludeRegions` set for the alternate hedge per spec §6.3.
///
/// The returned vector is `user_excluded ∪ (all_regions \ {all_regions[secondary_index]})` —
/// i.e. the alternate hedge is pinned to `all_regions[secondary_index]` by
/// excluding *every other* region (the primary and any tertiary
/// fallbacks) on top of the user's own exclusions. Order matches insertion
/// order, with `user_excluded` first.
///
/// `secondary_index` defaults to `1` in normal flow (second preferred
/// region), but is parameterized so callers can pin to any specific
/// index. If `secondary_index` is out of range the function falls back
/// to excluding nothing additional — the caller's existing exclusions
/// are returned unchanged.
pub(crate) fn build_secondary_excluded_regions(
    user_excluded: &[Region],
    all_regions: &[Region],
    secondary_index: usize,
) -> Vec<Region> {
    let mut excluded: Vec<Region> = user_excluded.to_vec();
    for (i, r) in all_regions.iter().enumerate() {
        if i == secondary_index {
            continue;
        }
        if !excluded.contains(r) {
            excluded.push(r.clone());
        }
    }
    excluded
}

/// Outcome of [`evaluate_hedge_eligibility`] — everything the pipeline
/// needs to dispatch [`OperationAction::Hedge`] for a single attempt.
///
/// Returned only when all five gates from spec §6.1 hold:
///
/// 1. A [`HedgingStrategy`] resolved via §11.3.1.
/// 2. [`should_hedge`] is `true` for the operation + post-exclusion
///    applicable-region count.
/// 3. The post-filter applicable list has ≥ 2 entries (i.e. an
///    alternate region exists at all).
/// 4. The alternate endpoint at index 1 of the applicable list resolves
///    to a concrete [`CosmosEndpoint`] in `account_state`.
///
/// `secondary_excluded_regions` is the per-hedge `ExcludeRegions` set
/// computed by [`build_secondary_excluded_regions`] (spec §6.3); the
/// secondary attempt is pinned to the alternate region by excluding
/// *every other* applicable region on top of the user's own exclusions.
///
/// [`OperationAction::Hedge`]:
/// crate::driver::pipeline::components::OperationAction::Hedge
#[derive(Debug)]
pub(crate) struct HedgeUpgrade {
    /// Routing decision for the alternate-region hedge.
    pub(crate) secondary_routing: RoutingDecision,
    /// `ExcludeRegions` set to pin the hedge to its alternate region.
    pub(crate) secondary_excluded_regions: Vec<Region>,
    /// The resolved hedge threshold (used to schedule the timer).
    pub(crate) threshold: HedgeThreshold,
    /// Snapshot of the strategy config for `HedgeDiagnostics` (§10.1).
    pub(crate) strategy_config: HedgingStrategyConfig,
}

/// Evaluates whether the per-attempt transient outcome should be upgraded
/// to a cross-region hedge per spec §6.1, returning the materialized
/// [`HedgeUpgrade`] when all gates hold.
///
/// `primary` is the routing decision for the just-completed attempt; it is
/// used to honor the same gateway-version preference when constructing the
/// secondary [`RoutingDecision`]. `request_timeout` is plumbed through to
/// [`resolve_availability_strategy`] so the §5.2 driver default
/// (`min(1000ms, request_timeout / 2)`) can be computed.
///
/// Returns `None` when hedging is disabled, the operation is ineligible,
/// or no alternate region can be selected — in all cases the caller
/// falls back to its non-hedged decision (typically `FailoverRetry`).
pub(crate) fn evaluate_hedge_eligibility(
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
    account_state: &AccountEndpointState,
    primary: &RoutingDecision,
    request_timeout: Option<Duration>,
) -> Option<HedgeUpgrade> {
    // Gate 1 — strategy resolution (§11.3.1).
    let strategy = resolve_availability_strategy(options, request_timeout)?;

    // Per-operation user `ExcludeRegions` (post-resolution view).
    let user_excluded: Vec<Region> = options
        .excluded_regions()
        .map(|r| r.0.clone())
        .unwrap_or_default();

    // Gate 2 — should_hedge against post-exclusion applicable count.
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

    // Gate 3 — we need an alternate. `should_hedge` already enforced
    // `applicable >= 2`, but re-check defensively to keep this function
    // independently sound (the test seam may bypass `should_hedge`).
    if applicable_regions.len() < 2 {
        return None;
    }

    // Gate 4 — resolve the alternate region back to its `CosmosEndpoint`.
    // The applicable list is built from regional endpoints; the lookup
    // must succeed unless the account_state mutated between filter and
    // find (impossible — we hold `&AccountEndpointState`).
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

    let secondary_excluded_regions =
        build_secondary_excluded_regions(&user_excluded, &applicable_regions, 1);

    Some(HedgeUpgrade {
        secondary_routing,
        secondary_excluded_regions,
        threshold: strategy.threshold(),
        strategy_config: HedgingStrategyConfig::new(strategy.threshold()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

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
        // `None` represents either Disabled at any layer or env-disabled
        // — both short-circuit before should_hedge is even called.
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();
        assert!(!should_hedge(None, &op, &state, &[]));
    }

    #[test]
    fn should_hedge_env_disabled() {
        // Mirrors the disabled-override semantic: env-disabled produces
        // `None` out of resolve_availability_strategy_with, which the
        // caller forwards into should_hedge as `None`.
        let state = account_state_with_regions(&[Region::EAST_US, Region::WEST_US_2]);
        let op = read_item_operation();

        let env = Arc::new(OperationOptions::default());
        let runtime = Arc::new(OperationOptions::default());
        let account = Arc::new(OperationOptions::default());
        let operation = OperationOptions::default();
        let view =
            OperationOptionsView::new(Some(env), Some(runtime), Some(account), Some(&operation));

        let resolved = resolve_availability_strategy_with(&view, None, |name| match name {
            "AZURE_COSMOS_HEDGING_DISABLED" => Ok("true".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });
        assert!(resolved.is_none());
        assert!(!should_hedge(resolved.as_ref(), &op, &state, &[]));
    }

    // ───────────────────────── is_final_result ─────────────────────────

    #[test]
    fn is_final_result_success() {
        assert!(is_final_result(&status(200, None)));
        assert!(is_final_result(&status(201, None)));
        assert!(is_final_result(&status(304, None)));
    }

    #[test]
    fn is_final_result_conflict() {
        assert!(is_final_result(&status(409, None)));
    }

    #[test]
    fn is_final_result_503() {
        assert!(!is_final_result(&status(503, None)));
    }

    #[test]
    fn is_final_result_404_0() {
        assert!(is_final_result(&status(404, None)));
        assert!(is_final_result(&status(404, Some(0))));
    }

    #[test]
    fn is_final_result_404_1002() {
        assert!(!is_final_result(&status(404, Some(1002))));
    }

    #[test]
    fn is_final_result_429() {
        assert!(!is_final_result(&status(429, None)));
    }

    #[test]
    fn is_final_result_403_transient() {
        // Per §7.2 row, 403 (with or without sub-status) is transient
        // for hedging purposes.
        assert!(!is_final_result(&status(403, None)));
        assert!(!is_final_result(&status(403, Some(3))));
    }

    #[test]
    fn is_final_result_other_client_errors_final() {
        for code in [400_u16, 401, 405, 412, 413] {
            assert!(
                is_final_result(&status(code, None)),
                "expected {} to be final",
                code
            );
        }
    }

    // ───────────────────────── build_secondary_excluded_regions ─────────────────────────

    #[test]
    fn alternate_region_pin_excludes_primary() {
        let regions = [Region::EAST_US, Region::WEST_US_2];
        let excluded = build_secondary_excluded_regions(&[], &regions, 1);
        assert_eq!(excluded, vec![Region::EAST_US]);
    }

    #[test]
    fn alternate_region_pin_unions_user_excludes() {
        // user_excluded = {X}, regions = [A, B, C], secondary = B → result = {X, A, C}.
        let user_excluded = [Region::CENTRAL_US];
        let regions = [Region::EAST_US, Region::WEST_US_2, Region::NORTH_EUROPE];
        let excluded = build_secondary_excluded_regions(&user_excluded, &regions, 1);
        assert_eq!(
            excluded,
            vec![Region::CENTRAL_US, Region::EAST_US, Region::NORTH_EUROPE]
        );
    }

    #[test]
    fn alternate_region_pin_deduplicates_user_overlap() {
        // If a user-excluded region also appears in the all-regions list
        // it is not added a second time.
        let user_excluded = [Region::EAST_US];
        let regions = [Region::EAST_US, Region::WEST_US_2, Region::CENTRAL_US];
        let excluded = build_secondary_excluded_regions(&user_excluded, &regions, 1);
        assert_eq!(excluded, vec![Region::EAST_US, Region::CENTRAL_US]);
    }

    #[test]
    fn alternate_region_pin_out_of_range_returns_user_set() {
        let user_excluded = [Region::WEST_EUROPE];
        let regions = [Region::EAST_US, Region::WEST_US_2];
        let excluded = build_secondary_excluded_regions(&user_excluded, &regions, 99);
        // No `secondary_index` matches → every region is excluded on top
        // of the user set. (Documents the degenerate fall-through; the
        // hedge would be unroutable, which the caller surfaces as a
        // transient "all eligible regions excluded" result per §14.1.)
        assert_eq!(
            excluded,
            vec![Region::WEST_EUROPE, Region::EAST_US, Region::WEST_US_2]
        );
    }

    // ───────────────────────── resolve_availability_strategy ─────────────────────────

    fn empty_view<'a>(op: &'a OperationOptions) -> OperationOptionsView<'a> {
        OperationOptionsView::new(None, None, None, Some(op))
    }

    #[test]
    fn resolve_returns_driver_default_when_nothing_set() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |_| {
            Err(std::env::VarError::NotPresent)
        })
        .expect("driver default is Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_driver_default_uses_half_request_timeout_when_under_cap() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy =
            resolve_availability_strategy_with(&view, Some(Duration::from_millis(600)), |_| {
                Err(std::env::VarError::NotPresent)
            })
            .expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(300));
    }

    #[test]
    fn resolve_driver_default_caps_at_1000ms() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy =
            resolve_availability_strategy_with(&view, Some(Duration::from_secs(30)), |_| {
                Err(std::env::VarError::NotPresent)
            })
            .expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(1000));
    }

    #[test]
    fn resolve_operation_disabled_returns_none() {
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |_| {
            // Even if env says "give us a threshold", explicit Disabled wins.
            Ok("250".to_string())
        });
        assert!(strategy.is_none());
    }

    #[test]
    fn resolve_operation_hedging_overrides_env() {
        let op_strategy =
            HedgingStrategy::new(HedgeThreshold::new(Duration::from_millis(200)).unwrap());
        let op = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(op_strategy))
            .build();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |_| Ok("750".to_string()))
            .expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(200));
    }

    #[test]
    fn resolve_env_disabled_short_circuits_to_none() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |name| match name {
            "AZURE_COSMOS_HEDGING_DISABLED" => Ok("true".to_string()),
            "AZURE_COSMOS_HEDGING_THRESHOLD_MS" => Ok("250".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });
        assert!(strategy.is_none());
    }

    #[test]
    fn resolve_env_threshold_overrides_driver_default() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |name| match name {
            "AZURE_COSMOS_HEDGING_THRESHOLD_MS" => Ok("250".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        })
        .expect("Some");
        assert_eq!(strategy.threshold().get(), Duration::from_millis(250));
    }

    #[test]
    fn resolve_invalid_env_threshold_falls_back_to_default() {
        let op = OperationOptions::default();
        let view = empty_view(&op);

        let strategy = resolve_availability_strategy_with(&view, None, |name| match name {
            "AZURE_COSMOS_HEDGING_THRESHOLD_MS" => Ok("0".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        })
        .expect("Some");
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
        // Per §6.3, the alternate is pinned by excluding every *other*
        // applicable region (here, just the primary EAST_US).
        assert_eq!(upgrade.secondary_excluded_regions, vec![Region::EAST_US]);
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
        // Alternate region pinned: exclude primary + tertiary.
        assert_eq!(
            upgrade.secondary_excluded_regions,
            vec![Region::EAST_US, Region::CENTRAL_US]
        );
    }
}
