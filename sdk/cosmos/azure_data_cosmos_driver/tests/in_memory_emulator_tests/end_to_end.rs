// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests that exercise the full SDK pipeline (CosmosClient →
//! ContainerClient → driver → in-memory emulator) **and** (optionally) a real
//! Cosmos DB account.
//!
//! Each test:
//! 1. Runs operations against the in-memory emulator via the SDK client.
//! 2. When `AZURE_COSMOS_CONNECTION_STRING` is set, repeats the same operations
//!    against a real account using a second SDK client.
//! 3. Compares status codes, headers, and payloads between the two
//!    backends using the shared [`super::validation`] comparison framework.
//!
//! Only `create_item` and `read_item` are tested here because these are the SDK
//! operations currently routed through the driver pipeline. Operations like
//! `replace_item`, `delete_item`, `create_database`, and `create_container` still
//! go through the gateway HTTP pipeline and cannot be intercepted by the
//! in-memory emulator transport. Add tests for those once they are migrated to
//! the driver.

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::regions::Region;
use azure_data_cosmos::CosmosAccountReference;
use azure_data_cosmos::{
    ContentResponseOnWrite, CosmosClient, CosmosClientBuilder, ItemReadOptions, ItemResponse,
    ItemWriteOptions, OperationOptions, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{ConnectionString, CosmosResponseHeaders};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use super::validation::{
    compare_responses, BodyValidationSpec, HeaderValidationSpec, ResponseSnapshot,
};

// ─── Test model ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

// ─── SDK → ResponseSnapshot adapter ─────────────────────────────────────────

/// Builds a [`ResponseSnapshot`] from an SDK [`ItemResponse`] so the shared
/// validation framework in [`super::validation`] can be reused.
fn snapshot_from_item_response<T>(resp: &ItemResponse<T>, label: &str) -> ResponseSnapshot {
    let headers = CosmosResponseHeaders::from_headers(resp.headers());
    ResponseSnapshot {
        status_code: u16::from(resp.status()),
        sub_status_code: headers.substatus.as_ref().map(|s| s.value()),
        headers,
        body: None, // body comparison handled separately via deserialization
        label: label.to_owned(),
    }
}

/// Compares an emulator and real [`ItemResponse`] using the shared header
/// validation spec for point operations.
fn compare_item_responses<T>(real: &ItemResponse<T>, emu: &ItemResponse<T>) {
    let real_snap = snapshot_from_item_response(real, "real");
    let emu_snap = snapshot_from_item_response(emu, "emulator");
    compare_responses(
        &real_snap,
        &emu_snap,
        &HeaderValidationSpec::for_point_operation(),
        BodyValidationSpec::Ignore, // body validated via typed deserialization
    );
}

/// Compares two SDK error responses: both must have the same HTTP status.
fn compare_sdk_errors(real: &azure_core::Error, emu: &azure_core::Error) {
    assert_eq!(
        real.http_status(),
        emu.http_status(),
        "Error status mismatch: real={:?} emulator={:?}",
        real.http_status(),
        emu.http_status(),
    );
}

/// Asserts emulator-only response metadata when no real account is available.
fn assert_emulator_item_response<T>(resp: &ItemResponse<T>, expected_status: StatusCode) {
    assert_eq!(resp.status(), expected_status);
    let snap = snapshot_from_item_response(resp, "emulator");
    assert!(
        snap.headers.request_charge.is_some(),
        "request_charge should be present",
    );
    assert!(
        snap.headers.session_token.is_some(),
        "session_token should be present",
    );
    assert!(
        snap.headers.server_duration_ms.is_some(),
        "server_duration_ms should be present",
    );
}

// ─── Dual Backend ────────────────────────────────────────────────────────────

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";

struct SdkDualBackend {
    emulator_client: CosmosClient,
    emulator_store: std::sync::Arc<azure_data_cosmos_driver::in_memory_emulator::EmulatorStore>,
    real_client: Option<CosmosClient>,
    run_id: String,
}

impl SdkDualBackend {
    async fn setup() -> Result<Self, Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();

        let run_id = Uuid::new_v4().to_string()[..8].to_string();

        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
        )])
        .with_consistency(ConsistencyLevel::Session);

        let emulator = InMemoryEmulatorHttpClient::new(config);
        let emulator_store = emulator.store();

        let emulator_account = CosmosAccountReference::with_master_key(
            EMULATOR_GATEWAY_URL.parse().unwrap(),
            azure_core::credentials::Secret::new("dGVzdGtleQ=="),
        );

        let emulator_client = CosmosClientBuilder::new()
            .with_driver_runtime_builder(emulator.runtime_builder())
            .build(
                emulator_account,
                RoutingStrategy::ProximityTo(Region::EAST_US),
            )
            .await?;

        let real_client = resolve_real_client().await?;

        Ok(Self {
            emulator_client,
            emulator_store,
            real_client,
            run_id,
        })
    }

    fn has_real(&self) -> bool {
        self.real_client.is_some()
    }

    fn unique_db_name(&self) -> String {
        format!("sdk-e2e-{}", self.run_id)
    }

    fn provision_emulator(&self, db: &str, container: &str, pk_path: &str) {
        self.emulator_store.create_database(db);
        self.emulator_store.create_container(
            db,
            container,
            serde_json::from_value(serde_json::json!({
                "paths": [pk_path],
                "kind": "Hash",
                "version": 2
            }))
            .unwrap(),
        );
    }

    async fn create_real_database(&self, db_name: &str) -> Result<(), Box<dyn Error>> {
        if let Some(ref client) = self.real_client {
            client.create_database(db_name, None).await?;
        }
        Ok(())
    }

    async fn create_real_container(
        &self,
        db_name: &str,
        container_name: &str,
        pk_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(ref client) = self.real_client {
            let db_client = client.database_client(db_name);
            let props = ContainerProperties::new(container_name.to_string(), pk_path.into());
            db_client.create_container(props, None).await?;
        }
        Ok(())
    }

    async fn cleanup_real_database(&self, db_name: &str) {
        if let Some(ref client) = self.real_client {
            let db_client = client.database_client(db_name);
            let _ = db_client.delete(None).await;
        }
    }

    async fn container_clients(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> Result<(ContainerClient, Option<ContainerClient>), Box<dyn Error>> {
        let emu = self
            .emulator_client
            .database_client(db_name)
            .container_client(container_name)
            .await?;

        let real = if let Some(ref client) = self.real_client {
            Some(
                client
                    .database_client(db_name)
                    .container_client(container_name)
                    .await?,
            )
        } else {
            None
        };

        Ok((emu, real))
    }
}

async fn setup_with_container() -> (
    SdkDualBackend,
    String,
    ContainerClient,
    Option<ContainerClient>,
) {
    let backend = SdkDualBackend::setup().await.unwrap();
    let db_name = backend.unique_db_name();
    let container_name = "testcoll";
    let pk_path = "/pk";

    backend.provision_emulator(&db_name, container_name, pk_path);

    if backend.has_real() {
        backend.create_real_database(&db_name).await.unwrap();
        backend
            .create_real_container(&db_name, container_name, pk_path)
            .await
            .unwrap();
    }

    let (emu_container, real_container) = backend
        .container_clients(&db_name, container_name)
        .await
        .unwrap();

    (backend, db_name, emu_container, real_container)
}

fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sdk_create_and_read_item() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let item = TestItem {
        id: "sdk-item-1".into(),
        pk: "pk1".into(),
        value: 42,
    };

    // ── Create item ──────────────────────────────────────────────
    let emu_create = emu_container
        .create_item(
            "pk1",
            "sdk-item-1",
            &item,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    assert_emulator_item_response(&emu_create, StatusCode::Created);

    if let Some(ref real) = real_container {
        let real_create = real
            .create_item(
                "pk1",
                "sdk-item-1",
                &item,
                Some(write_options_with_content()),
            )
            .await
            .unwrap();
        compare_item_responses(&real_create, &emu_create);
    }

    // ── Read item back ───────────────────────────────────────────
    let emu_read = emu_container
        .read_item::<TestItem>("pk1", "sdk-item-1", None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);
    assert!(emu_read.etag().is_some(), "emulator read should have etag");

    if let Some(ref real) = real_container {
        let real_read = real
            .read_item::<TestItem>("pk1", "sdk-item-1", None)
            .await
            .unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().json().unwrap();
        assert_eq!(real_doc.id, "sdk-item-1");
        assert_eq!(real_doc.value, 42);
    }

    let emu_doc: TestItem = emu_read.into_body().json().unwrap();
    assert_eq!(emu_doc.id, "sdk-item-1");
    assert_eq!(emu_doc.value, 42);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_create_multiple_items_and_read_back() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    for i in 0..3 {
        let item = TestItem {
            id: format!("multi-{i}"),
            pk: "pk1".into(),
            value: i,
        };
        let emu_resp = emu_container
            .create_item("pk1", &item.id, &item, None)
            .await
            .unwrap();
        assert_emulator_item_response(&emu_resp, StatusCode::Created);

        if let Some(ref real) = real_container {
            let real_resp = real
                .create_item("pk1", &item.id, &item, None)
                .await
                .unwrap();
            compare_item_responses(&real_resp, &emu_resp);
        }
    }

    for i in 0..3 {
        let id = format!("multi-{i}");
        let emu_read = emu_container
            .read_item::<TestItem>("pk1", &id, None)
            .await
            .unwrap();
        assert_emulator_item_response(&emu_read, StatusCode::Ok);

        let emu_doc: TestItem = emu_read.into_body().json().unwrap();
        assert_eq!(emu_doc.value, i);

        if let Some(ref real) = real_container {
            let real_read = real.read_item::<TestItem>("pk1", &id, None).await.unwrap();
            let real_doc: TestItem = real_read.into_body().json().unwrap();
            assert_eq!(real_doc.value, i);
        }
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_create_duplicate_item_returns_conflict() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let item = TestItem {
        id: "dup-item".into(),
        pk: "pk1".into(),
        value: 1,
    };

    emu_container
        .create_item("pk1", "dup-item", &item, None)
        .await
        .unwrap();
    if let Some(ref real) = real_container {
        real.create_item("pk1", "dup-item", &item, None)
            .await
            .unwrap();
    }

    let emu_err = emu_container
        .create_item("pk1", "dup-item", &item, None)
        .await
        .expect_err("emulator: duplicate create should fail");
    assert_eq!(
        emu_err.http_status(),
        Some(StatusCode::Conflict),
        "emulator: duplicate create should return 409",
    );

    if let Some(ref real) = real_container {
        let real_err = real
            .create_item("pk1", "dup-item", &item, None)
            .await
            .expect_err("real: duplicate create should fail");
        compare_sdk_errors(&real_err, &emu_err);
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_read_nonexistent_item_returns_not_found() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let emu_err = emu_container
        .read_item::<TestItem>("pk1", "does-not-exist", None)
        .await
        .expect_err("emulator: reading nonexistent item should fail");
    assert_eq!(
        emu_err.http_status(),
        Some(StatusCode::NotFound),
        "emulator: nonexistent item should return 404",
    );

    if let Some(ref real) = real_container {
        let real_err = real
            .read_item::<TestItem>("pk1", "does-not-exist", None)
            .await
            .expect_err("real: reading nonexistent item should fail");
        compare_sdk_errors(&real_err, &emu_err);
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_read_with_stale_session_token_returns_error() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let mut operation = OperationOptions::default();
    operation.max_session_retry_count = Some(0);
    let read_options = ItemReadOptions::default()
        .with_session_token("0:-1#999999")
        .with_operation_options(operation);

    let emu_err = emu_container
        .read_item::<TestItem>("pk1", "no-such-item", Some(read_options.clone()))
        .await
        .expect_err("emulator should return error for stale session read");
    assert_eq!(
        emu_err.http_status(),
        Some(StatusCode::NotFound),
        "emulator error should be HTTP 404",
    );

    if let Some(ref real) = real_container {
        let mut operation = OperationOptions::default();
        operation.max_session_retry_count = Some(0);
        let real_read_options = ItemReadOptions::default()
            .with_session_token("0:-1#999999")
            .with_operation_options(operation);

        let real_err = real
            .read_item::<TestItem>("pk1", "no-such-item", Some(real_read_options))
            .await
            .expect_err("real should return error for stale session read");
        compare_sdk_errors(&real_err, &emu_err);
    }

    backend.cleanup_real_database(&db_name).await;
}

// ─── Multi-region fault injection via SDK ────────────────────────────────────

/// Demonstrates combining the in-memory emulator with fault injection through
/// the SDK (`CosmosClient` → `ContainerClient`).
///
/// Setup:
/// - Multi-region emulator: East US (write) + West US (read-only), immediate
///   replication, session consistency.
/// - Fault rule: 503 ServiceUnavailable on ReadItem in East US with a hit
///   limit so the driver exhausts local retries then fails over.
///
/// Flow:
/// 1. Build a `CosmosClient` using the emulator's `runtime_builder()` with
///    fault injection rules applied.
/// 2. Create an item via the SDK.
/// 3. Read the item — the driver hits 503 in East US, retries, and fails
///    over to West US.
/// 4. Verify the read succeeds with 200, correct typed body, and all
///    expected Cosmos headers.
///
/// When `AZURE_COSMOS_CONNECTION_STRING` is set, a second `CosmosClient`
/// (backed by a real account) runs the same scenario and responses are
/// compared via [`compare_item_responses`].
#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn sdk_read_failover_on_503_via_fault_injection() {
    use azure_data_cosmos_driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use azure_data_cosmos_driver::in_memory_emulator::{
        ReplicationConfig, VirtualAccountConfig, VirtualRegion, WriteMode,
    };
    use azure_data_cosmos_driver::options::Region as DriverRegion;
    use std::sync::Arc;

    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    let run_id = Uuid::new_v4().to_string()[..8].to_string();

    // ── Fault injection rule ─────────────────────────────────────
    let fault_result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let fault_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(DriverRegion::EAST_US)
        .build();
    let emu_rule = Arc::new(
        FaultInjectionRuleBuilder::new("sdk-read-503-east", fault_result.clone())
            .with_condition(fault_condition.clone())
            .with_hit_limit(4)
            .build(),
    );

    // ── Multi-region emulator ────────────────────────────────────
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", azure_core::http::Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", azure_core::http::Url::parse(west_url).unwrap()),
    ])
    .with_write_mode(WriteMode::Single)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator_store = emulator.store();

    // Build the runtime with fault injection layered on top of the emulator.
    let runtime_builder = emulator
        .runtime_builder()
        .with_fault_injection_rules(vec![Arc::clone(&emu_rule)]);

    // Provision resources in the emulator store.
    let db_name = format!("sdk-fi-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container(
        &db_name,
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    // Build the SDK client with the emulator runtime.
    let emu_account = CosmosAccountReference::with_master_key(
        east_url.parse().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let emu_client = CosmosClientBuilder::new()
        .with_driver_runtime_builder(runtime_builder)
        .build(emu_account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();

    let emu_container = emu_client
        .database_client(&db_name)
        .container_client("testcoll")
        .await
        .unwrap();

    // ── Create item ──────────────────────────────────────────────
    let item = TestItem {
        id: "fi-item".into(),
        pk: "pk1".into(),
        value: 42,
    };
    let emu_create = emu_container
        .create_item("pk1", "fi-item", &item, Some(write_options_with_content()))
        .await
        .unwrap();
    assert_emulator_item_response(&emu_create, StatusCode::Created);

    // ── Read item — should failover from East US → West US ───────
    let emu_read = emu_container
        .read_item::<TestItem>("pk1", "fi-item", None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    // Verify the fault rule was hit (confirms 503 was injected).
    assert!(
        emu_rule.hit_count() > 0,
        "Fault rule should have been hit at least once (was hit {} times)",
        emu_rule.hit_count(),
    );

    // Verify response headers.
    assert!(emu_read.etag().is_some(), "etag should be present");
    let snap = snapshot_from_item_response(&emu_read, "emulator");
    assert!(snap.headers.activity_id.is_some(), "activity_id present");
    assert!(snap.headers.etag.is_some(), "etag present");
    assert!(
        snap.headers.request_charge.is_some(),
        "request_charge present",
    );
    assert!(
        snap.headers.session_token.is_some(),
        "session_token present",
    );
    assert!(
        snap.headers.server_duration_ms.is_some(),
        "server_duration_ms present",
    );
    assert!(
        snap.sub_status_code.is_none(),
        "successful read should have no substatus",
    );

    // Verify typed body.
    let emu_doc: TestItem = emu_read.into_body().json().unwrap();
    assert_eq!(emu_doc.id, "fi-item");
    assert_eq!(emu_doc.pk, "pk1");
    assert_eq!(emu_doc.value, 42);

    // ── Real account comparison (if available) ───────────────────
    if let Ok(Some(real_client)) =
        resolve_real_client_with_fault_injection(fault_condition, fault_result).await
    {
        let real_db_name = format!("sdk-fi-real-{run_id}");
        // Create DB + container on real account.
        real_client
            .create_database(&real_db_name, None)
            .await
            .unwrap();
        let real_db = real_client.database_client(&real_db_name);
        let props = ContainerProperties::new("testcoll".to_string(), "/pk".into());
        real_db.create_container(props, None).await.unwrap();
        let real_container = real_db.container_client("testcoll").await.unwrap();

        // Create item.
        let real_create = real_container
            .create_item("pk1", "fi-item", &item, Some(write_options_with_content()))
            .await
            .unwrap();
        assert_eq!(real_create.status(), StatusCode::Created);

        // Read item — should also failover.
        let real_read = real_container
            .read_item::<TestItem>("pk1", "fi-item", None)
            .await
            .unwrap();
        assert_eq!(real_read.status(), StatusCode::Ok);

        // Compare real vs. emulator read headers.
        // `snap` was captured from the emulator read before `into_body()` consumed it.
        let real_snap = snapshot_from_item_response(&real_read, "real");
        compare_responses(
            &real_snap,
            &snap,
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        );
        let real_doc: TestItem = real_read.into_body().json().unwrap();
        assert_eq!(real_doc.id, "fi-item");
        assert_eq!(real_doc.value, 42);

        // Cleanup.
        let _ = real_db.delete(None).await;
    }
}

/// Builds a real-account `CosmosClient` with fault injection rules matching the
/// emulator test. Returns `None` when no real account is configured.
///
/// Fault injection is applied at the driver runtime level via
/// `with_fault_injection_rules` on the runtime builder, then passed into the
/// SDK via `CosmosClientBuilder::with_driver_runtime_builder`.
#[cfg(feature = "fault_injection")]
async fn resolve_real_client_with_fault_injection(
    _condition: azure_data_cosmos_driver::fault_injection::FaultInjectionCondition,
    _result: azure_data_cosmos_driver::fault_injection::FaultInjectionResult,
) -> Result<Option<CosmosClient>, Box<dyn Error>> {
    use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    use azure_data_cosmos_driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use azure_data_cosmos_driver::options::Region as DriverRegion;
    use std::sync::Arc;

    let mode = std::env::var(TEST_MODE_ENV_VAR)
        .unwrap_or_default()
        .to_lowercase();
    if mode == "skipped" {
        return Ok(None);
    }

    let conn_str_raw = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) => val,
        Err(_) => return Ok(None),
    };

    let conn_str: ConnectionString = conn_str_raw.parse()?;
    let endpoint = conn_str.account_endpoint().to_string();
    let key = conn_str.account_key().secret().to_string();

    let account = CosmosAccountReference::with_master_key(
        endpoint.parse().unwrap(),
        azure_core::credentials::Secret::new(key),
    );

    // Build a driver-level fault injection rule.
    let fi_result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let fi_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(DriverRegion::EAST_US)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("sdk-read-503-east-real", fi_result)
            .with_condition(fi_condition)
            .with_hit_limit(4)
            .build(),
    );

    // Apply fault injection to the runtime builder and pass it to the SDK.
    let runtime_builder = CosmosDriverRuntime::builder().with_fault_injection_rules(vec![rule]);

    let client = CosmosClientBuilder::new()
        .with_driver_runtime_builder(runtime_builder)
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    Ok(Some(client))
}

// ─── Helper ──────────────────────────────────────────────────────────────────

async fn resolve_real_client() -> Result<Option<CosmosClient>, Box<dyn Error>> {
    let mode = std::env::var(TEST_MODE_ENV_VAR)
        .unwrap_or_default()
        .to_lowercase();
    if mode == "skipped" {
        return Ok(None);
    }

    let conn_str_raw = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) => val,
        Err(_) => {
            if mode == "required" {
                panic!(
                    "{} is not set but test mode is 'required'",
                    CONNECTION_STRING_ENV_VAR
                );
            }
            println!("  [sdk-e2e] Real account not configured — emulator-only mode");
            return Ok(None);
        }
    };

    let conn_str: ConnectionString = conn_str_raw.parse()?;
    let endpoint = conn_str.account_endpoint().to_string();
    let key = conn_str.account_key().secret().to_string();

    let account = CosmosAccountReference::with_master_key(
        endpoint.parse().unwrap(),
        azure_core::credentials::Secret::new(key),
    );

    let client = CosmosClientBuilder::new()
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    Ok(Some(client))
}
