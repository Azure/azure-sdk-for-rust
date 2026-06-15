// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! E2E tests for the hedging enablement kill-switch environment variable,
//! exercised through a *live* runtime build and a real read against the
//! emulator.
//!
//! ## Why this test does not assert that a hedge leg fires
//!
//! Cross-region read hedging only dispatches a second leg when the account
//! exposes at least two applicable preferred regions (the `should_hedge` gate;
//! see `docs/HEDGING_SPEC.md` §5). The single-region emulator never satisfies
//! that gate, so no emulator test can observe a hedge actually firing or being
//! suppressed. These tests therefore validate the part this layer newly adds —
//! and that the macro unit tests (which use the injectable `from_env_vars` /
//! `from_env_override_vars`) cannot cover: that the *real*
//! `OperationOptions::from_env_override()` + [`CosmosDriverRuntime`] assembly
//! reads the `{ENV}_OVERRIDE` kill switch (honoring lenient boolean spellings),
//! surfaces it on the top-priority env-override layer, and does not break the
//! live data path while the kill switch is active.

use crate::framework::DriverTestClient;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// A simple test item for the kill-switch data-path check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestItem {
    id: String,
    pk: String,
    value: String,
}

/// `AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE` set to a lenient spelling (`OFF`) is
/// parsed to `Some(false)` by the *real* runtime build, surfaces on the
/// top-priority env-override layer, and a real create + read still succeeds.
///
/// This exercises the disable direction of the kill switch (the entire point
/// of an incident kill switch) end-to-end: the env var → `from_env_override()`
/// → runtime env-override layer wiring, plus the live transport data path.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hedging_override_env_var_reaches_runtime_and_keeps_reads_healthy(
) -> Result<(), Box<dyn Error>> {
    // The `{ENV}_OVERRIDE` value is read once at runtime-build time from the
    // process environment, so it must be set *before* the runtime is built
    // inside `run_with_unique_db`. `set_var` is safe on edition 2021; the blast
    // radius across the emulator suite is nil because hedging never fires on the
    // single-region emulator regardless of this switch. The variable is removed
    // again as soon as the run completes (success or failure).
    std::env::set_var("AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE", "OFF");

    let outcome = Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
            // The runtime was built (inside `run_with_unique_db`) after the env
            // var was set above, so the `{ENV}_OVERRIDE` → `from_env_override()`
            // → runtime env-override layer must reflect the lenient `OFF`
            // spelling as `Some(false)`.
            let env_override = context.runtime().env_override_operation_options();
            assert_eq!(
                env_override.hedging_enabled,
                Some(false),
                "the lenient `OFF` spelling of AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE \
                 must resolve to Some(false) through the live runtime build",
            );

            // The kill switch must not break the live data path: a real create
            // and read against the emulator still succeed.
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "kill-switch-doc-1".to_string(),
                pk: "partition-1".to_string(),
                value: "hedging disabled by kill switch".to_string(),
            };
            let body = serde_json::to_vec(&item)?;

            let create_result = context
                .create_item(&container, &item.id, item.pk.clone(), &body)
                .await?;
            let create_diagnostics = create_result.diagnostics();
            context.validate_data_plane_diagnostics(&create_diagnostics, 201);

            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await?;
            let read_diagnostics = read_result.diagnostics();
            context.validate_data_plane_diagnostics(&read_diagnostics, 200);

            let read_item: TestItem = read_result.into_body().into_single()?;
            assert_eq!(read_item, item);

            Ok(())
        },
    ))
    .await;

    std::env::remove_var("AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE");
    outcome
}
