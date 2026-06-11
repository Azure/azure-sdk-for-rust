// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sequential drain node for cross-partition feed operations.
//!
//! `SequentialDrain` iterates its children in EPK order (left to right),
//! fully draining one child before advancing to the next. When a child
//! signals a partition split via [`PageResult::SplitRequired`], the drain
//! splices replacement nodes into its children list and retries.

use std::collections::VecDeque;

use async_trait::async_trait;

use crate::models::FeedRange;

use super::{PageResult, PipelineContext, PipelineNode, PipelineNodeState, RangedChildState};

/// Maximum number of consecutive split retries before giving up.
///
/// In practice a split produces 2–3 new ranges. This limit prevents infinite
/// loops if the topology provider keeps returning splits.
const MAX_SPLIT_RETRIES: usize = 10;

/// Sentinel `RangedChildState` emitted by `snapshot_state` when an
/// invariant violation (a child without a `feed_range`) would otherwise
/// silently drop the child's range from the snapshot. The min > max shape
/// is rejected by the planner's continuation-token validator with
/// `CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE`, surfacing the corrupt
/// snapshot at the next resume instead of letting the planner treat the
/// missing range as already-drained scope.
fn poison_sentinel_child() -> RangedChildState {
    RangedChildState {
        min_epk: "FF".to_owned(),
        max_epk: "00".to_owned(),
        state: PipelineNodeState::Request {
            server_continuation: None,
        },
    }
}

/// Drains child nodes sequentially in EPK order.
///
/// Each call to `next_page` returns the next page from the left-most (lowest EPK)
/// child. When that child is drained, it is removed and the next child becomes active.
/// When all children are drained, the node itself is drained.
pub(crate) struct SequentialDrain {
    children: VecDeque<Box<dyn PipelineNode>>,
}

impl SequentialDrain {
    /// Creates a new sequential drain over the given children.
    ///
    /// Children must be ordered by EPK range from smallest to largest.
    pub(crate) fn new(children: Vec<Box<dyn PipelineNode>>) -> Self {
        Self {
            children: children.into(),
        }
    }
}

#[async_trait]
impl PipelineNode for SequentialDrain {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        let mut split_retries = 0;

        loop {
            let Some(current) = self.children.front_mut() else {
                return Ok(PageResult::Drained);
            };

            match current.next_page(context).await? {
                PageResult::Page {
                    response,
                    is_terminal,
                } => {
                    if is_terminal {
                        // The front child has emitted its last page; evict it
                        // now so a snapshot taken after this call no longer
                        // references it. The drain itself is terminal only
                        // when this was its last child.
                        self.children.pop_front();
                        return Ok(PageResult::Page {
                            response,
                            is_terminal: self.children.is_empty(),
                        });
                    }
                    return Ok(PageResult::Page {
                        response,
                        is_terminal: false,
                    });
                }
                PageResult::Drained => {
                    self.children.pop_front();
                    // Loop to try the next child.
                }
                PageResult::SplitRequired { replacement_nodes } => {
                    split_retries += 1;
                    if split_retries > MAX_SPLIT_RETRIES {
                        // This should be ridiculously rare.
                        // The topology provider already waits for splits to converge before returning.
                        return Err(crate::error::CosmosError::builder()
                            .with_status(crate::error::CosmosStatus::CLIENT_SPLIT_RETRIES_EXHAUSTED)
                            .with_message(format!(
                                "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                 in SequentialDrain"
                            ))
                            .build());
                    }

                    // Remove the split child and splice in replacements at the front.
                    self.children.pop_front();
                    for (i, node) in replacement_nodes.into_iter().enumerate() {
                        self.children.insert(i, node);
                    }
                    // Loop to drain the first replacement.
                }
            }
        }
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.children.into_iter().collect()
    }

    fn snapshot_state(&self) -> PipelineNodeState {
        // Walk every still-pending child and record its range + state. The
        // resulting list is the authoritative remaining-work ledger: ranges
        // that are not present here have already been drained and must not
        // be re-queried on resume. Gaps outside any saved range are treated
        // as already-drained scope by the planner — so any missing-range
        // child silently dropped here would re-emit as silent data loss on
        // resume. Instead we APPEND a poison sentinel that the planner's
        // continuation-token validator hard-fails on (min > max), keeping
        // the surviving children's ranges in the wire payload so the error
        // path retains diagnostic context.
        let mut children = Vec::with_capacity(self.children.len() + 1);
        let mut saw_missing_range = false;
        for (idx, child) in self.children.iter().enumerate() {
            let Some(range) = child.feed_range() else {
                debug_assert!(
                    false,
                    "SequentialDrain child {idx} has no feed_range; emitting poison sentinel",
                );
                tracing::warn!(
                    child_index = idx,
                    total_children = self.children.len(),
                    children_collected_so_far = children.len(),
                    "SequentialDrain child has no feed_range during snapshot_state; appending poison sentinel — next resume will hard-fail via the continuation-token validator",
                );
                saw_missing_range = true;
                continue;
            };
            children.push(RangedChildState {
                min_epk: range.min_inclusive().as_str().to_string(),
                max_epk: range.max_exclusive().as_str().to_string(),
                state: child.snapshot_state(),
            });
        }
        if saw_missing_range {
            children.push(poison_sentinel_child());
            return PipelineNodeState::SequentialDrain { children };
        }
        if children.is_empty() {
            return PipelineNodeState::Drained;
        }
        PipelineNodeState::SequentialDrain { children }
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.children.front().and_then(|c| c.feed_range())
    }

    fn topology_can_change(&self) -> bool {
        // A SequentialDrain holds child nodes that cover the relevant EPK ranges,
        // thus it cannot itself be the target of a topology change error that would cause a split or merge.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::*;
    use crate::models::effective_partition_key::EffectivePartitionKey;

    #[tokio::test]
    async fn drains_single_child() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: response(b"p2"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let mut drain = SequentialDrain::new(vec![Box::new(child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"p2"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn drains_multiple_children_in_order() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c1-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c2-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: response(b"c2-p2"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let child3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c3-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let mut drain =
            SequentialDrain::new(vec![Box::new(child1), Box::new(child2), Box::new(child3)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c2-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c2-p2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c3-p1"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn empty_drain_is_immediately_drained() {
        let mut drain = SequentialDrain::new(vec![]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn propagates_child_error() {
        let child = MockLeaf::with_pages(vec![Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("test error")
            .build())]);
        let mut drain = SequentialDrain::new(vec![Box::new(child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = drain.next_page(&mut context).await.unwrap_err();
        let rendered = err.to_string();
        assert!(rendered.ends_with("test error"), "unexpected: {rendered}");
    }

    #[tokio::test]
    async fn handles_split_of_first_child() {
        let replacement1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"split-left"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let replacement2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"split-right"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement1), Box::new(replacement2)],
        })]);

        let trailing_child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"trailing"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let mut drain = SequentialDrain::new(vec![Box::new(split_child), Box::new(trailing_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"split-left"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"split-right"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"trailing"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn handles_split_of_middle_child() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let replacement = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c2-split"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement)],
        })]);

        let child3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c3"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let mut drain = SequentialDrain::new(vec![
            Box::new(child1),
            Box::new(split_child),
            Box::new(child3),
        ]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c2-split"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c3"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn handles_split_of_last_child() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let replacement = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"last-split"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement)],
        })]);

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(split_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"last-split"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn handles_cascading_split() {
        let final_leaf = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"final"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let cascading_replacement = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(final_leaf)],
        })]);

        let initial_split = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(cascading_replacement)],
        })]);

        let mut drain = SequentialDrain::new(vec![Box::new(initial_split)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"final"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn split_retry_limit_prevents_infinite_loop() {
        let mut current: Box<dyn PipelineNode> =
            Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: response(b"unreachable"),
                is_terminal: false,
            })]));

        for _ in 0..12 {
            current = Box::new(MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
                replacement_nodes: vec![current],
            })]));
        }

        let mut drain = SequentialDrain::new(vec![current]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = drain.next_page(&mut context).await.unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("exceeded maximum split retries (10) in SequentialDrain"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn child_drained_immediately_skips_to_next() {
        let empty_child = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let real_child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"data"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let mut drain = SequentialDrain::new(vec![Box::new(empty_child), Box::new(real_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"data"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn split_with_three_way_replacement() {
        let r1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"r1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let r2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"r2"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let r3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"r3"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(r1), Box::new(r2), Box::new(r3)],
        })]);

        let mut drain = SequentialDrain::new(vec![Box::new(split_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"r1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"r2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"r3"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn error_after_partial_drain() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"ok"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let child2 = MockLeaf::with_pages(vec![Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("boom")
            .build())]);

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(child2)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"ok"
        );
        let err = drain.next_page(&mut context).await.unwrap_err();
        let rendered = err.to_string();
        assert!(rendered.ends_with("boom"), "unexpected: {rendered}");
    }

    #[tokio::test]
    async fn multiple_pages_per_child_then_advance() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c1-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: response(b"c1-p2"),
                is_terminal: false,
            }),
            Ok(PageResult::Page {
                response: response(b"c1-p3"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c2-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(child2)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1-p2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c1-p3"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"c2-p1"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn split_produces_page_on_same_call() {
        let replacement = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"immediate"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ]);

        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement)],
        })]);

        let mut drain = SequentialDrain::new(vec![Box::new(split_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body_bytes(),
            b"immediate"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn terminal_page_pops_child_eagerly() {
        // The first child returns one terminal page; the drain must pop it
        // immediately so a snapshot taken right after the call already
        // points at the next child.
        let child1 = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"c1-final"),
            is_terminal: true,
        })])
        .with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("00"),
                EffectivePartitionKey::from("80"),
            )
            .unwrap(),
        );
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c2-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ])
        .with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("80"),
                EffectivePartitionKey::from("FF"),
            )
            .unwrap(),
        );

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(child2)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = unwrap_page(drain.next_page(&mut context).await);
        assert_eq!(page.body_bytes(), b"c1-final");

        // Snapshot must already reference only child2 (child1 was evicted on
        // its terminal page). The first entry's min_epk should be "80".
        let snapshot = drain.snapshot_state();
        let PipelineNodeState::SequentialDrain { children } = snapshot else {
            panic!("expected SequentialDrain snapshot, got {snapshot:?}");
        };
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].min_epk, "80");
        assert_eq!(children[0].max_epk, "FF");
    }

    #[tokio::test]
    async fn snapshot_preserves_all_pending_children() {
        // Mid-fan-out: drain has not advanced past child1 yet, so the
        // snapshot must record ALL three children's ranges + states.
        // This is the Defect-B regression guard: the old shape only
        // captured `front`, silently dropping child2/child3.
        let child1 = MockLeaf::with_pages(vec![]).with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("00"),
                EffectivePartitionKey::from("55"),
            )
            .unwrap(),
        );
        let child2 = MockLeaf::with_pages(vec![]).with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("55"),
                EffectivePartitionKey::from("AA"),
            )
            .unwrap(),
        );
        let child3 = MockLeaf::with_pages(vec![]).with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("AA"),
                EffectivePartitionKey::from("FF"),
            )
            .unwrap(),
        );
        let drain =
            SequentialDrain::new(vec![Box::new(child1), Box::new(child2), Box::new(child3)]);

        let snapshot = drain.snapshot_state();
        let PipelineNodeState::SequentialDrain { children } = snapshot else {
            panic!("expected SequentialDrain snapshot, got {snapshot:?}");
        };
        assert_eq!(children.len(), 3);
        assert_eq!(children[0].min_epk, "00");
        assert_eq!(children[0].max_epk, "55");
        assert_eq!(children[1].min_epk, "55");
        assert_eq!(children[1].max_epk, "AA");
        assert_eq!(children[2].min_epk, "AA");
        assert_eq!(children[2].max_epk, "FF");
    }

    #[tokio::test]
    async fn snapshot_of_empty_children_is_drained() {
        let drain = SequentialDrain::new(vec![]);
        assert!(matches!(drain.snapshot_state(), PipelineNodeState::Drained));
    }

    #[tokio::test]
    async fn terminal_page_on_last_child_marks_drain_terminal() {
        let only_child = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"final"),
            is_terminal: true,
        })])
        .with_feed_range(
            FeedRange::new(
                EffectivePartitionKey::from("00"),
                EffectivePartitionKey::from("FF"),
            )
            .unwrap(),
        );

        let mut drain = SequentialDrain::new(vec![Box::new(only_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        match drain.next_page(&mut context).await.unwrap() {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                assert_eq!(response.body_bytes(), b"final");
                assert!(is_terminal, "drain must propagate terminal flag");
            }
            other => panic!("expected Page, got {other:?}"),
        }
        assert!(matches!(drain.snapshot_state(), PipelineNodeState::Drained));
    }

    /// The poison sentinel emitted by `snapshot_state` when a child lacks
    /// a `feed_range` (an invariant violation) must be a min>max entry.
    /// That shape is rejected by the planner's continuation-token
    /// validator on the next resume with
    /// `CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE`, preventing the
    /// missing range from being treated as already-drained scope.
    #[test]
    fn poison_sentinel_child_has_min_greater_than_max() {
        let sentinel = poison_sentinel_child();
        assert!(
            sentinel.min_epk > sentinel.max_epk,
            "poison sentinel must have min>max so the validator rejects it; got min={} max={}",
            sentinel.min_epk,
            sentinel.max_epk,
        );
    }
}
