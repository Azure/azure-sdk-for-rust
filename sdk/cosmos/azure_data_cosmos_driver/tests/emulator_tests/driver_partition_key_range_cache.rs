// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(feature = "__internal_testing")]

//! Live-service coverage for the partition key range cache.

use std::collections::BTreeSet;

use azure_data_cosmos_driver::models::PartitionKey;

use crate::framework::DriverTestClient;

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
async fn live_routing_map_cold_warm_and_refresh_paths() -> Result<(), Box<dyn std::error::Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let driver = context.create_persistent_driver().await?;

        let cold = driver
            .resolve_all_partition_key_ranges(&container, false)
            .await?
            .ok_or("cold PK-range resolution returned no map")?;
        assert!(!cold.is_empty(), "a live container must have PK ranges");
        let expected_ids: BTreeSet<_> = cold.iter().map(|range| range.id.clone()).collect();

        let warm = driver
            .resolve_all_partition_key_ranges(&container, false)
            .await?
            .ok_or("warm PK-range resolution returned no map")?;
        let warm_ids: BTreeSet<_> = warm.iter().map(|range| range.id.clone()).collect();
        assert_eq!(warm_ids, expected_ids);

        let refreshed = driver
            .resolve_all_partition_key_ranges(&container, true)
            .await?
            .ok_or("refreshed PK-range resolution returned no map")?;
        let refreshed_ids: BTreeSet<_> = refreshed.iter().map(|range| range.id.clone()).collect();
        assert_eq!(refreshed_ids, expected_ids);

        let owning = driver
            .resolve_partition_key_ranges_for_key(
                &container,
                &PartitionKey::from("live-routing-key"),
                false,
            )
            .await?
            .ok_or("point-key resolution returned no owning range")?;
        assert_eq!(owning.len(), 1);
        assert!(expected_ids.contains(&owning[0].id));

        Ok(())
    })
    .await
}
