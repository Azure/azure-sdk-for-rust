// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data-plane regional fallback tests for single-master multi-region accounts
//! at the **driver** layer.
//!
//! Single-master counterpart to
//! `multi_write_tests::driver_excluded_regions_fallback::excluded_all_regions_read_uses_hub_not_global`.
//! Same scenario — every preferred region in `excluded_regions` — but against
//! a single-master account where the only valid write endpoint is the hub.
//!
//! Gated by `test_category = "multi_region"` — requires a live multi-region
//! Cosmos DB account.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::options::{ExcludedRegions, OperationOptions, Region};
use std::error::Error;

/// The primary region where the account's hub is located.
/// Must match the first preferred write region of the test account.
const HUB_REGION: Region = Region::EAST_US_2;

/// The satellite (read-only) region of the test account.
const SATELLITE_REGION: Region = Region::WEST_US_3;

/// On a single-master multi-region account, when every preferred region is in
/// `excluded_regions`, the driver must fall back to the hub regional endpoint
/// — never the global `*.documents.azure.com` endpoint (which is not behind
/// ATM and therefore not safe for data-plane traffic).
///
/// Single-master counterpart to the multi-master driver test of the same
/// name. Validates the fix for issue #4487 at the driver layer.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn excluded_all_regions_read_uses_hub_not_global_single_master(
) -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Seed an item that the read will target.
        let item_id = format!("item-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let pk = format!("pk-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let body = format!(r#"{{"id": "{item_id}", "pk": "{pk}", "value": "test"}}"#);
        context
            .create_item(&container, &item_id, pk.clone(), body.as_bytes())
            .await?;

        // Warm replication so the satellite has the item before we exclude
        // every region.
        let _ = context.read_item(&container, &item_id, pk.clone()).await;

        // Exclude both regions — on a single-master account the only valid
        // write endpoint is the hub, so the driver must fall back to that
        // regional endpoint and never the global one.
        let options = OperationOptions {
            excluded_regions: Some(ExcludedRegions::from_iter([HUB_REGION, SATELLITE_REGION])),
            ..OperationOptions::default()
        };

        let response = context
            .read_item_with_options(&container, &item_id, pk, options)
            .await
            .expect("read should succeed via hub region fallback");

        let requests = response.diagnostics().requests();
        assert!(
            !requests.is_empty(),
            "diagnostics should record at least one request"
        );
        for req in requests.iter() {
            assert!(
                req.region().is_some(),
                "data-plane request should always target a regional endpoint, \
                 but a request with no region (global endpoint) was detected"
            );
        }

        Ok(())
    })
    .await
}
