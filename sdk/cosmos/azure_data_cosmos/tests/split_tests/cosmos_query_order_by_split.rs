// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live-only split-resume coverage for the cross-partition streaming
//! `ORDER BY` pipeline (`StreamingOrderedMerge`), asserting the stronger
//! invariant an `ORDER BY` query must uphold: exact global sort order is
//! preserved across a real partition split and resume, not just "the same
//! items eventually come back".
//!
//! One container is seeded once with `mergeOrder` values that *repeat*
//! across every partition key (ties spread across partitions, not a
//! globally-unique key per item). Both an ASC and a DESC query are started
//! and each mid-tie-group continuation token is captured before a single
//! forced split, then both are resumed against the post-split topology.
//! Assertions cover every seeded id appearing exactly once, monotonic keys
//! in each direction, and complete cross-range tie groups across the topology
//! change. Equal-key rows may reorder because cross-range ties use EPK range
//! order while RID direction is local to each backend range.
//!
//! Exhaustive split/merge permutations are covered in-process by
//! `azure_data_cosmos_driver`'s mock-pipeline and driver-level tests; this
//! single live test proves the same behavior against a real service split.
//! Runs only under `test_category = "split"` against split-capable
//! resources.

use super::framework;
use crate::split_tests::cosmos_query_split::force_split_and_wait;

use std::error::Error;
use std::num::NonZeroU32;
use std::time::Duration;

use azure_core::http::StatusCode;
use azure_data_cosmos::feed::ContinuationToken;
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, ContainerProperties,
        IndexingPolicy, ThroughputProperties,
    },
    options::{MaxItemCountHint, QueryOptions},
    CosmosError, CosmosStatus, Query,
};
use framework::{MockItem, TestClient, TestOptions};
use futures::StreamExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A seeded document carrying both a numeric (`mergeOrder`) and a string
/// (`sortText`) sort key, so one container/one split covers ORDER BY
/// resume for both key kinds. Queries deserialize only the fields they need
/// ([`MockItem`] for the numeric key, [`StringKeyItem`] for the string key).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SeedItem {
    id: String,
    partition_key: String,
    merge_order: usize,
    sort_text: String,
}

/// Query projection for the string-keyed ORDER BY (reads only `id` and
/// `sortText`; serde ignores the seeded `partitionKey`/`mergeOrder`).
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct StringKeyItem {
    id: String,
    sort_text: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
struct LiveJoinItem {
    id: String,
    tag: String,
    rank: i64,
}

/// A string sort key for tie group `i`: ordered by the zero-padded index
/// prefix, with a constant suffix carrying a quote, backslash, and tab so
/// the resume boundary contains SQL special characters the parameterized filter
/// must round-trip without inlining them into query text.
fn string_key(i: usize) -> String {
    format!("g{i:02}-x'\\\t")
}

/// Runs `query`'s first page only (before any split), returning the items
/// it yielded plus a re-serialized continuation token — mirroring how a
/// real caller would capture and persist a mid-fan-out checkpoint.
async fn capture_first_page<T: DeserializeOwned + Send + 'static>(
    container_client: &ContainerClient,
    query: &str,
    page_size: u32,
) -> Result<(Vec<T>, ContinuationToken), Box<dyn Error>> {
    let options = QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
    let mut pages = container_client
        .query_items::<T>(query, FeedScope::full_container(), Some(options))
        .await?
        .into_pages();
    let first_page = pages
        .next()
        .await
        .expect("query should yield at least one page before split")?;
    let items = first_page.into_items();
    let serialized = pages.to_continuation_token()?.as_str().to_owned();
    Ok((items, ContinuationToken::from_string(serialized)))
}

/// Resumes `query` from `continuation` and drains it to completion,
/// appending to `collected`. Round-trips the continuation through string
/// serialization each page, matching how a real caller persists it across
/// processes.
async fn drain_resumed<T: DeserializeOwned + Send + 'static>(
    container_client: &ContainerClient,
    query: &str,
    page_size: u32,
    mut collected: Vec<T>,
    mut continuation: Option<ContinuationToken>,
) -> Result<Vec<T>, Box<dyn Error>> {
    loop {
        let mut resume_options = QueryOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
        if let Some(t) = continuation.take() {
            resume_options = resume_options.with_continuation_token(t);
        }
        let mut pages = container_client
            .query_items::<T>(query, FeedScope::full_container(), Some(resume_options))
            .await?
            .into_pages();

        let Some(page) = pages.next().await else {
            break;
        };
        collected.extend(page?.into_items());
        let serialized = pages.to_continuation_token()?.as_str().to_owned();
        drop(pages);
        continuation = Some(ContinuationToken::from_string(serialized));
    }
    Ok(collected)
}

async fn drain_resumed_query<T: DeserializeOwned + Send + 'static>(
    container_client: &ContainerClient,
    query: &Query,
    page_size: u32,
) -> Result<Vec<T>, Box<dyn Error>> {
    let mut collected = Vec::new();
    let mut continuation = None;
    loop {
        let mut options = QueryOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
        if let Some(token) = continuation.take() {
            options = options.with_continuation_token(token);
        }
        let mut pages = container_client
            .query_items::<T>(query.clone(), FeedScope::full_container(), Some(options))
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            break;
        };
        collected.extend(page?.into_items());
        let serialized = pages.to_continuation_token()?.as_str().to_owned();
        drop(pages);
        continuation = Some(ContinuationToken::from_string(serialized));
    }
    Ok(collected)
}

/// Drains `query` until it fails, returning the error. Panics if the query
/// drains cleanly — callers use this only for queries the driver must reject.
async fn expect_query_rejection(container_client: &ContainerClient, query: &Query) -> CosmosError {
    let mut pages = container_client
        .query_items::<serde_json::Value>(query.clone(), FeedScope::full_container(), None)
        .await
        .expect("starting the query should not fail")
        .into_pages();
    while let Some(page) = pages.next().await {
        if let Err(error) = page {
            return error;
        }
    }
    panic!("expected the query to be rejected, but it drained successfully");
}

/// Cross-partition streaming `ORDER BY` resume after a live split, for both
/// directions, with repeated (tied) sort-key values, and for both a numeric
/// (`mergeOrder`) and a string (`sortText`) sort key. The string key carries
/// SQL special characters (quote, backslash, tab), so its resume boundary
/// exercises the parameterized service-safe seek filter across a real split.
///
/// Seeds `PK_COUNT` partition keys, each contributing exactly one item per
/// group `0..TIE_GROUP_COUNT` — every sort-key value therefore ties across
/// all `PK_COUNT` partition keys (and, after the split, across both physical
/// partitions) instead of each item having a globally unique key. Captures
/// an ASC and a DESC continuation token mid-tie-group for each key kind,
/// forces one live split, then resumes all four against the post-split
/// topology.
///
/// Reusing the same container and single split, also drains a mixed-direction
/// multi-column query (`mergeOrder ASC, sortText DESC`) — whose entire tuple
/// ties within every group and is therefore broken only by `_rid` — to
/// completion pre-split for an authoritative ID baseline, then asserts the
/// post-split resume returns every ID exactly once and preserves tuple order.
/// Equal tuples may reorder after the topology change.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn order_by_query_resume_across_split_preserves_global_order(
) -> Result<(), Box<dyn Error>> {
    const PK_COUNT: usize = 50;
    const TIE_GROUP_COUNT: usize = 10;
    const PAGE_SIZE: u32 = 10;
    const ASC_QUERY: &str = "SELECT * FROM c ORDER BY c.mergeOrder ASC";
    const DESC_QUERY: &str = "SELECT * FROM c ORDER BY c.mergeOrder DESC";
    const STRING_ASC_QUERY: &str = "SELECT * FROM c ORDER BY c.sortText ASC";
    const STRING_DESC_QUERY: &str = "SELECT * FROM c ORDER BY c.sortText DESC";
    // Mixed-direction, multi-column: both columns are constant within a tie
    // group, so the whole tuple ties across every partition key and only the
    // document `_rid` order (first column ASC) breaks it.
    const MULTI_QUERY: &str = "SELECT * FROM c ORDER BY c.mergeOrder ASC, c.sortText DESC";

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let composite_index = CompositeIndex::default()
                .with_property(CompositeIndexProperty::new(
                    "/mergeOrder",
                    CompositeIndexOrder::Ascending,
                ))
                .with_property(CompositeIndexProperty::new(
                    "/sortText",
                    CompositeIndexOrder::Descending,
                ));
            let mut indexing_policy =
                IndexingPolicy::default().with_composite_index(composite_index);
            indexing_policy.automatic = true;
            let properties =
                ContainerProperties::new("OrderByResumeAcrossSplit", "/partitionKey".into())
                    .with_indexing_policy(indexing_policy);
            let throughput = ThroughputProperties::manual(1000);
            let container_client = std::sync::Arc::new(
                run_context
                    .create_container(
                        db_client,
                        properties,
                        Some(CreateContainerOptions::default().with_throughput(throughput)),
                    )
                    .await?,
            );

            println!(
                "Container created; seeding {PK_COUNT} partition keys x {TIE_GROUP_COUNT} \
                 tied groups (repeated numeric + string sort values spread across partitions)"
            );
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..TIE_GROUP_COUNT {
                    let item = SeedItem {
                        id: format!("{p}-{i}"),
                        partition_key: partition_key.clone(),
                        merge_order: i,
                        sort_text: string_key(i),
                    };
                    match container_client
                        .create_item(item.partition_key.clone(), &item.id.clone(), item, None)
                        .await
                    {
                        Ok(_) => {}
                        // A retried transport 503 can find the item already
                        // committed; unique ids make Conflict here benign.
                        Err(error) if error.status().status_code() == StatusCode::Conflict => {}
                        Err(error) => return Err(error.into()),
                    }
                }
            }

            let ranges_before = container_client.read_feed_ranges(None).await?;
            assert!(
                !ranges_before.is_empty(),
                "expected at least one physical partition before split, got {}",
                ranges_before.len()
            );
            let partitions_before = ranges_before.len();

            // Capture ASC + DESC tokens mid-tie-group (each group has more
            // members than one page) for BOTH the numeric and string keys,
            // before the single forced split below.
            let (asc_first, asc_token) =
                capture_first_page::<MockItem>(&container_client, ASC_QUERY, PAGE_SIZE).await?;
            let (desc_first, desc_token) =
                capture_first_page::<MockItem>(&container_client, DESC_QUERY, PAGE_SIZE).await?;
            let (str_asc_first, str_asc_token) =
                capture_first_page::<StringKeyItem>(&container_client, STRING_ASC_QUERY, PAGE_SIZE)
                    .await?;
            let (str_desc_first, str_desc_token) = capture_first_page::<StringKeyItem>(
                &container_client,
                STRING_DESC_QUERY,
                PAGE_SIZE,
            )
            .await?;

            // Fully drain the multi-column query on the pre-split topology for
            // the authoritative expected order, then capture a mid-tie
            // continuation to resume across the split and compare against it.
            let multi_baseline = drain_resumed::<SeedItem>(
                &container_client,
                MULTI_QUERY,
                PAGE_SIZE,
                Vec::new(),
                None,
            )
            .await?;
            let multi_baseline_ids: Vec<String> =
                multi_baseline.iter().map(|item| item.id.clone()).collect();
            let (multi_first, multi_token) =
                capture_first_page::<SeedItem>(&container_client, MULTI_QUERY, PAGE_SIZE).await?;

            println!(
                "Captured numeric ASC/DESC ({}/{}) and string ASC/DESC ({}/{}) tokens; forcing \
                 the single split all four will resume across",
                asc_first.len(),
                desc_first.len(),
                str_asc_first.len(),
                str_desc_first.len(),
            );
            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, \
                 after={partitions_after}"
            );

            // Resume each direction's continuation independently against
            // the now-split topology.
            let asc_all = drain_resumed::<MockItem>(
                &container_client,
                ASC_QUERY,
                PAGE_SIZE,
                asc_first,
                Some(asc_token),
            )
            .await?;
            let desc_all = drain_resumed::<MockItem>(
                &container_client,
                DESC_QUERY,
                PAGE_SIZE,
                desc_first,
                Some(desc_token),
            )
            .await?;
            let str_asc_all = drain_resumed::<StringKeyItem>(
                &container_client,
                STRING_ASC_QUERY,
                PAGE_SIZE,
                str_asc_first,
                Some(str_asc_token),
            )
            .await?;
            let str_desc_all = drain_resumed::<StringKeyItem>(
                &container_client,
                STRING_DESC_QUERY,
                PAGE_SIZE,
                str_desc_first,
                Some(str_desc_token),
            )
            .await?;
            let multi_all = drain_resumed::<SeedItem>(
                &container_client,
                MULTI_QUERY,
                PAGE_SIZE,
                multi_first,
                Some(multi_token),
            )
            .await?;

            // ── Every seeded id exactly once, in each direction ──────────
            let mut expected_ids: Vec<String> = (0..PK_COUNT)
                .flat_map(|p| (0..TIE_GROUP_COUNT).map(move |i| format!("{p}-{i}")))
                .collect();
            expected_ids.sort();

            let mut asc_ids: Vec<String> = asc_all.iter().map(|item| item.id.clone()).collect();
            asc_ids.sort();
            assert_eq!(
                asc_ids, expected_ids,
                "ASC drain must return every seeded id exactly once (no duplicates or losses \
                 across the split)"
            );

            let mut desc_ids: Vec<String> = desc_all.iter().map(|item| item.id.clone()).collect();
            desc_ids.sort();
            assert_eq!(
                desc_ids, expected_ids,
                "DESC drain must return every seeded id exactly once (no duplicates or losses \
                 across the split)"
            );

            // ── Monotonic keys per direction ─────────────────────────────
            let asc_keys: Vec<usize> = asc_all.iter().map(|item| item.merge_order).collect();
            assert!(
                asc_keys.windows(2).all(|w| w[0] <= w[1]),
                "ASC mergeOrder must be non-decreasing across the split/resume boundary: \
                 {asc_keys:?}"
            );
            let desc_keys: Vec<usize> = desc_all.iter().map(|item| item.merge_order).collect();
            assert!(
                desc_keys.windows(2).all(|w| w[0] >= w[1]),
                "DESC mergeOrder must be non-increasing across the split/resume boundary: \
                 {desc_keys:?}"
            );

            // Cross-range ties use EPK range order, while RID direction stays
            // local to each backend range. Only group completeness is stable
            // across ASC/DESC and topology changes.
            for group in 0..TIE_GROUP_COUNT {
                let asc_group: Vec<&str> = asc_all
                    .iter()
                    .filter(|item| item.merge_order == group)
                    .map(|item| item.id.as_str())
                    .collect();
                let desc_group: Vec<&str> = desc_all
                    .iter()
                    .filter(|item| item.merge_order == group)
                    .map(|item| item.id.as_str())
                    .collect();
                assert_eq!(
                    asc_group.len(),
                    PK_COUNT,
                    "tie group mergeOrder={group} must contain exactly one item per partition key"
                );
                assert_eq!(desc_group.len(), PK_COUNT);
            }

            // ── String-keyed ORDER BY resume (service-safe boundaries) ────
            // The same invariants for the special-character-laden string key,
            // proving the parameterized resume filter round-trips a boundary
            // containing a quote, backslash, and tab across a real split.
            let mut str_asc_ids: Vec<String> =
                str_asc_all.iter().map(|item| item.id.clone()).collect();
            str_asc_ids.sort();
            assert_eq!(
                str_asc_ids, expected_ids,
                "string ASC drain must return every seeded id exactly once across the split"
            );
            let mut str_desc_ids: Vec<String> =
                str_desc_all.iter().map(|item| item.id.clone()).collect();
            str_desc_ids.sort();
            assert_eq!(
                str_desc_ids, expected_ids,
                "string DESC drain must return every seeded id exactly once across the split"
            );

            let str_asc_keys: Vec<&str> = str_asc_all
                .iter()
                .map(|item| item.sort_text.as_str())
                .collect();
            assert!(
                str_asc_keys.windows(2).all(|w| w[0] <= w[1]),
                "string ASC sortText must be non-decreasing across the split/resume boundary: \
                 {str_asc_keys:?}"
            );
            let str_desc_keys: Vec<&str> = str_desc_all
                .iter()
                .map(|item| item.sort_text.as_str())
                .collect();
            assert!(
                str_desc_keys.windows(2).all(|w| w[0] >= w[1]),
                "string DESC sortText must be non-increasing across the split/resume boundary: \
                 {str_desc_keys:?}"
            );

            for group in 0..TIE_GROUP_COUNT {
                let key = string_key(group);
                let asc_group: Vec<&str> = str_asc_all
                    .iter()
                    .filter(|item| item.sort_text == key)
                    .map(|item| item.id.as_str())
                    .collect();
                let desc_group: Vec<&str> = str_desc_all
                    .iter()
                    .filter(|item| item.sort_text == key)
                    .map(|item| item.id.as_str())
                    .collect();
                assert_eq!(
                    asc_group.len(),
                    PK_COUNT,
                    "string tie group {group} must contain exactly one item per partition key"
                );
                assert_eq!(desc_group.len(), PK_COUNT);
            }

            // ── Multi-column resume preserves tuple order and exact-once IDs ─
            assert_eq!(
                multi_baseline_ids.len(),
                PK_COUNT * TIE_GROUP_COUNT,
                "multi-column baseline must observe every seeded id exactly once pre-split"
            );
            let multi_first_col: Vec<usize> =
                multi_baseline.iter().map(|item| item.merge_order).collect();
            assert!(
                multi_first_col.windows(2).all(|w| w[0] <= w[1]),
                "multi-column baseline's first column (mergeOrder ASC) must be non-decreasing: \
                 {multi_first_col:?}"
            );
            let mut multi_ids: Vec<String> = multi_all.iter().map(|item| item.id.clone()).collect();
            let mut expected_multi_ids = multi_baseline_ids;
            multi_ids.sort();
            expected_multi_ids.sort();
            assert_eq!(
                multi_ids, expected_multi_ids,
                "multi-column mixed-direction resume must return every row exactly once"
            );
            let multi_keys: Vec<(usize, &str)> = multi_all
                .iter()
                .map(|item| (item.merge_order, item.sort_text.as_str()))
                .collect();
            assert!(
                multi_keys.windows(2).all(|window| {
                    window[0].0 < window[1].0
                        || (window[0].0 == window[1].0 && window[0].1 >= window[1].1)
                }),
                "multi-column keys must remain ordered by mergeOrder ASC, sortText DESC"
            );

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}

/// Real-account query-shape matrix that does not force a split. Page size one
/// serializes and recreates the iterator at every boundary. Also asserts that
/// a sort key evaluating to an array or object is rejected client-side rather
/// than emitting an order the merge cannot reproduce.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn order_by_live_mixed_types_and_join_resume_matrix() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let composite_index = CompositeIndex::default()
                .with_property(CompositeIndexProperty::new(
                    "/primary",
                    CompositeIndexOrder::Ascending,
                ))
                .with_property(CompositeIndexProperty::new(
                    "/secondary",
                    CompositeIndexOrder::Descending,
                ));
            let mut indexing_policy =
                IndexingPolicy::default().with_composite_index(composite_index);
            indexing_policy.automatic = true;
            let properties = ContainerProperties::new("OrderByLiveMatrix", "/partitionKey".into())
                .with_indexing_policy(indexing_policy);
            let container_client = run_context
                .create_container(
                    db_client,
                    properties,
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(1000)),
                    ),
                )
                .await?;

            let mixed_values = [
                ("mixed-undefined", None),
                ("mixed-null", Some(serde_json::Value::Null)),
                ("mixed-false", Some(serde_json::json!(false))),
                ("mixed-true", Some(serde_json::json!(true))),
                ("mixed-number", Some(serde_json::json!(42))),
                ("mixed-string", Some(serde_json::json!("value"))),
            ];
            for (index, (id, sort_key)) in mixed_values.iter().enumerate() {
                let partition_key = format!("mixed-pk-{index}");
                let mut document = serde_json::json!({
                    "id": id,
                    "partitionKey": partition_key,
                    "testCase": "mixed"
                });
                if let Some(sort_key) = sort_key {
                    document
                        .as_object_mut()
                        .unwrap()
                        .insert("sortKey".to_owned(), sort_key.clone());
                }
                container_client
                    .create_item(partition_key, id, document, None)
                    .await?;
            }

            // Array and object sort keys live under their own `testCase` so the
            // scalar ordering above can still drain: the driver rejects the
            // whole query as soon as one complex key reaches the merge.
            for (index, sort_key) in [serde_json::json!([1]), serde_json::json!({"a": 1})]
                .iter()
                .enumerate()
            {
                let id = format!("complex-{index}");
                let partition_key = format!("complex-pk-{index}");
                container_client
                    .create_item(
                        &partition_key,
                        &id,
                        serde_json::json!({
                            "id": id,
                            "partitionKey": partition_key,
                            "testCase": "mixedComplex",
                            "sortKey": sort_key
                        }),
                        None,
                    )
                    .await?;
            }

            let expected_asc = [
                "mixed-undefined",
                "mixed-null",
                "mixed-false",
                "mixed-true",
                "mixed-number",
                "mixed-string",
            ];
            let asc = drain_resumed_query::<serde_json::Value>(
                &container_client,
                &Query::from("SELECT * FROM c WHERE c.testCase = 'mixed' ORDER BY c.sortKey ASC"),
                1,
            )
            .await?;
            let asc_ids: Vec<&str> = asc
                .iter()
                .map(|item| item["id"].as_str().unwrap())
                .collect();
            assert_eq!(asc_ids, expected_asc);

            let desc = drain_resumed_query::<serde_json::Value>(
                &container_client,
                &Query::from("SELECT * FROM c WHERE c.testCase = 'mixed' ORDER BY c.sortKey DESC"),
                1,
            )
            .await?;
            let desc_ids: Vec<&str> = desc
                .iter()
                .map(|item| item["id"].as_str().unwrap())
                .collect();
            assert_eq!(desc_ids, expected_asc.into_iter().rev().collect::<Vec<_>>());

            // Complex sort keys are rejected client-side in both directions:
            // the merge cannot reproduce the service's ordering of arrays and
            // objects, so it refuses rather than emitting a wrong order.
            for direction in ["ASC", "DESC"] {
                let error = expect_query_rejection(
                    &container_client,
                    &Query::from(format!(
                        "SELECT * FROM c WHERE c.testCase = 'mixedComplex' ORDER BY c.sortKey {direction}"
                    )),
                )
                .await;
                assert_eq!(
                    error.status(),
                    CosmosStatus::CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED,
                    "ORDER BY {direction} over a complex sort key must be rejected, got: {error}"
                );
            }

            for (index, secondary) in ["z", "m", "m", "b", "a", "x"].iter().enumerate() {
                let id = format!("undefined-{index}");
                let partition_key = format!("undefined-pk-{index}");
                let document = serde_json::json!({
                    "id": id,
                    "partitionKey": partition_key,
                    "testCase": "undefinedMulti",
                    "secondary": secondary
                });
                container_client
                    .create_item(partition_key, &id, document, None)
                    .await?;
            }
            for index in 0..3 {
                let id = format!("defined-{index}");
                let partition_key = format!("defined-pk-{index}");
                let document = serde_json::json!({
                    "id": id,
                    "partitionKey": partition_key,
                    "testCase": "undefinedMulti",
                    "primary": index,
                    "secondary": format!("s{index}")
                });
                container_client
                    .create_item(partition_key, &id, document, None)
                    .await?;
            }

            for query_text in [
                "SELECT * FROM c WHERE c.testCase = 'undefinedMulti' \
                 ORDER BY c.primary ASC, c.secondary DESC",
                "SELECT * FROM c WHERE c.testCase = 'undefinedMulti' \
                 ORDER BY c.primary DESC, c.secondary ASC",
            ] {
                let query = Query::from(query_text);
                let baseline =
                    drain_resumed_query::<serde_json::Value>(&container_client, &query, 100)
                        .await?;
                let baseline_ids: Vec<&str> = baseline
                    .iter()
                    .map(|item| item["id"].as_str().unwrap())
                    .collect();
                let mut actual_ids = baseline_ids.clone();
                actual_ids.sort_unstable();
                let mut expected_ids: Vec<String> = (0..6)
                    .map(|index| format!("undefined-{index}"))
                    .chain((0..3).map(|index| format!("defined-{index}")))
                    .collect();
                expected_ids.sort_unstable();
                assert_eq!(
                    actual_ids,
                    expected_ids.iter().map(String::as_str).collect::<Vec<_>>()
                );

                let undefined_rows: Vec<&serde_json::Value> = baseline
                    .iter()
                    .filter(|item| item.get("primary").is_none())
                    .collect();
                assert_eq!(undefined_rows.len(), 6);
                let undefined_secondaries: Vec<&str> = undefined_rows
                    .iter()
                    .map(|item| item["secondary"].as_str().unwrap())
                    .collect();
                if query_text.contains("primary ASC") {
                    assert!(baseline[..6]
                        .iter()
                        .all(|item| item.get("primary").is_none()));
                    assert!(undefined_secondaries
                        .windows(2)
                        .all(|window| window[0] >= window[1]));
                } else {
                    assert!(baseline[baseline.len() - 6..]
                        .iter()
                        .all(|item| item.get("primary").is_none()));
                    assert!(undefined_secondaries
                        .windows(2)
                        .all(|window| window[0] <= window[1]));
                }
                for page_size in [1, 5, 100] {
                    let resumed = drain_resumed_query::<serde_json::Value>(
                        &container_client,
                        &query,
                        page_size,
                    )
                    .await?;
                    let resumed_ids: Vec<&str> = resumed
                        .iter()
                        .map(|item| item["id"].as_str().unwrap())
                        .collect();
                    assert_eq!(
                        resumed_ids, baseline_ids,
                        "undefined-leading multi-column ORDER BY diverged at page size {page_size}"
                    );
                }
            }

            let join_documents = [
                serde_json::json!({
                    "id": "join-filtered",
                    "partitionKey": "join-pk-0",
                    "testCase": "join",
                    "rank": 0,
                    "tags": ["ignored"]
                }),
                serde_json::json!({
                    "id": "join-a",
                    "partitionKey": "join-pk-1",
                    "testCase": "join",
                    "rank": 1,
                    "tags": ["a", "b", "c"]
                }),
                serde_json::json!({
                    "id": "join-b",
                    "partitionKey": "join-pk-2",
                    "testCase": "join",
                    "rank": 2,
                    "tags": ["d", "e"]
                }),
            ];
            for document in join_documents {
                let id = document["id"].as_str().unwrap().to_owned();
                let partition_key = document["partitionKey"].as_str().unwrap().to_owned();
                container_client
                    .create_item(partition_key, &id, document, None)
                    .await?;
            }

            let join_query = Query::from(
                "SELECT VALUE {\"id\": c.id, \"tag\": t, \"rank\": c.rank} \
                 FROM c JOIN t IN c.tags \
                 WHERE c.testCase = @testCase AND c.rank >= @minRank \
                 ORDER BY c.rank ASC",
            )
            .with_parameter("@testCase", "join")?
            .with_parameter("@minRank", 1)?;
            let baseline =
                drain_resumed_query::<LiveJoinItem>(&container_client, &join_query, 100).await?;
            let resumed =
                drain_resumed_query::<LiveJoinItem>(&container_client, &join_query, 1).await?;
            assert_eq!(resumed, baseline);
            assert_eq!(resumed.len(), 5);
            assert!(resumed.iter().all(|item| item.rank >= 1));
            assert_eq!(
                resumed
                    .iter()
                    .map(|item| item.tag.as_str())
                    .collect::<Vec<_>>(),
                vec!["a", "b", "c", "d", "e"]
            );

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(20 * 60))),
    )
    .await
}
