// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

use super::framework;

use std::error::Error;

use azure_data_cosmos::{
    models::{ContainerProperties, ThroughputProperties},
    CreateContainerOptions, FeedRange,
};
use base64::Engine;

use framework::TestClient;

#[tokio::test]
pub async fn read_feed_ranges_returns_physical_partitions() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeContainer", "/pk".into());

            // Use 11000 RU/s to ensure at least 2 physical partitions (10000 RU/s per partition).
            let throughput = ThroughputProperties::manual(11000);
            let options = CreateContainerOptions::default().with_throughput(throughput);

            let container_client = run_context
                .create_container(db_client, properties, Some(options))
                .await?;

            let ranges = container_client.read_feed_ranges(None).await?;

            // With 11000 RU/s the service should create at least 2 physical partitions.
            assert!(
                ranges.len() >= 2,
                "expected at least 2 feed ranges with 11000 RU/s, got {}",
                ranges.len()
            );

            // All ranges should be contained within the full EPK space.
            let full = FeedRange::full();
            for range in &ranges {
                assert!(
                    full.contains(range),
                    "full range should contain every partition range"
                );
            }

            // No two ranges should overlap.
            for i in 0..ranges.len() {
                for j in (i + 1)..ranges.len() {
                    assert!(
                        !ranges[i].overlaps(&ranges[j]),
                        "ranges {i} and {j} should not overlap"
                    );
                }
            }

            // Each range should be serializable via Display and parseable via FromStr.
            for range in &ranges {
                let serialized = range.to_string();
                // Verify the serialized string is valid base64-encoded JSON
                // with the expected cross-SDK structure.
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(&serialized)
                    .expect("feed range Display should produce valid base64");
                let json: serde_json::Value =
                    serde_json::from_slice(&decoded).expect("decoded base64 should be valid JSON");
                let inner = json.get("Range").expect("expected 'Range' key");
                assert!(inner.get("min").is_some(), "expected 'min' field");
                assert!(inner.get("max").is_some(), "expected 'max' field");
                assert!(
                    inner.get("isMinInclusive").unwrap().as_bool().unwrap(),
                    "isMinInclusive should be true"
                );
                assert!(
                    !inner.get("isMaxInclusive").unwrap().as_bool().unwrap(),
                    "isMaxInclusive should be false"
                );

                // Verify FromStr can parse the serialized string and produces
                // a range contained within the full EPK space.
                let parsed: FeedRange = serialized
                    .parse()
                    .expect("feed range should be parseable from Display output");
                assert!(
                    full.contains(&parsed),
                    "parsed feed range should be within full EPK space"
                );
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

            // Use 11000 RU/s to ensure at least 2 physical partitions.
            let throughput = ThroughputProperties::manual(11000);
            let options = CreateContainerOptions::default().with_throughput(throughput);

            let container_client = run_context
                .create_container(db_client, properties, Some(options))
                .await?;

            // Get the physical partition ranges.
            let physical_ranges = container_client.read_feed_ranges(None).await?;

            // Get the feed range for a specific partition key.
            let pk_range = container_client
                .feed_range_from_partition_key("test_partition_key", None)
                .await?;

            // The returned range must match one of the physical partitions.
            let matches_physical = physical_ranges.iter().any(|pr| pr == &pk_range);
            assert!(
                matches_physical,
                "feed_range_from_partition_key should return one of the physical partition ranges"
            );

            // The same partition key should always map to the same range (deterministic).
            let pk_range_again = container_client
                .feed_range_from_partition_key("test_partition_key", None)
                .await?;
            assert_eq!(pk_range, pk_range_again, "same PK should map to same range");

            Ok(())
        },
        None,
    )
    .await
}
