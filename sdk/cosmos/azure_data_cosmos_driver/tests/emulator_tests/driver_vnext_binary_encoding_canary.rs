// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! TRACKING MARKER #5240 — canary pinning the vnext emulator's lack of Cosmos
//! binary JSON support.
//!
//! Reproduces the exact mechanism from #5240: the `create_seed_item` calls in
//! `driver_patch::cosmos_patch_pk_guard` and
//! `driver_patch::cosmos_patch_pk_guard_hierarchical`, which failed with
//! `400/1001 PartitionKeyMismatch` once the document body went out binary. The
//! emulator compares the partition key decoded from the body against the text
//! `x-ms-documentdb-partitionkey` header, so failing to decode a binary body
//! surfaces as a mismatch rather than a parse error.
//!
//! Those tests no longer reproduce it themselves — `patch_test_options()` pins
//! binary off for the whole patch suite, and
//! `framework::test_client::runtime_operation_options` pins it off for every
//! other vnext run. This canary re-enables binary at the **operation** layer,
//! which outranks both.
//!
//! Runs in vnext CI and asserts that binary writes succeed, so an emulator
//! regression fails the pipeline rather than remaining ignored.
//!
//! **When it passes, vnext supports binary encoding.** That is the signal to
//! drop `patch_test_options`'s binary override, `runtime_operation_options`,
//! `effective_binary_encoding` (azure_data_cosmos framework), and this canary.
//!
//! Use these exact documents. The rejection is **document-shape dependent**,
//! not blanket: a flat-PK document is accepted whenever `pk` immediately
//! follows `id` in sorted key order (the binary writer emits members sorted),
//! and rejected when another property sorts between them — `{id, pk, value}`
//! is accepted while `{id, name, pk}` is not. Hierarchical PK is rejected
//! unconditionally. A hand-rolled document is therefore liable to pass and
//! report the limitation as lifted while it is still very much present.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::{
    models::{ContainerReference, PartitionKey},
    options::{BinaryEncodingOptions, OperationOptions, OperationOptionsBuilder},
};
use serde_json::json;
use std::error::Error;

/// The expected outcome while #5240 is open — normally `400/1001
/// PartitionKeyMismatch`.
const STILL_UNSUPPORTED: &str = "vnext emulator still rejects a binary-encoded item write, so \
     #5240 remains open and the binary overrides in `patch_test_options` (driver \
     tests/emulator_tests/driver_patch.rs), `runtime_operation_options` (driver \
     tests/framework/test_client.rs) and `effective_binary_encoding` (azure_data_cosmos \
     tests/framework/test_client.rs) must stay";

/// Binary forced on at the operation layer, which outranks the runtime default
/// applied by the framework and by `patch_test_options`.
fn binary_on() -> OperationOptions {
    OperationOptionsBuilder::new()
        .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true))
        .build()
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_vnext"),
    ignore = "vnext-only canary; requires test_category 'emulator_vnext'"
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
                    .map_err(|error| format!("[{label}] {STILL_UNSUPPORTED}: {error}"))?;
            }

            Ok(())
        },
    ))
    .await
}
