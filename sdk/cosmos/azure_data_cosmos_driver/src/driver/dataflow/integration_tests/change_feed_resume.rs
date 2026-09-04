// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for change feed resume across a topology
//! change (partition merge).
//!
//! These tests compose the real planner ([`build_unordered_merge`]), request,
//! snapshot, and continuation-token layers together against the
//! `MockRequestExecutor` / `MockTopologyProvider` from `dataflow::mocks`, and
//! exercise the same serialize -> resume round-trip the public change feed
//! iterator surfaces to callers. No live account or emulator is required.
//!
//! They are the deterministic complement to the live
//! `cosmos_change_feed_split` test in the `azure_data_cosmos` crate: that test
//! forces a real split on a real account, while these reproduce the topology
//! change in-memory so the resume bookkeeping is covered on every CI run.
//!
//! Change feed differs from a cross-partition query in two ways these tests
//! must model:
//!
//! 1. The continuation token is carried by the **ETag** response header
//!    (re-sent as `If-None-Match` on the next poll), not `x-ms-continuation`.
//! 2. The stream is **infinite** — a partition never transitions to `Drained`,
//!    so the tests drive an exact number of pages rather than draining to the
//!    end.

use std::sync::Arc;

use azure_core::http::{Etag, StatusCode};
use futures::future::BoxFuture;

use super::super::{
    mocks::{MockRequestExecutor, MockTopologyProvider, NoopTopologyProvider},
    planner::build_unordered_merge,
    PartitionRoutingRefresh, Pipeline, PipelineContext, PipelineNodeState, RequestExecutor,
    RequestTarget, ResolvedRange,
};
use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{
        effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
        ChangeFeedStartFrom, ContainerProperties, ContainerReference, ContinuationToken,
        CosmosOperation, CosmosResponse, CosmosResponseHeaders, CosmosStatus, FeedRange,
        Precondition, ResolvedToken, SystemProperties,
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

/// Builds a change feed operation over the whole container, optionally
/// carrying an explicit start position. `None` means "from the beginning".
fn change_feed_operation(start_from: Option<ChangeFeedStartFrom>) -> Arc<CosmosOperation> {
    let mut op = CosmosOperation::change_feed(test_container(), Some(FeedRange::full()));
    if let Some(marker) = start_from {
        op = op.with_change_feed_start(marker);
    }
    Arc::new(op)
}

/// Builds an AllVersionsAndDeletes (full-fidelity) change feed operation over
/// the whole container, optionally carrying an explicit start position.
fn avad_change_feed_operation(start_from: Option<ChangeFeedStartFrom>) -> Arc<CosmosOperation> {
    let mut op = CosmosOperation::change_feed_all_versions_and_deletes(
        test_container(),
        Some(FeedRange::full()),
    );
    if let Some(marker) = start_from {
        op = op.with_change_feed_start(marker);
    }
    Arc::new(op)
}

fn resolved(min: &str, max: &str, pk_range_id: &str) -> ResolvedRange {
    ResolvedRange {
        partition_key_range_id: pk_range_id.to_string(),
        range: fr(min, max),
    }
}

/// Builds a [`FeedRange`] from raw EPK bounds (`""` is MIN, `"FF"` is MAX).
fn fr(min: &str, max: &str) -> FeedRange {
    FeedRange::new(
        EffectivePartitionKey::from(min),
        EffectivePartitionKey::from(max),
    )
    .unwrap()
}

/// Builds a change feed `CosmosResponse` carrying its continuation in the
/// **ETag** header (the change feed wire contract), plus a body the test can
/// collect to verify which pages were emitted.
fn cf_page(body: &[u8], etag: &str) -> CosmosResponse {
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    diagnostics.set_operation_status(StatusCode::Ok, None);
    let mut headers = CosmosResponseHeaders::new();
    headers.etag = Some(Etag::from(etag.to_owned()));
    CosmosResponse::new(
        body.to_vec(),
        headers,
        CosmosStatus::new(StatusCode::Ok),
        Arc::new(diagnostics.complete()),
    )
}

struct StartRecordingExecutor {
    inner: MockRequestExecutor,
    start_calls: Vec<Option<ChangeFeedStartFrom>>,
    precondition_calls: Vec<Option<Precondition>>,
}

impl StartRecordingExecutor {
    fn new(responses: Vec<crate::error::Result<CosmosResponse>>) -> Self {
        Self {
            inner: MockRequestExecutor::new(responses),
            start_calls: Vec::new(),
            precondition_calls: Vec::new(),
        }
    }
}

impl RequestExecutor for StartRecordingExecutor {
    fn execute_request<'a>(
        &'a mut self,
        operation: &'a CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> BoxFuture<'a, crate::error::Result<CosmosResponse>> {
        self.start_calls
            .push(operation.change_feed_start().cloned());
        self.precondition_calls
            .push(operation.precondition().cloned());
        self.inner
            .execute_request(operation, target, partition_routing_refresh, continuation)
    }
}

/// Drives a pipeline through exactly `n` pages and returns the bodies. Change
/// feed never drains, so a bounded count is the only sensible stop condition.
async fn drain_pages(
    pipeline: &mut Pipeline,
    executor: &mut dyn RequestExecutor,
    n: usize,
) -> Vec<Vec<u8>> {
    let mut pages = Vec::with_capacity(n);
    let mut topology = NoopTopologyProvider;
    for _ in 0..n {
        let mut context = PipelineContext::new(executor, Some(&mut topology));
        let response = pipeline
            .next_page(&mut context)
            .await
            .unwrap()
            .expect("change feed page, not drained");
        pages.push(response.body_bytes().to_vec());
    }
    pages
}

/// Round-trips a `PipelineNodeState` through the on-wire continuation token
/// (base64 + JSON) and back — the same path a real caller takes between
/// `to_continuation_token()` and resuming the feed.
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

fn assert_unordered_snapshot(
    state: &PipelineNodeState,
    expected_start: Option<ChangeFeedStartFrom>,
    expected_tokens: &[(&str, &str, &str)],
) {
    let PipelineNodeState::UnorderedMerge {
        active_tokens,
        start_from,
    } = state
    else {
        panic!("expected UnorderedMerge snapshot, got {state:?}");
    };
    assert_eq!(start_from, &expected_start);
    assert_eq!(active_tokens.len(), expected_tokens.len());
    for (token, (min, max, etag)) in active_tokens.iter().zip(expected_tokens) {
        assert_eq!(&token.min_epk, min);
        assert_eq!(&token.max_epk, max);
        assert_eq!(&token.server_continuation, etag);
    }
}

fn assert_merged_parent_targets(executor: &MockRequestExecutor) {
    let merged = fr("", "FF");
    assert_eq!(
        executor.target_calls,
        vec![
            RequestTarget::effective_partition_key_range(
                fr("", "80"),
                "pk-merged".to_owned(),
                merged.clone(),
            ),
            RequestTarget::effective_partition_key_range(
                fr("80", "FF"),
                "pk-merged".to_owned(),
                merged,
            ),
        ],
        "the merged physical range must retain both parent EPK slices",
    );
}

fn assert_now_resume_inputs(executor: &StartRecordingExecutor) {
    assert_eq!(
        executor.start_calls,
        vec![
            Some(ChangeFeedStartFrom::Now),
            Some(ChangeFeedStartFrom::Now)
        ],
    );
    assert_eq!(
        executor.precondition_calls,
        vec![
            Some(Precondition::if_none_match(Etag::from("*"))),
            Some(Precondition::if_none_match(Etag::from("*"))),
        ],
    );
}

// ── Tests ────────────────────────────────────────────────────────────────────

/// Baseline: a single-partition change feed that polls once, serializes,
/// resumes, and polls again. No topology change. Sanity-checks the end-to-end
/// round-trip — including that the ETag continuation and the `start_from`
/// marker both survive serialize -> resume — before the merge scenario.
#[tokio::test]
async fn single_partition_change_feed_resume_roundtrips() {
    let op = change_feed_operation(Some(ChangeFeedStartFrom::Now));

    // Session 1: one partition spans the full range. Poll once; the page
    // carries the next continuation in its ETag ("lsn-1").
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(cf_page(b"page-1", "lsn-1"))]);

    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages1, vec![b"page-1".to_vec()]);
    assert_eq!(
        executor1.continuation_calls,
        vec![None],
        "the first poll starts fresh (no If-None-Match continuation)",
    );

    let state = pipeline1.snapshot_state().unwrap();
    drop(pipeline1);

    // The snapshot must record the single partition's outstanding ETag and
    // re-persist the original start position so a never-polled partition
    // would honor it on resume.
    match &state {
        PipelineNodeState::UnorderedMerge {
            active_tokens,
            start_from,
        } => {
            assert_eq!(active_tokens.len(), 1, "got {active_tokens:?}");
            assert_eq!(active_tokens[0].min_epk, "");
            assert_eq!(active_tokens[0].max_epk, "FF");
            assert_eq!(active_tokens[0].server_continuation, "lsn-1");
            assert_eq!(*start_from, Some(ChangeFeedStartFrom::Now));
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }

    // Session 2: resume from the round-tripped token, same topology. The poll
    // must carry the saved ETag as its continuation — not restart the feed.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(cf_page(b"page-2", "lsn-2"))]);

    let mut pipeline2 =
        build_unordered_merge(&FeedRange::full(), &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let pages2 = drain_pages(&mut pipeline2, &mut executor2, 1).await;
    assert_eq!(pages2, vec![b"page-2".to_vec()]);
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("lsn-1".to_owned())],
        "resume must re-send the saved ETag, not poll from the start",
    );
}

/// End-to-end guard for change feed resume across a partition **merge**.
///
/// Session 1 polls two adjacent partitions, each returning its own ETag.
/// Between sessions the partitions merge into one wider range. On resume the
/// merged partition must keep reading **each** parent's progress — dropping
/// either saved continuation would skip (under a `Now`/point-in-time start) or
/// re-read that parent's tail.
///
/// `build_unordered_merge` rebuilds the merged range as one EPK-scoped leaf per
/// saved parent sub-range: `[, 80)` resumes from `lsn-left` and `[80, FF)` from
/// `lsn-right`, each carrying explicit `x-ms-start/end-epk` bounds. This matches
/// the per-EPK-range change feed resume used by the other Cosmos SDKs (.NET,
/// Java, Python), where a merge keeps the finer sub-ranges and their tokens
/// rather than collapsing to a single parent continuation.
#[tokio::test]
async fn change_feed_resume_across_merge_reads_each_parent_subrange() {
    // Read from the beginning (no explicit start marker) so the test isolates
    // the continuation-forwarding behavior across the merge.
    let op = change_feed_operation(None);

    // Session 1: two adjacent partitions [, 80) and [80, FF). Round-robin
    // polling visits left then right; each returns its own next-ETag.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"left-1", "lsn-left")),
        Ok(cf_page(b"right-1", "lsn-right")),
    ]);

    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 2).await;
    assert_eq!(pages1, vec![b"left-1".to_vec(), b"right-1".to_vec()]);
    assert_eq!(executor1.continuation_calls, vec![None, None]);

    // The snapshot records one outstanding ETag per parent partition.
    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::UnorderedMerge { active_tokens, .. } => {
            assert_eq!(active_tokens.len(), 2, "got {active_tokens:?}");
            assert_eq!(active_tokens[0].max_epk, "80");
            assert_eq!(active_tokens[0].server_continuation, "lsn-left");
            assert_eq!(active_tokens[1].min_epk, "80");
            assert_eq!(active_tokens[1].server_continuation, "lsn-right");
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    // Session 2: the two partitions have MERGED into one range [, FF).
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-merged")])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"merged-left-1", "lsn-left-2")),
        Ok(cf_page(b"merged-right-1", "lsn-right-2")),
    ]);

    let mut pipeline2 =
        build_unordered_merge(&FeedRange::full(), &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let pages2 = drain_pages(&mut pipeline2, &mut executor2, 2).await;
    assert_eq!(
        pages2,
        vec![b"merged-left-1".to_vec(), b"merged-right-1".to_vec()]
    );

    // The merged range rebuilds as two EPK-scoped leaves, each resuming from its
    // own saved parent ETag — no saved continuation is dropped.
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("lsn-left".to_owned()), Some("lsn-right".to_owned())],
        "merge must read each saved parent sub-range from its own ETag",
    );

    // Each leaf is scoped to its parent's sub-range within the merged physical
    // partition, so the wire layer emits `x-ms-start/end-epk` for both.
    assert_merged_parent_targets(&executor2);
}

#[tokio::test]
async fn point_in_time_merge_resume_keeps_marker_and_parent_slices() {
    let marker = ChangeFeedStartFrom::PointInTime(time::macros::datetime!(
        2026-08-21 12:34:56 UTC
    ));
    let op = change_feed_operation(Some(marker.clone()));
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"left-1", "pit-left")),
        Ok(cf_page(b"right-1", "pit-right")),
    ]);
    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    assert_eq!(
        drain_pages(&mut pipeline1, &mut executor1, 2).await,
        vec![b"left-1".to_vec(), b"right-1".to_vec()]
    );
    let state = pipeline1.snapshot_state().unwrap();
    assert_unordered_snapshot(
        &state,
        Some(marker),
        &[("", "80", "pit-left"), ("80", "FF", "pit-right")],
    );

    let resume_op = change_feed_operation(None);
    let resumed = round_trip_state(state, &resume_op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-merged")])]);
    let mut executor2 = StartRecordingExecutor::new(vec![
        Ok(cf_page(b"merged-left", "pit-left-2")),
        Ok(cf_page(b"merged-right", "pit-right-2")),
    ]);
    let mut pipeline2 = build_unordered_merge(
        &FeedRange::full(),
        &mut topology2,
        &resume_op,
        Some(resumed),
    )
    .await
    .unwrap();
    drain_pages(&mut pipeline2, &mut executor2, 2).await;
    assert_eq!(
        executor2.inner.continuation_calls,
        vec![Some("pit-left".to_owned()), Some("pit-right".to_owned())],
    );
    assert_eq!(
        executor2.start_calls,
        vec![
            Some(ChangeFeedStartFrom::PointInTime(
                time::macros::datetime!(2026-08-21 12:34:56 UTC)
            ));
            2
        ],
    );
    assert_merged_parent_targets(&executor2.inner);
}

#[tokio::test]
async fn latest_version_now_merge_resume_reapplies_now_to_unsaved_slice() {
    let op = change_feed_operation(Some(ChangeFeedStartFrom::Now));
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(cf_page(b"left-1", "now-left"))]);
    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    drain_pages(&mut pipeline1, &mut executor1, 1).await;
    let state = pipeline1.snapshot_state().unwrap();
    assert_unordered_snapshot(
        &state,
        Some(ChangeFeedStartFrom::Now),
        &[("", "80", "now-left")],
    );

    let resume_op = change_feed_operation(None);
    let resumed = round_trip_state(state, &resume_op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-merged")])]);
    let mut executor2 = StartRecordingExecutor::new(vec![
        Ok(cf_page(b"merged-left", "now-left-2")),
        Ok(cf_page(b"merged-right", "now-right-1")),
    ]);
    let mut pipeline2 = build_unordered_merge(
        &FeedRange::full(),
        &mut topology2,
        &resume_op,
        Some(resumed),
    )
    .await
    .unwrap();
    drain_pages(&mut pipeline2, &mut executor2, 2).await;
    assert_eq!(
        executor2.inner.continuation_calls,
        vec![Some("now-left".to_owned()), None],
        "the saved slice uses its ETag; the unsaved slice re-applies persisted Now",
    );
    // These are RequestExecutor inputs; transport tests cover final header precedence.
    assert_now_resume_inputs(&executor2);
    assert_merged_parent_targets(&executor2.inner);
}

#[tokio::test]
async fn avad_now_merge_resume_keeps_both_primed_parent_etags() {
    let op = avad_change_feed_operation(Some(ChangeFeedStartFrom::Now));
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"", "avad-left")),
        Ok(cf_page(b"", "avad-right")),
        Ok(cf_page(b"left-1", "avad-left-1")),
    ]);
    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    drain_pages(&mut pipeline1, &mut executor1, 1).await;
    let state = pipeline1.snapshot_state().unwrap();
    assert_unordered_snapshot(
        &state,
        Some(ChangeFeedStartFrom::Now),
        &[("", "80", "avad-left-1"), ("80", "FF", "avad-right")],
    );

    let resume_op = avad_change_feed_operation(None);
    let resumed = round_trip_state(state, &resume_op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-merged")])]);
    let mut executor2 = StartRecordingExecutor::new(vec![
        Ok(cf_page(b"merged-left", "avad-left-2")),
        Ok(cf_page(b"merged-right", "avad-right-1")),
    ]);
    let mut pipeline2 = build_unordered_merge(
        &FeedRange::full(),
        &mut topology2,
        &resume_op,
        Some(resumed),
    )
    .await
    .unwrap();
    drain_pages(&mut pipeline2, &mut executor2, 2).await;
    assert_eq!(
        executor2.inner.continuation_calls,
        vec![
            Some("avad-left-1".to_owned()),
            Some("avad-right".to_owned())
        ],
        "both AVAD parent slices must retain their pinned ETags through the merge",
    );
    assert_now_resume_inputs(&executor2);
    assert_merged_parent_targets(&executor2.inner);
}

/// Guards the AllVersionsAndDeletes lossless-`Now` contract: a fresh
/// full-fidelity feed must pin **every** range to a concrete starting ETag
/// before any checkpoint, so a range that is never served before a checkpoint
/// still resumes from its true starting position instead of a resume-time
/// `Now` (which would silently drop the versions and deletes in the gap).
///
/// Two partitions exist, but session 1 pulls only a single page — round-robin
/// alone would poll just the left range. Priming must poll the right range too,
/// so the checkpoint carries an ETag for both. On resume both ranges re-send
/// their pinned ETags rather than restarting.
///
/// The LatestVersion (incremental) feed deliberately does **not** prime: a
/// never-polled range there simply re-reads from the persisted start on resume,
/// which is benign because incremental only surfaces the latest version.
#[tokio::test]
async fn all_versions_and_deletes_pins_every_range_before_checkpoint() {
    let op = avad_change_feed_operation(Some(ChangeFeedStartFrom::Now));

    // Session 1: two partitions. Priming polls left then right (each a
    // start-from-`Now` 304 carrying only an ETag); the round-robin then serves
    // the left range's next poll. Three polls total for one served page.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor1 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"", "lsn-left-0")),       // prime left
        Ok(cf_page(b"", "lsn-right-0")),      // prime right (never served)
        Ok(cf_page(b"left-1", "lsn-left-1")), // served page
    ]);

    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages1, vec![b"left-1".to_vec()]);
    assert_eq!(
        executor1.continuation_calls,
        vec![None, None, Some("lsn-left-0".to_owned())],
        "priming starts both ranges fresh; the served left poll continues from its primed ETag",
    );

    // The snapshot must record an ETag for BOTH ranges — including the right
    // range that was only primed, never served — so neither resumes from `Now`.
    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::UnorderedMerge {
            active_tokens,
            start_from,
        } => {
            assert_eq!(
                active_tokens.len(),
                2,
                "both ranges must be pinned, got {active_tokens:?}",
            );
            assert_eq!(active_tokens[0].max_epk, "80");
            assert_eq!(active_tokens[0].server_continuation, "lsn-left-1");
            assert_eq!(active_tokens[1].min_epk, "80");
            assert_eq!(
                active_tokens[1].server_continuation, "lsn-right-0",
                "the never-served right range must still carry its primed ETag",
            );
            assert_eq!(*start_from, Some(ChangeFeedStartFrom::Now));
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    // Session 2: resume. Resume does not prime (every range already carries a
    // saved ETag). Both ranges must re-send their pinned ETags — the right one
    // in particular must NOT restart from `Now` and drop its gap.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"left-2", "lsn-left-2")),
        Ok(cf_page(b"right-2", "lsn-right-2")),
    ]);

    let mut pipeline2 =
        build_unordered_merge(&FeedRange::full(), &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let pages2 = drain_pages(&mut pipeline2, &mut executor2, 2).await;
    assert_eq!(pages2, vec![b"left-2".to_vec(), b"right-2".to_vec()]);
    assert_eq!(
        executor2.continuation_calls,
        vec![
            Some("lsn-left-1".to_owned()),
            Some("lsn-right-0".to_owned())
        ],
        "resume must re-send each range's pinned ETag, not restart from Now",
    );
}

/// The LatestVersion feed must **not** prime: a single page pull polls only one
/// range, and the never-polled range relies on the persisted `start_from` on
/// resume. This is the counterpart to
/// [`all_versions_and_deletes_pins_every_range_before_checkpoint`] and pins the
/// mode-gated behavior so priming can't accidentally leak into incremental.
#[tokio::test]
async fn latest_version_does_not_prime_ranges() {
    let op = change_feed_operation(Some(ChangeFeedStartFrom::Now));

    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    // Only one response is available: if priming were (incorrectly) enabled the
    // mock would be polled twice and panic on the missing second response.
    let mut executor1 = MockRequestExecutor::new(vec![Ok(cf_page(b"left-1", "lsn-left-1"))]);

    let mut pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    let pages1 = drain_pages(&mut pipeline1, &mut executor1, 1).await;
    assert_eq!(pages1, vec![b"left-1".to_vec()]);
    assert_eq!(
        executor1.continuation_calls,
        vec![None],
        "incremental polls a single range per page — no priming",
    );

    // Only the polled range is pinned; the other relies on `start_from`.
    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::UnorderedMerge { active_tokens, .. } => {
            assert_eq!(active_tokens.len(), 1, "got {active_tokens:?}");
            assert_eq!(active_tokens[0].server_continuation, "lsn-left-1");
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }
}

/// Guards the pre-first-page window of the AllVersionsAndDeletes lossless-`Now`
/// contract: a checkpoint taken **before** the first page is pulled snapshots an
/// empty token set (nothing has been polled yet). Resuming from that empty set
/// must still prime every range, otherwise every range would fall back to a
/// resume-time `Now` and drop the versions/deletes in the gap — the same data
/// loss the priming fix closes, relocated to the pre-first-page window.
///
/// A fully drained feed instead snapshots `PipelineNodeState::Drained`, so an
/// empty `UnorderedMerge` token set unambiguously means "not yet polled" and is
/// safe to treat as still-needs-priming.
#[tokio::test]
async fn all_versions_and_deletes_pins_ranges_when_resumed_before_first_page() {
    let op = avad_change_feed_operation(Some(ChangeFeedStartFrom::Now));

    // Session 1: build the pipeline and checkpoint immediately, before pulling
    // any page. No range has been polled, so the snapshot carries no tokens.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let pipeline1 = build_unordered_merge(&FeedRange::full(), &mut topology1, &op, None)
        .await
        .unwrap();
    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::UnorderedMerge {
            active_tokens,
            start_from,
        } => {
            assert!(
                active_tokens.is_empty(),
                "no range polled yet, expected no tokens, got {active_tokens:?}",
            );
            assert_eq!(*start_from, Some(ChangeFeedStartFrom::Now));
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    // Session 2: resume from the empty-token checkpoint. Priming must still run
    // (the token set is empty), pinning BOTH ranges before the first page.
    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(cf_page(b"", "lsn-left-0")),       // prime left
        Ok(cf_page(b"", "lsn-right-0")),      // prime right (never served)
        Ok(cf_page(b"left-1", "lsn-left-1")), // served page
    ]);

    let mut pipeline2 =
        build_unordered_merge(&FeedRange::full(), &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let pages2 = drain_pages(&mut pipeline2, &mut executor2, 1).await;
    assert_eq!(pages2, vec![b"left-1".to_vec()]);
    assert_eq!(
        executor2.continuation_calls,
        vec![None, None, Some("lsn-left-0".to_owned())],
        "resume-before-first-page must prime both ranges fresh, not skip priming",
    );

    // The follow-up checkpoint now pins both ranges — including the right range
    // that was only primed — so a subsequent resume is lossless.
    let state2 = pipeline2.snapshot_state().unwrap();
    match &state2 {
        PipelineNodeState::UnorderedMerge { active_tokens, .. } => {
            assert_eq!(
                active_tokens.len(),
                2,
                "both ranges must be pinned after priming, got {active_tokens:?}",
            );
            assert_eq!(active_tokens[1].min_epk, "80");
            assert_eq!(active_tokens[1].server_continuation, "lsn-right-0");
        }
        other => panic!("expected UnorderedMerge snapshot, got {other:?}"),
    }
}
