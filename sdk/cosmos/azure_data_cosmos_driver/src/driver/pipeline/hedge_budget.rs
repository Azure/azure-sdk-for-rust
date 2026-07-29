// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-client admission control for cross-region metadata hedge races.
//!
//! Threshold-based hedging has an uncomfortable failure mode. When a region
//! browns out, *every* operation against it eventually crosses the hedge
//! threshold, so every one of them spawns an alternate-region leg. The alternate
//! absorbs its own load, plus the failover traffic, plus one extra request per
//! hedging operation — precisely when it is least able to. Left unbounded that
//! is metastable: the alternate slows down, more operations cross the threshold,
//! more hedges spawn.
//!
//! [`HedgeBudget`] caps how many hedge races a single client may have open at
//! once. It is deliberately **non-blocking**: an operation that cannot get a
//! permit does not queue for one, it simply skips the hedge upgrade and follows
//! the ordinary sequential failover path. A hedge that waits in line has already
//! lost the latency argument it exists to win.
//!
//! # Scope
//!
//! Only the **metadata** pipeline is budgeted today; data-plane hedging is
//! admitted unconditionally. Extending the budget to the data plane is tracked
//! by <https://github.com/Azure/azure-sdk-for-rust/issues/4916>.
//!
//! The counter tracks hedge *races*, not spawned alternate legs — a permit is
//! taken when an operation enters the race and released when the race ends, so
//! an operation whose primary wins before the threshold still holds a permit for
//! the duration of its primary attempt. For metadata that is a tight bound,
//! because metadata hedge races are gated by distinct container /
//! partition-key-range cache misses rather than by request volume. Applying the
//! same race-scoped accounting to the data plane would be wrong: point-read
//! concurrency scales with application throughput, so a race-scoped cap would
//! refuse hedges to clients that were never going to spawn a leg at all.
//! Bounding data-plane amplification needs leg-scoped accounting inside the
//! race, which is a larger change to the hedge state machine: a refused
//! secondary has no representation in [`HedgedRaceResult`], whose
//! `BothTransient` variant means "this race consumed two regions" and would
//! skip a healthy region if returned after only the primary was tried.
//!
//! [`HedgedRaceResult`]: super::operation_pipeline::HedgedRaceResult

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::diagnostics::PipelineType;

/// Environment override for the metadata hedge concurrency limit.
const METADATA_LIMIT_ENV: &str = "AZURE_COSMOS_MAX_CONCURRENT_METADATA_HEDGES";

/// Default ceiling on concurrent metadata hedge races per client.
///
/// Sized as a guardrail rather than a throttle. Metadata hedge races are bounded
/// by distinct container / partition-key-range cache misses, so a client with
/// more than this many *simultaneous* metadata refreshes in flight is already
/// pathological — and that is exactly the shape a full-region brownout takes.
const DEFAULT_METADATA_LIMIT: usize = 32;

/// Per-client ceiling on concurrent cross-region metadata hedge races.
///
/// See the [module docs](self) for why the budget is non-blocking, why it counts
/// races rather than legs, and why the data plane is exempt.
#[derive(Debug)]
pub(crate) struct HedgeBudget {
    metadata: HedgeSlots,
}

impl HedgeBudget {
    /// Builds a budget from the process environment, falling back to
    /// [`DEFAULT_METADATA_LIMIT`].
    ///
    /// `0` disables metadata hedging outright (no permit can ever be issued).
    /// An unparseable value is ignored in favor of the default, so a typo
    /// degrades to documented behavior rather than to silently no hedging.
    pub(crate) fn from_env() -> Self {
        Self::new(limit_from_env(METADATA_LIMIT_ENV, DEFAULT_METADATA_LIMIT))
    }

    /// Builds a budget with an explicit metadata limit.
    pub(crate) fn new(metadata_limit: usize) -> Self {
        Self {
            metadata: HedgeSlots::new(metadata_limit),
        }
    }

    /// Admits one hedge race, or returns `None` when the metadata budget is
    /// exhausted.
    ///
    /// Data-plane races are always admitted — see the [module docs](self).
    ///
    /// The returned permit releases its slot on drop, so a race that ends early
    /// (primary wins pre-threshold, deadline fires) frees its slot immediately.
    pub(crate) fn try_admit(&self, pipeline_type: PipelineType) -> Option<HedgePermit<'_>> {
        if !pipeline_type.is_metadata() {
            return Some(HedgePermit { slots: None });
        }
        self.metadata.try_acquire()
    }

    /// Overrides the metadata limit so tests can drive the exhausted branch
    /// deterministically, without racing the process environment.
    ///
    /// Production code never calls this; the limit is fixed at construction.
    #[cfg(any(test, feature = "__internal_in_memory_emulator"))]
    pub(crate) fn set_metadata_limit_for_tests(&self, limit: usize) {
        self.metadata.limit.store(limit, Ordering::Relaxed);
    }
}

impl Default for HedgeBudget {
    fn default() -> Self {
        Self::new(DEFAULT_METADATA_LIMIT)
    }
}

/// Reads a limit from `name`, falling back to `default` when the variable is
/// unset, empty, or not a valid `usize`.
fn limit_from_env(name: &str, default: usize) -> usize {
    let Ok(raw) = std::env::var(name) else {
        return default;
    };
    match raw.trim().parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => {
            tracing::warn!(
                env_var = name,
                value = %raw,
                default,
                "Ignoring unparseable metadata hedge concurrency limit; using the default",
            );
            default
        }
    }
}

/// A counting permit pool with `try`-only acquisition.
///
/// Implemented over an [`AtomicUsize`] rather than a runtime semaphore so the
/// budget stays executor-agnostic, and because a blocking acquire is never the
/// right answer here.
#[derive(Debug)]
struct HedgeSlots {
    in_flight: AtomicUsize,
    /// Fixed at construction in production; atomic only so tests can rewrite it
    /// through a shared reference. See
    /// [`HedgeBudget::set_metadata_limit_for_tests`].
    limit: AtomicUsize,
}

impl HedgeSlots {
    fn new(limit: usize) -> Self {
        Self {
            in_flight: AtomicUsize::new(0),
            limit: AtomicUsize::new(limit),
        }
    }

    fn try_acquire(&self) -> Option<HedgePermit<'_>> {
        let limit = self.limit.load(Ordering::Relaxed);
        let mut current = self.in_flight.load(Ordering::Relaxed);
        loop {
            if current >= limit {
                return None;
            }
            match self.in_flight.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Some(HedgePermit { slots: Some(self) }),
                Err(observed) => current = observed,
            }
        }
    }
}

/// RAII handle for one admitted hedge race.
///
/// Held by the pipeline for as long as the race is open; dropping it returns the
/// slot to the pool. A permit for an unbudgeted pipeline holds no slot and its
/// drop is a no-op.
#[derive(Debug)]
pub(crate) struct HedgePermit<'a> {
    slots: Option<&'a HedgeSlots>,
}

impl Drop for HedgePermit<'_> {
    fn drop(&mut self) {
        if let Some(slots) = self.slots {
            slots.in_flight.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permits_are_issued_up_to_the_limit() {
        let budget = HedgeBudget::new(2);

        let first = budget.try_admit(PipelineType::Metadata);
        let second = budget.try_admit(PipelineType::Metadata);
        assert!(first.is_some());
        assert!(second.is_some());
        assert!(
            budget.try_admit(PipelineType::Metadata).is_none(),
            "a third metadata race must be refused rather than queued"
        );
    }

    #[test]
    fn dropping_a_permit_returns_the_slot() {
        let budget = HedgeBudget::new(1);

        let permit = budget.try_admit(PipelineType::Metadata);
        assert!(permit.is_some());
        assert!(budget.try_admit(PipelineType::Metadata).is_none());

        drop(permit);
        assert!(
            budget.try_admit(PipelineType::Metadata).is_some(),
            "a finished race must free its slot immediately"
        );
    }

    #[test]
    fn data_plane_is_not_budgeted() {
        let budget = HedgeBudget::new(0);

        assert!(
            budget.try_admit(PipelineType::Metadata).is_none(),
            "a zero metadata limit refuses every metadata race"
        );
        let permits: Vec<_> = (0..1000)
            .map(|_| {
                budget
                    .try_admit(PipelineType::DataPlane)
                    .expect("data-plane hedging must not be gated by the metadata budget")
            })
            .collect();
        assert_eq!(permits.len(), 1000);
    }

    #[test]
    fn slots_are_reusable_across_many_rounds() {
        let budget = HedgeBudget::new(1);
        for _ in 0..64 {
            let permit = budget
                .try_admit(PipelineType::Metadata)
                .expect("slot must be free at the start of each round");
            assert!(budget.try_admit(PipelineType::Metadata).is_none());
            drop(permit);
        }
    }

    #[test]
    fn unset_env_var_falls_back_to_the_default() {
        assert_eq!(
            limit_from_env("AZURE_COSMOS_HEDGE_LIMIT_NEVER_SET_IN_TESTS", 7),
            7
        );
    }

    #[test]
    fn default_budget_admits_the_documented_number_of_races() {
        let budget = HedgeBudget::default();
        let permits: Vec<_> = (0..DEFAULT_METADATA_LIMIT)
            .map(|_| {
                budget
                    .try_admit(PipelineType::Metadata)
                    .expect("default budget must admit up to its limit")
            })
            .collect();
        assert!(budget.try_admit(PipelineType::Metadata).is_none());
        drop(permits);
        assert!(budget.try_admit(PipelineType::Metadata).is_some());
    }
}
