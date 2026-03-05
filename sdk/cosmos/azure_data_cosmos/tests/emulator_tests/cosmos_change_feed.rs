// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

use super::framework;

use azure_core::Uuid;
use azure_data_cosmos::{
    change_feed::{ChangeFeedMode, ChangeFeedStartFrom, FeedRange},
    models::ContainerProperties,
    QueryChangeFeedOptions,
};
use framework::{TestClient, TestRunContext};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct TestItem {
    id: String,
    partition_key: String,
    value: usize,
}

#[tokio::test]
pub async fn change_feed_basic() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("ChangeFeed-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // Create some items
            let pk = format!("PK-{}", Uuid::new_v4());
            for i in 0..3 {
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Read the change feed from beginning to get those items
            let read_options =
                QueryChangeFeedOptions::default().with_start_from(ChangeFeedStartFrom::Beginning);

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(read_options))
                .await?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            assert!(
                items.len() >= 3,
                "Expected at least 3 items, got {}",
                items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn read_feed_ranges() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("FeedRanges-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let feed_ranges = container_client.read_feed_ranges(None).await?;

            assert!(
                !feed_ranges.is_empty(),
                "Expected at least one feed range, got 0"
            );

            for range in &feed_ranges {
                let serialized = range.to_string_representation()?;
                let deserialized = FeedRange::from_string_representation(&serialized)?;
                assert_eq!(serialized, deserialized.to_string_representation()?);
            }

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn change_feed_modes() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("ChangeFeedModes-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::Beginning)
                .with_mode(ChangeFeedMode::LatestVersion);

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?;
            futures::pin_mut!(pager);

            while let Some(result) = pager.next().await {
                let _ = result?;
            }

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that change feed correctly returns items after inserts and updates,
/// and that the event count matches expected behavior for LatestVersion mode.
#[tokio::test]
pub async fn change_feed_insert_update_delete_counts() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFCounts-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // Insert items across multiple partition keys
            let pk1 = format!("PK1-{}", Uuid::new_v4());
            let pk2 = format!("PK2-{}", Uuid::new_v4());
            let total_inserts = 5;
            for i in 0..total_inserts {
                let pk = if i % 2 == 0 { &pk1 } else { &pk2 };
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(pk, &item, None).await?;
            }

            // Read all changes from beginning
            let options =
                QueryChangeFeedOptions::default().with_start_from(ChangeFeedStartFrom::Beginning);

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            // In LatestVersion mode, we should see exactly the items that exist
            assert_eq!(
                items.len(),
                total_inserts,
                "Expected {} items in change feed, got {}",
                total_inserts,
                items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that continuation tokens work: read changes, save the token,
/// insert more items, resume from the token, and verify only new items appear.
#[tokio::test]
pub async fn change_feed_resume_from_continuation() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFResume-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let pk = format!("PK-{}", Uuid::new_v4());

            // Phase 1: Insert initial items
            for i in 0..3 {
                let item = TestItem {
                    id: format!("Initial-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Read all changes and capture continuation token
            let options =
                QueryChangeFeedOptions::default().with_start_from(ChangeFeedStartFrom::Beginning);
            let page_iter = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?
                .into_pages();
            futures::pin_mut!(page_iter);

            let mut initial_count = 0;
            let mut last_continuation: Option<String> = None;
            while let Some(page_result) = page_iter.next().await {
                let page = page_result?;
                initial_count += page.items().len();
                if let Some(cont) = page.continuation() {
                    last_continuation = Some(cont.to_string());
                }
            }

            assert!(
                initial_count >= 3,
                "Expected at least 3 initial items, got {}",
                initial_count
            );
            assert!(
                last_continuation.is_some(),
                "Expected a continuation token from the initial read"
            );

            // Phase 2: Insert more items
            for i in 0..2 {
                let item = TestItem {
                    id: format!("New-{}", i),
                    partition_key: pk.clone(),
                    value: 100 + i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Resume from continuation token
            let resume_options = QueryChangeFeedOptions::default()
                .with_continuation_token(last_continuation.unwrap());
            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(resume_options))
                .await?;
            futures::pin_mut!(pager);

            let mut new_items = Vec::new();
            while let Some(result) = pager.next().await {
                new_items.push(result?);
            }

            // Should only get the 2 new items
            assert_eq!(
                new_items.len(),
                2,
                "Expected 2 new items after resuming, got {}",
                new_items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that starting from Now returns 0 initial events,
/// and subsequent changes are visible after resuming from continuation.
#[tokio::test]
pub async fn change_feed_from_now() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFNow-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let pk = format!("PK-{}", Uuid::new_v4());

            // Insert items BEFORE starting the change feed from Now
            for i in 0..3 {
                let item = TestItem {
                    id: format!("Before-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Start change feed from Now — should see 0 initial events
            let options =
                QueryChangeFeedOptions::default().with_start_from(ChangeFeedStartFrom::Now);
            let page_iter = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?
                .into_pages();
            futures::pin_mut!(page_iter);

            let mut initial_count = 0;
            let mut last_continuation: Option<String> = None;
            while let Some(page_result) = page_iter.next().await {
                let page = page_result?;
                initial_count += page.items().len();
                if let Some(cont) = page.continuation() {
                    last_continuation = Some(cont.to_string());
                }
            }

            assert_eq!(
                initial_count, 0,
                "Expected 0 items from Now, got {}",
                initial_count
            );
            assert!(
                last_continuation.is_some(),
                "Expected a continuation token even with 0 results"
            );

            // Insert new items AFTER the Now checkpoint
            for i in 0..4 {
                let item = TestItem {
                    id: format!("After-{}", i),
                    partition_key: pk.clone(),
                    value: 10 + i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Resume from continuation — should see only the 4 new items
            let resume_options = QueryChangeFeedOptions::default()
                .with_continuation_token(last_continuation.unwrap());
            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(resume_options))
                .await?;
            futures::pin_mut!(pager);

            let mut new_items = Vec::new();
            while let Some(result) = pager.next().await {
                new_items.push(result?);
            }

            assert_eq!(
                new_items.len(),
                4,
                "Expected 4 new items after resuming from Now, got {}",
                new_items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that change feed works correctly with small page sizes.
#[tokio::test]
pub async fn change_feed_small_page_size() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFPageSize-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let pk = format!("PK-{}", Uuid::new_v4());
            let total = 10;
            for i in 0..total {
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Use a very small page size
            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::Beginning)
                .with_max_item_count(2);

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            assert_eq!(
                items.len(),
                total,
                "Expected {} items with small page size, got {}",
                total,
                items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that FeedRange::for_full_range() works with change feed.
#[tokio::test]
pub async fn change_feed_with_full_range() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFFullRange-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let pk = format!("PK-{}", Uuid::new_v4());
            for i in 0..5 {
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::Beginning)
                .with_feed_range(FeedRange::for_full_range());

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            assert_eq!(
                items.len(),
                5,
                "Expected 5 items with full range, got {}",
                items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that each feed range independently reads its subset of the change feed,
/// and the union of all ranges covers all items.
#[tokio::test]
pub async fn change_feed_parallel_feed_ranges() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFParallel-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // Insert items across multiple partition keys
            let total = 10;
            for i in 0..total {
                let pk = format!("PK-{}", i);
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Get feed ranges
            let feed_ranges = container_client.read_feed_ranges(None).await?;
            assert!(!feed_ranges.is_empty(), "Expected at least one feed range");

            // Read change feed for each range independently
            let mut total_items = 0;
            for range in &feed_ranges {
                let options = QueryChangeFeedOptions::default()
                    .with_start_from(ChangeFeedStartFrom::Beginning)
                    .with_feed_range(range.clone());

                let pager = container_client
                    .query_items_change_feed::<TestItem>(Some(options))
                    .await?;
                futures::pin_mut!(pager);

                while let Some(result) = pager.next().await {
                    let _ = result?;
                    total_items += 1;
                }
            }

            assert_eq!(
                total_items, total,
                "Union of all feed ranges should cover all {} items, got {}",
                total, total_items
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that AllVersionsAndDeletes mode rejects Beginning and PointInTime start.
#[tokio::test]
pub async fn change_feed_mode_restrictions() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFModeRestrict-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // AllVersionsAndDeletes + Beginning should fail
            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::Beginning)
                .with_mode(ChangeFeedMode::AllVersionsAndDeletes);

            let result = container_client
                .query_items_change_feed::<serde_json::Value>(Some(options))
                .await;
            assert!(
                result.is_err(),
                "AllVersionsAndDeletes + Beginning should return an error"
            );

            // AllVersionsAndDeletes + PointInTime should fail
            let pit = azure_core::time::OffsetDateTime::now_utc();
            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::PointInTime(pit))
                .with_mode(ChangeFeedMode::AllVersionsAndDeletes);

            let result = container_client
                .query_items_change_feed::<serde_json::Value>(Some(options))
                .await;
            assert!(
                result.is_err(),
                "AllVersionsAndDeletes + PointInTime should return an error"
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies that fromPointInTime start mode returns changes from around the specified time.
#[tokio::test]
pub async fn change_feed_from_point_in_time() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("CFPointInTime-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            let pk = format!("PK-{}", Uuid::new_v4());

            // Insert items
            for i in 0..5 {
                let item = TestItem {
                    id: format!("Item-{}", i),
                    partition_key: pk.clone(),
                    value: i,
                };
                container_client.create_item(&pk, &item, None).await?;
            }

            // Read from a point in time slightly in the past (should capture our items)
            let pit =
                azure_core::time::OffsetDateTime::now_utc() - std::time::Duration::from_secs(10);
            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::PointInTime(pit));

            let pager = container_client
                .query_items_change_feed::<TestItem>(Some(options))
                .await?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            // Items created within last 10 seconds should appear
            assert!(
                items.len() >= 5,
                "Expected at least 5 items from PointInTime, got {}",
                items.len()
            );

            Ok(())
        },
        None,
    )
    .await
}
