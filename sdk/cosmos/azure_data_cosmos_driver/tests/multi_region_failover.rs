// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for Step 2 multi-region failover, exercised via fault
//! injection at the HTTP transport layer.
//!
//! These tests are gated on:
//!   * `feature = "fault_injection"` — to install transport-level fault rules
//!     into the runtime via `CosmosDriverRuntime::builder().with_fault_injection_rules(...)`.
//!   * Environment variables (`AZURE_COSMOS_ENDPOINT`, `AZURE_COSMOS_KEY`,
//!     `AZURE_COSMOS_TEST_DATABASE`) — the assertions about region failover
//!     only have meaning against a real multi-region Cosmos topology.
//!
//! Each test installs a fault that reproduces one of the documented Step-2
//! multi-region failure modes (`WriteForbidden`, `ReadSessionNotAvailable`,
//! `ServiceUnavailable` on account metadata), drives a `read_database`
//! operation (or, for the AAD test, the lazy account-metadata fetch triggered
//! by `get_or_create_driver`), and asserts on two coverage outcomes:
//!
//!   1. **The fault was actually exercised** — `rule.hit_count() > 0` after
//!      the operation completes, so we know the test really did drive the
//!      fault-injection code path.
//!   2. **The surfaced error has the right shape** — it preserves the upstream
//!      HTTP status from the injected fault and does not get relabeled as a
//!      `missing field \`_self\`` serde failure (the bug subject of issue
//!      #4483).
//!
//! Deeper Step-8 assertions about internal state-machine effects
//! (`RefreshAccountProperties`, `MarkEndpointUnavailable`, etc.) require
//! observability hooks that do not yet exist on `CosmosDriver`; once those
//! land, each test can be extended in place without changing its
//! fault-injection setup.

#![cfg(feature = "fault_injection")]

use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::models::AccountReference;
use azure_identity::DeveloperToolsCredential;
use std::sync::Arc;

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn build_account_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let key = read_env("AZURE_COSMOS_KEY")?;
    let url = url::Url::parse(&endpoint).ok()?;
    Some(AccountReference::with_master_key(url, key))
}

/// Builds an `AccountReference` authenticated via `DeveloperToolsCredential`
/// (az login / azd login chain) for AAD-token coverage. Returns `None` if
/// `AZURE_COSMOS_ENDPOINT` is unset OR the credential chain cannot be
/// constructed in this environment.
///
/// Closes test-coverage **Gap 1** from the analysis on issue #4483: every
/// existing Cosmos integration test authenticates with master keys, which
/// are HMAC-signed locally and never validated against AAD. The entire
/// AAD-token error path (401 InvalidToken on token expiry, 403 from delayed
/// RBAC propagation, 403 from data-plane RBAC misconfiguration, 503 from
/// metadata-service hiccups during token acquisition) is therefore never
/// exercised in CI — which is exactly the shape of the production failure
/// reported in issue #4483.
fn build_account_with_token_credential_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let url = url::Url::parse(&endpoint).ok()?;
    let credential = DeveloperToolsCredential::new(None).ok()?;
    Some(AccountReference::with_credential(url, credential))
}

/// Installs a persistent fault that returns the given error type on every
/// `MetadataReadDatabaseAccount` request (i.e. on `GET /`, the account
/// properties endpoint).
///
/// `MetadataReadDatabaseAccount` is the right injection point for these
/// multi-region tests because:
///   1. It fires unconditionally on `get_or_create_driver`, so the fault is
///      guaranteed to be exercised regardless of which data-plane op runs
///      afterward.
///   2. It is the exact endpoint behind issue #4483, so the assertion that
///      the error surfaces as an HTTP-status error (not a serde
///      `missing field \`_self\`` failure) gives us the highest-value
///      coverage per test.
///
/// `read_database` itself does not currently map to a `FaultOperationType`
/// (see `FaultOperationType::from_operation_and_resource` in
/// `src/fault_injection/mod.rs`) so we cannot target the data-plane request
/// directly with an operation-type filter today.
fn build_account_metadata_fault_rule(
    id: &str,
    error_type: FaultInjectionErrorType,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataReadDatabaseAccount)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(error_type)
        .with_probability(1.0)
        .build();

    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    )
}

/// Asserts the rendered error from a faulted operation preserves the upstream
/// HTTP status keyword and does NOT leak the issue #4483 serde signature.
fn assert_no_self_serde_leak(rendered: &str, status_keyword: &str) {
    assert!(
        !rendered.contains("missing field `_self`"),
        "issue #4483: the user-visible error must NOT leak the internal \
         `missing field \\`_self\\`` serde detail. Got: {rendered}"
    );
    let lowered = rendered.to_lowercase();
    assert!(
        lowered.contains(&status_keyword.to_lowercase()),
        "surfaced error should reflect the injected fault `{status_keyword}`. Got: {rendered}"
    );
}

/// Behavioral coverage for the WriteForbidden (403/3) error path.
///
/// Injects a persistent 403 WriteForbidden on `GET /` and asserts that
/// `get_or_create_driver` surfaces the upstream HTTP status — not a serde
/// failure — to the caller. This closes the per-error-type slice of the
/// issue #4483 coverage gap for the WriteForbidden envelope on the
/// account-metadata path.
///
/// TODO(Step 8): once `CosmosDriver` exposes test-only observability for the
/// `RefreshAccountProperties` and `MarkEndpointUnavailable` state-machine
/// effects, extend this test to assert those effects fire after the 403/3 and
/// that the next retry targets a different region. Doing that today would
/// require adding new public-by-test API surface to `Driver`, which is out
/// of scope for the fix shipping in this PR.
#[tokio::test]
async fn write_forbidden_triggers_refresh_and_failover() {
    let Some(account) = build_account_from_env() else {
        return;
    };

    let rule = build_account_metadata_fault_rule(
        "multi-region-write-forbidden",
        FaultInjectionErrorType::WriteForbidden,
    );

    let runtime = CosmosDriverRuntime::builder()
        .with_fault_injection_rules(vec![Arc::clone(&rule)])
        .expect("rule installation should succeed")
        .build()
        .await
        .expect("runtime should be created");

    let err = runtime
        .get_or_create_driver(account, None)
        .await
        .expect_err(
            "get_or_create_driver must fail when GET / is faulted with a persistent 403 \
             WriteForbidden — the account-metadata fetch cannot complete",
        );

    let rendered = format!("{err:?} | {err}");
    assert_no_self_serde_leak(&rendered, "Forbidden");
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount WriteForbidden fault should have been hit at least once"
    );
}

/// Behavioral coverage for the ReadSessionNotAvailable (404/1002) error path.
///
/// Injects a persistent 404/1002 on `GET /` and asserts that
/// `get_or_create_driver` surfaces the upstream HTTP status — not a serde
/// failure. This closes the per-error-type slice of the issue #4483
/// coverage gap for the ReadSessionNotAvailable envelope on the
/// account-metadata path.
///
/// TODO(Step 8): once data-plane fault injection on `read_database` /
/// `read_item` is available and `CosmosDriver` exposes test-only observability
/// for session-retry decisions, extend this test to inject the fault on the
/// data-plane request itself and assert that:
///   - Single-write accounts retry the read in up to 2 preferred regions.
///   - Multi-write accounts retry across all endpoints in `endpoints.len()`.
///   - The retry succeeds in a different region when the fault is exhausted.
#[tokio::test]
async fn session_not_available_retries_across_locations() {
    let Some(account) = build_account_from_env() else {
        return;
    };

    let rule = build_account_metadata_fault_rule(
        "multi-region-read-session-not-available",
        FaultInjectionErrorType::ReadSessionNotAvailable,
    );

    let runtime = CosmosDriverRuntime::builder()
        .with_fault_injection_rules(vec![Arc::clone(&rule)])
        .expect("rule installation should succeed")
        .build()
        .await
        .expect("runtime should be created");

    let err = runtime
        .get_or_create_driver(account, None)
        .await
        .expect_err(
            "get_or_create_driver must fail when GET / is faulted with a persistent 404/1002 \
             ReadSessionNotAvailable — the account-metadata fetch cannot complete",
        );

    let rendered = format!("{err:?} | {err}");
    assert_no_self_serde_leak(&rendered, "NotFound");
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount ReadSessionNotAvailable fault should have been hit at least once"
    );
}

/// AAD-credential + fault-injection coverage — closes Gaps 1 and 3 from the
/// issue #4483 coverage analysis simultaneously.
///
/// Drives `get_or_create_driver` against a real Cosmos endpoint using
/// `DeveloperToolsCredential` (so the AAD token-acquisition + bearer-auth
/// pipeline runs end-to-end) while a persistent `ServiceUnavailable` (503)
/// fault is injected on `MetadataReadDatabaseAccount`.
///
/// Asserts:
///   * `get_or_create_driver` returns `Err` (the metadata fetch cannot
///     complete under a persistent 503).
///   * The surfaced error reflects the upstream HTTP 503 — it must NOT be
///     relabeled as a `missing field \`_self\`` serde failure (issue #4483).
///   * The fault rule was actually exercised (`hit_count() > 0`), so we know
///     the test really did drive the fault-injecting HTTP client.
///
/// Every other Cosmos integration test in this repo authenticates with
/// master keys, which are HMAC-signed locally and never validated against
/// AAD. The entire AAD-token error path (token-acquisition hiccups, 401
/// `InvalidToken`, 403 RBAC propagation, 503 from IMDS) is therefore never
/// exercised in CI — and that is exactly the shape of the production
/// failure reported in issue #4483.
///
/// The test skips cleanly (`return;`) when either `AZURE_COSMOS_ENDPOINT` is
/// unset or no `DeveloperToolsCredential` chain can be constructed in the
/// current environment, so it stays green in CI until those prerequisites
/// are provisioned. Run locally with:
///
/// ```sh
/// AZURE_COSMOS_ENDPOINT=... \
///   cargo test -p azure_data_cosmos_driver \
///   aad_token_credential_account_metadata_smoke_test \
///   --features fault_injection \
///   -- --nocapture
/// ```
#[tokio::test]
async fn aad_token_credential_account_metadata_smoke_test() {
    let Some(account) = build_account_with_token_credential_from_env() else {
        eprintln!(
            "Skipping AAD smoke test: AZURE_COSMOS_ENDPOINT unset or no usable credential chain"
        );
        return;
    };

    let rule = build_account_metadata_fault_rule(
        "aad-account-metadata-503",
        FaultInjectionErrorType::ServiceUnavailable,
    );

    let runtime = CosmosDriverRuntime::builder()
        .with_fault_injection_rules(vec![Arc::clone(&rule)])
        .expect("rule installation should succeed")
        .build()
        .await
        .expect("runtime should be created");

    // get_or_create_driver triggers the lazy account-metadata (`GET /`) fetch
    // with the AAD bearer token. Under a persistent 503 it must surface an
    // upstream HTTP-status error — never a serde "missing field `_self`"
    // failure (issue #4483).
    let err = runtime
        .get_or_create_driver(account, None)
        .await
        .expect_err(
            "get_or_create_driver must fail under a persistent 503 fault on \
             MetadataReadDatabaseAccount, even with an AAD credential",
        );

    let rendered = format!("{err:?} | {err}");
    assert!(
        !rendered.contains("missing field `_self`"),
        "issue #4483 regression: AAD + 503 on GET / must NOT surface as a \
         `missing field \\`_self\\`` serde failure. Got: {rendered}"
    );
    assert!(
        rendered.contains("503") || rendered.to_lowercase().contains("serviceunavailable"),
        "surfaced error should reflect the upstream HTTP 503 / ServiceUnavailable status. \
         Got: {rendered}"
    );
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount fault should have been hit at least once"
    );
}
