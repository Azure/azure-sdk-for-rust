// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Regression coverage for #5240: binary writes with flat and hierarchical partition keys.
//! Keep the exact documents: the original flat-PK failure depended on sorted member order.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::{
    models::{ContainerReference, PartitionKey},
    options::{BinaryEncodingOptions, OperationOptions, OperationOptionsBuilder},
};
use serde_json::json;
use std::error::Error;

/// Binary forced on at the operation layer, which outranks the runtime default
/// applied by the framework and by `patch_test_options`.
fn binary_on() -> OperationOptions {
    OperationOptionsBuilder::new()
        .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true))
        .build()
}

#[tokio::test]
#[cfg_attr(
    not(all(
        test_category = "emulator_vnext_binary",
        any(
            test_category = "emulator_vnext",
            test_category = "emulator_vnext_gateway_v2"
        )
    )),
    ignore = "requires a vnext binary matrix cell"
)]
pub async fn vnext_supports_binary_encoded_writes() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            // Seed from `cosmos_patch_pk_guard`.
            let flat_container = context
                .create_container(&database, &context.unique_container_name(), "/pk")
                .await?;
            let flat_id = "patch-pk-guard-001";
            let flat_pk = "tenant-a";
            let flat_body =
                serde_json::to_vec(&json!({ "id": flat_id, "pk": flat_pk, "name": "n" }))?;

            // Seed from `cosmos_patch_pk_guard_hierarchical`.
            let hpk_container = context
                .create_container_with_pk_paths(
                    &database,
                    &context.unique_container_name(),
                    &["/tenantId", "/userId"],
                )
                .await?;
            let hpk_id = "patch-hpk-001";
            let hpk_body = serde_json::to_vec(&json!({
                "id": hpk_id,
                "tenantId": "t1",
                "userId": "u1",
                "payload": "hello",
            }))?;

            let cases: Vec<(&str, &ContainerReference, &str, PartitionKey, &[u8])> = vec![
                (
                    "flat PK",
                    &flat_container,
                    flat_id,
                    PartitionKey::from(flat_pk),
                    &flat_body,
                ),
                (
                    "hierarchical PK",
                    &hpk_container,
                    hpk_id,
                    ("t1", "u1").into(),
                    &hpk_body,
                ),
            ];

            for (label, container, item_id, partition_key, body) in cases {
                context
                    .create_item_with_operation_options(
                        container,
                        item_id,
                        partition_key,
                        body,
                        binary_on(),
                    )
                    .await
                    .map_err(|error| {
                        format!("[{label}] vnext binary write regression (#5240): {error}")
                    })?;
            }

            Ok(())
        },
    ))
    .await
}
