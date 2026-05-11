// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore unroutable Meiswinkel
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
//! The suite now covers the SDK item methods routed through the driver-backed
//! emulator (`create_item`, `read_item`, `replace_item`, `upsert_item`, and
//! `delete_item`) plus explicit control-plane create coverage. Most data-plane
//! tests still pre-provision emulator resources directly in the store so the
//! individual scenarios can stay focused on the SDK operation under test.

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::{ContainerProperties, DatabaseProperties};
use azure_data_cosmos::regions::Region;
use azure_data_cosmos::CosmosAccountReference;
use azure_data_cosmos::{
    ContentResponseOnWrite, CosmosClient, CosmosClientBuilder, ItemReadOptions, ItemResponse,
    ItemWriteOptions, OperationOptions, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PaddedTestItem {
    id: String,
    pk: String,
    value: i64,
    padding: String,
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

fn compare_item_responses_with_spec<T>(
    real: &ItemResponse<T>,
    emu: &ItemResponse<T>,
    header_spec: &HeaderValidationSpec,
) {
    let real_snap = snapshot_from_item_response(real, "real");
    let emu_snap = snapshot_from_item_response(emu, "emulator");
    compare_responses(
        &real_snap,
        &emu_snap,
        header_spec,
        BodyValidationSpec::Ignore, // body validated via typed deserialization
    );
}

/// Compares an emulator and real [`ItemResponse`] using the shared header
/// validation spec for point operations.
fn compare_item_responses<T>(real: &ItemResponse<T>, emu: &ItemResponse<T>) {
    compare_item_responses_with_spec(real, emu, &HeaderValidationSpec::for_point_operation());
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

fn make_stale_session_token(token: &str) -> String {
    let mut parts = token.split('#');
    let prefix = parts.next().unwrap_or(token);
    let Some(_) = parts.next() else {
        return format!("{prefix}#9999999999");
    };

    let region_progress: Vec<String> = parts
        .map(|segment| match segment.split_once('=') {
            Some((region_id, _)) => format!("{region_id}=9999999999"),
            None => segment.to_string(),
        })
        .collect();

    if region_progress.is_empty() {
        format!("{prefix}#9999999999")
    } else {
        format!("{prefix}#9999999999#{}", region_progress.join("#"))
    }
}

fn assert_read_session_not_available(err: &azure_core::Error, label: &str) {
    assert_eq!(
        err.http_status(),
        Some(StatusCode::NotFound),
        "{label}: stale session read should return 404",
    );
    match err.kind() {
        azure_core::error::ErrorKind::HttpResponse { error_code, .. } => {
            assert_eq!(
                error_code.as_deref(),
                Some("1002"),
                "{label}: stale session read should surface substatus 1002",
            );
        }
        other => panic!("{label}: expected HttpResponse error, got {other}"),
    }
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

/// Reads an item, retrying transient `503 ServiceUnavailable` errors a bounded
/// number of times. Used by failover tests where the SDK's failover budget can
/// occasionally be exhausted on the failing region under CI contention before
/// the routing layer marks the endpoint unavailable. Logs every attempt so we
/// can see in CI which retry succeeded (or whether 503s are still occurring).
async fn read_item_with_503_retry(
    container: &ContainerClient,
    pk: &'static str,
    id: &'static str,
    label: &str,
) -> ItemResponse<TestItem> {
    const MAX_ATTEMPTS: usize = 5;
    let mut last_err: Option<azure_core::Error> = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match container.read_item::<TestItem>(pk, id, None).await {
            Ok(resp) => {
                eprintln!("[{label}] read_item succeeded on attempt {attempt}/{MAX_ATTEMPTS}",);
                return resp;
            }
            Err(e) => {
                let is_503 = matches!(
                    e.kind(),
                    azure_core::error::ErrorKind::HttpResponse {
                        status: StatusCode::ServiceUnavailable,
                        ..
                    },
                );
                eprintln!(
                    "[{label}] read_item attempt {attempt}/{MAX_ATTEMPTS} failed (is_503={is_503}): {e}",
                );
                if !is_503 {
                    panic!("[{label}] read_item failed with non-503 error: {e}");
                }
                last_err = Some(e);
            }
        }
    }
    panic!(
        "[{label}] read_item exhausted {MAX_ATTEMPTS} attempts; last error: {}",
        last_err.expect("at least one attempt failed"),
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
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);

        let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
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

fn padded_test_item(id: &str, value: i64, padding_len: usize) -> PaddedTestItem {
    PaddedTestItem {
        id: id.to_string(),
        pk: "pk1".to_string(),
        value,
        padding: "x".repeat(padding_len),
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sdk_create_database_and_container_through_driver() {
    let backend = SdkDualBackend::setup().await.unwrap();
    let db_name = backend.unique_db_name();
    let container_name = "sdk_cp";

    let emu_create_db = backend
        .emulator_client
        .create_database(&db_name, None)
        .await
        .unwrap();
    assert_eq!(emu_create_db.status(), StatusCode::Created);

    if let Some(ref real_client) = backend.real_client {
        let real_create_db = real_client.create_database(&db_name, None).await.unwrap();
        assert_eq!(real_create_db.status(), emu_create_db.status());

        let real_db: DatabaseProperties = real_create_db.into_model().unwrap();
        assert_eq!(real_db.id, db_name);
    }

    let emu_db: DatabaseProperties = emu_create_db.into_model().unwrap();
    assert_eq!(emu_db.id, db_name);

    let props = ContainerProperties::new(container_name.to_string(), "/pk".into());
    let emu_db_client = backend.emulator_client.database_client(&db_name);
    let emu_create_container = emu_db_client
        .create_container(props.clone(), None)
        .await
        .unwrap();
    assert_eq!(emu_create_container.status(), StatusCode::Created);

    if let Some(ref real_client) = backend.real_client {
        let real_db_client = real_client.database_client(&db_name);
        let real_create_container = real_db_client
            .create_container(props.clone(), None)
            .await
            .unwrap();
        assert_eq!(
            real_create_container.status(),
            emu_create_container.status()
        );

        let real_container_props: ContainerProperties = real_create_container.into_model().unwrap();
        assert_eq!(real_container_props.id, container_name);
    }

    let emu_container_props: ContainerProperties = emu_create_container.into_model().unwrap();
    assert_eq!(emu_container_props.id, container_name);

    let _emu_container = emu_db_client
        .container_client(container_name)
        .await
        .unwrap();

    backend.cleanup_real_database(&db_name).await;
}
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
async fn sdk_replace_item() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let original = TestItem {
        id: "replace-me".into(),
        pk: "pk1".into(),
        value: 1,
    };
    let updated = TestItem {
        id: "replace-me".into(),
        pk: "pk1".into(),
        value: 99,
    };

    let emu_create = emu_container
        .create_item(
            "pk1",
            &original.id,
            &original,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    assert_emulator_item_response(&emu_create, StatusCode::Created);

    if let Some(ref real) = real_container {
        let real_create = real
            .create_item(
                "pk1",
                &original.id,
                &original,
                Some(write_options_with_content()),
            )
            .await
            .unwrap();
        compare_item_responses(&real_create, &emu_create);
    }

    let emu_replace = emu_container
        .replace_item(
            "pk1",
            &updated.id,
            &updated,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    assert_emulator_item_response(&emu_replace, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_replace = real
            .replace_item(
                "pk1",
                &updated.id,
                &updated,
                Some(write_options_with_content()),
            )
            .await
            .unwrap();
        compare_item_responses(&real_replace, &emu_replace);

        let real_doc: TestItem = real_replace.into_body().json().unwrap();
        assert_eq!(real_doc.value, 99);
    }

    let emu_doc: TestItem = emu_replace.into_body().json().unwrap();
    assert_eq!(emu_doc.value, 99);

    let emu_read = emu_container
        .read_item::<TestItem>("pk1", &updated.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_read = real
            .read_item::<TestItem>("pk1", &updated.id, None)
            .await
            .unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().json().unwrap();
        assert_eq!(real_doc.value, 99);
    }

    let emu_read_doc: TestItem = emu_read.into_body().json().unwrap();
    assert_eq!(emu_read_doc.value, 99);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_upsert_item() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let initial = TestItem {
        id: "upsert-item".into(),
        pk: "pk1".into(),
        value: 10,
    };
    let updated = TestItem {
        id: "upsert-item".into(),
        pk: "pk1".into(),
        value: 20,
    };

    let emu_upsert_create = emu_container
        .upsert_item(
            "pk1",
            &initial.id,
            &initial,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    assert_emulator_item_response(&emu_upsert_create, StatusCode::Created);

    if let Some(ref real) = real_container {
        let real_upsert_create = real
            .upsert_item(
                "pk1",
                &initial.id,
                &initial,
                Some(write_options_with_content()),
            )
            .await
            .unwrap();
        compare_item_responses(&real_upsert_create, &emu_upsert_create);
    }

    let emu_upsert_update = emu_container
        .upsert_item(
            "pk1",
            &updated.id,
            &updated,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    assert_emulator_item_response(&emu_upsert_update, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_upsert_update = real
            .upsert_item(
                "pk1",
                &updated.id,
                &updated,
                Some(write_options_with_content()),
            )
            .await
            .unwrap();
        compare_item_responses(&real_upsert_update, &emu_upsert_update);

        let real_doc: TestItem = real_upsert_update.into_body().json().unwrap();
        assert_eq!(real_doc.value, 20);
    }

    let emu_doc: TestItem = emu_upsert_update.into_body().json().unwrap();
    assert_eq!(emu_doc.value, 20);

    let emu_read = emu_container
        .read_item::<TestItem>("pk1", &updated.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_read = real
            .read_item::<TestItem>("pk1", &updated.id, None)
            .await
            .unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().json().unwrap();
        assert_eq!(real_doc.value, 20);
    }

    let emu_read_doc: TestItem = emu_read.into_body().json().unwrap();
    assert_eq!(emu_read_doc.value, 20);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_delete_item() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let item = TestItem {
        id: "delete-me".into(),
        pk: "pk1".into(),
        value: 1,
    };

    let emu_create = emu_container
        .create_item("pk1", &item.id, &item, Some(write_options_with_content()))
        .await
        .unwrap();
    assert_emulator_item_response(&emu_create, StatusCode::Created);

    if let Some(ref real) = real_container {
        let real_create = real
            .create_item("pk1", &item.id, &item, Some(write_options_with_content()))
            .await
            .unwrap();
        compare_item_responses(&real_create, &emu_create);
    }

    let emu_delete = emu_container
        .delete_item("pk1", &item.id, None)
        .await
        .unwrap();
    assert_eq!(emu_delete.status(), StatusCode::NoContent);

    if let Some(ref real) = real_container {
        let real_delete = real.delete_item("pk1", &item.id, None).await.unwrap();
        compare_item_responses_with_spec(
            &real_delete,
            &emu_delete,
            &HeaderValidationSpec::for_delete_operation(),
        );
    }

    let emu_err = emu_container
        .read_item::<TestItem>("pk1", &item.id, None)
        .await
        .expect_err("emulator: reading deleted item should fail");
    assert_eq!(emu_err.http_status(), Some(StatusCode::NotFound));

    if let Some(ref real) = real_container {
        let real_err = real
            .read_item::<TestItem>("pk1", &item.id, None)
            .await
            .expect_err("real: reading deleted item should fail");
        compare_sdk_errors(&real_err, &emu_err);
    }

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

    let seed = TestItem {
        id: "seed-for-session".into(),
        pk: "pk1".into(),
        value: 0,
    };
    let emu_seed = emu_container
        .create_item("pk1", &seed.id, &seed, Some(write_options_with_content()))
        .await
        .expect("emulator seed create should succeed");
    let emu_seed_headers = CosmosResponseHeaders::from_headers(emu_seed.headers());
    let emu_stale_token = make_stale_session_token(
        emu_seed_headers
            .session_token
            .as_ref()
            .expect("emulator seed create should return a session token")
            .as_str(),
    );

    let mut operation = OperationOptions::default();
    operation.max_session_retry_count = Some(0);
    let read_options = ItemReadOptions::default()
        .with_session_token(emu_stale_token)
        .with_operation_options(operation);

    let emu_err = emu_container
        .read_item::<TestItem>("pk1", "seed-for-session", Some(read_options.clone()))
        .await
        .expect_err("emulator should return error for stale session read");
    assert_read_session_not_available(&emu_err, "emulator");

    if let Some(ref real) = real_container {
        let real_seed = real
            .create_item("pk1", &seed.id, &seed, Some(write_options_with_content()))
            .await
            .expect("real seed create should succeed");
        let real_seed_headers = CosmosResponseHeaders::from_headers(real_seed.headers());
        let real_stale_token = make_stale_session_token(
            real_seed_headers
                .session_token
                .as_ref()
                .expect("real seed create should return a session token")
                .as_str(),
        );

        let mut operation = OperationOptions::default();
        operation.max_session_retry_count = Some(0);
        let real_read_options = ItemReadOptions::default()
            .with_session_token(real_stale_token)
            .with_operation_options(operation);

        match real
            .read_item::<TestItem>("pk1", "seed-for-session", Some(real_read_options))
            .await
        {
            Err(real_err) => {
                assert_read_session_not_available(&real_err, "real");
                compare_sdk_errors(&real_err, &emu_err);
            }
            Ok(real_resp) => {
                let real_doc: TestItem = real_resp.into_body().json().unwrap();
                assert_eq!(real_doc.id, "seed-for-session");
                assert_eq!(real_doc.pk, "pk1");
            }
        }
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn sdk_create_retries_after_429_throttling() {
    let run_id = Uuid::new_v4().to_string()[..8].to_string();

    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_throttling_enabled(true);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();

    let db_name = format!("sdk-throttle-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container_with_config(
        &db_name,
        "throttle_coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

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
        .await
        .unwrap();

    let emu_container = emulator_client
        .database_client(&db_name)
        .container_client("throttle_coll")
        .await
        .unwrap();

    let seed = padded_test_item("seed-throttle", 1, 40 * 1024);
    emu_container
        .create_item("pk1", &seed.id, &seed, Some(write_options_with_content()))
        .await
        .unwrap();

    let throttled = padded_test_item("throttled-item", 42, 8 * 1024);
    let start = std::time::Instant::now();
    let emu_create = emu_container
        .create_item(
            "pk1",
            &throttled.id,
            &throttled,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let elapsed = start.elapsed();

    assert!(
        elapsed >= std::time::Duration::from_millis(200),
        "create should have retried after a 429 throttling response (elapsed: {:?})",
        elapsed,
    );
    assert_emulator_item_response(&emu_create, StatusCode::Created);

    let emu_doc: PaddedTestItem = emu_create.into_body().json().unwrap();
    assert_eq!(emu_doc.value, 42);
    assert_eq!(emu_doc.padding.len(), 8 * 1024);

    let emu_read = emu_container
        .read_item::<PaddedTestItem>("pk1", &throttled.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    let emu_read_doc: PaddedTestItem = emu_read.into_body().json().unwrap();
    assert_eq!(emu_read_doc.value, 42);
    assert_eq!(emu_read_doc.padding.len(), 8 * 1024);
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
#[ignore = "TODO(@FabianMeiswinkel): re-enable — tracked by https://github.com/Azure/azure-sdk-for-rust/issues/4365"]
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
    // No hit limit: East ALWAYS returns 503. This is the only way to truly enforce
    // that the eventual successful read came from West (failover actually happened),
    // since a hit limit would let the SDK eventually succeed on East after the rule
    // expires. The runtime below bumps `max_failover_retry_count` to give the SDK
    // enough budget to reach West even under CI contention, where the
    // MarkEndpointUnavailable effect can take longer to propagate across attempts.
    let emu_rule = Arc::new(
        FaultInjectionRuleBuilder::new("sdk-read-503-east", fault_result.clone())
            .with_condition(fault_condition.clone())
            .build(),
    );

    // ── Multi-region emulator ────────────────────────────────────
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", azure_core::http::Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", azure_core::http::Url::parse(west_url).unwrap()),
    ])
    .unwrap()
    .with_write_mode(WriteMode::Single)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();

    // Build the runtime with fault injection layered on top of the emulator.
    let runtime_builder = emulator
        .runtime_builder()
        .with_fault_injection_rules(vec![Arc::clone(&emu_rule)])
        .expect("distinct fault injection rule id");

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
    // The fault rule has no hit limit, so East ALWAYS returns 503. A successful
    // read can therefore only come from West — which is exactly what we want to
    // verify (real failover, not just rule expiry). Under CI contention the
    // SDK's failover budget (default `max_failover_retry_count = 3`) can
    // occasionally be exhausted on East before `MarkEndpointUnavailable`
    // propagates, surfacing the injected 503 to the caller. The retry helper
    // gives the routing layer additional attempts to converge on the
    // failed-over endpoint, and logs which attempt succeeded.
    let emu_read = read_item_with_503_retry(&emu_container, "pk1", "fi-item", "emulator").await;
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

        // Read item — should also failover. Same retry policy as the emulator side.
        let real_read = read_item_with_503_retry(&real_container, "pk1", "fi-item", "real").await;
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
        Ok(val) if !val.is_empty() => val,
        _ => return Ok(None),
    };

    // The CI test-setup script sets the value to "emulator" as a sentinel
    // when the Docker Cosmos DB Emulator is running. That is not a real
    // connection string — skip real-account comparison.
    if conn_str_raw.eq_ignore_ascii_case("emulator") {
        return Ok(None);
    }
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
            .build(),
    );

    // Apply fault injection to the runtime builder and pass it to the SDK.
    let runtime_builder = CosmosDriverRuntime::builder().with_fault_injection_rules(vec![rule])?;

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
        Ok(val) if !val.is_empty() => val,
        _ => {
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

    // The CI test-setup script sets the value to "emulator" as a sentinel
    // when the Docker Cosmos DB Emulator is running. That is not a real
    // connection string — skip real-account comparison.
    if conn_str_raw.eq_ignore_ascii_case("emulator") {
        return Ok(None);
    }
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
