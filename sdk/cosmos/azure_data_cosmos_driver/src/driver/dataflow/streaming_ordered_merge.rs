// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore unemitted unfetched rescan

//! Streaming cross-partition `ORDER BY` k-way merge node.
//!
//! [`StreamingOrderedMerge`] polls one leaf [`Request`] per active EPK range,
//! buffers one locally-sorted row per range, and repeatedly emits the
//! globally smallest buffered row (per [`compare_key_tuples`]). With the
//! documented fan-out cap (`FEED_OPERATIONS_REQS.md` §3, default 100), a
//! linear per-pop scan over active children is simpler than a real heap and
//! not a meaningful cost next to per-page network I/O.
//!
//! # Resume model
//!
//! Unlike [`super::SequentialDrain`]'s sparse cursor, global ordering means
//! *any* range may still have unemitted rows, so every active range is
//! snapshotted explicitly (see
//! [`super::snapshot::PipelineNodeState::StreamingOrderedMerge`]). A range
//! resumes at a **clean page boundary** (buffer empty, topology unchanged,
//! plain query) by replaying its saved backend continuation, or at a
//! **value boundary** (mid-page, topology changed, or already
//! resume-filtered) by reissuing the rewritten query with a `_rid`-aware
//! resume filter (see [`super::order_by::ResumeFilter`]) and discarding
//! already-emitted rows client-side.
//!
//! A backend continuation is opaque and bound to the exact query text that
//! minted it, so one captured for a resume-filtered query is never
//! persisted or reused (see [`ChildQueryShape`]); [`handle_split`] always
//! re-derives split replacements from the last-emitted boundary rather than
//! trusting a forwarded continuation. A complex (array/object) boundary can
//! only resume via positional rescan and therefore cannot cross a split.

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::value::RawValue;

use crate::models::{CosmosOperation, FeedRange, MaxItemCountHint};

use super::order_by::{
    compare_key_tuples, compare_rids, OrderByColumn, OrderByItem, OrderByResumeValue, ResumeFilter,
};
use super::query_plan::SortOrder;
use super::query_response::{self, PageAggregator};
use super::snapshot::{OrderByRangeToken, ValueBoundary};
use super::{
    intersect_feed_ranges, PageResult, PartitionRoutingRefresh, PipelineContext, PipelineNode,
    PipelineNodeState, Request, RequestTarget, ResolvedRange,
};

/// Default emitted-page size when no `max_item_count` hint is set
/// (matches Cosmos's default; independent of backend per-child page size).
const DEFAULT_MAX_ITEM_COUNT: usize = 100;

/// Maximum consecutive split retries per child before giving up (mirrors
/// `SequentialDrain`/`UnorderedMerge`).
const MAX_SPLIT_RETRIES: usize = 10;

/// A range's last-emitted key tuple (bounded resume-value form) and RID.
#[derive(Clone)]
struct LastEmitted {
    resume_values: Vec<OrderByResumeValue>,
    rid: String,
}

/// What a child's next fetched page must discard before its rows become
/// visible to the merge, right after resuming a value boundary.
enum PendingDiscard {
    /// Nothing to discard — a fresh start or a plain continuation resume.
    None,
    /// Discard every row at or before the resume boundary: keys before the
    /// boundary (per [`compare_key_tuples`]), or a full-key tie whose `_rid`
    /// is at or before `last_rid` in the first column's direction (numeric
    /// document-ordinal order via [`compare_rids`], matching the backend).
    /// A per-row predicate, so it stays correct across pages and split
    /// sub-ranges.
    ResumeBoundary {
        boundary_keys: Vec<OrderByItem>,
        last_rid: String,
        directions: Vec<SortOrder>,
    },
    /// Discard exactly `remaining` rows positionally (complex boundary, no
    /// filter). Only valid while topology is unchanged.
    Positional { remaining: u64 },
}

impl PendingDiscard {
    fn apply(&mut self, rows: &mut VecDeque<query_response::EnvelopeRow>) {
        match self {
            PendingDiscard::None => {}
            PendingDiscard::Positional { remaining } => {
                while *remaining > 0 {
                    if rows.pop_front().is_none() {
                        break;
                    }
                    *remaining -= 1;
                }
                if *remaining == 0 {
                    *self = PendingDiscard::None;
                }
            }
            PendingDiscard::ResumeBoundary {
                boundary_keys,
                last_rid,
                directions,
            } => {
                while let Some(front) = rows.front() {
                    // The first sort column governs the `_rid` tie direction
                    // (matching the backend within a full-key tie run).
                    let rid_direction = directions.first().copied().unwrap_or(SortOrder::Ascending);
                    let already_emitted =
                        match compare_key_tuples(&front.keys, boundary_keys, directions) {
                            Ordering::Less => true,
                            Ordering::Equal => {
                                compare_rids(&front.rid, last_rid, rid_direction)
                                    != Ordering::Greater
                            }
                            Ordering::Greater => false,
                        };
                    if !already_emitted {
                        // Rows are sorted, so every later row is past the boundary too.
                        *self = PendingDiscard::None;
                        return;
                    }
                    rows.pop_front();
                }
                // Boundary not reached this page (empty page, or a tie run
                // spanning pages): keep the discard active for the next page.
            }
        }
    }
}

/// Which query text a child's [`Request`] is executing, so
/// [`StreamingOrderedMerge::snapshot_state`] knows whether its backend
/// continuation (opaque, bound to the exact query text) is portable to
/// the plain-continuation resume path.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChildQueryShape {
    /// Runs the plain rewritten query; continuation may be snapshotted.
    Plain,
    /// Runs the rewritten query with a `_rid`-aware resume filter injected;
    /// continuation is bound to that filtered text and must never be
    /// stored — resumes from its scalar value boundary instead.
    ResumeFilterInjected,
}

/// One still-active child stream: an owned EPK range, a leaf [`Request`]
/// executing the rewritten query for that range, and buffered rows from
/// its current backend page. `pub(super)` only so `planner` can hold the
/// opaque `Vec<ChildStream>` [`build_children`] returns.
pub(super) struct ChildStream {
    range: FeedRange,
    node: Box<dyn PipelineNode>,
    buffered: VecDeque<query_response::EnvelopeRow>,
    /// `true` once a terminal (no-continuation) page was observed.
    drained: bool,
    last_emitted: Option<LastEmitted>,
    /// Rows emitted for this range, or `None` if not attributable (carved
    /// from a split). Only consulted by the complex-key positional resume.
    rows_emitted: Option<u64>,
    pending_discard: PendingDiscard,
    /// Which query text `node` is executing (see [`ChildQueryShape`]).
    query_shape: ChildQueryShape,
}

impl ChildStream {
    fn fresh(range: FeedRange, node: Box<dyn PipelineNode>) -> Self {
        Self {
            range,
            node,
            buffered: VecDeque::new(),
            drained: false,
            last_emitted: None,
            rows_emitted: Some(0),
            pending_discard: PendingDiscard::None,
            // Overridden to `ResumeFilterInjected` only by the scalar
            // `Exact` branch in `build_value_boundary_child`.
            query_shape: ChildQueryShape::Plain,
        }
    }

    fn record_emission(&mut self, keys: &[OrderByItem], rid: &str) {
        let resume_values: Vec<OrderByResumeValue> =
            keys.iter().map(OrderByItem::to_resume_value).collect();
        self.last_emitted = Some(LastEmitted {
            resume_values,
            rid: rid.to_owned(),
        });
        // A `None` (split-derived) count stays `None`.
        self.rows_emitted = self.rows_emitted.map(|n| n + 1);
    }

    fn boundary(&self) -> Option<ValueBoundary> {
        self.last_emitted.as_ref().map(|le| ValueBoundary {
            resume_values: le.resume_values.clone(),
            last_rid: le.rid.clone(),
            rows_emitted: self.rows_emitted,
        })
    }
}

/// Streams cross-partition `ORDER BY` results in globally sorted order
/// (see module docs for the merge algorithm and resume model).
pub(crate) struct StreamingOrderedMerge {
    /// The per-range rewritten operation with no resume filter — the base
    /// every child's `Request` starts from.
    plain_operation: Arc<CosmosOperation>,
    /// The rewritten query text, unmodified, still carrying the resume
    /// filter placeholder so a scalar filter can be substituted verbatim.
    rewritten_query: String,
    /// The query's source `ORDER BY` columns, used to build the resume
    /// filter from real source expressions, not the envelope's fields.
    columns: Vec<OrderByColumn>,
    directions: Vec<SortOrder>,
    fingerprint: String,
    children: Vec<ChildStream>,
}

impl StreamingOrderedMerge {
    pub(super) fn new(
        plain_operation: Arc<CosmosOperation>,
        rewritten_query: String,
        columns: Vec<OrderByColumn>,
        fingerprint: String,
        children: Vec<ChildStream>,
    ) -> Self {
        let directions = columns.iter().map(|c| c.direction).collect();
        Self {
            plain_operation,
            rewritten_query,
            columns,
            directions,
            fingerprint,
            children,
        }
    }

    fn max_item_count(&self) -> usize {
        match self.plain_operation.request_headers().max_item_count {
            Some(MaxItemCountHint::Limit(n)) => n.get() as usize,
            Some(MaxItemCountHint::ServerDecides) | None => DEFAULT_MAX_ITEM_COUNT,
        }
    }

    /// Borrows this node's query-shape context for [`build_children`] on a
    /// live split, so all resume paths rebuild filters identically.
    fn query_shape(&self) -> OrderByQueryShape<'_> {
        OrderByQueryShape {
            rewritten_query: &self.rewritten_query,
            columns: &self.columns,
        }
    }

    /// Ensures the child at `idx` has a buffered row or is drained,
    /// fetching/splitting as needed. Absorbs pages into `aggregator`.
    async fn prime_child(
        &mut self,
        idx: usize,
        context: &mut PipelineContext<'_>,
        aggregator: &mut PageAggregator,
    ) -> crate::error::Result<()> {
        let mut split_retries = 0;
        loop {
            if !self.children[idx].buffered.is_empty() || self.children[idx].drained {
                return Ok(());
            }

            match self.children[idx].node.next_page(context).await? {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    aggregator.absorb(&response);
                    let mut rows: VecDeque<query_response::EnvelopeRow> =
                        query_response::parse_envelope_page(
                            response.body(),
                            self.directions.len(),
                        )?
                        .into();
                    self.children[idx].pending_discard.apply(&mut rows);
                    self.children[idx].buffered = rows;
                    if is_terminal {
                        self.children[idx].drained = true;
                    }
                    if !self.children[idx].buffered.is_empty() || self.children[idx].drained {
                        return Ok(());
                    }
                    // Empty page with a continuation pending is not drained; re-poll.
                }
                PageResult::Drained => {
                    self.children[idx].drained = true;
                    return Ok(());
                }
                PageResult::SplitRequired { .. } => {
                    split_retries += 1;
                    if split_retries > MAX_SPLIT_RETRIES {
                        return Err(crate::error::CosmosError::builder()
                            .with_status(crate::error::CosmosStatus::CLIENT_SPLIT_RETRIES_EXHAUSTED)
                            .with_message(format!(
                                "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                 in StreamingOrderedMerge"
                            ))
                            .build());
                    }
                    self.handle_split(idx, context).await?;
                    // Loop: index `idx` now refers to the first replacement.
                }
            }
        }
    }

    /// Primes every currently-active child, restoring the merge invariant
    /// that before each [`select_min_child_index`] every non-drained child
    /// has a buffered head row (or is proven drained) — so no child is ever
    /// skipped for lacking a head. Re-reads `len()` each step because
    /// [`prime_child`]'s split handling can splice several replacements in at
    /// once (only the first is primed inline); already-buffered/drained
    /// children short-circuit, so no page is re-fetched.
    async fn prime_all_active_children(
        &mut self,
        context: &mut PipelineContext<'_>,
        aggregator: &mut PageAggregator,
    ) -> crate::error::Result<()> {
        let mut idx = 0;
        while idx < self.children.len() {
            self.prime_child(idx, context, aggregator).await?;
            idx += 1;
        }
        Ok(())
    }

    /// Handles a child's `SplitRequired` result: ignores `Request`'s own
    /// replacement nodes (unsafe to combine a forwarded continuation with
    /// a different filtered query shape) and rebuilds every replacement
    /// from the failed child's last-emitted boundary via [`build_children`].
    /// This resolves topology twice, but the second call is a cache hit.
    async fn handle_split(
        &mut self,
        idx: usize,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<()> {
        let scope = self.children[idx].range.clone();
        let resolved = context
            .resolve_ranges(&scope, PartitionRoutingRefresh::ForceRefresh)
            .await?;
        let prior_boundary = self.children[idx].boundary();

        let replacements = build_children(
            &resolved,
            &scope,
            &self.plain_operation,
            &self.query_shape(),
            None,
            prior_boundary.as_ref(),
        )?;

        self.children.remove(idx);
        for (i, child) in replacements.into_iter().enumerate() {
            self.children.insert(idx + i, child);
        }
        Ok(())
    }

    /// Index of the active child whose buffered head row compares smallest
    /// (per [`compare_key_tuples`], tie-broken by the direction-aware
    /// numeric `_rid` then range identity), or `None` if no child has a
    /// buffered row.
    fn select_min_child_index(&self) -> Option<usize> {
        let mut best: Option<usize> = None;
        for idx in 0..self.children.len() {
            if self.children[idx].buffered.front().is_none() {
                continue;
            }
            best = Some(match best {
                None => idx,
                Some(best_idx) => {
                    if self.row_less_than(idx, best_idx) {
                        idx
                    } else {
                        best_idx
                    }
                }
            });
        }
        best
    }

    fn row_less_than(&self, a_idx: usize, b_idx: usize) -> bool {
        let a = self.children[a_idx]
            .buffered
            .front()
            .expect("caller only compares children with a buffered row");
        let b = self.children[b_idx]
            .buffered
            .front()
            .expect("caller only compares children with a buffered row");
        // The full-key tie-break must match the backend's per-partition
        // order — numeric `_rid` in the first sort column's direction — so a
        // resumed range's scalar boundary is a clean cut point across a split.
        let rid_direction = self
            .directions
            .first()
            .copied()
            .unwrap_or(SortOrder::Ascending);
        let ordering = compare_key_tuples(&a.keys, &b.keys, &self.directions)
            .then_with(|| compare_rids(&a.rid, &b.rid, rid_direction))
            .then_with(|| {
                self.children[a_idx]
                    .range
                    .min_inclusive()
                    .cmp(self.children[b_idx].range.min_inclusive())
            });
        ordering == std::cmp::Ordering::Less
    }
}

#[async_trait]
impl PipelineNode for StreamingOrderedMerge {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.children.is_empty() {
            return Ok(PageResult::Drained);
        }

        let mut aggregator = PageAggregator::new();

        // Prime every child up front so the first `select_min_child_index`
        // sees a head row for each non-drained child (see
        // `prime_all_active_children`).
        self.prime_all_active_children(context, &mut aggregator)
            .await?;

        let cap = self.max_item_count();
        let mut payloads: Vec<Box<RawValue>> = Vec::new();

        while payloads.len() < cap {
            let Some(winner) = self.select_min_child_index() else {
                break;
            };
            let row = self.children[winner]
                .buffered
                .pop_front()
                .expect("select_min_child_index only returns indices with a buffered row");
            self.children[winner].record_emission(&row.keys, &row.rid);
            payloads.push(row.payload);
            // Re-prime before the next selection only if there's still room,
            // so unread rows stay unfetched until the next `next_page` call.
            // Priming *all* active children (not just `winner`) is required:
            // replenishing `winner` may split it into several replacements,
            // and every one must have a head row before the next selection or
            // a later replacement's smaller rows would be skipped.
            if payloads.len() < cap {
                self.prime_all_active_children(context, &mut aggregator)
                    .await?;
            }
        }

        // Evict fully-drained empty children, mirroring `SequentialDrain`,
        // so a later snapshot never references them.
        self.children
            .retain(|child| !(child.drained && child.buffered.is_empty()));
        let is_terminal = self.children.is_empty();

        let response = aggregator.build_page(&payloads)?;
        Ok(PageResult::Page {
            response,
            is_terminal,
        })
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.children.into_iter().map(|c| c.node).collect()
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        if self.children.is_empty() {
            return Ok(PipelineNodeState::Drained);
        }

        let mut ranges = Vec::with_capacity(self.children.len());
        for (idx, child) in self.children.iter().enumerate() {
            // Safe to snapshot the backend continuation into the plain
            // `server_continuation` field only when nothing is buffered
            // (else it points past unemitted rows) and the child runs the
            // plain query (`ChildQueryShape::Plain`) — a resume-filtered
            // child's continuation is bound to that filtered text and would
            // mismatch the plain query on resume; it resumes from its
            // scalar `boundary` instead.
            let server_continuation = if child.buffered.is_empty()
                && child.query_shape == ChildQueryShape::Plain
            {
                match child.node.snapshot_state()? {
                    PipelineNodeState::Request {
                        server_continuation,
                    } => server_continuation,
                    PipelineNodeState::Drained => None,
                    other => {
                        return Err(crate::error::CosmosError::builder()
                            .with_status(
                                crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE,
                            )
                            .with_message(format!(
                                "StreamingOrderedMerge child {idx} of {total} produced an \
                                 unsupported snapshot shape: {other:?}",
                                total = self.children.len(),
                            ))
                            .build());
                    }
                }
            } else {
                None
            };

            ranges.push(OrderByRangeToken {
                min_epk: child.range.min_inclusive().to_hex(),
                max_epk: child.range.max_exclusive().to_hex(),
                server_continuation,
                boundary: child.boundary(),
            });
        }

        Ok(PipelineNodeState::StreamingOrderedMerge {
            directions: self.directions.clone(),
            query_fingerprint: self.fingerprint.clone(),
            ranges,
        })
    }

    fn topology_can_change(&self) -> bool {
        // Splits are handled internally (`handle_split`); no parent needed.
        false
    }
}

/// The query-shape context needed to build (and resume) a range's child
/// streams (rewritten query text and source `ORDER BY` columns), shared by
/// fresh, resume, and live-split paths.
pub(super) struct OrderByQueryShape<'a> {
    pub(super) rewritten_query: &'a str,
    pub(super) columns: &'a [OrderByColumn],
}

/// Builds the child streams needed to cover `scope`, given its topology
/// resolution and prior resume state (`None` for a fresh range). Shared
/// by planner construction and the live split handler.
///
/// - Unchanged topology + `prior_continuation`: replays it as-is (safe —
///   see `snapshot_state` / [`ChildQueryShape`]).
/// - Else + `prior_boundary`: rebuilds via `build_value_boundary_child`.
///   Scalar boundaries are `_rid`-aware and split-safe; a complex boundary
///   crossing a split is rejected rather than risk dropping rows.
/// - Else: a fresh unfiltered start. On a topology change, resolved
///   ranges are clipped to `scope` and must exactly tile it.
pub(super) fn build_children(
    resolved: &[ResolvedRange],
    scope: &FeedRange,
    plain_operation: &Arc<CosmosOperation>,
    query_shape: &OrderByQueryShape<'_>,
    prior_continuation: Option<String>,
    prior_boundary: Option<&ValueBoundary>,
) -> crate::error::Result<Vec<ChildStream>> {
    let unchanged = resolved.len() == 1
        && resolved[0].range.min_inclusive() == scope.min_inclusive()
        && resolved[0].range.max_exclusive() == scope.max_exclusive();

    if unchanged {
        let resolved_range = &resolved[0];
        let target = RequestTarget::effective_partition_key_range(
            scope.clone(),
            resolved_range.partition_key_range_id.clone(),
            resolved_range.range.clone(),
        );
        let child = if let Some(continuation) = prior_continuation {
            // Safe to replay as-is (see doc comment above); carry the
            // bookkeeping fields forward for the next snapshot's boundary.
            let mut child = ChildStream::fresh(
                scope.clone(),
                Box::new(Request::new(
                    Arc::clone(plain_operation),
                    target,
                    Some(continuation),
                )),
            );
            if let Some(boundary) = prior_boundary {
                child.last_emitted = Some(LastEmitted {
                    resume_values: boundary.resume_values.clone(),
                    rid: boundary.last_rid.clone(),
                });
                child.rows_emitted = boundary.rows_emitted;
            }
            child
        } else if let Some(boundary) = prior_boundary {
            // Rows were emitted but no safe continuation was saved (a
            // mid-page checkpoint); a plain restart would re-emit them.
            build_value_boundary_child(
                scope.clone(),
                target,
                plain_operation,
                query_shape,
                boundary,
            )?
        } else {
            // Genuinely fresh: no continuation, no rows ever emitted.
            ChildStream::fresh(
                scope.clone(),
                Box::new(Request::new(Arc::clone(plain_operation), target, None)),
            )
        };
        return Ok(vec![child]);
    }

    // Topology changed (split or merge). Clip resolved ranges to `scope`
    // before validating coverage — a merge yields a range wider than
    // `scope`, and the raw bounds would spuriously fail. Mirrors
    // `SequentialDrain`'s resume path via `intersect_feed_ranges`.
    let mut clipped: Vec<(FeedRange, &ResolvedRange)> = Vec::with_capacity(resolved.len());
    for resolved_range in resolved {
        if let Some(owned) = intersect_feed_ranges(&resolved_range.range, scope) {
            clipped.push((owned, resolved_range));
        }
        // Non-overlapping ranges contribute nothing; a coverage gap fails below.
    }
    clipped.sort_by(|a, b| a.0.min_inclusive().cmp(b.0.min_inclusive()));
    validate_exact_coverage(scope, &clipped)?;

    // A complex (array/object) boundary can only resume via positional
    // rescan of the whole range's prefix; that count can't be attributed to
    // individual sub-ranges after a split, so reject rather than risk
    // silently omitting rows. Scalar boundaries are split-safe.
    if let Some(boundary) = prior_boundary {
        if boundary
            .resume_values
            .iter()
            .any(OrderByResumeValue::is_complex)
            && clipped.len() > 1
        {
            return Err(complex_boundary_topology_change());
        }
    }

    // A genuine split means the inherited `rows_emitted` count spans
    // sibling sub-ranges and can't seed a later positional rescan for any
    // one of them. A single clipped range (unchanged topology or a merge)
    // keeps its count.
    let count_attributable = clipped.len() == 1;

    let mut children = Vec::with_capacity(clipped.len());
    for (owned, resolved_range) in clipped {
        let target = RequestTarget::effective_partition_key_range(
            owned.clone(),
            resolved_range.partition_key_range_id.clone(),
            resolved_range.range.clone(),
        );
        let child = match prior_boundary {
            None => ChildStream::fresh(
                owned,
                Box::new(Request::new(Arc::clone(plain_operation), target, None)),
            ),
            Some(boundary) => {
                let mut child = build_value_boundary_child(
                    owned,
                    target,
                    plain_operation,
                    query_shape,
                    boundary,
                )?;
                if !count_attributable {
                    child.rows_emitted = None;
                }
                child
            }
        };
        children.push(child);
    }
    Ok(children)
}

/// Returns `Ok(())` if `clipped` (already sorted ascending by
/// `min_inclusive`, each range already clipped to `scope`) exactly tiles
/// `scope` end-to-end with no gaps or overlaps.
fn validate_exact_coverage(
    scope: &FeedRange,
    clipped: &[(FeedRange, &ResolvedRange)],
) -> crate::error::Result<()> {
    if clipped.is_empty() {
        return Err(split_replacement_invalid(
            "topology resolution produced no replacement ranges",
        ));
    }
    let mut cursor = scope.min_inclusive().clone();
    for (owned, _) in clipped {
        if owned.min_inclusive() != &cursor {
            return Err(split_replacement_invalid(format!(
                "replacement range [{}, {}) does not start at the expected cursor {}",
                owned.min_inclusive().to_hex(),
                owned.max_exclusive().to_hex(),
                cursor.to_hex(),
            )));
        }
        cursor = owned.max_exclusive().clone();
    }
    if &cursor != scope.max_exclusive() {
        return Err(split_replacement_invalid(format!(
            "replacement ranges cover up to {} but the prior range extended to {}",
            cursor.to_hex(),
            scope.max_exclusive().to_hex(),
        )));
    }
    Ok(())
}

fn split_replacement_invalid(
    message: impl Into<std::borrow::Cow<'static, str>>,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID)
        .with_message(message)
        .build()
}

/// Error for a range with a complex (array/object) resume boundary that is
/// affected by a partition split — its positional-rescan count can't be
/// attributed to post-split sub-ranges without risking dropped rows.
fn complex_boundary_topology_change() -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(
            crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_COMPLEX_BOUNDARY_TOPOLOGY_CHANGE,
        )
        .with_message(
            "cannot resume a cross-partition ORDER BY continuation with a complex \
             (array/object) sort-key boundary for a range affected by a partition split: the \
             array/object value is stored only as a bounded hash, so its already-emitted prefix \
             cannot be attributed to the post-split sub-ranges without risking dropped rows. \
             Re-issue the query from the beginning.",
        )
        .build()
}

/// Builds one child via the value-boundary resume path: a scalar-only
/// boundary injects a `_rid`-aware seek filter into the rewritten query's
/// placeholder and installs a matching [`PendingDiscard::ResumeBoundary`]
/// guard; a complex (array/object) boundary reissues the plain query and
/// positionally discards the already-emitted prefix instead. Callers must
/// already have rejected a complex boundary crossing a topology change (see
/// [`build_children`]).
fn build_value_boundary_child(
    owned_range: FeedRange,
    target: RequestTarget,
    plain_operation: &Arc<CosmosOperation>,
    query_shape: &OrderByQueryShape<'_>,
    boundary: &ValueBoundary,
) -> crate::error::Result<ChildStream> {
    // Seed collision-free resume-parameter names from the caller's existing
    // query-body parameters so a resume binding can never overwrite one.
    let existing_parameter_names = query_response::query_parameter_names(plain_operation.body())?;
    let (operation, pending_discard, child_shape) = match ResumeFilter::build(
        query_shape.columns,
        &boundary.resume_values,
        &existing_parameter_names,
    ) {
        ResumeFilter::Exact {
            where_fragment,
            parameters,
        } => {
            let filtered = query_response::rewritten_query_with_resume_filter(
                query_shape.rewritten_query,
                &where_fragment,
            )?;
            // The scalar boundary values are appended as query parameters,
            // never inlined into the SQL text.
            let body = query_response::rewrite_query_body_with_parameters(
                plain_operation.body(),
                &filtered,
                &parameters,
            )?;
            let operation = Arc::new((*Arc::clone(plain_operation)).clone().with_body(body));
            // All resume values are scalar here (`Exact` branch).
            let boundary_keys: Vec<OrderByItem> = boundary
                .resume_values
                .iter()
                .map(OrderByResumeValue::to_scalar_order_by_item)
                .collect::<Option<Vec<_>>>()
                .expect("ResumeFilter::Exact is only returned for an all-scalar boundary");
            let directions: Vec<SortOrder> =
                query_shape.columns.iter().map(|c| c.direction).collect();
            (
                operation,
                PendingDiscard::ResumeBoundary {
                    boundary_keys,
                    last_rid: boundary.last_rid.clone(),
                    directions,
                },
                // Continuation is bound to the filtered text; never snapshot it.
                ChildQueryShape::ResumeFilterInjected,
            )
        }
        ResumeFilter::PositionalRescan => {
            // A `None` count means the range was carved from a split, so a
            // positional rescan could silently drop rows — reject instead.
            let remaining = boundary
                .rows_emitted
                .ok_or_else(complex_boundary_topology_change)?;
            (
                Arc::clone(plain_operation),
                PendingDiscard::Positional { remaining },
                ChildQueryShape::Plain,
            )
        }
    };

    let mut child =
        ChildStream::fresh(owned_range, Box::new(Request::new(operation, target, None)));
    child.last_emitted = Some(LastEmitted {
        resume_values: boundary.resume_values.clone(),
        rid: boundary.last_rid.clone(),
    });
    child.rows_emitted = boundary.rows_emitted;
    child.pending_discard = pending_discard;
    child.query_shape = child_shape;
    Ok(child)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::{self, MockLeaf};
    use crate::models::effective_partition_key::EffectivePartitionKey;

    fn range(min: &str, max: &str) -> FeedRange {
        FeedRange::new(
            EffectivePartitionKey::from(min),
            EffectivePartitionKey::from(max),
        )
        .unwrap()
    }

    /// Builds a rewritten-envelope backend `CosmosResponse` with one row per
    /// `(rid, rank)` pair (for a `MockRequestExecutor` reply).
    fn envelope_response(
        rows: &[(&str, i64)],
        continuation: Option<&str>,
    ) -> crate::models::CosmosResponse {
        let documents: Vec<serde_json::Value> = rows
            .iter()
            .map(|(rid, rank)| {
                serde_json::json!({
                    "_rid": rid,
                    "orderByItems": [{"item": rank}],
                    "payload": {"id": rid, "rank": rank},
                })
            })
            .collect();
        let body =
            serde_json::json!({"_rid": "", "Documents": documents, "_count": documents.len()});
        mocks::response_with_continuation(&serde_json::to_vec(&body).unwrap(), continuation)
    }

    /// Wraps [`envelope_response`] in a `PageResult` for a `MockLeaf` page.
    fn envelope_page(
        rows: &[(&str, i64)],
        continuation: Option<&str>,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: envelope_response(rows, continuation),
            is_terminal: continuation.is_none(),
        })
    }

    fn ids(response: &crate::models::CosmosResponse) -> Vec<String> {
        let value: serde_json::Value = serde_json::from_slice(response.body_bytes()).unwrap();
        value["Documents"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item["id"].as_str().unwrap().to_owned())
            .collect()
    }

    fn merge(children: Vec<ChildStream>, directions: Vec<SortOrder>) -> StreamingOrderedMerge {
        // Column expressions are irrelevant to these already-built-child tests.
        let columns: Vec<OrderByColumn> = directions
            .iter()
            .enumerate()
            .map(|(i, direction)| OrderByColumn {
                expression: format!("c.col{i}"),
                direction: *direction,
            })
            .collect();
        StreamingOrderedMerge::new(
            Arc::new(mocks::operation()),
            "SELECT ...".to_owned(),
            columns,
            "fp".to_owned(),
            children,
        )
    }

    async fn next_page(node: &mut StreamingOrderedMerge) -> PageResult {
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        node.next_page(&mut context).await.unwrap()
    }

    /// A `MockLeaf` page signalling its range split; the merge rebuilds
    /// replacements itself via `handle_split`, so no `replacement_nodes` are
    /// needed here.
    fn split_required_page() -> crate::error::Result<PageResult> {
        Ok(PageResult::SplitRequired {
            replacement_nodes: Vec::new(),
        })
    }

    /// A merge whose children run the real rewritten `c.rank` query (body +
    /// resume-filter placeholder), so a live split rebuilds `_rid`-aware
    /// resume children through `build_children`/`build_value_boundary_child`.
    fn rank_merge(children: Vec<ChildStream>) -> StreamingOrderedMerge {
        StreamingOrderedMerge::new(
            query_operation(),
            REWRITTEN_WITH_PLACEHOLDER.to_owned(),
            rank_columns(),
            "fp".to_owned(),
            children,
        )
    }

    /// Drains `node` to completion against a real executor/topology (needed
    /// once a child splits mid-merge), returning every emitted id in order.
    async fn drain_all_ids(
        node: &mut StreamingOrderedMerge,
        executor: &mut mocks::MockRequestExecutor,
        topology: &mut mocks::MockTopologyProvider,
    ) -> Vec<String> {
        let mut all = Vec::new();
        loop {
            let mut context = PipelineContext::new(executor, Some(topology));
            match node.next_page(&mut context).await.unwrap() {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    all.extend(ids(&response));
                    if is_terminal {
                        break;
                    }
                }
                PageResult::Drained => break,
                PageResult::SplitRequired { .. } => {
                    panic!("merge must handle splits internally, never surface SplitRequired")
                }
            }
        }
        all
    }

    /// Regression for the in-flight split ordering defect: when replenishing
    /// the popped winner fans it out into several sub-ranges, *every*
    /// replacement (not just the first) must be primed before the next
    /// selection. P0 emits `[1, 2]` then splits into P0a (next `3`) and P0b
    /// (next `10, 20`); if P0b were left unprimed, `50` would be emitted
    /// before `10, 20`. A large page cap keeps popping within one page.
    #[tokio::test]
    async fn split_during_pop_loop_primes_all_split_replacements() {
        let p0 = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                split_required_page(),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("d50", 50)],
                None,
            )])),
        );
        let mut node = rank_merge(vec![p0, p1]);

        // `handle_split` re-resolves P0's range into two sub-ranges.
        let mut topology = mocks::MockTopologyProvider::new(vec![Ok(vec![
            resolved_range("", "40", "pk-a"),
            resolved_range("40", "80", "pk-b"),
        ])]);
        // The two replacement Request nodes fetch their post-split pages
        // (mock returns them unfiltered; the `_rid`-aware discard keeps all
        // rows past the `2` boundary).
        let mut executor = mocks::MockRequestExecutor::new(vec![
            Ok(envelope_response(&[("d3", 3)], None)),
            Ok(envelope_response(&[("d10", 10), ("d20", 20)], None)),
        ]);

        let emitted = drain_all_ids(&mut node, &mut executor, &mut topology).await;
        assert_eq!(
            emitted,
            vec!["d1", "d2", "d3", "d10", "d20", "d50"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            "the second split replacement's smaller rows (10, 20) must precede 50"
        );
    }

    /// Companion to the large-cap regression: with a page cap that fills
    /// exactly as the split completes, the merge must checkpoint immediately
    /// after the split — snapshotting *both* replacements — then resume in
    /// order on the next page. P0 emits `[1, 2]`, splits, emits `3`
    /// (cap = 3 reached), and P0b (`10, 20`) plus P1 (`50`) follow next page.
    #[tokio::test]
    async fn split_during_pop_loop_small_cap_checkpoints_after_split() {
        let p0 = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                split_required_page(),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("d50", 50)],
                None,
            )])),
        );
        let mut node = rank_merge(vec![p0, p1]);
        node.plain_operation = Arc::new((*node.plain_operation).clone().with_max_item_count(
            crate::models::MaxItemCountHint::Limit(std::num::NonZeroU32::new(3).unwrap()),
        ));

        let mut topology = mocks::MockTopologyProvider::new(vec![Ok(vec![
            resolved_range("", "40", "pk-a"),
            resolved_range("40", "80", "pk-b"),
        ])]);
        let mut executor = mocks::MockRequestExecutor::new(vec![
            Ok(envelope_response(&[("d3", 3)], None)),
            Ok(envelope_response(&[("d10", 10), ("d20", 20)], None)),
        ]);

        // Page 1 stops right after the split, at the cap.
        let page1 = {
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            node.next_page(&mut context).await.unwrap()
        };
        let PageResult::Page {
            response: r1,
            is_terminal: t1,
        } = page1
        else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&r1),
            vec!["d1".to_owned(), "d2".to_owned(), "d3".to_owned()]
        );
        assert!(!t1, "P0b and P1 rows remain buffered for the next page");

        // The checkpoint must retain both post-split children (P0b + P1),
        // with P0b carrying its `_rid`-aware resume boundary.
        match node.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert_eq!(
                    ranges.len(),
                    2,
                    "both post-split children must be snapshotted"
                );
                assert!(
                    ranges[0].boundary.is_some(),
                    "the surviving split replacement must resume from its value boundary"
                );
            }
            other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
        }

        // Page 2 drains the rest in order.
        let page2 = {
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            node.next_page(&mut context).await.unwrap()
        };
        let PageResult::Page {
            response: r2,
            is_terminal: t2,
        } = page2
        else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&r2),
            vec!["d10".to_owned(), "d20".to_owned(), "d50".to_owned()]
        );
        assert!(t2, "all children drained after the second page");
    }

    #[tokio::test]
    async fn ties_are_broken_deterministically_by_rid() {
        // Both children have a row with rank=1; RID "a" < "b" must win.
        let left = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(&[("b", 1)], None)])),
        );
        let right = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(&[("a", 1)], None)])),
        );
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["a".to_owned(), "b".to_owned()]);
    }

    #[tokio::test]
    async fn multi_column_mixed_direction_drives_pop_order() {
        // Column 0 ASC ties at 1; column 1 DESC means "b" sorts before "a".
        fn two_col_page(rid: &str, c0: i64, c1: &str) -> crate::error::Result<PageResult> {
            let body = serde_json::json!({
                "_rid": "",
                "Documents": [{
                    "_rid": rid,
                    "orderByItems": [{"item": c0}, {"item": c1}],
                    "payload": {"id": rid},
                }],
                "_count": 1,
            });
            Ok(PageResult::Page {
                response: mocks::response(&serde_json::to_vec(&body).unwrap()),
                is_terminal: true,
            })
        }
        let left = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![two_col_page("left", 1, "a")])),
        );
        let right = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![two_col_page("right", 1, "b")])),
        );
        let mut node = merge(
            vec![left, right],
            vec![SortOrder::Ascending, SortOrder::Descending],
        );
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["right".to_owned(), "left".to_owned()]);
    }

    #[tokio::test]
    async fn undefined_sorts_before_defined_values_across_partitions() {
        fn page(rid: &str, item: Option<i64>) -> crate::error::Result<PageResult> {
            let order_by_items = match item {
                Some(v) => serde_json::json!([{"item": v}]),
                None => serde_json::json!([{}]),
            };
            let body = serde_json::json!({
                "_rid": "",
                "Documents": [{"_rid": rid, "orderByItems": order_by_items, "payload": {"id": rid}}],
                "_count": 1,
            });
            Ok(PageResult::Page {
                response: mocks::response(&serde_json::to_vec(&body).unwrap()),
                is_terminal: true,
            })
        }
        let left = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![page("has-value", Some(1))])),
        );
        let right = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![page("undefined", None)])),
        );
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["undefined".to_owned(), "has-value".to_owned()]
        );
    }

    /// Helper: an all-scalar single-column resume-boundary discard.
    fn number_boundary_discard(value: f64, last_rid: &str) -> PendingDiscard {
        PendingDiscard::ResumeBoundary {
            boundary_keys: vec![OrderByItem::Number(value.into())],
            last_rid: last_rid.to_owned(),
            directions: vec![SortOrder::Ascending],
        }
    }

    #[tokio::test]
    async fn resume_with_boundary_discards_already_emitted_ties_by_rid() {
        // Rows tied on rank=5 with `_rid <= "tied-2"` were already emitted.
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("tied-1", 5), ("tied-2", 5), ("tied-3", 5), ("new", 6)],
                None,
            )])),
        );
        child.pending_discard = number_boundary_discard(5.0, "tied-2");
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["tied-3".to_owned(), "new".to_owned()]);
    }

    /// Regression: an empty leading page (with continuation) must keep the
    /// boundary discard active, not clear it, or later tied rows leak through.
    #[tokio::test]
    async fn resume_boundary_discard_survives_empty_leading_page() {
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[], Some("ct-empty")),
                envelope_page(&[("tied-1", 5), ("new", 6)], None),
            ])),
        );
        child.pending_discard = number_boundary_discard(5.0, "tied-1");
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["new".to_owned()],
            "the tied row on the second page must still be discarded"
        );
    }

    /// Regression: a tie run spanning a page boundary must stay fully
    /// discarded, not just the portion on the first page.
    #[tokio::test]
    async fn resume_boundary_discard_survives_tie_run_spanning_pages() {
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("tied-1", 5), ("tied-2", 5)], Some("ct-mid")),
                envelope_page(&[("tied-3", 5), ("new", 6)], None),
            ])),
        );
        child.pending_discard = number_boundary_discard(5.0, "tied-3");
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["new".to_owned()],
            "every tied row up to and including tied-3 was already emitted"
        );
    }

    /// Regression: after a split, both sub-ranges resume from the same
    /// boundary; the `_rid`-aware discard avoids dropping/duplicating rows.
    #[tokio::test]
    async fn split_resume_is_rid_aware_with_no_omissions_or_duplicates() {
        // Pre-split emitted a,b,c tied on rank=5; left keeps a,c + e (unemitted tie) + m;
        // right keeps b + z.
        let left = {
            let mut c = ChildStream::fresh(
                range("", "80"),
                Box::new(MockLeaf::with_pages(vec![envelope_page(
                    &[("a", 5), ("c", 5), ("e", 5), ("m", 7)],
                    None,
                )])),
            );
            c.pending_discard = number_boundary_discard(5.0, "c");
            c
        };
        let right = {
            let mut c = ChildStream::fresh(
                range("80", "FF"),
                Box::new(MockLeaf::with_pages(vec![envelope_page(
                    &[("b", 5), ("z", 6)],
                    None,
                )])),
            );
            c.pending_discard = number_boundary_discard(5.0, "c");
            c
        };
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["e".to_owned(), "z".to_owned(), "m".to_owned()],
            "the unemitted tied row `e` must survive (no omission) and no \
             already-emitted row may reappear (no duplicate)"
        );
    }

    #[tokio::test]
    async fn resume_with_positional_rescan_discards_exact_row_count() {
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("old-1", 1), ("old-2", 2), ("new", 3)],
                None,
            )])),
        );
        child.pending_discard = PendingDiscard::Positional { remaining: 2 };
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["new".to_owned()]);
    }

    // ── build_children resume/topology paths ─────────────────────────────

    fn query_operation() -> Arc<CosmosOperation> {
        Arc::new(
            mocks::operation().with_body(
                br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec(),
            ),
        )
    }

    /// A single ascending source column (`c.rank`) — what the planner
    /// builds from `QueryInfo::order_by_expressions` + `order_by`.
    fn rank_columns() -> Vec<OrderByColumn> {
        vec![OrderByColumn {
            expression: "c.rank".to_owned(),
            direction: SortOrder::Ascending,
        }]
    }

    /// A rewritten query carrying the Gateway's resume-filter placeholder,
    /// as a real rewritten `ORDER BY` query always has.
    const REWRITTEN_WITH_PLACEHOLDER: &str =
        "SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c \
         WHERE {documentdb-formattableorderbyquery-filter} ORDER BY c.rank ASC";

    /// The `OrderByQueryShape` for a single-`c.rank`-column scalar resume.
    fn rank_query_shape(columns: &[OrderByColumn]) -> OrderByQueryShape<'_> {
        OrderByQueryShape {
            rewritten_query: REWRITTEN_WITH_PLACEHOLDER,
            columns,
        }
    }

    fn resolved_range(min: &str, max: &str, id: &str) -> ResolvedRange {
        ResolvedRange {
            partition_key_range_id: id.to_owned(),
            range: range(min, max),
        }
    }

    fn scalar_boundary(value: f64, last_rid: &str) -> ValueBoundary {
        ValueBoundary {
            resume_values: vec![OrderByResumeValue::Number {
                value: value.into(),
            }],
            last_rid: last_rid.to_owned(),
            rows_emitted: Some(3),
        }
    }

    fn complex_boundary(last_rid: &str) -> ValueBoundary {
        ValueBoundary {
            resume_values: vec![
                OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value()
            ],
            last_rid: last_rid.to_owned(),
            rows_emitted: Some(3),
        }
    }

    /// A scalar boundary crossing a split fans out into one `_rid`-aware
    /// child per sub-range, never a replicated positional count.
    #[test]
    fn build_children_splits_scalar_boundary_into_rid_aware_children() {
        let op = query_operation();
        let scope = range("", "FF");
        let resolved = vec![
            resolved_range("", "80", "pk-left"),
            resolved_range("80", "FF", "pk-right"),
        ];
        let boundary = scalar_boundary(5.0, "c");
        let children = build_children(
            &resolved,
            &scope,
            &op,
            &rank_query_shape(&rank_columns()),
            None,
            Some(&boundary),
        )
        .expect("scalar boundary resumes across a split");
        assert_eq!(children.len(), 2);
        for child in &children {
            assert!(
                matches!(child.pending_discard, PendingDiscard::ResumeBoundary { .. }),
                "each split sub-range must resume via the _rid-aware discard"
            );
            assert_eq!(
                child.rows_emitted, None,
                "a split replacement's positional count is not attributable to it, \
                 so it must be dropped (guarding a later complex-boundary rescan)"
            );
        }
    }

    /// A complex (array/object) boundary crossing a split is rejected
    /// rather than replicating a whole-range positional count.
    #[test]
    fn build_children_rejects_complex_boundary_across_split() {
        let op = query_operation();
        let scope = range("", "FF");
        let resolved = vec![
            resolved_range("", "80", "pk-left"),
            resolved_range("80", "FF", "pk-right"),
        ];
        let boundary = complex_boundary("rid-1");
        let result = build_children(
            &resolved,
            &scope,
            &op,
            &rank_query_shape(&rank_columns()),
            None,
            Some(&boundary),
        );
        let err = match result {
            Ok(_) => panic!("a complex boundary cannot be split"),
            Err(e) => e,
        };
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_COMPLEX_BOUNDARY_TOPOLOGY_CHANGE
        );
    }

    /// A complex boundary with an unattributable (`None`) count must still
    /// be rejected on unchanged topology — the scalar-then-complex hazard.
    #[test]
    fn build_children_rejects_complex_boundary_with_unattributable_count() {
        let op = query_operation();
        let scope = range("", "80");
        let resolved = vec![resolved_range("", "80", "pk-0")];
        let boundary = ValueBoundary {
            resume_values: vec![
                OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value()
            ],
            last_rid: "rid-1".to_owned(),
            rows_emitted: None, // unattributable: left over from a prior split
        };
        let result = build_children(
            &resolved,
            &scope,
            &op,
            &rank_query_shape(&rank_columns()),
            None,
            Some(&boundary),
        );
        let err = match result {
            Ok(_) => panic!("a complex boundary with an unattributable count cannot resume"),
            Err(e) => e,
        };
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_COMPLEX_BOUNDARY_TOPOLOGY_CHANGE
        );
    }

    /// A merge resolves the saved sub-range to a wider physical range; it
    /// must be clipped to scope before coverage validation, not rejected.
    #[test]
    fn build_children_clips_merged_physical_range_to_scope() {
        let op = query_operation();
        let scope = range("", "80");
        // Post-merge: the saved [00,80) sub-range is now served by [00,FF).
        let resolved = vec![resolved_range("", "FF", "pk-merged")];
        let boundary = scalar_boundary(5.0, "c");
        let children = build_children(
            &resolved,
            &scope,
            &op,
            &rank_query_shape(&rank_columns()),
            None,
            Some(&boundary),
        )
        .expect("a merged (widened) physical range clips to the saved scope");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].range, scope);
    }

    /// A complex boundary is still resumable across a merge (single clipped
    /// sub-range), since the positional rescan applies unambiguously.
    #[test]
    fn build_children_allows_complex_boundary_across_merge() {
        let op = query_operation();
        let scope = range("", "80");
        let resolved = vec![resolved_range("", "FF", "pk-merged")];
        let boundary = complex_boundary("rid-1");
        let children = build_children(
            &resolved,
            &scope,
            &op,
            &rank_query_shape(&rank_columns()),
            None,
            Some(&boundary),
        )
        .expect("a complex boundary resumes across a merge (single clipped range)");
        assert_eq!(children.len(), 1);
        assert!(matches!(
            children[0].pending_discard,
            PendingDiscard::Positional { remaining: 3 }
        ));
    }

    #[tokio::test]
    async fn page_size_cap_retains_unread_rows_for_next_call() {
        let child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("a", 1), ("b", 2), ("c", 3)],
                None,
            )])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        // Force a page size of 1 directly via the operation.
        node.plain_operation = Arc::new((*node.plain_operation).clone().with_max_item_count(
            crate::models::MaxItemCountHint::Limit(std::num::NonZeroU32::new(1).unwrap()),
        ));

        let PageResult::Page {
            response: r1,
            is_terminal: t1,
        } = next_page(&mut node).await
        else {
            panic!("expected a page");
        };
        assert_eq!(ids(&r1), vec!["a".to_owned()]);
        assert!(!t1, "more rows remain buffered/available");

        let PageResult::Page {
            response: r2,
            is_terminal: t2,
        } = next_page(&mut node).await
        else {
            panic!("expected a page");
        };
        assert_eq!(ids(&r2), vec!["b".to_owned()]);
        assert!(!t2);

        let PageResult::Page {
            response: r3,
            is_terminal: t3,
        } = next_page(&mut node).await
        else {
            panic!("expected a page");
        };
        assert_eq!(ids(&r3), vec!["c".to_owned()]);
        assert!(t3, "the child is drained and its buffer is now empty");
    }

    #[tokio::test]
    async fn terminal_page_reports_drained_children() {
        let child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(&[("a", 1)], None)])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { is_terminal, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert!(is_terminal);
        assert!(node.children.is_empty());

        // Calling again on an already-fully-drained merge reports Drained.
        assert!(matches!(next_page(&mut node).await, PageResult::Drained));
    }

    #[tokio::test]
    async fn malformed_envelope_surfaces_typed_error() {
        let body = serde_json::json!({
            "_rid": "",
            "Documents": [{"_rid": "a", "orderByItems": [{"item": 1}]}], // missing payload
            "_count": 1,
        });
        let child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: mocks::response(&serde_json::to_vec(&body).unwrap()),
                is_terminal: true,
            })])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let err = node.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
    }

    #[tokio::test]
    async fn snapshot_state_reports_drained_for_empty_merge() {
        let node = merge(vec![], vec![SortOrder::Ascending]);
        assert_eq!(node.snapshot_state().unwrap(), PipelineNodeState::Drained);
    }

    /// Only a `ChildQueryShape::Plain` child may surface its live
    /// continuation; a `ResumeFilterInjected` child's is bound to the
    /// filtered text and must be suppressed, using its scalar boundary instead.
    #[test]
    fn snapshot_suppresses_resume_filtered_child_backend_continuation() {
        fn child_with_live_continuation(shape: ChildQueryShape, token: &str) -> ChildStream {
            // Empty buffer, not drained, with a live backend continuation.
            let mut child = ChildStream::fresh(
                range("", "FF"),
                Box::new(
                    MockLeaf::with_pages(vec![]).with_snapshot(PipelineNodeState::Request {
                        server_continuation: Some(token.to_owned()),
                    }),
                ),
            );
            child.last_emitted = Some(LastEmitted {
                resume_values: vec![OrderByResumeValue::Number { value: 1.0.into() }],
                rid: "a".to_owned(),
            });
            child.query_shape = shape;
            child
        }

        let resume_filtered = merge(
            vec![child_with_live_continuation(
                ChildQueryShape::ResumeFilterInjected,
                "resume-filtered-tok",
            )],
            vec![SortOrder::Ascending],
        );
        match resume_filtered.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert_eq!(ranges.len(), 1);
                assert_eq!(
                    ranges[0].server_continuation, None,
                    "a resume-filtered child must never leak its backend continuation"
                );
                assert!(
                    ranges[0].boundary.is_some(),
                    "it resumes from its scalar boundary instead"
                );
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }

        let plain = merge(
            vec![child_with_live_continuation(
                ChildQueryShape::Plain,
                "plain-tok",
            )],
            vec![SortOrder::Ascending],
        );
        match plain.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert_eq!(
                    ranges[0].server_continuation,
                    Some("plain-tok".to_owned()),
                    "a plain-query child's backend continuation is portable and must be saved"
                );
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn cancellation_error_propagates_from_child_fetch() {
        let child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![Err(
                mocks::non_topology_gone_error(),
            )])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(node.next_page(&mut context).await.is_err());
    }

    // ── Catalog-driven scenarios ─────────────────────────────────────────
    //
    // Reuses `tests/fixtures/streaming_order_by_scenarios.json`; this
    // file's copy of the fixture schema is minimal since separate
    // compilation units can't share a `pub(crate)` type — see
    // `tests/streaming_order_by_scenario_catalog.rs` for the canonical
    // strict schema every layer trusts.
    #[derive(serde::Deserialize)]
    struct CatalogFixture {
        scenarios: Vec<ScenarioFixture>,
    }

    #[derive(serde::Deserialize)]
    struct ScenarioFixture {
        id: String,
        layers: Vec<String>,
        query: QueryFixture,
        mock: Option<MockFixture>,
        #[serde(rename = "expectedIds", default)]
        expected_ids: Vec<String>,
        checkpoint: Option<serde_json::Value>,
        #[serde(rename = "expectedError")]
        expected_error: Option<serde_json::Value>,
    }

    #[derive(serde::Deserialize)]
    struct QueryFixture {
        columns: Vec<ColumnFixture>,
    }

    #[derive(serde::Deserialize)]
    struct ColumnFixture {
        direction: String,
    }

    #[derive(serde::Deserialize)]
    struct MockFixture {
        partitions: Vec<PartitionFixture>,
    }

    #[derive(serde::Deserialize)]
    struct PartitionFixture {
        pages: Vec<serde_json::Value>,
    }

    fn fixture_direction(s: &str) -> SortOrder {
        match s {
            "Ascending" => SortOrder::Ascending,
            "Descending" => SortOrder::Descending,
            other => panic!("unknown direction in fixture: {other}"),
        }
    }

    /// Runs catalog scenarios tagged `mockPipeline` through the real node,
    /// asserting drained order matches `expectedIds`. Split checkpoints are
    /// skipped (need the real planner — see `integration_tests::order_by_resume`);
    /// malformed scenarios are covered by `malformed_envelope_surfaces_typed_error`.
    #[tokio::test]
    async fn catalog_mock_pipeline_scenarios_drain_in_expected_order() {
        const CATALOG_JSON: &str =
            include_str!("../../../tests/fixtures/streaming_order_by_scenarios.json");
        let catalog: CatalogFixture =
            serde_json::from_str(CATALOG_JSON).expect("catalog must parse");

        let mut ran_at_least_one = false;
        let mut ran_a_resume_checkpoint = false;
        for scenario in &catalog.scenarios {
            if !scenario.layers.iter().any(|l| l == "mockPipeline") {
                continue;
            }
            let Some(mock) = &scenario.mock else {
                continue;
            };
            if scenario.expected_error.is_some() {
                // Covered by `malformed_envelope_surfaces_typed_error` instead.
                continue;
            }

            let directions: Vec<SortOrder> = scenario
                .query
                .columns
                .iter()
                .map(|c| fixture_direction(&c.direction))
                .collect();

            // Split checkpoints (`splitBeforeRow`/`replacementRanges`) need the
            // real planner + topology, so they can't run on `MockLeaf` here and
            // are skipped; mid-page-split execution coverage lives in the
            // `split_during_*` node tests and `integration_tests::order_by_resume`
            // (so no `splitBeforeRow > 0` fixture is added just to be skipped). A
            // value-boundary resume becomes a `_rid`-aware discard on the single child.
            let resume_discard: Option<PendingDiscard> = match scenario.checkpoint.as_ref() {
                None => None,
                Some(cp)
                    if cp.get("splitBeforeRow").is_some()
                        || cp.get("replacementRanges").is_some()
                        || cp.get("splitReplacementRanges").is_some() =>
                {
                    continue;
                }
                Some(cp) => match (
                    cp.get("resumeValues"),
                    cp.get("lastRid").and_then(|v| v.as_str()),
                ) {
                    (Some(values), Some(last_rid)) => {
                        let resume_values: Vec<OrderByResumeValue> =
                            serde_json::from_value(values.clone())
                                .expect("checkpoint.resumeValues must parse as OrderByResumeValue");
                        let boundary_keys: Vec<OrderByItem> = resume_values
                            .iter()
                            .map(OrderByResumeValue::to_scalar_order_by_item)
                            .collect::<Option<Vec<_>>>()
                            .expect("mock-harness resume scenarios use scalar boundaries");
                        ran_a_resume_checkpoint = true;
                        Some(PendingDiscard::ResumeBoundary {
                            boundary_keys,
                            last_rid: last_rid.to_owned(),
                            directions: directions.clone(),
                        })
                    }
                    // A checkpoint that only affects token minting leaves
                    // the drain order unchanged: drive the full fresh drain.
                    _ => None,
                },
            };
            ran_at_least_one = true;

            let mut children = Vec::new();
            for (idx, partition) in mock.partitions.iter().enumerate() {
                let pages: Vec<crate::error::Result<PageResult>> = partition
                    .pages
                    .iter()
                    .map(|page_value| {
                        let continuation = page_value
                            .get("continuation")
                            .and_then(|c| c.as_str())
                            .map(str::to_owned);
                        // Translate the fixture's `rid` field to the wire
                        // envelope's `_rid`.
                        let documents: Vec<serde_json::Value> = page_value["rows"]
                            .as_array()
                            .cloned()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|mut row| {
                                if let Some(obj) = row.as_object_mut() {
                                    if let Some(rid) = obj.remove("rid") {
                                        obj.insert("_rid".to_owned(), rid);
                                    }
                                }
                                row
                            })
                            .collect();
                        let body = serde_json::json!({
                            "_rid": "",
                            "Documents": documents,
                            "_count": documents.len(),
                        });
                        Ok(PageResult::Page {
                            response: mocks::response_with_continuation(
                                &serde_json::to_vec(&body).unwrap(),
                                continuation.as_deref(),
                            ),
                            is_terminal: continuation.is_none(),
                        })
                    })
                    .collect();
                let lo = format!("{:02x}", idx * 0x10);
                let hi = format!("{:02x}", (idx + 1) * 0x10);
                children.push(ChildStream::fresh(
                    range(&lo, &hi),
                    Box::new(MockLeaf::with_pages(pages)),
                ));
            }

            if let Some(discard) = resume_discard {
                // Value-boundary resume scenarios in the catalog are
                // single-range (the boundary belongs to one range).
                assert_eq!(
                    children.len(),
                    1,
                    "scenario {} declares a value-boundary resume but has {} partitions",
                    scenario.id,
                    children.len(),
                );
                children[0].pending_discard = discard;
            }

            let mut node = merge(children, directions);
            let mut collected_ids = Vec::new();
            loop {
                let mut executor = mocks::NoopRequestExecutor;
                let mut topology = mocks::NoopTopologyProvider;
                let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
                match node
                    .next_page(&mut context)
                    .await
                    .unwrap_or_else(|e| panic!("scenario {} failed: {e:?}", scenario.id))
                {
                    PageResult::Page {
                        response,
                        is_terminal,
                    } => {
                        collected_ids.extend(ids(&response));
                        if is_terminal {
                            break;
                        }
                    }
                    PageResult::Drained => break,
                    PageResult::SplitRequired { .. } => {
                        panic!("scenario {} unexpectedly required a split", scenario.id)
                    }
                }
            }

            assert_eq!(
                collected_ids, scenario.expected_ids,
                "scenario {} drained ids do not match expectedIds",
                scenario.id,
            );
        }
        assert!(
            ran_at_least_one,
            "no catalog scenario matched the mockPipeline filter; \
             the catalog-driven test wiring is broken"
        );
        assert!(
            ran_a_resume_checkpoint,
            "expected at least one value-boundary resume checkpoint scenario \
             (e.g. equal_key_resume_requiring_skip_count) to run in the mock harness"
        );
    }
}
