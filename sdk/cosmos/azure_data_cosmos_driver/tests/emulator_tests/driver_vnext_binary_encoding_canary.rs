// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! TRACKING MARKER #5240 — canary pinning the vnext emulator's lack of Cosmos
//! binary JSON support.
//!
//! The vnext emulator cannot decode a binary request body and rejects item
//! writes with `400/1001 PartitionKeyMismatch`, so the test frameworks disable
//! binary encoding for every vnext run (`runtime_operation_options` in the
//! driver framework, `effective_binary_encoding` in the `azure_data_cosmos`
//! framework).
//!
//! This test re-enables binary encoding at the operation layer and asserts the
//! rejection, so it is green while the limitation holds and **goes red the
//! moment vnext starts accepting binary writes**. That failure is the signal to
//! delete both workarounds and let vnext runs use the normal binary default.

use crate::framework::DriverTestClient;
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    error::CosmosError,
    options::{BinaryEncodingOptions, OperationOptionsBuilder},
    SubStatusCode,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Emitted when the binary write succeeds — the limitation is gone.
const BINARY_NOW_SUPPORTED: &str = "vnext emulator accepted a binary-encoded item write: #5240 is \
     fixed. Remove `runtime_operation_options` (driver tests/framework/test_client.rs) and \
     `effective_binary_encoding` (azure_data_cosmos tests/framework/test_client.rs), then delete \
     this canary.";

/// Emitted when the write still fails, but not the way #5240 documents.
const UNEXPECTED_FAILURE: &str = "vnext emulator rejected a binary-encoded item write with an \
     unexpected status; the #5240 workaround rationale needs re-verification";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestItem {
    id: String,
    pk: String,
    value: String,
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_vnext"),
    ignore = "vnext-only canary; requires test_category 'emulator_vnext'"
)]
pub async fn vnext_still_rejects_binary_encoded_writes() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item = TestItem {
            id: "binary-canary-001".to_string(),
            pk: "partition-1".to_string(),
            value: "Hello, Cosmos!".to_string(),
        };
        let item_json = serde_json::to_vec(&item)?;

        // Operation layer outranks the framework's runtime-level default.
        let binary_on = OperationOptionsBuilder::new()
            .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(true))
            .build();

        let error = match context
            .create_item_with_operation_options(
                &container,
                &item.id,
                item.pk.clone(),
                &item_json,
                binary_on,
            )
            .await
        {
            Ok(_) => return Err(BINARY_NOW_SUPPORTED.into()),
            Err(error) => error,
        };

        let status = error
            .downcast_ref::<CosmosError>()
            .ok_or_else(|| format!("{UNEXPECTED_FAILURE}: not a CosmosError: {error}"))?
            .status();

        assert_eq!(
            status.status_code(),
            StatusCode::BadRequest,
            "{UNEXPECTED_FAILURE}"
        );
        assert_eq!(
            status.sub_status(),
            Some(SubStatusCode::PARTITION_KEY_MISMATCH),
            "{UNEXPECTED_FAILURE}"
        );

        Ok(())
    })
    .await
}
