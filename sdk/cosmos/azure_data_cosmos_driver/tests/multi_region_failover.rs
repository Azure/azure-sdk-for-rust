// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for multi-region failover via HTTP-transport fault injection.
//! Gated on `feature = "fault_injection"` + env vars; tests skip cleanly when prereqs are absent.

#![cfg(feature = "fault_injection")]

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::error::CosmosError;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, CosmosResponse, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{OperationOptions, Region};
use azure_data_cosmos_driver::{CosmosStatus, SubStatusCode};
use azure_identity::DeveloperToolsCredential;
use std::sync::Arc;
use uuid::Uuid;

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

fn build_account_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let key = read_env("AZURE_COSMOS_KEY")?;
    let url = url::Url::parse(&endpoint).ok()?;
    Some(AccountReference::with_master_key(url, key))
}

/// `AccountReference` using `DeveloperToolsCredential` (az login chain) for AAD-token coverage.
/// Returns `None` if `AZURE_COSMOS_ENDPOINT` is unset or no credential chain is available.
fn build_account_with_token_credential_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let url = url::Url::parse(&endpoint).ok()?;
    let credential = DeveloperToolsCredential::new(None).ok()?;
    Some(AccountReference::with_credential(url, credential))
}

/// Account, container, partition-key value, and the region the fault should be scoped to.
/// Env: ENDPOINT, KEY, TEST_DATABASE, TEST_CONTAINER, TEST_PARTITION_KEY, TEST_REGION.
struct DataPlaneEnv {
    account: AccountReference,
    db_name: String,
    container_name: String,
    partition_key_value: String,
    fault_region: Region,
}

fn build_data_plane_env() -> Option<DataPlaneEnv> {
    let account = build_account_from_env()?;
    let db_name = read_env("AZURE_COSMOS_TEST_DATABASE")?;
    let container_name = read_env("AZURE_COSMOS_TEST_CONTAINER")?;
    let partition_key_value = read_env("AZURE_COSMOS_TEST_PARTITION_KEY")?;
    let region_name = read_env("AZURE_COSMOS_TEST_REGION")?;
    Some(DataPlaneEnv {
        account,
        db_name,
        container_name,
        partition_key_value,
        fault_region: Region::from(region_name),
    })
}

/// Persistent fault on every `MetadataReadDatabaseAccount` (GET /) request.
/// Fires unconditionally on `get_or_create_driver`, regardless of the data-plane op that follows.
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

/// Region-scoped, hit-limited fault on a data-plane op (e.g. `CreateItem`).
/// After `hit_limit` failures in the target region, attempts in other regions hit the real endpoint.
fn build_region_scoped_data_plane_fault_rule(
    id: &str,
    operation_type: FaultOperationType,
    region: Region,
    error_type: FaultInjectionErrorType,
    hit_limit: u32,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation_type)
        .with_region(region)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(error_type)
        .with_probability(1.0)
        .build();

    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .with_hit_limit(hit_limit)
            .build(),
    )
}

/// Asserts the error preserves the upstream HTTP status and is NOT relabeled as
/// `SERIALIZATION_RESPONSE_BODY_INVALID` — the pre-fix bug shape for non-2xx bodies.
fn assert_preserves_upstream_status(
    err: &CosmosError,
    expected_status: StatusCode,
    expected_sub_status: Option<SubStatusCode>,
) {
    let status = err.status();
    assert_ne!(
        status,
        CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
        "error must preserve the upstream HTTP status, not be \
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

/// Asserts the final request in `response`'s diagnostics was sent to a region
/// other than `faulted_region` — i.e. failover (or session-retry) actually
/// re-targeted the operation to a peer endpoint, not just retried in place.
fn assert_final_request_left_faulted_region(response: &CosmosResponse, faulted_region: &Region) {
    let diagnostics = response.diagnostics();
    let requests = diagnostics.requests();
    let final_request = requests
        .last()
        .expect("a successful response must have at least one request in its diagnostics");
    let final_region = final_request
        .region()
        .expect("the final request should have a region label");
    assert_ne!(
        final_region,
        faulted_region,
        "the successful final request should have been routed to a region other \
         than the faulted region {faulted_region:?}; instead it landed on \
         {final_region:?} (endpoint: {endpoint}). Regions contacted across the \
         operation: {contacted:?}.",
        endpoint = final_request.endpoint(),
        contacted = diagnostics.regions_contacted()
    );
}

/// Installs a fault rule into a fresh `CosmosDriverRuntime`.
async fn build_runtime_with_rule(rule: &Arc<FaultInjectionRule>) -> Arc<CosmosDriverRuntime> {
    CosmosDriverRuntime::builder()
        .with_fault_injection_rules(vec![Arc::clone(rule)])
        .expect("rule installation should succeed")
        .build()
        .await
        .expect("runtime should be created")
}

/// Installs a persistent metadata fault, calls `get_or_create_driver`, and asserts the surfaced
/// error preserves `(expected_status, expected_sub_status)` and the rule fired at least once.
async fn run_metadata_fault_test(
    account: AccountReference,
    rule_id: &str,
    error_type: FaultInjectionErrorType,
    expected_status: StatusCode,
    expected_sub_status: Option<SubStatusCode>,
) {
    let rule = build_account_metadata_fault_rule(rule_id, error_type);
    let runtime = build_runtime_with_rule(&rule).await;

    let err = runtime
        .get_or_create_driver(account, None)
        .await
        .expect_err("get_or_create_driver must fail under a persistent metadata fault");

    assert_preserves_upstream_status(&err, expected_status, expected_sub_status);
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount fault should have been hit at least once"
    );
}

/// Canonical item body for the data-plane failover tests.
fn make_item_body(id: &str, pk_value: &str) -> Vec<u8> {
    format!(r#"{{"id":"{}","pk":"{}"}}"#, id, pk_value).into_bytes()
}

/// Seeds an item under a clean (unfaulted) runtime so a later faulted-read test has something to find.
/// Panics on any failure — the seed must succeed before the read assertion is meaningful.
async fn seed_item(env: &DataPlaneEnv, id: &str) {
    let runtime = CosmosDriverRuntime::builder()
        .build()
        .await
        .expect("seed runtime should be created");
    let driver = runtime
        .get_or_create_driver(env.account.clone(), None)
        .await
        .expect("seed driver should be created");
    let container = driver
        .resolve_container(&env.db_name, &env.container_name)
        .await
        .expect("seed container must resolve");
    let pk = PartitionKey::from(env.partition_key_value.clone());
    let item_ref = ItemReference::from_name(&container, pk, id.to_owned());
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item_ref)
                .with_body(make_item_body(id, &env.partition_key_value)),
            OperationOptions::default(),
        )
        .await
        .expect("seed create_item must succeed before exercising the read fault");
}

/// 403 WriteForbidden on GET / must surface as upstream HTTP status, not a serde failure.
/// Pins the per-error-type slice of the account-metadata parser invariant.
#[tokio::test]
async fn write_forbidden_on_metadata_preserves_upstream_status() {
    let Some(account) = build_account_from_env() else {
        eprintln!(
            "Skipping WriteForbidden metadata fault test: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, or AZURE_COSMOS_TEST_DATABASE unset"
        );
        return;
    };

    run_metadata_fault_test(
        account,
        "multi-region-write-forbidden",
        FaultInjectionErrorType::WriteForbidden,
        StatusCode::Forbidden,
        Some(SubStatusCode::WRITE_FORBIDDEN),
    )
    .await;
}

/// 404/1002 ReadSessionNotAvailable on GET / must surface as upstream HTTP status, not a serde failure.
/// Pins the per-error-type slice of the account-metadata parser invariant.
#[tokio::test]
async fn session_not_available_on_metadata_preserves_upstream_status() {
    let Some(account) = build_account_from_env() else {
        eprintln!(
            "Skipping ReadSessionNotAvailable metadata fault test: AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, or AZURE_COSMOS_TEST_DATABASE unset"
        );
        return;
    };

    run_metadata_fault_test(
        account,
        "multi-region-read-session-not-available",
        FaultInjectionErrorType::ReadSessionNotAvailable,
        StatusCode::NotFound,
        Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
    )
    .await;
}

/// AAD smoke: drives `get_or_create_driver` under `DeveloperToolsCredential` with a persistent 503 fault.
/// Closes the AAD-coverage gap — every other Cosmos integration test in this repo uses HMAC master keys only.
#[tokio::test]
async fn aad_token_credential_account_metadata_smoke_test() {
    let Some(account) = build_account_with_token_credential_from_env() else {
        eprintln!(
            "Skipping AAD smoke test: AZURE_COSMOS_ENDPOINT unset or no usable credential chain"
        );
        return;
    };

    run_metadata_fault_test(
        account,
        "aad-account-metadata-503",
        FaultInjectionErrorType::ServiceUnavailable,
        StatusCode::ServiceUnavailable,
        None,
    )
    .await;
}

/// Region-scoped 403/3 fault on CreateItem with a hit limit; asserts the rule fired and the op
/// eventually succeeded — proving the driver re-routed to a region where the fault was not in effect.
#[tokio::test]
async fn write_forbidden_triggers_refresh_and_failover() {
    let Some(env) = build_data_plane_env() else {
        eprintln!(
            "Skipping write_forbidden_triggers_refresh_and_failover: requires \
             AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, AZURE_COSMOS_TEST_DATABASE, \
             AZURE_COSMOS_TEST_CONTAINER, AZURE_COSMOS_TEST_PARTITION_KEY, and \
             AZURE_COSMOS_TEST_REGION (the multi-write region the fault should \
             be scoped to)"
        );
        return;
    };

    let rule = build_region_scoped_data_plane_fault_rule(
        "data-plane-write-forbidden-region-scoped",
        FaultOperationType::CreateItem,
        env.fault_region.clone(),
        FaultInjectionErrorType::WriteForbidden,
        // 3 failures in the faulted region; the next attempt routes elsewhere.
        3,
    );

    let runtime = build_runtime_with_rule(&rule).await;

    let driver = runtime
        .get_or_create_driver(env.account, None)
        .await
        .expect("get_or_create_driver should succeed against a real account");

    let container = driver
        .resolve_container(&env.db_name, &env.container_name)
        .await
        .expect("container must resolve before exercising the data-plane fault");

    // Use a UUID id so reruns of the test don't collide on each other.
    let item_id = format!("failover-write-{}", Uuid::new_v4());
    let pk = PartitionKey::from(env.partition_key_value.clone());
    let item_ref = ItemReference::from_name(&container, pk, item_id.clone());

    let result = driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item_ref)
                .with_body(make_item_body(&item_id, &env.partition_key_value)),
            OperationOptions::default(),
        )
        .await;

    assert!(
        rule.hit_count() >= 1,
        "the region-scoped WriteForbidden fault should have fired at least once \
         in region {:?} before the driver re-routed; hit_count was 0",
        env.fault_region
    );
    let response = result.unwrap_or_else(|err| {
        panic!(
            "the driver should have failed over to another region after the \
             region-scoped WriteForbidden fault exhausted its hit_limit; instead \
             the operation failed with: {err:?}. If the configured account does \
             not actually have a second write region available, point \
             AZURE_COSMOS_TEST_REGION at a region that is not the only write region."
        )
    });
    assert_final_request_left_faulted_region(&response, &env.fault_region);
}

/// Region-scoped 404/1002 fault on ReadItem with a hit limit, after seeding the item on a clean runtime.
/// Asserts the rule fired and the read succeeded — proving cross-region session retry advanced.
#[tokio::test]
async fn session_not_available_retries_across_locations() {
    let Some(env) = build_data_plane_env() else {
        eprintln!(
            "Skipping session_not_available_retries_across_locations: requires \
             AZURE_COSMOS_ENDPOINT, AZURE_COSMOS_KEY, AZURE_COSMOS_TEST_DATABASE, \
             AZURE_COSMOS_TEST_CONTAINER, AZURE_COSMOS_TEST_PARTITION_KEY, and \
             AZURE_COSMOS_TEST_REGION"
        );
        return;
    };

    // Seed an item under a clean runtime so the faulted read has something to find.
    let seed_id = format!("failover-read-{}", Uuid::new_v4());
    seed_item(&env, &seed_id).await;

    let rule = build_region_scoped_data_plane_fault_rule(
        "data-plane-read-session-not-available-region-scoped",
        FaultOperationType::ReadItem,
        env.fault_region.clone(),
        FaultInjectionErrorType::ReadSessionNotAvailable,
        // Driver session retry has its own internal cap; 5 region-local
        // failures is enough to exhaust it without exhausting global retries.
        5,
    );

    let runtime = build_runtime_with_rule(&rule).await;

    let driver = runtime
        .get_or_create_driver(env.account, None)
        .await
        .expect("get_or_create_driver should succeed against a real account");

    let container = driver
        .resolve_container(&env.db_name, &env.container_name)
        .await
        .expect("container must resolve before exercising the data-plane fault");

    let pk = PartitionKey::from(env.partition_key_value.clone());
    let item_ref = ItemReference::from_name(&container, pk, seed_id.clone());

    let result = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item_ref),
            OperationOptions::default(),
        )
        .await;

    assert!(
        rule.hit_count() >= 1,
        "the region-scoped ReadSessionNotAvailable fault should have fired at \
         least once in region {:?} before the driver advanced to the next \
         preferred region; hit_count was 0",
        env.fault_region
    );
    let response = result.unwrap_or_else(|err| {
        panic!(
            "session retry should have advanced to the next preferred region and \
             the read should have eventually succeeded; instead it failed with: \
             {err:?}. If the configured account has only one read region, point \
             AZURE_COSMOS_TEST_REGION at a region that has at least one peer \
             available for read failover."
        )
    });
    assert_final_request_left_faulted_region(&response, &env.fault_region);
}
