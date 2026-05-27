// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostic types for cross-region hedging executions.
//!
//! When a hedging strategy is resolved and active for an operation, the
//! winning response carries a populated [`HedgeDiagnostics`]; otherwise
//! it is `None`.
//!
//! These types are pure data — constructed by `execute_hedged()` (in
//! `operation_pipeline.rs`) and surfaced through
//! `DiagnosticsContext::hedge_diagnostics`. They contain no behavior and
//! have no dependencies on the pipeline internals, which keeps them cheap
//! to construct on the happy path and trivial to assert against in tests.

use crate::options::{HedgeThreshold, Region};

/// Snapshot of the hedging strategy configuration that was active for an
/// operation.
///
/// Captured once at the start of `execute_hedged()` and attached to the
/// winning response's [`HedgeDiagnostics::strategy_config`]. Currently the
/// only field is the threshold; future strategy fields (e.g. retry caps,
/// fan-out limits) will be added here.
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

/// Final outcome classification of a hedging race.
///
/// Disambiguates the six terminal states `execute_hedged` can reach so
/// downstream observability consumers can compute accurate metrics —
/// most importantly hedge win-rate, which **must not** count terminal-
/// error states as alternate wins.
///
/// Always consult this field instead of inferring intent from
/// [`HedgeDiagnostics::was_hedge`] alone: `was_hedge` is `true` only when
/// the alternate produced the final response, but several non-`AlternateWon`
/// terminal states still record `regions_contacted = [primary, alternate]`
/// and `total_requests_launched = 2`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HedgeTerminalState {
    /// The primary returned a final response before the threshold timer
    /// fired; no alternate was ever spawned (zero-overhead happy path).
    PrimaryWonPreThreshold,

    /// The deadline fired before the threshold elapsed; no alternate was
    /// spawned. The primary was harvested within `HARVEST_WINDOW` for
    /// diagnostics but the operation surfaced a cancellation error.
    DeadlineExceededPreThreshold,

    /// The primary returned a final response after the threshold elapsed,
    /// winning the race against the spawned alternate. The alternate's
    /// in-flight request was structurally cancelled.
    PrimaryWonAfterHedge,

    /// The alternate returned a final response and won the race against
    /// the still-pending primary.
    AlternateWon,

    /// Both legs returned retriable failures (5xx / 429 / 408 / 410 /
    /// 404-1002 / transport error / deadline). The race produced no winner;
    /// the operation surfaced either a cancellation error (if the deadline
    /// drove termination) or a synthetic both-transient error.
    #[non_exhaustive]
    BothTransient {
        /// `true` iff the operation-level deadline had elapsed when the
        /// race concluded.
        deadline_elapsed: bool,
    },

    /// One leg returned a retriable failure and the operation-level
    /// deadline fired while waiting for the partner leg to complete.
    /// The operation surfaced a cancellation error.
    CancelledAwaitingPartner,
}

/// Diagnostic information about a hedging execution, attached to the
/// winning response when a hedging strategy was active for the operation.
///
/// The [`terminal_state`](Self::terminal_state) field is the authoritative
/// classification of how the race ended; use it (not
/// [`was_hedge`](Self::was_hedge) alone) when computing observability
/// metrics, especially hedge win-rate.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct HedgeDiagnostics {
    /// The hedging strategy configuration that was active.
    pub strategy_config: HedgingStrategyConfig,

    /// Regions that had requests launched (up to and including the winner
    /// or, for terminal-error states, both contacted legs).
    ///
    /// With the single-alternate model this is either `vec![primary]`
    /// (primary won before the threshold timer fired, or the deadline
    /// fired pre-threshold) or `vec![primary, alternate]` (the alternate
    /// hedge was spawned, regardless of outcome).
    pub regions_contacted: Vec<Region>,

    /// The region whose response was returned to the caller.
    ///
    /// For terminal-error states (`BothTransient`, `CancelledAwaitingPartner`,
    /// `DeadlineExceededPreThreshold`) no leg produced a final response,
    /// and this field holds the primary region as a sentinel — consult
    /// [`terminal_state`](Self::terminal_state) before interpreting it.
    pub response_region: Region,

    /// How many requests were launched (including the primary).
    ///
    /// Either `1` (no alternate spawned) or `2` (alternate spawned,
    /// regardless of outcome).
    pub total_requests_launched: usize,

    /// Whether the alternate hedge produced the final response that was
    /// returned to the caller.
    ///
    /// `true` only when [`terminal_state`](Self::terminal_state) is
    /// [`HedgeTerminalState::AlternateWon`]; `false` for every other
    /// terminal state, including terminal-error states where no leg won.
    /// This is the field hedge win-rate metrics should aggregate over.
    pub was_hedge: bool,

    /// Authoritative classification of the race outcome.
    ///
    /// Always consult this field when computing observability metrics
    /// rather than inferring intent from the other fields.
    pub terminal_state: HedgeTerminalState,
}

impl HedgeDiagnostics {
    /// Sentinel `Region` value used when a hedge leg ran against a
    /// global-endpoint account whose routed endpoint surfaces no named
    /// region. Downstream consumers can use this to distinguish "no
    /// named region" from a real Azure region.
    pub const UNKNOWN_REGION_SENTINEL: &'static str = "(unknown)";

    /// Constructs a [`HedgeDiagnostics`] for the *"primary won before the
    /// threshold fired"* case — the zero-overhead happy path.
    ///
    /// Terminal state: [`HedgeTerminalState::PrimaryWonPreThreshold`].
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
            terminal_state: HedgeTerminalState::PrimaryWonPreThreshold,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"deadline fired pre-
    /// threshold; primary harvested but no final response"* case.
    ///
    /// Terminal state: [`HedgeTerminalState::DeadlineExceededPreThreshold`].
    /// `was_hedge = false`. `response_region` is set to the primary region
    /// as a sentinel — the operation surfaces a cancellation error so no
    /// response is actually returned to the caller.
    pub fn primary_only_deadline_exceeded(
        strategy_config: HedgingStrategyConfig,
        primary_region: Region,
    ) -> Self {
        Self {
            strategy_config,
            regions_contacted: vec![primary_region.clone()],
            response_region: primary_region,
            total_requests_launched: 1,
            was_hedge: false,
            terminal_state: HedgeTerminalState::DeadlineExceededPreThreshold,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"primary spawned an
    /// alternate, primary still won"* case.
    ///
    /// Terminal state: [`HedgeTerminalState::PrimaryWonAfterHedge`].
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
            terminal_state: HedgeTerminalState::PrimaryWonAfterHedge,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"alternate hedge won the
    /// race"* case.
    ///
    /// Terminal state: [`HedgeTerminalState::AlternateWon`]. This is the
    /// **only** terminal state for which `was_hedge = true`. Hedge win-rate
    /// metrics should aggregate over this variant exclusively.
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
            terminal_state: HedgeTerminalState::AlternateWon,
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"both legs returned a
    /// retriable failure"* case.
    ///
    /// Terminal state: [`HedgeTerminalState::BothTransient`] with
    /// `deadline_elapsed` carried through. `was_hedge = false` — no leg
    /// produced a final response. `response_region` is the primary region
    /// as a sentinel; the operation surfaces either a cancellation error
    /// (when `deadline_elapsed = true`) or a synthetic both-transient error.
    pub fn both_transient(
        strategy_config: HedgingStrategyConfig,
        primary_region: Region,
        alternate_region: Region,
        deadline_elapsed: bool,
    ) -> Self {
        Self {
            strategy_config,
            regions_contacted: vec![primary_region.clone(), alternate_region],
            response_region: primary_region,
            total_requests_launched: 2,
            was_hedge: false,
            terminal_state: HedgeTerminalState::BothTransient { deadline_elapsed },
        }
    }

    /// Constructs a [`HedgeDiagnostics`] for the *"deadline fired while
    /// awaiting the partner leg after the first leg returned a retriable
    /// failure"* case.
    ///
    /// Terminal state: [`HedgeTerminalState::CancelledAwaitingPartner`].
    /// `was_hedge = false` — no leg produced a final response.
    /// `response_region` is the primary region as a sentinel; the operation
    /// surfaces a cancellation error.
    pub fn cancelled_awaiting_partner(
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
            terminal_state: HedgeTerminalState::CancelledAwaitingPartner,
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
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::PrimaryWonPreThreshold
        );
    }

    #[test]
    fn primary_only_deadline_exceeded_constructor() {
        let diag = HedgeDiagnostics::primary_only_deadline_exceeded(config(), Region::EAST_US);
        assert_eq!(diag.regions_contacted, vec![Region::EAST_US]);
        assert_eq!(diag.total_requests_launched, 1);
        assert!(
            !diag.was_hedge,
            "deadline-exceeded pre-threshold must not record was_hedge=true"
        );
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::DeadlineExceededPreThreshold
        );
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
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::PrimaryWonAfterHedge
        );
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
        assert_eq!(diag.terminal_state, HedgeTerminalState::AlternateWon);
    }

    #[test]
    fn both_transient_constructor_with_deadline_elapsed() {
        let diag =
            HedgeDiagnostics::both_transient(config(), Region::EAST_US, Region::WEST_US_2, true);
        assert_eq!(
            diag.regions_contacted,
            vec![Region::EAST_US, Region::WEST_US_2]
        );
        assert_eq!(
            diag.response_region,
            Region::EAST_US,
            "sentinel response_region is primary for terminal-error states"
        );
        assert_eq!(diag.total_requests_launched, 2);
        assert!(
            !diag.was_hedge,
            "both-transient must not record was_hedge=true — no leg won"
        );
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::BothTransient {
                deadline_elapsed: true
            }
        );
    }

    #[test]
    fn both_transient_constructor_without_deadline_elapsed() {
        let diag =
            HedgeDiagnostics::both_transient(config(), Region::EAST_US, Region::WEST_US_2, false);
        assert!(!diag.was_hedge);
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::BothTransient {
                deadline_elapsed: false
            }
        );
    }

    #[test]
    fn cancelled_awaiting_partner_constructor() {
        let diag = HedgeDiagnostics::cancelled_awaiting_partner(
            config(),
            Region::EAST_US,
            Region::WEST_US_2,
        );
        assert_eq!(
            diag.regions_contacted,
            vec![Region::EAST_US, Region::WEST_US_2]
        );
        assert_eq!(diag.response_region, Region::EAST_US);
        assert_eq!(diag.total_requests_launched, 2);
        assert!(
            !diag.was_hedge,
            "cancelled-awaiting-partner must not record was_hedge=true — no leg won"
        );
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::CancelledAwaitingPartner
        );
    }

    #[test]
    fn only_alternate_won_records_was_hedge_true() {
        // Invariant: was_hedge=true iff terminal_state == AlternateWon.
        // Hedge win-rate metrics depend on this.
        let east = Region::EAST_US;
        let west = Region::WEST_US_2;
        assert!(HedgeDiagnostics::hedge_won(config(), east.clone(), west.clone()).was_hedge);
        assert!(!HedgeDiagnostics::primary_only(config(), east.clone()).was_hedge);
        assert!(
            !HedgeDiagnostics::primary_only_deadline_exceeded(config(), east.clone()).was_hedge
        );
        assert!(
            !HedgeDiagnostics::primary_won_after_hedge(config(), east.clone(), west.clone())
                .was_hedge
        );
        assert!(
            !HedgeDiagnostics::both_transient(config(), east.clone(), west.clone(), true).was_hedge
        );
        assert!(
            !HedgeDiagnostics::both_transient(config(), east.clone(), west.clone(), false)
                .was_hedge
        );
        assert!(!HedgeDiagnostics::cancelled_awaiting_partner(config(), east, west).was_hedge);
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

    /// Regression test for the global-endpoint attachment contract.
    ///
    /// `HedgeDiagnostics` is `Some(_)` iff `execute_hedged()` ran, even
    /// for global-endpoint accounts whose routed endpoint surfaces no
    /// named region. The `execute_hedged` body substitutes a
    /// `Region::new("(unknown)")` sentinel at the diagnostics-construction
    /// sites in that case (the PPCB recording paths still see `None` so
    /// counters are not collapsed under one sentinel key). This test
    /// pins the sentinel string so downstream consumers can rely on it
    /// to distinguish "no named region" from a real Azure region.
    #[test]
    fn unknown_region_sentinel_constructs_diagnostics_for_global_endpoint() {
        let unknown = Region::new("(unknown)");
        let diag = HedgeDiagnostics::primary_only(config(), unknown.clone());
        assert_eq!(diag.response_region, unknown);
        assert_eq!(diag.regions_contacted, vec![unknown.clone()]);
        assert!(!diag.was_hedge);
        assert_eq!(
            diag.terminal_state,
            HedgeTerminalState::PrimaryWonPreThreshold
        );
        // Sentinel must not collide with any real region constant.
        assert_ne!(unknown, Region::EAST_US);
        assert_ne!(unknown, Region::WEST_US_2);
    }
}
