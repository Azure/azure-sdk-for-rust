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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: String,
    partition_key: String,
    value: usize,
}

#[tokio::test]
pub async fn change_feed_basic() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context: &TestRunContext, db_client| {
            // Create a container for this test
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

            let pager = container_client.query_items_change_feed::<TestItem>(Some(read_options))?;
            futures::pin_mut!(pager);

            let mut items = Vec::new();
            while let Some(result) = pager.next().await {
                items.push(result?);
            }

            // Verify we got our items (order not guaranteed)
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
            // Create a container for this test
            let container_id = format!("FeedRanges-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // Read feed ranges
            let feed_ranges = container_client.read_feed_ranges(None).await?;

            // Should have at least one feed range
            assert!(
                !feed_ranges.is_empty(),
                "Expected at least one feed range, got 0"
            );

            // Each feed range should be serializable for persistence
            for range in &feed_ranges {
                let serialized = range.to_string_representation()?;
                let deserialized = FeedRange::from_string_representation(&serialized)?;
                // Round-trip should work
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
            // Create a container for this test
            let container_id = format!("ChangeFeedModes-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;
            let container_client = db_client.container_client(&container_id).await;

            // Test LatestVersion mode
            let options = QueryChangeFeedOptions::default()
                .with_start_from(ChangeFeedStartFrom::Beginning)
                .with_mode(ChangeFeedMode::LatestVersion);

            let pager = container_client.query_items_change_feed::<TestItem>(Some(options))?;
            futures::pin_mut!(pager);

            // Should be able to iterate (even if empty)
            while let Some(result) = pager.next().await {
                let _ = result?;
            }

            // Note: AllVersionsAndDeletes mode requires a specific account configuration
            // and is not tested here as it may not be available on all emulators

            Ok(())
        },
        None,
    )
    .await
}
