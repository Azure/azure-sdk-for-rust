// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unordered merge node for cross-partition change feed operations.
//!
//! [`UnorderedMerge`] polls its children round-robin, yielding pages from
//! whichever child has results. Unlike [`SequentialDrain`](super::SequentialDrain),
//! children are **never evicted** when they return `is_terminal` (304 / no
//! continuation). This makes the node suitable for change feed, where every
//! partition must be polled indefinitely.

use std::collections::VecDeque;

use async_trait::async_trait;

use super::{PageResult, PipelineContext, PipelineNode, PipelineNodeState, RangedToken};
use crate::models::ChangeFeedStartFrom;

/// Maximum number of consecutive split retries before giving up.
const MAX_SPLIT_RETRIES: usize = 10;

/// Merges results from all child nodes in an unordered fashion.
///
/// Each call to `next_page` polls the next child in round-robin order.
/// Children that return 304 (no changes, `is_terminal: true`) are kept
/// alive with their continuation token so they can be polled again on the
/// next round. The node only reports `Drained` when it has no children at
/// all (which shouldn't happen in normal change feed usage).
pub(crate) struct UnorderedMerge {
    children: VecDeque<Box<dyn PipelineNode>>,
    /// Index of the next child to poll (wraps around).
    cursor: usize,
    /// The feed's original start position, carried so it can be re-persisted in
    /// the continuation token on every checkpoint. Partitions that were never
    /// polled re-apply it on resume instead of reading from the beginning.
    /// `None` means no start position was recorded.
    start_marker: Option<ChangeFeedStartFrom>,
    /// When set, poll every child once before serving the first page so each
    /// range records a concrete starting continuation (ETag) up front.
    ///
    /// Enabled only for a fresh AllVersionsAndDeletes feed. Without it, a range
    /// that is never polled before the first checkpoint has no saved
    /// continuation and resumes from the persisted `start_marker` (`Now`),
    /// which re-evaluates "now" at resume time and silently drops the
    /// intermediate versions and deletes that occurred in the gap. Priming
    /// pins every range to its actual starting position so resume is lossless.
    ///
    /// A fresh AllVersionsAndDeletes feed can only start from `Now`
    /// (`If-None-Match: *`; `Beginning`/`PointInTime` are rejected by the
    /// service), whose first poll is always a `304 Not Modified` carrying no
    /// items — only an ETag. Priming therefore captures each range's starting
    /// ETag without dropping any change. Cleared after the priming pass runs.
    prime_on_first_drain: bool,
}

impl UnorderedMerge {
    /// Creates a new unordered merge over the given children.
    pub(crate) fn new(children: Vec<Box<dyn PipelineNode>>) -> Self {
        Self {
            children: children.into(),
            cursor: 0,
            start_marker: None,
            prime_on_first_drain: false,
        }
    }

    /// Sets the change feed start marker carried in snapshots of this node.
    pub(crate) fn with_start_marker(mut self, start_marker: Option<ChangeFeedStartFrom>) -> Self {
        self.start_marker = start_marker;
        self
    }

    /// Enables priming every child once before the first page is served.
    ///
    /// See [`prime_on_first_drain`](Self::prime_on_first_drain). Set for a fresh
    /// AllVersionsAndDeletes feed so no range can resume from a stale `Now`.
    pub(crate) fn with_prime_on_first_drain(mut self, prime: bool) -> Self {
        self.prime_on_first_drain = prime;
        self
    }

    /// Polls every child once so each records its starting continuation.
    ///
    /// Only invoked for a fresh AllVersionsAndDeletes feed, whose first
    /// (start-from-`Now`) poll per range is a `304` with no items. The primed
    /// page is discarded; only the ETag it advanced — now held by the child —
    /// is retained, so the next real poll of that range resumes from its true
    /// starting position rather than from a resume-time `Now`.
    ///
    /// # Cost
    ///
    /// Priming front-loads one poll per range before the first page is served,
    /// so a wide fan-out AVAD feed pays N sequential round-trips (and N request
    /// units) of extra first-page latency. This is the price of the lossless-
    /// `Now` guarantee; LatestVersion feeds don't prime and don't pay it. Each
    /// priming poll goes through the same per-request path as a normal poll, so
    /// transient failures are retried by the pipeline's retry policy beneath
    /// this call. A non-retryable error on any single range aborts the whole
    /// first page (the `?` below) rather than starting the feed with an
    /// unpinned range, keeping the lossless guarantee all-or-nothing.
    async fn prime_children(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<()> {
        let mut idx = 0;
        while idx < self.children.len() {
            let mut split_retries = 0;
            loop {
                match self.children[idx].next_page(context).await? {
                    PageResult::Page { response, .. } => {
                        // Start-from-`Now` first poll: a 304 carrying only an
                        // ETag, which the child has now recorded. Discarding the
                        // (item-less) page loses nothing; advance to the next
                        // child. The ETag is what makes priming worthwhile — a
                        // 304 with no ETag would leave the child unpinned and
                        // silently resume from `Now`, so assert its presence to
                        // catch a service/transport contract violation in debug
                        // builds.
                        debug_assert!(
                            response.headers().etag.is_some(),
                            "priming poll returned a page without an ETag; the range \
                             cannot record its start position and would resume from `Now`"
                        );
                        idx += 1;
                        break;
                    }
                    PageResult::Drained => {
                        // A fully drained child contributes nothing; drop it and
                        // stay at the same index (the next child shifts in).
                        self.children.remove(idx);
                        break;
                    }
                    PageResult::SplitRequired { replacement_nodes } => {
                        split_retries += 1;
                        if split_retries > MAX_SPLIT_RETRIES {
                            return Err(crate::error::CosmosError::builder()
                                .with_status(
                                    crate::error::CosmosStatus::CLIENT_SPLIT_RETRIES_EXHAUSTED,
                                )
                                .with_message(format!(
                                    "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                     while priming UnorderedMerge children"
                                ))
                                .build());
                        }
                        // Splice the replacement ranges in place and re-prime
                        // from the first replacement (same index).
                        self.children.remove(idx);
                        for (i, node) in replacement_nodes.into_iter().enumerate() {
                            self.children.insert(idx + i, node);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl PipelineNode for UnorderedMerge {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        if self.children.is_empty() {
            return Ok(PageResult::Drained);
        }

        // For a fresh AllVersionsAndDeletes feed, poll every range once up front
        // so each records its concrete starting continuation before any
        // checkpoint can be taken. This runs exactly once.
        if self.prime_on_first_drain {
            self.prime_on_first_drain = false;
            self.prime_children(context).await?;
            if self.children.is_empty() {
                return Ok(PageResult::Drained);
            }
        }

        let mut split_retries = 0;
        let children_count = self.children.len();
        // Try up to `children_count` children to find one with data.
        // If all return terminal (304), return the last terminal response.
        let mut attempts = 0;

        loop {
            if self.children.is_empty() {
                return Ok(PageResult::Drained);
            }

            let idx = self.cursor % self.children.len();
            let child = &mut self.children[idx];

            match child.next_page(context).await? {
                PageResult::Page {
                    response,
                    // A child's `is_terminal` (304 / no continuation) is
                    // intentionally ignored: change feed partitions persist
                    // and may yield data on a later poll, so we never evict
                    // or propagate terminal upward.
                    is_terminal: _,
                } => {
                    // Advance cursor to next child for round-robin.
                    self.cursor = (idx + 1) % self.children.len();

                    // For change feed, is_terminal means "no more changes
                    // right now" (304). We keep the child alive — it will
                    // have new data on a future poll.
                    //
                    // Propagate the page to the caller. The iterator layer
                    // decides whether to surface 304 pages as empty results.
                    return Ok(PageResult::Page {
                        response,
                        // UnorderedMerge never signals terminal to its
                        // parent — the change feed stream is infinite.
                        is_terminal: false,
                    });
                }
                PageResult::Drained => {
                    // A child that's fully drained (not just 304) can be
                    // removed. This shouldn't happen in normal change feed
                    // usage but handles edge cases gracefully.
                    self.children.remove(idx);
                    if self.children.is_empty() {
                        return Ok(PageResult::Drained);
                    }
                    // `idx == self.cursor` here (idx is `cursor % len`), so the
                    // removed element is at the cursor itself: the next child
                    // simply shifts into `idx`. No decrement is needed; just
                    // re-wrap the cursor against the new, smaller length.
                    self.cursor %= self.children.len();
                    attempts += 1;
                    if attempts >= children_count {
                        return Ok(PageResult::Drained);
                    }
                }
                PageResult::SplitRequired { replacement_nodes } => {
                    split_retries += 1;
                    if split_retries > MAX_SPLIT_RETRIES {
                        return Err(crate::error::CosmosError::builder()
                            .with_status(crate::error::CosmosStatus::CLIENT_SPLIT_RETRIES_EXHAUSTED)
                            .with_message(format!(
                                "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                 in UnorderedMerge"
                            ))
                            .build());
                    }

                    // Remove the split child and splice in replacements.
                    self.children.remove(idx);
                    for (i, node) in replacement_nodes.into_iter().enumerate() {
                        self.children.insert(idx + i, node);
                    }
                    // Retry from the same position (first replacement).
                }
            }
        }
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.children.into_iter().collect()
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        if self.children.is_empty() {
            return Ok(PipelineNodeState::Drained);
        }

        let mut active_tokens: Vec<RangedToken> = Vec::new();

        for (idx, child) in self.children.iter().enumerate() {
            let Some(range) = child.feed_range() else {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE,
                    )
                    .with_message(format!(
                        "UnorderedMerge child {idx} of {total} has no feed_range; \
                         cannot snapshot continuation state safely",
                        total = self.children.len(),
                    ))
                    .build());
            };

            let child_state = child.snapshot_state()?;
            match child_state.into_child_contribution("UnorderedMerge", idx, self.children.len())? {
                super::snapshot::ChildSnapshotContribution::Drained => {
                    // Drained children contribute nothing to the token.
                }
                super::snapshot::ChildSnapshotContribution::Pending {
                    server_continuation,
                } => {
                    if let Some(token) = server_continuation {
                        active_tokens.push(RangedToken {
                            min_epk: range.min_inclusive().to_hex(),
                            max_epk: range.max_exclusive().to_hex(),
                            server_continuation: token,
                        });
                    }
                }
            }
        }

        Ok(PipelineNodeState::UnorderedMerge {
            active_tokens,
            start_from: self.start_marker.clone(),
        })
    }

    fn topology_can_change(&self) -> bool {
        // UnorderedMerge handles splits internally by splicing replacement
        // nodes into its children list, so it does not need a parent to do
        // so on its behalf.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::super::mocks::*;
    use super::*;

    #[tokio::test]
    async fn polls_children_round_robin() {
        let child_a = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"a1"),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: response(b"a2"),
                is_terminal: false,
            }),
        ]);
        let child_b = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"b1"),
            is_terminal: false,
        })]);

        let mut merge = UnorderedMerge::new(vec![Box::new(child_a), Box::new(child_b)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        // First poll → child 0 (a1)
        let r = merge.next_page(&mut ctx).await.unwrap();
        assert!(matches!(r, PageResult::Page { .. }));
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"a1");
        }

        // Second poll → child 1 (b1)
        let r = merge.next_page(&mut ctx).await.unwrap();
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"b1");
        }

        // Third poll → child 0 again (a2)
        let r = merge.next_page(&mut ctx).await.unwrap();
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"a2");
        }
    }

    #[tokio::test]
    async fn terminal_pages_do_not_evict_children() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"page1"),
                is_terminal: true, // 304 / no continuation
            }),
            Ok(PageResult::Page {
                response: response(b"page2"),
                is_terminal: false,
            }),
        ]);

        let mut merge = UnorderedMerge::new(vec![Box::new(child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        // First poll — terminal page, but child stays
        let r = merge.next_page(&mut ctx).await.unwrap();
        assert!(matches!(
            r,
            PageResult::Page {
                is_terminal: false,
                ..
            }
        ));

        // Second poll — child is still alive
        let r = merge.next_page(&mut ctx).await.unwrap();
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"page2");
        }
    }

    #[tokio::test]
    async fn handles_split_required() {
        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![
                Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: response(b"split-a"),
                    is_terminal: false,
                })])),
                Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: response(b"split-b"),
                    is_terminal: false,
                })])),
            ],
        })]);

        let mut merge = UnorderedMerge::new(vec![Box::new(split_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        // First poll triggers split, retries with first replacement
        let r = merge.next_page(&mut ctx).await.unwrap();
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"split-a");
        }

        // Second poll → second replacement
        let r = merge.next_page(&mut ctx).await.unwrap();
        if let PageResult::Page { response, .. } = r {
            assert_eq!(response.body_bytes(), b"split-b");
        }
    }

    #[tokio::test]
    async fn never_signals_terminal_to_parent() {
        let child = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"data"),
            is_terminal: true,
        })]);

        let mut merge = UnorderedMerge::new(vec![Box::new(child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        let r = merge.next_page(&mut ctx).await.unwrap();
        match r {
            PageResult::Page { is_terminal, .. } => {
                assert!(!is_terminal, "UnorderedMerge must never signal terminal");
            }
            other => panic!("expected Page, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn prime_on_first_drain_polls_every_child_once() {
        // Child 0 has a second page; children 1 and 2 have one each. Priming
        // must poll every child once (consuming a1, b1, c1) before the
        // round-robin serves a page, so the first served page is child 0's
        // *second* poll (a2) rather than a1.
        let child_a = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response_with_etag(b"a1", "etag-a1"),
                is_terminal: true,
            }),
            Ok(PageResult::Page {
                response: response(b"a2"),
                is_terminal: false,
            }),
        ]);
        let child_b = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response_with_etag(b"b1", "etag-b1"),
            is_terminal: true,
        })]);
        let child_c = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response_with_etag(b"c1", "etag-c1"),
            is_terminal: true,
        })]);

        let mut merge = UnorderedMerge::new(vec![
            Box::new(child_a),
            Box::new(child_b),
            Box::new(child_c),
        ])
        .with_prime_on_first_drain(true);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        let r = merge.next_page(&mut ctx).await.unwrap();
        match r {
            PageResult::Page { response, .. } => {
                assert_eq!(
                    response.body_bytes(),
                    b"a2",
                    "priming must consume every child's first poll before serving a page"
                );
            }
            other => panic!("expected Page, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn prime_on_first_drain_handles_split() {
        // A range that has split before its first poll must be primed through
        // its replacements: priming polls each replacement once before the
        // round-robin serves a page.
        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![
                Box::new(MockLeaf::with_pages(vec![
                    Ok(PageResult::Page {
                        response: response_with_etag(b"ra1", "etag-ra1"),
                        is_terminal: true,
                    }),
                    Ok(PageResult::Page {
                        response: response(b"ra2"),
                        is_terminal: false,
                    }),
                ])),
                Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                    response: response_with_etag(b"rb1", "etag-rb1"),
                    is_terminal: true,
                })])),
            ],
        })]);

        let mut merge =
            UnorderedMerge::new(vec![Box::new(split_child)]).with_prime_on_first_drain(true);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut ctx = PipelineContext::new(&mut executor, Some(&mut topology));

        // Priming splices in the two replacements and polls each once (ra1,
        // rb1); the served page is the first replacement's second poll (ra2).
        let r = merge.next_page(&mut ctx).await.unwrap();
        match r {
            PageResult::Page { response, .. } => {
                assert_eq!(response.body_bytes(), b"ra2");
            }
            other => panic!("expected Page, got {other:?}"),
        }
    }
}
