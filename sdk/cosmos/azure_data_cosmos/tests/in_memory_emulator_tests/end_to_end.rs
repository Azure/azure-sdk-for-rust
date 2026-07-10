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
use azure_data_cosmos::{
    models::{ContainerProperties, DatabaseProperties, ItemResponse, ThroughputProperties},
    options::{
        AvailabilityStrategy, ContentResponseOnWrite, CreateContainerOptions, ItemReadOptions,
        ItemWriteOptions, OperationOptions, OperationOptionsBuilder, Region,
        ThrottlingRetryOptionsBuilder,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, Query, RoutingStrategy, TransactionalBatch,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, Instant};
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
struct QueryTestItem {
    id: String,
    pk: String,
    score: i64,
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
fn snapshot_from_item_response(resp: &ItemResponse, label: &str) -> ResponseSnapshot {
    let headers = resp.headers().clone();
    let sub_status_code = headers.substatus().map(|s| s.value());
    ResponseSnapshot {
        status_code: u16::from(resp.status()),
        sub_status_code,
        headers: headers.__into_driver_headers(),
        body: None, // body comparison handled separately via deserialization
        label: label.to_owned(),
    }
}

fn compare_item_responses_with_spec(
    real: &ItemResponse,
    emu: &ItemResponse,
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
fn compare_item_responses(real: &ItemResponse, emu: &ItemResponse) {
    compare_item_responses_with_spec(real, emu, &HeaderValidationSpec::for_point_operation());
}

/// Compares two SDK error responses: both must have the same HTTP status.
fn compare_sdk_errors(real: &azure_data_cosmos::CosmosError, emu: &azure_data_cosmos::CosmosError) {
    assert_eq!(
        real.status().status_code(),
        emu.status().status_code(),
        "CosmosError status mismatch: real={:?} emulator={:?}",
        real.status().status_code(),
        emu.status().status_code(),
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

fn assert_read_session_not_available(err: &azure_data_cosmos::CosmosError, label: &str) {
    assert_eq!(
        err.status().status_code(),
        StatusCode::NotFound,
        "{label}: stale session read should return 404",
    );
    assert_eq!(
        err.status().sub_status().map(|s| s.value()),
        Some(1002),
        "{label}: stale session read should surface substatus 1002",
    );
}

/// Asserts a stale-session read was rejected on the real backend, tolerating a
/// documented gateway-implementation divergence: classic gateway returns
/// 404 / sub-status 1002 (ReadSessionNotAvailable), while the Gateway 2.0
/// thin-client path surfaces the backend's structural session-token rejection
/// as 400 BadRequest ("Session token specified is invalid."). A bumped-LSN
/// token only trips the soft path on the shared backend; GW2 instead rejects
/// the fabricated token's region structure. Both are valid "this session token
/// cannot be satisfied" signals, so accept either.
fn assert_stale_session_rejected(err: &azure_data_cosmos::CosmosError, label: &str) {
    match err.status().status_code() {
        StatusCode::NotFound => assert_eq!(
            err.status().sub_status().map(|s| s.value()),
            Some(1002),
            "{label}: 404 stale session read should surface substatus 1002",
        ),
        StatusCode::BadRequest => {}
        other => panic!(
            "{label}: stale session read should return 404/1002 or 400 BadRequest, got {other:?}",
        ),
    }
}

/// Asserts emulator-only response metadata when no real account is available.
fn assert_emulator_item_response(resp: &ItemResponse, expected_status: StatusCode) {
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
#[cfg(feature = "fault_injection")]
async fn read_item_with_503_retry(
    container: &ContainerClient,
    pk: &'static str,
    id: &'static str,
    label: &str,
) -> ItemResponse {
    const MAX_ATTEMPTS: usize = 5;
    let mut last_err: Option<azure_data_cosmos::CosmosError> = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match container.read_item(pk, id, None).await {
            Ok(resp) => {
                eprintln!("[{label}] read_item succeeded on attempt {attempt}/{MAX_ATTEMPTS}",);
                return resp;
            }
            Err(e) => {
                let is_503 = e.status().status_code() == StatusCode::ServiceUnavailable;
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
const SETUP_TIMEOUT_SECONDS_ENV_VAR: &str = "AZURE_COSMOS_TEST_SETUP_TIMEOUT_SECONDS";
const DEFAULT_SETUP_TIMEOUT_SECONDS: u64 = 180;

fn setup_timeout() -> Duration {
    std::env::var(SETUP_TIMEOUT_SECONDS_ENV_VAR)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(DEFAULT_SETUP_TIMEOUT_SECONDS))
}

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

        let emulator_account = AccountReference::with_authentication_key(
            EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
            azure_core::credentials::Secret::new("dGVzdGtleQ=="),
        );

        let emulator_client = CosmosClientBuilder::new()
            .with_runtime(
                CosmosRuntimeBuilder::from(emulator.runtime_builder())
                    .build()
                    .await?,
            )
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
            Some(resolve_container_when_ready(client, db_name, container_name).await?)
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

fn read_options_without_hedging() -> ItemReadOptions {
    let mut operation = OperationOptions::default();
    operation.hedging_enabled = Some(false);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    ItemReadOptions::default().with_operation_options(operation)
}

/// Extracts the session token's global LSN from a write response, asserting the
/// token is present (every write under Session consistency must return one).
fn session_global_lsn(resp: &ItemResponse, label: &str, op: &str) -> u64 {
    let token = resp
        .headers()
        .session_token()
        .unwrap_or_else(|| panic!("[{label}] {op} response must carry a session token"))
        .as_str();
    super::session_token::global_lsn(token)
}

/// Drives a create → replace → delete sequence against a single backend and
/// asserts the response session token's global LSN strictly advances on every
/// write. This is the live counterpart to the in-memory
/// `cache_advances_as_write_responses_arrive` test: real Cosmos bumps the
/// partition LSN (and thus the returned session token) on every write, and the
/// in-memory emulator must match so it stays a faithful test double.
async fn assert_session_token_advances(container: &ContainerClient, label: &str) {
    let pk = "pk1";
    let id = format!("advance-{label}");

    let created = container
        .create_item(
            pk,
            &id,
            &TestItem {
                id: id.clone(),
                pk: pk.into(),
                value: 1,
            },
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let create_lsn = session_global_lsn(&created, label, "create");

    let replaced = container
        .replace_item(
            pk,
            &id,
            &TestItem {
                id: id.clone(),
                pk: pk.into(),
                value: 2,
            },
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let replace_lsn = session_global_lsn(&replaced, label, "replace");

    let deleted = container.delete_item(pk, &id, None).await.unwrap();
    let delete_lsn = session_global_lsn(&deleted, label, "delete");

    assert!(
        replace_lsn > create_lsn,
        "[{label}] replace must advance the session token's global LSN: \
         create={create_lsn} replace={replace_lsn}"
    );
    assert!(
        delete_lsn > replace_lsn,
        "[{label}] delete must advance the session token's global LSN: \
         replace={replace_lsn} delete={delete_lsn}"
    );
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
        assert_eq!(real_db.id.as_deref(), Some(db_name.as_str()));
    }

    let emu_db: DatabaseProperties = emu_create_db.into_model().unwrap();
    assert_eq!(emu_db.id.as_deref(), Some(db_name.as_str()));

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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn sdk_query_metadata_databases_and_containers() {
    let (backend, db_name, _emu_container, _real_container) = setup_with_container().await;

    let db_query = Query::from("SELECT * FROM c WHERE c.id = @id")
        .with_parameter("@id", db_name.as_str())
        .unwrap();
    let emu_databases: Vec<DatabaseProperties> = backend
        .emulator_client
        .query_databases(db_query.clone(), None)
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(emu_databases.len(), 1);
    assert_eq!(emu_databases[0].id.as_deref(), Some(db_name.as_str()));

    if let Some(ref real_client) = backend.real_client {
        let real_databases: Vec<DatabaseProperties> = real_client
            .query_databases(db_query, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        assert_eq!(real_databases.len(), emu_databases.len());
        assert_eq!(real_databases[0].id, emu_databases[0].id);
    }

    let container_query = Query::from("SELECT * FROM c WHERE c.id = @id")
        .with_parameter("@id", "testcoll")
        .unwrap();
    let emu_containers: Vec<ContainerProperties> = backend
        .emulator_client
        .database_client(&db_name)
        .query_containers(container_query.clone(), None)
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(emu_containers.len(), 1);
    assert_eq!(emu_containers[0].id, "testcoll");

    if let Some(ref real_client) = backend.real_client {
        let real_containers: Vec<ContainerProperties> = real_client
            .database_client(&db_name)
            .query_containers(container_query, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        assert_eq!(real_containers.len(), emu_containers.len());
        assert_eq!(real_containers[0].id, emu_containers[0].id);
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn sdk_container_throughput_read_and_replace() {
    let backend = SdkDualBackend::setup().await.unwrap();
    let db_name = backend.unique_db_name();
    let container_name = "sdk_offer";
    let props = ContainerProperties::new(container_name.to_string(), "/pk".into());
    let options =
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(400));

    backend
        .emulator_client
        .create_database(&db_name, None)
        .await
        .unwrap();
    backend
        .emulator_client
        .database_client(&db_name)
        .create_container(props.clone(), Some(options.clone()))
        .await
        .unwrap();

    if let Some(ref real_client) = backend.real_client {
        real_client.create_database(&db_name, None).await.unwrap();
        real_client
            .database_client(&db_name)
            .create_container(props.clone(), Some(options))
            .await
            .unwrap();
    }

    let emu_container = backend
        .emulator_client
        .database_client(&db_name)
        .container_client(container_name)
        .await
        .unwrap();
    let emu_throughput = emu_container.read_throughput(None).await.unwrap().unwrap();
    assert_eq!(emu_throughput.throughput(), Some(400));

    let emu_replaced = emu_container
        .begin_replace_throughput(ThroughputProperties::manual(500), None)
        .await
        .unwrap()
        .await
        .unwrap()
        .into_model()
        .unwrap();
    assert_eq!(emu_replaced.throughput(), Some(500));
    let emu_throughput = emu_container.read_throughput(None).await.unwrap().unwrap();
    assert_eq!(emu_throughput.throughput(), Some(500));

    if let Some(ref real_client) = backend.real_client {
        let real_container = real_client
            .database_client(&db_name)
            .container_client(container_name)
            .await
            .unwrap();
        let real_throughput = real_container.read_throughput(None).await.unwrap().unwrap();
        assert_eq!(real_throughput.throughput(), Some(400));

        let real_replaced = real_container
            .begin_replace_throughput(ThroughputProperties::manual(500), None)
            .await
            .unwrap()
            .await
            .unwrap()
            .into_model()
            .unwrap();
        assert_eq!(real_replaced.throughput(), emu_replaced.throughput());
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
        .read_item("pk1", "sdk-item-1", None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);
    assert!(
        emu_read.headers().etag().is_some(),
        "emulator read should have etag"
    );

    if let Some(ref real) = real_container {
        let real_read = real.read_item("pk1", "sdk-item-1", None).await.unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().into_single().unwrap();
        assert_eq!(real_doc.id, "sdk-item-1");
        assert_eq!(real_doc.value, 42);
    }

    let emu_doc: TestItem = emu_read.into_body().into_single().unwrap();
    assert_eq!(emu_doc.id, "sdk-item-1");
    assert_eq!(emu_doc.value, 42);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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

        let real_doc: TestItem = real_replace.into_body().into_single().unwrap();
        assert_eq!(real_doc.value, 99);
    }

    let emu_doc: TestItem = emu_replace.into_body().into_single().unwrap();
    assert_eq!(emu_doc.value, 99);

    let emu_read = emu_container
        .read_item("pk1", &updated.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_read = real.read_item("pk1", &updated.id, None).await.unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().into_single().unwrap();
        assert_eq!(real_doc.value, 99);
    }

    let emu_read_doc: TestItem = emu_read.into_body().into_single().unwrap();
    assert_eq!(emu_read_doc.value, 99);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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

        let real_doc: TestItem = real_upsert_update.into_body().into_single().unwrap();
        assert_eq!(real_doc.value, 20);
    }

    let emu_doc: TestItem = emu_upsert_update.into_body().into_single().unwrap();
    assert_eq!(emu_doc.value, 20);

    let emu_read = emu_container
        .read_item("pk1", &updated.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    if let Some(ref real) = real_container {
        let real_read = real.read_item("pk1", &updated.id, None).await.unwrap();
        compare_item_responses(&real_read, &emu_read);

        let real_doc: TestItem = real_read.into_body().into_single().unwrap();
        assert_eq!(real_doc.value, 20);
    }

    let emu_read_doc: TestItem = emu_read.into_body().into_single().unwrap();
    assert_eq!(emu_read_doc.value, 20);

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
        .read_item("pk1", &item.id, None)
        .await
        .expect_err("emulator: reading deleted item should fail");
    assert_eq!(emu_err.status().status_code(), StatusCode::NotFound);

    if let Some(ref real) = real_container {
        let real_err = real
            .read_item("pk1", &item.id, None)
            .await
            .expect_err("real: reading deleted item should fail");
        compare_sdk_errors(&real_err, &emu_err);
    }

    backend.cleanup_real_database(&db_name).await;
}
#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn sdk_session_token_advances_on_create_replace_delete() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    assert_session_token_advances(&emu_container, "emulator").await;

    if let Some(ref real) = real_container {
        assert_session_token_advances(real, "real").await;
    }

    backend.cleanup_real_database(&db_name).await;
}
#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
        let emu_read = emu_container.read_item("pk1", &id, None).await.unwrap();
        assert_emulator_item_response(&emu_read, StatusCode::Ok);

        let emu_doc: TestItem = emu_read.into_body().into_single().unwrap();
        assert_eq!(emu_doc.value, i);

        if let Some(ref real) = real_container {
            let real_read = real.read_item("pk1", &id, None).await.unwrap();
            let real_doc: TestItem = real_read.into_body().into_single().unwrap();
            assert_eq!(real_doc.value, i);
        }
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn sdk_query_items_with_filter_and_projection() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    for i in 0..3 {
        let item = QueryTestItem {
            id: format!("query-{i}"),
            pk: "pk1".into(),
            score: i,
        };
        emu_container
            .create_item("pk1", &item.id, &item, None)
            .await
            .unwrap();
        if let Some(ref real) = real_container {
            real.create_item("pk1", &item.id, &item, None)
                .await
                .unwrap();
        }
    }

    fn query() -> Query {
        Query::from("SELECT * FROM c WHERE c.pk = @pk AND c.score >= @min")
            .with_parameter("@pk", "pk1")
            .unwrap()
            .with_parameter("@min", 1)
            .unwrap()
    }

    let emu_items: Vec<QueryTestItem> = emu_container
        .query_items(query(), FeedScope::partition("pk1"), None)
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();
    assert_eq!(emu_items.len(), 2);
    assert_eq!(emu_items[0].id, "query-1");
    assert_eq!(emu_items[1].id, "query-2");

    if let Some(ref real) = real_container {
        let real_items: Vec<QueryTestItem> = real
            .query_items(query(), FeedScope::partition("pk1"), None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap();
        assert_eq!(real_items.len(), emu_items.len());
        assert_eq!(
            real_items.iter().map(|i| &i.id).collect::<Vec<_>>(),
            emu_items.iter().map(|i| &i.id).collect::<Vec<_>>()
        );
    }

    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn sdk_transactional_batch_create_read_and_rollback() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let emu_batch = TransactionalBatch::new("pk1")
        .create_item(TestItem {
            id: "batch-1".into(),
            pk: "pk1".into(),
            value: 1,
        })
        .unwrap()
        .read_item("batch-1", None);
    let emu_response = emu_container
        .execute_transactional_batch(emu_batch, None)
        .await
        .unwrap();
    assert_eq!(emu_response.status(), StatusCode::Ok);
    let emu_model = emu_response.into_model().unwrap();
    assert_eq!(
        emu_model
            .results()
            .iter()
            .map(|r| r.status_code())
            .collect::<Vec<_>>(),
        vec![201, 200]
    );

    if let Some(ref real) = real_container {
        let real_batch = TransactionalBatch::new("pk1")
            .create_item(TestItem {
                id: "batch-1".into(),
                pk: "pk1".into(),
                value: 1,
            })
            .unwrap()
            .read_item("batch-1", None);
        let real_response = real
            .execute_transactional_batch(real_batch, None)
            .await
            .unwrap();
        assert_eq!(real_response.status(), StatusCode::Ok);
        let real_model = real_response.into_model().unwrap();
        assert_eq!(
            real_model
                .results()
                .iter()
                .map(|r| r.status_code())
                .collect::<Vec<_>>(),
            vec![201, 200]
        );
    }

    let failing_batch = TransactionalBatch::new("pk1")
        .create_item(TestItem {
            id: "batch-rollback".into(),
            pk: "pk1".into(),
            value: 9,
        })
        .unwrap()
        .delete_item("missing", None);
    let emu_response = emu_container
        .execute_transactional_batch(failing_batch, None)
        .await
        .unwrap();
    let emu_model = emu_response.into_model().unwrap();
    assert_eq!(
        emu_model
            .results()
            .iter()
            .map(|r| r.status_code())
            .collect::<Vec<_>>(),
        vec![424, 404]
    );
    let emu_err = emu_container
        .read_item("pk1", "batch-rollback", None)
        .await
        .expect_err("rolled-back batch item must not exist in emulator");
    assert_eq!(emu_err.status().status_code(), StatusCode::NotFound);

    if let Some(ref real) = real_container {
        let failing_batch = TransactionalBatch::new("pk1")
            .create_item(TestItem {
                id: "batch-rollback".into(),
                pk: "pk1".into(),
                value: 9,
            })
            .unwrap()
            .delete_item("missing", None);
        let real_response = real
            .execute_transactional_batch(failing_batch, None)
            .await
            .unwrap();
        let real_model = real_response.into_model().unwrap();
        assert_eq!(
            real_model
                .results()
                .iter()
                .map(|r| r.status_code())
                .collect::<Vec<_>>(),
            vec![424, 404]
        );
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
        emu_err.status().status_code(),
        StatusCode::Conflict,
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
    let read_options = read_options_without_hedging();

    let emu_err = emu_container
        .read_item("pk1", "does-not-exist", Some(read_options.clone()))
        .await
        .expect_err("emulator: reading nonexistent item should fail");
    assert_eq!(
        emu_err.status().status_code(),
        StatusCode::NotFound,
        "emulator: nonexistent item should return 404",
    );

    if let Some(ref real) = real_container {
        let real_err = real
            .read_item("pk1", "does-not-exist", Some(read_options))
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
    let emu_seed_headers = emu_seed.headers().clone();
    let emu_stale_token = make_stale_session_token(
        emu_seed_headers
            .session_token()
            .expect("emulator seed create should return a session token")
            .as_str(),
    );

    let mut operation = OperationOptions::default();
    operation.max_session_retry_count = Some(0);
    let read_options = ItemReadOptions::default()
        .with_session_token(emu_stale_token)
        .with_operation_options(operation);

    let emu_err = emu_container
        .read_item("pk1", "seed-for-session", Some(read_options.clone()))
        .await
        .expect_err("emulator should return error for stale session read");
    assert_read_session_not_available(&emu_err, "emulator");

    if let Some(ref real) = real_container {
        let real_seed = real
            .create_item("pk1", &seed.id, &seed, Some(write_options_with_content()))
            .await
            .expect("real seed create should succeed");
        let real_seed_headers = real_seed.headers().clone();
        let real_stale_token = make_stale_session_token(
            real_seed_headers
                .session_token()
                .expect("real seed create should return a session token")
                .as_str(),
        );

        let mut operation = OperationOptions::default();
        operation.max_session_retry_count = Some(0);
        let real_read_options = ItemReadOptions::default()
            .with_session_token(real_stale_token)
            .with_operation_options(operation);

        match real
            .read_item("pk1", "seed-for-session", Some(real_read_options))
            .await
        {
            Err(real_err) => {
                assert_stale_session_rejected(&real_err, "real");
            }
            Ok(real_resp) => {
                let real_doc: TestItem = real_resp.into_body().into_single().unwrap();
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

    let emulator_account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let emulator_client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
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

    let emu_doc: PaddedTestItem = emu_create.into_body().into_single().unwrap();
    assert_eq!(emu_doc.value, 42);
    assert_eq!(emu_doc.padding.len(), 8 * 1024);

    let emu_read = emu_container
        .read_item("pk1", &throttled.id, None)
        .await
        .unwrap();
    assert_emulator_item_response(&emu_read, StatusCode::Ok);

    let emu_read_doc: PaddedTestItem = emu_read.into_body().into_single().unwrap();
    assert_eq!(emu_read_doc.value, 42);
    assert_eq!(emu_read_doc.padding.len(), 8 * 1024);
}

/// Validates that disabling throttle retries via per-client default
/// [`OperationOptions`] containing a [`ThrottlingRetryOptions`] with
/// `max_retry_count = 0` is honored end-to-end.
///
/// This is the negative counterpart to [`sdk_create_retries_after_429_throttling`]:
/// the same throttling-enabled emulator setup is used, but the client is built
/// with `max_retry_count = 0` (retries disabled). With retries disabled the
/// driver must surface the first 429 to the caller instead of transparently
/// retrying, proving the grouped setter is honored end-to-end.
#[tokio::test]
async fn sdk_throttling_retry_options_disables_retry() {
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

    let db_name = format!("sdk-throttle-no-retry-{run_id}");
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

    let emulator_account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    // Build the client with the grouped throttling-retry options as the per-client
    // default operation options, disabling throttle retries (max_retry_count = 0).
    let emulator_client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .with_default_operation_options(
            OperationOptionsBuilder::new()
                .with_throttling_retry_options(
                    ThrottlingRetryOptionsBuilder::new()
                        .with_max_retry_count(0)
                        .build(),
                )
                .build(),
        )
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

    // Seed a large item to consume the partition's RU budget so the next write
    // is throttled (mirrors `sdk_create_retries_after_429_throttling`).
    let seed = padded_test_item("seed-throttle", 1, 40 * 1024);
    emu_container
        .create_item("pk1", &seed.id, &seed, Some(write_options_with_content()))
        .await
        .unwrap();

    // With retries disabled, the throttled create must surface the first 429
    // rather than retrying until the RU budget refills.
    let throttled = padded_test_item("throttled-item", 42, 8 * 1024);
    let err = emu_container
        .create_item(
            "pk1",
            &throttled.id,
            &throttled,
            Some(write_options_with_content()),
        )
        .await
        .expect_err("create should fail fast with a 429 when throttle retries are disabled");

    assert_eq!(
        err.status().status_code(),
        StatusCode::TooManyRequests,
        "throttled create should surface HTTP 429 when retries are disabled",
    );
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

    let runtime_builder = emulator.runtime_builder();

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
    let emu_account = AccountReference::with_authentication_key(
        east_url.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let emu_client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(runtime_builder)
                .build()
                .await
                .unwrap(),
        )
        .with_fault_injection_rules(vec![Arc::clone(&emu_rule)])
        .unwrap()
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
    assert!(
        emu_read.headers().etag().is_some(),
        "etag should be present"
    );
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
    let emu_doc: TestItem = emu_read.into_body().into_single().unwrap();
    assert_eq!(emu_doc.id, "fi-item");
    assert_eq!(emu_doc.pk, "pk1");
    assert_eq!(emu_doc.value, 42);

    // ── Real account comparison (if available) ───────────────────
    //
    // Runs the same 503-on-East scenario against the ARM-provisioned account
    // (when one is configured) and asserts the real service's response
    // matches the emulator's. Returns `Ok(None)` when no real account is
    // available (local dev, emulator-only CI legs) so the emulator portion
    // remains the single source of truth in those modes.
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
        // Real accounts provision containers asynchronously; tolerate the
        // transient 404/1013 CollectionCreateInProgress before the first read.
        let real_container = resolve_container_when_ready(&real_client, &real_db_name, "testcoll")
            .await
            .unwrap();

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
        let real_doc: TestItem = real_read.into_body().into_single().unwrap();
        assert_eq!(real_doc.id, "fi-item");
        assert_eq!(real_doc.value, 42);

        // Cleanup.
        let _ = real_db.delete(None).await;
    }
}

/// Builds a real-account `CosmosClient` with fault injection rules matching the
/// emulator test. Returns `Ok(None)` when no real account is configured (so
/// the test reduces to its emulator-only leg).
///
/// Fault injection is applied at the SDK builder level via
/// `with_fault_injection`; it is forwarded onto the per-driver options at
/// build time.
#[cfg(feature = "fault_injection")]
async fn resolve_real_client_with_fault_injection(
    condition: azure_data_cosmos_driver::fault_injection::FaultInjectionCondition,
    result: azure_data_cosmos_driver::fault_injection::FaultInjectionResult,
) -> Result<Option<CosmosClient>, Box<dyn Error>> {
    use azure_data_cosmos_driver::fault_injection::FaultInjectionRuleBuilder;
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

    let account = AccountReference::with_authentication_key(
        endpoint.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new(key),
    );

    // Mirror the emulator-side rule against the real account, using the same
    // condition/result the caller built so both legs of the test share a
    // single source of truth.
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("sdk-read-503-east-real", result)
            .with_condition(condition)
            .build(),
    );

    // Apply fault injection at the SDK builder layer.
    let client = CosmosClientBuilder::new()
        .with_fault_injection_rules(vec![rule])?
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

    let account = AccountReference::with_authentication_key(
        endpoint.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new(key),
    );

    let client = CosmosClientBuilder::new()
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    Ok(Some(client))
}

/// Detects the transient `404 / 1013 CollectionCreateInProgress` status that a
/// real Cosmos account returns while a freshly created container is still being
/// provisioned in the background.
fn collection_create_in_progress(err: &azure_data_cosmos::CosmosError) -> bool {
    let status = err.status();
    status.status_code() == StatusCode::NotFound
        && status.sub_status().map(|s| s.value()) == Some(1013)
}

/// Transient `401 Unauthorized` a freshly deployed account can briefly return
/// before its master key has propagated to every regional gateway (observed on
/// the multi-region Gateway 2.0 leg). Retriable only within the bounded setup
/// readiness window below; the SDK itself treats 401 as definitive.
fn transient_deployment_unauthorized(err: &azure_data_cosmos::CosmosError) -> bool {
    err.status().status_code() == StatusCode::Unauthorized
}

/// Resolves a container on a real account, tolerating asynchronous container
/// provisioning.
///
/// Real accounts create containers in the background, so the first metadata
/// resolve — and the first data-plane request — can fail with
/// `404 / 1013 CollectionCreateInProgress` for several seconds after
/// `create_container` returns. This polls (with exponential backoff up to a
/// bounded deadline) until the container both resolves and serves a data-plane
/// read, so live dual-backend tests don't flake on creation timing. The
/// in-memory emulator provisions synchronously and never hits this path.
async fn resolve_container_when_ready(
    client: &CosmosClient,
    db_name: &str,
    container_name: &str,
) -> Result<ContainerClient, Box<dyn Error>> {
    const MAX_BACKOFF: Duration = Duration::from_secs(5);
    let deadline = Instant::now() + setup_timeout();

    // Phase 1: resolve the container's metadata (routing / PK ranges).
    let mut backoff = Duration::from_millis(250);
    let container = loop {
        match client
            .database_client(db_name)
            .container_client(container_name)
            .await
        {
            Ok(container) => break container,
            Err(e)
                if (collection_create_in_progress(&e) || transient_deployment_unauthorized(&e))
                    && Instant::now() < deadline =>
            {
                if transient_deployment_unauthorized(&e) {
                    eprintln!("resolve_container_when_ready: retrying transient 401 during metadata resolve for {db_name}/{container_name}");
                }
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    };

    // Phase 2: confirm the container serves data-plane reads. A guaranteed-
    // missing item returns `404 / 1003 NotFound` once the container is ready,
    // versus `404 / 1013` while it is still provisioning.
    backoff = Duration::from_millis(250);
    loop {
        match container
            .read_item("readiness-probe", "readiness-probe", None)
            .await
        {
            Ok(_) => return Ok(container),
            Err(e)
                if e.status().status_code() == StatusCode::NotFound
                    && !collection_create_in_progress(&e) =>
            {
                return Ok(container)
            }
            Err(e) if collection_create_in_progress(&e) && Instant::now() < deadline => {
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) if transient_deployment_unauthorized(&e) && Instant::now() < deadline => {
                eprintln!("resolve_container_when_ready: retrying transient 401 during readiness probe for {db_name}/{container_name}");
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    }
}
