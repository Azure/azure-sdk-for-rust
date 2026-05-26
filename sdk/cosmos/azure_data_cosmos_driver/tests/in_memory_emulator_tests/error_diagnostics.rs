// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration test for diagnostics attachment on the abort path.
//!
//! Guards against the regression where the operation pipeline's abort branch
//! returns the `Error` without grafting the operation's
//! [`DiagnosticsContext`] (retry history, region attempts, per-request
//! events) onto it. The success path attaches diagnostics to
//! [`CosmosResponse`]; the failure path must mirror that contract on
//! `Error::diagnostics()`. Without this coverage, a refactor that drops the
//! `error.with_diagnostics(diagnostics.complete())` call at the abort site
//! would silently regress observability for every failed operation.

use std::sync::Arc;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::OperationOptions;

const GATEWAY_URL: &str = "https://eastus.emulator.local";

fn build_emulator() -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    // No databases are created — every read_database below will return 404.
    Arc::new(InMemoryEmulatorHttpClient::new(config))
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Regression guard for diagnostics-on-abort. Reading a non-existent
/// database produces a 404 that the retry pipeline routes to
/// `OperationAction::Abort`. The returned `Error` must carry the
/// operation's real per-attempt diagnostics — not `None`, and not the
/// process-wide `error_placeholder()` that `build_service_error` stamps
/// onto the wire-level payload before the pipeline gets a chance to
/// upgrade it.
#[tokio::test]
async fn aborted_operation_error_carries_operation_diagnostics() {
    let emulator = build_emulator();

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize against the in-memory emulator");

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "nonexistent".to_string());

    let err = driver
        .execute_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await
        .expect_err("read of nonexistent database must surface a 404 error");

    let diagnostics = err
        .diagnostics()
        .expect("aborted operation error must carry the operation's DiagnosticsContext");

    // The placeholder `error_placeholder()` has zero per-request entries and
    // the all-zeros activity id. The real operation diagnostics minted by
    // `execute_operation_pipeline` records at least one attempt against the
    // emulator and uses a freshly generated activity id, so both checks are
    // sufficient to distinguish the two.
    assert!(
        diagnostics.request_count() >= 1,
        "operation diagnostics must record the failing HTTP attempt; got {} requests",
        diagnostics.request_count(),
    );
    assert_ne!(
        diagnostics.activity_id().to_string(),
        "00000000-0000-0000-0000-000000000000",
        "operation diagnostics must use a real activity id, not the error placeholder",
    );
}
