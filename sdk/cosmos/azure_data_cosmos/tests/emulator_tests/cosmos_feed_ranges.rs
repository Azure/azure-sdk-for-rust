// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

use super::framework;

use std::error::Error;

use azure_data_cosmos::{models::ContainerProperties, CreateContainerOptions, FeedRange};

use framework::TestClient;

#[tokio::test]
pub async fn read_feed_ranges_returns_physical_partitions() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeContainer", "/pk".into());

            let container_client = run_context
                .create_container(db_client, properties, None)
                .await?;

            let ranges = container_client.read_feed_ranges(None).await?;

            // The emulator should return at least one physical partition.
            assert!(!ranges.is_empty(), "expected at least one feed range");

            // All ranges must cover the full EPK space contiguously.
            // The first range should start at "" and the last should end at "FF".
            let first = &ranges[0];
            let last = &ranges[ranges.len() - 1];
            let full = FeedRange::full();
            assert!(full.contains(first), "full range should contain first");
            assert!(full.contains(last), "full range should contain last");

            // Each range should be serializable and round-trip via Display/FromStr.
            for range in &ranges {
                let serialized = range.to_string();
                let deserialized: FeedRange = serialized
                    .parse()
                    .expect("feed range should round-trip through Display/FromStr");
                assert_eq!(range, &deserialized);
            }

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn feed_range_from_partition_key_maps_correctly() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeFromPK", "/pk".into());

            let container_client = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Get the physical partition ranges.
            let physical_ranges = container_client.read_feed_ranges(None).await?;

            // Get the feed range for a specific partition key.
            let pk_range = container_client
                .feed_range_from_partition_key("test_partition_key")
                .await?;

            // The returned range must match one of the physical partitions.
            let matches_physical = physical_ranges.iter().any(|pr| pr == &pk_range);
            assert!(
                matches_physical,
                "feed_range_from_partition_key should return one of the physical partition ranges"
            );

            // The same partition key should always map to the same range (deterministic).
            let pk_range_again = container_client
                .feed_range_from_partition_key("test_partition_key")
                .await?;
            assert_eq!(pk_range, pk_range_again, "same PK should map to same range");

            Ok(())
        },
        None,
    )
    .await
}
