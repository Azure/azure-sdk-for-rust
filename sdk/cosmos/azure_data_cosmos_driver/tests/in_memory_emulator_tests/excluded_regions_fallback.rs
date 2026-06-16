// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for the regional fallback that fires when
//! every preferred region is excluded.
//!
//! These tests are the in-memory replacement for the live multi-region
//! integration tests that previously lived under
//! `tests/multi_region_tests/` and `tests/multi_write_tests/`. They drive
//! the full `CosmosDriver::execute_operation` pipeline against the
//! in-memory emulator and capture every dispatched request via a
//! `RequestObserver` so the test asserts the exact endpoint host used
//! for each request, not just "some regional endpoint".
//!
//! The behavior under test is:
//!
//! When the caller's `OperationOptions::excluded_regions` removes every
//! region from the preferred lists, the pipeline must fall back to the
//! **hub regional endpoint** (the first published write region), and
//! must never fall back to the global `*.documents.azure.com` endpoint —
//! which is not behind ATM (Azure Traffic Manager) and therefore unsafe
//! for data-plane and pipeline-routed metadata traffic.
//!
//! Three scenarios are covered:
//!
//! 1. **Multi-master account, data-plane read** — both regions write &
//!    read, so the fallback path must pick the first write region as
//!    the hub.
//! 2. **Single-master account, data-plane read** — only the write region
//!    is in `preferred_write_endpoints`, and the fallback must resolve
//!    to it even though the read endpoint list also held it.
//! 3. **Single-master account, pipeline-routed metadata read
//!    (`read_database`)** — guards the metadata side of the same fix.
//!    Until the production change in
//!    `operation_pipeline::resolve_endpoint`, metadata operations fell
//!    through to the global endpoint; they must now land on the hub
//!    regional endpoint instead.
//!
//! All three tests use the same observer pattern: capture the host of
//! every dispatched request, ignore the `GET /` account-topology fetches
//! that bootstrap the driver (those legitimately target the global
//! endpoint), and assert the surviving data-plane / metadata requests
//! all hit the hub.

use std::sync::{Arc, Mutex};

use azure_core::http::{Method, Request, Url};

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, RequestObserver,
    VirtualAccountConfig, VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, DatabaseReference, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{ExcludedRegions, OperationOptions, Region};

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
const EAST_HOST: &str = "eastus.emulator.local";

/// `RequestObserver` that records the host of every dispatched request
/// along with whether the request was an account-topology fetch
/// (`GET /`). Tests filter out the `GET /` requests when asserting where
/// data-plane and metadata-CRUD traffic landed, because account-topology
/// fetches bypass the routing path under test.
#[derive(Debug, Default)]
struct HostRecorder {
    /// Tuples of `(host, is_account_read)`.
    requests: Mutex<Vec<(String, bool)>>,
}

impl HostRecorder {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Hosts of all requests EXCEPT account-topology reads (`GET /`).
    fn data_plane_hosts(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, is_account)| !*is_account)
            .map(|(host, _)| host.clone())
            .collect()
    }
}

impl RequestObserver for HostRecorder {
    fn on_request(&self, request: &Request) {
        let host = request.url().host_str().unwrap_or_default().to_string();
        let is_account_read = request.method() == Method::Get && request.url().path() == "/";
        self.requests.lock().unwrap().push((host, is_account_read));
    }
}

/// Builds a two-region in-memory emulator (East US = hub, West US =
/// satellite) with a pre-provisioned `testdb`/`testcoll`. `write_mode`
/// selects single- vs multi-master semantics. The supplied observer is
/// attached so callers can read back the per-request host afterwards.
fn build_two_region_emulator(
    write_mode: WriteMode,
    observer: Arc<HostRecorder>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()),
    ])
    .unwrap()
    .with_write_mode(write_mode)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = InMemoryEmulatorHttpClient::new(config).with_request_observer(observer);
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );
    Arc::new(emulator)
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(EAST_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Multi-master regional fallback: when every preferred region is in
/// `excluded_regions`, a data-plane read must land on the **hub regional
/// endpoint** (first published write region) rather than the global
/// endpoint.
///
/// Regression guard for the fix in `operation_pipeline::resolve_endpoint`
/// — without it, the same scenario routed data-plane traffic to the
/// global endpoint, which is not behind ATM and silently violated the
/// region-exclusion preference at the network layer.
#[tokio::test]
async fn dataplane_excluded_all_preferred_multi_master_falls_back_to_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_two_region_emulator(WriteMode::Multi, recorder.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize");

    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");

    // Seed the item without exclusions so it lands in the store. With
    // `ReplicationConfig::immediate()` the multi-master account
    // propagates the write to both regions synchronously, so any
    // subsequent read that reaches either region will find it.
    let pk = "pk1";
    let item_id = "fallback-item-multi";
    let body = serde_json::json!({"id": item_id, "pk": pk, "value": 1}).to_string();
    let seed_item =
        ItemReference::from_name(&container, PartitionKey::from(pk), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(seed_item).with_body(body.into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("seeding write should succeed before exclusion is applied");

    // Drop everything captured during init + seeding — only the post-
    // exclusion read matters for the assertion.
    recorder.requests.lock().unwrap().clear();

    let read_item =
        ItemReference::from_name(&container, PartitionKey::from(pk), item_id.to_string());
    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([
        Region::EAST_US,
        Region::WEST_US,
    ]));
    driver
        .execute_operation(CosmosOperation::read_item(read_item), opts)
        .await
        .expect("read must succeed via the hub fallback");

    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty(),
        "expected the post-exclusion read to issue at least one data-plane request"
    );
    for host in &hosts {
        assert_eq!(
            host, EAST_HOST,
            "every data-plane request under all-regions-excluded must \
             land on the hub regional endpoint ({EAST_HOST}); observed hosts: {hosts:?}"
        );
    }
}

/// Single-master regional fallback: same assertion as the multi-master
/// test, but with `WriteMode::Single`. Exercises the path where
/// `preferred_write_endpoints` contains exactly one entry — the hub —
/// and the read endpoint list also contains the satellite. The fallback
/// must still resolve to the hub regional endpoint, not the global one.
#[tokio::test]
async fn dataplane_excluded_all_preferred_single_master_falls_back_to_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_two_region_emulator(WriteMode::Single, recorder.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize");

    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");

    let pk = "pk1";
    let item_id = "fallback-item-single";
    let body = serde_json::json!({"id": item_id, "pk": pk, "value": 1}).to_string();
    let seed_item =
        ItemReference::from_name(&container, PartitionKey::from(pk), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(seed_item).with_body(body.into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("seeding write should succeed before exclusion is applied");

    recorder.requests.lock().unwrap().clear();

    let read_item =
        ItemReference::from_name(&container, PartitionKey::from(pk), item_id.to_string());
    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([
        Region::EAST_US,
        Region::WEST_US,
    ]));
    driver
        .execute_operation(CosmosOperation::read_item(read_item), opts)
        .await
        .expect("read must succeed via the hub fallback");

    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty(),
        "expected the post-exclusion read to issue at least one data-plane request"
    );
    for host in &hosts {
        assert_eq!(
            host, EAST_HOST,
            "every data-plane request under all-regions-excluded must \
             land on the hub regional endpoint ({EAST_HOST}); observed hosts: {hosts:?}"
        );
    }
}

/// Pipeline-routed metadata-CRUD fallback: a `read_database` flows
/// through the same `resolve_endpoint` path as data-plane operations.
/// Before the fix, this returned the global endpoint when every
/// preferred region was excluded; after the fix it must return the
/// hub regional endpoint. Account-topology fetches still legitimately
/// target the global endpoint via a different code path, so the
/// observer filters out `GET /` before asserting.
#[tokio::test]
async fn metadata_excluded_all_preferred_falls_back_to_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_two_region_emulator(WriteMode::Single, recorder.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize");

    // Drop init traffic so only the post-exclusion metadata op counts.
    recorder.requests.lock().unwrap().clear();

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());
    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([
        Region::EAST_US,
        Region::WEST_US,
    ]));
    driver
        .execute_operation(CosmosOperation::read_database(db_ref), opts)
        .await
        .expect("read_database must succeed via the hub fallback");

    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty(),
        "expected the metadata op to issue at least one non-`GET /` request"
    );
    for host in &hosts {
        assert_eq!(
            host, EAST_HOST,
            "every pipeline-routed metadata request under all-regions-excluded \
             must land on the hub regional endpoint ({EAST_HOST}); observed hosts: {hosts:?}"
        );
    }
}
