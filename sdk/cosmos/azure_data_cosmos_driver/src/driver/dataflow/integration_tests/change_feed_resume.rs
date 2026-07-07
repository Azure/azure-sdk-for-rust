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

use super::super::{
    mocks::{MockRequestExecutor, MockTopologyProvider, NoopTopologyProvider},
    planner::build_unordered_merge,
    Pipeline, PipelineContext, PipelineNodeState, RequestTarget, ResolvedRange,
};
use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{
        effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
        ChangeFeedStartFrom, ContainerProperties, ContainerReference, ContinuationToken,
        CosmosOperation, CosmosResponse, CosmosResponseHeaders, CosmosStatus, FeedRange,
        ResolvedToken, SystemProperties,
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

/// Drives a pipeline through exactly `n` pages and returns the bodies. Change
/// feed never drains, so a bounded count is the only sensible stop condition.
async fn drain_pages(
    pipeline: &mut Pipeline,
    executor: &mut MockRequestExecutor,
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
    let merged = fr("", "FF");
    assert_eq!(
        executor2.target_calls,
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
        "each merged leaf must carry its parent's EPK sub-range",
    );
}
