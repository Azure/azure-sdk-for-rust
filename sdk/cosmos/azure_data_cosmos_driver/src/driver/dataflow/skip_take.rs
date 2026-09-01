// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Skip/take node implementing cross-partition `OFFSET` / `LIMIT` / `TOP`.
//!
//! [`SkipTake`] wraps a cross-partition child (a
//! [`SequentialDrain`](super::SequentialDrain) for unordered fan-out, or a
//! streaming ordered merge for `ORDER BY`) and applies a **global** skip then
//! take across the documents streaming out of that child, in whatever order the
//! child yields them:
//!
//! ```text
//!   OFFSET (skip)  ->  LIMIT / TOP (take)
//! ```
//!
//! The backend has already bounded each partition's contribution via the
//! query plan's `rewrittenQuery` (e.g. `OFFSET 0 LIMIT offset+limit`), so this
//! node only has to reconcile the child's documents into the final global
//! window. It does that by splitting each page's `Documents` into per-document
//! slices (see [`super::skip_take_page`]), dropping/truncating that list, and
//! emitting the survivors as a [`ResponseBody::Items`] body — no envelope is
//! re-serialized. It stops early once `take` is satisfied, so a `TOP n` query
//! never drains partitions it doesn't need.

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;

use crate::diagnostics::DiagnosticsContext;
use crate::models::{CosmosResponse, FeedRange, RequestCharge, ResponseBody};

use super::{skip_take_page, PageResult, PipelineContext, PipelineNode, PipelineNodeState};

/// Applies a global `OFFSET` (`remaining_skip`) then `LIMIT`/`TOP`
/// (`remaining_take`, `None` = unbounded) over its single child's pages.
pub(crate) struct SkipTake {
    child: Box<dyn PipelineNode>,
    remaining_skip: u64,
    remaining_take: Option<u64>,
    /// Set once `take` is satisfied (or the child drains) so subsequent pulls
    /// short-circuit to `Drained` without touching the child again.
    exhausted: bool,
    /// Request charges (RU) from backend pages that were fully consumed by the
    /// outstanding `OFFSET` and therefore suppressed (see the `continue` in
    /// [`next_page`](Self::next_page)). Folded into the next emitted page so the
    /// public page does not under-report the RUs actually billed.
    suppressed_charge: RequestCharge,
    /// Diagnostics from those same suppressed pages, in arrival order. Merged
    /// (as prior attempts) into the next emitted page so `request_count()` and
    /// per-request diagnostics account for every backend request the skip
    /// touched.
    suppressed_diagnostics: Vec<Arc<DiagnosticsContext>>,
    /// The most recently suppressed page, retained as a template so that if the
    /// child drains while `suppressed_diagnostics` is still non-empty (i.e. the
    /// skip consumed the very last backend page without a terminal marker) the
    /// accumulated charge/diagnostics can still be flushed as a final empty
    /// page rather than being dropped.
    pending_flush: Option<CosmosResponse>,
    /// Whether a re-split page is emitted as Cosmos binary JSON. Derived from
    /// the negotiated operation, never from the bytes of a received page, so
    /// this node agrees with [`PageAggregator`] on the same query.
    ///
    /// [`PageAggregator`]: super::query_response::PageAggregator
    emit_binary: bool,
    /// Set when a page was consumed from the child but could not be processed,
    /// leaving the window counters and the child's continuation permanently
    /// disagreeing. The node then refuses to produce further pages or a resume
    /// token — see [`process_page`](Self::process_page) for why no error on
    /// that path is recoverable.
    poisoned: bool,
}

impl SkipTake {
    /// Wraps `child`, skipping `skip` documents then taking up to `take`
    /// (`None` = all remaining). `emit_binary` is the encoding the operation
    /// hands back to the caller (see
    /// [`CosmosOperation::emits_binary_payload`]) — not
    /// [`negotiates_binary_response`], which is the encoding asked of the
    /// service. The two differ under `request_text_response`, where the wire
    /// stays binary but this node must emit text.
    ///
    /// [`CosmosOperation::emits_binary_payload`]: crate::models::CosmosOperation::emits_binary_payload
    /// [`negotiates_binary_response`]: crate::models::CosmosOperation::negotiates_binary_response
    pub(crate) fn new(
        child: Box<dyn PipelineNode>,
        skip: u64,
        take: Option<u64>,
        emit_binary: bool,
    ) -> Self {
        Self {
            child,
            remaining_skip: skip,
            remaining_take: take,
            exhausted: false,
            suppressed_charge: RequestCharge::default(),
            suppressed_diagnostics: Vec::new(),
            pending_flush: None,
            emit_binary,
            poisoned: false,
        }
    }

    /// Rebuilds a response around the surviving per-document `items`, preserving
    /// status and headers (with `x-ms-item-count` updated to `emitted`), and
    /// folding in any request charge and diagnostics accumulated from suppressed
    /// (fully-skipped) pages. The body is emitted as [`ResponseBody::Items`] so
    /// the calling SDK reads each document directly without re-parsing an
    /// envelope.
    fn rebuild(
        &mut self,
        response: &CosmosResponse,
        items: Vec<Bytes>,
        emitted: u64,
    ) -> CosmosResponse {
        let mut headers = response.headers().clone();
        headers.item_count = Some(emitted as u32);
        // Fold RUs charged by fully-skipped pages into this public page so the
        // billed total is not under-reported.
        if self.suppressed_charge != RequestCharge::default() {
            let base = headers.request_charge.unwrap_or_default();
            headers.request_charge = Some(base + self.suppressed_charge);
        }
        let rebuilt = CosmosResponse::new(
            ResponseBody::from_items(items),
            headers,
            response.status(),
            response.diagnostics(),
        );
        // Prepend the suppressed pages' diagnostics (they happened before this
        // page) so request counts and per-request diagnostics are complete.
        let merged = rebuilt.with_aggregated_prior_diagnostics(&self.suppressed_diagnostics);
        self.clear_suppressed();
        merged
    }

    /// Accumulates a fully-skipped page's request charge and diagnostics, and
    /// retains it as the flush template (see [`pending_flush`](Self::pending_flush)).
    fn suppress(&mut self, response: CosmosResponse) {
        self.suppressed_charge =
            self.suppressed_charge + response.headers().request_charge.unwrap_or_default();
        self.suppressed_diagnostics.push(response.diagnostics());
        self.pending_flush = Some(response);
    }

    /// Clears the suppressed-page accumulators after they have been surfaced.
    fn clear_suppressed(&mut self) {
        self.suppressed_charge = RequestCharge::default();
        self.suppressed_diagnostics.clear();
        self.pending_flush = None;
    }

    /// Emits a final empty page carrying the accumulated suppressed charge and
    /// diagnostics, or `None` if nothing is pending. Used when the child drains
    /// without ever surfacing a terminal page for the fully-skipped tail.
    fn flush_suppressed(&mut self) -> Option<PageResult> {
        let template = self.pending_flush.take()?;
        // `suppressed_diagnostics` already contains every suppressed page's
        // diagnostics (including this template's), so aggregate them directly
        // rather than re-using the template's own diagnostics as a base.
        let diagnostics =
            DiagnosticsContext::aggregate_sub_operations(&self.suppressed_diagnostics)
                .map(Arc::new)
                .unwrap_or_else(|| template.diagnostics());
        let mut headers = template.headers().clone();
        headers.item_count = Some(0);
        headers.request_charge = Some(self.suppressed_charge);
        // An empty ordered/unordered page is an empty item list, not an empty
        // re-serialized envelope.
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

    /// Applies the window to one page from the child, returning the page to
    /// emit or `None` when the page was fully skipped and retained for its
    /// RU/diagnostics.
    ///
    /// **Every error out of this method is unrecoverable in place**, which is
    /// why the caller poisons on all of them rather than on individual steps.
    /// The child's page is already consumed, so its continuation has moved,
    /// while `remaining_skip` / `remaining_take` still hold their pre-page
    /// values. Resuming from that pair would skip the consumed documents *and*
    /// re-apply the full window to the ones after them.
    ///
    /// The sibling ordered merge takes the opposite stance because it owns
    /// per-row boundaries and can hold rows back; a `SkipTake` is handed a
    /// whole page at once and cannot return part of it.
    fn process_page(
        &mut self,
        response: CosmosResponse,
        is_terminal: bool,
    ) -> crate::error::Result<Option<PageResult>> {
        // Split the child's page into per-document slices. An ordered merge
        // hands us pre-split `Items`; a raw backend feed page arrives as
        // `Bytes` and is split as text, then re-encoded below.
        let (items, needs_encode) = match response.body() {
            ResponseBody::Items(items) => (items.clone(), false),
            ResponseBody::Bytes(b) => (skip_take_page::split_feed_envelope(b)?, true),
            ResponseBody::NoPayload => (Vec::new(), false),
        };

        let outcome =
            skip_take_page::skip_take_items(items, self.remaining_skip, self.remaining_take);

        // Encode only the survivors: a document the window discards must not be
        // able to fail the query.
        let emitted_items = if needs_encode {
            skip_take_page::encode_items(outcome.items, self.emit_binary)?
        } else {
            outcome.items
        };

        self.remaining_skip -= outcome.dropped;
        if let Some(take) = self.remaining_take.as_mut() {
            *take -= outcome.emitted;
        }

        let take_exhausted = matches!(self.remaining_take, Some(0));
        let terminal = is_terminal || take_exhausted;

        // A page fully consumed by the outstanding skip is retained for its
        // RU/diagnostics rather than surfaced as an empty intermediate page.
        if outcome.emitted == 0 && !terminal {
            self.suppress(response);
            return Ok(None);
        }

        if take_exhausted {
            self.exhausted = true;
        }

        let new_response = self.rebuild(&response, emitted_items, outcome.emitted);
        Ok(Some(PageResult::Page {
            response: new_response,
            is_terminal: terminal,
        }))
    }

    /// The error a poisoned node returns from every subsequent call.
    ///
    /// Deliberately the same status as the failure that caused it: the poison is
    /// not an independent fault, it is that fault made durable. The cause is
    /// deterministic — replaying the same child page fails the same way — so
    /// the message points at the encoding rather than suggesting a retry.
    fn poisoned_error() -> crate::error::CosmosError {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message(
                "skip/take node is unusable: a page was consumed from its child but could not be \
                 processed, so the offset/limit window and the child's resume position no longer \
                 agree and no continuation token can be minted; the page holds a document the \
                 negotiated encoding cannot represent, so retrying is futile — re-run the query \
                 with a text response instead",
            )
            .build()
    }
}

#[async_trait]
impl PipelineNode for SkipTake {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.poisoned {
            return Err(Self::poisoned_error());
        }
        if self.exhausted {
            return Ok(PageResult::Drained);
        }

        loop {
            match self.child.next_page(context).await? {
                PageResult::Drained => {
                    self.exhausted = true;
                    // If the skip consumed the final backend page(s) without a
                    // terminal marker, surface their RU/diagnostics as a final
                    // empty page rather than dropping them.
                    if let Some(flushed) = self.flush_suppressed() {
                        return Ok(flushed);
                    }
                    return Ok(PageResult::Drained);
                }
                PageResult::SplitRequired { .. } => {
                    // A split must never reach a Skip/Take node: it always reads
                    // from a child that absorbs splits internally (the ordered
                    // merge or the sequential fan-out drain). A propagated split
                    // means the pipeline was mis-assembled, so fail loudly rather
                    // than silently mishandling it.
                    return Err(crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT,
                        )
                        .with_message(
                            "SkipTake received a SplitRequired from its child; splits must be \
                             absorbed below the skip/take node",
                        )
                        .build());
                }
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    // Every fallible step in `process_page` runs *after* the
                    // child handed over the page, so poison on any error rather
                    // than at individual call sites.
                    match self.process_page(response, is_terminal) {
                        Ok(Some(page)) => return Ok(page),
                        // Fully consumed by the outstanding skip; pull the next.
                        Ok(None) => continue,
                        Err(err) => {
                            self.poisoned = true;
                            return Err(err);
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![self.child]
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        // A poisoned node's window counters no longer describe the child's
        // position, so any token minted here would resume incorrectly.
        if self.poisoned {
            return Err(Self::poisoned_error());
        }
        // Once the window is fully satisfied there is nothing left to resume.
        if self.exhausted {
            return Ok(PipelineNodeState::Drained);
        }
        Ok(PipelineNodeState::SkipTake {
            remaining_skip: self.remaining_skip,
            remaining_take: self.remaining_take,
            child: Box::new(self.child.snapshot_state()?),
        })
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.child.feed_range()
    }

    fn topology_can_change(&self) -> bool {
        // The wrapped fan-out node owns the partition ranges and handles its own
        // splits, so a `SkipTake` is safe as the pipeline root.
        false
    }

    fn fan_out_width(&self) -> usize {
        // A `SkipTake` wraps a single fan-out child and issues no request of its
        // own, so its leaf fan-out is exactly the child's.
        self.child.fan_out_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::*;
    use crate::models::ResponseBody;

    /// Builds a query-page response body from a list of integer ids.
    fn page_body(ids: &[u64]) -> Vec<u8> {
        let docs: Vec<String> = ids.iter().map(|i| format!(r#"{{"id":{i}}}"#)).collect();
        format!(
            r#"{{"Documents":[{}],"_count":{}}}"#,
            docs.join(","),
            ids.len()
        )
        .into_bytes()
    }

    fn page_result(ids: &[u64], is_terminal: bool) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: response(&page_body(ids)),
            is_terminal,
        })
    }

    fn ids_of(response: &CosmosResponse) -> Vec<u64> {
        // SkipTake now emits a pre-split `Items` body; each item is one
        // document's exact bytes.
        let items = match response.body() {
            ResponseBody::Items(items) => items.clone(),
            ResponseBody::NoPayload => Vec::new(),
            ResponseBody::Bytes(_) => panic!("expected Items body"),
        };
        items
            .iter()
            .map(|b| {
                let d: serde_json::Value = serde_json::from_slice(b).unwrap();
                d["id"].as_u64().unwrap()
            })
            .collect()
    }

    async fn drain_ids(node: &mut SkipTake) -> Vec<u64> {
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let mut all = Vec::new();
        loop {
            match node.next_page(&mut context).await.unwrap() {
                PageResult::Page { response, .. } => all.extend(ids_of(&response)),
                PageResult::Drained => break,
                PageResult::SplitRequired { .. } => panic!("unexpected split"),
            }
        }
        all
    }

    #[tokio::test]
    async fn top_stops_early_without_draining() {
        // TOP 2 over a child with two pages: the second page must never be pulled.
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2, 3], false),
            // If SkipTake pulled again this would be returned, but it must not.
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 0, Some(2), false);
        assert_eq!(drain_ids(&mut node).await, vec![1, 2]);
    }

    #[tokio::test]
    async fn offset_spans_pages() {
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3, 4], false),
            page_result(&[5, 6], true),
            Ok(PageResult::Drained),
        ]);
        // Skip 3, take rest → 4,5,6.
        let mut node = SkipTake::new(Box::new(child), 3, None, false);
        assert_eq!(drain_ids(&mut node).await, vec![4, 5, 6]);
    }

    #[tokio::test]
    async fn offset_and_limit_across_pages() {
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3, 4], false),
            page_result(&[5, 6], true),
            Ok(PageResult::Drained),
        ]);
        // OFFSET 1 LIMIT 3 → 2,3,4.
        let mut node = SkipTake::new(Box::new(child), 1, Some(3), false);
        assert_eq!(drain_ids(&mut node).await, vec![2, 3, 4]);
    }

    #[tokio::test]
    async fn skip_larger_than_total_yields_nothing() {
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 10, Some(5), false);
        assert_eq!(drain_ids(&mut node).await, Vec::<u64>::new());
    }

    #[tokio::test]
    async fn snapshot_reports_progress_then_drained() {
        let child = MockLeaf::with_pages(vec![page_result(&[1, 2, 3, 4], false)]);
        let mut node = SkipTake::new(Box::new(child), 1, Some(2), false);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        // First (and only needed) page satisfies the full window → exhausted.
        let page = node.next_page(&mut context).await.unwrap();
        match page {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                assert_eq!(ids_of(&response), vec![2, 3]);
                assert!(is_terminal);
            }
            other => panic!("expected page, got {other:?}"),
        }
        assert!(matches!(
            node.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn snapshot_mid_window_preserves_remaining() {
        // Take not yet satisfied after the first page → snapshot must carry the
        // remaining skip/take so a resume continues correctly.
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3, 4], false),
        ]);
        let mut node = SkipTake::new(Box::new(child), 1, Some(5), false);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let _ = node.next_page(&mut context).await.unwrap(); // emits id 2
        match node.snapshot_state().unwrap() {
            PipelineNodeState::SkipTake {
                remaining_skip,
                remaining_take,
                ..
            } => {
                assert_eq!(remaining_skip, 0);
                assert_eq!(remaining_take, Some(4));
            }
            other => panic!("expected SkipTake snapshot, got {other:?}"),
        }
    }

    /// A page whose documents parse as JSON (so the envelope splits) but cannot
    /// be re-encoded to Cosmos binary JSON, forcing `encode_items` to fail.
    fn page_failing_binary_encode(is_terminal: bool) -> crate::error::Result<PageResult> {
        // `1e999` is out of `f64` range, so `transcode_to_binary`'s *parse*
        // rejects it. `RawValue` defers number parsing, so the envelope split
        // still succeeds and the failure lands on the re-encode.
        let body = br#"{"Documents":[{"id":1,"n":1e999}],"_count":1}"#.to_vec();
        Ok(PageResult::Page {
            response: response(&body),
            is_terminal,
        })
    }

    #[tokio::test]
    async fn encode_failure_poisons_the_node_instead_of_resuming_wrong() {
        // The child page is consumed before `encode_items` runs, so its
        // continuation has advanced while `remaining_skip` has not. Resuming
        // from that pair would skip the consumed documents *and* re-apply the
        // window to the ones after them.
        let child = MockLeaf::with_pages(vec![
            page_failing_binary_encode(false),
            page_result(&[2], true),
        ]);
        let mut node = SkipTake::new(Box::new(child), 0, None, true);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let error = node
            .next_page(&mut context)
            .await
            .expect_err("re-encoding an infinite number must fail");
        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
        );

        // No further page, even though the child has one left to give.
        let second = node
            .next_page(&mut context)
            .await
            .expect_err("a poisoned node must not emit another page");
        assert!(
            second.to_string().contains("skip/take node is unusable"),
            "the second failure must be the poison, not an incidental error: {second}",
        );

        // And no resume token, which is the assertion that matters.
        let snapshot = node
            .snapshot_state()
            .expect_err("a poisoned node must not mint a resume token");
        assert!(
            snapshot.to_string().contains("skip/take node is unusable"),
            "snapshot must fail with the poison: {snapshot}",
        );
    }

    /// A page whose envelope cannot be split at all, failing before the window
    /// is applied. Reaches `split_feed_envelope` rather than `encode_items`.
    fn page_failing_envelope_split(is_terminal: bool) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: response(b"{\"Documents\":[ truncated"),
            is_terminal,
        })
    }

    #[tokio::test]
    async fn envelope_split_failure_poisons_the_node_too() {
        // The split runs in the same unrecoverable window as the encode, so
        // poisoning must not be specific to the encode step.
        let child = MockLeaf::with_pages(vec![
            page_failing_envelope_split(false),
            page_result(&[2], true),
        ]);
        let mut node = SkipTake::new(Box::new(child), 5, None, false);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        node.next_page(&mut context)
            .await
            .expect_err("a malformed envelope must fail");

        let second = node
            .next_page(&mut context)
            .await
            .expect_err("a poisoned node must not emit another page");
        assert!(
            second.to_string().contains("skip/take node is unusable"),
            "the second failure must be the poison: {second}",
        );

        let snapshot = node
            .snapshot_state()
            .expect_err("a poisoned node must not mint a resume token");
        assert!(
            snapshot.to_string().contains("skip/take node is unusable"),
            "snapshot must fail with the poison: {snapshot}",
        );
    }

    /// Builds a query-page response body/charge pair for the RU-accounting tests.
    fn charged_page(ids: &[u64], is_terminal: bool, ru: f64) -> crate::error::Result<PageResult> {
        Ok(PageResult::Page {
            response: response_with_charge(&page_body(ids), ru),
            is_terminal,
        })
    }

    fn request_charge_of(response: &CosmosResponse) -> f64 {
        response
            .headers()
            .request_charge
            .expect("response should carry a request charge")
            .value()
    }

    #[tokio::test]
    async fn suppressed_pages_request_charge_is_folded_into_next_page() {
        // OFFSET 3 fully consumes the first two (non-terminal) pages, which are
        // suppressed; their RU must not be dropped but folded into the first
        // emitted page. Charges: 3.0 + 5.0 (skipped) + 7.0 (emitted) = 15.0.
        let child = MockLeaf::with_pages(vec![
            charged_page(&[1, 2], false, 3.0),
            charged_page(&[3], false, 5.0),
            charged_page(&[4, 5], true, 7.0),
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 3, None, false);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = node.next_page(&mut context).await.unwrap();
        match page {
            PageResult::Page { response, .. } => {
                assert_eq!(ids_of(&response), vec![4, 5]);
                assert_eq!(request_charge_of(&response), 15.0);
            }
            other => panic!("expected page, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn suppressed_terminal_empty_page_reports_accumulated_charge() {
        // OFFSET past the end: every page is fully skipped. The terminal empty
        // page must still surface the summed RU (2.0 + 4.0 = 6.0).
        let child = MockLeaf::with_pages(vec![
            charged_page(&[1, 2], false, 2.0),
            charged_page(&[3], true, 4.0),
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 10, None, false);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = node.next_page(&mut context).await.unwrap();
        match page {
            PageResult::Page { response, .. } => {
                assert_eq!(ids_of(&response), Vec::<u64>::new());
                assert_eq!(request_charge_of(&response), 6.0);
            }
            other => panic!("expected page, got {other:?}"),
        }
    }
}
