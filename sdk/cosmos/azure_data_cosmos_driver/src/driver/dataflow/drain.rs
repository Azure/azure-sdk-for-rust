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

use super::{ChildNodes, PageResult, PipelineContext, PipelineNode, PipelineNodeState};

/// Maximum number of consecutive split retries before giving up.
///
/// In practice a split produces 2–3 new ranges. This limit prevents infinite
/// loops if the topology provider keeps returning splits.
const MAX_SPLIT_RETRIES: usize = 10;

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
    ) -> azure_core::Result<PageResult> {
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
                        return Err(azure_core::Error::with_message(
                            azure_core::error::ErrorKind::Other,
                            format!(
                                "exceeded maximum split retries ({MAX_SPLIT_RETRIES}) \
                                 in SequentialDrain"
                            ),
                        ));
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

    fn children(&self) -> ChildNodes<'_> {
        let (front, back) = self.children.as_slices();
        if back.is_empty() {
            ChildNodes::Slice(front)
        } else {
            ChildNodes::Split(front, back)
        }
    }

    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        self.children.into_iter().collect()
    }

    fn snapshot_state(&self) -> PipelineNodeState {
        let Some(front) = self.children.front() else {
            return PipelineNodeState::Drained;
        };
        let Some(range) = front.feed_range() else {
            // Shouldn't happen for an EPK-ordered drain, but degrade gracefully:
            // serialize the child snapshot directly with no cursor.
            return front.snapshot_state();
        };
        PipelineNodeState::SequentialDrain {
            current_min_epk: range.min_inclusive().as_str().to_string(),
            left_most: Box::new(front.snapshot_state()),
        }
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.children.front().and_then(|c| c.feed_range())
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c2-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c2-p2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c3-p1"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn empty_drain_is_immediately_drained() {
        let mut drain = SequentialDrain::new(vec![]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn propagates_child_error() {
        let child = MockLeaf::with_pages(vec![Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "test error",
        ))]);
        let mut drain = SequentialDrain::new(vec![Box::new(child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let err = drain.next_page(&mut context).await.unwrap_err();
        assert_eq!(err.to_string(), "test error");
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"split-left"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"split-right"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c2-split"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let err = drain.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            err.to_string(),
            "exceeded maximum split retries (10) in SequentialDrain"
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"r1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"r2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let child2 = MockLeaf::with_pages(vec![Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "boom",
        ))]);

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(child2)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"ok"
        );
        let err = drain.next_page(&mut context).await.unwrap_err();
        assert_eq!(err.to_string(), "boom");
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1-p1"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1-p2"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"c1-p3"
        );
        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
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
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        assert_eq!(
            unwrap_page(drain.next_page(&mut context).await).body(),
            b"immediate"
        );
        assert_drained(drain.next_page(&mut context).await);
    }

    #[tokio::test]
    async fn children_returns_all_nodes() {
        let c1 = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let c2 = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let c3 = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);

        let drain = SequentialDrain::new(vec![Box::new(c1), Box::new(c2), Box::new(c3)]);
        assert_eq!(drain.children().len(), 3);
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
        .with_feed_range(FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("80"),
        ));
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page {
                response: response(b"c2-p1"),
                is_terminal: false,
            }),
            Ok(PageResult::Drained),
        ])
        .with_feed_range(FeedRange::new(
            EffectivePartitionKey::from("80"),
            EffectivePartitionKey::from("FF"),
        ));

        let mut drain = SequentialDrain::new(vec![Box::new(child1), Box::new(child2)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let page = unwrap_page(drain.next_page(&mut context).await);
        assert_eq!(page.body(), b"c1-final");

        // Snapshot must already reference child2 (cursor at "80"), not the
        // just-drained child1.
        let snapshot = drain.snapshot_state();
        let PipelineNodeState::SequentialDrain {
            current_min_epk, ..
        } = snapshot
        else {
            panic!("expected SequentialDrain snapshot, got {snapshot:?}");
        };
        assert_eq!(current_min_epk, "80");
    }

    #[tokio::test]
    async fn terminal_page_on_last_child_marks_drain_terminal() {
        let only_child = MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"final"),
            is_terminal: true,
        })])
        .with_feed_range(FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("FF"),
        ));

        let mut drain = SequentialDrain::new(vec![Box::new(only_child)]);
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        match drain.next_page(&mut context).await.unwrap() {
            PageResult::Page {
                response,
                is_terminal,
            } => {
                assert_eq!(response.body(), b"final");
                assert!(is_terminal, "drain must propagate terminal flag");
            }
            other => panic!("expected Page, got {other:?}"),
        }
        assert!(matches!(drain.snapshot_state(), PipelineNodeState::Drained));
    }
}
