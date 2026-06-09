// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for cross-partition query resume across a
//! topology change (partition split).
//!
//! These tests compose the real planner, drain, request, snapshot, and
//! continuation-token layers together against the `MockRequestExecutor` /
//! `MockTopologyProvider` from `dataflow::mocks`, and exercise the same
//! serialize → resume round-trip the public iterator surfaces to callers.
//!
//! They guard end-to-end against two stacked defects:
//!
//! 1. After a partition split, `build_sequential_drain` must forward the
//!    saved continuation to every surviving leaf in the saved range's
//!    scope — not just the first.
//! 2. `SequentialDrain::snapshot_state` must record every still-pending
//!    sibling, not just the front, so that pausing a fan-out query
//!    mid-flight preserves the sibling siblings' un-started pre-split-token
//!    state across the serialize → resume boundary.
//!
//! Both regressions surfaced live as "items the caller already consumed on
//! a prior page are re-emitted on resume". Each test below drains the full
//! page sequence, then asserts every emitted page body appears exactly once
//! and the full expected set is present.

use std::sync::Arc;

use super::{
    mocks::{MockRequestExecutor, MockTopologyProvider},
    planner::build_sequential_drain,
    query_plan::{QueryPlan, QueryRange},
    Pipeline, PipelineContext, PipelineNodeState, RangedChildState, ResolvedRange,
};
use crate::{
    diagnostics::DiagnosticsContextBuilder,
    error::Result,
    models::{
        effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
        ContainerProperties, ContainerReference, ContinuationToken, CosmosOperation,
        CosmosResponse, CosmosResponseHeaders, CosmosStatus, FeedRange, ResolvedToken,
        SystemProperties,
    },
    options::DiagnosticsOptions,
};

// ── Test fixtures ───────────────────────────────────────────────────────────

fn test_account() -> AccountReference {
    AccountReference::with_master_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    )
}

fn test_container_props() -> ContainerProperties {
    use std::borrow::Cow;
    ContainerProperties {
        id: Cow::Owned("coll".into()),
        partition_key: serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap(),
        system_properties: SystemProperties::default(),
    }
}

fn test_container() -> ContainerReference {
    ContainerReference::new(
        test_account(),
        "db",
        "db_rid",
        "coll",
        "coll_rid",
        &test_container_props(),
    )
}

fn cross_partition_query_operation() -> Arc<CosmosOperation> {
    Arc::new(
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
    )
}

fn full_range_plan() -> QueryPlan {
    QueryPlan {
        partitioned_query_execution_info_version: 1,
        query_info: None,
        query_ranges: vec![QueryRange {
            min: String::new(),
            max: "FF".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: false,
        }],
        hybrid_search_query_info: None,
    }
}

fn resolved(min: &str, max: &str, pk_range_id: &str) -> ResolvedRange {
    ResolvedRange {
        partition_key_range_id: pk_range_id.to_string(),
        range: FeedRange::new(
            EffectivePartitionKey::from(min),
            EffectivePartitionKey::from(max),
        )
        .unwrap(),
    }
}

/// Builds a `CosmosResponse` with the given body bytes and optional
/// server continuation header. The body bytes act as a per-page "marker"
/// the test can collect later to verify exactly-once delivery.
fn page_response(body: &[u8], continuation: Option<&str>) -> CosmosResponse {
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    diagnostics.set_operation_status(azure_core::http::StatusCode::Ok, None);
    let mut headers = CosmosResponseHeaders::new();
    headers.continuation = continuation.map(str::to_owned);
    CosmosResponse::new(
        body.to_vec(),
        headers,
        CosmosStatus::new(azure_core::http::StatusCode::Ok),
        Arc::new(diagnostics.complete()),
    )
}

/// Drives a pipeline to completion, collecting every page body and the
/// continuation token issued for each subsequent request.
async fn drain_all(pipeline: &mut Pipeline, executor: &mut MockRequestExecutor) -> Vec<Vec<u8>> {
    let mut pages = Vec::new();
    let mut topology = super::mocks::NoopTopologyProvider;
    loop {
        let mut context = PipelineContext::new(executor, Some(&mut topology));
        match pipeline.next_page(&mut context).await.unwrap() {
            Some(response) => pages.push(response.body_bytes().to_vec()),
            None => break,
        }
    }
    pages
}

/// Drives a pipeline through exactly `n` pages and returns the bodies plus
/// the snapshot state captured after the n-th page.
async fn drain_pages(
    pipeline: &mut Pipeline,
    executor: &mut MockRequestExecutor,
    n: usize,
) -> Vec<Vec<u8>> {
    let mut pages = Vec::with_capacity(n);
    let mut topology = super::mocks::NoopTopologyProvider;
    for _ in 0..n {
        let mut context = PipelineContext::new(executor, Some(&mut topology));
        let response = pipeline
            .next_page(&mut context)
            .await
            .unwrap()
            .expect("expected page, not drained");
        pages.push(response.body_bytes().to_vec());
    }
    pages
}

/// Round-trips a `PipelineNodeState` through the on-wire continuation
/// token (base64 + JSON), then back to a `PipelineNodeState` — the same
/// path a real caller takes between `to_continuation_token()` and
/// `plan_operation(resume = ...)`.
fn round_trip_state(state: PipelineNodeState, op: &CosmosOperation) -> PipelineNodeState {
    let token = ContinuationToken::encode_v1(op, &state).expect("encode succeeds");
    let resolved = token.resolve().expect("decode succeeds");
    match resolved {
        ResolvedToken::ClientV1(token_state) => {
            token_state
                .is_valid_for_operation(op)
                .expect("operation compatible");
            token_state.into_root_node_state()
        }
        ResolvedToken::ServerOpaque(_) => panic!("expected ClientV1 token"),
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

/// Baseline: a single-partition query that pages once, serializes, resumes,
/// and drains. No topology change. Sanity-checks the end-to-end round-trip
/// before the more interesting split scenarios.
#[tokio::test]
async fn single_partition_resume_roundtrips_cleanly() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // Session 1: build, drain page 1 (returns continuation "ct-1"), snapshot.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(page_response(b"page-1", Some("ct-1")))]);

    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages1, vec![b"page-1".to_vec()]);
    assert_eq!(executor1.continuation_calls, vec![None]);
    let state = pipeline1.snapshot_state();
    drop(pipeline1);

    // Session 2: resume, drain page 2 + drained, no further continuation.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(page_response(b"page-2", None))]);

    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_state))
        .await
        .unwrap();
    let pages2 = drain_all(&mut pipeline2, &mut executor2).await;
    assert_eq!(pages2, vec![b"page-2".to_vec()]);
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("ct-1".to_owned())],
        "page 2 must be requested with the continuation page 1 returned",
    );
}

/// Defect A regression guard at the end-to-end level. Session 1 sees a
/// single-leaf topology and returns a continuation. Between sessions the
/// partition splits into two children. Session 2 must forward the saved
/// continuation to BOTH children — otherwise the second child fresh-starts
/// and re-emits whatever the first child already returned.
#[tokio::test]
async fn resume_after_split_forwards_continuation_to_every_surviving_leaf() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // Session 1: one partition spans the full range. Page 1 carries 5 items;
    // continuation "ct-pre-split" marks the un-drained tail.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(page_response(
        b"page-1-presplit",
        Some("ct-pre-split"),
    ))]);

    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    let state = pipeline1.snapshot_state();
    drop(pipeline1);

    // Session 2: the topology has split into [, 80) + [80, FF). The saved
    // continuation must be forwarded to BOTH children. We model each
    // child's response as a single drained page so the test can confirm
    // both leaves issued requests bearing "ct-pre-split".
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(page_response(b"page-left", None)),
        Ok(page_response(b"page-right", None)),
    ]);

    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_state))
        .await
        .unwrap();
    let pages2 = drain_all(&mut pipeline2, &mut executor2).await;

    // Every page exactly once, in EPK order.
    assert_eq!(pages1, vec![b"page-1-presplit".to_vec()]);
    assert_eq!(pages2, vec![b"page-left".to_vec(), b"page-right".to_vec()]);

    // The regression bug: the second leaf used to be called with `None`
    // (fresh start) instead of the saved continuation, which is what
    // produced duplicate items on a real account.
    assert_eq!(
        executor2.continuation_calls,
        vec![
            Some("ct-pre-split".to_owned()),
            Some("ct-pre-split".to_owned()),
        ],
        "both post-split leaves must resume with the saved continuation",
    );
}

/// Defect B regression guard at the end-to-end level — the canonical bug
/// from the original user repro. A snapshot taken mid-fan-out (front
/// sibling has progressed; later siblings still owe their pre-split
/// continuation) must preserve EVERY pending child's state, not just the
/// front. With the lossy snapshot, the later siblings' continuations were
/// silently dropped and they fresh-started on resume, producing
/// duplicates.
#[tokio::test]
async fn resume_mid_fanout_preserves_every_sibling_state() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // Session 1: post-split topology with two siblings. Both leaves start
    // with no continuation (fresh fan-out). Page 1 drains the LEFT sibling
    // partially, returning a left-scoped continuation "ct-left". The
    // RIGHT sibling is never touched yet — it still owes its original
    // (fresh, `None`) start token.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(page_response(b"left-page-1", Some("ct-left")))]);

    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages1, vec![b"left-page-1".to_vec()]);
    assert_eq!(executor1.continuation_calls, vec![None]);
    let state = pipeline1.snapshot_state();
    drop(pipeline1);

    // The snapshot must carry BOTH siblings, not just the front. Inspect
    // before round-tripping so the failure mode is obvious.
    match &state {
        PipelineNodeState::SequentialDrain { children } => {
            assert_eq!(
                children.len(),
                2,
                "snapshot must preserve both siblings; got {:?}",
                children,
            );
            assert_eq!(children[0].min_epk, "");
            assert_eq!(children[0].max_epk, "80");
            assert!(matches!(
                children[0].state,
                PipelineNodeState::Request {
                    server_continuation: Some(ref c),
                } if c == "ct-left"
            ));
            assert_eq!(children[1].min_epk, "80");
            assert_eq!(children[1].max_epk, "FF");
            assert!(matches!(
                children[1].state,
                PipelineNodeState::Request {
                    server_continuation: None,
                }
            ));
        }
        other => panic!("expected SequentialDrain snapshot, got {other:?}"),
    }

    // Session 2: resume with the round-tripped token. Same topology
    // (no split between sessions). The left sibling must resume with
    // "ct-left"; the right sibling must start fresh (None) — and crucially
    // must NOT be skipped, which is exactly what the lossy snapshot used
    // to do.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(page_response(b"left-page-2", None)),
        Ok(page_response(b"right-page-1", None)),
    ]);

    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_state))
        .await
        .unwrap();
    let pages2 = drain_all(&mut pipeline2, &mut executor2).await;
    assert_eq!(
        pages2,
        vec![b"left-page-2".to_vec(), b"right-page-1".to_vec()],
        "resume must drain the rest of left (using ct-left) THEN the untouched right sibling",
    );

    assert_eq!(
        executor2.continuation_calls,
        vec![Some("ct-left".to_owned()), None],
        "left resumes with ct-left; right resumes fresh (its un-started pre-split state)",
    );

    // Cross-page no-duplicates / no-losses check (the user-visible
    // symptom). The full set of page markers across both sessions must
    // appear exactly once.
    let mut all_pages: Vec<Vec<u8>> = pages1.into_iter().chain(pages2.into_iter()).collect();
    all_pages.sort();
    let mut expected: Vec<Vec<u8>> = vec![
        b"left-page-1".to_vec(),
        b"left-page-2".to_vec(),
        b"right-page-1".to_vec(),
    ];
    expected.sort();
    assert_eq!(all_pages, expected);
}

/// Combined defect-A + defect-B scenario: snapshot mid-fan-out AND the
/// front sibling splits between sessions. The right sibling's un-started
/// state must survive the snapshot (Defect B), and the left sibling's
/// saved continuation must fan out to BOTH of its post-split children
/// (Defect A).
#[tokio::test]
async fn resume_mid_fanout_then_split_preserves_state_and_fans_out_continuation() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // Session 1: post-split topology with two siblings. Page 1 returns a
    // left-scoped continuation; right is never touched.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(page_response(b"left-page-1", Some("ct-left")))]);

    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    let state = pipeline1.snapshot_state();
    drop(pipeline1);

    // Session 2: the LEFT sibling has now split into [, 40) + [40, 80).
    // The right sibling is unchanged. Defect A: BOTH halves of the
    // post-split left sibling must inherit "ct-left". Defect B: right
    // sibling must still be visited fresh.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "40", "pk-left-l"),
        resolved("40", "80", "pk-left-r"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(page_response(b"left-l-page-1", None)),
        Ok(page_response(b"left-r-page-1", None)),
        Ok(page_response(b"right-page-1", None)),
    ]);

    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_state))
        .await
        .unwrap();
    let pages2 = drain_all(&mut pipeline2, &mut executor2).await;

    assert_eq!(
        pages2,
        vec![
            b"left-l-page-1".to_vec(),
            b"left-r-page-1".to_vec(),
            b"right-page-1".to_vec(),
        ],
    );

    // Defect A: ct-left flows to both halves of the split left sibling.
    // Defect B: right sibling's saved None state survives the round-trip.
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("ct-left".to_owned()), Some("ct-left".to_owned()), None,],
    );

    // Aggregate no-duplicate / no-loss assertion.
    let mut all_pages: Vec<Vec<u8>> = pages1.into_iter().chain(pages2.into_iter()).collect();
    all_pages.sort();
    let mut expected: Vec<Vec<u8>> = vec![
        b"left-page-1".to_vec(),
        b"left-l-page-1".to_vec(),
        b"left-r-page-1".to_vec(),
        b"right-page-1".to_vec(),
    ];
    expected.sort();
    assert_eq!(all_pages, expected);
}

/// Snapshot of a fully-drained left sibling MUST mark it `Drained` so the
/// resume does not re-query its scope. Even if the topology later changes
/// in that scope, the saved-children ledger is authoritative remaining
/// work — drained scope stays drained.
#[tokio::test]
async fn resume_does_not_requery_already_drained_sibling_scope() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // Construct a snapshot directly: left sibling is fully drained,
    // right sibling still owes its work with continuation "ct-right".
    // This is the shape the public path produces after the left
    // sibling's last page emits no continuation.
    let saved_state = PipelineNodeState::SequentialDrain {
        children: vec![
            RangedChildState {
                min_epk: String::new(),
                max_epk: "80".to_string(),
                state: PipelineNodeState::Drained,
            },
            RangedChildState {
                min_epk: "80".to_string(),
                max_epk: "FF".to_string(),
                state: PipelineNodeState::Request {
                    server_continuation: Some("ct-right".to_owned()),
                },
            },
        ],
    };

    // Resume after a split that ALSO affected the drained left scope.
    // The planner must still NOT issue any request against [, 40) or
    // [40, 80) — that scope is drained per the saved ledger, even
    // though the topology has changed within it.
    let resumed_state = round_trip_state(saved_state, &op);
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "40", "pk-left-l"),
        resolved("40", "80", "pk-left-r"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor = MockRequestExecutor::new(vec![Ok(page_response(b"right-page-1", None))]);

    let mut pipeline = build_sequential_drain(&plan, &mut topology, &op, Some(resumed_state))
        .await
        .unwrap();
    let pages = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(pages, vec![b"right-page-1".to_vec()]);
    assert_eq!(
        executor.continuation_calls,
        vec![Some("ct-right".to_owned())],
        "drained left scope must not be re-queried; only the right sibling executes",
    );
}

/// A saved range that the current topology cannot fully cover must fail
/// the resume rather than silently dropping work — that silent-drop
/// behavior is exactly the class of bug this PR closes. This guards the
/// `CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED` error path at the
/// integration level.
#[tokio::test]
async fn resume_fails_loudly_when_saved_range_cannot_be_covered() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    let saved_state = PipelineNodeState::SequentialDrain {
        children: vec![RangedChildState {
            min_epk: "55".to_string(),
            max_epk: "AA".to_string(),
            state: PipelineNodeState::Request {
                server_continuation: Some("ct-orphan".to_owned()),
            },
        }],
    };
    let resumed_state = round_trip_state(saved_state, &op);

    // Topology only covers part of the saved range — [, 70) + [70, 80).
    // The saved range [55, AA) extends past 80 with no leaf to honor it.
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "70", "pk-a"),
        resolved("70", "80", "pk-b"),
    ])]);

    let err: Result<Pipeline> =
        build_sequential_drain(&plan, &mut topology, &op, Some(resumed_state)).await;
    let err = err.expect_err("expected unhonored-saved-range error");
    let rendered = err.to_string();
    assert!(
        rendered.contains("saved") || rendered.contains("unhonored") || rendered.contains("cover"),
        "error message should describe the unhonored-saved-range failure; got: {rendered}"
    );
}

/// End-to-end deterministic equivalent of the live multi-PK / single-item
/// repro: three sessions, two serialize → resume round-trips, with the
/// partition split landing between session 1 and session 2.
///
/// This is the only test that drives the exact loop the live repro takes:
/// a single-leaf pre-split snapshot is decoded against a post-split
/// topology AND that fanned-out state is snapshotted AGAIN mid-fan-out.
/// Both defects must be fixed for the back sibling to carry its
/// pre-split token through to the round that actually queries it.
///
/// With Defect 1 alone fixed, the back sibling enters the in-memory
/// pipeline with `T1` at session 2 — but session 2's snapshot drops it,
/// so by session 3 it has fresh-started.
///
/// With Defect 2 alone fixed, the snapshot preserves whatever was in
/// memory — but Defect 1 means what was in memory is `None`, so the
/// back sibling still fresh-starts at session 3.
///
/// Only with both fixed does the back sibling reach the executor at
/// session 3 carrying the original `T1` — which is what suppresses the
/// duplicates against a real server.
#[tokio::test]
async fn three_session_loop_propagates_presplit_token_through_two_snapshots() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // ── Session 1 ──────────────────────────────────────────────────────
    // Pre-split: a single physical partition covers [, FF). Page 1
    // returns a server continuation T1 that conceptually covers the
    // whole range.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-pre")])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(page_response(b"page-1-pre", Some("T1")))]);
    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages_s1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages_s1, vec![b"page-1-pre".to_vec()]);
    assert_eq!(executor1.continuation_calls, vec![None]);

    let state_s1 = pipeline1.snapshot_state();
    drop(pipeline1);

    // The session-1 snapshot must carry exactly one child entry covering
    // [, FF) with Request{T1}. This is what the public wire token then
    // encodes.
    match &state_s1 {
        PipelineNodeState::SequentialDrain { children } => {
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].min_epk, "");
            assert_eq!(children[0].max_epk, "FF");
            assert!(matches!(
                children[0].state,
                PipelineNodeState::Request {
                    server_continuation: Some(ref c),
                } if c == "T1"
            ));
        }
        other => panic!("expected SequentialDrain snapshot at session 1, got {other:?}"),
    }

    // ── Session 2 ──────────────────────────────────────────────────────
    // Between sessions, the partition split into [, 80) + [80, FF).
    // Defect 1 territory: the planner must fan T1 out to BOTH children
    // when decoding the session-1 token. We drain ONE page (the front
    // child progresses to T2_a; the back child is still untouched and
    // must remain Request{T1} in memory). Then we snapshot AGAIN —
    // Defect 2 territory: that snapshot must carry both children, with
    // the back child still owing T1.
    let resumed_s2 = round_trip_state(state_s1, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(page_response(
        b"page-1-postsplit-left",
        Some("T2_a"),
    ))]);
    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_s2))
        .await
        .unwrap();
    let pages_s2 = drain_pages(&mut pipeline2, &mut executor2, 1).await;
    assert_eq!(pages_s2, vec![b"page-1-postsplit-left".to_vec()]);
    // The single request issued in session 2 went to the LEFT child
    // (front) carrying T1 — the visible Defect-1 evidence.
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("T1".to_owned())],
        "session 2's first executor call must be the front child carrying the pre-split token",
    );

    let state_s2 = pipeline2.snapshot_state();
    drop(pipeline2);

    // Session-2 snapshot inspection — the canonical Defect-2 surface.
    // Front child progressed to T2_a; back child must STILL carry T1.
    // Before the fix this entry was silently dropped and the back child
    // fresh-started at session 3 producing duplicates.
    match &state_s2 {
        PipelineNodeState::SequentialDrain { children } => {
            assert_eq!(
                children.len(),
                2,
                "session 2 snapshot must preserve both post-split children, not just the front; got {children:?}",
            );
            assert_eq!(children[0].min_epk, "");
            assert_eq!(children[0].max_epk, "80");
            assert!(matches!(
                children[0].state,
                PipelineNodeState::Request {
                    server_continuation: Some(ref c),
                } if c == "T2_a"
            ));
            assert_eq!(children[1].min_epk, "80");
            assert_eq!(children[1].max_epk, "FF");
            assert!(
                matches!(
                    children[1].state,
                    PipelineNodeState::Request {
                        server_continuation: Some(ref c),
                    } if c == "T1"
                ),
                "back child must still owe pre-split T1, not None or T2_a; got {:?}",
                children[1].state,
            );
        }
        other => panic!("expected SequentialDrain snapshot at session 2, got {other:?}"),
    }

    // ── Session 3 ──────────────────────────────────────────────────────
    // Resume from the session-2 token. Topology unchanged (no further
    // splits). The front child drains in one more page (no continuation
    // returned). Then the planner must visit the back child carrying
    // T1 — NOT None — otherwise the server would re-return everything
    // T1 was already past, which is exactly the duplicate-emission bug.
    let resumed_s3 = round_trip_state(state_s2, &op);
    let mut topology3 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor3 = MockRequestExecutor::new(vec![
        Ok(page_response(b"page-2-postsplit-left", None)),
        Ok(page_response(b"page-1-postsplit-right", None)),
    ]);
    let mut pipeline3 = build_sequential_drain(&plan, &mut topology3, &op, Some(resumed_s3))
        .await
        .unwrap();
    let pages_s3 = drain_all(&mut pipeline3, &mut executor3).await;
    assert_eq!(
        pages_s3,
        vec![
            b"page-2-postsplit-left".to_vec(),
            b"page-1-postsplit-right".to_vec(),
        ],
    );

    // The whole reason for this test: the back child's request at
    // session 3 must carry T1. With either defect unfixed, this value
    // is None and the live test sees 55/50.
    assert_eq!(
        executor3.continuation_calls,
        vec![Some("T2_a".to_owned()), Some("T1".to_owned())],
        "session 3 must drain front from T2_a then visit back with the preserved pre-split T1",
    );

    // Aggregate no-duplicate / no-loss assertion across all three
    // sessions — the user-visible symptom.
    let mut all_pages: Vec<Vec<u8>> = pages_s1
        .into_iter()
        .chain(pages_s2.into_iter())
        .chain(pages_s3.into_iter())
        .collect();
    all_pages.sort();
    let mut expected: Vec<Vec<u8>> = vec![
        b"page-1-pre".to_vec(),
        b"page-1-postsplit-left".to_vec(),
        b"page-2-postsplit-left".to_vec(),
        b"page-1-postsplit-right".to_vec(),
    ];
    expected.sort();
    assert_eq!(all_pages, expected);
}

/// O10: cascading splits — the back sibling from a first split itself
/// splits again before the user gets back to it. The new planner's
/// interval-join shape should clone the saved back-sibling token to each
/// of the grand-child leaves the back range is now resolved to. This
/// guards the "split-of-a-split" path that the basic post-split tests
/// don't reach, and is the highest-likely-to-regress hole around the
/// per-range list shape.
#[tokio::test]
async fn cascading_split_propagates_back_sibling_token_to_every_grand_child() {
    let op = cross_partition_query_operation();
    let plan = full_range_plan();

    // ── Session 1: pre-split, single physical partition. ─────────────
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-pre")])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(page_response(b"page-1-pre", Some("T1")))]);
    let mut pipeline1 = build_sequential_drain(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let pages_s1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages_s1, vec![b"page-1-pre".to_vec()]);

    let state_s1 = pipeline1.snapshot_state();
    drop(pipeline1);

    // ── Session 2: first split → [, 80) + [80, FF). Drain the front
    // child to completion so only the back child remains pending with
    // T1. Snapshot again.
    let resumed_s2 = round_trip_state(state_s1, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 =
        MockRequestExecutor::new(vec![Ok(page_response(b"page-1-postsplit-left", None))]);
    let mut pipeline2 = build_sequential_drain(&plan, &mut topology2, &op, Some(resumed_s2))
        .await
        .unwrap();
    let pages_s2 = drain_pages(&mut pipeline2, &mut executor2, 1).await;
    assert_eq!(pages_s2, vec![b"page-1-postsplit-left".to_vec()]);

    let state_s2 = pipeline2.snapshot_state();
    drop(pipeline2);

    // Sanity-check: snapshot now has just the back child owing T1
    // (front child drained; popped off the queue).
    match &state_s2 {
        PipelineNodeState::SequentialDrain { children } => {
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].min_epk, "80");
            assert_eq!(children[0].max_epk, "FF");
            assert!(matches!(
                children[0].state,
                PipelineNodeState::Request {
                    server_continuation: Some(ref c),
                } if c == "T1"
            ));
        }
        other => panic!("expected SequentialDrain snapshot at session 2, got {other:?}"),
    }

    // ── Session 3: cascading split — the back range [80, FF) has
    // itself split into [80, C0) + [C0, FF). The planner must clone T1
    // to BOTH grand-children. Drain both to completion.
    let resumed_s3 = round_trip_state(state_s2, &op);
    let mut topology3 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("80", "C0", "pk-back-left"),
        resolved("C0", "FF", "pk-back-right"),
    ])]);
    let mut executor3 = MockRequestExecutor::new(vec![
        Ok(page_response(b"page-1-back-left", None)),
        Ok(page_response(b"page-1-back-right", None)),
    ]);
    let mut pipeline3 = build_sequential_drain(&plan, &mut topology3, &op, Some(resumed_s3))
        .await
        .unwrap();
    let pages_s3 = drain_all(&mut pipeline3, &mut executor3).await;
    assert_eq!(
        pages_s3,
        vec![b"page-1-back-left".to_vec(), b"page-1-back-right".to_vec()],
    );

    // BOTH grand-children must have been queried with T1 — neither
    // None. If the planner accidentally moved the token to the first
    // grand-child only (the Defect-1 shape, but at the resume-decode
    // layer instead of the in-memory fan-out), the second grand-child
    // would carry None and re-emit the full [C0, FF) post-split data
    // that T1 was already past.
    assert_eq!(
        executor3.continuation_calls,
        vec![Some("T1".to_owned()), Some("T1".to_owned())],
        "both back-range grand-children must receive the preserved pre-split T1",
    );

    let mut all_pages: Vec<Vec<u8>> = pages_s1
        .into_iter()
        .chain(pages_s2.into_iter())
        .chain(pages_s3.into_iter())
        .collect();
    all_pages.sort();
    let mut expected: Vec<Vec<u8>> = vec![
        b"page-1-pre".to_vec(),
        b"page-1-postsplit-left".to_vec(),
        b"page-1-back-left".to_vec(),
        b"page-1-back-right".to_vec(),
    ];
    expected.sort();
    assert_eq!(all_pages, expected);
}
