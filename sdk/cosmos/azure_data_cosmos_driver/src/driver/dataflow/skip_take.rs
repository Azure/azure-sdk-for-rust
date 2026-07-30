// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Skip/take node implementing cross-partition `OFFSET` / `LIMIT` / `TOP`.
//!
//! [`SkipTake`] wraps the cross-partition fan-out root (a
//! [`SequentialDrain`](super::SequentialDrain)) and applies a **global** skip
//! then take across the documents streaming out of every partition, in the
//! pipeline's natural EPK order:
//!
//! ```text
//!   OFFSET (skip)  ->  LIMIT / TOP (take)
//! ```
//!
//! The backend has already bounded each partition's contribution via the
//! query plan's `rewrittenQuery` (e.g. `OFFSET 0 LIMIT offset+limit`), so this
//! node only has to reconcile the streams into the final global window. It does
//! that by trimming each page's `Documents` array (see
//! [`super::query_response`]) and stopping early once `take` is satisfied — so a
//! `TOP n` query never drains partitions it doesn't need.

use std::sync::Arc;

use async_trait::async_trait;

use crate::diagnostics::DiagnosticsContext;
use crate::models::{CosmosResponse, FeedRange, RequestCharge, ResponseBody};

use super::{
    query_response, PageResult, PipelineContext, PipelineNode, PipelineNodeState, SkipTakeStage,
};

/// Applies a global `OFFSET` (`remaining_skip`) then `LIMIT`/`TOP`
/// (`remaining_take`, `None` = unbounded) over its single child's pages.
pub(crate) struct SkipTake {
    child: Box<dyn PipelineNode>,
    /// Which SQL construct produced this window (`TOP` vs `OFFSET`/`LIMIT`).
    /// Carried into the continuation snapshot so a resume can reject a token
    /// whose shape does not match the query plan.
    stage: SkipTakeStage,
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
    /// page rather than being dropped. `Vec<u8>` is the trimmed (empty) body.
    pending_flush: Option<(CosmosResponse, Vec<u8>)>,
}

impl SkipTake {
    /// Wraps `child`, skipping `skip` documents then taking up to `take`
    /// (`None` = all remaining). `stage` records whether the window came from
    /// `TOP` or `OFFSET`/`LIMIT` for continuation-token validation.
    pub(crate) fn new(
        child: Box<dyn PipelineNode>,
        skip: u64,
        take: Option<u64>,
        stage: SkipTakeStage,
    ) -> Self {
        Self {
            child,
            stage,
            remaining_skip: skip,
            remaining_take: take,
            exhausted: false,
            suppressed_charge: RequestCharge::default(),
            suppressed_diagnostics: Vec::new(),
            pending_flush: None,
        }
    }

    /// Rebuilds a response around a trimmed body, preserving status and headers
    /// (with `x-ms-item-count` updated to `emitted`), and folding in any request
    /// charge and diagnostics accumulated from suppressed (fully-skipped) pages.
    fn rebuild(
        &mut self,
        response: &CosmosResponse,
        body: Vec<u8>,
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
        let rebuilt = CosmosResponse::new(body, headers, response.status(), response.diagnostics());
        // Prepend the suppressed pages' diagnostics (they happened before this
        // page) so request counts and per-request diagnostics are complete.
        let merged = rebuilt.with_aggregated_prior_diagnostics(&self.suppressed_diagnostics);
        self.clear_suppressed();
        merged
    }

    /// Accumulates a fully-skipped page's request charge and diagnostics, and
    /// retains it as the flush template (see [`pending_flush`](Self::pending_flush)).
    fn suppress(&mut self, response: CosmosResponse, empty_body: Vec<u8>) {
        self.suppressed_charge =
            self.suppressed_charge + response.headers().request_charge.unwrap_or_default();
        self.suppressed_diagnostics.push(response.diagnostics());
        self.pending_flush = Some((response, empty_body));
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
        let (template, body) = self.pending_flush.take()?;
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
        let response = CosmosResponse::new(body, headers, template.status(), diagnostics);
        self.clear_suppressed();
        Some(PageResult::Page {
            response,
            is_terminal: true,
        })
    }
}

#[async_trait]
impl PipelineNode for SkipTake {
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
                    // If the skip consumed the final backend page(s) without a
                    // terminal marker, surface their RU/diagnostics as a final
                    // empty page rather than dropping them.
                    if let Some(flushed) = self.flush_suppressed() {
                        return Ok(flushed);
                    }
                    return Ok(PageResult::Drained);
                }
                PageResult::SplitRequired { replacement_nodes } => {
                    // The wrapped fan-out node absorbs splits internally and
                    // never surfaces `SplitRequired`; forward defensively so a
                    // future child type that does is not silently dropped.
                    return Ok(PageResult::SplitRequired { replacement_nodes });
                }
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    let bytes: &[u8] = match response.body() {
                        ResponseBody::Bytes(b) => b.as_ref(),
                        ResponseBody::NoPayload => &[],
                        ResponseBody::Items(_) => {
                            return Err(crate::error::CosmosError::builder()
                                .with_status(
                                    crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
                                )
                                .with_message(
                                    "SkipTake received a pre-split Items response; expected a raw \
                                     query page envelope",
                                )
                                .build());
                        }
                    };

                    let outcome = query_response::skip_take_page(
                        bytes,
                        self.remaining_skip,
                        self.remaining_take,
                    )?;
                    self.remaining_skip -= outcome.dropped;
                    if let Some(take) = self.remaining_take.as_mut() {
                        *take -= outcome.emitted;
                    }

                    let take_exhausted = matches!(self.remaining_take, Some(0));
                    let terminal = is_terminal || take_exhausted;

                    // Suppress a page that was fully consumed by the outstanding
                    // skip and is not the child's last page — pull the next page
                    // instead of surfacing an empty intermediate page. Retain the
                    // page's RU/diagnostics so the next emitted page accounts for
                    // them.
                    if outcome.emitted == 0 && !terminal {
                        self.suppress(response, outcome.body);
                        continue;
                    }

                    if take_exhausted {
                        self.exhausted = true;
                    }

                    let new_response = self.rebuild(&response, outcome.body, outcome.emitted);
                    return Ok(PageResult::Page {
                        response: new_response,
                        is_terminal: terminal,
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
        // Once the window is fully satisfied there is nothing left to resume.
        if self.exhausted {
            return Ok(PipelineNodeState::Drained);
        }
        Ok(PipelineNodeState::SkipTake {
            stage: self.stage,
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
        let body = match response.body() {
            ResponseBody::Bytes(b) => b.to_vec(),
            _ => panic!("expected Bytes body"),
        };
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        v["Documents"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["id"].as_u64().unwrap())
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
        let mut node = SkipTake::new(Box::new(child), 0, Some(2), SkipTakeStage::Top);
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
        let mut node = SkipTake::new(Box::new(child), 3, None, SkipTakeStage::OffsetLimit);
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
        let mut node = SkipTake::new(Box::new(child), 1, Some(3), SkipTakeStage::OffsetLimit);
        assert_eq!(drain_ids(&mut node).await, vec![2, 3, 4]);
    }

    #[tokio::test]
    async fn skip_larger_than_total_yields_nothing() {
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 10, Some(5), SkipTakeStage::OffsetLimit);
        assert_eq!(drain_ids(&mut node).await, Vec::<u64>::new());
    }

    #[tokio::test]
    async fn snapshot_reports_progress_then_drained() {
        let child = MockLeaf::with_pages(vec![page_result(&[1, 2, 3, 4], false)]);
        let mut node = SkipTake::new(Box::new(child), 1, Some(2), SkipTakeStage::OffsetLimit);
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
        let mut node = SkipTake::new(Box::new(child), 1, Some(5), SkipTakeStage::OffsetLimit);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let _ = node.next_page(&mut context).await.unwrap(); // emits id 2
        match node.snapshot_state().unwrap() {
            PipelineNodeState::SkipTake {
                stage,
                remaining_skip,
                remaining_take,
                ..
            } => {
                assert_eq!(stage, SkipTakeStage::OffsetLimit);
                assert_eq!(remaining_skip, 0);
                assert_eq!(remaining_take, Some(4));
            }
            other => panic!("expected SkipTake snapshot, got {other:?}"),
        }
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
        let mut node = SkipTake::new(Box::new(child), 3, None, SkipTakeStage::OffsetLimit);
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
        let mut node = SkipTake::new(Box::new(child), 10, None, SkipTakeStage::OffsetLimit);
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
