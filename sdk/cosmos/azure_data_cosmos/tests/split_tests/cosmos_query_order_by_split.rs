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
//! in each direction, and — since ties can't be resolved by `mergeOrder`
//! alone — that DESC's tied-group order is the exact reverse of ASC's,
//! proving both share the same underlying document-`_rid` total order
//! (see `driver::dataflow::order_by::compare_rids`) rather than arbitrary
//! or storage-order noise that happens to differ per direction.
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
};
use framework::{MockItem, TestClient, TestOptions};
use futures::StreamExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A seeded document carrying both a numeric (`mergeOrder`) and a string
/// (`sortText`) sort key, so one container/one split covers ORDER BY
/// resume for both key kinds. Queries deserialize only the fields they need
/// ([`MockItem`] for the numeric key, [`StringKeyItem`] for the string key).
#[derive(Serialize, Clone, Debug)]
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

/// A seeded document carrying an array-valued sort key, for the dedicated
/// array-keyed `ORDER BY` split-resume test.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ArraySeedItem {
    id: String,
    partition_key: String,
    array_key: Vec<i64>,
}

/// Projection for the array-keyed `ORDER BY` (reads only `id`).
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct ArrayKeyItem {
    id: String,
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
/// completion pre-split for an authoritative baseline, then asserts its
/// post-split resume reproduces that exact sequence, not merely the same ids.
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
            let properties =
                ContainerProperties::new("OrderByResumeAcrossSplit", "/partitionKey".into())
                    .with_indexing_policy(
                        IndexingPolicy::default().with_composite_index(composite_index),
                    );
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
            let multi_baseline = drain_resumed::<MockItem>(
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
                capture_first_page::<MockItem>(&container_client, MULTI_QUERY, PAGE_SIZE).await?;

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
            let multi_all = drain_resumed::<MockItem>(
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

            // ── Tie order is a real, direction-symmetric rid order ───────
            // For every tied `mergeOrder` group, DESC's item sequence must
            // be the *exact reverse* of ASC's: both are broken by the same
            // document-`_rid` total order, just applied in opposite
            // directions. Anything else (arbitrary order, storage-order
            // noise, or a direction-dependent tie-break bug) would make the
            // two sequences disagree once reversed.
            for group in 0..TIE_GROUP_COUNT {
                let asc_group: Vec<&str> = asc_all
                    .iter()
                    .filter(|item| item.merge_order == group)
                    .map(|item| item.id.as_str())
                    .collect();
                let mut desc_group: Vec<&str> = desc_all
                    .iter()
                    .filter(|item| item.merge_order == group)
                    .map(|item| item.id.as_str())
                    .collect();
                desc_group.reverse();
                assert_eq!(
                    asc_group, desc_group,
                    "tie group mergeOrder={group}: DESC order reversed must equal ASC order \
                     (both broken by the same rid total order)"
                );
                assert_eq!(
                    asc_group.len(),
                    PK_COUNT,
                    "tie group mergeOrder={group} must contain exactly one item per partition key"
                );
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
                let mut desc_group: Vec<&str> = str_desc_all
                    .iter()
                    .filter(|item| item.sort_text == key)
                    .map(|item| item.id.as_str())
                    .collect();
                desc_group.reverse();
                assert_eq!(
                    asc_group, desc_group,
                    "string tie group {group}: DESC order reversed must equal ASC order \
                     (both broken by the same rid total order)"
                );
                assert_eq!(
                    asc_group.len(),
                    PK_COUNT,
                    "string tie group {group} must contain exactly one item per partition key"
                );
            }

            // ── Multi-column resume equals the pre-split baseline exactly ─
            // The strongest cross-split invariant: a full tuple tie is resolved
            // only by `_rid`, and `_rid`s are stable across a split, so the exact
            // pre-split drain order must survive the split/resume — not merely the
            // same id set.
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
            let multi_ids: Vec<String> = multi_all.iter().map(|item| item.id.clone()).collect();
            assert_eq!(
                multi_ids, multi_baseline_ids,
                "multi-column mixed-direction resume across the split must reproduce the exact \
                 pre-split drain order (full tuple ties broken only by `_rid`)"
            );

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}

/// A dedicated live check that a cross-partition streaming `ORDER BY` on an
/// **array-valued** sort key resumes across a real split with no omissions
/// or duplicates.
///
/// Array/object sort values are ordered by the backend's bounded hash (there
/// is no documented structural order), and Rust sends that boundary to the
/// backend as the structured `resumeFilter` (its complex value serialized as
/// `{"type":"array","low":..,"high":..}`). This asserts the resume-correctness
/// invariant — every seeded id returned exactly once across the split — rather
/// than an exact global sequence (which is undefined for complex keys).
///
/// By design this exercises the **saved-token** resume path: the first page is
/// drained and its continuation token captured, the split is then forced while
/// *no query is active*, and the query is reissued from the saved token. That
/// token's complex boundary replays through the structured `resumeFilter`,
/// whose backend `DistinctHash` seek excludes already-emitted rows across the
/// new topology — so the test is expected to pass. Live in-process splits —
/// where the split child forwards its own backend continuation into each
/// replacement, so no client-side discard is needed — are covered exhaustively
/// by the `streaming_ordered_merge` unit tests.
///
/// Kept separate from `order_by_query_resume_across_split_preserves_global_order`
/// so its default (index-everything) container can serve an array-path
/// `ORDER BY` without a hand-tuned composite index, and so a service that does
/// not support ordering by an array value surfaces here without destabilizing
/// the scalar coverage. Gated on `test_category = "split"`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn order_by_array_key_resume_across_split_has_no_omissions() -> Result<(), Box<dyn Error>>
{
    const PK_COUNT: usize = 30;
    const TIE_GROUP_COUNT: usize = 5;
    const PAGE_SIZE: u32 = 10;
    const ARRAY_QUERY: &str = "SELECT * FROM c ORDER BY c.arrayKey ASC";

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Default (index-everything) indexing so the array sort path is
            // servable without a hand-tuned composite index.
            let properties = ContainerProperties::new(
                "OrderByArrayKeyResumeAcrossSplit",
                "/partitionKey".into(),
            );
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

            // Each group's array value `[i]` ties across every partition key,
            // so a tie run spreads across both post-split physical partitions.
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..TIE_GROUP_COUNT {
                    let item = ArraySeedItem {
                        id: format!("{p}-{i}"),
                        partition_key: partition_key.clone(),
                        array_key: vec![i as i64],
                    };
                    match container_client
                        .create_item(item.partition_key.clone(), &item.id.clone(), item, None)
                        .await
                    {
                        Ok(_) => {}
                        Err(error) if error.status().status_code() == StatusCode::Conflict => {}
                        Err(error) => return Err(error.into()),
                    }
                }
            }

            let ranges_before = container_client.read_feed_ranges(None).await?;
            let partitions_before = ranges_before.len();

            let (array_first, array_token) =
                capture_first_page::<ArrayKeyItem>(&container_client, ARRAY_QUERY, PAGE_SIZE)
                    .await?;

            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, \
                 after={partitions_after}"
            );

            let array_all = drain_resumed::<ArrayKeyItem>(
                &container_client,
                ARRAY_QUERY,
                PAGE_SIZE,
                array_first,
                Some(array_token),
            )
            .await?;

            let mut expected_ids: Vec<String> = (0..PK_COUNT)
                .flat_map(|p| (0..TIE_GROUP_COUNT).map(move |i| format!("{p}-{i}")))
                .collect();
            expected_ids.sort();
            let mut array_ids: Vec<String> = array_all.iter().map(|item| item.id.clone()).collect();
            array_ids.sort();
            assert_eq!(
                array_ids, expected_ids,
                "array-keyed ORDER BY resume across the split must return every seeded id \
                 exactly once (no omissions or duplicates)"
            );
            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}
