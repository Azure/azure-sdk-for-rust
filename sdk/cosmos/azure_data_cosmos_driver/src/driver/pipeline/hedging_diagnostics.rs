// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostic types for cross-region hedging executions.
//!
//! See [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md) §10.1 for the
//! full attachment contract. Briefly: when a hedging strategy was
//! **resolved and active** for an operation (i.e. `should_hedge()` returned
//! `true` and `execute_hedged()` was entered), the winning response carries
//! a populated [`HedgeDiagnostics`]; otherwise it is `None`.
//!
//! These types are pure data — they are constructed by `execute_hedged()`
//! (in `operation_pipeline.rs`) and surfaced through
//! `DiagnosticsContext::hedge_diagnostics` (§10.2). They contain no behavior
//! and have no dependencies on the pipeline internals, which keeps them
//! cheap to construct on the happy path and trivial to assert against in
//! tests.

use crate::options::{HedgeThreshold, Region};

/// Snapshot of the hedging strategy configuration that was active for an
/// operation.
///
/// Captured once at the start of `execute_hedged()` and attached to the
/// winning response's [`HedgeDiagnostics::strategy_config`]. Currently the
/// only field is the threshold (per spec §4.1); future strategy fields
/// (e.g. retry caps, fan-out limits) will be added here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct HedgingStrategyConfig {
    /// The configured threshold before the alternate-region hedge fires.
    pub threshold: HedgeThreshold,
}

impl HedgingStrategyConfig {
    /// Creates a new strategy config snapshot from the given threshold.
    pub const fn new(threshold: HedgeThreshold) -> Self {
        Self { threshold }
    }
}

/// Diagnostic information about a hedging execution, attached to the
/// winning response when a hedging strategy was active for the operation.
///
/// See [`docs/HEDGING_SPEC.md`](../../../docs/HEDGING_SPEC.md) §10.1 for the
/// full field semantics, including the "primary wins before the first
/// hedge fires" attachment contract that lets callers distinguish:
///
/// - *"hedging was active and the primary won before the threshold elapsed"*
///   (use [`HedgeDiagnostics::primary_only`]) from
/// - *"hedging was active and the alternate region won the race"*
///   (use [`HedgeDiagnostics::hedge_won`]) from
/// - *"hedging was not selected for this operation"*
///   (no attachment — `DiagnosticsContext::hedge_diagnostics` is `None`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct HedgeDiagnostics {
    /// The hedging strategy configuration that was active.
    pub strategy_config: HedgingStrategyConfig,

    /// Regions that had requests launched (up to and including the winner).
    ///
    /// With the single-alternate model (§6) this is either
    /// `vec![primary]` (primary won before the threshold timer fired)
    /// or `vec![primary, alternate]` (the alternate hedge was spawned).
    pub regions_contacted: Vec<Region>,

    /// The target region of the winning response.
    pub response_region: Region,

    /// How many requests were launched (including the primary).
    ///
    /// Either `1` (no hedge fired) or `2` (alternate spawned).
    pub total_requests_launched: usize,

    /// Whether the alternate hedge won the race.
    ///
    /// `false` when the primary returned a final result (either before the
    /// threshold elapsed, or after racing the alternate).
    pub was_hedge: bool,
}

impl HedgeDiagnostics {
    /// Constructs a [`HedgeDiagnostics`] for the *"primary won before the
    /// threshold fired"* case (spec §10.1 attachment contract).
    ///
    /// `regions_contacted = vec![primary_region]`,
    /// `response_region = primary_region`,
    /// `total_requests_launched = 1`,
    /// `was_hedge = false`.
    pub fn primary_only(strategy_config: HedgingStrategyConfig, primary_region: Region) -> Self {
        Self {
            strategy_config,
            regions_contacted: vec![primary_region.clone()],
            response_region: primary_region,
            total_requests_launched: 1,
            was_hedge: false,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"primary spawned an
    /// alternate, primary still won"* case.
    ///
    /// `regions_contacted = vec![primary_region, alternate_region]`,
    /// `response_region = primary_region`,
    /// `total_requests_launched = 2`,
    /// `was_hedge = false`.
    pub fn primary_won_after_hedge(
        strategy_config: HedgingStrategyConfig,
        primary_region: Region,
        alternate_region: Region,
    ) -> Self {
        Self {
            strategy_config,
            regions_contacted: vec![primary_region.clone(), alternate_region],
            response_region: primary_region,
            total_requests_launched: 2,
            was_hedge: false,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"alternate hedge won the
    /// race"* case (spec §10.1).
    ///
    /// `regions_contacted = vec![primary_region, alternate_region]`,
    /// `response_region = alternate_region`,
    /// `total_requests_launched = 2`,
    /// `was_hedge = true`.
    pub fn hedge_won(
        strategy_config: HedgingStrategyConfig,
        primary_region: Region,
        alternate_region: Region,
    ) -> Self {
        Self {
            strategy_config,
            regions_contacted: vec![primary_region, alternate_region.clone()],
            response_region: alternate_region,
            total_requests_launched: 2,
            was_hedge: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn threshold() -> HedgeThreshold {
        HedgeThreshold::new(Duration::from_millis(500)).expect("500ms is non-zero")
    }

    fn config() -> HedgingStrategyConfig {
        HedgingStrategyConfig::new(threshold())
    }

    #[test]
    fn strategy_config_exposes_threshold() {
        let cfg = config();
        assert_eq!(cfg.threshold, threshold());
    }

    #[test]
    fn primary_only_constructor() {
        let diag = HedgeDiagnostics::primary_only(config(), Region::EAST_US);
        assert_eq!(diag.strategy_config, config());
        assert_eq!(diag.regions_contacted, vec![Region::EAST_US]);
        assert_eq!(diag.response_region, Region::EAST_US);
        assert_eq!(diag.total_requests_launched, 1);
        assert!(!diag.was_hedge);
    }

    #[test]
    fn primary_won_after_hedge_constructor() {
        let diag =
            HedgeDiagnostics::primary_won_after_hedge(config(), Region::EAST_US, Region::WEST_US_2);
        assert_eq!(
            diag.regions_contacted,
            vec![Region::EAST_US, Region::WEST_US_2]
        );
        assert_eq!(diag.response_region, Region::EAST_US);
        assert_eq!(diag.total_requests_launched, 2);
        assert!(!diag.was_hedge);
    }

    #[test]
    fn hedge_won_constructor() {
        let diag = HedgeDiagnostics::hedge_won(config(), Region::EAST_US, Region::WEST_US_2);
        assert_eq!(
            diag.regions_contacted,
            vec![Region::EAST_US, Region::WEST_US_2]
        );
        assert_eq!(diag.response_region, Region::WEST_US_2);
        assert_eq!(diag.total_requests_launched, 2);
        assert!(diag.was_hedge);
    }

    #[test]
    fn debug_clone_round_trip() {
        let diag = HedgeDiagnostics::hedge_won(config(), Region::EAST_US, Region::WEST_US_2);
        let cloned = diag.clone();
        assert_eq!(diag, cloned);
        // Debug must not panic and must include the winner region.
        let dbg = format!("{:?}", diag);
        assert!(
            dbg.contains("westus2"),
            "Debug output missing winner region: {dbg}"
        );
    }
}
