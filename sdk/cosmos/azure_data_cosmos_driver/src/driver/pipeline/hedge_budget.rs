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
//! once, from [`HedgingOptions::max_concurrent_metadata_attempts`]. It is
//! deliberately **non-blocking**: an operation that cannot get a permit does not
//! queue for one, it simply skips the hedge upgrade and follows the ordinary
//! sequential failover path. A hedge that waits in line has already lost the
//! latency argument it exists to win.
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
//! [`HedgingOptions::max_concurrent_metadata_attempts`]:
//!     crate::options::HedgingOptions::max_concurrent_metadata_attempts

use async_lock::{Semaphore, SemaphoreGuard};

use crate::diagnostics::PipelineKind;
use crate::options::HedgingOptions;

/// Per-client ceiling on concurrent cross-region metadata hedge races.
///
/// See the [module docs](self) for why the budget is non-blocking, why it counts
/// races rather than legs, and why the data plane is exempt.
#[derive(Debug)]
pub(crate) struct HedgeBudget {
    metadata: Semaphore,
}

impl HedgeBudget {
    /// Builds a budget from the driver's [`HedgingOptions`].
    pub(crate) fn new(options: &HedgingOptions) -> Self {
        Self::with_metadata_limit(options.max_concurrent_metadata_attempts())
    }

    /// Builds a budget with an explicit metadata limit.
    ///
    /// `0` disables metadata hedging outright: no permit can ever be issued.
    pub(crate) fn with_metadata_limit(metadata_limit: usize) -> Self {
        Self {
            metadata: Semaphore::new(metadata_limit),
        }
    }

    /// Admits one hedge race, or returns `None` when the metadata budget is
    /// exhausted.
    ///
    /// Data-plane races are always admitted — see the [module docs](self).
    ///
    /// Never blocks and never retries: acquisition is a single `try_acquire`, so
    /// a caller under contention is refused immediately rather than spinning for
    /// a slot it did not want to wait for in the first place.
    ///
    /// The returned permit releases its slot on drop, so a race that ends early
    /// (primary wins pre-threshold, deadline fires) frees its slot immediately.
    pub(crate) fn try_admit(&self, pipeline_type: PipelineKind) -> Option<HedgePermit<'_>> {
        if !pipeline_type.is_metadata() {
            return Some(HedgePermit::Unbudgeted);
        }
        self.metadata.try_acquire().map(HedgePermit::Admitted)
    }
}

impl Default for HedgeBudget {
    fn default() -> Self {
        Self::new(&HedgingOptions::default())
    }
}

/// Handle for one hedge race that has been allowed to proceed.
///
/// Held by the pipeline for as long as the race is open.
#[derive(Debug)]
pub(crate) enum HedgePermit<'a> {
    /// The race is on an unbudgeted pipeline and consumed no slot.
    Unbudgeted,
    /// The race holds a metadata slot.
    ///
    /// The guard is never read: it exists so that dropping the permit returns
    /// the slot to the semaphore.
    Admitted(#[allow(dead_code)] SemaphoreGuard<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS;

    #[test]
    fn permits_are_issued_up_to_the_limit() {
        let budget = HedgeBudget::with_metadata_limit(2);

        let first = budget.try_admit(PipelineKind::Metadata);
        let second = budget.try_admit(PipelineKind::Metadata);
        assert!(first.is_some());
        assert!(second.is_some());
        assert!(
            budget.try_admit(PipelineKind::Metadata).is_none(),
            "a third metadata race must be refused rather than queued"
        );
    }

    #[test]
    fn dropping_a_permit_returns_the_slot() {
        let budget = HedgeBudget::with_metadata_limit(1);

        let permit = budget.try_admit(PipelineKind::Metadata);
        assert!(permit.is_some());
        assert!(budget.try_admit(PipelineKind::Metadata).is_none());

        drop(permit);
        assert!(
            budget.try_admit(PipelineKind::Metadata).is_some(),
            "a finished race must free its slot immediately"
        );
    }

    #[test]
    fn data_plane_is_not_budgeted() {
        let budget = HedgeBudget::with_metadata_limit(0);

        assert!(
            budget.try_admit(PipelineKind::Metadata).is_none(),
            "a zero metadata limit refuses every metadata race"
        );
        let permits: Vec<_> = (0..1000)
            .map(|_| {
                budget
                    .try_admit(PipelineKind::DataPlane)
                    .expect("data-plane hedging must not be gated by the metadata budget")
            })
            .collect();
        assert_eq!(permits.len(), 1000);
    }

    #[test]
    fn slots_are_reusable_across_many_rounds() {
        let budget = HedgeBudget::with_metadata_limit(1);
        for _ in 0..64 {
            let permit = budget
                .try_admit(PipelineKind::Metadata)
                .expect("slot must be free at the start of each round");
            assert!(budget.try_admit(PipelineKind::Metadata).is_none());
            drop(permit);
        }
    }

    #[test]
    fn budget_is_built_from_the_driver_option() {
        let options = HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(3)
            .build();
        let budget = HedgeBudget::new(&options);

        let permits: Vec<_> = (0..3)
            .map(|_| {
                budget
                    .try_admit(PipelineKind::Metadata)
                    .expect("the configured limit must be admitted in full")
            })
            .collect();
        assert!(budget.try_admit(PipelineKind::Metadata).is_none());
        drop(permits);
    }

    #[test]
    fn default_budget_admits_the_documented_number_of_races() {
        let budget = HedgeBudget::default();
        let permits: Vec<_> = (0..DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS)
            .map(|_| {
                budget
                    .try_admit(PipelineKind::Metadata)
                    .expect("default budget must admit up to its limit")
            })
            .collect();
        assert!(budget.try_admit(PipelineKind::Metadata).is_none());
        drop(permits);
        assert!(budget.try_admit(PipelineKind::Metadata).is_some());
    }
}
