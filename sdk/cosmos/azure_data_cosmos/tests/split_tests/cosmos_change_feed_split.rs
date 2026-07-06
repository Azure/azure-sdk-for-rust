// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live split test for the change feed pull API.
//!
//! A continuation token captured before a partition split must resume cleanly
//! across the split: every change written after the captured position is
//! delivered exactly once — with no replay of pre-token history and no losses —
//! even on the post-split child partitions whose parent-partition token is
//! reused.
//!
//! Forcing a real split is expensive, so this reuses
//! [`force_split_and_wait`](super::cosmos_query_split::force_split_and_wait)
//! from the query split test rather than duplicating the throughput-bump and
//! topology-polling logic.

use super::cosmos_query_split::force_split_and_wait;
use super::framework;

use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use azure_data_cosmos::feed::{ChangeFeedPageIterator, ContinuationToken, FeedScope};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::{ChangeFeedOptions, ChangeFeedStartFrom, CreateContainerOptions};
use framework::{MockItem, TestClient, TestOptions};
use futures::StreamExt;

/// Maximum page polls a drain loop performs before giving up, guarding the
/// change feed's intentionally infinite stream against looping forever.
const MAX_DRAIN_POLLS: usize = 500;

/// Consecutive empty (304) pages that signal the feed has caught up. The
/// `UnorderedMerge` resets to a non-empty page as soon as any partition has
/// data, so a streak of empties reliably means every partition has drained.
const EMPTY_STREAK_TO_STOP: usize = 5;

/// Drains a change feed iterator until it reports a streak of empty pages or a
/// poll cap is reached, returning every item seen.
async fn drain_changes(
    iterator: &mut ChangeFeedPageIterator<MockItem>,
) -> Result<Vec<MockItem>, Box<dyn Error>> {
    let mut collected = Vec::new();
    let mut empty_streak = 0usize;
    let mut polls = 0usize;

    while let Some(page) = iterator.next().await {
        let page = page?;
        polls += 1;

        if page.items().is_empty() {
            empty_streak += 1;
            if empty_streak >= EMPTY_STREAK_TO_STOP {
                break;
            }
        } else {
            empty_streak = 0;
            collected.extend(page.into_items());
        }

        if polls >= MAX_DRAIN_POLLS {
            break;
        }
    }

    Ok(collected)
}

/// A continuation token captured from a caught-up change feed before a split
/// resumes correctly after the split: only the post-split writes are returned,
/// each exactly once, with no replay of the pre-token baseline.
///
/// This exercises the cross-partition resume path where a pre-split parent
/// token is reused against the post-split child partitions, while partitions
/// without a saved continuation re-apply the feed's original start position.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn change_feed_resume_across_split() -> Result<(), Box<dyn Error>> {
    const PK_COUNT: usize = 30;
    const BASELINE_PER_PK: usize = 3;
    const NEW_PER_PK: usize = 2;

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("ChangeFeedResumeAcrossSplit", "/partitionKey".into());
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

            // Seed the baseline (pre-split, pre-token) changes.
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..BASELINE_PER_PK {
                    let item = MockItem {
                        id: format!("baseline-{p}-{i}"),
                        partition_key: partition_key.clone(),
                        merge_order: p * BASELINE_PER_PK + i,
                    };
                    let id = item.id.clone();
                    container_client
                        .create_item(item.partition_key.clone(), &id, item, None)
                        .await?;
                }
            }

            let partitions_before = container_client.read_feed_ranges(None).await?.len();
            assert!(
                partitions_before >= 1,
                "expected at least one physical partition before the split"
            );

            // Drain the whole baseline from the beginning, then capture a resume
            // token at the caught-up position.
            let mut iterator = container_client
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?;
            let baseline_seen = drain_changes(&mut iterator).await?;
            assert_eq!(
                baseline_seen.len(),
                PK_COUNT * BASELINE_PER_PK,
                "expected to drain the full baseline before capturing the resume token"
            );

            let token = iterator.to_continuation_token()?;
            let token = ContinuationToken::from_string(token.as_str().to_owned());
            drop(iterator);

            // Force a real split AFTER the token is captured so the resume must
            // map the pre-split parent token onto the post-split children.
            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, after={partitions_after}"
            );

            // Write new changes after the split.
            let mut expected_new: Vec<String> = Vec::new();
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..NEW_PER_PK {
                    let item = MockItem {
                        id: format!("post-{p}-{i}"),
                        partition_key: partition_key.clone(),
                        merge_order: 100_000 + p * NEW_PER_PK + i,
                    };
                    expected_new.push(item.id.clone());
                    let id = item.id.clone();
                    container_client
                        .create_item(item.partition_key.clone(), &id, item, None)
                        .await?;
                }
            }

            // Resume from the pre-split token: exactly the post-split changes
            // must surface, with no replayed baseline and no losses.
            let mut resumed = container_client
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    // Ignored on resume: the token carries its own position.
                    ChangeFeedStartFrom::Beginning,
                    Some(ChangeFeedOptions::default().with_continuation_token(token)),
                )
                .await?;
            let resumed_items = drain_changes(&mut resumed).await?;

            let mut collected_ids: Vec<String> =
                resumed_items.into_iter().map(|item| item.id).collect();
            collected_ids.sort();
            expected_new.sort();

            assert_eq!(
                collected_ids, expected_new,
                "resume across split must deliver exactly the post-split changes once each"
            );
            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}
