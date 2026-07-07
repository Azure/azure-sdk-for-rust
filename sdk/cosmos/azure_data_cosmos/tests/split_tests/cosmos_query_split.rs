// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::split_tests::framework::InconclusiveError;

use super::framework;

use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos::feed::ContinuationToken;
use azure_data_cosmos::options::{CreateContainerOptions, ReadFeedRangesOptions};
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, ThroughputProperties},
    options::{MaxItemCountHint, QueryOptions},
};
use framework::{MockItem, TestClient, TestOptions};
use futures::{StreamExt, TryStreamExt};

// We wait 10 minutes for the split to occur. If it doesn't, we "fail" the test,
// but with a clear message indicating that it only failed because the split didn't happen in time, rather than panicking on some other assumption later in the test.
// Splits _often_ complete in time, but when they don't, we don't necessarily want to block any given PR.
const SPLIT_POLL_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const SPLIT_POLL_INTERVAL: Duration = Duration::from_secs(15);

/// Triggers a partition split on `container_client` by raising throughput to
/// 13000 RU/s (>10k forces at least 2 physical partitions), then polls
/// `read_feed_ranges` until the topology grows past `starting_partitions`.
/// Returns the observed physical partition count.
///
/// Pass the partition count observed BEFORE the throughput bump so the wait
/// loop is robust against accounts that already start at >= 2 physical
/// partitions — otherwise the loop would return immediately with no actual
/// topology change.
///
/// Returns [`InconclusiveError::SplitNotCompleted`] if a strictly larger
/// topology has not appeared within [`SPLIT_POLL_TIMEOUT`].
pub(crate) async fn force_split_and_wait(
    container_client: &ContainerClient,
    starting_partitions: usize,
) -> Result<usize, Box<dyn Error>> {
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
    let final_throughput =
        last_throughput.expect("throughput poller should have yielded at least one response");
    assert_eq!(Some(13000), final_throughput.throughput());
    println!(
        "Throughput update completed, new throughput: {} RU/s",
        final_throughput.throughput().unwrap_or(0)
    );

    let mut iterations = 0;
    let observed_ranges = loop {
        let ranges = container_client
            .read_feed_ranges(Some(
                ReadFeedRangesOptions::default().with_force_refresh(true),
            ))
            .await?;
        if ranges.len() > starting_partitions {
            break ranges;
        }
        if split_start.elapsed() >= SPLIT_POLL_TIMEOUT {
            return Err(InconclusiveError::SplitNotCompleted.into());
        }

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
    Ok(observed_ranges.len())
}

/// Cross-partition query resume after a split, covering both saved-token
/// shapes in a single live container.
///
/// Seeds 50 partition keys × 10 items = 500 documents and reads with a page
/// size of 10. Captures two continuation tokens against the pre-split
/// topology:
///
/// * **Token A** after page 1 — mid-fan-out across multiple sibling
///   partitions; some siblings have not yet been touched. This is the
///   canonical repro for the duplicate-emission defect: pre-fix, resuming
///   from Token A across a split silently dropped non-front siblings'
///   pre-split state and re-emitted items.
///
/// * **Token B** after draining 5 pages (50 items) — each page boundary
///   round-trips the continuation, so the captured token records
///   per-partition state several pages deep.
///
/// A single split is forced after both tokens are captured (live splits are
/// expensive — collapsing two scenarios into one container saves ~10
/// minutes of CI time). Each token is then resumed independently and
/// asserted to drain to the full seeded set with no duplicates and no
/// losses.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn query_resume_across_split_covers_both_snapshot_shapes() -> Result<(), Box<dyn Error>> {
    const PK_COUNT: usize = 50;
    const ITEMS_PER_PK: usize = 10;
    const PAGE_SIZE: u32 = 10;
    const PAGES_FOR_TOKEN_B: usize = 5;

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("QueryResumeAcrossSplit", "/partitionKey".into());
            let throughput = ThroughputProperties::manual(1000);
            let container_client = Arc::new(
                run_context
                    .create_container(
                        db_client,
                        properties,
                        Some(CreateContainerOptions::default().with_throughput(throughput)),
                    )
                    .await?,
            );

            println!(
                "Container created; seeding {PK_COUNT} partition keys × {ITEMS_PER_PK} items"
            );
            let mut expected_ids: Vec<String> = Vec::new();
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..ITEMS_PER_PK {
                    let item = MockItem {
                        id: format!("{p}-{i}"),
                        partition_key: partition_key.clone(),
                        merge_order: p * ITEMS_PER_PK + i,
                    };
                    expected_ids.push(item.id.clone());
                    container_client
                        .create_item(item.partition_key.clone(), &item.id.clone(), item, None)
                        .await?;
                }
            }

            let ranges_before = container_client.read_feed_ranges(None).await?;
            assert!(
                !ranges_before.is_empty(),
                "expected at least one physical partition before split, got {}",
                ranges_before.len()
            );
            let partitions_before = ranges_before.len();

            // Capture Token A after page 1 (mid-fan-out shape) and
            // Token B after PAGES_FOR_TOKEN_B pages (deep per-partition
            // state), reusing a single query stream so each token reflects
            // the genuine snapshot shape the driver writes between pages.
            let mut token_a_collected: Vec<String> = Vec::new();
            let mut token_b_collected: Vec<String> = Vec::new();
            let (token_a, token_b) = {
                let options = QueryOptions::default().with_max_item_count(
                    MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()),
                );
                let mut pages = container_client
                    .query_items::<MockItem>(
                        "SELECT * FROM c",
                        FeedScope::full_container(),
                        Some(options),
                    )
                    .await?
                    .into_pages();

                let first_page = pages
                    .next()
                    .await
                    .expect("query should yield at least one page before split")?;
                for item in first_page.into_items() {
                    token_a_collected.push(item.id);
                }
                let token_a_serialized = pages.to_continuation_token()?.as_str().to_owned();
                token_b_collected.extend(token_a_collected.iter().cloned());

                for page_idx in 1..PAGES_FOR_TOKEN_B {
                    let Some(page) = pages.next().await else {
                        panic!(
                            "pre-split drain expected at least {PAGES_FOR_TOKEN_B} pages but stopped at {page_idx}"
                        );
                    };
                    let page = page?;
                    for item in page.into_items() {
                        token_b_collected.push(item.id);
                    }
                }
                let token_b_serialized = pages.to_continuation_token()?.as_str().to_owned();

                (
                    ContinuationToken::from_string(token_a_serialized),
                    ContinuationToken::from_string(token_b_serialized),
                )
            };

            println!(
                "Captured token A after {} items, token B after {} items; forcing split",
                token_a_collected.len(),
                token_b_collected.len()
            );
            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, after={partitions_after}"
            );

            // Resume from Token A across the split — must collect every
            // seeded item with no duplicates and no losses.
            drain_from_token(
                &container_client,
                token_a,
                PAGE_SIZE,
                &mut token_a_collected,
            )
            .await?;
            assert_drained_matches_expected(&token_a_collected, &expected_ids, "token A");

            // Resume from Token B across the same split — same invariant.
            drain_from_token(
                &container_client,
                token_b,
                PAGE_SIZE,
                &mut token_b_collected,
            )
            .await?;
            assert_drained_matches_expected(&token_b_collected, &expected_ids, "token B");

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}

/// Drains a query from `start_token` to completion, pushing every item
/// onto `collected` and round-tripping the continuation through string
/// serialization between every page so each resume exercises a fresh
/// pipeline.
async fn drain_from_token(
    container_client: &ContainerClient,
    start_token: ContinuationToken,
    page_size: u32,
    collected: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let mut continuation = Some(start_token);
    loop {
        let mut resume_options = QueryOptions::default()
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
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
    Ok(())
}

fn assert_drained_matches_expected(collected: &[String], expected: &[String], label: &str) {
    let mut collected_sorted = collected.to_vec();
    collected_sorted.sort();
    let mut expected_sorted = expected.to_vec();
    expected_sorted.sort();
    assert_eq!(
        collected.len(),
        expected.len(),
        "{label}: collected {} items but expected {} (duplicates or losses across split)",
        collected.len(),
        expected.len()
    );
    assert_eq!(
        collected_sorted, expected_sorted,
        "{label}: items collected across split should match the seeded ground truth (order-independent)"
    );
}
