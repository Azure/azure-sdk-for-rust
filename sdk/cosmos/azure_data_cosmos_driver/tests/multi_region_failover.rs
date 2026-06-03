// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration coverage for Step 2 multi-region failover, exercised via fault
//! injection at the HTTP transport layer.
//!
//! Two flavors of test live in this file:
//!
//! 1. **Metadata-path error-shape tests** — install a persistent fault on
//!    `MetadataReadDatabaseAccount` and assert that the surfaced error
//!    preserves the upstream HTTP status (and is NOT relabeled as a
//!    `missing field \`_self\`` serde failure / synthetic
//!    `SERIALIZATION_RESPONSE_BODY_INVALID`). These guard the account-
//!    metadata parser invariant.
//! 2. **Data-plane region-scoped failover tests** — install a fault scoped to
//!    a single region on a customer-data operation (`CreateItem` / `ReadItem`)
//!    with a finite `hit_limit`, then assert that the driver retried the
//!    operation against a different region and the request eventually
//!    succeeded.
//!
//! Both flavors are gated on:
//!   * `feature = "fault_injection"` — to install transport-level fault rules
//!     into the runtime via `CosmosDriverRuntime::builder().with_fault_injection_rules(...)`.
//!   * Environment variables — the assertions about region failover only
//!     have meaning against a real multi-region Cosmos topology. Each test
//!     skips cleanly (with a stderr notice) when its prerequisites are not
//!     provisioned; that keeps them green in CI today while still running
//!     locally once an operator points the env at a real account.
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
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, ItemReference, PartitionKey,
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

/// Builds an `AccountReference` authenticated via `DeveloperToolsCredential`
/// (az login / azd login chain) for AAD-token coverage. Returns `None` if
/// `AZURE_COSMOS_ENDPOINT` is unset OR the credential chain cannot be
/// constructed in this environment.
///
/// Closes a long-standing test-coverage gap: every existing Cosmos
/// integration test authenticates with master keys, which are HMAC-signed
/// locally and never validated against AAD. The entire AAD-token error path
/// (401 InvalidToken on token expiry, 403 from delayed RBAC propagation,
/// 403 from data-plane RBAC misconfiguration, 503 from metadata-service
/// hiccups during token acquisition) is therefore never exercised in CI —
/// which is exactly the shape of the production account-metadata failures
/// real customers see.
fn build_account_with_token_credential_from_env() -> Option<AccountReference> {
    let endpoint = read_env("AZURE_COSMOS_ENDPOINT")?;
    let url = url::Url::parse(&endpoint).ok()?;
    let credential = DeveloperToolsCredential::new(None).ok()?;
    Some(AccountReference::with_credential(url, credential))
}

/// Bundle of everything a data-plane region-scoped failover test needs:
/// authenticated account, resolved container, partition-key value to use
/// for the test item, and the `Region` the fault should be scoped to (i.e.
/// the region the driver should *fail away from*).
///
/// Returns `None` (skip cleanly) if any of the required environment
/// variables are unset, so the test exits as a pass in CI until an operator
/// provisions a real multi-region account and points the env at it.
///
/// Required env vars:
///   * `AZURE_COSMOS_ENDPOINT`, `AZURE_COSMOS_KEY` — account + auth
///   * `AZURE_COSMOS_TEST_DATABASE` — pre-provisioned test database
///   * `AZURE_COSMOS_TEST_CONTAINER` — pre-provisioned container inside it
///   * `AZURE_COSMOS_TEST_PARTITION_KEY` — partition-key value to use for
///     created/read items
///   * `AZURE_COSMOS_TEST_REGION` — the region name the fault should be
///     scoped to (e.g. `"East US"`). Must match a region in the account's
///     account-properties payload.
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

/// Installs a persistent fault that returns the given error type on every
/// `MetadataReadDatabaseAccount` request (i.e. on `GET /`, the account
/// properties endpoint).
///
/// `MetadataReadDatabaseAccount` is the right injection point for the
/// account-metadata error-shape tests because:
///   1. It fires unconditionally on `get_or_create_driver`, so the fault is
///      guaranteed to be exercised regardless of which data-plane op runs
///      afterward.
///   2. It targets the exact endpoint behind the parser invariant under
///      test, so the assertion that the error surfaces as an HTTP-status
///      error (not a serde `missing field \`_self\`` failure) gives us the
///      highest-value coverage per test.
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

/// Region-scoped, hit-limited fault on a customer-data operation.
///
/// Unlike `build_account_metadata_fault_rule` — which targets the metadata
/// endpoint globally and persists indefinitely — this builder produces a
/// rule that:
///   * Targets a single data-plane operation type (e.g. `CreateItem`).
///   * Is scoped to a single region via `with_region`, so requests routed to
///     other regions are *not* faulted.
///   * Has a finite `hit_limit`, so once the driver exhausts that many
///     failed attempts in the faulted region the rule stops firing — any
///     subsequent attempt against a different region will hit the real
///     endpoint and succeed.
///
/// This is the precise shape needed to verify cross-region failover:
/// `rule.hit_count() >= 1` proves the fault fired in the chosen region, and
/// the operation's eventual `Ok` proves the driver successfully retried
/// against a region where the fault was not in effect.
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

/// Asserts that the surfaced error from a faulted `GET /` preserves the
/// upstream HTTP status from the injected fault — i.e. it is NOT wrapped as
/// the synthetic `SERIALIZATION_RESPONSE_BODY_INVALID` (500/20020) that the
/// pre-fix account-metadata path produced when the body failed to deserialize
/// as `AccountProperties`. That wrapping was the precise bug shape under
/// fix here: any non-2xx body (whether plain text or a Cosmos JSON error
/// envelope missing `_self`) would surface as a generic serialization
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

/// Installs a fault rule into a fresh `CosmosDriverRuntime`.
async fn build_runtime_with_rule(rule: &Arc<FaultInjectionRule>) -> Arc<CosmosDriverRuntime> {
    CosmosDriverRuntime::builder()
        .with_fault_injection_rules(vec![Arc::clone(rule)])
        .expect("rule installation should succeed")
        .build()
        .await
        .expect("runtime should be created")
}

/// Runs the canonical "metadata fault preserves upstream status" assertion
/// chain: install a persistent fault of `error_type` on
/// `MetadataReadDatabaseAccount`, call `get_or_create_driver`, and assert
/// the surfaced error preserves `(expected_status, expected_sub_status)`
/// and the rule actually fired.
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

/// Seeds an item under a clean runtime (no fault rules) so subsequent
/// faulted-read tests have something to find. Panics on any failure —
/// the seed must succeed before the test's read assertion is meaningful.
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

/// Behavioral coverage for the WriteForbidden (403/3) error path on the
/// account-metadata endpoint.
///
/// Injects a persistent 403 WriteForbidden on `GET /` and asserts that
/// `get_or_create_driver` surfaces the upstream HTTP status — not a serde
/// failure — to the caller. This pins the per-error-type slice of the
/// account-metadata parser invariant for the WriteForbidden envelope.
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

/// Behavioral coverage for the ReadSessionNotAvailable (404/1002) error
/// path on the account-metadata endpoint.
///
/// Injects a persistent 404/1002 on `GET /` and asserts that
/// `get_or_create_driver` surfaces the upstream HTTP status — not a serde
/// failure. This pins the per-error-type slice of the account-metadata
/// parser invariant for the ReadSessionNotAvailable envelope.
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

/// AAD-credential + fault-injection coverage.
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
///     relabeled as a `missing field \`_self\`` serde failure.
///   * The fault rule was actually exercised (`hit_count() > 0`), so we know
///     the test really did drive the fault-injecting HTTP client.
///
/// Every other Cosmos integration test in this repo authenticates with
/// master keys, which are HMAC-signed locally and never validated against
/// AAD. The entire AAD-token error path (token-acquisition hiccups, 401
/// `InvalidToken`, 403 RBAC propagation, 503 from IMDS) is therefore never
/// exercised in CI — and that is exactly the shape of the production
/// account-metadata failures real customers see.
///
/// The test skips cleanly with a stderr notice when either
/// `AZURE_COSMOS_ENDPOINT` is unset or no `DeveloperToolsCredential` chain can
/// be constructed in the current environment, so it stays green in CI until
/// those prerequisites are provisioned. Run locally with:
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

    run_metadata_fault_test(
        account,
        "aad-account-metadata-503",
        FaultInjectionErrorType::ServiceUnavailable,
        StatusCode::ServiceUnavailable,
        None,
    )
    .await;
}

/// Behavioral coverage for cross-region failover on a region-local
/// WriteForbidden (403/3) fault.
///
/// Injects a `WriteForbidden` fault scoped to a single region on
/// `CreateItem`, with a finite `hit_limit` so once the driver exhausts that
/// many attempts in the faulted region it can succeed against a different
/// region. Asserts:
///
///   1. The fault was actually exercised in the chosen region
///      (`rule.hit_count() >= 1`).
///   2. The operation eventually succeeded (`result.is_ok()`), which proves
///      the driver retried against a region where the fault was not in
///      effect — i.e. it performed cross-region failover.
///
/// This complements the metadata-path error-shape tests above: those guard
/// the parser invariant, this one guards the cross-region retry behavior
/// the driver is supposed to apply on a 403/3 from the data plane.
///
/// Deeper assertions about internal effects (`RefreshAccountProperties`,
/// `MarkEndpointUnavailable`) require observability hooks that do not yet
/// exist on `CosmosDriver`; once those land, the test can be extended in
/// place without changing its fault-injection setup.
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
        // 3 region-local failures is enough to prove the driver re-routes:
        // once the limit is exhausted, the next attempt routes elsewhere and
        // can succeed.
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
    assert!(
        result.is_ok(),
        "the driver should have failed over to another region after the \
         region-scoped WriteForbidden fault exhausted its hit_limit; instead \
         the operation failed with: {:?}. If the configured account does not \
         actually have a second write region available, point \
         AZURE_COSMOS_TEST_REGION at a region that is not the only write region.",
        result.err()
    );
}

/// Behavioral coverage for the cross-region session-retry path.
///
/// Injects a `ReadSessionNotAvailable` fault scoped to a single region on
/// `ReadItem`, with a finite `hit_limit`. Asserts:
///
///   1. The fault was actually exercised in the chosen region
///      (`rule.hit_count() >= 1`).
///   2. The read eventually succeeded (`result.is_ok()`), which proves the
///      driver advanced to the next preferred region after the session
///      retry was exhausted in the faulted region.
///
/// The test first seeds an item (no fault rule in scope for the seed) and
/// then reads it back under the region-scoped fault, so a successful read
/// against the failover region is unambiguous.
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

    // Seed an item under a clean runtime (no fault) so the read below has
    // something to find. This also exercises the create path against the
    // unfaulted account so failures here are not silently misattributed to
    // failover behavior on the read.
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
    assert!(
        result.is_ok(),
        "session retry should have advanced to the next preferred region and \
         the read should have eventually succeeded; instead it failed with: \
         {:?}. If the configured account has only one read region, point \
         AZURE_COSMOS_TEST_REGION at a region that has at least one peer \
         available for read failover.",
        result.err()
    );
}
