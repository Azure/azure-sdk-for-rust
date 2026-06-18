// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for multi-region failover via HTTP-transport fault injection.
//!
//! These tests target a real, multi-region, multi-write Cosmos DB account. They are
//! gated by `test_category = "multi_write"` (set via RUSTFLAGS by the
//! `Session MultiWrite` live-test matrix entry — see
//! `sdk/cosmos/live-platform-matrix.json` and `sdk/cosmos/test-resources.bicep`) so
//! per-PR builds skip them entirely. Inside the live leg the framework reads
//! `AZURE_COSMOS_CONNECTION_STRING` exported by the bicep template; if it is unset
//! the tests skip cleanly via `DriverTestClient::run_with_*`.

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
use azure_data_cosmos_driver::models::{ContainerReference, DatabaseReference};
use azure_data_cosmos_driver::options::DriverOptions;
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::options::{ExcludedRegions, OperationOptionsBuilder, Region};
use azure_data_cosmos_driver::{CosmosStatus, SubStatusCode};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

// The framework module is shared across test binaries; not all exports are used
// from every binary so silence both dead-code and unused-import lints (other
// driver test binaries do the same thing).
#[allow(dead_code, unused_imports)]
mod framework;

use framework::resolve_test_env;

/// Persistent fault on every `MetadataReadDatabaseAccount` (GET /) request.
/// Fires unconditionally on `create_driver`, regardless of the data-plane op that follows.
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

/// Builds a fresh `CosmosDriverRuntime` (no FI applied — FI is per-driver).
async fn build_runtime() -> Arc<CosmosDriverRuntime> {
    CosmosDriverRuntime::builder()
        .build()
        .await
        .expect("runtime should be created")
}

/// Installs a persistent metadata fault, calls `create_driver`, and asserts the surfaced
/// error preserves `(expected_status, expected_sub_status)` and the rule fired at least once.
async fn run_metadata_fault_test(
    account: AccountReference,
    rule_id: &str,
    error_type: FaultInjectionErrorType,
    expected_status: StatusCode,
    expected_sub_status: Option<SubStatusCode>,
) {
    let rule = build_account_metadata_fault_rule(rule_id, error_type);
    let runtime = build_runtime().await;

    let err = runtime
        .create_driver(
            DriverOptions::builder(account)
                .with_fault_injection_rules(vec![Arc::clone(&rule)])
                .expect("rule installation should succeed")
                .build(),
        )
        .await
        .expect_err("create_driver must fail under a persistent metadata fault");

    assert_preserves_upstream_status(&err, expected_status, expected_sub_status);
    assert!(
        rule.hit_count() > 0,
        "MetadataReadDatabaseAccount fault should have been hit at least once"
    );
}

/// Resolves the master-key-based `AccountReference` from the framework env, or
/// prints a skip message and returns `None`. Mirrors the skip semantics of
/// `DriverTestClient::run_*`.
fn resolve_account_or_skip(test_name: &str) -> Option<AccountReference> {
    match resolve_test_env() {
        Ok(Some(env)) => Some(env.account),
        Ok(None) => {
            println!("Skipping {test_name}: Cosmos DB environment not configured");
            None
        }
        Err(e) => panic!("{test_name}: failed to resolve test env: {e}"),
    }
}

/// 403 WriteForbidden on GET / must surface as upstream HTTP status, not a serde failure.
/// Pins the per-error-type slice of the account-metadata parser invariant.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
async fn write_forbidden_on_metadata_preserves_upstream_status() {
    let Some(account) =
        resolve_account_or_skip("write_forbidden_on_metadata_preserves_upstream_status")
    else {
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
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
async fn session_not_available_on_metadata_preserves_upstream_status() {
    let Some(account) =
        resolve_account_or_skip("session_not_available_on_metadata_preserves_upstream_status")
    else {
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

// ── Data-plane failover helpers ─────────────────────────────────────────────

/// The first preferred write region declared in `sdk/cosmos/test-resources.bicep`.
/// Faults are scoped here so the test asserts the driver re-routes to the peer
/// region (`West US 3`) instead of just retrying in place.
const HUB_REGION: Region = Region::EAST_US_2;
const SATELLITE_REGION: Region = Region::WEST_US_3;

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

/// Creates a fresh DB + container via raw `CosmosOperation`s + JSON bodies (mirrors
/// `framework::DriverTestClient::{create_database, create_container_with_pk_paths}`).
/// Caller is responsible for cleanup via [`delete_database`].
async fn create_unique_db_and_container(
    runtime: &Arc<CosmosDriverRuntime>,
    account: &AccountReference,
) -> Result<(DatabaseReference, ContainerReference), Box<dyn Error>> {
    let driver = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await?;
    let db_name = format!("failover-test-db-{}", Uuid::new_v4());
    let container_name = "c".to_string();

    // Create database.
    let db_body = format!(r#"{{"id":"{db_name}"}}"#);
    let db_op = CosmosOperation::create_database(account.clone()).with_body(db_body.into_bytes());
    let db_result = driver
        .execute_singleton_operation(db_op, OperationOptions::default())
        .await?;
    let db_diag = db_result.diagnostics();
    let db_status = db_diag.status();
    if !db_status.map(|s| s.is_success()).unwrap_or(false) {
        return Err(format!("create database failed, status: {db_status:?}").into());
    }
    let db_ref = DatabaseReference::from_name(account.clone(), db_name.clone());

    // Create container.
    let container_body = format!(
        r#"{{"id":"{container_name}","partitionKey":{{"paths":["/pk"],"kind":"Hash","version":2}}}}"#
    );
    let container_op =
        CosmosOperation::create_container(db_ref.clone()).with_body(container_body.into_bytes());
    let container_result = driver
        .execute_singleton_operation(container_op, OperationOptions::default())
        .await?;
    let container_diag = container_result.diagnostics();
    let container_status = container_diag.status();
    if !container_status.map(|s| s.is_success()).unwrap_or(false) {
        return Err(format!("create container failed, status: {container_status:?}").into());
    }
    let container_ref = driver
        .resolve_container_by_name(&db_name, &container_name)
        .await?;
    Ok((db_ref, container_ref))
}

/// Best-effort DB cleanup.
async fn delete_database(
    runtime: &Arc<CosmosDriverRuntime>,
    account: &AccountReference,
    db: &DatabaseReference,
) {
    if let Ok(driver) = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await
    {
        let op = CosmosOperation::delete_database(db.clone());
        let _ = driver
            .execute_singleton_operation(op, OperationOptions::default())
            .await;
    }
}

/// Warms up a single region by issuing a `CreateItem` that excludes every other
/// region — forcing the operation through that specific regional endpoint until
/// the region's resource cache learns about the just-created container.
///
/// Background: a freshly-created container is published on the global gateway
/// (and the home region) immediately, but satellite regional endpoints can
/// return `401 Unauthorized` with body
/// `"The MAC signature found in the HTTP request is not the same as the computed signature"`
/// for the first few seconds — effectively a cache-miss surfaced as 401 rather
/// than 404. This loop polls until the region responds 2xx (or panics after
/// `WARMUP_TIMEOUT`), so subsequent fault-injection failover assertions are
/// deterministic. Mirrors Java's pattern of touching a pre-existing shared
/// container before fault rules are installed.
async fn warmup_region(
    runtime: &Arc<CosmosDriverRuntime>,
    account: &AccountReference,
    container: &ContainerReference,
    target_region: Region,
    all_regions: &[Region],
) -> Result<(), Box<dyn Error>> {
    const WARMUP_TIMEOUT: Duration = Duration::from_secs(30);
    const RETRY_DELAY: Duration = Duration::from_millis(500);

    let excluded: ExcludedRegions = all_regions
        .iter()
        .filter(|r| *r != &target_region)
        .cloned()
        .collect();
    let options = OperationOptionsBuilder::new()
        .with_excluded_regions(excluded)
        .build();

    let driver = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await?;
    let start = std::time::Instant::now();
    let mut last_err = None;
    while start.elapsed() < WARMUP_TIMEOUT {
        let item_id = format!("warmup-{}-{}", target_region, Uuid::new_v4());
        let body = format!(r#"{{"id":"{item_id}","pk":"warmup"}}"#).into_bytes();
        let item_ref =
            ItemReference::from_name(container, PartitionKey::from("warmup".to_string()), item_id);
        let op = CosmosOperation::create_item(item_ref).with_body(body);
        match driver
            .execute_singleton_operation(op, options.clone())
            .await
        {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(e);
                tokio::time::sleep(RETRY_DELAY).await;
            }
        }
    }
    Err(format!(
        "warmup of region {target_region:?} timed out after {WARMUP_TIMEOUT:?}; \
         last error: {last_err:?}"
    )
    .into())
}

/// Region-scoped 403/3 fault on CreateItem with a hit limit; asserts the rule fired and the op
/// eventually succeeded — proving the driver re-routed to a region where the fault was not in effect.
///
/// The rule is built **disabled**, warmup writes are issued to both write regions
/// (so neither has a cold resource-cache when failover lands on it), and only
/// then is the rule enabled and the real operation exercised.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
async fn write_forbidden_triggers_refresh_and_failover() -> Result<(), Box<dyn Error>> {
    run_data_plane_failover_test(
        "data-plane-write-forbidden-region-scoped",
        FaultOperationType::CreateItem,
        FaultInjectionErrorType::WriteForbidden,
        3,
        async |driver, container, fault_region| {
            let item_id = format!("failover-write-{}", Uuid::new_v4());
            let body = format!(r#"{{"id":"{item_id}","pk":"p"}}"#).into_bytes();
            let item_ref =
                ItemReference::from_name(&container, PartitionKey::from("p".to_string()), item_id);
            let op = CosmosOperation::create_item(item_ref).with_body(body);
            let response = driver
                .execute_singleton_operation(op, Default::default())
                .await
                .map_err(|e| -> Box<dyn Error> {
                    format!(
                        "the driver should have failed over to another write region after the \
                         region-scoped WriteForbidden fault exhausted its hit_limit in \
                         {fault_region:?}; instead the operation failed with: {e:?}"
                    )
                    .into()
                })?;
            assert_final_request_left_faulted_region(&response, &fault_region);
            Ok(())
        },
    )
    .await
}

/// Region-scoped 404/1002 fault on ReadItem with a hit limit, after seeding the item on a clean runtime.
/// Asserts the rule fired and the read succeeded — proving cross-region session retry advanced.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
async fn session_not_available_retries_across_locations() -> Result<(), Box<dyn Error>> {
    run_data_plane_failover_test(
        "data-plane-read-session-not-available-region-scoped",
        FaultOperationType::ReadItem,
        FaultInjectionErrorType::ReadSessionNotAvailable,
        5,
        async |driver, container, fault_region| {
            // Seed an item we can read back. Pre-warmup already wrote a couple,
            // but those use the warmup PK; seed one under our own PK.
            let item_id = format!("failover-read-{}", Uuid::new_v4());
            let body = format!(r#"{{"id":"{item_id}","pk":"p"}}"#).into_bytes();
            let item_ref = ItemReference::from_name(
                &container,
                PartitionKey::from("p".to_string()),
                item_id.clone(),
            );
            driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(item_ref.clone()).with_body(body),
                    Default::default(),
                )
                .await?;

            let response = driver
                .execute_singleton_operation(
                    CosmosOperation::read_item(item_ref),
                    Default::default(),
                )
                .await
                .map_err(|e| -> Box<dyn Error> {
                    format!(
                        "session retry should have advanced to the next preferred region and \
                         the read should have eventually succeeded; instead it failed with: \
                         {e:?} (faulted region: {fault_region:?})"
                    )
                    .into()
                })?;
            assert_final_request_left_faulted_region(&response, &fault_region);
            Ok(())
        },
    )
    .await
}

/// Asserts the final request landed on `expected_region`. Mirrors
/// `assert_final_request_left_faulted_region` but with positive intent.
fn assert_final_request_landed_in_region(response: &CosmosResponse, expected_region: &Region) {
    let diagnostics = response.diagnostics();
    let requests = diagnostics.requests();
    let final_request = requests
        .last()
        .expect("a successful response must have at least one request in its diagnostics");
    let final_region = final_request
        .region()
        .expect("the final request should have a region label");
    assert_eq!(
        final_region,
        expected_region,
        "the final request should have landed in {expected_region:?}; instead it landed on \
         {final_region:?} (endpoint: {endpoint}). Regions contacted across the operation: \
         {contacted:?}.",
        endpoint = final_request.endpoint(),
        contacted = diagnostics.regions_contacted()
    );
}

/// Pins `excluded_regions` end-to-end against a live multi-write account.
///
/// Live backend coverage catches silent global fallback that URL-level emulator checks cannot.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
async fn excluded_regions_honored_end_to_end() -> Result<(), Box<dyn Error>> {
    let Some(env) = resolve_test_env()? else {
        println!(
            "Skipping excluded_regions_honored_end_to_end: Cosmos DB environment not configured"
        );
        return Ok(());
    };
    let account = env.account;

    let runtime = CosmosDriverRuntime::builder().build().await?;
    let (db_ref, container_ref) = create_unique_db_and_container(&runtime, &account).await?;

    let all_regions = [HUB_REGION, SATELLITE_REGION];
    let mut setup_err = None;
    for r in &all_regions {
        if let Err(e) =
            warmup_region(&runtime, &account, &container_ref, r.clone(), &all_regions).await
        {
            setup_err = Some(format!("warmup of {r:?} failed: {e}"));
            break;
        }
    }
    if let Some(msg) = setup_err {
        delete_database(&runtime, &account, &db_ref).await;
        return Err(msg.into());
    }

    let driver = runtime.get_or_create_driver(account.clone(), None).await?;

    // Iterate to ensure excluded-region routing is sticky, not a one-attempt coincidence.
    let excluded: ExcludedRegions = std::iter::once(HUB_REGION).collect();
    let options = OperationOptionsBuilder::new()
        .with_excluded_regions(excluded)
        .build();

    let mut result: Result<(), Box<dyn Error>> = Ok(());
    for i in 0..3 {
        let item_id = format!("excluded-regions-{i}-{}", Uuid::new_v4());
        let body = format!(r#"{{"id":"{item_id}","pk":"p"}}"#).into_bytes();
        let item_ref =
            ItemReference::from_name(&container_ref, PartitionKey::from("p".to_string()), item_id);
        let op = CosmosOperation::create_item(item_ref).with_body(body);
        match driver
            .execute_singleton_operation(op, options.clone())
            .await
        {
            Ok(response) => assert_final_request_landed_in_region(&response, &SATELLITE_REGION),
            Err(e) => {
                result = Err(format!(
                    "create_item with excluded_regions=[{HUB_REGION:?}] should have succeeded \
                     against {SATELLITE_REGION:?}; instead failed with: {e:?}"
                )
                .into());
                break;
            }
        }
    }

    delete_database(&runtime, &account, &db_ref).await;
    result
}

/// Builds a region-scoped fault rule **disabled**, sets up a fresh DB+container
/// on a separate clean runtime, warms up both write regions, then builds a
/// runtime with the rule, enables it, and invokes the test closure.
async fn run_data_plane_failover_test<F, Fut>(
    rule_id: &str,
    op_type: FaultOperationType,
    err_type: FaultInjectionErrorType,
    hit_limit: u32,
    exercise: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(
        Arc<azure_data_cosmos_driver::driver::CosmosDriver>,
        ContainerReference,
        Region,
    ) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn Error>>>,
{
    let Some(env) = resolve_test_env()? else {
        println!("Skipping {rule_id}: Cosmos DB environment not configured");
        return Ok(());
    };
    let account = env.account;

    // Setup runtime: NO fault rule. Used for DB/container creation + warmup so
    // those operations are unaffected by the rule we will exercise.
    let setup_runtime = CosmosDriverRuntime::builder().build().await?;
    let (db_ref, container_ref) = create_unique_db_and_container(&setup_runtime, &account).await?;

    let all_regions = [HUB_REGION, SATELLITE_REGION];
    let mut warmup_err = None;
    for r in &all_regions {
        if let Err(e) = warmup_region(
            &setup_runtime,
            &account,
            &container_ref,
            r.clone(),
            &all_regions,
        )
        .await
        {
            warmup_err = Some(format!("warmup of {r:?} failed: {e}"));
            break;
        }
    }
    if let Some(msg) = warmup_err {
        delete_database(&setup_runtime, &account, &db_ref).await;
        return Err(msg.into());
    }

    // Exercise runtime: install the fault rule on the per-driver options.
    let rule = build_region_scoped_data_plane_fault_rule(
        rule_id, op_type, HUB_REGION, err_type, hit_limit,
    );
    let exercise_runtime = CosmosDriverRuntime::builder().build().await?;
    let exercise_driver = exercise_runtime
        .create_driver(
            DriverOptions::builder(account.clone())
                .with_fault_injection_rules(vec![Arc::clone(&rule)])?
                .build(),
        )
        .await?;

    let result = exercise(exercise_driver, container_ref.clone(), HUB_REGION).await;

    assert!(
        rule.hit_count() >= 1,
        "the region-scoped fault should have fired at least once in {HUB_REGION:?} \
         before the driver failed over; hit_count was 0. Test result: {result:?}"
    );

    delete_database(&setup_runtime, &account, &db_ref).await;
    result
}
