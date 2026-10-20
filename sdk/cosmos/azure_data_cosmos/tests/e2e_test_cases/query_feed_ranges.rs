// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::should_run,
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn feed_ranges_cover_partition_key_routing() -> TestResult {
    if !should_run("query.feed-ranges").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        let physical_ranges = fixture.container.read_feed_ranges(None).await?;
        assert!(!physical_ranges.is_empty());
        for (index, left) in physical_ranges.iter().enumerate() {
            assert!(!left.to_string().is_empty());
            for right in physical_ranges.iter().skip(index + 1) {
                assert_ne!(left, right, "physical feed ranges must be distinct");
            }
        }

        let mapped = fixture
            .container
            .feed_range_from_partition_key("routing-key", None)
            .await?;
        assert_eq!(mapped.len(), 1);
        assert!(physical_ranges.contains(&mapped[0]));
        let repeated = fixture
            .container
            .feed_range_from_partition_key("routing-key", None)
            .await?;
        assert_eq!(mapped, repeated);
        Ok(())
    })
    .await
}
