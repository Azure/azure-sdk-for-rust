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
//! resume-filtered) by reissuing the query with the .NET-compatible
//! structured `resumeFilter` field (see
//! [`super::query_response::with_resume_filter`]) and discarding
//! already-emitted rows client-side (see
//! [`super::order_by::classify_row_vs_boundary`]).
//!
//! A backend continuation is opaque and bound to the exact request body
//! that minted it, so one captured for a resume-filtered request is never
//! persisted or reused (see [`ChildQueryShape`]). On a live split,
//! [`Request::split_for_topology_change`] forwards the split child's backend
//! continuation into every replacement leaf. The Cosmos contract (also relied
//! on by [`super::SequentialDrain`]) is that a parent partition's continuation
//! stays valid on each post-split child under EPK scoping, so a replacement
//! resumes *after* every row the split child already emitted.
//! [`StreamingOrderedMerge::handle_split`] therefore forwards only the split
//! child's `last_emitted` bookkeeping (for `skip_count` accumulation and
//! future snapshots) and installs *no* client-side discard on the first
//! replacement page — reapplying one would wrongly drop later JOIN rows that
//! share the boundary `(key, _rid)`. This holds for scalar and complex
//! boundaries alike. A **saved-token** resume across a split instead rebuilds
//! each range through the structured `resumeFilter` (see
//! [`build_value_boundary_child`]), whose backend `DistinctHash` seek handles
//! scalar and complex boundaries alike. A replacement that carries no usable
//! continuation yet inherits an emitted boundary (a resume-filtered range that
//! split before its first page, or a generic non-`Request` node) is rebuilt
//! via that boundary discard when its shape allows, else rejected with a typed
//! `CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID` rather than guessing at
//! an unknown stream's position.

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::value::RawValue;

use crate::models::{CosmosOperation, FeedRange, MaxItemCountHint};

use super::order_by::{
    classify_row_vs_boundary, compare_key_tuples, compare_rids, OrderByItem, OrderByResumeValue,
    RowVsBoundary,
};
use super::query_plan::SortOrder;
use super::query_response::{self, PageAggregator};
use super::snapshot::{OrderByRangeToken, ValueBoundary};
use super::{
    intersect_feed_ranges, PageResult, PipelineContext, PipelineNode, PipelineNodeState, Request,
    RequestTarget, ResolvedRange,
};

/// Default emitted-page size when no `max_item_count` hint is set
/// (matches Cosmos's default; independent of backend per-child page size).
const DEFAULT_MAX_ITEM_COUNT: usize = 100;

/// Maximum consecutive split retries per child before giving up (mirrors
/// `SequentialDrain`/`UnorderedMerge`).
const MAX_SPLIT_RETRIES: usize = 10;

/// A range's last-emitted key tuple (bounded resume-value form), RID, and a
/// `_rid`-tie `skip_count`. `skip_count` is the number of consecutive rows
/// emitted so far that share this exact `(resume_values, rid)` — a JOIN (or
/// array unwind) can emit several result rows from one document with the same
/// `_rid` and sort key, so the count is needed to skip precisely those
/// already-emitted duplicates on resume (mirrors .NET's
/// `OrderByContinuationToken.SkipCount`). It is always `>= 1` once any row is
/// recorded.
#[derive(Clone)]
struct LastEmitted {
    resume_values: Vec<OrderByResumeValue>,
    rid: String,
    skip_count: u32,
}

/// What a child's next fetched page must discard before its rows become
/// visible to the merge, right after resuming a value boundary.
enum PendingDiscard {
    /// Nothing to discard — a fresh start or a plain continuation resume.
    None,
    /// Discard every row at or before the resume boundary, applying the same
    /// three-phase `(sort key, _rid, skip_count)` seek .NET's
    /// `FilterNextAsync` uses:
    ///
    /// 1. **sort key** — keys strictly before the boundary (per
    ///    [`classify_row_vs_boundary`]) are dropped; a key strictly after
    ///    stops the discard.
    /// 2. **`_rid`** — within a full-key tie, a row whose `_rid` is strictly
    ///    before `last_rid` in the first column's direction (numeric
    ///    document-ordinal order via [`compare_rids`], matching the backend)
    ///    is dropped; a `_rid` strictly after stops the discard.
    /// 3. **`skip_count`** — for the exact `(boundary key, last_rid)` group,
    ///    drop exactly `skip_count` rows (the already-emitted JOIN
    ///    duplicates), then stop so the remaining rows of that group emit.
    ///
    /// A per-row predicate, so it stays correct across pages and split
    /// sub-ranges even when the backend's structured `resumeFilter` already
    /// trims most of the prefix. `skip_count` is decremented as matching rows
    /// are dropped and persists across empty/partial pages until consumed.
    ResumeBoundary {
        resume_values: Vec<OrderByResumeValue>,
        last_rid: String,
        skip_count: u32,
        directions: Vec<SortOrder>,
    },
}

impl PendingDiscard {
    fn apply(&mut self, rows: &mut VecDeque<query_response::EnvelopeRow>) {
        match self {
            PendingDiscard::None => {}
            PendingDiscard::ResumeBoundary {
                resume_values,
                last_rid,
                skip_count,
                directions,
            } => {
                // The first sort column governs the `_rid` tie direction
                // (matching the backend within a full-key tie run).
                let rid_direction = directions.first().copied().unwrap_or(SortOrder::Ascending);
                while let Some(front) = rows.front() {
                    let discard =
                        match classify_row_vs_boundary(&front.keys, resume_values, directions) {
                            // Phase 1: sorts strictly before the boundary key.
                            RowVsBoundary::Before => true,
                            // Phase 1: at/after the boundary key (or an
                            // indeterminate complex column) — nothing left to
                            // discard.
                            RowVsBoundary::AfterOrIndeterminate => false,
                            RowVsBoundary::Tie => {
                                match compare_rids(&front.rid, last_rid, rid_direction) {
                                    // Phase 2: `_rid` strictly before the boundary.
                                    Ordering::Less => true,
                                    // Phase 2: `_rid` strictly after the boundary.
                                    Ordering::Greater => false,
                                    // Phase 3: exact `(key, _rid)` group — drop
                                    // exactly `skip_count` already-emitted
                                    // duplicates, then keep the rest.
                                    Ordering::Equal => {
                                        if *skip_count > 0 {
                                            *skip_count -= 1;
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                }
                            }
                        };
                    if !discard {
                        // Rows are sorted, so every later row is past the boundary too.
                        *self = PendingDiscard::None;
                        return;
                    }
                    rows.pop_front();
                }
                // Boundary not reached this page (empty page, or a tie/skip run
                // spanning pages): keep the discard active — with any remaining
                // `skip_count` — for the next page.
            }
        }
    }
}

/// Which request body a child's [`Request`] is executing, so
/// [`StreamingOrderedMerge::snapshot_state`] knows whether its backend
/// continuation (opaque, bound to the exact request body) is portable to
/// the plain-continuation resume path.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ChildQueryShape {
    /// Runs the plain query body (no `resumeFilter`); continuation may be
    /// snapshotted.
    Plain,
    /// Runs the query body with a structured `resumeFilter` injected;
    /// continuation is bound to that body and must never be stored — resumes
    /// from its value boundary instead.
    ResumeFilterInjected,
}

/// One still-active child stream: an owned EPK range, a leaf [`Request`]
/// executing the query for that range, and buffered rows from its current
/// backend page. `pub(super)` only so `planner` can hold the opaque
/// `Vec<ChildStream>` [`build_children`] returns.
pub(super) struct ChildStream {
    range: FeedRange,
    node: Box<dyn PipelineNode>,
    buffered: VecDeque<query_response::EnvelopeRow>,
    /// `true` once a terminal (no-continuation) page was observed.
    drained: bool,
    last_emitted: Option<LastEmitted>,
    pending_discard: PendingDiscard,
    /// Which request body `node` is executing (see [`ChildQueryShape`]).
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
            pending_discard: PendingDiscard::None,
            // Overridden to `ResumeFilterInjected` only by
            // `build_value_boundary_child`.
            query_shape: ChildQueryShape::Plain,
        }
    }

    /// Records the just-emitted row as the new resume boundary, tracking the
    /// `_rid`-tie `skip_count`: consecutive rows sharing the exact
    /// `(resume_values, rid)` increment the count (a JOIN duplicate of the
    /// same document), and any change to the key tuple or `_rid` resets it to
    /// `1`. Returns a typed error only on the (practically unreachable) u32
    /// overflow, so a boundary is never silently truncated.
    fn record_emission(&mut self, keys: &[OrderByItem], rid: &str) -> crate::error::Result<()> {
        let resume_values: Vec<OrderByResumeValue> =
            keys.iter().map(OrderByItem::to_resume_value).collect();
        let skip_count = match &self.last_emitted {
            Some(last) if last.rid == rid && last.resume_values == resume_values => {
                last.skip_count.checked_add(1).ok_or_else(|| {
                    crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID,
                        )
                        .with_message(
                            "ORDER BY resume skip_count overflowed u32 (a single document \
                             produced more than u32::MAX result rows sharing one _rid)",
                        )
                        .build()
                })?
            }
            // First row overall, or a new key/_rid group: this row is the
            // first (and so far only) emission of its `(key, _rid)` group.
            _ => 1,
        };
        self.last_emitted = Some(LastEmitted {
            resume_values,
            rid: rid.to_owned(),
            skip_count,
        });
        Ok(())
    }

    fn boundary(&self) -> Option<ValueBoundary> {
        self.last_emitted.as_ref().map(|le| ValueBoundary {
            resume_values: le.resume_values.clone(),
            last_rid: le.rid.clone(),
            skip_count: le.skip_count,
        })
    }
}

/// Streams cross-partition `ORDER BY` results in globally sorted order
/// (see module docs for the merge algorithm and resume model).
pub(crate) struct StreamingOrderedMerge {
    /// The per-range operation with no `resumeFilter` (its `query` text
    /// already placeholder-substituted with `true`) — the base every
    /// child's `Request` starts from. A resumed range injects a structured
    /// `resumeFilter` into a clone of this body.
    plain_operation: Arc<CosmosOperation>,
    directions: Vec<SortOrder>,
    children: Vec<ChildStream>,
}

impl StreamingOrderedMerge {
    pub(super) fn new(
        plain_operation: Arc<CosmosOperation>,
        directions: Vec<SortOrder>,
        children: Vec<ChildStream>,
    ) -> Self {
        Self {
            plain_operation,
            directions,
            children,
        }
    }

    fn max_item_count(&self) -> usize {
        match self.plain_operation.request_headers().max_item_count {
            Some(MaxItemCountHint::Limit(n)) => n.get() as usize,
            Some(MaxItemCountHint::ServerDecides) | None => DEFAULT_MAX_ITEM_COUNT,
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
                PageResult::SplitRequired { replacement_nodes } => {
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
                    self.handle_split(idx, replacement_nodes)?;
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

    /// Handles a child's `SplitRequired` by consuming the `replacement_nodes`
    /// the child produced, keeping the merge ignorant of their concrete type.
    ///
    /// The split child provides one replacement leaf per post-split sub-range;
    /// this validates each carries a [`feed_range`](PipelineNode::feed_range),
    /// sorts them, and confirms they exactly tile the split child's scope with
    /// no gaps or overlaps, then wraps each in a [`ChildStream`] that inherits
    /// the split child's resume state (see [`wrap_split_replacement`]). No
    /// topology is re-resolved here — the split child already did that when
    /// producing the nodes, forwarding its backend continuation into each
    /// replacement so they resume past every already-emitted row.
    fn handle_split(
        &mut self,
        idx: usize,
        replacement_nodes: Vec<Box<dyn PipelineNode>>,
    ) -> crate::error::Result<()> {
        let scope = self.children[idx].range.clone();
        let prior_boundary = self.children[idx].boundary();
        let query_shape = self.children[idx].query_shape;

        // Every replacement must own an EPK sub-range; a missing one would
        // make coverage unverifiable, so reject rather than risk a gap.
        let mut replacements: Vec<(FeedRange, Box<dyn PipelineNode>)> =
            Vec::with_capacity(replacement_nodes.len());
        for node in replacement_nodes {
            let range = node
                .feed_range()
                .ok_or_else(|| {
                    split_replacement_invalid(
                        "StreamingOrderedMerge split replacement node has no feed_range",
                    )
                })?
                .clone();
            replacements.push((range, node));
        }
        replacements.sort_by(|a, b| a.0.min_inclusive().cmp(b.0.min_inclusive()));
        validate_exact_coverage(&scope, replacements.iter().map(|(range, _)| range))?;

        // Wrap each replacement before mutating `self.children`, so a rejected
        // replacement leaves the merge unchanged rather than half-spliced.
        let mut wrapped = Vec::with_capacity(replacements.len());
        for (range, node) in replacements {
            wrapped.push(wrap_split_replacement(
                range,
                node,
                prior_boundary.as_ref(),
                query_shape,
                &self.directions,
            )?);
        }

        self.children.remove(idx);
        for (i, child) in wrapped.into_iter().enumerate() {
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
            self.children[winner].record_emission(&row.keys, &row.rid)?;
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
            ranges,
        })
    }

    fn topology_can_change(&self) -> bool {
        // Splits are handled internally (`handle_split`); no parent needed.
        false
    }
}

/// Builds the child streams needed to cover `scope`, given its topology
/// resolution and prior resume state (`None` for a fresh range). Shared
/// by planner construction and the live split handler. `directions` seed
/// the client-side discard's boundary comparison.
///
/// - Unchanged topology + `prior_continuation`: replays it as-is (safe —
///   see `snapshot_state` / [`ChildQueryShape`]).
/// - Else + `prior_boundary`: rebuilds via `build_value_boundary_child`,
///   which sends the boundary as a structured `resumeFilter`. This is a
///   per-row seek, so it stays correct across a split for scalar and
///   complex boundaries alike.
/// - Else: a fresh unfiltered start. On a topology change, resolved
///   ranges are clipped to `scope` and must exactly tile it.
pub(super) fn build_children(
    resolved: &[ResolvedRange],
    scope: &FeedRange,
    plain_operation: &Arc<CosmosOperation>,
    directions: &[SortOrder],
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
            // last-emitted boundary forward for the next snapshot.
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
                    skip_count: boundary.skip_count,
                });
            }
            child
        } else if let Some(boundary) = prior_boundary {
            // Rows were emitted but no safe continuation was saved (a
            // mid-page checkpoint); a plain restart would re-emit them.
            build_value_boundary_child(
                scope.clone(),
                target,
                plain_operation,
                directions,
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
    validate_exact_coverage(scope, clipped.iter().map(|(range, _)| range))?;

    // Every sub-range resumes from the same last-emitted boundary via a
    // structured `resumeFilter` (a per-row seek), so a split needs no
    // per-range row-count attribution — scalar and complex boundaries alike
    // are split-safe.
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
                build_value_boundary_child(owned, target, plain_operation, directions, boundary)?
            }
        };
        children.push(child);
    }
    Ok(children)
}

/// Returns `Ok(())` if `ranges` (yielded in ascending `min_inclusive` order,
/// each already clipped to `scope`) exactly tiles `scope` end-to-end with no
/// gaps or overlaps. Shared by the planner's split/merge resume path
/// ([`build_children`]) and the live split handler
/// ([`StreamingOrderedMerge::handle_split`]).
fn validate_exact_coverage<'a>(
    scope: &FeedRange,
    ranges: impl Iterator<Item = &'a FeedRange>,
) -> crate::error::Result<()> {
    let mut cursor = scope.min_inclusive().clone();
    let mut saw_any = false;
    for owned in ranges {
        saw_any = true;
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
    if !saw_any {
        return Err(split_replacement_invalid(
            "topology resolution produced no replacement ranges",
        ));
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

/// Wraps one split-replacement leaf into a [`ChildStream`] carrying the split
/// child's resume state. `query_shape` is copied from the split child so a
/// resume-filtered range's continuation is never persisted or reused.
///
/// `prior_boundary` is the split child's current boundary (`None` if it never
/// emitted a row):
///
/// - `None`: a fresh start — no rows emitted before the split, nothing to
///   discard.
/// - `Some` + the leaf reports a `Request` with a forwarded backend
///   continuation: trust it. [`Request::split_for_topology_change`] carried the
///   split child's continuation into the replacement, so it already resumes
///   *after* every emitted row. Only `last_emitted` is forwarded (for
///   `skip_count` accumulation and future snapshots) — installing a discard
///   here would wrongly drop later JOIN rows sharing the boundary
///   `(key, _rid)`.
/// - `Some` + no forwarded continuation + a resume-filtered range: rebuild the
///   [`PendingDiscard::ResumeBoundary`] from the boundary, exactly like a
///   saved-token resume. Such a leaf re-runs the structured `resumeFilter`
///   (the backend seeks to the boundary), so the discard only trims the
///   already-emitted prefix — safe for scalar and complex boundaries.
/// - `Some` + no forwarded continuation + a plain range (or generic node):
///   the leaf's resume position is unknown (a plain replay would re-fetch from
///   the start, which the client discard can't order for a complex boundary),
///   so reject with a typed error rather than guess. For a real plain
///   `Request` this is unreachable — a plain child that had emitted a row is
///   always `Continuing` when it splits, so its replacement always carries a
///   forwarded continuation.
fn wrap_split_replacement(
    range: FeedRange,
    node: Box<dyn PipelineNode>,
    prior_boundary: Option<&ValueBoundary>,
    query_shape: ChildQueryShape,
    directions: &[SortOrder],
) -> crate::error::Result<ChildStream> {
    let mut child = ChildStream::fresh(range, node);
    child.query_shape = query_shape;

    let Some(boundary) = prior_boundary else {
        // Fresh start: no rows emitted before the split, nothing to discard.
        return Ok(child);
    };

    let forwarded_continuation = matches!(
        child.node.snapshot_state()?,
        PipelineNodeState::Request {
            server_continuation: Some(_),
        }
    );
    if forwarded_continuation {
        // The forwarded continuation positions the replacement past every
        // emitted row; carry `last_emitted` only (no client discard).
        child.last_emitted = Some(LastEmitted {
            resume_values: boundary.resume_values.clone(),
            rid: boundary.last_rid.clone(),
            skip_count: boundary.skip_count,
        });
        return Ok(child);
    }

    // No forwarded continuation but the split child had emitted rows. A
    // resume-filtered replacement re-seeks the backend to the boundary, so
    // rebuild the discard to trim the already-emitted prefix (scalar or
    // complex). A plain (or generic) replacement's position is unknown; reject.
    if query_shape != ChildQueryShape::ResumeFilterInjected {
        return Err(split_replacement_invalid(
            "StreamingOrderedMerge split replacement carries no continuation to \
             resume a mid-group boundary and cannot be repositioned",
        ));
    }
    child.last_emitted = Some(LastEmitted {
        resume_values: boundary.resume_values.clone(),
        rid: boundary.last_rid.clone(),
        skip_count: boundary.skip_count,
    });
    child.pending_discard = PendingDiscard::ResumeBoundary {
        resume_values: boundary.resume_values.clone(),
        last_rid: boundary.last_rid.clone(),
        skip_count: boundary.skip_count,
        directions: directions.to_vec(),
    };
    Ok(child)
}

fn split_replacement_invalid(
    message: impl Into<std::borrow::Cow<'static, str>>,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID)
        .with_message(message)
        .build()
}

/// Builds one child via the value-boundary resume path: sends the
/// last-emitted boundary to the backend as a structured `resumeFilter`
/// (`rid` present, `exclude:false`) injected into a clone of the plain
/// query body, and installs a matching [`PendingDiscard::ResumeBoundary`]
/// guard so the already-emitted prefix of the boundary tie run is trimmed
/// client-side. Works for scalar and complex (array/object) boundaries
/// alike — the backend seek is a per-row predicate, so it stays correct
/// across a split.
fn build_value_boundary_child(
    owned_range: FeedRange,
    target: RequestTarget,
    plain_operation: &Arc<CosmosOperation>,
    directions: &[SortOrder],
    boundary: &ValueBoundary,
) -> crate::error::Result<ChildStream> {
    let body = query_response::with_resume_filter(
        plain_operation.body(),
        &boundary.resume_values,
        Some(&boundary.last_rid),
        false,
    )?;
    let operation = Arc::new((*Arc::clone(plain_operation)).clone().with_body(body));

    let mut child =
        ChildStream::fresh(owned_range, Box::new(Request::new(operation, target, None)));
    child.last_emitted = Some(LastEmitted {
        resume_values: boundary.resume_values.clone(),
        rid: boundary.last_rid.clone(),
        skip_count: boundary.skip_count,
    });
    child.pending_discard = PendingDiscard::ResumeBoundary {
        resume_values: boundary.resume_values.clone(),
        last_rid: boundary.last_rid.clone(),
        skip_count: boundary.skip_count,
        directions: directions.to_vec(),
    };
    // The continuation is bound to the resume-filtered body; never snapshot it.
    child.query_shape = ChildQueryShape::ResumeFilterInjected;
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

    /// A single-column complex (array) envelope backend `CosmosResponse`:
    /// each row's sort key is the array `[value]`. Shared by
    /// [`array_envelope_page`] (MockLeaf pages) and `MockRequestExecutor`
    /// replies for the resume-filtered split test.
    fn array_envelope_response(
        rows: &[(&str, i64)],
        continuation: Option<&str>,
    ) -> crate::models::CosmosResponse {
        let documents: Vec<serde_json::Value> = rows
            .iter()
            .map(|(rid, value)| {
                serde_json::json!({
                    "_rid": rid,
                    "orderByItems": [{"item": [value]}],
                    "payload": {"id": rid},
                })
            })
            .collect();
        let body =
            serde_json::json!({"_rid": "", "Documents": documents, "_count": documents.len()});
        mocks::response_with_continuation(&serde_json::to_vec(&body).unwrap(), continuation)
    }

    /// A single-column complex (array) envelope page: each row's sort key is
    /// the array `[value]`, exercising the hash-based boundary discard.
    fn array_envelope_page(
        rows: &[(&str, i64)],
        continuation: Option<&str>,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: array_envelope_response(rows, continuation),
            is_terminal: continuation.is_none(),
        })
    }

    /// A JOIN-shaped envelope backend `CosmosResponse`: `(rid, rank, id)` rows
    /// where several rows can share one `_rid` (a single document expanded by a
    /// JOIN) while carrying distinct payload `id`s. Shared by
    /// [`join_envelope_page`] (MockLeaf pages) and `MockRequestExecutor` replies
    /// for the live-split forwarded-continuation test.
    fn join_envelope_response(
        rows: &[(&str, i64, &str)],
        continuation: Option<&str>,
    ) -> crate::models::CosmosResponse {
        let documents: Vec<serde_json::Value> = rows
            .iter()
            .map(|(rid, rank, id)| {
                serde_json::json!({
                    "_rid": rid,
                    "orderByItems": [{"item": rank}],
                    "payload": {"id": id},
                })
            })
            .collect();
        let body =
            serde_json::json!({"_rid": "", "Documents": documents, "_count": documents.len()});
        mocks::response_with_continuation(&serde_json::to_vec(&body).unwrap(), continuation)
    }

    /// A JOIN-shaped envelope page: `(rid, rank, id)` rows where several rows
    /// can share one `_rid` (a single document expanded by a JOIN) while
    /// carrying distinct payload `id`s. Exercises the `skip_count` phase of the
    /// resume discard, which a local emulator cannot produce since it does not
    /// execute JOINs.
    fn join_envelope_page(
        rows: &[(&str, i64, &str)],
        continuation: Option<&str>,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: join_envelope_response(rows, continuation),
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
        StreamingOrderedMerge::new(Arc::new(mocks::operation()), directions, children)
    }

    async fn next_page(node: &mut StreamingOrderedMerge) -> PageResult {
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        node.next_page(&mut context).await.unwrap()
    }

    /// A `MockLeaf` split page carrying `replacement_nodes` — the leaves the
    /// merge splices in and wraps with the split child's resume state. Using
    /// non-`Request` (`MockLeaf`) replacements proves the merge consumes the
    /// supplied nodes directly, staying ignorant of their concrete type.
    fn split_page(
        replacement_nodes: Vec<Box<dyn PipelineNode>>,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::SplitRequired { replacement_nodes })
    }

    /// A `MockLeaf` split-replacement leaf covering `[min, max)` with pre-set
    /// pages, reported through [`PipelineNode::feed_range`] so the merge can
    /// verify the replacements tile the split child's scope. Its default
    /// snapshot is `Drained` (no forwarded continuation), so it models a
    /// *generic* replacement whose resume position the merge cannot trust —
    /// see [`positioned_replacement_leaf`] for the common live-split case.
    fn replacement_leaf(
        min: &str,
        max: &str,
        pages: Vec<crate::error::Result<PageResult>>,
    ) -> Box<dyn PipelineNode> {
        Box::new(MockLeaf::with_pages(pages).with_feed_range(range(min, max)))
    }

    /// A split-replacement leaf that reports a `Request` snapshot carrying a
    /// forwarded backend continuation, modeling what
    /// [`Request::split_for_topology_change`] hands the merge on a real live
    /// split. `handle_split` reads this snapshot to confirm the replacement
    /// already resumes past every emitted row, so it installs no client-side
    /// discard.
    fn positioned_replacement_leaf(
        min: &str,
        max: &str,
        pages: Vec<crate::error::Result<PageResult>>,
    ) -> Box<dyn PipelineNode> {
        Box::new(
            MockLeaf::with_pages(pages)
                .with_feed_range(range(min, max))
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("split-forwarded-ct".to_owned()),
                }),
        )
    }

    /// Drains `node` to completion against a no-op executor/topology (a merge
    /// handles splits internally by consuming replacement nodes, so no live
    /// topology resolution is needed), returning every emitted id in order.
    async fn drain_all_ids(node: &mut StreamingOrderedMerge) -> Vec<String> {
        let mut all = Vec::new();
        loop {
            match next_page(node).await {
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
    /// before `10, 20`. A large page cap keeps popping within one page. The
    /// replacements carry a forwarded continuation (positioned past the `2`
    /// boundary), so the merge wraps and orders them without re-resolving and
    /// with no client-side discard.
    #[tokio::test]
    async fn split_during_pop_loop_primes_all_split_replacements() {
        let p0 = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                // The split child yields two sub-range leaves that carry the
                // forwarded continuation, so their pages already start past the
                // `2` boundary.
                split_page(vec![
                    positioned_replacement_leaf("", "40", vec![envelope_page(&[("d3", 3)], None)]),
                    positioned_replacement_leaf(
                        "40",
                        "80",
                        vec![envelope_page(&[("d10", 10), ("d20", 20)], None)],
                    ),
                ]),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("d50", 50)],
                None,
            )])),
        );
        let mut node = merge(vec![p0, p1], vec![SortOrder::Ascending]);

        let emitted = drain_all_ids(&mut node).await;
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
                split_page(vec![
                    positioned_replacement_leaf("", "40", vec![envelope_page(&[("d3", 3)], None)]),
                    positioned_replacement_leaf(
                        "40",
                        "80",
                        vec![envelope_page(&[("d10", 10), ("d20", 20)], None)],
                    ),
                ]),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("d50", 50)],
                None,
            )])),
        );
        let mut node = merge(vec![p0, p1], vec![SortOrder::Ascending]);
        node.plain_operation = Arc::new((*node.plain_operation).clone().with_max_item_count(
            crate::models::MaxItemCountHint::Limit(std::num::NonZeroU32::new(3).unwrap()),
        ));

        // Page 1 stops right after the split, at the cap.
        let page1 = next_page(&mut node).await;
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
        // with P0b carrying the forwarded resume boundary for its snapshot.
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
        let page2 = next_page(&mut node).await;
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

    /// A split replacement may itself split before it yields a row (cascading
    /// split). Each level supplies its own replacement leaves, each carrying a
    /// forwarded continuation, so ordering and resume correctness hold no
    /// matter how deep the cascade goes — with no client-side discard.
    #[tokio::test]
    async fn cascading_split_replacements_preserve_order_and_boundary() {
        // P0 emits [1, 2] then splits into P0a + P0b; P0a splits *again* into
        // P0a1 (next 3) and P0a2 (next 5) before yielding a row.
        let p0a_cascade = split_page(vec![
            positioned_replacement_leaf("", "20", vec![envelope_page(&[("d3", 3)], None)]),
            positioned_replacement_leaf("20", "40", vec![envelope_page(&[("d5", 5)], None)]),
        ]);
        let p0 = ChildStream::fresh(
            range("", "80"),
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                split_page(vec![
                    positioned_replacement_leaf("", "40", vec![p0a_cascade]),
                    positioned_replacement_leaf(
                        "40",
                        "80",
                        vec![envelope_page(&[("d10", 10)], None)],
                    ),
                ]),
            ])),
        );
        let mut node = merge(vec![p0], vec![SortOrder::Ascending]);
        let emitted = drain_all_ids(&mut node).await;
        assert_eq!(
            emitted,
            vec!["d1", "d2", "d3", "d5", "d10"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            "rows from a cascaded (twice-split) sub-range stay globally ordered"
        );
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

    /// Helper: an all-scalar single-column resume-boundary discard. `skip_count`
    /// is the number of already-emitted rows sharing the exact `(value, rid)`
    /// (usually `1` for a non-JOIN boundary).
    fn number_boundary_discard(value: f64, last_rid: &str, skip_count: u32) -> PendingDiscard {
        PendingDiscard::ResumeBoundary {
            resume_values: vec![OrderByResumeValue::Number {
                value: value.into(),
            }],
            last_rid: last_rid.to_owned(),
            skip_count,
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
        child.pending_discard = number_boundary_discard(5.0, "tied-2", 1);
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
        child.pending_discard = number_boundary_discard(5.0, "tied-1", 1);
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
        child.pending_discard = number_boundary_discard(5.0, "tied-3", 1);
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
            c.pending_discard = number_boundary_discard(5.0, "c", 1);
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
            c.pending_discard = number_boundary_discard(5.0, "c", 1);
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

    /// A complex (array) boundary discards already-emitted ties via the
    /// hash-based comparison, keeping same-hash rows past the rid cut-off.
    /// This replaces the old positional rescan. (A differently-valued complex
    /// row orders by the backend's hash order, which a `MockLeaf` can't
    /// reproduce, so this exercises the tie run.)
    #[tokio::test]
    async fn resume_with_complex_boundary_discards_matching_hash_ties_by_rid() {
        let boundary =
            OrderByItem::Array(vec![OrderByItem::Number(5_i64.into())]).to_resume_value();
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![array_envelope_page(
                &[("tied-1", 5), ("tied-2", 5), ("tied-3", 5)],
                None,
            )])),
        );
        child.pending_discard = PendingDiscard::ResumeBoundary {
            resume_values: vec![boundary],
            last_rid: "tied-1".to_owned(),
            skip_count: 1,
            directions: vec![SortOrder::Ascending],
        };
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        // `tied-1` (== boundary hash, rid <= last_rid) is dropped; `tied-2`
        // and `tied-3` (same hash, rid > last_rid) survive.
        assert_eq!(
            ids(&response),
            vec!["tied-2".to_owned(), "tied-3".to_owned()]
        );
    }

    /// Regression for the complex-boundary discard: a *distinct* un-emitted
    /// complex value must never be dropped, no matter how its MurmurHash
    /// happens to order against the boundary's — hash order is not Cosmos
    /// sort order. (The earlier hash-order comparison could classify `[1]`
    /// as "before" the `[5]` boundary and silently drop it, ~50% of the
    /// time, causing data loss.)
    #[tokio::test]
    async fn resume_complex_boundary_never_drops_distinct_unemitted_value() {
        let boundary =
            OrderByItem::Array(vec![OrderByItem::Number(5_i64.into())]).to_resume_value();
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![array_envelope_page(
                &[("emitted", 5), ("distinct", 1)],
                None,
            )])),
        );
        child.pending_discard = PendingDiscard::ResumeBoundary {
            resume_values: vec![boundary],
            last_rid: "emitted".to_owned(),
            skip_count: 1,
            directions: vec![SortOrder::Ascending],
        };
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        // `emitted` (exact-hash tie, rid <= last_rid) is dropped; the
        // distinct array `[1]` is kept — never inferred "before" from hash.
        assert_eq!(ids(&response), vec!["distinct".to_owned()]);
    }

    // ── skip_count: JOIN duplicate-RID resume ────────────────────────────

    #[test]
    fn record_emission_tracks_skip_count_for_duplicate_key_rid() {
        let mut child = ChildStream::fresh(range("", "FF"), Box::new(MockLeaf::with_pages(vec![])));
        let key5 = [OrderByItem::Number(5_i64.into())];
        let key6 = [OrderByItem::Number(6_i64.into())];

        // First emission of (5, docA): skip_count starts at 1.
        child.record_emission(&key5, "docA").unwrap();
        assert_eq!(child.boundary().unwrap().skip_count, 1);
        // Same (5, docA) again (a JOIN duplicate of one document): increments.
        child.record_emission(&key5, "docA").unwrap();
        assert_eq!(child.boundary().unwrap().skip_count, 2);
        // A new key with the same rid resets the count.
        child.record_emission(&key6, "docA").unwrap();
        assert_eq!(child.boundary().unwrap().skip_count, 1);
        // A new rid with the same key also resets.
        child.record_emission(&key6, "docB").unwrap();
        let boundary = child.boundary().unwrap();
        assert_eq!(boundary.skip_count, 1);
        assert_eq!(boundary.last_rid, "docB");
    }

    #[tokio::test]
    async fn resume_skips_exactly_skip_count_duplicate_rid_rows() {
        // Resume at (rank=5, rid=docA) after emitting 2 of docA's JOIN rows.
        // The page re-returns all 3 docA rows plus a docB row; the discard
        // drops exactly 2 (the emitted duplicates) and keeps the third, then
        // the later document.
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![join_envelope_page(
                &[
                    ("docA", 5, "a1"),
                    ("docA", 5, "a2"),
                    ("docA", 5, "a3"),
                    ("docB", 6, "b1"),
                ],
                None,
            )])),
        );
        child.pending_discard = number_boundary_discard(5.0, "docA", 2);
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["a3".to_owned(), "b1".to_owned()]);
    }

    #[tokio::test]
    async fn resume_skip_count_persists_across_pages() {
        // skip_count = 3, but docA's JOIN rows straddle a page boundary: 2 on
        // the first page, 2 on the second. The discard must carry the residual
        // skip across the page break, dropping exactly 3 docA rows total.
        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![
                join_envelope_page(&[("docA", 5, "a1"), ("docA", 5, "a2")], Some("ct-mid")),
                join_envelope_page(
                    &[("docA", 5, "a3"), ("docA", 5, "a4"), ("docB", 6, "b1")],
                    None,
                ),
            ])),
        );
        child.pending_discard = number_boundary_discard(5.0, "docA", 3);
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["a4".to_owned(), "b1".to_owned()],
            "3 already-emitted docA duplicates dropped across the page break; a4 and b1 survive"
        );
    }

    #[tokio::test]
    async fn legacy_boundary_missing_skip_count_discards_boundary_row() {
        // A continuation token minted before `skip_count` existed omits the
        // field; it must deserialize as skip_count == 1 so the single boundary
        // row is still dropped on resume, never re-emitted.
        let boundary: ValueBoundary = serde_json::from_str(
            r#"{"resume_values":[{"type":"number","value":5.0}],"last_rid":"boundary"}"#,
        )
        .unwrap();
        assert_eq!(boundary.skip_count, 1);

        let mut child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![envelope_page(
                &[("boundary", 5), ("new", 6)],
                None,
            )])),
        );
        child.pending_discard = PendingDiscard::ResumeBoundary {
            resume_values: boundary.resume_values,
            last_rid: boundary.last_rid,
            skip_count: boundary.skip_count,
            directions: vec![SortOrder::Ascending],
        };
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["new".to_owned()]);
    }

    // ── Live-split: forwarded-continuation resume ───────────────────────

    /// Blocker regression: one JOIN document (rid `docA`, key 5) expands into
    /// 150 result rows. Page 1 emits rows #1–100; the next fetch live-splits.
    /// The replacement is a real `Request` in `Continuing` state carrying the
    /// forwarded backend continuation (Cosmos contract: a parent partition's
    /// continuation stays valid on a post-split child), so it resumes *after*
    /// row #100 and its first page starts at #101. No client discard is
    /// applied, so rows #101–150 all emit. The old code reinstalled the
    /// boundary discard with `skip_count = 100` and would have silently dropped
    /// all 50.
    #[tokio::test]
    async fn live_split_forwarded_continuation_emits_post_boundary_join_rows() {
        let id_1_100: Vec<String> = (1..=100).map(|i| format!("a{i}")).collect();
        let id_101_150: Vec<String> = (101..=150).map(|i| format!("a{i}")).collect();
        let page1_rows: Vec<(&str, i64, &str)> =
            id_1_100.iter().map(|s| ("docA", 5, s.as_str())).collect();
        let page2_rows: Vec<(&str, i64, &str)> =
            id_101_150.iter().map(|s| ("docA", 5, s.as_str())).collect();

        // The split child yields a real `Request` replacement carrying the
        // forwarded continuation, mirroring `split_for_topology_change`.
        let target = RequestTarget::effective_partition_key_range(
            range("", "FF"),
            "pk-0".to_owned(),
            range("", "FF"),
        );
        let replacement: Box<dyn PipelineNode> = Box::new(Request::new(
            Arc::new(mocks::operation()),
            target,
            Some("p0-ct".to_owned()),
        ));
        let split_child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![
                join_envelope_page(&page1_rows, Some("p0-ct")),
                split_page(vec![replacement]),
            ])),
        );
        let mut node = merge(vec![split_child], vec![SortOrder::Ascending]);

        // The replacement fetches once with the forwarded continuation; the
        // backend returns only rows #101–150, already past the emitted prefix.
        let mut executor =
            mocks::MockRequestExecutor::new(vec![Ok(join_envelope_response(&page2_rows, None))]);
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let PageResult::Page { response: r1, .. } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected a page");
        };
        assert_eq!(ids(&r1), id_1_100, "page 1 emits the first 100 JOIN rows");

        let PageResult::Page { response: r2, .. } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&r2),
            id_101_150,
            "rows #101–150 must all emit — none dropped by a stale skip_count discard"
        );

        assert_eq!(
            executor.continuation_calls,
            vec![Some("p0-ct".to_owned())],
            "the replacement resumed once from the forwarded continuation"
        );
    }

    /// A live split whose replacement reports no forwarded continuation, yet
    /// the split child had already emitted a row (boundary set), cannot be
    /// safely repositioned — a generic node's resume position is unknown.
    /// Reattaching the `skip_count` discard could drop or duplicate rows, so
    /// the merge rejects with a typed `SPLIT_REPLACEMENT_INVALID` error.
    #[tokio::test]
    async fn live_split_replacement_without_continuation_with_boundary_errors() {
        let split_child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![split_page(vec![
                replacement_leaf("", "80", vec![envelope_page(&[("d3", 3)], None)]),
                replacement_leaf("80", "FF", vec![envelope_page(&[("d9", 9)], None)]),
            ])])),
        );
        let mut node = merge(vec![split_child], vec![SortOrder::Ascending]);
        // The split child had emitted a row before splitting (a plain boundary).
        node.children[0].last_emitted = Some(LastEmitted {
            resume_values: vec![OrderByResumeValue::Number { value: 2.0.into() }],
            rid: "d2".to_owned(),
            skip_count: 1,
        });

        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let err = node
            .next_page(&mut context)
            .await
            .expect_err("a replacement with an unknown position must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
            "must surface the typed split-replacement-invalid error (20215), got: {err}"
        );
    }

    /// The companion accept case: a child that live-splits *before* emitting
    /// any row has no boundary to protect, so generic replacements (no
    /// forwarded continuation) are accepted and their rows stream through
    /// fresh, in order.
    #[tokio::test]
    async fn live_split_initial_no_boundary_accepts_generic_replacements() {
        let split_child = ChildStream::fresh(
            range("", "FF"),
            Box::new(MockLeaf::with_pages(vec![split_page(vec![
                replacement_leaf("", "80", vec![envelope_page(&[("d3", 3)], None)]),
                replacement_leaf("80", "FF", vec![envelope_page(&[("d9", 9)], None)]),
            ])])),
        );
        // `last_emitted` stays `None` — no row emitted before the split.
        let mut node = merge(vec![split_child], vec![SortOrder::Ascending]);
        let emitted = drain_all_ids(&mut node).await;
        assert_eq!(
            emitted,
            vec!["d3".to_owned(), "d9".to_owned()],
            "an initial split with no boundary accepts fresh replacements"
        );
    }

    /// A `ResumeFilterInjected` child that live-splits *before its first page*
    /// (so its replacements carry no forwarded continuation) is rebuilt via the
    /// boundary discard: each replacement re-runs the structured `resumeFilter`
    /// (the backend `DistinctHash` seek), so the merge reinstalls the boundary
    /// discard and orders the results. This is safe for a complex boundary
    /// because the backend seek already excluded the emitted rows. The
    /// replacement leaves are real `Request`s whose operation body carries that
    /// `resumeFilter`, mirroring what `Request::split_for_topology_change`
    /// clones on a live split.
    #[tokio::test]
    async fn resume_filter_injected_complex_boundary_live_split_is_accepted_and_ordered() {
        let boundary = complex_boundary("rid-1");
        let filtered_body = query_response::with_resume_filter(
            query_operation().body(),
            &boundary.resume_values,
            Some(&boundary.last_rid),
            false,
        )
        .expect("resume-filter body");
        let filtered_op = Arc::new((*query_operation()).clone().with_body(filtered_body));
        let replacement = |min: &'static str, max: &'static str| -> Box<dyn PipelineNode> {
            let target = RequestTarget::effective_partition_key_range(
                range(min, max),
                format!("pk-{min}-{max}"),
                range(min, max),
            );
            Box::new(Request::new(Arc::clone(&filtered_op), target, None))
        };
        // The split child (MockLeaf) yields two resume-filtered Request leaves.
        let split_child = {
            let mut c = ChildStream::fresh(
                range("", "80"),
                Box::new(MockLeaf::with_pages(vec![split_page(vec![
                    replacement("", "40"),
                    replacement("40", "80"),
                ])])),
            );
            c.query_shape = ChildQueryShape::ResumeFilterInjected;
            c.last_emitted = Some(LastEmitted {
                resume_values: boundary.resume_values.clone(),
                rid: boundary.last_rid.clone(),
                skip_count: boundary.skip_count,
            });
            c
        };
        let mut node = merge(vec![split_child], vec![SortOrder::Ascending]);

        // The backend seek already excluded emitted rows, so each replacement
        // returns only distinct, later complex values (d3 < d5).
        let mut executor = mocks::MockRequestExecutor::new(vec![
            Ok(array_envelope_response(&[("d3", 3)], None)),
            Ok(array_envelope_response(&[("d5", 5)], None)),
        ]);
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let page = node
            .next_page(&mut context)
            .await
            .expect("a resume-filtered complex-boundary split must be accepted, not rejected");
        let PageResult::Page { response, .. } = page else {
            panic!("expected a page");
        };
        assert_eq!(
            ids(&response),
            vec!["d3".to_owned(), "d5".to_owned()],
            "replacements must be accepted and ordered by key across the split"
        );
        // Both replacement requests carried the structured resumeFilter seek.
        assert!(
            executor.body_text(0).contains("resumeFilter"),
            "first replacement request must carry the resumeFilter body, got: {}",
            executor.body_text(0)
        );
        assert!(
            executor.body_text(1).contains("resumeFilter"),
            "second replacement request must carry the resumeFilter body, got: {}",
            executor.body_text(1)
        );
    }

    // ── build_children resume/topology paths ─────────────────────────────

    fn query_operation() -> Arc<CosmosOperation> {
        Arc::new(
            mocks::operation().with_body(
                br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec(),
            ),
        )
    }

    /// One ascending sort direction, matching what the planner derives from
    /// `QueryInfo::order_by`.
    const ASC: &[SortOrder] = &[SortOrder::Ascending];

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
            skip_count: 1,
        }
    }

    fn complex_boundary(last_rid: &str) -> ValueBoundary {
        ValueBoundary {
            resume_values: vec![
                OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value()
            ],
            last_rid: last_rid.to_owned(),
            skip_count: 1,
        }
    }

    /// A scalar boundary crossing a split fans out into one resume-filtered
    /// child per sub-range, each with the boundary discard installed.
    #[test]
    fn build_children_splits_scalar_boundary_into_resume_filtered_children() {
        let op = query_operation();
        let scope = range("", "FF");
        let resolved = vec![
            resolved_range("", "80", "pk-left"),
            resolved_range("80", "FF", "pk-right"),
        ];
        let boundary = scalar_boundary(5.0, "c");
        let children = build_children(&resolved, &scope, &op, ASC, None, Some(&boundary))
            .expect("scalar boundary resumes across a split");
        assert_eq!(children.len(), 2);
        for child in &children {
            assert!(
                matches!(child.pending_discard, PendingDiscard::ResumeBoundary { .. }),
                "each split sub-range must resume via the boundary discard"
            );
            assert_eq!(
                child.query_shape,
                ChildQueryShape::ResumeFilterInjected,
                "a resume-filtered child's continuation must never be snapshotted"
            );
        }
    }

    /// A complex (array/object) boundary now also fans out across a split via
    /// the structured `resumeFilter` — the old topology-change rejection is
    /// gone because the backend seek is a per-row predicate.
    #[test]
    fn build_children_splits_complex_boundary_into_resume_filtered_children() {
        let op = query_operation();
        let scope = range("", "FF");
        let resolved = vec![
            resolved_range("", "80", "pk-left"),
            resolved_range("80", "FF", "pk-right"),
        ];
        let boundary = complex_boundary("rid-1");
        let children = build_children(&resolved, &scope, &op, ASC, None, Some(&boundary))
            .expect("a complex boundary now resumes across a split");
        assert_eq!(children.len(), 2);
        for child in &children {
            assert!(matches!(
                child.pending_discard,
                PendingDiscard::ResumeBoundary { .. }
            ));
        }
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
        let children = build_children(&resolved, &scope, &op, ASC, None, Some(&boundary))
            .expect("a merged (widened) physical range clips to the saved scope");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].range, scope);
    }

    /// A complex boundary resumes across a merge (single clipped sub-range)
    /// via the structured `resumeFilter`.
    #[test]
    fn build_children_allows_complex_boundary_across_merge() {
        let op = query_operation();
        let scope = range("", "80");
        let resolved = vec![resolved_range("", "FF", "pk-merged")];
        let boundary = complex_boundary("rid-1");
        let children = build_children(&resolved, &scope, &op, ASC, None, Some(&boundary))
            .expect("a complex boundary resumes across a merge (single clipped range)");
        assert_eq!(children.len(), 1);
        assert!(matches!(
            children[0].pending_discard,
            PendingDiscard::ResumeBoundary { .. }
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
                skip_count: 1,
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
                        // A real boundary always emitted at least the boundary
                        // row, so a fixture that omits `skipCount` resumes as 1.
                        let skip_count = cp
                            .get("skipCount")
                            .and_then(|v| v.as_u64())
                            .map_or(1, |n| n as u32);
                        ran_a_resume_checkpoint = true;
                        Some(PendingDiscard::ResumeBoundary {
                            resume_values,
                            last_rid: last_rid.to_owned(),
                            skip_count,
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
