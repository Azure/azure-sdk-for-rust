// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bounding of an operation's per-attempt diagnostics under a retry storm.
//!
//! A single Cosmos operation appends one [`RequestDiagnostics`] record per
//! attempt to its [`DiagnosticsContext`](super::DiagnosticsContext). A pathological
//! retry storm (a partition hammered with `429`/`410` for the whole retry budget,
//! or a `410` fanned out across thousands of physical-partition endpoints) would
//! otherwise grow that list — and every artifact derived from it — without bound.
//!
//! This module bounds the *finalized* per-attempt list to at most
//! [`DiagnosticsOptions::max_request_diagnostics`](crate::options::DiagnosticsOptions::max_request_diagnostics)
//! records, while preserving the storm's shape and exact aggregates:
//!
//! - **Run-collapse (Phase 1):** consecutive near-identical retries — same
//!   region, endpoint, status (incl. sub-status) and execution context — are
//!   collapsed to their first and last record plus an exact per-run rollup
//!   (count, total request charge, min/max/P50 duration).
//! - **Global-bucket fallback (Phase 2):** when Phase 1 alone does not fit within
//!   the cap (e.g. a region ping-pong `A→B→A→B` where every consecutive run is
//!   length one), attempts are grouped by key regardless of order and only the
//!   `cap` largest buckets are kept for BOTH the retained records and the per-run
//!   rollup, so they stay coherent and the serialized artifact is bounded by the
//!   cap rather than by storm cardinality.
//!
//! Every drop is surfaced **explicitly** on [`CompactionInfo`]
//! (`retained_truncated`, `omitted_runs`, `omitted_request_count`) — never silent.
//!
//! Compaction runs only at finalization
//! ([`DiagnosticsContextBuilder::complete`](super::DiagnosticsContextBuilder)), never
//! mid-operation, so it never invalidates the index-based `RequestHandle`s handed
//! out while the operation is in flight. The bound is on the finalized serialized
//! artifact, not on live mid-operation memory.

use std::collections::HashMap;

use serde::Serialize;

use super::diagnostics_context::{percentile_sorted, ExecutionContext, RequestDiagnostics};
use crate::models::{CosmosStatus, RequestCharge};
use crate::options::Region;

/// Metadata describing how an operation's per-attempt diagnostics were compacted
/// under a retry storm.
///
/// Present on a [`DiagnosticsContext`](super::DiagnosticsContext) only when the
/// number of attempts exceeded the configured
/// [`max_request_diagnostics`](crate::options::DiagnosticsOptions::max_request_diagnostics)
/// cap. It records the true attempt count, how many records were retained, and a
/// per-run rollup so the storm's shape (which region/endpoint/status repeated,
/// and how many times) is preserved even though the middle of each run was
/// dropped.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CompactionInfo {
    /// True total number of attempts before compaction.
    pub original_request_count: usize,
    /// Number of per-attempt records retained after compaction.
    pub retained_request_count: usize,
    /// Number of runs whose middle records were dropped (run length > 2).
    pub collapsed_runs: usize,
    /// Total number of distinct runs detected. Equal to `runs.len()` unless the
    /// per-run rollup was itself bounded under a high-cardinality storm, in
    /// which case `runs` holds only the largest ones and `omitted_runs` the rest.
    pub total_runs: usize,
    /// `true` when the retained per-attempt list hit the configured cap and
    /// later records were dropped (the global-bucket fallback under an
    /// order-ping-pong storm with more than `cap / 2` distinct keys). The
    /// dropped attempts are still counted in `original_request_count` and the
    /// aggregate rollup; this flag makes that truncation **explicit**, never
    /// silent.
    #[serde(default, skip_serializing_if = "bool_is_false")]
    pub retained_truncated: bool,
    /// Number of runs omitted from `runs` because the per-run rollup was bounded
    /// to keep the serialized artifact size independent of storm *cardinality*
    /// (e.g. a `410` fan-out across thousands of physical-partition endpoints).
    /// `0` when every run is present.
    #[serde(default, skip_serializing_if = "usize_is_zero")]
    pub omitted_runs: usize,
    /// Total attempt count represented by the omitted runs (see `omitted_runs`).
    /// These attempts remain reflected in `original_request_count`; only their
    /// per-run rollup rows were elided.
    #[serde(default, skip_serializing_if = "usize_is_zero")]
    pub omitted_request_count: usize,
    /// Per-run rollup, in operation order (or first-seen order under the
    /// global-bucket fallback). Bounded to the largest runs by attempt count
    /// under a high-cardinality storm; see `omitted_runs` for the remainder.
    pub runs: Vec<CompactedRun>,
}

/// A single collapsed run of near-identical retries.
///
/// Groups attempts that share the same region, endpoint, status (including
/// sub-status) and execution context. The first and last attempt of each run
/// are retained in full in
/// [`DiagnosticsContext::requests`](super::DiagnosticsContext::requests); this
/// rollup carries the count and duration/charge statistics for the run so the
/// elided middle is still accounted for.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CompactedRun {
    /// Normalized region name for the run, if the attempts carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Endpoint the run targeted.
    pub endpoint: String,
    /// HTTP status and Cosmos sub-status shared by the run.
    #[serde(flatten)]
    pub status: CosmosStatus,
    /// Execution context shared by the run.
    pub execution_context: ExecutionContext,
    /// Number of attempts in the run.
    pub count: usize,
    /// Total request charge (RU) across the run.
    pub total_request_charge: RequestCharge,
    /// Minimum per-attempt duration in the run, in milliseconds.
    pub min_duration_ms: u64,
    /// Maximum per-attempt duration in the run, in milliseconds.
    pub max_duration_ms: u64,
    /// Median (P50) per-attempt duration in the run, in milliseconds.
    pub p50_duration_ms: u64,
}

/// The key that defines a "near-identical" run: same region, endpoint, status
/// (incl. sub-status) and execution context.
#[derive(Clone, PartialEq, Eq, Hash)]
struct CompactionKey {
    region: Option<Region>,
    endpoint: String,
    status: CosmosStatus,
    execution_context: ExecutionContext,
}

impl CompactionKey {
    fn of(req: &RequestDiagnostics) -> Self {
        Self {
            region: req.region().cloned(),
            endpoint: req.endpoint().to_string(),
            status: *req.status(),
            execution_context: req.execution_context(),
        }
    }
}

/// The retained per-attempt records plus the per-run rollup produced by
/// compaction.
pub(super) struct CompactionResult {
    pub(super) retained: Vec<RequestDiagnostics>,
    pub(super) runs: Vec<CompactedRun>,
    pub(super) collapsed_runs: usize,
    /// `true` when `retained` was truncated to the cap (records dropped beyond
    /// the first+last-per-run set); surfaced on `CompactionInfo::retained_truncated`.
    pub(super) retained_truncated: bool,
    /// Total number of distinct runs detected, before the per-run rollup was
    /// bounded; surfaced on `CompactionInfo::total_runs`.
    pub(super) total_runs: usize,
    /// Number of runs dropped from `runs` because the rollup was bounded to the
    /// cap; surfaced on `CompactionInfo::omitted_runs`.
    pub(super) omitted_runs: usize,
    /// Total attempt count represented by the omitted runs; surfaced on
    /// `CompactionInfo::omitted_request_count`.
    pub(super) omitted_request_count: usize,
}

/// `skip_serializing_if` helper: a `0` count is omitted from compaction output.
fn usize_is_zero(n: &usize) -> bool {
    *n == 0
}

/// `skip_serializing_if` helper: a `false` flag is omitted from compaction output.
fn bool_is_false(b: &bool) -> bool {
    !*b
}

/// Builds a [`CompactedRun`] rollup from a run/bucket of attempts.
fn compacted_run(reqs: &[&RequestDiagnostics]) -> CompactedRun {
    let count = reqs.len();
    let mut durations: Vec<u64> = reqs.iter().map(|r| r.duration_ms()).collect();
    durations.sort_unstable();
    let total_request_charge: RequestCharge = reqs.iter().map(|r| r.request_charge()).sum();
    let first = reqs[0];
    CompactedRun {
        region: first.region().map(|r| r.as_str().to_string()),
        endpoint: first.endpoint().to_string(),
        status: *first.status(),
        execution_context: first.execution_context(),
        count,
        total_request_charge,
        min_duration_ms: durations.first().copied().unwrap_or(0),
        max_duration_ms: durations.last().copied().unwrap_or(0),
        p50_duration_ms: percentile_sorted(&durations, 50),
    }
}

/// Pushes the first and (when the run has more than one attempt) last record of
/// a run into `retained`.
fn push_first_last(retained: &mut Vec<RequestDiagnostics>, run: &[&RequestDiagnostics]) {
    if let Some(first) = run.first() {
        retained.push((*first).clone());
    }
    if run.len() > 1 {
        if let Some(last) = run.last() {
            retained.push((*last).clone());
        }
    }
}

/// Bounds an operation's per-attempt diagnostics to at most `cap` records.
///
/// Phase 1 collapses runs of **consecutive** near-identical retries in order
/// (the common same-region storm). If that alone does not fit within `cap`
/// (e.g. a region ping-pong `A→B→A→B` where every consecutive run is length
/// one), Phase 2 falls back to a global key-bucket collapse that groups all
/// attempts by key regardless of order and keeps only the `cap` largest buckets
/// (by attempt count) for BOTH the retained records and the per-run rollup, so
/// the two stay coherent and the serialized artifact is bounded by `cap` rather
/// than by storm cardinality. Every drop is surfaced explicitly
/// (`retained_truncated`, `omitted_runs`, `omitted_request_count`), never silent.
pub(super) fn compact_requests(requests: Vec<RequestDiagnostics>, cap: usize) -> CompactionResult {
    let run_length = run_length_compact(&requests);
    if run_length.retained.len() <= cap {
        return run_length;
    }
    global_bucket_compact(&requests, cap)
}

/// Phase 1: order-preserving run-length collapse of consecutive equal-key runs.
fn run_length_compact(requests: &[RequestDiagnostics]) -> CompactionResult {
    let mut retained = Vec::new();
    let mut runs = Vec::new();
    let mut collapsed_runs = 0usize;

    let mut i = 0;
    while i < requests.len() {
        let key = CompactionKey::of(&requests[i]);
        let mut j = i + 1;
        while j < requests.len() && CompactionKey::of(&requests[j]) == key {
            j += 1;
        }
        let run: Vec<&RequestDiagnostics> = requests[i..j].iter().collect();
        runs.push(compacted_run(&run));
        push_first_last(&mut retained, &run);
        if run.len() > 2 {
            collapsed_runs += 1;
        }
        i = j;
    }

    let total_runs = runs.len();
    CompactionResult {
        retained,
        runs,
        collapsed_runs,
        retained_truncated: false,
        total_runs,
        omitted_runs: 0,
        omitted_request_count: 0,
    }
}

/// Phase 2: order-robust global key-bucket collapse, bounded by `cap`.
///
/// Groups every attempt by [`CompactionKey`] (first-seen order preserved for
/// output), then bounds BOTH the retained records and the per-run rollup with a
/// single ranking so they stay coherent. When there are more than `cap` distinct
/// keys (a high-cardinality `410` fan-out across physical-partition endpoints),
/// only the `cap` buckets with the highest attempt count are kept (tie-break
/// first-seen order); the rest are rolled into an explicit
/// `(omitted_runs, omitted_request_count)` marker. Both the per-run rollup and
/// the retained first+last records are drawn from that SAME kept set, so every
/// retained record has a matching run in the rollup — a downstream span emitter
/// never sees an attempt whose run was omitted. A final truncation keeps
/// `retained.len() <= cap` (marked via `retained_truncated`, never silent) when
/// the kept buckets' first+last records still exceed the cap.
fn global_bucket_compact(requests: &[RequestDiagnostics], cap: usize) -> CompactionResult {
    let mut order: Vec<CompactionKey> = Vec::new();
    let mut groups: HashMap<CompactionKey, Vec<&RequestDiagnostics>> = HashMap::new();
    for req in requests {
        let key = CompactionKey::of(req);
        match groups.get_mut(&key) {
            Some(bucket) => bucket.push(req),
            None => {
                order.push(key.clone());
                groups.insert(key, vec![req]);
            }
        }
    }

    let total_runs = order.len();
    let counts: Vec<usize> = order.iter().map(|key| groups[key].len()).collect();

    // `collapsed_runs` counts every run whose middle was dropped (length > 2),
    // whether or not it survives the rollup bound.
    let collapsed_runs = counts.iter().filter(|&&c| c > 2).count();

    // One ranking drives BOTH the kept records and the kept rollup so they stay
    // coherent. Keep the `cap` buckets with the highest attempt count (tie-break
    // first-seen index); `keep[i]` marks whether `order[i]` survives.
    let keep: Vec<bool> = if total_runs > cap {
        let mut ranked: Vec<usize> = (0..total_runs).collect();
        ranked.sort_by(|&a, &b| counts[b].cmp(&counts[a]).then(a.cmp(&b)));
        let mut kept = vec![false; total_runs];
        for &i in ranked.iter().take(cap) {
            kept[i] = true;
        }
        kept
    } else {
        vec![true; total_runs]
    };

    let mut omitted_runs = 0usize;
    let mut omitted_request_count = 0usize;
    for (i, &count) in counts.iter().enumerate() {
        if !keep[i] {
            omitted_runs += 1;
            omitted_request_count += count;
        }
    }

    // Emit the kept runs and their first+last exemplars in first-seen order.
    // Because `retained` is built only from kept buckets, its keys are a subset
    // of the run rollup keys (coherent by construction).
    let mut retained = Vec::new();
    let mut runs = Vec::new();
    for (i, key) in order.iter().enumerate() {
        if !keep[i] {
            continue;
        }
        let bucket = &groups[key];
        runs.push(compacted_run(bucket));
        push_first_last(&mut retained, bucket);
    }

    // Even limited to kept buckets, first+last (up to 2 per run) can exceed the
    // record cap. Truncate to `cap`, keeping the earliest kept buckets' records;
    // the drop is surfaced via `retained_truncated` and the omitted attempts stay
    // exact in `CompactionInfo`.
    let retained_truncated = retained.len() > cap;
    if retained_truncated {
        retained.truncate(cap);
    }

    CompactionResult {
        retained,
        runs,
        collapsed_runs,
        retained_truncated,
        total_runs,
        omitted_runs,
        omitted_request_count,
    }
}
