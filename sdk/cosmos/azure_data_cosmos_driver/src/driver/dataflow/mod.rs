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
//! - Planner: [`planner::build_trivial_pipeline`] handles point reads and
//!   single-partition operations; [`planner::build_sequential_drain`] handles
//!   cross-partition queries by consuming a backend query plan and resolving
//!   it against the current topology.
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

mod context;
mod drain;
mod drained;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
pub(crate) mod mocks;
mod node;
mod pipeline;
pub(crate) mod planner;
pub(crate) mod query_plan;
mod request;
mod snapshot;
mod topology;
mod unordered_merge;

pub(crate) use context::{
    PartitionRoutingRefresh, PipelineContext, RequestExecutor, ResolvedRange, TopologyProvider,
};
pub(crate) use drain::SequentialDrain;
pub(crate) use drained::DrainedLeaf;
pub(crate) use node::{PageResult, PipelineNode};
pub use pipeline::OperationPlan;
pub(crate) use pipeline::Pipeline;
pub(crate) use request::{intersect_feed_ranges, Request, RequestTarget};
pub(crate) use snapshot::{PipelineNodeState, RangedToken};
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
}
