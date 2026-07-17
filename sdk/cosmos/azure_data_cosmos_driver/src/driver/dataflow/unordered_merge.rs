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
}

impl UnorderedMerge {
    /// Creates a new unordered merge over the given children.
    pub(crate) fn new(children: Vec<Box<dyn PipelineNode>>) -> Self {
        Self {
            children: children.into(),
            cursor: 0,
            start_marker: None,
        }
    }

    /// Sets the change feed start marker carried in snapshots of this node.
    pub(crate) fn with_start_marker(mut self, start_marker: Option<ChangeFeedStartFrom>) -> Self {
        self.start_marker = start_marker;
        self
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
}
