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
//! The design follows Spec 0012: Feed operations and dataflow, including paged
//! operations, split recovery, continuation tokens, and cross-partition strategies.

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
mod recovery_diagnostics;
mod request;
mod skip_take;
mod skip_take_page;
mod snapshot;
mod streaming_ordered_merge;
mod topology;
mod unordered_merge;

pub(crate) use context::{
    single_resolved_range, PartitionRoutingRefresh, PipelineContext, RequestExecutor,
    ResolvedRange, TopologyProvider,
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
    async fn poisoned_operators_collect_suppressed_current_and_pending_attempts() {
        use crate::error::status_codes;

        for distinct in [false, true] {
            let suppressed = response(br#"{"Documents":[]}"#).with_aggregated_prior_diagnostics(&[
                response_with_request_diagnostics(1).diagnostics(),
            ]);
            let current = response(b"{").with_aggregated_prior_diagnostics(&[
                response_with_request_diagnostics(1).diagnostics(),
            ]);
            let pending = gone_error_with_diagnostics();
            let expected_ids: Vec<_> = [
                suppressed.diagnostics(),
                current.diagnostics(),
                pending.diagnostics().unwrap(),
            ]
            .iter()
            .flat_map(|context| {
                context
                    .requests()
                    .iter()
                    .map(|request| request.activity_id().cloned())
                    .collect::<Vec<_>>()
            })
            .collect();
            let child = Box::new(
                MockLeaf::with_pages(vec![
                    Ok(PageResult::Page {
                        response: suppressed,
                        is_terminal: false,
                    }),
                    Ok(PageResult::Page {
                        response: current,
                        is_terminal: false,
                    }),
                ])
                .with_pending_error(pending),
            );
            let mut root: Box<dyn PipelineNode> = if distinct {
                Box::new(Distinct::new(child, query_plan::DistinctType::Ordered))
            } else {
                Box::new(SkipTake::new(child, 1, Some(1), false))
            };
            let mut executor = NoopRequestExecutor;
            let mut context = PipelineContext::new(&mut executor, None);
            let error = root.next_page(&mut context).await.unwrap_err();
            assert_eq!(
                error.status(),
                status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
            );
            let diagnostics = error.diagnostics().unwrap();
            assert_eq!(diagnostics.request_count(), 3);
            assert_eq!(
                diagnostics
                    .requests()
                    .iter()
                    .map(|request| request.activity_id().cloned())
                    .collect::<Vec<_>>(),
                expected_ids
            );
            assert_eq!(diagnostics.effective_status(), Some(error.status()));
            assert!(root.take_pending_error().is_none());
            assert!(root
                .next_page(&mut context)
                .await
                .unwrap_err()
                .diagnostics()
                .is_none());
        }
    }

    #[tokio::test]
    async fn outer_query_error_retains_recovered_and_suppressed_attempts() {
        use crate::{
            driver::dataflow::{
                distinct_hash::hash_value,
                query_plan::{DistinctType, SortOrder},
            },
            error::status_codes,
            models::{FeedRange, RequestCharge},
        };
        use std::sync::Arc;

        let first_row = serde_json::json!({
            "_rid": "first", "orderByItems": [{"item": 1}], "payload": {"id": 1},
        });
        let body =
            serde_json::to_vec(&serde_json::json!({"Documents": [first_row.clone()]})).unwrap();
        for (mode, malformed) in
            (0..4).flat_map(|mode| [false, true].map(|malformed| (mode, malformed)))
        {
            let leaf = Box::new(Request::new(
                Arc::new(operation()),
                RequestTarget::effective_partition_key_range(
                    FeedRange::full(),
                    "0".into(),
                    FeedRange::full(),
                ),
                None,
            ));
            let drain: Box<dyn PipelineNode> = Box::new(SequentialDrain::new(vec![leaf]));
            let root: Box<dyn PipelineNode> = match mode {
                0 => Box::new(SkipTake::new(drain, 1, None, false)),
                1 | 3 => {
                    let distinct = Box::new(Distinct::with_last_hash(
                        drain,
                        DistinctType::Ordered,
                        Some(hash_value(&first_row).unwrap()),
                        false,
                    ));
                    if mode == 3 {
                        Box::new(SkipTake::new(distinct, 0, None, false))
                    } else {
                        distinct
                    }
                }
                _ => Box::new(NonStreamingOrderedMerge::new(
                    drain,
                    vec![SortOrder::Ascending],
                    2,
                    0,
                    2,
                    None,
                    false,
                )),
            };
            let mut responses = vec![
                Err(gone_error_with_diagnostics()),
                Ok(response_with_continuation(&body, Some("next"))
                    .with_aggregated_prior_diagnostics(&[
                        response_with_request_diagnostics(1).diagnostics()
                    ])),
            ];
            if malformed {
                responses.push(Ok(response(b"{").with_aggregated_prior_diagnostics(&[
                    response_with_request_diagnostics(1).diagnostics(),
                ])));
            } else {
                responses.extend((0..11).map(|_| Err(gone_error_with_diagnostics())));
            }
            responses.push(Ok(response(br#"{"Documents":[{"_rid":"next","orderByItems":[{"item":2}],"payload":{"id":2}}]}"#)
                .with_aggregated_prior_diagnostics(&[response_with_request_diagnostics(1).diagnostics()])));
            let mut executor = MockRequestExecutor::new(responses);
            let mut topology = PhysicalTopologyProvider::new(vec![ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: Vec::new(),
                range: FeedRange::full(),
            }]);
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            let mut pipeline = Pipeline::new(root);
            let error = pipeline.next_page(&mut context).await.unwrap_err();
            let expected_status = match (malformed, mode) {
                (true, 2) => status_codes::SERVICE_ORDER_BY_ENVELOPE_INVALID,
                (true, _) => status_codes::SERIALIZATION_RESPONSE_BODY_INVALID,
                (false, _) => status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED,
            };
            assert_eq!(error.status(), expected_status);
            let diagnostics = error.diagnostics().unwrap();
            assert_eq!(
                diagnostics.request_count(),
                if malformed { 3 } else { 13 },
                "wrapper mode {mode}"
            );
            assert_eq!(
                diagnostics.total_request_charge(),
                RequestCharge::new(if malformed { 1.0 } else { 12.0 })
            );
            assert_eq!(diagnostics.effective_status(), Some(error.status()));
            let json: serde_json::Value =
                serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
            assert_eq!(
                json["topology_recovery"]["total_attempts"],
                if malformed { 1 } else { 12 }
            );
            if malformed {
                continue;
            }
            let next = pipeline.next_page(&mut context).await.unwrap().unwrap();
            assert_eq!(
                next.diagnostics().request_count(),
                1,
                "already reported attempts must not be replayed"
            );
            assert!(!next
                .diagnostics()
                .to_json_string(None)
                .contains("topology_recovery"));
        }
    }

    #[tokio::test]
    async fn split_exhaustion_retains_failed_attempts() {
        use crate::error::status_codes;
        use crate::models::{FeedRange, RequestCharge};
        use std::sync::Arc;

        for mode in 0..3 {
            let responses = (0..11)
                .map(|_| Err(gone_error_with_diagnostics()))
                .collect();
            let leaf = Box::new(Request::new(
                Arc::new(operation()),
                RequestTarget::effective_partition_key_range(
                    FeedRange::full(),
                    "0".into(),
                    FeedRange::full(),
                ),
                None,
            ));
            let root: Box<dyn PipelineNode> = if mode > 0 {
                Box::new(UnorderedMerge::new(vec![leaf]).with_prime_on_first_drain(mode == 2))
            } else {
                Box::new(SequentialDrain::new(vec![leaf]))
            };
            let mut pipeline = Pipeline::new(root);
            let mut executor = MockRequestExecutor::new(responses);
            let mut topology = mocks::PhysicalTopologyProvider::new(vec![ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: Vec::new(),
                range: FeedRange::full(),
            }]);
            let error = pipeline
                .next_page(&mut PipelineContext::new(
                    &mut executor,
                    Some(&mut topology),
                ))
                .await
                .unwrap_err();
            assert_eq!(error.status(), status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED);
            let diagnostics = error
                .diagnostics()
                .expect("split exhaustion must retain diagnostics");
            assert_eq!(diagnostics.request_count(), 11);
            assert_eq!(diagnostics.total_request_charge(), RequestCharge::new(11.0));
            assert_eq!(diagnostics.effective_status(), Some(error.status()));
            assert!(diagnostics
                .requests()
                .iter()
                .all(|request| *request.status() == gone_error().status()));
            assert!(diagnostics
                .requests()
                .iter()
                .all(|request| request.endpoint() == "https://acct.example/"));
            let json: serde_json::Value =
                serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
            assert_eq!(json["topology_recovery"]["total_attempts"], 11);
            assert_eq!(
                json["topology_recovery"]["attempts"]
                    .as_array()
                    .unwrap()
                    .len(),
                11
            );
        }
    }

    #[tokio::test]
    async fn split_recovery_reports_parent_once_across_children() {
        use crate::models::{effective_partition_key::EffectivePartitionKey, FeedRange};
        use std::sync::Arc;
        for unordered in [false, true] {
            let leaf = Box::new(Request::new(
                Arc::new(operation()),
                RequestTarget::effective_partition_key_range(
                    FeedRange::full(),
                    "0".into(),
                    FeedRange::full(),
                ),
                None,
            ));
            let root: Box<dyn PipelineNode> = if unordered {
                Box::new(UnorderedMerge::new(vec![leaf]))
            } else {
                Box::new(SequentialDrain::new(vec![leaf]))
            };
            let mut pipeline = Pipeline::new(root);
            let mut executor = MockRequestExecutor::new(vec![
                Err(gone_error_with_diagnostics()),
                Ok(response_with_request_diagnostics(1)),
                Ok(response_with_request_diagnostics(1)),
            ]);
            let boundary = EffectivePartitionKey::try_from("80").unwrap();
            let mut topology = PhysicalTopologyProvider::new(vec![
                ResolvedRange {
                    partition_key_range_id: "1".into(),
                    parents: vec!["0".into()],
                    range: FeedRange::new(EffectivePartitionKey::MIN, boundary.clone()).unwrap(),
                },
                ResolvedRange {
                    partition_key_range_id: "2".into(),
                    parents: vec!["0".into()],
                    range: FeedRange::new(boundary, EffectivePartitionKey::MAX).unwrap(),
                },
            ]);
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            let first = pipeline.next_page(&mut context).await.unwrap().unwrap();
            let second = pipeline.next_page(&mut context).await.unwrap().unwrap();
            assert_eq!(first.diagnostics().request_count(), 2);
            assert_eq!(second.diagnostics().request_count(), 1);
            let first_json: serde_json::Value =
                serde_json::from_str(first.diagnostics().to_json_string(None)).unwrap();
            assert_eq!(
                first_json["topology_recovery"]["attempts"][0]["total_replacements"],
                2
            );
            assert!(!second
                .diagnostics()
                .to_json_string(None)
                .contains("topology_recovery"));
        }
    }

    #[tokio::test]
    async fn split_refresh_failure_retains_original_attempt() {
        use crate::models::FeedRange;
        use std::sync::Arc;
        let leaf = Box::new(Request::new(
            Arc::new(operation()),
            RequestTarget::effective_partition_key_range(
                FeedRange::full(),
                "0".into(),
                FeedRange::full(),
            ),
            None,
        ));
        let mut pipeline = Pipeline::new(Box::new(SequentialDrain::new(vec![leaf])));
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error_with_diagnostics())]);
        let mut topology = NoopTopologyProvider;
        let error = pipeline
            .next_page(&mut PipelineContext::new(
                &mut executor,
                Some(&mut topology),
            ))
            .await
            .unwrap_err();
        let diagnostics = error.diagnostics().unwrap();
        assert_eq!(diagnostics.request_count(), 1);
        assert_eq!(diagnostics.effective_status(), Some(error.status()));
        let json: serde_json::Value =
            serde_json::from_str(diagnostics.to_json_string(None)).unwrap();
        assert_eq!(
            json["topology_recovery"]["attempts"][0]["error_status"]["status"],
            "400"
        );
    }

    #[tokio::test]
    async fn topology_recovery_retains_attempts_when_retry_fails() {
        use crate::models::FeedRange;
        use std::sync::Arc;
        for logical in [false, true] {
            let target = if logical {
                logical_partition_target()
            } else {
                RequestTarget::effective_partition_key_range(
                    FeedRange::full(),
                    "0".into(),
                    FeedRange::full(),
                )
            };
            let leaf = Box::new(Request::new(Arc::new(operation()), target, None));
            let mut pipeline = Pipeline::new(Box::new(SequentialDrain::new(vec![leaf])));
            let later_status =
                crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest);
            let later = crate::error::CosmosError::builder()
                .with_status(later_status)
                .with_diagnostics(response_with_request_diagnostics(1).diagnostics())
                .build();
            let mut executor =
                MockRequestExecutor::new(vec![Err(gone_error_with_diagnostics()), Err(later)]);
            let mut topology = PhysicalTopologyProvider::new(vec![ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: Vec::new(),
                range: FeedRange::full(),
            }]);
            let error = pipeline
                .next_page(&mut PipelineContext::new(
                    &mut executor,
                    Some(&mut topology),
                ))
                .await
                .unwrap_err();
            assert_eq!(error.status(), later_status);
            assert_eq!(
                error.diagnostics().unwrap().effective_status(),
                Some(later_status)
            );
            assert_eq!(error.diagnostics().unwrap().request_count(), 2);
        }
    }

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
            Some(crate::error::status_codes::substatus::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE),
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
            Some(crate::error::status_codes::substatus::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE),
            "a plan that was never poisoned must not report transcode poisoning; got: {err}"
        );
    }

    #[test]
    fn planning_deadline_is_consumed_without_remaining_on_reusable_plan() {
        let mut plan = plan();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);

        plan.set_initial_execution_deadline(deadline);

        assert!(plan.operation.absolute_deadline().is_none());
        assert_eq!(plan.take_initial_execution_deadline(), Some(deadline));
        assert!(plan.take_initial_execution_deadline().is_none());
        assert!(plan.operation.absolute_deadline().is_none());
    }
}
