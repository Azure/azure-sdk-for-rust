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

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::error::CosmosError;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::models::AccountReference;
use azure_data_cosmos_driver::{CosmosStatus, SubStatusCode};
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

/// Asserts that the surfaced error from a faulted `GET /` preserves the
/// upstream HTTP status from the injected fault — i.e. it is NOT wrapped as
/// the synthetic `SERIALIZATION_RESPONSE_BODY_INVALID` (500/20020) that
/// today's account-metadata path produces when the body fails to deserialize
/// as `AccountProperties`. That wrapping is the precise bug shape behind
/// issue #4483: any non-2xx body (whether plain text or a Cosmos JSON error
/// envelope missing `_self`) currently surfaces as a generic serialization
/// failure instead of the upstream status.
///
/// Checking `status.status_code()` (and the sub-status when the fault sets
/// one) is structurally precise — it does not depend on `Display` text or
/// the exact body shape the fault injector chose to emit, both of which
/// could change without changing the user-observable behavior under test.
fn assert_preserves_upstream_status(
    err: &CosmosError,
    expected_status: StatusCode,
    expected_sub_status: Option<SubStatusCode>,
) {
    let status = err.status();
    assert_ne!(
        status,
        CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
        "issue #4483: error must preserve the upstream HTTP status, not be \
         relabeled as SERIALIZATION_RESPONSE_BODY_INVALID. Got: {err:?}"
    );
    assert_eq!(
        status.status_code(),
        expected_status,
        "expected the injected upstream status to be preserved. Got: {err:?}"
    );
    if let Some(expected_sub) = expected_sub_status {
        assert_eq!(
            status.sub_status(),
            Some(expected_sub),
            "expected the injected sub-status to be preserved. Got: {err:?}"
        );
    }
}

/// Behavioral coverage for the WriteForbidden (403/3) error path.
///
/// Injects a persistent 403 WriteForbidden on `GET /` and asserts that
/// `get_or_create_driver` surfaces the upstream HTTP status — not a serde
/// failure — to the caller. This closes the per-error-type slice of the
/// issue #4483 coverage gap for the WriteForbidden envelope on the
/// account-metadata path.
#[tokio::test]
async fn write_forbidden_on_metadata_preserves_upstream_status_issue_4483() {
    let Some(account) = build_account_from_env() else {
        eprintln!(
            "Skipping WriteForbidden metadata fault test: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, or AZURE_COSMOS_TEST_DATABASE unset"
        );
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

    assert_preserves_upstream_status(
        &err,
        StatusCode::Forbidden,
        Some(SubStatusCode::WRITE_FORBIDDEN),
    );
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
#[tokio::test]
async fn session_not_available_on_metadata_preserves_upstream_status_issue_4483() {
    let Some(account) = build_account_from_env() else {
        eprintln!(
            "Skipping ReadSessionNotAvailable metadata fault test: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, or AZURE_COSMOS_TEST_DATABASE unset"
        );
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

    assert_preserves_upstream_status(
        &err,
        StatusCode::NotFound,
        Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
    );
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
/// The test skips cleanly with a stderr notice when either
/// `AZURE_COSMOS_ENDPOINT` is unset or no `DeveloperToolsCredential` chain can
/// be constructed in the current environment, so it stays green in CI until those prerequisites
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
    // with the AAD bearer token. Under a persistent 503 it must surface the
    // upstream HTTP status — never the synthetic
    // `SERIALIZATION_RESPONSE_BODY_INVALID` (500/20020) that issue #4483
    // currently produces when the body can't be deserialized as
    // `AccountProperties`.
    let err = runtime
        .get_or_create_driver(account, None)
        .await
        .expect_err(
            "get_or_create_driver must fail under a persistent 503 fault on \
             MetadataReadDatabaseAccount, even with an AAD credential",
        );

    assert_preserves_upstream_status(&err, StatusCode::ServiceUnavailable, None);
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount fault should have been hit at least once"
    );
}
