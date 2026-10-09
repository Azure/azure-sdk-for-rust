// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore unemitted unfetched rescan

//! Streaming cross-partition `ORDER BY` k-way merge node.
//!
//! [`StreamingOrderedMerge`] polls one leaf [`Request`] per active EPK range,
//! buffers one locally-sorted row per range, and repeatedly emits the
//! globally smallest buffered row (per [`compare_key_tuples`]) through a
//! min-heap of child indices. Only the consumed child is reinserted; topology
//! changes rebuild the heap after every replacement has a head row.
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
//! [`StreamingOrderedMerge::handle_split`] therefore forwards the split
//! child's `last_emitted` bookkeeping (for `skip_count` accumulation and
//! future snapshots) and installs no *fresh* client-side discard on the first
//! replacement page — rebuilding one from the boundary would wrongly drop
//! later JOIN rows that share the boundary `(key, _rid)`. An
//! already-armed discard is a separate matter and does carry over: it means a
//! resume prefix was only partially consumed, so its remaining (decremented)
//! skip still applies past the forwarded continuation. A **saved-token**
//! resume across a split instead rebuilds each range through the structured
//! `resumeFilter` (see [`build_value_boundary_child`]), whose backend seek
//! skips past the boundary. A replacement that carries no usable
//! continuation yet inherits an emitted boundary (a resume-filtered range that
//! split before its first page, or a generic non-`Request` node) is rebuilt
//! via that boundary discard when its shape allows, else rejected with a typed
//! `CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID` rather than guessing at
//! an unknown stream's position.

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::sync::Arc;

use super::recovery_diagnostics::RecoveryDiagnostics;
use async_trait::async_trait;

use crate::models::{CosmosOperation, FeedRange, ItemView, MaxItemCountHint, SessionToken};

use super::binary_heap;
use super::order_by::{
    classify_row_vs_boundary, compare_key_tuples, compare_rids, OrderByItem, OrderByResumeValue,
    RowVsBoundary,
};
use super::query_plan::SortOrder;
use super::query_response::{self, PageAggregator};
use super::snapshot::{OrderByRangeToken, ValueBoundary};
use super::{
    intersect_feed_ranges, split_replacement_invalid, validate_exact_coverage, PageResult,
    PipelineContext, PipelineNode, PipelineNodeState, Request, RequestTarget, ResolvedRange,
    SplitReplacements,
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
#[derive(Clone)]
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
    /// Drops the leading rows of a freshly fetched page that were already
    /// emitted before the resume point, per the three-phase seek documented on
    /// [`PendingDiscard::ResumeBoundary`]. Self-clearing: once a row past the
    /// boundary is seen the discard is spent and set to
    /// [`PendingDiscard::None`], otherwise it stays armed (carrying any
    /// remaining `skip_count`) so a run spanning several pages resolves
    /// correctly. A no-op for [`PendingDiscard::None`].
    fn apply(
        &mut self,
        rows: &mut VecDeque<query_response::EnvelopeRow>,
        rid_direction: SortOrder,
    ) {
        match self {
            PendingDiscard::None => {}
            PendingDiscard::ResumeBoundary {
                resume_values,
                last_rid,
                skip_count,
                directions,
            } => {
                while let Some(front) = rows.front() {
                    let discard =
                        match classify_row_vs_boundary(&front.keys, resume_values, directions) {
                            // Phase 1: sorts strictly before the boundary key.
                            RowVsBoundary::Before => true,
                            // Phase 1: at or after the boundary key — nothing
                            // left to discard.
                            RowVsBoundary::After => false,
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
        let resume_values: Vec<OrderByResumeValue> = keys
            .iter()
            .enumerate()
            .map(|(column, key)| {
                key.to_resume_value()
                    .ok_or_else(|| super::order_by::complex_order_by_error(column, key.type_name()))
            })
            .collect::<crate::error::Result<_>>()?;
        let skip_count = match &self.last_emitted {
            Some(last) if last.rid == rid && last.resume_values == resume_values => {
                last.skip_count.checked_add(1).ok_or_else(|| {
                    crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID,
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

    /// Fetches until this stream has a buffered head row or is proven drained,
    /// absorbing each page into `aggregator` and applying any
    /// [`PendingDiscard`] to the first page. An empty page that still carries a
    /// continuation is not terminal, so it is re-polled.
    ///
    /// A split is reported back rather than handled here: resolving one splices
    /// replacements into the merge's `children`, which a single stream cannot
    /// do. See [`StreamingOrderedMerge::handle_split`].
    async fn ensure_filled(
        &mut self,
        context: &mut PipelineContext<'_>,
        aggregator: &mut PageAggregator,
        directions: &[SortOrder],
        recovery: &mut RecoveryDiagnostics,
        #[cfg(test)] retention: Option<tests::RetentionSample>,
    ) -> crate::error::Result<FillOutcome> {
        loop {
            if !self.buffered.is_empty() || self.drained {
                return Ok(FillOutcome::Filled);
            }

            match self.node.next_page(context).await? {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    #[cfg(test)]
                    if let (Some(retention), crate::models::ResponseBody::Bytes(page)) =
                        (&retention, response.body())
                    {
                        retention.record(Some(page));
                    }
                    let validated = aggregator.absorb_session_token(&response).and_then(|()| {
                        let rows: VecDeque<query_response::EnvelopeRow> =
                            query_response::parse_envelope_page(response.body(), directions.len())?
                                .into();
                        let fallback = directions.first().copied().unwrap_or(SortOrder::Ascending);
                        let rid_direction = if matches!(
                            &self.pending_discard,
                            PendingDiscard::ResumeBoundary { .. }
                        ) {
                            query_response::effective_rid_direction(response.headers(), fallback)?
                        } else {
                            fallback
                        };
                        Ok((rows, rid_direction))
                    });
                    let (mut rows, rid_direction) = validated.map_err(|error| {
                        recovery.absorb(Some(response.diagnostics()));
                        recovery.attach_error(error)
                    })?;
                    let response = recovery.attach_response(response);
                    aggregator.absorb_validated(&response);
                    self.pending_discard.apply(&mut rows, rid_direction);
                    self.buffered = rows;
                    if is_terminal {
                        self.drained = true;
                    }
                    if !self.buffered.is_empty() || self.drained {
                        return Ok(FillOutcome::Filled);
                    }
                    // Empty page with a continuation pending is not drained; re-poll.
                }
                PageResult::Drained => {
                    self.drained = true;
                    return Ok(FillOutcome::Filled);
                }
                PageResult::SplitRequired { replacements } => {
                    return Ok(FillOutcome::SplitRequired { replacements });
                }
            }
        }
    }
}

/// Result of [`ChildStream::ensure_filled`].
enum FillOutcome {
    /// The stream has a buffered head row, or is proven drained.
    Filled,
    /// The stream's range split; the caller must splice these replacements in.
    SplitRequired { replacements: SplitReplacements },
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
    session_token: Option<SessionToken>,
    /// Stable hash of the originating query text and parameters, persisted in
    /// every snapshot and re-checked on resume (see
    /// [`super::snapshot::PipelineNodeState::StreamingOrderedMerge`]).
    query_fingerprint: String,
    /// An error raised *after* rows were already consumed into the page being
    /// assembled. Emitting rows advances each child's resume boundary, so
    /// dropping the partial page would let a continuation token captured after
    /// the failure skip those rows permanently. The partial page is returned
    /// first and this surfaces on the next [`Self::next_page`] call instead.
    deferred_error: Option<crate::error::CosmosError>,
    /// Set when a fetched page was committed by its child [`Request`] but then
    /// failed validation, so every child's committed continuation may now point
    /// past rows that were never emitted. Sticky: once set, snapshots resume
    /// from value boundaries instead of server continuations.
    continuation_unsafe: bool,
    /// Whether emitted pages carry Cosmos binary JSON items. Fixed at
    /// construction from the negotiated operation, so a page served entirely
    /// from buffered rows encodes the same as one that hit the network. This
    /// tracks the *emitted* format, not the wire: under
    /// `request_text_response` the wire is binary while items stay text.
    emit_binary: bool,
    #[cfg(test)]
    retention: Option<tests::RetentionObserver>,
}

impl StreamingOrderedMerge {
    pub(super) fn new(
        plain_operation: Arc<CosmosOperation>,
        directions: Vec<SortOrder>,
        children: Vec<ChildStream>,
        query_fingerprint: String,
    ) -> Self {
        Self {
            emit_binary: plain_operation.emits_binary_payload(),
            plain_operation,
            directions,
            children,
            session_token: None,
            deferred_error: None,
            continuation_unsafe: false,
            query_fingerprint,
            #[cfg(test)]
            retention: None,
        }
    }

    fn max_item_count(&self) -> usize {
        match self.plain_operation.request_headers().max_item_count {
            Some(MaxItemCountHint::Limit(n)) => n.get() as usize,
            Some(MaxItemCountHint::ServerDecides) | None => DEFAULT_MAX_ITEM_COUNT,
        }
    }

    /// Ensures the child at `idx` has a buffered row or is drained, fetching
    /// and resolving splits as needed. Absorbs pages into `aggregator`.
    /// Returns `true` if a split spliced replacements into `self.children`,
    /// which invalidates every index at or after `idx`.
    async fn ensure_stream_filled(
        &mut self,
        idx: usize,
        context: &mut PipelineContext<'_>,
        aggregator: &mut PageAggregator,
    ) -> crate::error::Result<bool> {
        let mut split_retries = 0;
        let mut topology_changed = false;
        let mut recovery = RecoveryDiagnostics::default();
        loop {
            #[cfg(test)]
            let retention = self.retention_sample();
            // Disjoint field borrows: `children` mutably, `directions` shared.
            let outcome = self.children[idx]
                .ensure_filled(
                    context,
                    aggregator,
                    &self.directions,
                    &mut recovery,
                    #[cfg(test)]
                    retention,
                )
                .await;
            // Commit before propagating: a fill that absorbed a page and then
            // failed to parse it still advanced session state we must not lose.
            self.session_token = aggregator.session_token().cloned();
            let outcome = outcome.map_err(|error| recovery.attach_error(error))?;
            match outcome {
                FillOutcome::Filled => return Ok(topology_changed),
                FillOutcome::SplitRequired { mut replacements } => {
                    recovery.absorb(replacements.take_diagnostics());
                    split_retries += 1;
                    if split_retries > MAX_SPLIT_RETRIES {
                        return Err(recovery.attach_error(
                            crate::error::CosmosError::builder()
                                .with_status(
                                    crate::error::status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED,
                                )
                                .with_message(format!(
                                    "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                 in StreamingOrderedMerge"
                                ))
                                .build(),
                        ));
                    }
                    self.handle_split(idx, replacements)
                        .map_err(|error| recovery.attach_error(error))?;
                    topology_changed = true;
                    // Loop: index `idx` now refers to the first replacement.
                }
            }
        }
    }

    /// Fills every currently-active child, restoring the merge invariant
    /// that before rebuilding the merge heap every non-drained child
    /// has a buffered head row (or is proven drained) — so no child is ever
    /// skipped for lacking a head. Re-reads `len()` each step because
    /// [`Self::ensure_stream_filled`]'s split handling can splice several
    /// replacements in at once (only the first is filled inline);
    /// already-buffered/drained children short-circuit, so no page is
    /// re-fetched.
    async fn ensure_all_streams_filled(
        &mut self,
        context: &mut PipelineContext<'_>,
        aggregator: &mut PageAggregator,
    ) -> crate::error::Result<()> {
        let mut idx = 0;
        while idx < self.children.len() {
            self.ensure_stream_filled(idx, context, aggregator).await?;
            idx += 1;
        }
        Ok(())
    }

    /// Handles a child's `SplitRequired` by consuming the [`SplitReplacements`]
    /// the child produced, keeping the merge ignorant of their concrete type.
    ///
    /// The split child provides one replacement leaf per post-split sub-range,
    /// already proven at construction to carry a
    /// [`feed_range`](PipelineNode::feed_range) and to exactly tile the split
    /// child's scope. This wraps each in a [`ChildStream`] that inherits the
    /// split child's resume state (see [`wrap_split_replacement`]). No topology
    /// is re-resolved here — the split child already performed one forced
    /// topology refresh before producing the nodes, forwarding its backend
    /// continuation into each replacement so they resume past every
    /// already-emitted row.
    fn handle_split(
        &mut self,
        idx: usize,
        replacements: SplitReplacements,
    ) -> crate::error::Result<()> {
        let prior_boundary = self.children[idx].boundary();
        let query_shape = self.children[idx].query_shape;
        // Cloned live, not rebuilt from `prior_boundary`: an armed discard has
        // already been decremented by the pages it consumed, while the boundary
        // still carries the pre-decrement `skip_count`.
        let prior_discard = self.children[idx].pending_discard.clone();

        let replacements = replacements.into_ranged()?;
        // Wrap each replacement before mutating `self.children`, so a rejected
        // replacement leaves the merge unchanged rather than half-spliced.
        let mut wrapped = Vec::with_capacity(replacements.len());
        for (range, node) in replacements {
            wrapped.push(wrap_split_replacement(
                range,
                node,
                prior_boundary.as_ref(),
                &prior_discard,
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

    fn build_head_heap(&self) -> Vec<usize> {
        let mut heap = Vec::with_capacity(self.children.len());
        for idx in 0..self.children.len() {
            if self.children[idx].buffered.front().is_some() {
                binary_heap::push_by(&mut heap, idx, |left, right| {
                    self.row_less_than(*left, *right)
                });
            }
        }
        heap
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
        let ordering = compare_key_tuples(&a.keys, &b.keys, &self.directions).then_with(|| {
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
        // A failure deferred by a prior partial page takes precedence over
        // everything, including the drained short-circuit below.
        if let Some(err) = self.deferred_error.take() {
            return Err(err);
        }
        if self.children.is_empty() {
            return Ok(PageResult::Drained);
        }

        #[cfg(test)]
        self.record_output_retention(&[]);
        let mut aggregator = PageAggregator::new(self.emit_binary);
        aggregator.seed_session_token(self.session_token.clone());

        // Prime every child up front so the heap sees a head row for each
        // non-drained child. This page has emitted nothing, but a fill commits
        // its backend continuation before the body is validated, so a failure
        // can still leave a child advanced past rows an earlier call never
        // delivered; the snapshot must fall back to the scalar boundary.
        if let Err(err) = self
            .ensure_all_streams_filled(context, &mut aggregator)
            .await
        {
            self.continuation_unsafe = true;
            return Err(aggregator.attach_error(err));
        }
        let mut head_heap = self.build_head_heap();

        let cap = self.max_item_count();
        let mut items: Vec<ItemView> = Vec::new();

        while items.len() < cap {
            let Some(winner) = binary_heap::pop_by(&mut head_heap, |left, right| {
                self.row_less_than(*left, *right)
            }) else {
                break;
            };
            let row = self.children[winner]
                .buffered
                .pop_front()
                .expect("head heap only contains indices with a buffered row");
            // Encode *before* `record_emission` advances the boundary. Once the
            // boundary moves, this row sits behind the resume point and can no
            // longer be replayed, so a failure after that point would drop it
            // silently. Encoding first keeps the failure recoverable.
            let item = match aggregator.encode_item(items.len(), &row.payload) {
                Ok(item) => item,
                Err(err) => {
                    self.children[winner].buffered.push_front(row);
                    if items.is_empty() {
                        return Err(aggregator.attach_error(err));
                    }
                    self.deferred_error = Some(err);
                    break;
                }
            };
            if let Err(err) = self.children[winner].record_emission(&row.keys, &row.rid) {
                // The boundary was not advanced, so put the row back and let
                // it be re-emitted on a later attempt.
                self.children[winner].buffered.push_front(row);
                if items.is_empty() {
                    return Err(aggregator.attach_error(err));
                }
                self.deferred_error = Some(err);
                break;
            }
            items.push(item);
            #[cfg(test)]
            self.record_output_retention(&items);
            if items.len() < cap {
                if self.children[winner].buffered.front().is_some() {
                    binary_heap::push_by(&mut head_heap, winner, |left, right| {
                        self.row_less_than(*left, *right)
                    });
                } else {
                    // From here on rows have already been consumed and their
                    // boundaries advanced, so a fetch failure must not discard
                    // the page — defer it instead.
                    let topology_changed = match self
                        .ensure_stream_filled(winner, context, &mut aggregator)
                        .await
                    {
                        Ok(changed) => changed,
                        Err(err) => {
                            self.deferred_error = Some(err);
                            self.continuation_unsafe = true;
                            break;
                        }
                    };
                    if topology_changed {
                        // Split replacements shift child indices and only the
                        // first replacement was filled inline.
                        if let Err(err) = self
                            .ensure_all_streams_filled(context, &mut aggregator)
                            .await
                        {
                            self.deferred_error = Some(err);
                            self.continuation_unsafe = true;
                            break;
                        }
                        head_heap = self.build_head_heap();
                    } else if self.children[winner].buffered.front().is_some() {
                        binary_heap::push_by(&mut head_heap, winner, |left, right| {
                            self.row_less_than(*left, *right)
                        });
                    }
                }
            }
        }

        // Evict fully-drained empty children, mirroring `SequentialDrain`,
        // so a later snapshot never references them.
        self.children
            .retain(|child| !(child.drained && child.buffered.is_empty()));
        // A deferred error still has to be delivered, so the stream can never
        // be reported terminal while one is pending.
        let is_terminal = self.children.is_empty() && self.deferred_error.is_none();

        self.session_token = aggregator.session_token().cloned();
        Ok(PageResult::Page {
            response: aggregator.build_page(items),
            is_terminal,
        })
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.children.into_iter().map(|c| c.node).collect()
    }

    fn take_pending_error(&mut self) -> Option<crate::error::CosmosError> {
        self.deferred_error.take()
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        if self.children.is_empty() {
            return Ok(PipelineNodeState::Drained);
        }

        let mut ranges = Vec::with_capacity(self.children.len());
        for (idx, child) in self.children.iter().enumerate() {
            // Safe to snapshot the backend continuation into the plain
            // `server_continuation` field only when nothing is buffered
            // (else it points past unemitted rows), the child runs the
            // plain query (`ChildQueryShape::Plain`) — a resume-filtered
            // child's continuation is bound to that filtered text and would
            // mismatch the plain query on resume — and no fetched page failed
            // validation after being committed (`continuation_unsafe`, which
            // would likewise point past rows that never reached the caller).
            // Otherwise the child resumes from its scalar `boundary` instead.
            let server_continuation = if child.buffered.is_empty()
                && child.query_shape == ChildQueryShape::Plain
                && !self.continuation_unsafe
            {
                match child.node.snapshot_state()? {
                    PipelineNodeState::Request {
                        server_continuation,
                    } => server_continuation,
                    PipelineNodeState::Drained => None,
                    other => {
                        return Err(crate::error::CosmosError::builder()
                            .with_status(
                                crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE,
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
            query_fingerprint: Some(self.query_fingerprint.clone()),
            ranges,
        })
    }

    fn topology_can_change(&self) -> bool {
        // Splits are handled internally (`handle_split`); no parent needed.
        false
    }

    fn fan_out_width(&self) -> usize {
        self.children.iter().map(|c| c.node.fan_out_width()).sum()
    }
}

/// Stable fingerprint of the originating query body (query text plus
/// parameters, exactly as the caller supplied it) *and* the operation's feed
/// scope, persisted in a continuation token so a resume can prove the token
/// belongs to this query over this scope.
///
/// The scope is part of the fingerprint because nothing else binds it. A
/// resumed node treats its saved ranges as authoritative, and
/// `ContinuationToken::is_valid_for_operation` checks only the operation kind
/// and container RID — so replaying a token under a different
/// `FeedScope` (a different explicit range, or a different partial
/// hierarchical partition key prefix) would otherwise read outside the
/// requested scope, or silently return only the scope the token was minted
/// for.
///
/// Hashed with the same MurmurHash3-128 used elsewhere in the driver, so the
/// value is byte-stable across processes and SDK builds (unlike
/// `std::hash::DefaultHasher`). The original body is fingerprinted rather than
/// the Gateway's rewritten query so a service-side rewrite change does not
/// invalidate in-flight tokens.
///
/// Bodies retain their exact serialized form for compatibility with existing
/// tokens. The planner calls this before any optional request encoding.
pub(super) fn query_fingerprint(body: Option<&[u8]>, scope: Option<&FeedRange>) -> String {
    // The body hash is rendered fixed-width first so the two components can
    // never run together; EPK hex is `[0-9A-F]*`, so neither separator can
    // occur inside a bound. Bounds render canonically (trailing zero bytes
    // stripped) so two EPKs hash alike exactly when they compare equal — the
    // backend and other SDKs may hand back a bound with that padding trimmed.
    // An absent scope hashes as empty, which stays distinct from the
    // full-container range (`-FF`).
    let body = body.unwrap_or_default();
    let body_hash = crate::models::murmur_hash::murmurhash3_128(body, 0);
    let scope = match scope {
        Some(range) => format!(
            "{}-{}",
            range.min_inclusive().to_canonical_hex(),
            range.max_exclusive().to_canonical_hex()
        ),
        None => String::new(),
    };
    format!(
        "{:032x}",
        crate::models::murmur_hash::murmurhash3_128(
            format!("{body_hash:032x}:{scope}").as_bytes(),
            0
        )
    )
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
///   per-row seek, so it stays correct across a split.
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
        let target = RequestTarget::effective_partition_key_range_with_parents(
            scope.clone(),
            resolved_range.partition_key_range_id.clone(),
            resolved_range.parents.clone(),
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
    // per-range row-count attribution.
    let mut children = Vec::with_capacity(clipped.len());
    for (owned, resolved_range) in clipped {
        let target = RequestTarget::effective_partition_key_range_with_parents(
            owned.clone(),
            resolved_range.partition_key_range_id.clone(),
            resolved_range.parents.clone(),
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
///   *after* every emitted row. `last_emitted` is forwarded (for `skip_count`
///   accumulation and future snapshots), and no *fresh* discard is built —
///   rebuilding one from the boundary would wrongly drop later JOIN rows
///   sharing the boundary `(key, _rid)`. `prior_discard` still carries over
///   verbatim: if it is armed, the resume prefix was only partially consumed,
///   and its remaining (already-decremented) skip applies past the forwarded
///   continuation.
/// - `Some` + no forwarded continuation + a resume-filtered range: rebuild the
///   [`PendingDiscard::ResumeBoundary`] from the boundary, exactly like a
///   saved-token resume. Such a leaf re-runs the structured `resumeFilter`
///   (the backend seeks to the boundary), so the discard only trims the
///   already-emitted prefix.
/// - `Some` + no forwarded continuation + a plain range (or generic node):
///   the leaf's resume position is unknown (a plain replay would re-fetch from
///   the start, and the client discard has no way to tell which rows that
///   replay already covered), so reject with a typed error rather than guess.
///   For a real plain
///   `Request` this is unreachable — a plain child that had emitted a row is
///   always `Continuing` when it splits, so its replacement always carries a
///   forwarded continuation.
fn wrap_split_replacement(
    range: FeedRange,
    node: Box<dyn PipelineNode>,
    prior_boundary: Option<&ValueBoundary>,
    prior_discard: &PendingDiscard,
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
        // The forwarded continuation positions the replacement past every row
        // the backend already returned, so no boundary re-seek is needed. A
        // still-armed discard, though, means the resume prefix was only
        // partially consumed (its pages were entirely duplicates), so the
        // remaining skip has to carry over or those rows emit twice. It must be
        // the live, already-decremented discard: `boundary` still holds the
        // pre-decrement `skip_count`, since nothing was emitted to advance it.
        child.last_emitted = Some(LastEmitted {
            resume_values: boundary.resume_values.clone(),
            rid: boundary.last_rid.clone(),
            skip_count: boundary.skip_count,
        });
        child.pending_discard = prior_discard.clone();
        return Ok(child);
    }

    // No forwarded continuation but the split child had emitted rows. A
    // resume-filtered replacement re-seeks the backend to the boundary, so
    // rebuild the discard to trim the already-emitted prefix. A plain (or
    // generic) replacement's position is unknown; reject.
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

/// Builds one child via the value-boundary resume path: sends the
/// last-emitted boundary to the backend as a structured `resumeFilter`
/// (`rid` present, `exclude:false`) injected into a clone of the plain
/// query body, and installs a matching [`PendingDiscard::ResumeBoundary`]
/// guard so the already-emitted prefix of the boundary tie run is trimmed
/// client-side. The backend seek is a per-row predicate, so it stays
/// correct across a split.
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
    use std::{
        collections::HashMap,
        hint::black_box,
        num::NonZeroU32,
        sync::atomic::{AtomicUsize, Ordering as AtomicOrdering},
        time::{Duration, Instant},
    };

    use crate::driver::dataflow::mocks::{self, MockLeaf};
    use crate::models::effective_partition_key::EffectivePartitionKey;

    #[derive(Default)]
    pub(super) struct RetentionObserver {
        peak_bytes: Arc<AtomicUsize>,
        output_pages: HashMap<usize, usize>,
    }

    pub(super) struct RetentionSample {
        peak_bytes: Arc<AtomicUsize>,
        pages: HashMap<usize, usize>,
    }

    impl RetentionSample {
        pub(super) fn record(&self, fetched_page: Option<&[u8]>) {
            let mut bytes: usize = self.pages.values().sum();
            if let Some(page) = fetched_page {
                if crate::binary_json::is_binary(page)
                    && !self.pages.contains_key(&(page.as_ptr() as usize))
                {
                    bytes += page.len();
                }
            }
            self.peak_bytes.fetch_max(bytes, AtomicOrdering::Relaxed);
        }
    }

    impl StreamingOrderedMerge {
        pub(super) fn retention_sample(&self) -> Option<RetentionSample> {
            let observer = self.retention.as_ref()?;
            // Store addresses and lengths, never owning clones that would extend retention.
            let mut pages = observer.output_pages.clone();
            for child in &self.children {
                for row in &child.buffered {
                    if let query_response::EnvelopePayload::Binary(item) = &row.payload {
                        let page = item.source_page();
                        pages.insert(page.as_ptr() as usize, page.len());
                    }
                }
            }
            Some(RetentionSample {
                peak_bytes: observer.peak_bytes.clone(),
                pages,
            })
        }

        pub(super) fn record_output_retention(&mut self, items: &[ItemView]) {
            let Some(observer) = &mut self.retention else {
                return;
            };
            observer.output_pages.clear();
            for item in items {
                let page = item.source_page();
                if crate::binary_json::is_binary(page) {
                    observer
                        .output_pages
                        .insert(page.as_ptr() as usize, page.len());
                }
            }
            self.retention_sample().unwrap().record(None);
        }
    }

    fn range(min: &str, max: &str) -> crate::error::Result<FeedRange> {
        FeedRange::new(
            EffectivePartitionKey::try_from(min)?,
            EffectivePartitionKey::try_from(max)?,
        )
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

    fn envelope_page_with_execution_info(
        rows: &[(&str, i64)],
        execution_info: &str,
    ) -> crate::error::Result<PageResult> {
        let response = envelope_response(rows, None);
        let headers = crate::models::CosmosResponseHeaders {
            query_execution_info: Some(execution_info.to_owned()),
            ..Default::default()
        };
        Ok(PageResult::Page {
            response: crate::models::CosmosResponse::new(
                response.body_bytes().to_vec(),
                headers,
                response.status(),
                response.diagnostics(),
            ),
            is_terminal: true,
        })
    }

    fn envelope_page_with_session_token(
        rows: &[(&str, i64)],
        session_token: &'static str,
    ) -> crate::error::Result<PageResult> {
        let response = envelope_response(rows, None);
        let headers = crate::models::CosmosResponseHeaders {
            session_token: Some(SessionToken::new(session_token)),
            ..Default::default()
        };
        Ok(PageResult::Page {
            response: crate::models::CosmosResponse::new(
                response.body_bytes().to_vec(),
                headers,
                response.status(),
                response.diagnostics(),
            ),
            is_terminal: true,
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
        assert!(!matches!(
            response.body(),
            crate::models::ResponseBody::Bytes(_)
        ));
        let items: Vec<serde_json::Value> = response.body().clone().into_items().unwrap();
        items
            .iter()
            .map(|item| item["id"].as_str().unwrap().to_owned())
            .collect()
    }

    fn merge(children: Vec<ChildStream>, directions: Vec<SortOrder>) -> StreamingOrderedMerge {
        StreamingOrderedMerge::new(
            Arc::new(mocks::operation()),
            directions,
            children,
            "test-fingerprint".to_owned(),
        )
    }

    #[tokio::test]
    async fn split_exhaustion_keeps_attempt_diagnostics() {
        let range = FeedRange::full();
        let request = Request::new(
            Arc::new(mocks::operation()),
            RequestTarget::effective_partition_key_range(range.clone(), "0".into(), range.clone()),
            None,
        );
        let mut node = merge(
            vec![ChildStream::fresh(range.clone(), Box::new(request))],
            vec![SortOrder::Ascending],
        );
        let mut executor = mocks::MockRequestExecutor::new(
            (0..11)
                .map(|_| Err(mocks::gone_error_with_diagnostics()))
                .collect(),
        );
        let mut topology =
            mocks::PhysicalTopologyProvider::new(vec![super::super::ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: Vec::new(),
                range,
            }]);
        let error = node
            .next_page(&mut PipelineContext::new(
                &mut executor,
                Some(&mut topology),
            ))
            .await
            .unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED
        );
        let diagnostics = error.diagnostics().unwrap();
        assert_eq!(diagnostics.request_count(), 11);
        assert_eq!(diagnostics.effective_status(), Some(error.status()));
    }

    #[tokio::test]
    async fn recovered_attempts_survive_failure_before_first_page() {
        for malformed in [false, true] {
            let left_range = range("", "80").unwrap();
            let page = if malformed {
                mocks::response(b"{")
            } else {
                join_envelope_response(&[("rid", 1, "first")], None)
            };
            let replacement = Box::new(
                mocks::MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: page,
                    is_terminal: true,
                })])
                .with_feed_range(left_range.clone()),
            );
            let replacements = SplitReplacements::try_tiling(&left_range, vec![replacement])
                .unwrap()
                .with_diagnostics(mocks::gone_error_with_diagnostics().diagnostics());
            let left = ChildStream::fresh(
                left_range,
                Box::new(mocks::MockLeaf::with_pages(vec![Ok(
                    PageResult::SplitRequired { replacements },
                )])),
            );
            let right = ChildStream::fresh(
                range("80", "FF").unwrap(),
                Box::new(mocks::MockLeaf::with_pages(vec![Err(
                    mocks::gone_error_with_diagnostics(),
                )])),
            );
            let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
            let mut executor = mocks::NoopRequestExecutor;
            let error = node
                .next_page(&mut PipelineContext::new(&mut executor, None))
                .await
                .unwrap_err();
            let diagnostics = error
                .diagnostics()
                .expect("immediate errors must retain absorbed recovery attempts");
            assert_eq!(diagnostics.request_count(), if malformed { 1 } else { 2 });
            assert_eq!(diagnostics.effective_status(), Some(error.status()));
        }
    }

    #[tokio::test]
    async fn poisoned_distinct_retains_pending_child_recovery_diagnostics() {
        use crate::{
            diagnostics::{
                DiagnosticsContextBuilder, ExecutionContext, PipelineKind, TransportHttpVersion,
                TransportKind, TransportSecurity,
            },
            driver::dataflow::{query_plan::DistinctType, Distinct, SkipTake},
            error::status_codes,
            models::{CosmosResponse, RequestCharge},
        };
        use std::error::Error as _;

        for nested in [false, true] {
            let payload = format!("{}0{}", "[".repeat(130), "]".repeat(130));
            let body = format!(
                r#"{{"Documents":[{{"_rid":"first","orderByItems":[{{"item":1}}],"payload":{payload}}}]}}"#
            );
            let scope = FeedRange::full();
            let request = Request::new(
                Arc::new(mocks::operation()),
                RequestTarget::effective_partition_key_range(
                    scope.clone(),
                    "0".into(),
                    scope.clone(),
                ),
                None,
            );
            let mut merged = merge(
                vec![ChildStream::fresh(scope.clone(), Box::new(request))],
                vec![SortOrder::Ascending],
            );
            merged.emit_binary = false;
            let distinct: Box<dyn PipelineNode> =
                Box::new(Distinct::new(Box::new(merged), DistinctType::Ordered));
            let mut root: Box<dyn PipelineNode> = if nested {
                Box::new(SkipTake::new(distinct, 0, Some(1), false))
            } else {
                distinct
            };
            let charged = |response: CosmosResponse| {
                let mut headers = response.headers().clone();
                headers.request_charge = Some(RequestCharge::new(1.0));
                let mut diagnostics = DiagnosticsContextBuilder::new(
                    crate::models::ActivityId::new_uuid(),
                    Arc::new(crate::options::DiagnosticsOptions::default()),
                );
                let endpoint = crate::driver::routing::CosmosEndpoint::global(
                    url::Url::parse("https://acct.example/").unwrap(),
                );
                let handle = diagnostics.start_request(
                    ExecutionContext::Initial,
                    PipelineKind::DataPlane,
                    TransportSecurity::Secure,
                    TransportKind::Gateway,
                    TransportHttpVersion::Http11,
                    &endpoint,
                );
                diagnostics.update_request(handle, |request| {
                    request.with_charge(RequestCharge::new(1.0))
                });
                diagnostics.complete_request(handle, response.status().status_code(), None);
                diagnostics.set_operation_status(response.status().status_code(), None);
                CosmosResponse::new(
                    response.body().clone(),
                    headers,
                    response.status(),
                    Arc::new(diagnostics.complete()),
                )
            };
            let mut executor = mocks::MockRequestExecutor::new(vec![
                Ok(charged(mocks::response_with_continuation(
                    body.as_bytes(),
                    Some("more"),
                ))),
                Err(mocks::gone_error_with_diagnostics()),
                Ok(charged(mocks::response(b"{"))),
            ]);
            let mut topology =
                mocks::PhysicalTopologyProvider::new(vec![super::super::ResolvedRange {
                    partition_key_range_id: "0".into(),
                    parents: Vec::new(),
                    range: scope,
                }]);
            {
                let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
                let error = root.next_page(&mut context).await.unwrap_err();
                assert_eq!(
                    error.status(),
                    status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
                );
                assert!(error
                    .source()
                    .unwrap()
                    .to_string()
                    .contains("recursion limit exceeded"));
                let diagnostics = error.diagnostics().unwrap();
                assert_eq!(diagnostics.request_count(), 3);
                assert_eq!(diagnostics.total_request_charge(), RequestCharge::new(3.0));
                assert_eq!(diagnostics.effective_status(), Some(error.status()));
                assert_eq!(
                    diagnostics
                        .requests()
                        .iter()
                        .map(|request| request.status().status_code())
                        .collect::<Vec<_>>(),
                    vec![
                        azure_core::http::StatusCode::Ok,
                        azure_core::http::StatusCode::Gone,
                        azure_core::http::StatusCode::Ok
                    ]
                );
                let json: serde_json::Value =
                    serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
                assert_eq!(json["topology_recovery"]["total_attempts"], 1);
                assert!(root.take_pending_error().is_none());
                let repeated = root.next_page(&mut context).await.unwrap_err();
                assert_eq!(repeated.status(), error.status());
                assert!(
                    repeated.diagnostics().is_none(),
                    "reported attempts must not be delivered again"
                );
            }
            assert_eq!(
                executor.target_calls.len(),
                3,
                "poisoning must not fetch another page"
            );
        }
    }

    #[tokio::test]
    async fn terminal_window_hands_off_deferred_recovery_error_without_polling() {
        use crate::driver::dataflow::{query_plan::DistinctType, Distinct, SkipTake};
        use crate::models::{CosmosResponse, RequestCharge};

        for (wrapper, skip) in (0..3).flat_map(|wrapper| [0, 1].map(|skip| (wrapper, skip))) {
            let left_range = range("", "80").unwrap();
            let right_range = range("80", "FF").unwrap();
            let request = |range: FeedRange, id: &str| {
                ChildStream::fresh(
                    range.clone(),
                    Box::new(Request::new(
                        Arc::new(mocks::operation()),
                        RequestTarget::effective_partition_key_range(
                            range.clone(),
                            id.into(),
                            range,
                        ),
                        None,
                    )),
                )
            };
            let merged: Box<dyn PipelineNode> = Box::new(merge(
                vec![
                    request(left_range.clone(), "0"),
                    request(right_range.clone(), "1"),
                ],
                vec![SortOrder::Ascending],
            ));
            let child: Box<dyn PipelineNode> = match wrapper {
                0 => merged,
                1 => Box::new(Distinct::new(merged, DistinctType::Ordered)),
                _ => Box::new(Distinct::new(
                    Box::new(SkipTake::new(merged, 0, Some(2), false)),
                    DistinctType::Ordered,
                )),
            };
            let mut window = SkipTake::new(child, skip, Some(2 - skip), false);
            let charged = |response: CosmosResponse| {
                let mut headers = response.headers().clone();
                headers.request_charge = Some(RequestCharge::new(1.0));
                let diagnostics = mocks::gone_error_with_diagnostics().diagnostics().unwrap();
                CosmosResponse::new(
                    response.body().clone(),
                    headers,
                    response.status(),
                    Arc::new(diagnostics.clone_with_status(response.status())),
                )
            };
            let mut executor = mocks::MockRequestExecutor::new(vec![
                Ok(charged(join_envelope_response(
                    &[("a", 1, "first")],
                    Some("left"),
                ))),
                Ok(charged(join_envelope_response(
                    &[("b", 2, "second")],
                    Some("right"),
                ))),
                Ok(charged(join_envelope_response(
                    &[("c", 3, "third")],
                    Some("left-next"),
                ))),
                Err(mocks::gone_error_with_diagnostics()),
                Ok(charged(mocks::response(b"{"))),
            ]);
            let mut topology = mocks::PhysicalTopologyProvider::new(vec![
                super::super::ResolvedRange {
                    partition_key_range_id: "0".into(),
                    parents: vec![],
                    range: left_range,
                },
                super::super::ResolvedRange {
                    partition_key_range_id: "1".into(),
                    parents: vec![],
                    range: right_range,
                },
            ]);
            {
                let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
                let PageResult::Page {
                    response,
                    is_terminal,
                } = window.next_page(&mut context).await.unwrap()
                else {
                    panic!("expected window rows");
                };
                assert_eq!(
                    ids(&response),
                    if skip == 0 {
                        vec!["first", "second"]
                    } else {
                        vec!["second"]
                    }
                );
                assert!(
                    !is_terminal,
                    "pending error must remain observable after TOP is satisfied"
                );
                assert_eq!(response.diagnostics().request_count(), 3);
                assert_eq!(
                    response.diagnostics().total_request_charge(),
                    RequestCharge::new(3.0)
                );
                assert!(!response
                    .diagnostics()
                    .to_json_string(None)
                    .contains("topology_recovery"));
                let pending = window
                    .snapshot_state()
                    .expect_err("a pending error must not be saved as drained");
                assert_eq!(pending.diagnostics().unwrap().request_count(), 2);
                let error = window
                    .next_page(&mut context)
                    .await
                    .expect_err("deliver the pending error");
                let diagnostics = error.diagnostics().unwrap();
                assert_eq!(error.status(), pending.status());
                assert_eq!(diagnostics.request_count(), 2);
                assert_eq!(diagnostics.total_request_charge(), RequestCharge::new(2.0));
                assert_eq!(diagnostics.effective_status(), Some(error.status()));
                let json: serde_json::Value =
                    serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
                assert_eq!(json["topology_recovery"]["total_attempts"], 1);
                assert!(matches!(
                    window.next_page(&mut context).await.unwrap(),
                    PageResult::Drained
                ));
                assert!(matches!(
                    window.snapshot_state().unwrap(),
                    PipelineNodeState::Drained
                ));
            }
            assert_eq!(
                executor.target_calls.len(),
                5,
                "handoff must not execute another request"
            );
        }
    }

    #[tokio::test]
    async fn malformed_replacement_diagnostics_belong_to_deferred_error() {
        for invalid_header in [false, true] {
            let scope = FeedRange::full();
            let prior = mocks::gone_error_with_diagnostics().diagnostics().unwrap();
            let prior = Arc::new(prior.with_topology_recovery(
                "0",
                &scope,
                std::time::Duration::ZERO,
                std::time::Duration::ZERO,
                None,
                [("1".to_owned(), String::new(), "FF".to_owned())].into_iter(),
            ));
            let mut headers = mocks::response(b"").headers().clone();
            headers.session_token = Some(SessionToken::new("0:1#20"));
            headers.query_execution_info = invalid_header.then(|| "{".to_owned());
            let body = if invalid_header {
                join_envelope_response(&[("rid2", 2, "next")], None)
                    .body_bytes()
                    .to_vec()
            } else {
                b"{".to_vec()
            };
            let malformed = crate::models::CosmosResponse::new(
                body,
                headers,
                mocks::response(b"").status(),
                mocks::response_with_request_diagnostics(1).diagnostics(),
            );
            let replacement = Box::new(
                MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: malformed,
                    is_terminal: true,
                })])
                .with_feed_range(scope.clone())
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("next".into()),
                }),
            );
            let replacements = SplitReplacements::try_tiling(&scope, vec![replacement])
                .unwrap()
                .with_diagnostics(Some(prior));
            let child = Box::new(MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
                replacements,
            })]));
            let mut node = merge(
                vec![ChildStream {
                    buffered: query_response::parse_envelope_page(
                        join_envelope_response(&[("rid", 1, "first")], Some("more")).body(),
                        1,
                    )
                    .unwrap()
                    .into(),
                    ..ChildStream::fresh(scope, child)
                }],
                vec![SortOrder::Ascending],
            );
            if invalid_header {
                node.children[0].pending_discard = PendingDiscard::ResumeBoundary {
                    resume_values: node.children[0]
                        .buffered
                        .front()
                        .unwrap()
                        .keys
                        .iter()
                        .map(|key| key.to_resume_value().unwrap())
                        .collect(),
                    last_rid: "rid".into(),
                    skip_count: 1,
                    directions: vec![SortOrder::Ascending],
                };
            }
            let mut executor = mocks::NoopRequestExecutor;
            let mut context = PipelineContext::new(&mut executor, None);
            let PageResult::Page {
                response,
                is_terminal,
            } = node.next_page(&mut context).await.unwrap()
            else {
                panic!("expected partial page");
            };
            assert_eq!(ids(&response), vec!["first"]);
            assert!(!is_terminal);
            assert_eq!(response.diagnostics().request_count(), 0);
            assert!(!response
                .diagnostics()
                .to_json_string(None)
                .contains("topology_recovery"));
            assert_eq!(
                node.session_token.as_ref().map(SessionToken::as_str),
                Some("0:1#20")
            );
            assert!(node.continuation_unsafe);
            let error = node.next_page(&mut context).await.unwrap_err();
            let diagnostics = error
                .diagnostics()
                .expect("deferred validation error must retain both attempts");
            assert_eq!(diagnostics.request_count(), 2);
            assert_eq!(diagnostics.effective_status(), Some(error.status()));
            let json: serde_json::Value =
                serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
            assert_eq!(json["topology_recovery"]["total_attempts"], 1);
        }
    }

    #[tokio::test]
    async fn deferred_split_exhaustion_does_not_duplicate_attempts_on_partial_page() {
        let range = FeedRange::full();
        let mut child: Box<dyn PipelineNode> = Box::new(
            mocks::MockLeaf::with_pages(vec![])
                .with_feed_range(range.clone())
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("more".into()),
                }),
        );
        for _ in 0..11 {
            let replacements = SplitReplacements::try_tiling(&range, vec![child])
                .unwrap()
                .with_diagnostics(mocks::gone_error_with_diagnostics().diagnostics());
            child = Box::new(
                mocks::MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired { replacements })])
                    .with_feed_range(range.clone())
                    .with_snapshot(PipelineNodeState::Request {
                        server_continuation: Some("more".into()),
                    }),
            );
        }
        let mut node = merge(
            vec![ChildStream {
                buffered: query_response::parse_envelope_page(
                    join_envelope_response(&[("rid", 1, "first")], Some("more")).body(),
                    1,
                )
                .unwrap()
                .into(),
                ..ChildStream::fresh(range, child)
            }],
            vec![SortOrder::Ascending],
        );
        let mut executor = mocks::NoopRequestExecutor;
        let mut context = PipelineContext::new(&mut executor, None);
        let PageResult::Page {
            response,
            is_terminal,
        } = node.next_page(&mut context).await.unwrap()
        else {
            panic!("expected partial page");
        };
        assert_eq!(ids(&response), vec!["first"]);
        assert!(!is_terminal);
        assert_eq!(response.diagnostics().request_count(), 0);
        let error = node.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED
        );
        assert_eq!(error.diagnostics().unwrap().request_count(), 11);
    }

    #[tokio::test]
    async fn binary_split_recovery_preserves_page_context_and_diagnostics() {
        for malformed in [false, true] {
            let scope = FeedRange::full();
            let first = crate::binary_json::test_support::reference_rows(&[("first", 0)], "first");
            let next = crate::binary_json::test_support::reference_rows(&[("next", 1)], "next");
            let mut body = next.to_vec();
            if malformed {
                body.pop();
            }
            let mut headers = mocks::response(b"").headers().clone();
            headers.session_token = Some(SessionToken::new("0:1#20"));
            let replacement = Box::new(
                MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: crate::models::CosmosResponse::new(
                        body,
                        headers,
                        mocks::response(b"").status(),
                        mocks::response_with_request_diagnostics(1).diagnostics(),
                    ),
                    is_terminal: true,
                })])
                .with_feed_range(scope.clone())
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("next".into()),
                }),
            );
            let replacements = SplitReplacements::try_tiling(&scope, vec![replacement])
                .unwrap()
                .with_diagnostics(mocks::gone_error_with_diagnostics().diagnostics());
            let children = vec![ChildStream {
                buffered: query_response::parse_envelope_page(mocks::response(&first).body(), 1)
                    .unwrap()
                    .into(),
                ..ChildStream::fresh(
                    scope,
                    Box::new(MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
                        replacements,
                    })])),
                )
            }];
            let mut node = StreamingOrderedMerge::new(
                Arc::new(mocks::operation().with_supported_serialization_formats("CosmosBinary")),
                vec![SortOrder::Ascending],
                children,
                "binary-recovery".to_owned(),
            );
            let mut executor = mocks::NoopRequestExecutor;
            let mut context = PipelineContext::new(&mut executor, None);
            let PageResult::Page {
                response,
                is_terminal,
            } = node.next_page(&mut context).await.unwrap()
            else {
                panic!("expected a binary page");
            };
            let crate::models::ResponseBody::ContextualItems(views) = response.body() else {
                panic!("recovery must preserve contextual binary items");
            };
            assert_eq!(views[0].source_page(), first.as_ref());
            assert_eq!(
                node.session_token.as_ref().map(SessionToken::as_str),
                Some("0:1#20")
            );
            if malformed {
                assert_eq!(views.len(), 1);
                assert_eq!(ids(&response), ["first"]);
                assert!(!is_terminal);
                assert_eq!(response.diagnostics().request_count(), 0);
                assert!(node.continuation_unsafe);
                let error = node.next_page(&mut context).await.unwrap_err();
                assert_eq!(
                    error.status(),
                    crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
                );
                let diagnostics = error.diagnostics().unwrap();
                assert_eq!(diagnostics.request_count(), 2);
                assert_eq!(diagnostics.effective_status(), Some(error.status()));
            } else {
                assert_eq!(views.len(), 2);
                assert_eq!(views[1].source_page(), next.as_ref());
                assert_eq!(ids(&response), ["first", "next"]);
                assert!(is_terminal);
                assert_eq!(response.diagnostics().request_count(), 2);
                assert!(node.take_pending_error().is_none());
            }
        }
    }

    struct SelectiveMergeMeasurements {
        source_bytes: usize,
        peak_merge_page_bytes: usize,
        retained_output_page_bytes: usize,
        elapsed: Duration,
    }

    async fn selective_binary_merge_measurements(rounds: u32) -> SelectiveMergeMeasurements {
        const FAN_OUT: usize = 16;
        const ITEMS_PER_PAGE: usize = 100;
        const SKIP: u64 = 20;
        const TAKE: u64 = 10;

        let pages = mocks::binary_order_by_pages(FAN_OUT, ITEMS_PER_PAGE, 1024);
        let source_bytes: usize = pages.iter().map(bytes::Bytes::len).sum();
        let operation = mocks::operation()
            .with_supported_serialization_formats("CosmosBinary")
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(30).unwrap()));

        let start = Instant::now();
        let mut retained_output_page_bytes = 0;
        let mut peak_merge_page_bytes = 0;
        for _ in 0..rounds {
            let children: Vec<_> = pages
                .iter()
                .enumerate()
                .map(|(partition, page)| {
                    let min = if partition == 0 {
                        String::new()
                    } else {
                        format!("{partition:X}0")
                    };
                    let max = if partition + 1 == FAN_OUT {
                        "FF".to_owned()
                    } else {
                        format!("{:X}0", partition + 1)
                    };
                    ChildStream::fresh(
                        range(&min, &max).unwrap(),
                        Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                            response: mocks::response(page),
                            is_terminal: true,
                        })])),
                    )
                })
                .collect();
            let mut merge = StreamingOrderedMerge::new(
                Arc::new(operation.clone()),
                vec![SortOrder::Ascending],
                children,
                "baseline-fingerprint".to_owned(),
            );
            let observer = RetentionObserver::default();
            let peak_bytes = observer.peak_bytes.clone();
            merge.retention = Some(observer);
            let mut node =
                super::super::skip_take::SkipTake::new(Box::new(merge), SKIP, Some(TAKE), true);
            let mut executor = mocks::NoopRequestExecutor;
            let mut topology = mocks::NoopTopologyProvider;
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            let PageResult::Page {
                response,
                is_terminal,
            } = node.next_page(&mut context).await.unwrap()
            else {
                panic!("expected a page from the selective merge");
            };
            assert!(is_terminal);
            let crate::models::ResponseBody::ContextualItems(views) = response.body() else {
                panic!("binary ORDER BY survivors must retain their original pages");
            };
            let mut retained = HashMap::new();
            for view in views {
                retained.insert(view.source_page().as_ptr(), view.source_page().len());
                assert_eq!(view.raw_value(), &view.source_page()[view.value_range()]);
            }
            assert_eq!(retained.len(), TAKE as usize);
            retained_output_page_bytes = retained_output_page_bytes.max(retained.values().sum());
            let measured_peak = peak_bytes.load(AtomicOrdering::Relaxed);
            assert_eq!(measured_peak, source_bytes);
            peak_merge_page_bytes = peak_merge_page_bytes.max(measured_peak);
            let items: Vec<serde_json::Value> = response.body().clone().into_items().unwrap();
            let ranks: Vec<_> = items
                .iter()
                .map(|item| item["id"].as_u64().unwrap())
                .collect();
            assert_eq!(ranks, (SKIP..SKIP + TAKE).collect::<Vec<_>>());
            black_box(items);
        }
        SelectiveMergeMeasurements {
            source_bytes,
            peak_merge_page_bytes,
            retained_output_page_bytes,
            elapsed: start.elapsed(),
        }
    }

    #[tokio::test]
    async fn retention_high_water_includes_every_buffered_partition() {
        let measurements = selective_binary_merge_measurements(1).await;
        assert_eq!(
            measurements.peak_merge_page_bytes,
            measurements.source_bytes
        );
        assert!(measurements.peak_merge_page_bytes > measurements.retained_output_page_bytes);
    }

    #[tokio::test]
    async fn retention_high_water_includes_output_during_replenishment() {
        let first = crate::binary_json::test_support::reference_rows(&[("a", 0)], "first");
        let next =
            crate::binary_json::test_support::reference_rows(&[("b", 1)], &"next".repeat(1024));
        let sibling = crate::binary_json::test_support::reference_rows(&[("c", 2)], "sibling");
        let operation = mocks::operation()
            .with_supported_serialization_formats("CosmosBinary")
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
        let children = vec![
            ChildStream::fresh(
                range("", "80").unwrap(),
                Box::new(MockLeaf::with_pages(vec![
                    Ok(PageResult::Page {
                        response: mocks::response(&first),
                        is_terminal: false,
                    }),
                    Ok(PageResult::Page {
                        response: mocks::response(&next),
                        is_terminal: true,
                    }),
                ])),
            ),
            ChildStream::fresh(
                range("80", "FF").unwrap(),
                Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: mocks::response(&sibling),
                    is_terminal: true,
                })])),
            ),
        ];
        let mut merge = StreamingOrderedMerge::new(
            Arc::new(operation),
            vec![SortOrder::Ascending],
            children,
            "retention".to_owned(),
        );
        let observer = RetentionObserver::default();
        let peak_bytes = observer.peak_bytes.clone();
        merge.retention = Some(observer);
        let PageResult::Page { response, .. } = next_page(&mut merge).await else {
            panic!("expected an ordered page");
        };
        assert_eq!(ids(&response), ["a", "b"]);
        assert_eq!(
            peak_bytes.load(AtomicOrdering::Relaxed),
            first.len() + next.len() + sibling.len(),
        );
    }

    #[tokio::test]
    async fn retention_high_water_counts_shared_pages_once() {
        let page = crate::binary_json::test_support::reference_rows(&[("a", 0)], "shared");
        let response = mocks::response(&page);
        let operation = mocks::operation()
            .with_supported_serialization_formats("CosmosBinary")
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
        let children = [("", "80"), ("80", "FF")]
            .into_iter()
            .map(|(min, max)| {
                ChildStream::fresh(
                    range(min, max).unwrap(),
                    Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                        response: response.clone(),
                        is_terminal: true,
                    })])),
                )
            })
            .collect();
        let mut merge = StreamingOrderedMerge::new(
            Arc::new(operation),
            vec![SortOrder::Ascending],
            children,
            "shared-retention".to_owned(),
        );
        let observer = RetentionObserver::default();
        let peak_bytes = observer.peak_bytes.clone();
        merge.retention = Some(observer);
        let PageResult::Page { response, .. } = next_page(&mut merge).await else {
            panic!("expected an ordered page");
        };
        assert_eq!(ids(&response), ["a", "a"]);
        assert_eq!(peak_bytes.load(AtomicOrdering::Relaxed), page.len());
    }

    #[tokio::test]
    #[ignore = "run explicitly to characterize selective binary ORDER BY merging"]
    async fn measure_binary_order_by_selective_streaming_merge() {
        let measurements = selective_binary_merge_measurements(40).await;
        println!(
            "ORDER BY selective merge: fan_out=16 items_per_page=100 \
             source_bytes={} skip=20 take=10 rounds=40 \
             peak_retained_merge_page_bytes={} retained_output_page_bytes={} \
             instrumented_elapsed_ms={:.2}",
            measurements.source_bytes,
            measurements.peak_merge_page_bytes,
            measurements.retained_output_page_bytes,
            measurements.elapsed.as_secs_f64() * 1000.0,
        );
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
    /// `scope_min`/`scope_max` are the split child's range; the replacements
    /// must exactly tile it or [`SplitReplacements::try_tiling`] rejects them.
    fn split_page(
        scope_min: &str,
        scope_max: &str,
        replacement_nodes: Vec<Box<dyn PipelineNode>>,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::SplitRequired {
            replacements: SplitReplacements::try_tiling(
                &range(scope_min, scope_max)?,
                replacement_nodes,
            )?,
        })
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
    ) -> crate::error::Result<Box<dyn PipelineNode>> {
        Ok(Box::new(
            MockLeaf::with_pages(pages).with_feed_range(range(min, max)?),
        ))
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
    ) -> crate::error::Result<Box<dyn PipelineNode>> {
        Ok(Box::new(
            MockLeaf::with_pages(pages)
                .with_feed_range(range(min, max)?)
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("split-forwarded-ct".to_owned()),
                }),
        ))
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
    /// replacement (not just the first) must be filled before the next
    /// selection. P0 emits `[1, 2]` then splits into P0a (next `3`) and P0b
    /// (next `10, 20`); if P0b were left unfilled, `50` would be emitted
    /// before `10, 20`. A large page cap keeps popping within one page. The
    /// replacements carry a forwarded continuation (positioned past the `2`
    /// boundary), so the merge wraps and orders them without re-resolving and
    /// with no client-side discard.
    #[tokio::test]
    async fn split_during_pop_loop_fills_all_split_replacements() -> crate::error::Result<()> {
        let p0 = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                // The split child yields two sub-range leaves that carry the
                // forwarded continuation, so their pages already start past the
                // `2` boundary.
                split_page(
                    "",
                    "80",
                    vec![
                        positioned_replacement_leaf(
                            "",
                            "40",
                            vec![envelope_page(&[("d3", 3)], None)],
                        )?,
                        positioned_replacement_leaf(
                            "40",
                            "80",
                            vec![envelope_page(&[("d10", 10), ("d20", 20)], None)],
                        )?,
                    ],
                ),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF")?,
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
        Ok(())
    }

    /// Regression: a split landing mid-skip-run must not resurrect the
    /// already-emitted rows the run still owes.
    ///
    /// A resumed child carries `skip_count = 3` (three JOIN rows sharing
    /// `(rank=5, _rid="dup")` were emitted before the checkpoint). Its first
    /// page holds only two of those duplicates, so the discard consumes the
    /// whole page and stays armed with one skip left and nothing buffered —
    /// which forces a re-poll, and that is where the split lands. The
    /// replacement carries the forwarded continuation, so it resumes at the
    /// third duplicate: without carrying the live discard across, `d-dup3`
    /// would be emitted a second time.
    ///
    /// The remaining skip must be the decremented `1`, not the boundary's
    /// original `3` — rebuilding from the boundary would also swallow the
    /// legitimate `d-dup4` and `d9`.
    #[tokio::test]
    async fn split_mid_skip_run_carries_remaining_discard_to_replacement(
    ) -> crate::error::Result<()> {
        let mut child = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                // Page 1 is entirely already-emitted duplicates: the discard
                // eats both, leaving skip_count = 1 and an empty buffer.
                join_envelope_page(
                    &[("dup", 5, "d-dup1"), ("dup", 5, "d-dup2")],
                    Some("resumed-ct"),
                ),
                split_page(
                    "",
                    "80",
                    vec![positioned_replacement_leaf(
                        "",
                        "80",
                        vec![join_envelope_page(
                            &[("dup", 5, "d-dup3"), ("dup", 5, "d-dup4"), ("e", 9, "d9")],
                            None,
                        )],
                    )?],
                ),
            ])),
        );
        child.query_shape = ChildQueryShape::ResumeFilterInjected;
        child.last_emitted = Some(LastEmitted {
            resume_values: vec![OrderByResumeValue::Number { value: 5.0.into() }],
            rid: "dup".to_owned(),
            skip_count: 3,
        });
        child.pending_discard = number_boundary_discard(5.0, "dup", 3);

        let mut node = merge(vec![child], vec![SortOrder::Ascending]);

        let emitted = drain_all_ids(&mut node).await;
        assert_eq!(
            emitted,
            vec!["d-dup4", "d9"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            "the third duplicate is still owed to the skip run and must not re-emit, \
             while the fourth duplicate and the next key must survive"
        );
        Ok(())
    }

    /// Companion to the large-cap regression: with a page cap that fills
    /// exactly as the split completes, the merge must checkpoint immediately
    /// after the split — snapshotting *both* replacements — then resume in
    /// order on the next page. P0 emits `[1, 2]`, splits, emits `3`
    /// (cap = 3 reached), and P0b (`10, 20`) plus P1 (`50`) follow next page.
    #[tokio::test]
    async fn split_during_pop_loop_small_cap_checkpoints_after_split() -> crate::error::Result<()> {
        let p0 = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                split_page(
                    "",
                    "80",
                    vec![
                        positioned_replacement_leaf(
                            "",
                            "40",
                            vec![envelope_page(&[("d3", 3)], None)],
                        )?,
                        positioned_replacement_leaf(
                            "40",
                            "80",
                            vec![envelope_page(&[("d10", 10), ("d20", 20)], None)],
                        )?,
                    ],
                ),
            ])),
        );
        let p1 = ChildStream::fresh(
            range("80", "FF")?,
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
        Ok(())
    }

    /// A split replacement may itself split before it yields a row (cascading
    /// split). Each level supplies its own replacement leaves, each carrying a
    /// forwarded continuation, so ordering and resume correctness hold no
    /// matter how deep the cascade goes — with no client-side discard.
    #[tokio::test]
    async fn cascading_split_replacements_preserve_order_and_boundary() -> crate::error::Result<()>
    {
        // P0 emits [1, 2] then splits into P0a + P0b; P0a splits *again* into
        // P0a1 (next 3) and P0a2 (next 5) before yielding a row.
        let p0a_cascade = split_page(
            "",
            "40",
            vec![
                positioned_replacement_leaf("", "20", vec![envelope_page(&[("d3", 3)], None)])?,
                positioned_replacement_leaf("20", "40", vec![envelope_page(&[("d5", 5)], None)])?,
            ],
        );
        let p0 = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct")),
                split_page(
                    "",
                    "80",
                    vec![
                        positioned_replacement_leaf("", "40", vec![p0a_cascade])?,
                        positioned_replacement_leaf(
                            "40",
                            "80",
                            vec![envelope_page(&[("d10", 10)], None)],
                        )?,
                    ],
                ),
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
        Ok(())
    }

    #[tokio::test]
    async fn cross_partition_ties_are_broken_by_range() -> crate::error::Result<()> {
        // Both children have rank=1; the leftmost EPK range wins regardless
        // of RID, matching .NET and Java.
        let left = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![envelope_page(&[("b", 1)], None)])),
        );
        let right = ChildStream::fresh(
            range("80", "FF")?,
            Box::new(MockLeaf::with_pages(vec![envelope_page(&[("a", 1)], None)])),
        );
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["b".to_owned(), "a".to_owned()]);
        Ok(())
    }

    #[tokio::test]
    async fn buffered_only_page_preserves_merged_session_token() -> crate::error::Result<()> {
        let left = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page_with_session_token(&[("a", 1)], "0:1#10"),
            ])),
        );
        let right = ChildStream::fresh(
            range("80", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page_with_session_token(&[("b", 2)], "1:1#20"),
            ])),
        );
        let operation = Arc::new(
            mocks::operation().with_max_item_count(MaxItemCountHint::Limit(
                std::num::NonZeroU32::new(1).unwrap(),
            )),
        );
        let mut node = StreamingOrderedMerge::new(
            operation,
            vec![SortOrder::Ascending],
            vec![left, right],
            "test-fingerprint".to_owned(),
        );

        let PageResult::Page {
            response: first, ..
        } = next_page(&mut node).await
        else {
            panic!("expected first page");
        };
        let PageResult::Page {
            response: second, ..
        } = next_page(&mut node).await
        else {
            panic!("expected buffered second page");
        };
        assert_eq!(
            first
                .headers()
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("0:1#10,1:1#20")
        );
        assert_eq!(
            second
                .headers()
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("0:1#10,1:1#20")
        );
        Ok(())
    }

    #[tokio::test]
    async fn partial_fill_failure_preserves_absorbed_session_token() -> crate::error::Result<()> {
        let left = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page_with_session_token(&[("a", 1)], "0:1#10"),
            ])),
        );
        let transient = crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::ServiceUnavailable,
            ))
            .with_message("transient")
            .build();
        let right = ChildStream::fresh(
            range("80", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                Err(transient),
                envelope_page_with_session_token(&[("b", 2)], "1:1#20"),
            ])),
        );
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert!(node.next_page(&mut context).await.is_err());
        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected retry page");
        };
        assert_eq!(
            response
                .headers()
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("0:1#10,1:1#20")
        );
        Ok(())
    }

    #[tokio::test]
    async fn absorbed_session_token_survives_page_parse_failure() -> crate::error::Result<()> {
        // A page can absorb cleanly and *then* fail to parse — a complex
        // (array) ORDER BY key is rejected downstream of `absorb`. The token
        // it carried must still be committed, or session state from a response
        // we already received is lost for good.
        let left = ChildStream::fresh(
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page_with_session_token(&[("a", 1)], "0:1#10"),
            ])),
        );
        // First page: complex key, carries a token. Second: parses, no token.
        let unparseable = {
            let response = array_envelope_response(&[("b", 2)], None);
            let headers = crate::models::CosmosResponseHeaders {
                session_token: Some(SessionToken::new("1:1#20")),
                ..Default::default()
            };
            Ok(PageResult::Page {
                response: crate::models::CosmosResponse::new(
                    response.body_bytes().to_vec(),
                    headers,
                    response.status(),
                    response.diagnostics(),
                ),
                is_terminal: true,
            })
        };
        let right = ChildStream::fresh(
            range("80", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                unparseable,
                Ok(PageResult::Page {
                    response: envelope_response(&[("b", 2)], None),
                    is_terminal: true,
                }),
            ])),
        );
        let mut node = merge(vec![left, right], vec![SortOrder::Ascending]);
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert!(node.next_page(&mut context).await.is_err());
        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected retry page");
        };
        // The retry page carries no token of its own, so `1:1#20` can only be
        // present if the failed fill committed it.
        assert_eq!(
            response
                .headers()
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("0:1#10,1:1#20")
        );
        Ok(())
    }

    #[tokio::test]
    async fn multi_column_mixed_direction_drives_pop_order() -> crate::error::Result<()> {
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
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![two_col_page("left", 1, "a")])),
        );
        let right = ChildStream::fresh(
            range("80", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn undefined_sorts_before_defined_values_across_partitions() -> crate::error::Result<()> {
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
            range("", "80")?,
            Box::new(MockLeaf::with_pages(vec![page("has-value", Some(1))])),
        );
        let right = ChildStream::fresh(
            range("80", "FF")?,
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
        Ok(())
    }

    /// An array/object ORDER BY value fails the query deterministically:
    /// the backend sorts complex values by a structural hash the client
    /// cannot reproduce from JSON, so any client-side merge would silently
    /// mis-order them. Python and JavaScript reject these the same way.
    #[tokio::test]
    async fn complex_order_by_values_are_rejected() -> crate::error::Result<()> {
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![array_envelope_page(
                &[("a", 5)],
                None,
            )])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let mut executor = mocks::MockRequestExecutor::new(vec![]);
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let error = node
            .next_page(&mut context)
            .await
            .expect_err("an array ORDER BY value must fail the query");
        assert_eq!(
            error.status().sub_status(),
            Some(crate::error::status_codes::substatus::CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED),
        );
        Ok(())
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
    async fn resume_with_boundary_discards_already_emitted_ties_by_rid() -> crate::error::Result<()>
    {
        // Rows tied on rank=5 with `_rid <= "tied-2"` were already emitted.
        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn resume_uses_backend_reverse_index_scan_for_rid_discard() -> crate::error::Result<()> {
        let mut child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page_with_execution_info(
                    &[("c", 5), ("b", 5), ("a", 5)],
                    r#"{"reverseRidEnabled":false,"reverseIndexScan":true}"#,
                ),
            ])),
        );
        child.pending_discard = PendingDiscard::ResumeBoundary {
            resume_values: vec![OrderByResumeValue::Number {
                value: 5_i64.into(),
            }],
            last_rid: "b".to_owned(),
            skip_count: 1,
            directions: vec![SortOrder::Ascending],
        };
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);

        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["a".to_owned()]);
        Ok(())
    }

    /// Regression: an empty leading page (with continuation) must keep the
    /// boundary discard active, not clear it, or later tied rows leak through.
    #[tokio::test]
    async fn resume_boundary_discard_survives_empty_leading_page() -> crate::error::Result<()> {
        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    /// Regression: a tie run spanning a page boundary must stay fully
    /// discarded, not just the portion on the first page.
    #[tokio::test]
    async fn resume_boundary_discard_survives_tie_run_spanning_pages() -> crate::error::Result<()> {
        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    /// Regression: after a split, both sub-ranges resume from the same
    /// boundary; the `_rid`-aware discard avoids dropping/duplicating rows.
    #[tokio::test]
    async fn split_resume_is_rid_aware_with_no_omissions_or_duplicates() -> crate::error::Result<()>
    {
        // Pre-split emitted a,b,c tied on rank=5; left keeps a,c + e (unemitted tie) + m;
        // right keeps b + z.
        let left = {
            let mut c = ChildStream::fresh(
                range("", "80")?,
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
                range("80", "FF")?,
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
        Ok(())
    }

    // ── skip_count: JOIN duplicate-RID resume ────────────────────────────

    #[test]
    fn record_emission_tracks_skip_count_for_duplicate_key_rid() -> crate::error::Result<()> {
        let mut child =
            ChildStream::fresh(range("", "FF")?, Box::new(MockLeaf::with_pages(vec![])));
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
        Ok(())
    }

    #[tokio::test]
    async fn resume_skips_exactly_skip_count_duplicate_rid_rows() -> crate::error::Result<()> {
        // Resume at (rank=5, rid=docA) after emitting 2 of docA's JOIN rows.
        // The page re-returns all 3 docA rows plus a docB row; the discard
        // drops exactly 2 (the emitted duplicates) and keeps the third, then
        // the later document.
        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn resume_skip_count_persists_across_pages() -> crate::error::Result<()> {
        // skip_count = 3, but docA's JOIN rows straddle a page boundary: 2 on
        // the first page, 2 on the second. The discard must carry the residual
        // skip across the page break, dropping exactly 3 docA rows total.
        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn legacy_boundary_missing_skip_count_discards_boundary_row() -> crate::error::Result<()>
    {
        // A continuation token minted before `skip_count` existed omits the
        // field; it must deserialize as skip_count == 1 so the single boundary
        // row is still dropped on resume, never re-emitted.
        let boundary: ValueBoundary = serde_json::from_str(
            r#"{"resume_values":[{"type":"number","value":5.0}],"last_rid":"boundary"}"#,
        )
        .unwrap();
        assert_eq!(boundary.skip_count, 1);

        let mut child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
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
    async fn live_split_forwarded_continuation_emits_post_boundary_join_rows(
    ) -> crate::error::Result<()> {
        let id_1_100: Vec<String> = (1..=100).map(|i| format!("a{i}")).collect();
        let id_101_150: Vec<String> = (101..=150).map(|i| format!("a{i}")).collect();
        let page1_rows: Vec<(&str, i64, &str)> =
            id_1_100.iter().map(|s| ("docA", 5, s.as_str())).collect();
        let page2_rows: Vec<(&str, i64, &str)> =
            id_101_150.iter().map(|s| ("docA", 5, s.as_str())).collect();

        // The split child yields a real `Request` replacement carrying the
        // forwarded continuation, mirroring `split_for_topology_change`.
        let target = RequestTarget::effective_partition_key_range(
            range("", "FF")?,
            "pk-0".to_owned(),
            range("", "FF")?,
        );
        let replacement: Box<dyn PipelineNode> = Box::new(Request::new(
            Arc::new(mocks::operation()),
            target,
            Some("p0-ct".to_owned()),
        ));
        let split_child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                join_envelope_page(&page1_rows, Some("p0-ct")),
                split_page("", "FF", vec![replacement]),
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
        Ok(())
    }

    /// A live split whose replacement reports no forwarded continuation, yet
    /// the split child had already emitted a row (boundary set), cannot be
    /// safely repositioned — a generic node's resume position is unknown.
    /// Reattaching the `skip_count` discard could drop or duplicate rows, so
    /// the merge rejects with a typed `SPLIT_REPLACEMENT_INVALID` error.
    #[tokio::test]
    async fn live_split_replacement_without_continuation_with_boundary_errors(
    ) -> crate::error::Result<()> {
        let split_child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![split_page(
                "",
                "FF",
                vec![
                    replacement_leaf("", "80", vec![envelope_page(&[("d3", 3)], None)])?,
                    replacement_leaf("80", "FF", vec![envelope_page(&[("d9", 9)], None)])?,
                ],
            )])),
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
            Some(crate::error::status_codes::substatus::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
            "must surface the typed split-replacement-invalid error (20215), got: {err}"
        );
        Ok(())
    }

    /// The companion accept case: a child that live-splits *before* emitting
    /// any row has no boundary to protect, so generic replacements (no
    /// forwarded continuation) are accepted and their rows stream through
    /// fresh, in order.
    #[tokio::test]
    async fn live_split_initial_no_boundary_accepts_generic_replacements(
    ) -> crate::error::Result<()> {
        let split_child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![split_page(
                "",
                "FF",
                vec![
                    replacement_leaf("", "80", vec![envelope_page(&[("d3", 3)], None)])?,
                    replacement_leaf("80", "FF", vec![envelope_page(&[("d9", 9)], None)])?,
                ],
            )])),
        );
        // `last_emitted` stays `None` — no row emitted before the split.
        let mut node = merge(vec![split_child], vec![SortOrder::Ascending]);
        let emitted = drain_all_ids(&mut node).await;
        assert_eq!(
            emitted,
            vec!["d3".to_owned(), "d9".to_owned()],
            "an initial split with no boundary accepts fresh replacements"
        );
        Ok(())
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

    fn resolved_range(min: &str, max: &str, id: &str) -> crate::error::Result<ResolvedRange> {
        Ok(ResolvedRange {
            partition_key_range_id: id.to_owned(),
            parents: Vec::new(),
            range: range(min, max)?,
        })
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

    /// A scalar boundary crossing a split fans out into one resume-filtered
    /// child per sub-range, each with the boundary discard installed.
    #[test]
    fn build_children_splits_scalar_boundary_into_resume_filtered_children(
    ) -> crate::error::Result<()> {
        let op = query_operation();
        let scope = range("", "FF")?;
        let resolved = vec![
            resolved_range("", "80", "pk-left")?,
            resolved_range("80", "FF", "pk-right")?,
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
        Ok(())
    }

    /// A merge resolves the saved sub-range to a wider physical range; it
    /// must be clipped to scope before coverage validation, not rejected.
    #[test]
    fn build_children_clips_merged_physical_range_to_scope() -> crate::error::Result<()> {
        let op = query_operation();
        let scope = range("", "80")?;
        // Post-merge: the saved [00,80) sub-range is now served by [00,FF).
        let resolved = vec![resolved_range("", "FF", "pk-merged")?];
        let boundary = scalar_boundary(5.0, "c");
        let children = build_children(&resolved, &scope, &op, ASC, None, Some(&boundary))
            .expect("a merged (widened) physical range clips to the saved scope");
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].range, scope);
        Ok(())
    }

    #[tokio::test]
    async fn page_size_cap_retains_unread_rows_for_next_call() -> crate::error::Result<()> {
        let child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn terminal_page_reports_drained_children() -> crate::error::Result<()> {
        let child = ChildStream::fresh(
            range("", "FF")?,
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
        Ok(())
    }

    #[tokio::test]
    async fn malformed_envelope_surfaces_typed_error() -> crate::error::Result<()> {
        let body = serde_json::json!({
            "_rid": "",
            "Documents": [{"_rid": "a", "orderByItems": [{"item": 1}]}], // missing payload
            "_count": 1,
        });
        let child = ChildStream::fresh(
            range("", "FF")?,
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
            crate::error::status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID
        );
        Ok(())
    }

    /// Regression: a page committed by the child `Request` but rejected during
    /// validation leaves that continuation pointing past rows the caller never
    /// received. Snapshotting it would silently drop them on resume, so the
    /// snapshot must fall back to the value boundary instead.
    #[tokio::test]
    async fn snapshot_after_failed_page_validation_drops_server_continuation(
    ) -> crate::error::Result<()> {
        // Missing `payload` — rejected by `parse_envelope_page` *after* the
        // leaf committed `ct-2`.
        let malformed = serde_json::json!({
            "_rid": "",
            "Documents": [{"_rid": "d2", "orderByItems": [{"item": 2}]}],
            "_count": 1,
        });
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(
                MockLeaf::with_pages(vec![
                    envelope_page(&[("d1", 1)], Some("ct-1")),
                    Ok(PageResult::Page {
                        response: mocks::response_with_continuation(
                            &serde_json::to_vec(&malformed).unwrap(),
                            Some("ct-2"),
                        ),
                        is_terminal: false,
                    }),
                ])
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("ct-2".to_owned()),
                }),
            ),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);

        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected the already-consumed row to be returned as a partial page");
        };
        assert_eq!(ids(&response), vec!["d1"]);

        match node.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert!(
                    ranges[0].server_continuation.is_none(),
                    "the continuation of a page that failed validation must not be snapshotted"
                );
                assert_eq!(
                    ranges[0]
                        .boundary
                        .as_ref()
                        .expect("the emitted row advanced the boundary")
                        .last_rid,
                    "d1",
                    "the range must resume from the last row actually delivered"
                );
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }
        Ok(())
    }

    /// Regression: the same hazard on the *priming* path. With a page size of 1
    /// the emit loop never reaches the in-loop refill, so the failing fetch
    /// lands on the next call's priming step — after an earlier call already
    /// emitted rows. That fetch still commits `ct-2` before validation rejects
    /// it, so the snapshot must fall back to the boundary here too.
    #[tokio::test]
    async fn snapshot_after_failed_priming_validation_drops_server_continuation(
    ) -> crate::error::Result<()> {
        let malformed = serde_json::json!({
            "_rid": "",
            "Documents": [{"_rid": "d2", "orderByItems": [{"item": 2}]}],
            "_count": 1,
        });
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(
                MockLeaf::with_pages(vec![
                    envelope_page(&[("d1", 1)], Some("ct-1")),
                    Ok(PageResult::Page {
                        response: mocks::response_with_continuation(
                            &serde_json::to_vec(&malformed).unwrap(),
                            Some("ct-2"),
                        ),
                        is_terminal: false,
                    }),
                ])
                .with_snapshot(PipelineNodeState::Request {
                    server_continuation: Some("ct-2".to_owned()),
                }),
            ),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        node.plain_operation = Arc::new((*node.plain_operation).clone().with_max_item_count(
            crate::models::MaxItemCountHint::Limit(std::num::NonZeroU32::new(1).unwrap()),
        ));

        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        // Fills the buffer and emits `d1`; the cap is reached before the
        // in-loop refill, so nothing has been marked unsafe yet.
        let PageResult::Page { response, .. } = node.next_page(&mut context).await.unwrap() else {
            panic!("expected a page");
        };
        assert_eq!(ids(&response), vec!["d1"]);

        // The malformed page is fetched while priming, with no row emitted on
        // this call.
        assert!(node.next_page(&mut context).await.is_err());

        match node.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert!(
                    ranges[0].server_continuation.is_none(),
                    "a continuation committed while priming and then rejected must not be \
                     snapshotted, or the resumed query skips the page it stands for"
                );
                assert_eq!(
                    ranges[0]
                        .boundary
                        .as_ref()
                        .expect("the earlier page advanced the boundary")
                        .last_rid,
                    "d1",
                    "the range must resume from the last row actually delivered"
                );
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }
        Ok(())
    }

    /// Regression: a per-item encode failure must not consume the row it fails
    /// on.
    ///
    /// The emitted encoding is re-derived per item, so an item can fail to
    /// encode after earlier items in the same page succeeded. If the boundary
    /// advanced first, the failing row would already sit behind the resume
    /// point: the caller retries, resumes past it, and the document vanishes
    /// with no error on the second attempt. Encoding before
    /// `record_emission` keeps the row replayable.
    ///
    /// Nesting past `binary_json::reader::MAX_DEPTH` is the injection because
    /// it is the one input the encoder rejects by contract; `parse_envelope_page`
    /// retains payloads as `RawValue`, which does not walk the document, so it
    /// reaches the encoder intact.
    #[tokio::test]
    async fn binary_encode_failure_does_not_advance_boundary_past_the_failing_row(
    ) -> crate::error::Result<()> {
        let mut deep = serde_json::json!(1);
        for _ in 0..(crate::binary_json::reader::MAX_DEPTH + 8) {
            deep = serde_json::Value::Array(vec![deep]);
        }
        let body = serde_json::json!({
            "_rid": "",
            "Documents": [
                {"_rid": "d1", "orderByItems": [{"item": 1}], "payload": {"id": "d1"}},
                {"_rid": "d2", "orderByItems": [{"item": 2}], "payload": {"id": "d2", "deep": deep}},
            ],
            "_count": 2,
        });
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: mocks::response_with_continuation(
                    &serde_json::to_vec(&body).unwrap(),
                    None,
                ),
                is_terminal: true,
            })])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        node.emit_binary = true;

        let PageResult::Page { response, .. } = next_page(&mut node).await else {
            panic!("expected the successfully-encoded rows as a partial page");
        };
        let items = match response.body() {
            crate::models::ResponseBody::Items(items) => items.clone(),
            other => panic!("expected an Items body, got {other:?}"),
        };
        assert_eq!(items.len(), 1, "only `d1` encodes");
        assert_eq!(
            response.headers().item_count,
            Some(1),
            "the item count must report what was emitted, not what was popped"
        );

        match node.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                assert_eq!(
                    ranges[0]
                        .boundary
                        .as_ref()
                        .expect("`d1` was delivered, so the boundary advanced to it")
                        .last_rid,
                    "d1",
                    "the boundary must not pass the row that failed to encode, or a resume \
                     silently skips it"
                );
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }

        // The deferred failure is still delivered, so the caller learns the
        // page is incomplete rather than seeing a short page and stopping.
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let err = node
            .next_page(&mut context)
            .await
            .expect_err("the deferred encode failure must surface");
        assert_eq!(
            err.status(),
            crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        Ok(())
    }

    /// When the *first* row fails to encode there is nothing to defer the
    /// failure behind, so `next_page` returns `Err` and drops the
    /// `PageAggregator` along with the absorbed page's session token. That is
    /// safe because `ensure_stream_filled` already committed the merged token
    /// to the node; without that commit a retry could issue a read weaker than
    /// one the session had satisfied.
    #[tokio::test]
    async fn empty_page_encode_failure_keeps_the_absorbed_session_token() -> crate::error::Result<()>
    {
        let mut deep = serde_json::json!(1);
        for _ in 0..(crate::binary_json::reader::MAX_DEPTH + 8) {
            deep = serde_json::Value::Array(vec![deep]);
        }
        // The *only* row fails, so `items` is still empty at the failure.
        let body = serde_json::json!({
            "_rid": "",
            "Documents": [
                {"_rid": "d1", "orderByItems": [{"item": 1}], "payload": {"id": "d1", "deep": deep}},
            ],
            "_count": 1,
        });
        let response = mocks::response_with_continuation(&serde_json::to_vec(&body).unwrap(), None);
        let headers = crate::models::CosmosResponseHeaders {
            session_token: Some(SessionToken::new("0:1#10")),
            ..Default::default()
        };
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: crate::models::CosmosResponse::new(
                    response.body_bytes().to_vec(),
                    headers,
                    response.status(),
                    response.diagnostics(),
                ),
                is_terminal: true,
            })])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        node.emit_binary = true;

        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let err = node
            .next_page(&mut context)
            .await
            .expect_err("the only row fails to encode, so there is no partial page to emit");
        assert_eq!(
            err.status(),
            crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
        );
        assert_eq!(
            node.session_token.as_ref().map(SessionToken::as_str),
            Some("0:1#10"),
            "the absorbed page's session token must outlive the discarded aggregator"
        );
        Ok(())
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
    fn snapshot_suppresses_resume_filtered_child_backend_continuation() -> crate::error::Result<()>
    {
        fn child_with_live_continuation(
            shape: ChildQueryShape,
            token: &str,
        ) -> crate::error::Result<ChildStream> {
            // Empty buffer, not drained, with a live backend continuation.
            let mut child = ChildStream::fresh(
                range("", "FF")?,
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
            Ok(child)
        }

        let resume_filtered = merge(
            vec![child_with_live_continuation(
                ChildQueryShape::ResumeFilterInjected,
                "resume-filtered-tok",
            )?],
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
            )?],
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
        Ok(())
    }

    #[tokio::test]
    async fn cancellation_error_propagates_from_child_fetch() -> crate::error::Result<()> {
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![Err(
                mocks::non_topology_gone_error(),
            )])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);
        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(node.next_page(&mut context).await.is_err());
        Ok(())
    }

    /// Regression: a fetch failure that happens *after* rows were consumed
    /// into the page under construction must not discard those rows. Emitting
    /// a row advances its child's resume boundary, and
    /// `azure_data_cosmos::feed::iterator` deliberately keeps the plan alive
    /// on error so the caller can still capture a continuation token — so
    /// dropping the page would permanently skip every consumed row. The merge
    /// returns the partial page instead and surfaces the error on the next
    /// call.
    #[tokio::test]
    async fn fetch_failure_after_emission_returns_partial_page_then_error(
    ) -> crate::error::Result<()> {
        // A single child so the failing replenish happens inside the pop loop
        // rather than during the up-front fill pass.
        let child = ChildStream::fresh(
            range("", "FF")?,
            Box::new(MockLeaf::with_pages(vec![
                envelope_page(&[("d1", 1)], Some("ct-1")),
                Err(mocks::non_topology_gone_error()),
            ])),
        );
        let mut node = merge(vec![child], vec![SortOrder::Ascending]);

        let PageResult::Page {
            response,
            is_terminal,
        } = next_page(&mut node).await
        else {
            panic!("expected the already-consumed row to be returned as a partial page");
        };
        assert_eq!(
            ids(&response),
            vec!["d1"],
            "the row consumed before the failure must still be delivered"
        );
        assert!(
            !is_terminal,
            "a stream with a deferred error is never terminal"
        );

        // The boundary matches exactly what was delivered, so a token captured
        // here resumes at the right place.
        match node.snapshot_state().unwrap() {
            PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
                let boundary = ranges[0]
                    .boundary
                    .as_ref()
                    .expect("the emitted row advanced the boundary");
                assert_eq!(boundary.last_rid, "d1");
            }
            other => panic!("expected StreamingOrderedMerge, got {other:?}"),
        }

        let mut executor = mocks::NoopRequestExecutor;
        let mut topology = mocks::NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(
            node.next_page(&mut context).await.is_err(),
            "the deferred error must surface on the next call, not be swallowed"
        );
        Ok(())
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
    async fn catalog_mock_pipeline_scenarios_drain_in_expected_order() -> crate::error::Result<()> {
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
                // Error scenarios are covered by dedicated Rust tests instead:
                // see `malformed_envelope_surfaces_typed_error` and
                // `complex_order_by_values_are_rejected`.
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
                    range(&lo, &hi)?,
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
        Ok(())
    }

    /// The same query body under two different feed scopes must not share a
    /// fingerprint. A resumed node treats its saved ranges as authoritative,
    /// so a token replayed under a narrower or wider scope would otherwise
    /// read outside the caller's scope (or silently return only the old
    /// subset).
    #[test]
    fn query_fingerprint_distinguishes_feed_scope() -> crate::error::Result<()> {
        let body = br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#;
        let full = query_fingerprint(Some(body), Some(&FeedRange::full()));
        let left = query_fingerprint(Some(body), Some(&range("", "80")?));
        let right = query_fingerprint(Some(body), Some(&range("80", "FF")?));
        let unscoped = query_fingerprint(Some(body), None);

        assert_ne!(full, left);
        assert_ne!(full, right);
        assert_ne!(left, right);
        // An absent scope is its own value, distinct from the full container.
        assert_ne!(full, unscoped);
        Ok(())
    }

    /// Neither separator can appear inside an EPK hex bound, so no pair of
    /// distinct (body, scope) inputs can serialize to the same hash preimage.
    #[test]
    fn query_fingerprint_separators_cannot_collide() -> crate::error::Result<()> {
        // Both scopes render as `408080` once the bound separator is dropped.
        assert_ne!(
            query_fingerprint(None, Some(&range("40", "8080")?)),
            query_fingerprint(None, Some(&range("4080", "80")?)),
        );
        Ok(())
    }

    /// `EffectivePartitionKey`'s `Ord` treats trailing zero bytes as
    /// insignificant, and the backend and other SDKs may hand back a bound with
    /// that padding trimmed. Bounds that compare equal must fingerprint alike,
    /// or a valid resume fails with a hard token error.
    #[test]
    fn query_fingerprint_ignores_trailing_zero_padding_in_scope() -> crate::error::Result<()> {
        assert_eq!(
            query_fingerprint(None, Some(&range("", "80")?)),
            query_fingerprint(None, Some(&range("", "8000")?)),
        );
        assert_eq!(
            query_fingerprint(None, Some(&range("40", "80")?)),
            query_fingerprint(None, Some(&range("400000", "8000")?)),
        );
        Ok(())
    }

    /// Preserve the exact fingerprint minted before binary query encoding so
    /// in-flight text tokens remain valid.
    #[test]
    fn parameterized_text_query_fingerprint_matches_historical_value() -> crate::error::Result<()> {
        let body = br#"{"query":"SELECT * FROM c WHERE c.rank >= @min ORDER BY c.rank","parameters":[{"name":"@min","value":1}]}"#;
        assert_eq!(
            query_fingerprint(Some(body), Some(&range("", "80")?)),
            "b84b9c269862dcd73781038d90add3be",
        );
        Ok(())
    }
}
