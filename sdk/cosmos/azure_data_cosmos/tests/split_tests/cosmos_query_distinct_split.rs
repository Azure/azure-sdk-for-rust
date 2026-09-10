// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live split coverage for text and binary cross-partition `DISTINCT`.
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
//! Three invariants are asserted across one live split:
//!
//! - **Unordered** `DISTINCT` drained straight through a split returns each
//!   value exactly once.
//! - **Ordered** `DISTINCT` (matching `ORDER BY`) resumed from a continuation
//!   token captured *before* the split returns each value exactly once, in
//!   sorted order, with no gap at the boundary.
//! - **Windowed ordered** `DISTINCT` retains its deduplication and row-window
//!   state while a split occurs between output pages.
//!
//! Runs only under `test_category = "split"` against split-capable resources.

use super::framework;
use crate::split_tests::cosmos_query_split::force_split_and_wait;

use std::collections::BTreeSet;
use std::error::Error;
use std::num::NonZeroU32;
use std::time::Duration;

use azure_data_cosmos::feed::{ContinuationToken, QueryPageIterator};
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, CosmosStatus, ThroughputProperties},
    options::{BinaryEncodingOptions, MaxItemCountHint, OperationOptions, QueryOptions},
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

fn query_options(binary: bool, page_size: u32) -> QueryOptions {
    let mut operation = OperationOptions::default();
    operation.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(binary));
    QueryOptions::default()
        .with_operation_options(operation)
        .with_max_item_count(MaxItemCountHint::Limit(
            NonZeroU32::new(page_size).expect("page size is non-zero"),
        ))
}

async fn start_query(
    container_client: &ContainerClient,
    query: &str,
    binary: bool,
    page_size: u32,
) -> Result<(QueryPageIterator<String>, Vec<String>), Box<dyn Error>> {
    let mut pages = container_client
        .query_items::<String>(
            query,
            FeedScope::full_container(),
            Some(query_options(binary, page_size)),
        )
        .await?
        .into_pages();
    let first_page = pages
        .next()
        .await
        .expect("query should yield at least one page")?
        .into_items();
    Ok((pages, first_page))
}

async fn drain_remaining(
    pages: &mut QueryPageIterator<String>,
    mut collected: Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    while let Some(page) = pages.next().await {
        collected.extend(page?.into_items());
    }
    Ok(collected)
}

async fn capture_ordered_checkpoint(
    container_client: &ContainerClient,
    binary: bool,
) -> Result<(Vec<String>, ContinuationToken), Box<dyn Error>> {
    let (pages, collected) = start_query(container_client, ORDERED_QUERY, binary, 2).await?;
    let token = match pages.to_continuation_token() {
        Ok(token) => ContinuationToken::from_string(token.as_str().to_owned()),
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
    assert!(
        !collected.is_empty(),
        "the pre-split checkpoint must have emitted at least one value"
    );
    Ok((collected, token))
}

async fn resume_ordered(
    container_client: &ContainerClient,
    binary: bool,
    mut collected: Vec<String>,
    token: ContinuationToken,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut continuation = Some(token);
    loop {
        let mut options = query_options(binary, 2);
        if let Some(token) = continuation.take() {
            options = options.with_continuation_token(token);
        }
        let mut pages = container_client
            .query_items::<String>(ORDERED_QUERY, FeedScope::full_container(), Some(options))
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            break;
        };
        collected.extend(page?.into_items());
        continuation = Some(ContinuationToken::from_string(
            pages.to_continuation_token()?.as_str().to_owned(),
        ));
    }
    Ok(collected)
}

/// Cross-partition `DISTINCT` across a live partition split.
///
/// Captures text and binary ordered, windowed, and unordered state before one
/// shared split, then finishes every scenario against the child topology.
#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
pub async fn text_and_binary_distinct_queries_reuse_one_split() -> Result<(), Box<dyn Error>> {
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

            let (ordered_text, ordered_text_token) =
                capture_ordered_checkpoint(&container_client, false).await?;
            let (ordered_binary, ordered_binary_token) =
                capture_ordered_checkpoint(&container_client, true).await?;

            let (mut windowed_text_pages, windowed_text) =
                start_query(&container_client, WINDOWED_QUERY, false, 1).await?;
            let (mut windowed_binary_pages, windowed_binary) =
                start_query(&container_client, WINDOWED_QUERY, true, 1).await?;
            assert_eq!(windowed_text.len(), 1);
            assert_eq!(windowed_binary.len(), 1);

            let (mut unordered_text_pages, unordered_text) =
                start_query(&container_client, UNORDERED_QUERY, false, PAGE_SIZE).await?;
            let (mut unordered_binary_pages, unordered_binary) =
                start_query(&container_client, UNORDERED_QUERY, true, PAGE_SIZE).await?;

            let partitions_after =
                force_split_and_wait(&container_client, partitions_before).await?;
            assert!(
                partitions_after > partitions_before,
                "split must increase partition count: before={partitions_before}, \
                 after={partitions_after}"
            );

            let unordered_text = drain_remaining(&mut unordered_text_pages, unordered_text).await?;
            let unordered_binary =
                drain_remaining(&mut unordered_binary_pages, unordered_binary).await?;
            assert_each_value_exactly_once(
                &unordered_text,
                "text unordered DISTINCT across a split",
            );
            assert_each_value_exactly_once(
                &unordered_binary,
                "binary unordered DISTINCT across a split",
            );

            for (binary, collected, token, context) in [
                (
                    false,
                    ordered_text,
                    ordered_text_token,
                    "text ordered DISTINCT resumed across a split",
                ),
                (
                    true,
                    ordered_binary,
                    ordered_binary_token,
                    "binary ordered DISTINCT resumed across a split",
                ),
            ] {
                let ordered = resume_ordered(&container_client, binary, collected, token).await?;
                assert_each_value_exactly_once(&ordered, context);
                let mut sorted = ordered.clone();
                sorted.sort();
                assert_eq!(
                    ordered, sorted,
                    "{context}: global sort order must survive the split"
                );
            }

            let windowed_text = drain_remaining(&mut windowed_text_pages, windowed_text).await?;
            let windowed_binary =
                drain_remaining(&mut windowed_binary_pages, windowed_binary).await?;
            for (encoding, windowed) in [("text", windowed_text), ("binary", windowed_binary)] {
                assert_eq!(
                    windowed,
                    vec!["g01", "g02"],
                    "{encoding} `{WINDOWED_QUERY}` must preserve the exact DISTINCT window \
                     across the split"
                );
            }

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_timeout(Duration::from_secs(40 * 60))
                .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true)),
        ),
    )
    .await
}
