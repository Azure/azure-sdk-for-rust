// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::split_tests::framework::InconclusiveError;

use super::framework;

use std::error::Error;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use azure_data_cosmos::{
    models::{ContainerProperties, ThroughputProperties},
    options::{MaxItemCountHint, QueryOptions},
    query::FeedScope,
    ContinuationToken, CreateContainerOptions, ReadFeedRangesOptions,
};
use framework::{MockItem, TestClient, TestOptions};
use futures::{StreamExt, TryStreamExt};

const PAGE_SIZE: u32 = 5;
const PARTITION_KEY_COUNT: usize = 5;
const ITEMS_PER_PARTITION_KEY: usize = 5;

// We wait 10 minutes for the split to occur. If it doesn't, we "fail" the test,
// but with a clear message indicating that it only failed because the split didn't happen in time, rather than panicking on some other assumption later in the test.
// Splits _often_ complete in time, but when they don't, we don't necessarily want to block any given PR.
const SPLIT_POLL_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const SPLIT_POLL_INTERVAL: Duration = Duration::from_secs(15);

#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn query_continuation_survives_partition_split() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create a container with a single physical partition by
            // pinning throughput to 1000 RU/s.
            let properties =
                ContainerProperties::new("QuerySplitContainer", "/partitionKey".into());
            let throughput = ThroughputProperties::manual(1000);
            let container_client = run_context
                .create_container(
                    db_client,
                    properties,
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            println!("Container created with 1000 RU/s throughput to ensure single physical partition, inserting docs");

            // Seed enough items across multiple PK values that a
            // split can actually redistribute documents and that a page size
            // of PAGE_SIZE yields at least 3 pages.
            let mut expected_ids: Vec<String> = Vec::new();
            for p in 0..PARTITION_KEY_COUNT {
                let partition_key = format!("partition{p}");
                for i in 0..ITEMS_PER_PARTITION_KEY {
                    let item = MockItem {
                        id: format!("{p}-{i}"),
                        partition_key: partition_key.clone(),
                        merge_order: p * ITEMS_PER_PARTITION_KEY + i,
                    };
                    expected_ids.push(item.id.clone());
                    container_client
                        .create_item(item.partition_key.clone(), &item.id.clone(), item, None)
                        .await?;
                }
            }
            assert!(
                expected_ids.len() >= (PAGE_SIZE as usize) * 3,
                "need at least 3 pages worth of items, have {}",
                expected_ids.len()
            );

            println!("Documents inserted, starting query with pagination to capture continuation token");

            // Confirm single physical partition.
            let ranges_before = container_client.read_feed_ranges(None).await?;
            assert!(
                ranges_before.len() == 1,
                "expected single physical partition before split, got {}",
                ranges_before.len()
            );

            // Fetch a single page and capture a continuation token.
            let mut collected: Vec<String> = Vec::new();
            let saved_token = {
                let initial_options = QueryOptions::default().with_max_item_count(
                    MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()),
                );
                let mut pages = container_client
                    .query_items::<MockItem>(
                        "SELECT * FROM c",
                        FeedScope::full_container(),
                        Some(initial_options),
                    )
                    .await?
                    .into_pages();

                // There's a small chance we get empty pages from the backend, so loop until we get at least _something_ back.
                // That way we know we've got a continuation token representing actual progress through the item stream.
                while collected.len() == 0 {
                    let first_page = pages
                        .next()
                        .await
                        .expect("query should yield at least one page before split")?;
                    for item in first_page.into_items() {
                        collected.push(item.id);
                    }
                }

                // Round-trip through string form to mirror real usage (e.g.
                // persisting the token across processes).
                let token = pages.to_continuation_token()?;
                let serialized = token.as_str().to_owned();

                // Assert that we've just got the first five items:
                let expected_first_page: Vec<String> = expected_ids
                    .iter()
                    .take(collected.len())
                    .cloned()
                    .collect();
                assert_eq!(
                    collected, expected_first_page,
                    "first page should contain the first {} items in id sort order",
                    PAGE_SIZE
                );
                ContinuationToken::from_string(serialized)
            };

            println!("Captured continuation token after fetching first page, now updating throughput to trigger split");

            // Force a split by raising throughput to 13000 RU/s
            // (>10k forces at least 2 physical partitions).
            let split_start = Instant::now();
            let new_throughput = ThroughputProperties::manual(13000);
            let mut poller = container_client
                .begin_replace_throughput(new_throughput, None)
                .await?;
            println!("Throughput update initiated, polling for completion...");
            let mut last_throughput = None;
            let mut poll_count = 0;
            while let Some(status) = poller.try_next().await? {
                if split_start.elapsed() >= SPLIT_POLL_TIMEOUT {
                    return Err(InconclusiveError::SplitNotCompleted.into());
                }

                assert!(status.status().is_success());
                last_throughput = Some(status.into_model()?);
                if poll_count % 15 == 0 {
                    println!(
                        "Throughput update in progress... polled {} times, last observed throughput: {} RU/s",
                        poll_count,
                        last_throughput.as_ref().and_then(|t| t.throughput()).unwrap_or(0)
                    );
                }
                poll_count += 1;
            }
            let final_throughput = last_throughput
                .expect("throughput poller should have yielded at least one response");
            assert_eq!(Some(13000), final_throughput.throughput());
            println!("Throughput update completed, new throughput: {} RU/s", final_throughput.throughput().unwrap_or(0));

            // Poll read_feed_ranges until we observe >= 2 physical
            // partitions or the timeout elapses.
            let mut iterations = 0;
            let observed_ranges = loop {
                let ranges = container_client
                    .read_feed_ranges(Some(ReadFeedRangesOptions::default().with_force_refresh(true)))
                    .await?;
                if ranges.len() >= 2 {
                    break ranges;
                }
                if split_start.elapsed() >= SPLIT_POLL_TIMEOUT {
                    return Err(InconclusiveError::SplitNotCompleted.into());
                }

                // Every minute, print a message to indicate we're still waiting and the test isn't hanging
                if iterations % 4 == 0 {
                    println!(
                        "Waiting for split to complete... elapsed: {:?}, last observed physical partition count: {}",
                        split_start.elapsed(),
                        ranges.len()
                    );
                }
                tokio::time::sleep(SPLIT_POLL_INTERVAL).await;
                iterations += 1;
            };
            println!(
                "Split observed after {:?}; physical partition count: {}",
                split_start.elapsed(),
                observed_ranges.len()
            );

            // Resume pagination using the saved continuation token.
            // Round-trip the token between every page so we keep exercising
            // the suspend/resume path now that the topology has changed.
            let mut continuation = Some(saved_token);
            loop {
                let mut resume_options = QueryOptions::default().with_max_item_count(
                    MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()),
                );
                if let Some(token) = continuation.take() {
                    resume_options = resume_options.with_continuation_token(token);
                }

                let mut pages = container_client
                    .query_items::<MockItem>(
                        "SELECT * FROM c",
                        FeedScope::full_container(),
                        Some(resume_options),
                    )
                    .await?
                    .into_pages();

                let Some(page) = pages.next().await else {
                    break;
                };
                let page = page?;
                for item in page.into_items() {
                    collected.push(item.id);
                }

                let token = pages.to_continuation_token()?;
                let serialized = token.as_str().to_owned();
                drop(pages);
                continuation = Some(ContinuationToken::from_string(serialized));
            }

            // The query is a plain `SELECT *` with no `ORDER BY`, so Cosmos
            // makes no guarantees about the relative order in which items are
            // returned across physical partitions, and that order is expected
            // to shift mid-stream when a split fans the request out across the
            // new EPK ranges. What the SDK *does* guarantee, and what this
            // assertion is here to verify, is that every seeded item is
            // returned exactly once: no losses and no duplicates across the
            // split boundary. Sort both sides before comparing so an ordering
            // change cannot mask (or be mistaken for) a correctness failure.
            let mut collected_sorted = collected.clone();
            collected_sorted.sort();
            let mut expected_sorted = expected_ids.clone();
            expected_sorted.sort();
            assert_eq!(
                collected.len(),
                expected_ids.len(),
                "collected {} items but expected {} (duplicates or losses across split)",
                collected.len(),
                expected_ids.len()
            );
            assert_eq!(
                collected_sorted, expected_sorted,
                "items collected across split should match the seeded ground truth (order-independent)"
            );

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}
