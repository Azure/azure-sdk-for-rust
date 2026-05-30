// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for Step 2 multi-region failover.
//!
//! These tests are environment-gated because they require a multi-region Cosmos
//! account topology and real account metadata refresh behavior.
//!
//! TODO(Step 8): These are currently smoke tests — they confirm an operation
//! runs without crashing but do not inject faults or assert on retry/failover
//! behavior. When fault injection lands in Step 8, convert these to real
//! behavioral tests with assertions on retry counts, endpoint selection, and
//! effect application.

use azure_data_cosmos_driver::models::AccountReference;
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::{driver::CosmosDriverRuntime, models::CosmosOperation};
use azure_identity::DeveloperToolsCredential;

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

#[tokio::test]
#[ignore = "Requires live multi-region Cosmos account and credentials"]
async fn write_forbidden_triggers_refresh_and_failover() {
    // TODO(Step 8): Inject a 403/3 WriteForbidden fault and assert that:
    // - Account properties are refreshed (RefreshAccountProperties effect)
    // - The endpoint is marked unavailable (MarkEndpointUnavailable effect)
    // - The retry targets a different region
    let Some(account) = build_account_from_env() else {
        return;
    };

    let runtime = CosmosDriverRuntime::builder()
        .build()
        .await
        .expect("runtime should be created");
    let driver = runtime
        .get_or_create_driver(account, None)
        .await
        .expect("driver should be created");

    let db = read_env("AZURE_COSMOS_TEST_DATABASE").expect("missing test database env");
    let db_ref = azure_data_cosmos_driver::models::DatabaseReference::from_name(
        driver.account().clone(),
        db,
    );

    let _ = driver
        .execute_singleton_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await;
}

#[tokio::test]
#[ignore = "Requires live multi-region Cosmos account and credentials"]
async fn session_not_available_retries_across_locations() {
    // TODO(Step 8): Inject a 404/1002 ReadSessionNotAvailable fault and assert:
    // - Session retry advances to the next preferred region
    // - Single-write accounts retry up to 2 times, multi-write up to endpoints.len()
    // - The operation succeeds after retrying in another region
    let Some(account) = build_account_from_env() else {
        return;
    };

    let runtime = CosmosDriverRuntime::builder()
        .build()
        .await
        .expect("runtime should be created");
    let driver = runtime
        .get_or_create_driver(account, None)
        .await
        .expect("driver should be created");

    let db = read_env("AZURE_COSMOS_TEST_DATABASE").expect("missing test database env");
    let db_ref = azure_data_cosmos_driver::models::DatabaseReference::from_name(
        driver.account().clone(),
        db,
    );

    let _ = driver
        .execute_singleton_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await;
}

/// AAD-credential smoke test — closes Gaps 1 and 4 from the issue #4483
/// coverage analysis.
///
/// Drives a single `read_database` operation against a real Cosmos account
/// using `DeveloperToolsCredential`, which exercises the AAD token-acquisition
/// + bearer-auth path end-to-end. Every other Cosmos integration test in
/// this repo authenticates with master keys, so the AAD code path (token
/// expiry/refresh, 401 `InvalidToken`, 403 RBAC propagation, 503 from IMDS
/// during token acquisition) has zero CI coverage today — which is the
/// production trigger reported in issue #4483.
///
/// `#[ignore]`-gated until a live multi-region account + AAD-capable
/// identity (workload identity / Managed Identity / az login) is wired into
/// CI. Run locally with:
///
/// ```sh
/// AZURE_COSMOS_ENDPOINT=... AZURE_COSMOS_TEST_DATABASE=... \
///   cargo test -p azure_data_cosmos_driver \
///   aad_token_credential_account_metadata_smoke_test \
///   -- --ignored --nocapture
/// ```
///
/// TODO(Step 8): extend with fault injection on `MetadataReadDatabaseAccount`
/// to assert the AAD-token + account-metadata interaction surfaces upstream
/// HTTP errors correctly (the integration-level companion to the unit test
/// `fetch_account_properties_surfaces_5xx_body_as_status_error_issue_4483`).
#[tokio::test]
#[ignore = "Requires live Cosmos account and AAD credentials (DeveloperToolsCredential chain)"]
async fn aad_token_credential_account_metadata_smoke_test() {
    let Some(account) = build_account_with_token_credential_from_env() else {
        eprintln!(
            "Skipping AAD smoke test: AZURE_COSMOS_ENDPOINT unset or no usable credential chain"
        );
        return;
    };

    let runtime = CosmosDriverRuntime::builder()
        .build()
        .await
        .expect("runtime should be created");

    // get_or_create_driver triggers the lazy account-metadata (`GET /`) fetch
    // with the AAD bearer token. If the token-acquisition path is broken or
    // the data-plane RBAC role assignment hasn't propagated, this is where
    // we'd see a 401/403 surface — exactly the path that issue #4483 broke.
    let driver = runtime
        .get_or_create_driver(account, None)
        .await
        .expect("driver should be created with AAD credential");

    let db = read_env("AZURE_COSMOS_TEST_DATABASE").expect("missing test database env");
    let db_ref = azure_data_cosmos_driver::models::DatabaseReference::from_name(
        driver.account().clone(),
        db,
    );

    let _ = driver
        .execute_singleton_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await;
}
