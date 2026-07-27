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

use async_trait::async_trait;

use crate::models::{CosmosResponse, FeedRange, ResponseBody};

use super::{query_response, PageResult, PipelineContext, PipelineNode, PipelineNodeState};

/// Applies a global `OFFSET` (`remaining_skip`) then `LIMIT`/`TOP`
/// (`remaining_take`, `None` = unbounded) over its single child's pages.
pub(crate) struct SkipTake {
    child: Box<dyn PipelineNode>,
    remaining_skip: u64,
    remaining_take: Option<u64>,
    /// Set once `take` is satisfied (or the child drains) so subsequent pulls
    /// short-circuit to `Drained` without touching the child again.
    exhausted: bool,
}

impl SkipTake {
    /// Wraps `child`, skipping `skip` documents then taking up to `take`
    /// (`None` = all remaining).
    pub(crate) fn new(child: Box<dyn PipelineNode>, skip: u64, take: Option<u64>) -> Self {
        Self {
            child,
            remaining_skip: skip,
            remaining_take: take,
            exhausted: false,
        }
    }

    /// Rebuilds a response around a trimmed body, preserving status,
    /// diagnostics, and headers (with `x-ms-item-count` updated to `emitted`).
    fn rebuild(response: &CosmosResponse, body: Vec<u8>, emitted: u64) -> CosmosResponse {
        let mut headers = response.headers().clone();
        headers.item_count = Some(emitted as u32);
        CosmosResponse::new(body, headers, response.status(), response.diagnostics())
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
                    // instead of surfacing an empty intermediate page.
                    if outcome.emitted == 0 && !terminal {
                        continue;
                    }

                    if take_exhausted {
                        self.exhausted = true;
                    }

                    let new_response = Self::rebuild(&response, outcome.body, outcome.emitted);
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
        let mut node = SkipTake::new(Box::new(child), 0, Some(2));
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
        let mut node = SkipTake::new(Box::new(child), 3, None);
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
        let mut node = SkipTake::new(Box::new(child), 1, Some(3));
        assert_eq!(drain_ids(&mut node).await, vec![2, 3, 4]);
    }

    #[tokio::test]
    async fn skip_larger_than_total_yields_nothing() {
        let child = MockLeaf::with_pages(vec![
            page_result(&[1, 2], false),
            page_result(&[3], true),
            Ok(PageResult::Drained),
        ]);
        let mut node = SkipTake::new(Box::new(child), 10, Some(5));
        assert_eq!(drain_ids(&mut node).await, Vec::<u64>::new());
    }

    #[tokio::test]
    async fn snapshot_reports_progress_then_drained() {
        let child = MockLeaf::with_pages(vec![page_result(&[1, 2, 3, 4], false)]);
        let mut node = SkipTake::new(Box::new(child), 1, Some(2));
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
        let mut node = SkipTake::new(Box::new(child), 1, Some(5));
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
}
