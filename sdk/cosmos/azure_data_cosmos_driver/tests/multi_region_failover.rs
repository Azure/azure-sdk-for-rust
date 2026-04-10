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

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn build_account_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let key = read_env("AZURE_COSMOS_KEY")?;
    let url = url::Url::parse(&endpoint).ok()?;
    Some(AccountReference::with_master_key(url, key))
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
        .execute_operation(
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
        .execute_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await;
}
