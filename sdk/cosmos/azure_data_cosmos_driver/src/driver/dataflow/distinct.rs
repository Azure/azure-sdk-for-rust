// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-partition `DISTINCT` deduplication.
//!
//! [`Distinct`] wraps the cross-partition fan-out root and drops rows whose
//! projected payload is structurally equal to one already emitted. It sits
//! *above* the merge, matching .NET's `DistinctQueryPipelineStage` and Java's
//! `DistinctDocumentQueryExecutionContext`:
//!
//! ```text
//!   Unordered:  Distinct -> SequentialDrain        -> Request…
//!   Ordered:    Distinct -> StreamingOrderedMerge  -> Request…
//! ```
//!
//! Both peers key on the **whole projected row**, not on the `ORDER BY` items,
//! so one node serves both modes and only the map differs.
//!
//! # Ordered vs unordered
//!
//! - [`DistinctMap::Ordered`] retains a single hash. `ORDER BY` guarantees
//!   structurally equal rows arrive adjacently, so comparing against the last
//!   emitted value is sufficient and the node runs in O(1) memory.
//! - [`DistinctMap::Unordered`] retains every hash seen. There is no ordering
//!   to exploit, so a duplicate may arrive arbitrarily far from its twin —
//!   including from a different partition.
//!
//! # Continuation
//!
//! Ordered `DISTINCT` is resumable: the 16-byte `last_hash` is all a resumed
//! node needs, because a value it has moved past can never reappear.
//!
//! Unordered `DISTINCT` is not. The set *is* the state, and serializing it
//! would mean an unbounded token; truncating it would silently re-emit
//! duplicates. [`Distinct::snapshot_state`] therefore fails with
//! [`CosmosStatus::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED`], which surfaces
//! at `OperationPlan::to_continuation_token` time — while the caller still
//! holds a live plan and can either keep draining in-process or rewrite the
//! query with a matching `ORDER BY`. .NET refuses here too, with the same
//! guidance. Java does not: it emits a token carrying only the source
//! continuation, then rebuilds an empty map on resume, so an unordered
//! `DISTINCT` resumed from a Java token silently re-emits values it already
//! returned.
//!
//! # Splits
//!
//! The wrapped fan-out node absorbs `SplitRequired` internally and is never
//! rebuilt, so the map survives a split mid-drain and an already-emitted value
//! cannot be resurrected. A split that does reach this node is refused rather
//! than forwarded: `SplitRequired` replaces the node that emits it, so passing
//! it up would discard the map along with the node.

use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;

use crate::diagnostics::DiagnosticsContext;
use crate::error::CosmosStatus;
use crate::models::{CosmosResponse, FeedRange, RequestCharge, ResponseBody};

use super::distinct_hash::{hash_value, Hash128};
use super::query_plan::DistinctType;
use super::{skip_take_page, PageResult, PipelineContext, PipelineNode, PipelineNodeState};

/// Guidance surfaced when a caller asks for a continuation token on an
/// unordered `DISTINCT` query. Mirrors .NET's
/// `DisallowContinuationTokenMessages.Distinct`.
const UNORDERED_CONTINUATION_MESSAGE: &str =
    "DISTINCT queries only return continuation tokens when there is a matching ORDER BY clause. \
     For example, rewrite `SELECT DISTINCT VALUE c.name FROM c` as \
     `SELECT DISTINCT VALUE c.name FROM c ORDER BY c.name`.";

/// Deduplication state, keyed on a 128-bit structural hash of the row payload.
enum DistinctMap {
    /// Adjacency deduplication over an `ORDER BY`-sorted stream. `None` until
    /// the first row is emitted (or, on resume, seeded from the token).
    Ordered { last_hash: Option<Hash128> },

    /// Global deduplication over an unordered stream.
    ///
    /// Unbounded by design, matching .NET's `UnorderedDistinctMap` and Java's
    /// `UnorderedDistinctMap`: ~16 bytes per *distinct* value seen. Since the
    /// query cannot be resumed anyway, the set only has to survive one drain.
    Unordered { seen: HashSet<Hash128> },
}

impl DistinctMap {
    /// Records `hash` and reports whether the row should be emitted.
    fn accept(&mut self, hash: Hash128) -> bool {
        match self {
            Self::Ordered { last_hash } => {
                let is_new = *last_hash != Some(hash);
                // Advance unconditionally: a non-adjacent repeat is a different
                // run and must reset the comparison point.
                *last_hash = Some(hash);
                is_new
            }
            Self::Unordered { seen } => seen.insert(hash),
        }
    }
}

/// Deduplicates its single child's pages by structural payload equality.
pub(crate) struct Distinct {
    child: Box<dyn PipelineNode>,
    map: DistinctMap,
    /// Set once the child drains so subsequent pulls short-circuit.
    exhausted: bool,
    /// Request charge from pages whose rows were entirely duplicates and were
    /// therefore suppressed rather than emitted as empty pages. Folded into the
    /// next emitted page so billed RUs are never under-reported.
    suppressed_charge: RequestCharge,
    /// Diagnostics from those same pages, folded incrementally into a single
    /// context rather than retained one-per-page: `aggregate_sub_operations`
    /// re-bounds its record list to `max_request_diagnostics`, so a long run of
    /// all-duplicate pages cannot grow the artifact without limit.
    suppressed_diagnostics: Option<Arc<DiagnosticsContext>>,
    /// The most recently suppressed page, kept as a template so accumulated
    /// charge/diagnostics can still be flushed as a final empty page if the
    /// child drains without ever surfacing a terminal page.
    pending_flush: Option<CosmosResponse>,
}

impl Distinct {
    /// Wraps `child` with a fresh map for `distinct_type`.
    #[cfg(test)]
    pub(crate) fn new(child: Box<dyn PipelineNode>, distinct_type: DistinctType) -> Self {
        Self::with_last_hash(child, distinct_type, None)
    }

    /// Wraps `child`, seeding an ordered map with the hash of the last row
    /// emitted before a checkpoint. `last_hash` is ignored for an unordered
    /// map, whose state is never persisted.
    pub(crate) fn with_last_hash(
        child: Box<dyn PipelineNode>,
        distinct_type: DistinctType,
        last_hash: Option<Hash128>,
    ) -> Self {
        let map = match distinct_type {
            // `None` never reaches here — the planner only wraps a root when
            // the plan asks for deduplication — but treating it as unordered
            // keeps this total and errs toward deduplicating rather than
            // silently passing duplicates through.
            DistinctType::Unordered | DistinctType::None => DistinctMap::Unordered {
                seen: HashSet::new(),
            },
            DistinctType::Ordered => DistinctMap::Ordered { last_hash },
        };
        Self {
            child,
            map,
            exhausted: false,
            suppressed_charge: RequestCharge::default(),
            suppressed_diagnostics: None,
            pending_flush: None,
        }
    }

    /// Selects the items that survive deduplication, preserving order and each
    /// item's exact backend bytes.
    fn select_survivors(&mut self, items: &[Bytes]) -> crate::error::Result<Vec<Bytes>> {
        let mut keep = Vec::with_capacity(items.len());
        for item in items {
            let value: serde_json::Value = serde_json::from_slice(item).map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to parse a DISTINCT row payload as JSON")
                    .with_source(e)
                    .build()
            })?;
            if self.map.accept(hash_value(&value)?) {
                keep.push(item.clone());
            }
        }
        Ok(keep)
    }

    /// Rebuilds a response around a trimmed body, updating `x-ms-item-count`
    /// and folding in charge/diagnostics accumulated from suppressed pages.
    fn rebuild(
        &mut self,
        response: &CosmosResponse,
        survivors: Vec<Bytes>,
        emitted: usize,
    ) -> CosmosResponse {
        let mut headers = response.headers().clone();
        headers.item_count = Some(emitted as u32);
        if self.suppressed_charge != RequestCharge::default() {
            let base = headers.request_charge.unwrap_or_default();
            headers.request_charge = Some(base + self.suppressed_charge);
        }
        let rebuilt = CosmosResponse::new(
            ResponseBody::from_items(survivors),
            headers,
            response.status(),
            response.diagnostics(),
        );
        let merged = match self.suppressed_diagnostics.as_ref() {
            Some(accumulated) => {
                rebuilt.with_aggregated_prior_diagnostics(std::slice::from_ref(accumulated))
            }
            None => rebuilt,
        };
        self.clear_suppressed();
        merged
    }

    /// Accumulates an all-duplicate page's charge and diagnostics.
    fn suppress(&mut self, response: CosmosResponse) {
        self.suppressed_charge =
            self.suppressed_charge + response.headers().request_charge.unwrap_or_default();
        let incoming = response.diagnostics();
        // Fold on arrival so only one context is ever retained. The newest page
        // stays last, preserving the aggregate's "operation-level fields come
        // from the final source" contract.
        self.suppressed_diagnostics = match self.suppressed_diagnostics.take() {
            None => Some(incoming),
            Some(accumulated) => {
                DiagnosticsContext::aggregate_sub_operations(&[accumulated, incoming]).map(Arc::new)
            }
        };
        self.pending_flush = Some(response);
    }

    fn clear_suppressed(&mut self) {
        self.suppressed_charge = RequestCharge::default();
        self.suppressed_diagnostics = None;
        self.pending_flush = None;
    }

    /// Emits a final empty page carrying accumulated suppressed charge and
    /// diagnostics, or `None` if nothing is pending.
    fn flush_suppressed(&mut self) -> Option<PageResult> {
        let template = self.pending_flush.take()?;
        // `suppressed_diagnostics` already includes the template's own
        // diagnostics, so aggregate the list rather than layering onto it.
        let diagnostics = self
            .suppressed_diagnostics
            .clone()
            .unwrap_or_else(|| template.diagnostics());
        let mut headers = template.headers().clone();
        headers.item_count = Some(0);
        headers.request_charge = Some(self.suppressed_charge);
        let response = CosmosResponse::new(
            ResponseBody::from_items(Vec::new()),
            headers,
            template.status(),
            diagnostics,
        );
        self.clear_suppressed();
        Some(PageResult::Page {
            response,
            is_terminal: true,
        })
    }
}

#[async_trait]
impl PipelineNode for Distinct {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.exhausted {
            return Ok(PageResult::Drained);
        }

        loop {
            match self.child.next_page(context).await? {
                PageResult::Drained => {
                    self.exhausted = true;
                    // Flush charge/diagnostics from a fully-duplicate tail that
                    // never got a terminal page of its own.
                    if let Some(flushed) = self.flush_suppressed() {
                        return Ok(flushed);
                    }
                    return Ok(PageResult::Drained);
                }
                PageResult::SplitRequired { .. } => {
                    // `SplitRequired` replaces the node that emits it, so
                    // forwarding would drop this node along with its
                    // deduplication map and resurrect suppressed values. The
                    // wrapped fan-out node absorbs splits internally, so this
                    // is unreachable today; fail loudly if that ever changes.
                    return Err(crate::error::CosmosError::builder()
                        .with_status(CosmosStatus::CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT)
                        .with_message(
                            "DISTINCT cannot forward a partition split; the wrapped fan-out \
                             node must absorb splits internally",
                        )
                        .build());
                }
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    // Normalize the child's page into per-document slices. A
                    // streaming ordered merge (and a `SkipTake` below us) hands
                    // over pre-split `Items`; a raw backend feed page arrives as
                    // `Bytes`; `NoPayload` is a zero-document page.
                    let items: Vec<Bytes> = match response.body() {
                        ResponseBody::Items(items) => items.clone(),
                        ResponseBody::Bytes(bytes) => skip_take_page::split_feed_envelope(bytes)?,
                        ResponseBody::NoPayload => Vec::new(),
                    };
                    let survivors = self.select_survivors(&items)?;
                    let emitted = survivors.len();

                    // An all-duplicate intermediate page becomes a pull rather
                    // than an empty public page; its RU/diagnostics are retained.
                    if emitted == 0 && !is_terminal {
                        self.suppress(response);
                        continue;
                    }

                    let new_response = self.rebuild(&response, survivors, emitted);
                    return Ok(PageResult::Page {
                        response: new_response,
                        is_terminal,
                    });
                }
            }
        }
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![self.child]
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        // A drained pipeline has no deduplication state left to lose, so even
        // an unordered map can snapshot: resuming `Drained` re-emits nothing.
        // Refusing here would break the common "page to completion, then
        // persist the token" pattern on its very last iteration.
        if self.exhausted {
            return Ok(PipelineNodeState::Drained);
        }
        let last_hash = match &self.map {
            DistinctMap::Ordered { last_hash } => *last_hash,
            DistinctMap::Unordered { .. } => {
                return Err(crate::error::CosmosError::builder()
                    .with_status(CosmosStatus::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED)
                    .with_message(UNORDERED_CONTINUATION_MESSAGE)
                    .build());
            }
        };
        Ok(PipelineNodeState::Distinct {
            distinct_type: DistinctType::Ordered,
            last_hash,
            child: Box::new(self.child.snapshot_state()?),
        })
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.child.feed_range()
    }

    fn topology_can_change(&self) -> bool {
        // The wrapped fan-out node owns the partition ranges and handles its
        // own splits, so a `Distinct` is safe as the pipeline root.
        false
    }

    fn fan_out_width(&self) -> usize {
        // `Distinct` issues no request of its own.
        self.child.fan_out_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::*;
    use crate::driver::dataflow::node::SplitReplacements;
    use crate::models::ResponseBody;

    /// Builds a query-page body from a list of raw JSON document texts.
    fn page_body(documents: &[&str]) -> Vec<u8> {
        format!(
            r#"{{"_rid":"","Documents":[{}],"_count":{}}}"#,
            documents.join(","),
            documents.len()
        )
        .into_bytes()
    }

    fn page(documents: &[&str], is_terminal: bool) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: response(&page_body(documents)),
            is_terminal,
        })
    }

    fn charged_page(
        documents: &[&str],
        is_terminal: bool,
        ru: f64,
    ) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: response_with_charge(&page_body(documents), ru),
            is_terminal,
        })
    }

    fn documents_of(response: &CosmosResponse) -> Vec<serde_json::Value> {
        // `Distinct` emits pre-split `Items`, matching `StreamingOrderedMerge`
        // and `SkipTake`, so a parent node never has to re-split the page.
        match response.body() {
            ResponseBody::Items(items) => items
                .iter()
                .map(|item| serde_json::from_slice(item).unwrap())
                .collect(),
            ResponseBody::NoPayload => Vec::new(),
            other => panic!("expected an Items body, got {other:?}"),
        }
    }

    async fn drain(node: &mut Distinct) -> Vec<serde_json::Value> {
        drain_with_charge(node).await.0
    }

    /// Drains `node`, returning the emitted documents plus the total request
    /// charge reported across every emitted page.
    async fn drain_with_charge(node: &mut Distinct) -> (Vec<serde_json::Value>, f64) {
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let mut all = Vec::new();
        let mut charge = 0.0;
        loop {
            match node.next_page(&mut context).await.unwrap() {
                PageResult::Page { response, .. } => {
                    all.extend(documents_of(&response));
                    charge += response
                        .headers()
                        .request_charge
                        .map(|c| c.value())
                        .unwrap_or_default();
                }
                PageResult::Drained => break,
                PageResult::SplitRequired { .. } => panic!("unexpected split"),
            }
        }
        (all, charge)
    }

    fn strings(values: &[serde_json::Value]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    // ── Unordered map ────────────────────────────────────────────────────

    /// .NET `DistinctQueryPipelineStageTests.SanityTests`: duplicates that
    /// straddle page boundaries must collapse.
    #[tokio::test]
    async fn unordered_dedupes_across_pages() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#"{"item":42}"#, r#"{"item":1337}"#], false),
            page(&[r#"{"item":1337}"#, r#"{"item":42}"#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(
            strings(&drain(&mut node).await),
            vec![r#"{"item":42}"#, r#"{"item":1337}"#]
        );
    }

    /// Java `DistinctQueryTests.queryDocuments`: the same value arriving from a
    /// different partition is still one row. `MockLeaf` stands in for the
    /// fan-out root, which has already interleaved partitions by this point.
    #[tokio::test]
    async fn unordered_dedupes_across_partitions() {
        let child = MockLeaf::with_pages(vec![
            // Partition 0 contributed Seattle/Redmond; partition 1 repeats them.
            page(&[r#""Seattle""#, r#""Redmond""#], false),
            page(&[r#""Redmond""#, r#""Boston""#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(
            strings(&drain(&mut node).await),
            vec![r#""Seattle""#, r#""Redmond""#, r#""Boston""#]
        );
    }

    #[tokio::test]
    async fn unordered_dedupes_within_a_single_page() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#"1"#, r#"1"#, r#"2"#, r#"1"#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(strings(&drain(&mut node).await), vec!["1", "2"]);
    }

    #[tokio::test]
    async fn unordered_passes_through_when_nothing_repeats() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#"1"#, r#"2"#], false),
            page(&[r#"3"#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(strings(&drain(&mut node).await), vec!["1", "2", "3"]);
    }

    /// Structural equality, not textual: key order must not create a second row.
    #[tokio::test]
    async fn unordered_collapses_objects_differing_only_in_key_order() {
        let child = MockLeaf::with_pages(vec![
            page(
                &[
                    r#"{"name":"fido","species":"dog"}"#,
                    r#"{"species":"dog","name":"fido"}"#,
                    r#"{"name":"fido","species":"cat"}"#,
                ],
                true,
            ),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(drain(&mut node).await.len(), 2);
    }

    /// Arrays are position-sensitive, so these are two rows.
    #[tokio::test]
    async fn unordered_keeps_arrays_that_differ_only_in_order() {
        let child = MockLeaf::with_pages(vec![
            page(&["[1,2]", "[2,1]", "[1,2]"], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(strings(&drain(&mut node).await), vec!["[1,2]", "[2,1]"]);
    }

    /// Java `queryDocumentsForDistinctIntValues`: `5` and `5.0` are one value.
    #[tokio::test]
    async fn unordered_collapses_equal_numbers_written_differently() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#"{"intprop":5}"#, r#"{"intprop":5.0}"#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(drain(&mut node).await.len(), 1);
    }

    // ── Ordered map ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn ordered_dedupes_adjacent_runs() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#""Boston""#, r#""Redmond""#, r#""Redmond""#], false),
            page(&[r#""Seattle""#, r#""Seattle""#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Ordered);
        assert_eq!(
            strings(&drain(&mut node).await),
            vec![r#""Boston""#, r#""Redmond""#, r#""Seattle""#]
        );
    }

    /// Documents the adjacency assumption: an ordered map deliberately does
    /// *not* catch a non-adjacent repeat. This is only reachable when the merge
    /// really is sorted, which the planner guarantees by only choosing
    /// `Ordered` when the plan reports it.
    #[tokio::test]
    async fn ordered_does_not_dedupe_non_adjacent_repeats() {
        let child =
            MockLeaf::with_pages(vec![page(&["1", "2", "1"], true), Ok(PageResult::Drained)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Ordered);
        assert_eq!(strings(&drain(&mut node).await), vec!["1", "2", "1"]);
    }

    /// A run split across a page boundary must not re-emit its head.
    #[tokio::test]
    async fn ordered_dedupes_a_run_spanning_pages() {
        let child = MockLeaf::with_pages(vec![
            page(&[r#""Redmond""#], false),
            page(&[r#""Redmond""#, r#""Seattle""#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Ordered);
        assert_eq!(
            strings(&drain(&mut node).await),
            vec![r#""Redmond""#, r#""Seattle""#]
        );
    }

    /// Seeding from a checkpoint suppresses the re-delivered boundary row.
    #[tokio::test]
    async fn ordered_resume_suppresses_the_reemitted_boundary_row() {
        let boundary = hash_value(&serde_json::json!("Redmond")).unwrap();
        let child = MockLeaf::with_pages(vec![
            page(&[r#""Redmond""#, r#""Seattle""#], true),
            Ok(PageResult::Drained),
        ]);
        let mut node =
            Distinct::with_last_hash(Box::new(child), DistinctType::Ordered, Some(boundary));
        assert_eq!(strings(&drain(&mut node).await), vec![r#""Seattle""#]);
    }

    // ── Page shaping / accounting ────────────────────────────────────────

    #[tokio::test]
    async fn item_count_reflects_the_trimmed_page() {
        let child =
            MockLeaf::with_pages(vec![page(&["1", "1", "2"], true), Ok(PageResult::Drained)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        match node.next_page(&mut context).await.unwrap() {
            PageResult::Page { response, .. } => {
                assert_eq!(response.headers().item_count, Some(2));
                assert_eq!(strings(&documents_of(&response)), vec!["1", "2"]);
            }
            other => panic!("expected a page, got {other:?}"),
        }
    }

    /// An all-duplicate intermediate page must not surface as an empty page,
    /// and its RU must survive onto the next real page.
    #[tokio::test]
    async fn all_duplicate_intermediate_page_is_suppressed_but_charged() {
        let child = MockLeaf::with_pages(vec![
            charged_page(&["1"], false, 2.0),
            charged_page(&["1"], false, 3.0),
            charged_page(&["2"], true, 5.0),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let first = node.next_page(&mut context).await.unwrap();
        let PageResult::Page { response, .. } = first else {
            panic!("expected a page");
        };
        assert_eq!(strings(&documents_of(&response)), vec!["1"]);

        // The all-duplicate second page is skipped; the third page carries both
        // its own 5 RU and the suppressed 3 RU.
        let second = node.next_page(&mut context).await.unwrap();
        let PageResult::Page { response, .. } = second else {
            panic!("expected a page");
        };
        assert_eq!(strings(&documents_of(&response)), vec!["2"]);
        assert_eq!(response.headers().request_charge.unwrap().value(), 8.0);
    }

    /// A fully-duplicate *tail* still has to report its RU, so the node flushes
    /// a final empty page rather than dropping the charge.
    #[tokio::test]
    async fn all_duplicate_tail_flushes_its_charge() {
        let child = MockLeaf::with_pages(vec![
            charged_page(&["1"], false, 2.0),
            charged_page(&["1"], false, 4.0),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let _ = node.next_page(&mut context).await.unwrap();
        match node.next_page(&mut context).await.unwrap() {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                assert!(is_terminal);
                assert!(documents_of(&response).is_empty());
                assert_eq!(response.headers().request_charge.unwrap().value(), 4.0);
            }
            other => panic!("expected a flushed empty page, got {other:?}"),
        }
        assert!(matches!(
            node.next_page(&mut context).await.unwrap(),
            PageResult::Drained
        ));
    }

    /// A terminal all-duplicate page is emitted as an empty terminal page (not
    /// suppressed), so the caller sees the stream end.
    #[tokio::test]
    async fn terminal_all_duplicate_page_is_emitted_empty() {
        let child = MockLeaf::with_pages(vec![
            page(&["1"], false),
            page(&["1"], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let _ = node.next_page(&mut context).await.unwrap();
        match node.next_page(&mut context).await.unwrap() {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                assert!(is_terminal);
                assert!(documents_of(&response).is_empty());
                assert_eq!(response.headers().item_count, Some(0));
            }
            other => panic!("expected an empty terminal page, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn empty_backend_page_is_handled() {
        let child = MockLeaf::with_pages(vec![page(&[], true), Ok(PageResult::Drained)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert!(drain(&mut node).await.is_empty());
    }

    // ── Continuation ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn ordered_snapshot_carries_the_last_emitted_hash() {
        let child = MockLeaf::with_pages(vec![page(&[r#""Redmond""#], false)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Ordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let _ = node.next_page(&mut context).await.unwrap();

        match node.snapshot_state().unwrap() {
            PipelineNodeState::Distinct {
                distinct_type,
                last_hash,
                ..
            } => {
                assert_eq!(distinct_type, DistinctType::Ordered);
                assert_eq!(
                    last_hash,
                    Some(hash_value(&serde_json::json!("Redmond")).unwrap())
                );
            }
            other => panic!("expected a Distinct snapshot, got {other:?}"),
        }
    }

    #[test]
    fn ordered_snapshot_before_any_row_has_no_hash() {
        let child = MockLeaf::with_pages(vec![]);
        let node = Distinct::new(Box::new(child), DistinctType::Ordered);
        match node.snapshot_state().unwrap() {
            PipelineNodeState::Distinct { last_hash, .. } => assert_eq!(last_hash, None),
            other => panic!("expected a Distinct snapshot, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn ordered_snapshot_after_draining_is_drained() {
        let child = MockLeaf::with_pages(vec![page(&["1"], true), Ok(PageResult::Drained)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Ordered);
        let _ = drain(&mut node).await;
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
    }

    /// The load-bearing continuation contract: an unordered `DISTINCT` must
    /// fail loudly at token-mint time rather than hand back a token whose
    /// resume would re-emit rows.
    #[test]
    fn unordered_snapshot_is_refused_with_actionable_guidance() {
        let child = MockLeaf::with_pages(vec![]);
        let node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let err = node
            .snapshot_state()
            .expect_err("an unordered DISTINCT must not produce a resumable snapshot");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED),
        );
        assert!(
            err.to_string().contains("ORDER BY"),
            "the message must tell the caller how to make the query resumable: {err}"
        );
    }

    /// A drained pipeline has no deduplication state left to lose, so even an
    /// unordered map can snapshot — resuming `Drained` re-emits nothing. This
    /// is what makes the common "page to completion, then persist the token"
    /// pattern work for an unordered `DISTINCT` query.
    #[tokio::test]
    async fn unordered_snapshot_is_allowed_once_drained() {
        let child = MockLeaf::with_pages(vec![page(&["1"], true), Ok(PageResult::Drained)]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let _ = drain(&mut node).await;
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
    }

    // ── Errors ───────────────────────────────────────────────────────────

    #[tokio::test]
    async fn malformed_page_body_surfaces_a_typed_error() {
        let child = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"not json"),
            is_terminal: true,
        })]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(node.next_page(&mut context).await.is_err());
    }

    #[tokio::test]
    async fn forwarded_split_surfaces_a_typed_error() {
        // `SplitRequired` replaces the node that emits it, so forwarding one
        // would discard this node's deduplication map and let suppressed values
        // reappear. Unreachable in production (the wrapped fan-out node absorbs
        // splits), so this pins the guard rather than a live path.
        let replacement = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacements: SplitReplacements::untiled(vec![Box::new(replacement)]),
        })]);
        let mut node = Distinct::new(Box::new(child), DistinctType::Unordered);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = node.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            err.status(),
            CosmosStatus::CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT
        );
    }

    #[tokio::test]
    async fn fan_out_width_matches_the_child() {
        let child = MockLeaf::with_pages(vec![]);
        let expected = child.fan_out_width();
        let node = Distinct::new(Box::new(child), DistinctType::Unordered);
        assert_eq!(node.fan_out_width(), expected);
        assert!(!node.topology_can_change());
    }

    // ── Catalog-driven scenarios ─────────────────────────────────────────
    //
    // Reuses `tests/fixtures/distinct_scenarios.json`; this file's copy of the
    // fixture schema is minimal since separate compilation units can't share a
    // `pub(crate)` type — see `tests/distinct_scenario_catalog.rs` for the
    // canonical strict schema every layer trusts.

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
        #[serde(rename = "expectedValues", default)]
        expected_values: Vec<serde_json::Value>,
        checkpoint: Option<serde_json::Value>,
        #[serde(rename = "expectedError")]
        expected_error: Option<serde_json::Value>,
    }

    #[derive(serde::Deserialize)]
    struct QueryFixture {
        #[serde(rename = "distinctType")]
        distinct_type: String,
    }

    #[derive(serde::Deserialize)]
    struct MockFixture {
        partitions: Vec<PartitionFixture>,
    }

    #[derive(serde::Deserialize)]
    struct PartitionFixture {
        pages: Vec<PageFixture>,
    }

    #[derive(serde::Deserialize)]
    struct PageFixture {
        rows: Vec<RowFixture>,
    }

    #[derive(serde::Deserialize)]
    struct RowFixture {
        payload: serde_json::Value,
    }

    fn fixture_distinct_type(text: &str) -> DistinctType {
        match text {
            "Ordered" => DistinctType::Ordered,
            "Unordered" => DistinctType::Unordered,
            "None" => DistinctType::None,
            other => panic!("unknown distinctType in fixture: {other}"),
        }
    }

    /// Runs every catalog scenario tagged `mockPipeline` or `distinctMap`
    /// through the real node.
    ///
    /// `SequentialDrain` and `StreamingOrderedMerge` both hand their parent a
    /// flat sequence of pages, so the fixture's per-partition pages are
    /// concatenated in partition order — exactly what `Distinct` observes in
    /// production. Error scenarios need the planner (see `planner`'s
    /// `peel_distinct_resume` tests and `integration_tests::distinct_resume`)
    /// and are covered by dedicated tests instead.
    #[tokio::test]
    async fn catalog_scenarios_dedupe_as_expected() {
        const CATALOG_JSON: &str = include_str!("../../../tests/fixtures/distinct_scenarios.json");
        let catalog: CatalogFixture =
            serde_json::from_str(CATALOG_JSON).expect("catalog must parse");

        let mut ran = 0usize;
        let mut ran_a_resume_checkpoint = false;
        for scenario in &catalog.scenarios {
            if !scenario
                .layers
                .iter()
                .any(|l| l == "mockPipeline" || l == "distinctMap")
            {
                continue;
            }
            if scenario.expected_error.is_some() {
                continue;
            }
            let Some(mock) = &scenario.mock else {
                continue;
            };

            let mut pages: Vec<crate::error::Result<PageResult>> = Vec::new();
            let page_specs: Vec<&PageFixture> = mock
                .partitions
                .iter()
                .flat_map(|partition| partition.pages.iter())
                .collect();
            // Every page carries a distinct, non-round charge so the
            // suppress/flush accounting cannot accidentally balance.
            let mut expected_charge = 0.0;
            for (index, page_spec) in page_specs.iter().enumerate() {
                let documents: Vec<String> = page_spec
                    .rows
                    .iter()
                    .map(|row| row.payload.to_string())
                    .collect();
                let refs: Vec<&str> = documents.iter().map(String::as_str).collect();
                let ru = 1.25 + index as f64;
                expected_charge += ru;
                pages.push(charged_page(&refs, index + 1 == page_specs.len(), ru));
            }
            pages.push(Ok(PageResult::Drained));

            // A checkpoint seeds the ordered map with the hash of the last
            // value emitted before the resume.
            let last_hash = match scenario
                .checkpoint
                .as_ref()
                .and_then(|c| c.get("lastValue"))
            {
                Some(value) => {
                    ran_a_resume_checkpoint = true;
                    Some(hash_value(value).expect("checkpoint value hashes"))
                }
                None => None,
            };

            let mut node = Distinct::with_last_hash(
                Box::new(MockLeaf::with_pages(pages)),
                fixture_distinct_type(&scenario.query.distinct_type),
                last_hash,
            );
            let (values, charge) = drain_with_charge(&mut node).await;
            assert_eq!(
                values, scenario.expected_values,
                "scenario {} produced the wrong deduplicated stream",
                scenario.id
            );
            // Suppressing an all-duplicate page must never lose its RU: the
            // charge the caller sees has to equal what the backend billed.
            assert!(
                (charge - expected_charge).abs() < 1e-9,
                "scenario {}: emitted pages reported {charge} RU but the backend billed \
                 {expected_charge}",
                scenario.id
            );
            ran += 1;
        }

        // Exact, not a floor, so a scenario that stops matching the loop's
        // filters shows up as a failure rather than a quietly smaller run.
        // (A `mockPipeline` scenario with no `mock` is excluded from both
        // sides here; `mock_pipeline_scenarios_carry_a_mock_or_expect_an_error`
        // in the catalog test is what catches that.)
        let expected: usize = catalog
            .scenarios
            .iter()
            .filter(|s| {
                s.layers
                    .iter()
                    .any(|l| l == "mockPipeline" || l == "distinctMap")
                    && s.expected_error.is_none()
                    && s.mock.is_some()
            })
            .count();
        assert_eq!(
            ran, expected,
            "every eligible mock-pipeline scenario must run"
        );
        assert!(
            ran >= 10,
            "expected the catalog to drive a meaningful number of mock-pipeline scenarios, ran {ran}"
        );
        assert!(
            ran_a_resume_checkpoint,
            "no catalog scenario exercised an ordered resume checkpoint"
        );
    }
}
