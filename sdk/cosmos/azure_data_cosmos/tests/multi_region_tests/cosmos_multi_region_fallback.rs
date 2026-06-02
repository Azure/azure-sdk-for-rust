// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data-plane regional fallback tests for single-master multi-region accounts.
//!
//! This module is the single-master counterpart to
//! `multi_write_tests::cosmos_multi_write_fault_injection::excluded_all_regions_read_uses_hub_not_global`.
//! Both tests assert the same invariant — when every preferred region is in
//! `excluded_regions`, the SDK must still route data-plane requests through a
//! regional endpoint (never the global `*.documents.azure.com` endpoint, which
//! is not behind ATM). Splitting them across `test_category` gates lets CI
//! cover both account topologies via the existing `live-platform-matrix.json`
//! entries:
//!
//! * `multi_write` → multi-master account (the existing test).
//! * `multi_region` → single-master multi-region account (this test).

#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::Uuid;
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::{ExcludedRegions, ItemReadOptions, OperationOptions};
use framework::{TestClient, HUB_REGION, SATELLITE_REGION};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NestedItem {
    nested_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
    nested: NestedItem,
    bool_value: bool,
}

/// Single-master counterpart to
/// `cosmos_multi_write_fault_injection::excluded_all_regions_read_uses_hub_not_global`.
///
/// On a single-master multi-region account (East US 2 hub, West US 3 read-only
/// satellite), exclude both regions and verify the SDK falls back to the hub
/// write region rather than the global endpoint. This is the scenario raised
/// on PR #4503: the same all-regions-excluded configuration can occur on a
/// single-master account when the operator switches the hub region to a new
/// location while the application has both the old and new regions in its
/// excluded list.
///
/// Validates the fix for issue #4487.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn excluded_all_regions_read_uses_hub_not_global_single_master(
) -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let container_client = db_client.container_client(&container_id).await?;
            let unique_id = Uuid::new_v4().to_string();
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("item-{}", unique_id);
            let item = TestItem {
                id: item_id.clone().into(),
                partition_key: Some(pk.clone().into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "test".to_string(),
                },
                bool_value: true,
            };

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            // Ensure replication to the read region before excluding it.
            let _ = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await;

            // Exclude both regions — on a single-master account the only
            // valid write endpoint is the hub, so the SDK must fall back to
            // that regional endpoint and never the global one.
            let mut operation = OperationOptions::default();
            operation.excluded_regions =
                Some(ExcludedRegions::from_iter([HUB_REGION, SATELLITE_REGION]));
            let options = ItemReadOptions::default().with_operation_options(operation);

            let response = run_context
                .read_item(&container_client, &pk, &item_id, Some(options))
                .await
                .expect("read should succeed via hub region fallback");

            // Assert that every request went to a regional endpoint (has a
            // region), proving the global endpoint was never used.
            let requests = response.diagnostics().requests();
            for req in requests.iter() {
                assert!(
                    req.region().is_some(),
                    "data-plane request should always target a regional endpoint, \
                     but a request with no region (global endpoint) was detected"
                );
            }

            Ok(())
        },
        None,
    )
    .await
}
