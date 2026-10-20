// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live split-resume coverage for cross-partition `TOP` and `OFFSET` / `LIMIT`.
//!
//! The test seeds a known set into a single physical partition, captures
//! continuations for result windows, forces a real split, and resumes against
//! the child ranges. It also issues fresh queries after the split, exercising
//! the Gateway query-plan rewrite path. Cross-partition order is unspecified
//! without `ORDER BY`, so assertions verify the stable contract: global window
//! cardinality, no duplicate IDs, and membership in the seeded set.

use super::cosmos_query_split::force_split_and_wait;
use super::framework;

use std::collections::HashSet;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::{ContinuationToken, FeedScope};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::{CreateContainerOptions, MaxItemCountHint, QueryOptions};
use framework::{MockItem, TestClient, TestOptions};
use futures::StreamExt;

const PK_COUNT: usize = 50;
const ITEMS_PER_PK: usize = 10;
const TOTAL_ITEMS: usize = PK_COUNT * ITEMS_PER_PK;
const PAGE_SIZE: u32 = 2;

/// Starts a query, returns its first page's IDs, and round-trips its
/// continuation through the public string form before the later split.
async fn capture_first_page(
    container_client: &ContainerClient,
    query: &str,
) -> Result<(Vec<String>, ContinuationToken), Box<dyn Error>> {
    let options = QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    let mut pages = container_client
        .query_items::<MockItem>(query, FeedScope::full_container(), Some(options))
        .await?
        .into_pages();
    let first_page = pages
        .next()
        .await
        .expect("result window must return a first page before the split")?;
    let ids = first_page
        .into_items()
        .into_iter()
        .map(|item| item.id)
        .collect();
    let serialized = pages.to_continuation_token()?.as_str().to_owned();
    Ok((ids, ContinuationToken::from_string(serialized)))
}

/// Drains `query` after `continuation`, creating a fresh page iterator for
/// every page. This mirrors a caller that persists the SDK continuation across
/// process boundaries and makes the post-split resume path explicit.
async fn drain_from_continuation(
    container_client: &ContainerClient,
    query: &str,
    mut continuation: Option<ContinuationToken>,
    mut collected: Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    loop {
        let mut options = QueryOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
        if let Some(token) = continuation.take() {
            options = options.with_continuation_token(token);
        }

        let mut pages = container_client
            .query_items::<MockItem>(query, FeedScope::full_container(), Some(options))
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            break;
        };
        collected.extend(page?.into_items().into_iter().map(|item| item.id));

        let serialized = pages.to_continuation_token()?.as_str().to_owned();
        drop(pages);
        continuation = Some(ContinuationToken::from_string(serialized));
    }
    Ok(collected)
}

fn assert_global_window(
    label: &str,
    ids: &[String],
    expected_count: usize,
    seeded_ids: &HashSet<String>,
) {
    assert_eq!(
        ids.len(),
        expected_count,
        "{label}: expected {expected_count} results, got {}: {ids:?}",
        ids.len()
    );

    let unique_ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
    assert_eq!(
        unique_ids.len(),
        ids.len(),
        "{label}: result contains duplicate IDs: {ids:?}"
    );
    assert!(
        ids.iter().all(|id| seeded_ids.contains(id)),
        "{label}: result contains an ID that was not seeded: {ids:?}"
    );
}

/// `TOP` and `OFFSET` / `LIMIT` keep their global result-window semantics
/// through a real physical partition split.
///
/// Fresh post-split queries prove Gateway emits usable `OffsetAndLimit` and
/// `Top` plan shapes for the child ranges. Pre-split continuations prove that
/// the driver's `SkipTake` remaining skip/take state resumes without duplicate
/// or lost items when the parent range is replaced by child ranges.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn skip_take_queries_preserve_global_windows_across_split() -> Result<(), Box<dyn Error>>
{
    const TOP_QUERY: &str = "SELECT TOP 17 * FROM c";
    const OFFSET_LIMIT_QUERY: &str = "SELECT * FROM c OFFSET 2 LIMIT 17";
    const TAIL_OFFSET_LIMIT_QUERY: &str = "SELECT * FROM c OFFSET 497 LIMIT 17";

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("SkipTakeAcrossSplit", "/partitionKey".into());
            let container_client = Arc::new(
                run_context
                    .create_container(
                        db_client,
                        properties,
                        Some(
                            CreateContainerOptions::default()
                                .with_throughput(ThroughputProperties::manual(1000)),
                        ),
                    )
                    .await?,
            );

            let mut seeded_ids = HashSet::with_capacity(TOTAL_ITEMS);
            for partition in 0..PK_COUNT {
                let partition_key = format!("pk{partition}");
                for item_index in 0..ITEMS_PER_PK {
                    let item = MockItem {
                        id: format!("{partition}-{item_index}"),
                        partition_key: partition_key.clone(),
                        merge_order: partition * ITEMS_PER_PK + item_index,
                    };
                    seeded_ids.insert(item.id.clone());
                    container_client
                        .create_item(item.partition_key.clone(), &item.id.clone(), item, None)
                        .await?;
                }
            }
            assert_eq!(seeded_ids.len(), TOTAL_ITEMS);

            let partitions_before = container_client.read_feed_ranges(None).await?.len();
            assert!(
                partitions_before >= 1,
                "expected at least one physical partition before the split"
            );

            let (top_first, top_token) = capture_first_page(&container_client, TOP_QUERY).await?;
            let (offset_first, offset_token) =
                capture_first_page(&container_client, OFFSET_LIMIT_QUERY).await?;
            let (tail_first, tail_token) =
                capture_first_page(&container_client, TAIL_OFFSET_LIMIT_QUERY).await?;

            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, \
                 after={partitions_after}"
            );

            let resumed_top =
                drain_from_continuation(&container_client, TOP_QUERY, Some(top_token), top_first)
                    .await?;
            assert_global_window("resumed TOP 17", &resumed_top, 17, &seeded_ids);

            let resumed_offset = drain_from_continuation(
                &container_client,
                OFFSET_LIMIT_QUERY,
                Some(offset_token),
                offset_first,
            )
            .await?;
            assert_global_window(
                "resumed OFFSET 2 LIMIT 17",
                &resumed_offset,
                17,
                &seeded_ids,
            );

            let resumed_tail = drain_from_continuation(
                &container_client,
                TAIL_OFFSET_LIMIT_QUERY,
                Some(tail_token),
                tail_first,
            )
            .await?;
            assert_global_window("resumed OFFSET 497 LIMIT 17", &resumed_tail, 3, &seeded_ids);

            for (label, query, expected_count) in [
                ("fresh TOP 0", "SELECT TOP 0 * FROM c", 0),
                ("fresh TOP 1", "SELECT TOP 1 * FROM c", 1),
                ("fresh TOP 17", TOP_QUERY, 17),
                (
                    "fresh OFFSET 0 LIMIT 17",
                    "SELECT * FROM c OFFSET 0 LIMIT 17",
                    17,
                ),
                ("fresh OFFSET 2 LIMIT 17", OFFSET_LIMIT_QUERY, 17),
                ("fresh OFFSET 497 LIMIT 17", TAIL_OFFSET_LIMIT_QUERY, 3),
                (
                    "fresh past-end OFFSET",
                    "SELECT * FROM c OFFSET 500 LIMIT 17",
                    0,
                ),
                ("fresh LIMIT 0", "SELECT * FROM c OFFSET 1 LIMIT 0", 0),
            ] {
                let ids =
                    drain_from_continuation(&container_client, query, None, Vec::new()).await?;
                assert_global_window(label, &ids, expected_count, &seeded_ids);
            }

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}
