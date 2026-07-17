// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore repolled

//! Driver-level integration tests for the cross-partition streaming
//! `ORDER BY` pipeline (`StreamingOrderedMerge`).
//!
//! Composes the real planner, merge node, snapshot, and continuation-token
//! layers against the `MockRequestExecutor` / `MockTopologyProvider` from
//! `dataflow::mocks`, mirroring `query_resume.rs`'s pattern for
//! `SequentialDrain`. Each mocked backend page uses the rewritten-envelope
//! shape (`{"_rid": ..., "Documents": [{"_rid", "orderByItems", "payload"}]}`)
//! `StreamingOrderedMerge` expects.

use std::sync::Arc;

use super::super::{
    mocks::{MockRequestExecutor, MockTopologyProvider},
    order_by::{query_fingerprint, OrderByItem, OrderByResumeValue},
    planner::build_streaming_ordered_merge,
    query_plan::{QueryInfo, QueryPlan, QueryRange, SortOrder},
    snapshot::{OrderByRangeToken, ValueBoundary},
    Pipeline, PipelineContext, PipelineNodeState, ResolvedRange,
};
use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{
        effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
        ContainerProperties, ContainerReference, ContinuationToken, CosmosOperation,
        CosmosResponse, CosmosResponseHeaders, CosmosStatus, FeedRange, MaxItemCountHint,
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

fn order_by_operation() -> Arc<CosmosOperation> {
    Arc::new(
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec()),
    )
}

/// Like [`order_by_operation`] but with an explicit `max_item_count`, to
/// force a precise page boundary.
fn order_by_operation_with_page_size(n: u32) -> Arc<CosmosOperation> {
    Arc::new(
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec())
            .with_max_item_count(MaxItemCountHint::Limit(
                std::num::NonZeroU32::new(n).unwrap(),
            )),
    )
}

/// A single-ascending-column ORDER BY plan spanning the full container.
fn order_by_plan() -> QueryPlan {
    QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(QueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.rank".to_owned()],
            rewritten_query: Some(
                "SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c \
                 WHERE {documentdb-formattableorderbyquery-filter} ORDER BY c.rank ASC"
                    .to_owned(),
            ),
            ..Default::default()
        }),
        query_ranges: vec![QueryRange {
            min: String::new(),
            max: "FF".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: false,
        }],
        hybrid_search_query_info: None,
    }
}

/// Like [`order_by_plan`] but with `c.rank`'s sort `direction` configurable,
/// for DESC-specific coverage (`order_by_plan` stays ASC-only since many
/// existing tests depend on its exact shape).
fn order_by_plan_with_direction(direction: SortOrder) -> QueryPlan {
    let keyword = match direction {
        SortOrder::Ascending => "ASC",
        SortOrder::Descending => "DESC",
    };
    QueryPlan {
        partitioned_query_execution_info_version: 2,
        query_info: Some(QueryInfo {
            order_by: vec![direction],
            order_by_expressions: vec!["c.rank".to_owned()],
            rewritten_query: Some(format!(
                "SELECT c._rid, [{{\"item\":c.rank}}] AS orderByItems, c AS payload FROM c \
                 WHERE {{documentdb-formattableorderbyquery-filter}} ORDER BY c.rank {keyword}"
            )),
            ..Default::default()
        }),
        query_ranges: vec![QueryRange {
            min: String::new(),
            max: "FF".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: false,
        }],
        hybrid_search_query_info: None,
    }
}

/// A realistic, 16-byte-encoded document `_rid` (matching real Cosmos DB's
/// hierarchical RID layout) whose little-endian document-ordinal segment is
/// `doc_id` — exercises `models::resource_id::compare_document_rids`'s
/// numeric decode path, unlike the short synthetic rids used elsewhere in
/// this file (e.g. `"a"`, `"tied-1"`), which take its raw-string fallback.
fn real_rid(doc_id: u64) -> String {
    let mut bytes = [0u8; 16];
    bytes[0..4].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]);
    bytes[4..8].copy_from_slice(&[0x80, 0x01, 0x02, 0x03]);
    bytes[8..16].copy_from_slice(&doc_id.to_le_bytes());
    crate::models::resource_id::encode_rid(&bytes)
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

/// Builds a rewritten-envelope page with one row per `(rid, rank)` pair.
fn envelope_page(rows: &[(&str, i64)], continuation: Option<&str>) -> CosmosResponse {
    let documents: Vec<serde_json::Value> = rows
        .iter()
        .map(|(rid, rank)| {
            serde_json::json!({
                "_rid": rid,
                "orderByItems": [{"item": rank}],
                "payload": {"id": rid, "rank": rank},
            })
        })
        .collect();
    let body = serde_json::json!({
        "_rid": "",
        "Documents": documents,
        "_count": documents.len(),
    });
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    diagnostics.set_operation_status(azure_core::http::StatusCode::Ok, None);
    let mut headers = CosmosResponseHeaders::new();
    headers.continuation = continuation.map(str::to_owned);
    headers.request_charge = Some(crate::models::RequestCharge::new(1.5));
    CosmosResponse::new(
        serde_json::to_vec(&body).unwrap(),
        headers,
        CosmosStatus::new(azure_core::http::StatusCode::Ok),
        Arc::new(diagnostics.complete()),
    )
}

/// Like [`envelope_page`] but with a string sort key per row, for
/// string-boundary resume coverage.
fn string_envelope_page(rows: &[(&str, &str)], continuation: Option<&str>) -> CosmosResponse {
    let documents: Vec<serde_json::Value> = rows
        .iter()
        .map(|(rid, key)| {
            serde_json::json!({
                "_rid": rid,
                "orderByItems": [{"item": key}],
                "payload": {"id": rid, "key": key},
            })
        })
        .collect();
    let body = serde_json::json!({
        "_rid": "",
        "Documents": documents,
        "_count": documents.len(),
    });
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    diagnostics.set_operation_status(azure_core::http::StatusCode::Ok, None);
    let mut headers = CosmosResponseHeaders::new();
    headers.continuation = continuation.map(str::to_owned);
    headers.request_charge = Some(crate::models::RequestCharge::new(1.5));
    CosmosResponse::new(
        serde_json::to_vec(&body).unwrap(),
        headers,
        CosmosStatus::new(azure_core::http::StatusCode::Ok),
        Arc::new(diagnostics.complete()),
    )
}

/// Extracts every item's `id` across `page`'s `Documents`, in wire order.
fn ids_in_page(page: &CosmosResponse) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_slice(page.body_bytes()).unwrap();
    value["Documents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap().to_owned())
        .collect()
}

async fn drain_all(pipeline: &mut Pipeline, executor: &mut MockRequestExecutor) -> Vec<String> {
    let mut ids = Vec::new();
    let mut topology = super::super::mocks::NoopTopologyProvider;
    loop {
        let mut context = PipelineContext::new(executor, Some(&mut topology));
        match pipeline.next_page(&mut context).await.unwrap() {
            Some(response) => ids.extend(ids_in_page(&response)),
            None => break,
        }
    }
    ids
}

/// Like [`drain_all`] but supplies a real topology provider, needed when a
/// split happens during iteration.
async fn drain_all_with_topology(
    pipeline: &mut Pipeline,
    executor: &mut MockRequestExecutor,
    topology: &mut MockTopologyProvider,
) -> Vec<String> {
    let mut ids = Vec::new();
    loop {
        let mut context = PipelineContext::new(executor, Some(topology));
        match pipeline.next_page(&mut context).await.unwrap() {
            Some(response) => ids.extend(ids_in_page(&response)),
            None => break,
        }
    }
    ids
}

async fn drain_one(pipeline: &mut Pipeline, executor: &mut MockRequestExecutor) -> Vec<String> {
    let mut topology = super::super::mocks::NoopTopologyProvider;
    let mut context = PipelineContext::new(executor, Some(&mut topology));
    let response = pipeline
        .next_page(&mut context)
        .await
        .unwrap()
        .expect("expected a page, not drained");
    ids_in_page(&response)
}

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

// ── Tests ────────────────────────────────────────────────────────────────

/// Baseline: two partitions each locally sorted ascending by `rank`; the
/// merge must interleave them into one globally sorted stream.
#[tokio::test]
async fn merges_two_partitions_into_global_order() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[("l1", 1), ("l2", 3), ("l3", 5)], None)),
        Ok(envelope_page(&[("r1", 2), ("r2", 4), ("r3", 6)], None)),
    ]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;

    assert_eq!(
        ids,
        vec!["l1", "r1", "l2", "r2", "l3", "r3"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "rows must interleave in ascending rank order across both partitions"
    );
}

/// A single partition, single page: the trivial case must still flow
/// through the merge machinery correctly (no fan-out needed).
#[tokio::test]
async fn single_partition_passthrough() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor =
        MockRequestExecutor::new(vec![Ok(envelope_page(&[("a", 1), ("b", 2)], None))]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(ids, vec!["a".to_owned(), "b".to_owned()]);
}

/// An empty backend page carrying a continuation must be transparently
/// re-polled, not mistaken for "drained" or surfaced as an empty result.
#[tokio::test]
async fn empty_page_with_continuation_is_repolled() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[], Some("ct-empty"))),
        Ok(envelope_page(&[("a", 1)], None)),
    ]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(ids, vec!["a".to_owned()]);
    assert_eq!(
        executor.continuation_calls,
        vec![None, Some("ct-empty".to_owned())]
    );
}

/// Empty total result must surface as a single empty, terminal page
/// (matching a plain `Request`), not an error.
#[tokio::test]
async fn empty_total_result_surfaces_as_terminal_empty_page() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor = MockRequestExecutor::new(vec![Ok(envelope_page(&[], None))]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert!(ids.is_empty());
}

/// Baseline resume: session 1 drains one page (continuation "ct-1"
/// pending); session 2 must forward it, not fresh-start.
#[tokio::test]
async fn resume_with_unchanged_topology_forwards_continuation() {
    let op = order_by_operation_with_page_size(1);
    let plan = order_by_plan();

    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(envelope_page(&[("a", 1)], Some("ct-1")))]);
    let mut pipeline1 = build_streaming_ordered_merge(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let ids1 = drain_one(&mut pipeline1, &mut executor1).await;
    assert_eq!(ids1, vec!["a".to_owned()]);
    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(ranges.len(), 1);
            assert_eq!(ranges[0].server_continuation, Some("ct-1".to_owned()));
            assert!(
                ranges[0].boundary.is_some(),
                "boundary must be recorded even though a plain continuation is also available"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    let resumed_state = round_trip_state(state, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(envelope_page(&[("b", 2)], None))]);
    let mut pipeline2 =
        build_streaming_ordered_merge(&plan, &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let ids2 = drain_all(&mut pipeline2, &mut executor2).await;
    assert_eq!(ids2, vec!["b".to_owned()]);
    assert_eq!(
        executor2.continuation_calls,
        vec![Some("ct-1".to_owned())],
        "resume must reuse the saved plain continuation when topology is unchanged"
    );
}

/// Regression: a mid-page checkpoint (no safe `server_continuation`)
/// resumed against unchanged topology must apply the value-boundary
/// resume path, not a fresh restart that re-emits delivered rows.
#[tokio::test]
async fn resume_with_unchanged_topology_and_no_saved_continuation_does_not_duplicate_rows() {
    let op = order_by_operation_with_page_size(1);
    let plan = order_by_plan();

    // `max_item_count = 1` surfaces one row per call, leaving two rows
    // buffered — the "mid-page" case forcing `server_continuation = None`.
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(envelope_page(
        &[("a", 1), ("b", 2), ("c", 3)],
        None,
    ))]);
    let mut pipeline1 = build_streaming_ordered_merge(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let ids1 = drain_one(&mut pipeline1, &mut executor1).await;
    assert_eq!(ids1, vec!["a".to_owned()]);

    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(ranges.len(), 1);
            assert_eq!(
                ranges[0].server_continuation, None,
                "buffer held unread rows at checkpoint time, so no plain \
                 continuation is safe to save"
            );
            assert!(
                ranges[0].boundary.is_some(),
                "a row was already emitted, so a resume boundary must be recorded"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    let resumed_state = round_trip_state(state, &op);
    // Mock re-returns the full unfiltered page (mock can't evaluate SQL
    // filters); the client-side discard must strip the "a" row back out.
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(envelope_page(
        &[("a", 1), ("b", 2), ("c", 3)],
        None,
    ))]);
    let mut pipeline2 =
        build_streaming_ordered_merge(&plan, &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let ids2 = drain_all(&mut pipeline2, &mut executor2).await;
    assert_eq!(
        ids2,
        vec!["b".to_owned(), "c".to_owned()],
        "row \"a\" was already emitted before the checkpoint and must not \
         be re-emitted on resume"
    );
}

/// Catalog-sourced regression for `equal_key_resume_requiring_skip_count`:
/// resuming a tied-row value boundary on unchanged topology must apply
/// the `_rid`-aware discard, not a fresh restart.
#[tokio::test]
async fn catalog_equal_key_resume_requiring_skip_count_replays_correctly() {
    const CATALOG_JSON: &str =
        include_str!("../../../../tests/fixtures/streaming_order_by_scenarios.json");

    // Parse generically and pick out only the one scenario this test needs.
    let catalog: serde_json::Value =
        serde_json::from_str(CATALOG_JSON).expect("catalog must parse as JSON");
    let scenario = catalog["scenarios"]
        .as_array()
        .expect("catalog must have a scenarios array")
        .iter()
        .find(|s| s["id"] == "equal_key_resume_requiring_skip_count")
        .expect("catalog must contain scenario equal_key_resume_requiring_skip_count");

    let expected_ids: Vec<String> = scenario["expectedIds"]
        .as_array()
        .expect("scenario must declare expectedIds")
        .iter()
        .map(|v| {
            v.as_str()
                .expect("expectedIds entries must be strings")
                .to_owned()
        })
        .collect();

    let rows: Vec<(String, i64)> = scenario["mock"]["partitions"][0]["pages"][0]["rows"]
        .as_array()
        .expect("scenario must declare mock rows")
        .iter()
        .map(|row| {
            let rid = row["_rid"]
                .as_str()
                .or_else(|| row["rid"].as_str())
                .expect("row must have a rid")
                .to_owned();
            let item = row["orderByItems"][0]["item"]
                .as_i64()
                .expect("this scenario's sort key is a plain integer");
            (rid, item)
        })
        .collect();
    let row_refs: Vec<(&str, i64)> = rows
        .iter()
        .map(|(rid, rank)| (rid.as_str(), *rank))
        .collect();

    let checkpoint = &scenario["checkpoint"];
    let resume_values: Vec<OrderByResumeValue> =
        serde_json::from_value(checkpoint["resumeValues"].clone())
            .expect("checkpoint.resumeValues must parse as OrderByResumeValue");
    let last_rid = checkpoint["lastRid"]
        .as_str()
        .expect("checkpoint must declare lastRid")
        .to_owned();
    // `skipCount` records how many rows tied with the boundary were
    // already emitted; the `_rid`-aware resume no longer consumes a
    // positional count, but the field is still meaningful metadata (and
    // seeds `rows_emitted`).
    let rows_emitted = Some(
        checkpoint["skipCount"]
            .as_u64()
            .expect("checkpoint must declare skipCount"),
    );

    let op = order_by_operation();
    let plan = order_by_plan();
    let rewritten_query = plan
        .query_info
        .as_ref()
        .unwrap()
        .rewritten_query
        .as_deref()
        .unwrap();

    let resumed_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: query_fingerprint(rewritten_query),
        ranges: vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values,
                last_rid,
                rows_emitted,
            }),
        }],
    };

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor = MockRequestExecutor::new(vec![Ok(envelope_page(&row_refs, None))]);
    let mut pipeline =
        build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resumed_state))
            .await
            .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(
        ids, expected_ids,
        "scenario {} drained ids do not match the catalog's expectedIds",
        scenario["id"]
    );
}

/// Split during live iteration: the merge must splice in replacement
/// ranges and keep global ordering across the remaining rows.
#[tokio::test]
async fn split_mid_merge_splices_replacements_and_preserves_order() {
    let op = order_by_operation();
    let plan = order_by_plan();

    // A split resolves the new topology twice (see `handle_split`'s doc
    // comment), so the same post-split resolution is queued twice.
    let mut topology = MockTopologyProvider::new(vec![
        Ok(vec![resolved("", "FF", "pk-0")]),
        Ok(vec![
            resolved("", "80", "pk-left"),
            resolved("80", "FF", "pk-right"),
        ]),
        Ok(vec![
            resolved("", "80", "pk-left"),
            resolved("80", "FF", "pk-right"),
        ]),
    ]);
    let mut executor = MockRequestExecutor::new(vec![
        Err(super::super::mocks::gone_error()),
        Ok(envelope_page(&[("l1", 1), ("l2", 3)], None)),
        Ok(envelope_page(&[("r1", 2), ("r2", 4)], None)),
    ]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all_with_topology(&mut pipeline, &mut executor, &mut topology).await;
    assert_eq!(
        ids,
        vec!["l1", "r1", "l2", "r2"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    );
}

/// Regression for the in-flight split ordering defect: the split happens
/// while *replenishing the popped winner mid-pop-loop* (not during the
/// initial prime), fanning P0 into two sub-ranges. Both replacements must be
/// primed before the next selection, or P0b's `10, 20` would be skipped and
/// P1's `50` emitted ahead of them. A default (large) page cap keeps popping
/// within a single page so the mis-ordering would surface immediately.
#[tokio::test]
async fn split_during_replenish_primes_all_replacements_preserving_order() {
    let op = order_by_operation();
    let plan = order_by_plan();

    // Initial resolve yields two live partitions; P0 then splits (its own
    // resolve + `handle_split`'s), so the post-split resolution is queued
    // twice after the initial one.
    let mut topology = MockTopologyProvider::new(vec![
        Ok(vec![
            resolved("", "80", "pk-p0"),
            resolved("80", "FF", "pk-p1"),
        ]),
        Ok(vec![
            resolved("", "40", "pk-a"),
            resolved("40", "80", "pk-b"),
        ]),
        Ok(vec![
            resolved("", "40", "pk-a"),
            resolved("40", "80", "pk-b"),
        ]),
    ]);
    // P0's first page delivers 1, 2 (continuation pending); replenishing it
    // 410s into a split. P0a resumes with 3, P0b with 10, 20 (the mock can't
    // honor the seek filter, so the `_rid`-aware discard keeps rows past 2).
    let mut executor = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct"))),
        Ok(envelope_page(&[("d50", 50)], None)),
        Err(super::super::mocks::gone_error()),
        Ok(envelope_page(&[("d3", 3)], None)),
        Ok(envelope_page(&[("d10", 10), ("d20", 20)], None)),
    ]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let ids = drain_all_with_topology(&mut pipeline, &mut executor, &mut topology).await;
    assert_eq!(
        ids,
        vec!["d1", "d2", "d3", "d10", "d20", "d50"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "the second split replacement's rows (10, 20) must precede 50"
    );
}

/// Companion to the large-cap regression: a page cap that fills exactly as a
/// mid-pop-loop split completes must checkpoint immediately after the split
/// (snapshotting both replacements), then resume in global order. Page 1
/// yields 1, 2, 3; the persisted token resumes 10, 20, 50 across the split.
#[tokio::test]
async fn split_during_replenish_checkpoint_resumes_in_global_order() {
    let op = order_by_operation_with_page_size(3);
    let plan = order_by_plan();

    let mut topology1 = MockTopologyProvider::new(vec![
        Ok(vec![
            resolved("", "80", "pk-p0"),
            resolved("80", "FF", "pk-p1"),
        ]),
        Ok(vec![
            resolved("", "40", "pk-a"),
            resolved("40", "80", "pk-b"),
        ]),
        Ok(vec![
            resolved("", "40", "pk-a"),
            resolved("40", "80", "pk-b"),
        ]),
    ]);
    let mut executor1 = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[("d1", 1), ("d2", 2)], Some("p0-ct"))),
        Ok(envelope_page(&[("d50", 50)], None)),
        Err(super::super::mocks::gone_error()),
        Ok(envelope_page(&[("d3", 3)], None)),
        Ok(envelope_page(&[("d10", 10), ("d20", 20)], None)),
    ]);

    let mut pipeline1 = build_streaming_ordered_merge(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let page1 = {
        let mut context = PipelineContext::new(&mut executor1, Some(&mut topology1));
        pipeline1
            .next_page(&mut context)
            .await
            .unwrap()
            .expect("expected a first page")
    };
    assert_eq!(
        ids_in_page(&page1),
        vec!["d1".to_owned(), "d2".to_owned(), "d3".to_owned()],
        "page 1 stops at the cap right after the split"
    );

    let state = pipeline1.snapshot_state().unwrap();
    match &state {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(
                ranges.len(),
                2,
                "both post-split children (P0b + P1) must survive into the checkpoint"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    let resumed_state = round_trip_state(state, &op);
    // On resume both saved ranges are unchanged; P0b re-seeks past its value
    // boundary (mock returns 10, 20 unfiltered), P1 restarts fresh (50).
    let mut topology2 = MockTopologyProvider::new(vec![
        Ok(vec![resolved("40", "80", "pk-b")]),
        Ok(vec![resolved("80", "FF", "pk-p1")]),
    ]);
    let mut executor2 = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[("d10", 10), ("d20", 20)], None)),
        Ok(envelope_page(&[("d50", 50)], None)),
    ]);
    let mut pipeline2 =
        build_streaming_ordered_merge(&plan, &mut topology2, &op, Some(resumed_state))
            .await
            .unwrap();
    let resumed = drain_all(&mut pipeline2, &mut executor2).await;
    assert_eq!(
        resumed,
        vec!["d10", "d20", "d50"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "resume continues the global order across the split checkpoint"
    );
}

/// Regression: after a split, both sub-ranges resume from the same
/// boundary; the `_rid`-aware discard must avoid dropping/duplicating rows.
#[tokio::test]
async fn resume_after_split_with_emitted_ties_has_no_omissions_or_duplicates() {
    let op = order_by_operation();
    let plan = order_by_plan();
    let rewritten_query = plan
        .query_info
        .as_ref()
        .unwrap()
        .rewritten_query
        .as_deref()
        .unwrap();

    let resumed_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: query_fingerprint(rewritten_query),
        ranges: vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![OrderByResumeValue::Number { value: 5.0.into() }],
                last_rid: "c".to_owned(),
                rows_emitted: Some(3),
            }),
        }],
    };

    // The saved range resolves to two post-split sub-ranges.
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    // Mock can't honor the server-side filter, so each sub-range returns
    // rows unfiltered. `[,80)` holds a, c (emitted), e (unemitted tie),
    // m (rank 7); `[80,FF)` holds b (emitted), z (rank 6).
    let mut executor = MockRequestExecutor::new(vec![
        Ok(envelope_page(
            &[("a", 5), ("c", 5), ("e", 5), ("m", 7)],
            None,
        )),
        Ok(envelope_page(&[("b", 5), ("z", 6)], None)),
    ]);

    let mut pipeline =
        build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resumed_state))
            .await
            .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(
        ids,
        vec!["e", "z", "m"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "already-emitted tied rows (a, b, c) are dropped by `_rid`; the \
         unemitted tied row `e` survives with no duplicates"
    );
}

/// Regression: a complex boundary crossing a split must fail fast, since
/// its positional count can't be attributed to the new sub-ranges.
#[tokio::test]
async fn resume_rejects_complex_boundary_across_split() {
    let op = order_by_operation();
    let plan = order_by_plan();
    let rewritten_query = plan
        .query_info
        .as_ref()
        .unwrap()
        .rewritten_query
        .as_deref()
        .unwrap();

    let complex = OrderByItem::Array(vec![OrderByItem::Number(1.0.into())]).to_resume_value();
    let resumed_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: query_fingerprint(rewritten_query),
        ranges: vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![complex],
                last_rid: "c".to_owned(),
                rows_emitted: Some(3),
            }),
        }],
    };

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resumed_state))
        .await
        .unwrap_err();
    assert_eq!(
        err.status(),
        CosmosStatus::CLIENT_STREAMING_MERGE_COMPLEX_BOUNDARY_TOPOLOGY_CHANGE
    );
}

/// Regression: a saved sub-range resolving to a wider merged partition
/// must be clipped to scope, not rejected as "over-covering".
#[tokio::test]
async fn resume_after_merge_clips_widened_range_and_drains() {
    let op = order_by_operation();
    let plan = order_by_plan();
    let rewritten_query = plan
        .query_info
        .as_ref()
        .unwrap()
        .rewritten_query
        .as_deref()
        .unwrap();

    let resumed_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: query_fingerprint(rewritten_query),
        ranges: vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "80".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![OrderByResumeValue::Number { value: 5.0.into() }],
                last_rid: "c".to_owned(),
                rows_emitted: Some(3),
            }),
        }],
    };

    // Post-merge: the saved [,80) sub-range is now served by a wider
    // physical partition [,FF).
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-merged")])]);
    let mut executor = MockRequestExecutor::new(vec![Ok(envelope_page(
        &[("a", 5), ("c", 5), ("e", 5), ("n", 8)],
        None,
    ))]);

    let mut pipeline =
        build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resumed_state))
            .await
            .expect("resume across a merge (widened physical range) must not be rejected");
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert_eq!(
        ids,
        vec!["e", "n"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "already-emitted rows (a, c) are dropped; the range's unemitted rows drain"
    );
}

/// A token minted for a different rewritten query (fingerprint mismatch)
/// must be rejected outright, not silently resumed.
#[tokio::test]
async fn resume_rejects_fingerprint_mismatch() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mismatched_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: "not-the-real-fingerprint".to_owned(),
        ranges: vec![super::super::snapshot::OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: Some("ct".to_owned()),
            boundary: None,
        }],
    };

    let mut topology = MockTopologyProvider::new(vec![]);
    let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(mismatched_state))
        .await
        .unwrap_err();
    assert_eq!(
        err.status(),
        CosmosStatus::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID
    );
}

/// A continuation token shaped for a different node type (e.g.
/// `SequentialDrain`) must be rejected with a clear shape-mismatch error.
#[tokio::test]
async fn resume_rejects_wrong_node_shape() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let wrong_shape = PipelineNodeState::SequentialDrain {
        left_most_undrained_epk: String::new(),
        active_tokens: vec![],
    };

    let mut topology = MockTopologyProvider::new(vec![]);
    let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(wrong_shape))
        .await
        .unwrap_err();
    assert_eq!(
        err.status(),
        CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH
    );
}

/// A `Drained` continuation token must short-circuit to a drained pipeline
/// without issuing any requests.
#[tokio::test]
async fn resume_from_drained_short_circuits() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![]);
    let mut executor = MockRequestExecutor::new(vec![]);
    let mut pipeline =
        build_streaming_ordered_merge(&plan, &mut topology, &op, Some(PipelineNodeState::Drained))
            .await
            .unwrap();
    let ids = drain_all(&mut pipeline, &mut executor).await;
    assert!(ids.is_empty());
}

/// Request charge is summed across every backend page; the emitted page
/// never carries a raw backend continuation header.
#[tokio::test]
async fn aggregates_request_charge_and_omits_backend_continuation_header() {
    let op = order_by_operation();
    let plan = order_by_plan();

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![
        resolved("", "80", "pk-left"),
        resolved("80", "FF", "pk-right"),
    ])]);
    let mut executor = MockRequestExecutor::new(vec![
        Ok(envelope_page(&[("l1", 1)], None)),
        Ok(envelope_page(&[("r1", 2)], None)),
    ]);

    let mut pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
        .await
        .unwrap();
    let mut noop_topology = super::super::mocks::NoopTopologyProvider;
    let mut context = PipelineContext::new(&mut executor, Some(&mut noop_topology));
    let response = pipeline
        .next_page(&mut context)
        .await
        .unwrap()
        .expect("expected a page");
    assert_eq!(
        response.headers().request_charge,
        Some(crate::models::RequestCharge::new(3.0)),
        "charge from both partitions' pages must be summed"
    );
    assert!(response.headers().continuation.is_none());
}

// ── Query-shape / continuation-binding regression ───────────────────────────

/// Asserts a recorded request body is the resume-filtered value-boundary
/// query: the rewritten query with a scalar seek filter substituted for
/// the Gateway placeholder — not the plain query, the raw placeholder, or
/// an outer subquery wrapper (a rejected shape).
fn assert_is_resume_filtered_query(body: &str) {
    assert!(
        !body.contains("{documentdb-formattableorderbyquery-filter}"),
        "the resume filter placeholder must be substituted, not sent verbatim: {body}"
    );
    assert!(
        !body.contains("SELECT VALUE r FROM ("),
        "resume must inject a flat scalar filter, never wrap the rewritten query as an \
         outer envelope subquery: {body}"
    );
    assert!(
        body.contains("IS_NUMBER(c.rank)") && !body.contains("WHERE true"),
        "resume-filtered query must carry the value-boundary seek filter substituted for \
         the Gateway placeholder, not the fresh `WHERE true`: {body}"
    );
    // `c._rid` appears exactly once, in the envelope's `SELECT` projection
    // (needed for the client-side numeric discard) — the filter itself
    // carries no `_rid` predicate: a base64 `_rid` string doesn't sort in
    // document-ordinal order, so the tie-break stays client-side (see
    // `order_by::ResumeFilter`).
    assert_eq!(
        body.matches("_rid").count(),
        1,
        "{body}: rid stays client-side, not in the filter"
    );
    assert!(
        body.contains("c.rank"),
        "resume filter must reference the original source ORDER BY expression, not a \
         synthesized envelope field: {body}"
    );
    // The scalar boundary value is bound as a `@cosmosResumeFilter*`
    // parameter, never inlined as SQL text (see `order_by::ResumeFilter`).
    assert!(
        body.contains("c.rank > @cosmosResumeFilter0"),
        "resume filter must compare against a bound parameter, not an inline literal: {body}"
    );
}

/// Shared driver for a fresh → mid-page resume-filtered → second-resume
/// cycle, asserting the resume-filtered query's backend continuation is
/// never snapshotted into the plain `server_continuation` nor replayed
/// against the plain query on the next resume.
///
/// This is the end-to-end regression for the query-shape binding bug:
/// before the fix, once a resume-filtered child reached an empty buffer
/// with a live backend continuation, `snapshot_state` stored that opaque,
/// filtered-query-bound token in the plain field, and the next resume fed
/// it to the plain query — an opaque-token/query-shape mismatch (400 /
/// duplication / omission risk).
///
/// Each session emits one row (`max_item_count = 1`): session 1 emits the
/// first row and buffers the rest (mid-page checkpoint); session 2 resumes
/// via the resume-filtered query, emits one row, and checkpoints with an
/// empty buffer while its continuation is still live; session 3 resumes
/// again and drains the last row. The mock can't evaluate the server-side
/// filter, so pages return unfiltered — the client-side discard strips
/// already-emitted rows. Since the mock replies FIFO regardless of the
/// token handed to it, drained order alone wouldn't prove token binding,
/// so recorded request bodies and continuation calls are inspected directly.
async fn run_resume_filtered_binding_cycle(
    session1_page: &[(&str, i64)],
    session2_page: &[(&str, i64)],
    session3_page: &[(&str, i64)],
    resume_filtered_ct: &str,
    expected_order: &[&str],
) {
    let op = order_by_operation_with_page_size(1);
    let plan = order_by_plan();

    // ── Session 1: fresh, mid-page checkpoint ───────────────────────────
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 =
        MockRequestExecutor::new(vec![Ok(envelope_page(session1_page, Some("s1-more")))]);
    let mut pipeline1 = build_streaming_ordered_merge(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let ids1 = drain_one(&mut pipeline1, &mut executor1).await;
    assert_eq!(ids1.len(), 1, "page size 1 must surface exactly one row");
    // The fresh child ran the plain rewritten query: the Gateway
    // resume-filter placeholder was replaced with `true` (never wrapped as
    // an outer subquery, and never sent verbatim).
    let fresh_body = executor1.body_text(0);
    assert!(
        !fresh_body.contains("SELECT VALUE r FROM ("),
        "session 1 must issue the plain rewritten query, not a wrapped subquery: {fresh_body}"
    );
    assert!(
        !fresh_body.contains("{documentdb-formattableorderbyquery-filter}"),
        "session 1 must substitute the fresh placeholder, not send it verbatim: {fresh_body}"
    );
    assert!(
        fresh_body.contains("WHERE true"),
        "a fresh streaming ORDER BY query replaces the placeholder with `true`: {fresh_body}"
    );

    let state1 = pipeline1.snapshot_state().unwrap();
    match &state1 {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(ranges.len(), 1);
            assert_eq!(
                ranges[0].server_continuation, None,
                "mid-page checkpoint holds unread rows, so no plain continuation is safe"
            );
            assert!(
                ranges[0].boundary.is_some(),
                "a row was emitted, so a scalar resume boundary must be recorded"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline1);

    // ── Session 2: scalar value-boundary resume-filtered query ──────────
    let resumed1 = round_trip_state(state1, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(envelope_page(
        session2_page,
        Some(resume_filtered_ct),
    ))]);
    let mut pipeline2 = build_streaming_ordered_merge(&plan, &mut topology2, &op, Some(resumed1))
        .await
        .unwrap();
    let ids2 = drain_one(&mut pipeline2, &mut executor2).await;
    assert_eq!(ids2.len(), 1);

    // The resume must run the resume-filtered `_rid`-aware query with a
    // *fresh* start (no forwarded backend continuation).
    assert_eq!(
        executor2.continuation_calls,
        vec![None],
        "a value-boundary resume starts the resume-filtered query fresh"
    );
    assert_is_resume_filtered_query(&executor2.body_text(0));

    let state2 = pipeline2.snapshot_state().unwrap();
    let (state2_continuation, state2_boundary_rid) = match &state2 {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(ranges.len(), 1);
            (
                ranges[0].server_continuation.clone(),
                ranges[0].boundary.as_ref().map(|b| b.last_rid.clone()),
            )
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    };
    // The core assertion: the resume-filtered query's live backend
    // continuation is NOT captured into the plain `server_continuation`
    // field. Before the fix this was `Some(resume_filtered_ct)`.
    assert_eq!(
        state2_continuation, None,
        "a resume-filtered value-boundary child must never snapshot its backend \
         continuation into the plain server_continuation field"
    );
    assert_eq!(
        state2_boundary_rid.as_deref(),
        Some(ids2[0].as_str()),
        "the scalar boundary must advance to the row just emitted"
    );
    drop(pipeline2);

    // ── Session 3: second resume ────────────────────────────────────────
    let resumed2 = round_trip_state(state2, &op);
    let mut topology3 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor3 = MockRequestExecutor::new(vec![Ok(envelope_page(session3_page, None))]);
    let mut pipeline3 = build_streaming_ordered_merge(&plan, &mut topology3, &op, Some(resumed2))
        .await
        .unwrap();
    let ids3 = drain_all(&mut pipeline3, &mut executor3).await;

    // The decisive proof: the second resume rebuilds the resume-filtered
    // query from the scalar boundary and starts it fresh. It must NOT replay
    // the resume-filtered token against the plain query.
    assert!(
        !executor3
            .continuation_calls
            .contains(&Some(resume_filtered_ct.to_owned())),
        "the resume-filtered backend continuation must never be replayed on resume; \
         continuation_calls = {:?}",
        executor3.continuation_calls
    );
    assert_eq!(
        executor3.continuation_calls.first(),
        Some(&None),
        "the second resume must start the rebuilt resume-filtered query fresh, not forward a token"
    );
    assert_is_resume_filtered_query(&executor3.body_text(0));

    // End-to-end correctness: every row is delivered exactly once, in order.
    let mut all_ids = ids1;
    all_ids.extend(ids2);
    all_ids.extend(ids3);
    assert_eq!(
        all_ids,
        expected_order
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        "across two snapshot/resume cycles every row must be delivered exactly once, in order"
    );
}

/// Regression (distinct keys): a resume-filtered checkpoint with a live
/// continuation must not persist it; next resume re-derives from boundary.
#[tokio::test]
async fn resume_filtered_query_never_replays_backend_continuation_against_plain_query() {
    run_resume_filtered_binding_cycle(
        &[("a", 1), ("b", 2)],
        &[("a", 1), ("b", 2)],
        &[("b", 2), ("c", 3)],
        "resume-filtered-ct-1",
        &["a", "b", "c"],
    )
    .await;
}

/// Regression (tied keys): correctness rests on the `_rid` tiebreak
/// surviving the resume-filtered snapshot without mis-binding.
#[tokio::test]
async fn resume_filtered_query_with_tied_keys_never_replays_backend_continuation() {
    run_resume_filtered_binding_cycle(
        &[("a", 5), ("b", 5)],
        &[("a", 5), ("b", 5)],
        &[("b", 5), ("c", 5)],
        "resume-filtered-ct-tied",
        &["a", "b", "c"],
    )
    .await;
}

/// A resume whose boundary value is a string full of SQL special characters
/// (quote, backslash, newline, tab, non-ASCII) must bind it as a query
/// parameter, never inline it as SQL text. Asserts the recorded request
/// body: the SQL references the `@cosmosResumeFilter0` parameter, the raw
/// string never appears in the query text, and the body's `parameters`
/// array carries the exact string verbatim (B1 service-safe boundaries).
#[tokio::test]
async fn string_boundary_resume_binds_parameter_verbatim_not_inline_sql() {
    let nasty = "a' OR 1=1 -- \\ \n\t\u{2713}";
    let op = order_by_operation();
    let plan = order_by_plan();
    let rewritten_query = plan
        .query_info
        .as_ref()
        .unwrap()
        .rewritten_query
        .as_deref()
        .unwrap();

    let resumed_state = PipelineNodeState::StreamingOrderedMerge {
        directions: vec![SortOrder::Ascending],
        query_fingerprint: query_fingerprint(rewritten_query),
        ranges: vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![OrderByResumeValue::String {
                    value: nasty.to_owned(),
                }],
                last_rid: "rid-1".to_owned(),
                rows_emitted: Some(1),
            }),
        }],
    };

    let mut topology = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor = MockRequestExecutor::new(vec![Ok(envelope_page(&[], None))]);
    let mut pipeline =
        build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resumed_state))
            .await
            .unwrap();
    let _ = drain_all(&mut pipeline, &mut executor).await;

    let body = executor.body_text(0);
    let value: serde_json::Value =
        serde_json::from_str(&body).expect("recorded request body must be valid JSON");
    let query = value["query"].as_str().expect("body has a query string");
    assert!(
        query.contains("@cosmosResumeFilter0"),
        "SQL must reference the bound parameter: {query}"
    );
    // The adversarial text must never leak into the SQL — only into the value.
    for needle in ["OR 1=1", "'", "\\"] {
        assert!(
            !query.contains(needle),
            "boundary text {needle:?} must not appear in the query SQL: {query}"
        );
    }
    let parameters = value["parameters"].as_array().expect("body has parameters");
    let bound = parameters
        .iter()
        .find(|p| p["name"] == "@cosmosResumeFilter0")
        .expect("resume parameter must be present in the body");
    assert_eq!(
        bound["value"],
        serde_json::Value::String(nasty.to_owned()),
        "the exact boundary string round-trips as the parameter value"
    );
}

/// Asserts a recorded request body's resume filter binds `expected` as the
/// `@cosmosResumeFilter0` parameter and never inlines it into the SQL.
fn assert_string_param_boundary(body: &str, expected: &str) {
    let value: serde_json::Value =
        serde_json::from_str(body).expect("recorded request body must be valid JSON");
    let query = value["query"].as_str().expect("body has a query string");
    assert!(
        query.contains("@cosmosResumeFilter0"),
        "SQL must reference the bound parameter: {query}"
    );
    assert!(
        !query.contains(expected),
        "boundary text {expected:?} must not appear in the query SQL: {query}"
    );
    let bound = value["parameters"]
        .as_array()
        .expect("body has parameters")
        .iter()
        .find(|p| p["name"] == "@cosmosResumeFilter0")
        .expect("resume parameter must be present");
    assert_eq!(
        bound["value"],
        serde_json::Value::String(expected.to_owned())
    );
}

/// Repeated resume with string sort keys carrying SQL special characters:
/// fresh → checkpoint → resume → checkpoint → resume. Every resumed request
/// body must bind the boundary string as a parameter (never inline it), the
/// binding must survive each token serialize/deserialize round-trip, and all
/// rows must be delivered exactly once in order.
#[tokio::test]
async fn repeated_string_boundary_resume_binds_parameter_each_cycle() {
    // Keys sort a < b < c and each carries a distinct special character.
    let (ka, kb, kc) = ("k1' ", "k2\\", "k3\t");
    let op = order_by_operation_with_page_size(1);
    let plan = order_by_plan();

    // ── Session 1: fresh, emit "a", checkpoint mid-page ─────────────────
    let mut topology1 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor1 = MockRequestExecutor::new(vec![Ok(string_envelope_page(
        &[("a", ka), ("b", kb), ("c", kc)],
        Some("s1-more"),
    ))]);
    let mut pipeline1 = build_streaming_ordered_merge(&plan, &mut topology1, &op, None)
        .await
        .unwrap();
    let ids1 = drain_one(&mut pipeline1, &mut executor1).await;
    assert_eq!(ids1, vec!["a".to_owned()]);
    let state1 = pipeline1.snapshot_state().unwrap();
    drop(pipeline1);

    // ── Session 2: resume from the "a" (ka) boundary ────────────────────
    let resumed1 = round_trip_state(state1, &op);
    let mut topology2 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor2 = MockRequestExecutor::new(vec![Ok(string_envelope_page(
        &[("b", kb), ("c", kc)],
        Some("s2-more"),
    ))]);
    let mut pipeline2 = build_streaming_ordered_merge(&plan, &mut topology2, &op, Some(resumed1))
        .await
        .unwrap();
    let ids2 = drain_one(&mut pipeline2, &mut executor2).await;
    assert_eq!(ids2, vec!["b".to_owned()]);
    assert_string_param_boundary(&executor2.body_text(0), ka);
    let state2 = pipeline2.snapshot_state().unwrap();
    drop(pipeline2);

    // ── Session 3: second resume from the "b" (kb) boundary ─────────────
    let resumed2 = round_trip_state(state2, &op);
    let mut topology3 = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor3 =
        MockRequestExecutor::new(vec![Ok(string_envelope_page(&[("c", kc)], None))]);
    let mut pipeline3 = build_streaming_ordered_merge(&plan, &mut topology3, &op, Some(resumed2))
        .await
        .unwrap();
    let ids3 = drain_all(&mut pipeline3, &mut executor3).await;
    assert_eq!(ids3, vec!["c".to_owned()]);
    assert_string_param_boundary(&executor3.body_text(0), kb);

    let mut all = ids1;
    all.extend(ids2);
    all.extend(ids3);
    assert_eq!(
        all,
        vec!["a", "b", "c"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        "across two string-boundary resume cycles every row is delivered once, in order"
    );
}

// ── Full-key tie, direction-aware, multi-page, two-resume regression ───────

/// Drives one direction's full-key-tie resume regression end to end: six
/// documents, all tied on `rank`, with real (16-byte-encoded) rids in
/// `direction`'s document-ordinal order — matching how the real backend
/// returns a tied run within a partition (see `order_by::compare_rids`).
///
/// Exercises, together:
///  - a tie run spanning more than one raw backend page fetch, both before
///    (session A) and after (session C, where an entire page is discarded
///    and the fetch loop must continue to the next one) a checkpoint;
///  - two full snapshot/resume cycles (A -> B -> C);
///  - the resume filter's seek operator matching `direction` (`>` for
///    ascending, `<` for descending), with no `_rid` predicate anywhere in
///    the SQL — the tie-break is numeric and applied client-side (a base64
///    `_rid` string does not sort in document order; see
///    `order_by::ResumeFilter`).
///
/// Every one of the six rids must be delivered exactly once, in strict
/// `direction` order — proving no duplicates and no omissions.
async fn tied_full_key_resume_spans_pages_and_two_cycles(direction: SortOrder) {
    const RANK: i64 = 5;
    // Ordinals in `direction`'s document order, matching real backend
    // behavior for a full-key tie within one partition.
    let ordinals: [u64; 6] = match direction {
        SortOrder::Ascending => [10, 20, 30, 40, 50, 60],
        SortOrder::Descending => [60, 50, 40, 30, 20, 10],
    };
    let rid = |i: usize| real_rid(ordinals[i]);
    let rows: Vec<(String, i64)> = (0..6).map(|i| (rid(i), RANK)).collect();
    let refs = |range: std::ops::Range<usize>| -> Vec<(&str, i64)> {
        rows[range].iter().map(|(r, v)| (r.as_str(), *v)).collect()
    };
    let seek_operator = match direction {
        SortOrder::Ascending => ">",
        SortOrder::Descending => "<",
    };

    let op = order_by_operation_with_page_size(1);
    let plan = order_by_plan_with_direction(direction);

    // ── Session A: fresh. The tie run spans two raw backend page fetches
    // (rid(0),rid(1) then rid(2),rid(3)) before any checkpoint. ──
    let mut topology_a = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor_a = MockRequestExecutor::new(vec![
        Ok(envelope_page(&refs(0..2), Some("bp-1"))),
        Ok(envelope_page(&refs(2..4), None)),
    ]);
    let mut pipeline_a = build_streaming_ordered_merge(&plan, &mut topology_a, &op, None)
        .await
        .unwrap();
    let mut ids_a = Vec::new();
    ids_a.extend(drain_one(&mut pipeline_a, &mut executor_a).await); // rid(0): backend page 1
    ids_a.extend(drain_one(&mut pipeline_a, &mut executor_a).await); // rid(1): still buffered
    ids_a.extend(drain_one(&mut pipeline_a, &mut executor_a).await); // rid(2): fetches backend page 2
    assert_eq!(ids_a, vec![rid(0), rid(1), rid(2)]);
    assert_eq!(
        executor_a.query_bodies.len(),
        2,
        "the tie run must have spanned exactly two raw backend page fetches"
    );

    let state_a = pipeline_a.snapshot_state().unwrap();
    match &state_a {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(ranges.len(), 1);
            assert_eq!(
                ranges[0].server_continuation, None,
                "rid(3) is still buffered (unread) from backend page 2, so no plain \
                 continuation is safe to save"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline_a);

    // ── Session B: first resume. The mock can't evaluate the SQL filter, so
    // it replays the whole unfiltered tie run; the client-side discard
    // strips rid(0)..=rid(2) (already emitted), surfacing only rid(3). ──
    let resumed_a = round_trip_state(state_a, &op);
    let mut topology_b = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor_b = MockRequestExecutor::new(vec![
        Ok(envelope_page(&refs(0..4), Some("bp-2"))),
        Ok(envelope_page(&refs(4..6), None)),
    ]);
    let mut pipeline_b =
        build_streaming_ordered_merge(&plan, &mut topology_b, &op, Some(resumed_a))
            .await
            .unwrap();
    let ids_b = drain_one(&mut pipeline_b, &mut executor_b).await;
    assert_eq!(
        ids_b,
        vec![rid(3)],
        "already-emitted rid(0)..=rid(2) must be discarded client-side"
    );
    assert_eq!(
        executor_b.continuation_calls,
        vec![None],
        "a value-boundary resume starts the resume-filtered query fresh"
    );
    assert_eq!(
        executor_b.query_bodies.len(),
        1,
        "rid(3) surfaced from the first backend page alone; the second must not be fetched yet"
    );
    let body_b = executor_b.body_text(0);
    assert!(
        body_b.contains(&format!("c.rank {seek_operator}")),
        "{body_b}: the seek operator must match `direction` ({direction:?})"
    );
    assert_eq!(
        body_b.matches("_rid").count(),
        1,
        "{body_b}: `_rid` appears only in the envelope's SELECT projection; the filter \
         itself carries no `_rid` predicate — the tie-break stays client-side"
    );

    let state_b = pipeline_b.snapshot_state().unwrap();
    match &state_b {
        PipelineNodeState::StreamingOrderedMerge { ranges, .. } => {
            assert_eq!(
                ranges[0].server_continuation, None,
                "a resume-filtered child's continuation must never be snapshotted, even \
                 though its buffer is now empty and \"bp-2\" is technically still live"
            );
        }
        other => panic!("expected StreamingOrderedMerge snapshot, got {other:?}"),
    }
    drop(pipeline_b);

    // ── Session C: second resume. Backend page 1 here (rid(0)..=rid(3)) is
    // *entirely* discarded (every row ties at-or-before the rid(3)
    // boundary) — the discard must stay active across that whole page and
    // into backend page 2, where rid(4) finally survives. ──
    let resumed_b = round_trip_state(state_b, &op);
    let mut topology_c = MockTopologyProvider::new(vec![Ok(vec![resolved("", "FF", "pk-0")])]);
    let mut executor_c = MockRequestExecutor::new(vec![
        Ok(envelope_page(&refs(0..4), Some("bp-3"))),
        Ok(envelope_page(&refs(4..6), None)),
    ]);
    let mut pipeline_c =
        build_streaming_ordered_merge(&plan, &mut topology_c, &op, Some(resumed_b))
            .await
            .unwrap();
    let ids_c = drain_all(&mut pipeline_c, &mut executor_c).await;
    assert_eq!(
        ids_c,
        vec![rid(4), rid(5)],
        "already-emitted rid(0)..=rid(3) must be discarded again on this second resume"
    );
    assert_eq!(
        executor_c.query_bodies.len(),
        2,
        "the fully-discarded first page must not stop the fetch loop before the second"
    );
    assert_eq!(
        executor_c.continuation_calls.first(),
        Some(&None),
        "the second resume must start the rebuilt resume-filtered query fresh — never \
         forwarding session B's live \"bp-2\" continuation, which its snapshot never kept"
    );

    // End-to-end: every row delivered exactly once, in strict `direction`
    // rid order — proving no duplicates and no omissions.
    let mut all_ids = ids_a;
    all_ids.extend(ids_b);
    all_ids.extend(ids_c);
    assert_eq!(
        all_ids,
        (0..6).map(rid).collect::<Vec<_>>(),
        "every tied row must be delivered exactly once, in strict {direction:?} rid order"
    );
}

#[tokio::test]
async fn ascending_full_key_ties_span_backend_pages_and_survive_two_resumes() {
    tied_full_key_resume_spans_pages_and_two_cycles(SortOrder::Ascending).await;
}

#[tokio::test]
async fn descending_full_key_ties_span_backend_pages_and_survive_two_resumes() {
    tied_full_key_resume_spans_pages_and_two_cycles(SortOrder::Descending).await;
}
