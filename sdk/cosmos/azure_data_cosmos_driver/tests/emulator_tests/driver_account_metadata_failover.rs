// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator-level fault-injection coverage for the account-metadata (GET /) endpoint.
//! Complements the in-process unit tests in `src/driver/cosmos_driver.rs`.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::*;
use std::error::Error;
use std::sync::Arc;

/// Persistent 503 on GET / must surface as upstream HTTP status — never relabeled as
/// `SERIALIZATION_RESPONSE_BODY_INVALID` ("missing field `_self`").
#[tokio::test]
pub async fn account_metadata_503_surfaces_as_status_error() -> Result<(), Box<dyn Error>> {
    // Persistent 503 on every MetadataReadDatabaseAccount so the first lazy fetch
    // (via create_driver) hits the fault.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataReadDatabaseAccount)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("account-metadata-503-always", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // Use the variant without `_unique_db_`: the harness must NOT preemptively create
    // a database before our closure runs — we need to drive the first lazy fetch.
    DriverTestClient::run_with_fault_injection(rules, async |context| {
        let db_name = context.unique_database_name();

        // First op on the driver triggers the lazy account-properties fetch via
        // create_driver. Under a persistent 503 it must surface upstream HTTP status.
        let err = context.create_database(&db_name).await.expect_err(
            "create_database must fail when GET / is faulted with a persistent 503; \
                 the account-metadata fetch is the first network call and cannot succeed",
        );

        // Harness wraps the typed error in Box<dyn Error>; inspect Display/Debug for the bug
        // signature (the scripted-transport unit test asserts on the typed error directly).
        let rendered = format!("{err:?} | {err}");

        assert!(
            !rendered.contains("missing field `_self`"),
            "the user-visible error for a 503 on GET / must NOT leak the \
             internal `missing field \\`_self\\`` serde detail. Got: {rendered}"
        );

        assert!(
            rendered.contains("503") || rendered.to_lowercase().contains("serviceunavailable"),
            "the surfaced error should reflect the upstream HTTP 503 / \
             ServiceUnavailable status. Got: {rendered}"
        );

        assert!(
            rule.hit_count() > 0,
            "MetadataReadDatabaseAccount fault should have been hit at least once"
        );

        Ok(())
    })
    .await
}
