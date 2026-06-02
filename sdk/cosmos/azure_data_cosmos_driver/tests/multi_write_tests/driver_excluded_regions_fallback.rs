// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data-plane regional fallback tests for multi-master accounts at the
//! **driver** layer.
//!
//! These are the driver-layer counterpart to
//! `azure_data_cosmos::tests::multi_write_tests::cosmos_multi_write_fault_injection::excluded_all_regions_read_uses_hub_not_global`.
//! The SDK-layer test exercises the public `ExcludedRegions` API on the SDK
//! client; this test drives the same routing through `CosmosDriverRuntime`
//! and `OperationOptions` directly, asserting the fix in
//! `driver::pipeline::operation_pipeline::resolve_endpoint` at the layer that
//! actually owns it.
//!
//! Gated by `test_category = "multi_write"` — requires a live multi-master
//! Cosmos DB account.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::options::{ExcludedRegions, OperationOptions, Region};
use std::error::Error;

/// The primary write region of the test account.
/// Must match the first preferred write region.
const HUB_REGION: Region = Region::EAST_US_2;

/// The satellite region (second preferred region) of the test account.
const SATELLITE_REGION: Region = Region::WEST_US_3;

/// On a multi-master account, when every preferred region is in
/// `excluded_regions`, the driver must still issue the read through a
/// regional endpoint (the hub write endpoint as last-resort fallback) and
/// never through the global `*.documents.azure.com` endpoint — which is not
/// behind ATM and therefore not safe for data-plane traffic.
///
/// Validates the fix for issue #4487 at the driver layer.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn excluded_all_regions_read_uses_hub_not_global() -> Result<(), Box<dyn Error>> {
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

        // Exclude both regions — there is no preferred region left for the
        // SDK to pick. The hub write endpoint must be used as the last-resort
        // regional fallback; the global endpoint must NEVER be used.
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
