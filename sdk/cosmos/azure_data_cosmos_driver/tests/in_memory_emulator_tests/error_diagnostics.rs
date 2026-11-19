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

use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::{DiagnosticsVerbosity, DriverOptions, OperationOptions};

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
    AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Regression guard for diagnostics-on-abort. Reading a non-existent
/// database produces a 404 that the retry pipeline routes to
/// `OperationAction::Abort`. The returned `Error` must carry the
/// operation's real per-attempt diagnostics — not `None`, and not a
/// default/empty context. The retry layer (`build_service_error`)
/// intentionally constructs the typed `Error` with `diagnostics: None`
/// and relies on the operation pipeline's abort branch to graft the
/// operation's completed `DiagnosticsContext` onto the error via
/// `Error::with_diagnostics` before it leaves the pipeline.
#[tokio::test]
async fn aborted_operation_error_carries_operation_diagnostics() {
    let emulator = build_emulator();

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize against the in-memory emulator");

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "nonexistent".to_string());

    let before = SystemTime::now();
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

    assert!(diagnostics.start_time() >= before);
    assert!(diagnostics.start_time() <= SystemTime::now());
    let json: serde_json::Value =
        serde_json::from_str(diagnostics.to_json_string(Some(DiagnosticsVerbosity::Detailed)))
            .unwrap();
    assert!(json["start_time_utc"].as_str().unwrap().ends_with('Z'));
    for (request, json) in diagnostics
        .requests()
        .iter()
        .zip(json["requests"].as_array().unwrap())
    {
        assert!(request.start_time() >= diagnostics.start_time());
        assert!(json["start_time_utc"].as_str().unwrap().ends_with('Z'));
        for event in request.events() {
            let wall_elapsed = event
                .timestamp_utc()
                .duration_since(request.start_time())
                .unwrap();
            let monotonic_elapsed = event.timestamp().duration_since(request.started_at());
            // Windows SystemTime arithmetic has 100 ns precision.
            assert!(wall_elapsed.abs_diff(monotonic_elapsed) <= Duration::from_nanos(100));
        }
    }

    // A default/empty `DiagnosticsContext` would have zero per-request
    // entries and a placeholder activity id. The real operation
    // diagnostics minted by `execute_operation_pipeline` records at
    // least one attempt against the emulator and uses a freshly
    // generated activity id, so both checks are sufficient to
    // distinguish the two.
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
