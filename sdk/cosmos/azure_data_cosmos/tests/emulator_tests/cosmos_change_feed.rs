// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator integration tests for the change feed pull API.
//!
//! These mirror the standard change feed scenarios validated by the .NET and
//! Java SDKs:
//!
//! * `StartFrom::Beginning` returns all historical changes (single partition
//!   and cross-partition fan-out).
//! * `StartFrom::Now` excludes pre-existing items and only surfaces changes
//!   made after the iterator's starting position.
//! * A quiescent feed returns an empty page (HTTP 304 Not Modified) instead of
//!   erroring or terminating the stream.
//! * A continuation token resumes the feed and only yields changes that
//!   occurred after the captured position.
//! * Resuming a partially-polled `StartFrom::Now` feed does not replay history
//!   on the partitions that were never polled before the checkpoint.

use super::framework;

use std::error::Error;
use std::num::NonZeroU32;
use std::time::Duration;

use azure_data_cosmos::feed::{ChangeFeedPageIterator, ContinuationToken, FeedScope};
use azure_data_cosmos::models::{ChangeFeedItem, ThroughputProperties};
use azure_data_cosmos::options::{ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint};
use framework::{test_data, MockItem, TestClient, TestOptions};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use time::OffsetDateTime;

/// Maximum number of page polls a drain loop will perform before giving up.
///
/// Guards against the change feed's intentionally infinite stream looping
/// forever if a test's stop condition is never met.
const MAX_DRAIN_POLLS: usize = 200;

/// Number of consecutive empty (304) pages that a test treats as "the feed has
/// caught up". Because [`UnorderedMerge`] resets to a non-empty page as soon as
/// any partition has data, a streak of consecutive empties reliably means every
/// partition has drained its backlog.
///
/// This is purely a **test heuristic** for terminating a drain loop against the
/// emulator — it is *not* a protocol guarantee. The change feed is a
/// conceptually infinite stream and the service never signals "end of feed", so
/// production consumers must rely on continuation tokens rather than an empty
/// streak. The streak is kept comfortably above 1 so a transient single empty
/// page (e.g. one partition briefly quiescent while another still has data)
/// does not end the loop prematurely.
const EMPTY_STREAK_TO_STOP: usize = 5;

/// Polls a change feed iterator until it reports no further changes (a streak
/// of empty 304 pages) or a poll cap is reached, returning every item seen.
async fn drain_changes<T>(
    iterator: &mut ChangeFeedPageIterator<T>,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
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

/// Sorts items by their numeric `id` so collections gathered in partition or
/// page order can be compared deterministically.
fn sort_by_id(items: &mut [MockItem]) {
    items.sort_by_key(|item| item.id.parse::<usize>().unwrap_or(usize::MAX));
}

/// Extracts the post-change document (`current`) from each change feed
/// envelope.
///
/// Every change feed item is a [`ChangeFeedItem<MockItem>`] envelope; these
/// LatestVersion tests only assert on the document, which is carried under
/// `current` (never a delete, so `current` is always present).
fn currents(envelopes: Vec<ChangeFeedItem<MockItem>>) -> Vec<MockItem> {
    envelopes
        .into_iter()
        .map(|envelope| {
            envelope
                .current()
                .cloned()
                .expect("LatestVersion change feed items carry a current document")
        })
        .collect()
}

/// `StartFrom::Beginning` against a single logical partition returns exactly
/// the items written to that partition. This exercises the trivial
/// (single-request) change feed path.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_from_beginning_single_partition() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let mut expected: Vec<MockItem> = items
                .iter()
                .filter(|item| item.partition_key == "partition3")
                .cloned()
                .collect();
            sort_by_id(&mut expected);

            let container = test_data::create_container_with_items(db_client, items, None).await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition3"),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?;

            let mut actual = currents(drain_changes(&mut iterator).await?);
            sort_by_id(&mut actual);

            assert_eq!(expected, actual);
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// `StartFrom::Beginning` over the full container fans out across every
/// physical partition (via `UnorderedMerge`) and returns all items. The
/// container is provisioned with enough throughput to force multiple physical
/// partitions so the cross-partition merge path is genuinely exercised.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_from_beginning_full_container() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let mut expected = items.clone();
            sort_by_id(&mut expected);

            // 11000 RU/s forces the service to create at least 2 physical
            // partitions, so the full-container read must fan out.
            let container = test_data::create_container_with_items(
                db_client,
                items,
                Some(ThroughputProperties::manual(11000)),
            )
            .await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?;

            let mut actual = currents(drain_changes(&mut iterator).await?);
            sort_by_id(&mut actual);

            assert_eq!(expected, actual);
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// `StartFrom::Now` excludes items written before the iterator's start
/// position and surfaces only changes made afterwards.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_start_from_now_returns_only_new_changes() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            // Baseline items exist before the iterator is created.
            let baseline = test_data::generate_mock_items(10, 5);
            let container =
                test_data::create_container_with_items(db_client, baseline, None).await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    ChangeFeedStartFrom::Now,
                    None,
                )
                .await?;

            // The first poll establishes the "now" position; because no writes
            // have happened since, it must be empty (baseline excluded).
            let first = iterator
                .next()
                .await
                .expect("change feed stream always yields a page")?;
            assert!(
                first.items().is_empty(),
                "StartFrom::Now should not return baseline items, got {} items",
                first.items().len()
            );

            // Write new items after the "now" marker.
            let new_items: Vec<MockItem> = (0..3)
                .map(|i| MockItem {
                    id: format!("100{i}"),
                    partition_key: "partition0".to_string(),
                    merge_order: 1000 + i,
                })
                .collect();
            for item in &new_items {
                container
                    .create_item("partition0", &item.id, item, None)
                    .await?;
            }

            let mut actual = currents(drain_changes(&mut iterator).await?);
            sort_by_id(&mut actual);

            let mut expected = new_items;
            sort_by_id(&mut expected);

            assert_eq!(expected, actual);
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A feed with no changes returns an empty page (HTTP 304 Not Modified)
/// without erroring or terminating the stream.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_no_changes_returns_empty_page() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            // Empty container — there are no changes to report.
            let container =
                test_data::create_container_with_items(db_client, Vec::new(), None).await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?;

            // Several consecutive polls must each yield an empty page and must
            // not error or end the stream.
            for poll in 0..3 {
                let page = iterator
                    .next()
                    .await
                    .expect("change feed stream always yields a page")?;
                assert!(
                    page.items().is_empty(),
                    "poll {poll} expected an empty page, got {} items",
                    page.items().len()
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A continuation token captured from one iterator resumes the feed in a fresh
/// iterator, yielding only the changes that occurred after the captured
/// position.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_continuation_token_resume() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let baseline = test_data::generate_mock_items(10, 5);
            let mut expected_baseline: Vec<MockItem> = baseline
                .iter()
                .filter(|item| item.partition_key == "partition0")
                .cloned()
                .collect();
            sort_by_id(&mut expected_baseline);

            let container =
                test_data::create_container_with_items(db_client, baseline, None).await?;

            // Drain the baseline, then capture a resume token.
            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    ChangeFeedStartFrom::Beginning,
                    None,
                )
                .await?;
            let mut first_batch = currents(drain_changes(&mut iterator).await?);
            sort_by_id(&mut first_batch);
            assert_eq!(expected_baseline, first_batch);

            // Round-trip the token through its string form to mimic persisting
            // it across processes.
            let token = iterator.to_continuation_token()?;
            let token = ContinuationToken::from_string(token.as_str().to_owned());
            drop(iterator);

            // Write new items after the captured position.
            let new_items: Vec<MockItem> = (0..3)
                .map(|i| MockItem {
                    id: format!("200{i}"),
                    partition_key: "partition0".to_string(),
                    merge_order: 2000 + i,
                })
                .collect();
            for item in &new_items {
                container
                    .create_item("partition0", &item.id, item, None)
                    .await?;
            }

            // Resume from the token: only the new items should appear.
            let mut resumed = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    // Ignored on resume: the token carries its own position.
                    ChangeFeedStartFrom::Beginning,
                    Some(ChangeFeedOptions::default().with_continuation_token(token)),
                )
                .await?;
            let mut second_batch = currents(drain_changes(&mut resumed).await?);
            sort_by_id(&mut second_batch);

            let mut expected_new = new_items;
            sort_by_id(&mut expected_new);

            assert_eq!(expected_new, second_batch);
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Regression: resuming a full-container `StartFrom::Now` feed after only some
/// physical partitions have been polled must NOT replay historical
/// (pre-checkpoint) changes on the partitions that were never polled.
///
/// Before the fix, the start-from position was applied only on the very first
/// (non-resumed) request, so on resume the partitions that had no saved token
/// were rebuilt as fresh reads and silently dumped their entire history from
/// the beginning.
///
/// Gated on `test_category = "emulator"` only — intentionally not run against
/// `test_category = "emulator_vnext"`. The precondition below requires the first
/// `StartFrom::Now` poll of a multi-partition, full-container feed to return an
/// empty page (baseline excluded), which the vnext (Linux) emulator does not
/// model reliably: its coarse commit-timestamp/LSN granularity can surface a
/// just-written baseline item as the tip of `If-None-Match: *`. The assertion is
/// non-deterministic there — in one weekly run it passed on one vnext agent and
/// failed on another — while it passes reliably on live accounts.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator' (the vnext emulator does not reliably model multi-partition StartFrom::Now)"
)]
pub async fn change_feed_now_resume_does_not_replay_history() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let baseline = test_data::generate_mock_items(10, 10);

            // 11000 RU/s forces at least 2 physical partitions so a single poll
            // leaves at least one partition unpolled (and thus without a saved
            // token) at checkpoint time.
            let container = test_data::create_container_with_items(
                db_client,
                baseline,
                Some(ThroughputProperties::manual(11000)),
            )
            .await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    ChangeFeedStartFrom::Now,
                    None,
                )
                .await?;

            // Poll once: with round-robin fan-out this advances (and captures an
            // ETag for) only one physical partition, leaving the rest unpolled.
            // `StartFrom::Now` excludes the baseline, so the page must be empty.
            let first = iterator
                .next()
                .await
                .expect("change feed stream always yields a page")?;
            assert!(
                first.items().is_empty(),
                "StartFrom::Now must not return baseline items, got {}",
                first.items().len()
            );

            // Round-trip the token through its string form to mimic persistence.
            let token = iterator.to_continuation_token()?;
            let token = ContinuationToken::from_string(token.as_str().to_owned());
            drop(iterator);

            // Resume with no intervening writes: every partition — polled or not
            // — must yield only empty pages. A non-empty page means an unpolled
            // partition replayed its history instead of honoring `Now`.
            let mut resumed = container
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    // Ignored on resume: the token carries its own position.
                    ChangeFeedStartFrom::Beginning,
                    Some(ChangeFeedOptions::default().with_continuation_token(token)),
                )
                .await?;

            let replayed = drain_changes(&mut resumed).await?;
            assert!(
                replayed.is_empty(),
                "resume replayed {} historical change(s); unpolled partitions must \
                 honor the original StartFrom::Now position rather than reading from \
                 the beginning",
                replayed.len()
            );
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// `StartFrom::PointInTime` begins the feed at a captured timestamp: changes
/// written before the marker are excluded and only changes written after it are
/// returned.
///
/// The change feed start time has one-second granularity (it is sent as an
/// RFC 1123 `If-Modified-Since` header), so the baseline writes, the captured
/// marker, and the post-marker writes are each separated by a short guard band
/// to keep the boundary unambiguous.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_point_in_time_excludes_earlier_changes() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            // Baseline items written before the point-in-time marker.
            let baseline = test_data::generate_mock_items(10, 5);
            let container =
                test_data::create_container_with_items(db_client, baseline, None).await?;

            // Guard band on each side of the captured marker so second-level
            // granularity cannot blur the baseline and the new writes together.
            tokio::time::sleep(Duration::from_secs(2)).await;
            let marker = OffsetDateTime::now_utc();
            tokio::time::sleep(Duration::from_secs(2)).await;

            // New items written strictly after the marker.
            let new_items: Vec<MockItem> = (0..3)
                .map(|i| MockItem {
                    id: format!("500{i}"),
                    partition_key: "partition0".to_string(),
                    merge_order: 5000 + i,
                })
                .collect();
            for item in &new_items {
                container
                    .create_item("partition0", &item.id, item, None)
                    .await?;
            }

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    ChangeFeedStartFrom::PointInTime(marker),
                    None,
                )
                .await?;

            let mut actual = currents(drain_changes(&mut iterator).await?);
            sort_by_id(&mut actual);

            let mut expected = new_items;
            sort_by_id(&mut expected);

            assert_eq!(
                expected, actual,
                "PointInTime must return only changes written after the marker"
            );
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A `max_item_count` limit caps how many items each change feed page returns,
/// so a backlog larger than the limit is delivered across multiple pages while
/// still surfacing every item exactly once.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn change_feed_max_item_count_pages_backlog() -> Result<(), Box<dyn Error>> {
    const PAGE_LIMIT: u32 = 10;

    TestClient::run_with_unique_db(
        async |_, db_client| {
            // 25 items in a single logical partition; at a page limit of 10 the
            // backlog must span multiple pages.
            let items: Vec<MockItem> = (0..25)
                .map(|i| MockItem {
                    id: format!("{i}"),
                    partition_key: "partition0".to_string(),
                    merge_order: i,
                })
                .collect();
            let mut expected = items.clone();
            sort_by_id(&mut expected);

            let container =
                test_data::create_container_with_items(db_client, items, None).await?;

            let mut iterator = container
                .query_change_feed::<MockItem>(
                    FeedScope::partition("partition0"),
                    ChangeFeedStartFrom::Beginning,
                    Some(ChangeFeedOptions::default().with_max_item_count(
                        MaxItemCountHint::Limit(NonZeroU32::new(PAGE_LIMIT).unwrap()),
                    )),
                )
                .await?;

            let mut collected: Vec<MockItem> = Vec::new();
            let mut non_empty_pages = 0usize;
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
                    non_empty_pages += 1;
                    assert!(
                        page.items().len() <= PAGE_LIMIT as usize,
                        "page returned {} items, exceeding the max_item_count limit of {PAGE_LIMIT}",
                        page.items().len()
                    );
                    collected.extend(currents(page.into_items()));
                }

                if polls >= MAX_DRAIN_POLLS {
                    break;
                }
            }

            assert!(
                non_empty_pages >= 2,
                "a 25-item backlog at a page limit of {PAGE_LIMIT} should span multiple pages, saw {non_empty_pages}"
            );
            sort_by_id(&mut collected);
            assert_eq!(
                expected, collected,
                "every item must be delivered exactly once across the paged reads"
            );
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
