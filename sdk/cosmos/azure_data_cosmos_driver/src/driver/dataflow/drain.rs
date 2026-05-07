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

use super::{ChildNodes, PageResult, PipelineContext, PipelineNode};

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
                PageResult::Page(response) => return Ok(PageResult::Page(response)),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::mocks::*;

    #[tokio::test]
    async fn drains_single_child() {
        let child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"p1"))),
            Ok(PageResult::Page(response(b"p2"))),
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
            Ok(PageResult::Page(response(b"c1-p1"))),
            Ok(PageResult::Drained),
        ]);
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c2-p1"))),
            Ok(PageResult::Page(response(b"c2-p2"))),
            Ok(PageResult::Drained),
        ]);
        let child3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c3-p1"))),
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
        assert!(err.to_string().contains("test error"));
    }

    #[tokio::test]
    async fn handles_split_of_first_child() {
        let replacement1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"split-left"))),
            Ok(PageResult::Drained),
        ]);
        let replacement2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"split-right"))),
            Ok(PageResult::Drained),
        ]);

        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement1), Box::new(replacement2)],
        })]);

        let trailing_child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"trailing"))),
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
            Ok(PageResult::Page(response(b"c1"))),
            Ok(PageResult::Drained),
        ]);

        let replacement = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c2-split"))),
            Ok(PageResult::Drained),
        ]);
        let split_child = MockLeaf::with_pages(vec![Ok(PageResult::SplitRequired {
            replacement_nodes: vec![Box::new(replacement)],
        })]);

        let child3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c3"))),
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
            Ok(PageResult::Page(response(b"c1"))),
            Ok(PageResult::Drained),
        ]);

        let replacement = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"last-split"))),
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
            Ok(PageResult::Page(response(b"final"))),
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
        let mut current: Box<dyn PipelineNode> = Box::new(MockLeaf::with_pages(vec![Ok(
            PageResult::Page(response(b"unreachable")),
        )]));

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
        assert!(err.to_string().contains("split retries"));
    }

    #[tokio::test]
    async fn child_drained_immediately_skips_to_next() {
        let empty_child = MockLeaf::with_pages(vec![Ok(PageResult::Drained)]);
        let real_child = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"data"))),
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
            Ok(PageResult::Page(response(b"r1"))),
            Ok(PageResult::Drained),
        ]);
        let r2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"r2"))),
            Ok(PageResult::Drained),
        ]);
        let r3 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"r3"))),
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
            Ok(PageResult::Page(response(b"ok"))),
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
        assert!(err.to_string().contains("boom"));
    }

    #[tokio::test]
    async fn multiple_pages_per_child_then_advance() {
        let child1 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c1-p1"))),
            Ok(PageResult::Page(response(b"c1-p2"))),
            Ok(PageResult::Page(response(b"c1-p3"))),
            Ok(PageResult::Drained),
        ]);
        let child2 = MockLeaf::with_pages(vec![
            Ok(PageResult::Page(response(b"c2-p1"))),
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
            Ok(PageResult::Page(response(b"immediate"))),
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
}
