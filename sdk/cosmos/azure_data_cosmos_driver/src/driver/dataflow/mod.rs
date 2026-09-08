// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dataflow pipeline nodes for paged Cosmos DB operations.
//!
//! Everything in this module is driver-internal except [`OperationPlan`],
//! which is the only type re-exported to public APIs. The rest is the
//! machinery `CosmosDriver` uses to plan, execute, and resume paged
//! operations.
//!
//! # Navigation map
//!
//! - Leaf nodes: [`Request`] (executes a single Cosmos DB request and pages
//!   through continuation tokens) and [`DrainedLeaf`] (a no-op leaf used when
//!   resuming an already-completed plan).
//! - Intermediate nodes: [`SequentialDrain`] iterates EPK-ordered children
//!   left-to-right, draining each before advancing. [`UnorderedMerge`] polls
//!   children round-robin without evicting them, suitable for change feed.
//!   [`StreamingOrderedMerge`] k-way merges globally-ordered `ORDER BY`
//!   results across children, each executing a Gateway-rewritten query.
//! - Planner: [`planner::build_trivial_pipeline`] handles point reads and
//!   single-partition operations; [`planner::build_sequential_drain`] handles
//!   natural-order cross-partition queries; [`planner::build_streaming_ordered_merge`]
//!   handles cross-partition `ORDER BY` queries — all by consuming a backend
//!   query plan and resolving it against the current topology.
//! - Serializable state: [`PipelineNodeState`] (see [`snapshot`]) is the
//!   in-memory shape of a continuation snapshot; the wire-format token lives
//!   in [`crate::models::ContinuationToken`].
//! - Topology adapter: [`CachedTopologyProvider`] backs the
//!   [`TopologyProvider`] trait with the driver's
//!   [`PartitionKeyRangeCache`](crate::driver::cache::PartitionKeyRangeCache).
//!
//! See `FEED_OPERATIONS_REQS.md` for the design intent behind the dataflow
//! pipeline (paged operations, split recovery, continuation tokens, planned
//! cross-partition strategies).

mod binary_heap;
mod context;
mod distinct;
pub(crate) mod distinct_hash;
mod drain;
mod drained;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
pub(crate) mod mocks;
mod node;
mod non_streaming_ordered_merge;
pub(crate) mod order_by;
mod pipeline;
pub(crate) mod planner;
pub(crate) mod query_plan;
mod query_response;
mod request;
mod skip_take;
mod skip_take_page;
mod snapshot;
mod streaming_ordered_merge;
mod topology;
mod unordered_merge;

pub(crate) use context::{
    PartitionRoutingRefresh, PipelineContext, RequestExecutor, ResolvedRange, TopologyProvider,
};
pub(crate) use distinct::Distinct;
pub(crate) use drain::SequentialDrain;
pub(crate) use drained::DrainedLeaf;
pub(crate) use node::{
    split_replacement_invalid, validate_exact_coverage, PageResult, PipelineNode, SplitReplacements,
};
pub(crate) use non_streaming_ordered_merge::NonStreamingOrderedMerge;
pub use pipeline::OperationPlan;
pub(crate) use pipeline::Pipeline;
pub(crate) use request::{intersect_feed_ranges, Request, RequestTarget};
pub(crate) use skip_take::SkipTake;
pub(crate) use snapshot::{PipelineNodeState, RangedToken};
pub(crate) use streaming_ordered_merge::StreamingOrderedMerge;
pub(crate) use topology::CachedTopologyProvider;
pub(crate) use unordered_merge::UnorderedMerge;

#[cfg(test)]
mod tests {
    use super::mocks::*;
    use super::*;

    #[tokio::test]
    async fn pipeline_forwards_pages_from_root() {
        let mut pipeline =
            Pipeline::new(Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
                response: response(b"page"),
                is_terminal: false,
            })])));
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = pipeline.next_page(&mut context).await.unwrap().unwrap();

        assert_eq!(page.body_bytes(), b"page");
    }

    /// Builds a plan over a single-page mock leaf. The operation is a
    /// `read_database`, so minting a token normally fails with the *non-query*
    /// error — which is what makes it a usable control below.
    fn plan() -> OperationPlan {
        let pipeline = Pipeline::new(Box::new(MockLeaf::with_pages(vec![Ok(PageResult::Page {
            response: response(b"page"),
            is_terminal: true,
        })])));
        OperationPlan::new(
            pipeline,
            std::sync::Arc::new(operation()),
            crate::options::PlanOptions::default(),
            false,
        )
    }

    /// A poisoned plan must refuse to mint rather than hand back a token that
    /// skips the page the caller never received.
    #[test]
    fn poisoned_plan_refuses_to_mint_a_continuation_token() {
        let mut plan = plan();
        plan.poison_continuation();

        let err = plan
            .to_continuation_token()
            .expect_err("a poisoned plan must not mint a token");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE),
            "got: {err}"
        );
    }

    /// Control for the test above: the same plan fails for its own unrelated
    /// reason, proving the poison check is what produced the error there.
    #[test]
    fn a_clean_plan_reaches_the_normal_minting_path() {
        let err = plan()
            .to_continuation_token()
            .expect_err("a read_database operation cannot be tokenized");
        assert_ne!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE),
            "a plan that was never poisoned must not report transcode poisoning; got: {err}"
        );
    }
}
