// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// cspell:ignore serviceunavailable

//! Integration coverage for account-metadata (`GET /`) fault behavior tested
//! against the local emulator via fault injection.
//!
//! Closes test-coverage **Gap 3** from the analysis on issue #4483: the
//! account-metadata endpoint is hit on every client startup and (after
//! issue #4407) every 5 minutes for the lifetime of every long-running
//! process, but it has no fault-injection coverage today. The companion
//! coverage for `MetadataPartitionKeyRanges` lives in
//! `driver_partition_failover.rs`; this file mirrors that pattern but
//! targets `FaultOperationType::MetadataReadDatabaseAccount` instead.
//!
//! This is the end-to-end emulator-level complement to the in-process unit
//! test `fetch_account_properties_surfaces_5xx_body_as_status_error_issue_4483`
//! in `src/driver/cosmos_driver.rs`: that one drives the parser directly with
//! a scripted transport, while this one exercises the real fault-injecting
//! HTTP client factory wired into the runtime.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::*;
use std::error::Error;
use std::sync::Arc;

/// Repro for issue #4483: when `GET /` (the account-metadata endpoint) returns
/// a persistent 503 with an error body, the surfaced error must reflect the
/// upstream HTTP status — it must NOT be relabeled as a
/// `SERIALIZATION_RESPONSE_BODY_INVALID` (500.20020) `missing field \`_self\``
/// serde failure.
///
/// This is the integration-level mirror of the existing in-process unit test
/// `fetch_account_properties_surfaces_5xx_body_as_status_error_issue_4483`.
/// Both will start passing once the parser at
/// `cosmos_driver.rs::parse_account_properties_payload` is gated on
/// `response.is_success()` (or equivalent) before attempting to deserialize
/// the body as `AccountProperties`.
///
/// Marked `#[ignore]` because the bug is present at HEAD — the test will fail
/// until the fix lands. After the fix, the maintainer should swap the
/// attribute to:
///
/// ```ignore
/// #[cfg_attr(not(test_category = "emulator"), ignore = "requires test_category 'emulator'")]
/// ```
///
/// Run locally with:
///
/// ```sh
/// cargo test -p azure_data_cosmos_driver \
///   account_metadata_503_surfaces_as_status_error_issue_4483 \
///   --features fault_injection \
///   -- --ignored --nocapture
/// ```
#[tokio::test]
pub async fn account_metadata_503_surfaces_as_status_error_issue_4483() -> Result<(), Box<dyn Error>>
{
    // Inject a persistent 503 on ALL MetadataReadDatabaseAccount requests so
    // that the very first call (driven by `get_or_create_driver` → account
    // properties cache miss → `fetch_account_properties`) hits the fault.
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

    // Use `run_with_fault_injection` (not the `_unique_db_` variant) so the
    // harness does NOT preemptively create a database before our closure
    // runs — we need to be the ones who first trigger the lazy account
    // metadata fetch so we can inspect the resulting error directly.
    DriverTestClient::run_with_fault_injection(rules, async |context| {
        let db_name = context.unique_database_name();

        // The first operation against the driver triggers the lazy account
        // properties fetch via `runtime.get_or_create_driver(...)`. With the
        // persistent 503 fault active, that fetch must fail with an upstream
        // HTTP-status error — NOT a serde "missing field `_self`" error.
        let err = context.create_database(&db_name).await.expect_err(
            "create_database must fail when GET / is faulted with a persistent 503; \
                 the account-metadata fetch is the first network call and cannot succeed",
        );

        // The harness wraps the typed error in `Box<dyn Error>`, so we
        // inspect its Display/Debug representations for the bug signature.
        // The existing scripted-transport unit test asserts on the typed
        // error directly; this one asserts on the user-visible string shape
        // that customers will see in logs.
        let rendered = format!("{err:?} | {err}");

        assert!(
            !rendered.contains("missing field `_self`"),
            "issue #4483: the user-visible error for a 503 on GET / must NOT leak the \
             internal `missing field \\`_self\\`` serde detail. Got: {rendered}"
        );

        assert!(
            rendered.contains("503") || rendered.to_lowercase().contains("serviceunavailable"),
            "issue #4483: the surfaced error should reflect the upstream HTTP 503 / \
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
