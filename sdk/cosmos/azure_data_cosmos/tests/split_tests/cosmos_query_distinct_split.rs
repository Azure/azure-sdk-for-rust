// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live-only split coverage for cross-partition `DISTINCT`.
//!
//! Neither .NET nor Java tests `DISTINCT` against a real partition split
//! (.NET's `FullPipelineTests.TestMerge` covers `ORDER BY` only), so this is
//! the one behavior with no peer precedent to lean on. It matters because the
//! deduplication state lives in a stage *above* the fan-out root: if a split
//! ever caused that stage to be rebuilt, values emitted before the split would
//! come back.
//!
//! The container is seeded so every `groupKey` value is produced by many
//! partition keys — after a split, both physical partitions still contribute
//! rows for the same values, which is precisely the case a per-partition or
//! per-page dedup would get wrong.
//!
//! Two invariants are asserted across one live split:
//!
//! - **Unordered** `DISTINCT` drained straight through a split returns each
//!   value exactly once.
//! - **Ordered** `DISTINCT` (matching `ORDER BY`) resumed from a continuation
//!   token captured *before* the split returns each value exactly once, in
//!   sorted order, with no gap at the boundary.
//!
//! Runs only under `test_category = "split"` against split-capable resources.

use super::framework;
use crate::split_tests::cosmos_query_split::force_split_and_wait;

use std::collections::BTreeSet;
use std::error::Error;
use std::num::NonZeroU32;
use std::time::Duration;

use azure_data_cosmos::feed::ContinuationToken;
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, CosmosStatus, ThroughputProperties},
    options::{MaxItemCountHint, QueryOptions},
};
use framework::{TestClient, TestOptions};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const PK_COUNT: usize = 40;
const GROUP_COUNT: usize = 8;
const PAGE_SIZE: u32 = 5;

const UNORDERED_QUERY: &str = "SELECT DISTINCT VALUE c.groupKey FROM c";
const ORDERED_QUERY: &str = "SELECT DISTINCT VALUE c.groupKey FROM c ORDER BY c.groupKey";
/// `DISTINCT` composed under a global row window. `GROUP_COUNT` distinct keys
/// exist, so skipping one and taking two must yield exactly two — the window
/// counts deduplicated values, not raw rows.
const WINDOWED_QUERY: &str =
    "SELECT DISTINCT VALUE c.groupKey FROM c ORDER BY c.groupKey OFFSET 1 LIMIT 2";

/// A seeded document whose `groupKey` deliberately repeats across every
/// partition key, so deduplication has to be global.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct SeedItem {
    id: String,
    partition_key: String,
    group_key: String,
}

/// The distinct values the seed produces, in sorted order.
fn expected_group_keys() -> Vec<String> {
    (0..GROUP_COUNT).map(|i| format!("g{i:02}")).collect()
}

fn assert_each_value_exactly_once(actual: &[String], context: &str) {
    let expected = expected_group_keys();
    let unique: BTreeSet<&String> = actual.iter().collect();
    assert_eq!(
        unique.len(),
        actual.len(),
        "{context}: DISTINCT returned a duplicate value: {actual:?}"
    );
    let mut sorted = actual.to_vec();
    sorted.sort();
    assert_eq!(
        sorted, expected,
        "{context}: DISTINCT did not return exactly the seeded value set"
    );
}

/// Drains `query` one page at a time, invoking `after_first_page` once the
/// first page has been collected (used to force the split mid-drain).
async fn drain_with_hook<F, Fut>(
    container_client: &ContainerClient,
    query: &str,
    mut after_first_page: F,
) -> Result<Vec<String>, Box<dyn Error>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn Error>>>,
{
    let options = QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    let mut pages = container_client
        .query_items::<String>(query, FeedScope::full_container(), Some(options))
        .await?
        .into_pages();

    let mut collected = Vec::new();
    let mut page_index = 0usize;
    while let Some(page) = pages.next().await {
        collected.extend(page?.into_items());
        if page_index == 0 {
            after_first_page().await?;
        }
        page_index += 1;
    }
    Ok(collected)
}

/// Cross-partition `DISTINCT` across a live partition split.
///
/// Part 1 drains an unordered `DISTINCT` straight through a forced split.
/// Part 2 captures an ordered `DISTINCT` continuation token before the split
/// (already taken, since the split happened in part 1) and resumes it against
/// the post-split topology.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn distinct_query_across_split_returns_each_value_once() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("DistinctAcrossSplit", "/partitionKey".into());
            let throughput = ThroughputProperties::manual(1000);
            let container_client = run_context
                .create_container(
                    db_client,
                    properties,
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            println!(
                "Container created; seeding {PK_COUNT} partition keys x {GROUP_COUNT} repeated \
                 group keys"
            );
            for p in 0..PK_COUNT {
                let partition_key = format!("pk{p}");
                for i in 0..GROUP_COUNT {
                    let item = SeedItem {
                        id: format!("{p}-{i}"),
                        partition_key: partition_key.clone(),
                        group_key: format!("g{i:02}"),
                    };
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

            // ── Ordered DISTINCT: capture a token before the split ────────
            //
            // Taken first so the checkpoint predates the topology change.
            let ordered_options = QueryOptions::default()
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
            let mut ordered_pages = container_client
                .query_items::<String>(
                    ORDERED_QUERY,
                    FeedScope::full_container(),
                    Some(ordered_options),
                )
                .await?
                .into_pages();
            let mut ordered_collected: Vec<String> = ordered_pages
                .next()
                .await
                .expect("ordered DISTINCT should yield at least one page")?
                .into_items();
            // `ORDERED_QUERY` uses the `VALUE` form, which the service reports
            // as `distinctType: Ordered` and is therefore resumable. Read the
            // outcome back rather than asserting it, so that if a future service
            // version downgrades the shape this reports a self-describing skip
            // instead of a bare panic that would blame the wrong thing.
            let ordered_token = match ordered_pages.to_continuation_token() {
                Ok(token) => ContinuationToken::from_string(token.as_str().to_owned()),
                // Only an explicit "this shape cannot be resumed" refusal is a
                // legitimate service-plan downgrade. Any other failure is a
                // continuation regression and must not be swallowed, or this
                // test would go green while resume is broken.
                Err(error)
                    if error.status().sub_status()
                        == CosmosStatus::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED.sub_status() =>
                {
                    panic!(
                        "service planned `{ORDERED_QUERY}` as unordered DISTINCT (continuation \
                         refused: {error}). The VALUE form with a matching ORDER BY is a required \
                         contract for resumable DISTINCT; if the service genuinely changed, update \
                         `plan::distinct_is_ordered` and the docs rather than skipping this test."
                    );
                }
                Err(error) => {
                    return Err(format!(
                        "minting a continuation for `{ORDERED_QUERY}` failed with an unexpected \
                         error (not the unsupported-continuation status): {error}"
                    )
                    .into());
                }
            };
            drop(ordered_pages);
            assert!(
                !ordered_collected.is_empty(),
                "the pre-split checkpoint must have emitted at least one value"
            );

            // ── Unordered DISTINCT: drain straight through the split ──────
            let mut split_done = false;
            let unordered = drain_with_hook(&container_client, UNORDERED_QUERY, || {
                let container_client = container_client.clone();
                let should_split = !split_done;
                split_done = true;
                async move {
                    if should_split {
                        let partitions_after =
                            force_split_and_wait(&container_client, partitions_before).await?;
                        assert!(
                            partitions_after > partitions_before,
                            "split must increase partition count: before={partitions_before}, \
                             after={partitions_after}"
                        );
                    }
                    Ok(())
                }
            })
            .await?;
            assert_each_value_exactly_once(&unordered, "unordered DISTINCT across a split");

            // ── Ordered DISTINCT: resume the pre-split token ──────────────
            let mut continuation = Some(ordered_token);
            loop {
                let mut options = QueryOptions::default()
                    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
                if let Some(token) = continuation.take() {
                    options = options.with_continuation_token(token);
                }
                let mut pages = container_client
                    .query_items::<String>(
                        ORDERED_QUERY,
                        FeedScope::full_container(),
                        Some(options),
                    )
                    .await?
                    .into_pages();
                let Some(page) = pages.next().await else {
                    break;
                };
                ordered_collected.extend(page?.into_items());
                let serialized = pages.to_continuation_token()?.as_str().to_owned();
                drop(pages);
                continuation = Some(ContinuationToken::from_string(serialized));
            }

            assert_each_value_exactly_once(
                &ordered_collected,
                "ordered DISTINCT resumed across a split",
            );
            let mut sorted = ordered_collected.clone();
            sorted.sort();
            assert_eq!(
                ordered_collected, sorted,
                "an ordered DISTINCT resume must preserve global sort order across the split"
            );

            // ── DISTINCT under a row window, drained across the split ─────
            //
            // `SkipTake` wraps `Distinct`, so a split must preserve both the
            // dedup state and the window's remaining budget. Losing the former
            // re-emits a value the window already paid for; losing the latter
            // restarts the offset and over-returns. The emulator covers this
            // with a simulated split — this is the same shape against a real
            // one, where the fan-out is genuinely rebuilt.
            let windowed =
                drain_with_hook(&container_client, WINDOWED_QUERY, || async { Ok(()) }).await?;
            assert_eq!(
                windowed.len(),
                2,
                "`{WINDOWED_QUERY}` must apply its window to deduplicated values across the \
                 split, got {windowed:?}"
            );
            let mut windowed_sorted = windowed.clone();
            windowed_sorted.sort();
            windowed_sorted.dedup();
            assert_eq!(
                windowed_sorted.len(),
                windowed.len(),
                "windowed DISTINCT across a split returned duplicates: {windowed:?}"
            );
            let mut in_order = windowed.clone();
            in_order.sort();
            assert_eq!(
                windowed, in_order,
                "windowed DISTINCT must preserve global sort order across the split"
            );

            Ok(())
        },
        // A real split takes minutes; the 80s default would abort mid-poll.
        // Matches the other split tests in this directory.
        Some(TestOptions::new().with_timeout(Duration::from_secs(40 * 60))),
    )
    .await
}
