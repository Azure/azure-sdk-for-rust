// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for **region offline/online** and **failover
//! priority changes** — the two ARM operations the emulator gained after they
//! were observed against live accounts.
//!
//! Both accounts used for the observation had three regions, because a priority
//! swap between positions 1 and 2 (which must *not* move the write region) is
//! only distinguishable from one involving position 0 (which must) when a third
//! region exists.
//!
//! What the live service does, and what these tests pin:
//!
//! | Observation | Test |
//! | --- | --- |
//! | An offlined region leaves both `readableLocations` and `writableLocations` | [`offlined_region_is_dropped_from_both_location_lists`] |
//! | Its endpoint stops resolving (DNS withdrawn), rather than answering `403/1008` | [`request_to_offlined_region_fails_at_the_transport_layer`] |
//! | Offlining the write region fails over instead of being rejected | [`offlining_the_write_region_fails_over`] |
//! | A priority change that does not touch p0 is invisible to clients | [`priority_change_below_position_zero_is_invisible`] |
//! | On single-write, promotion to p0 moves write ownership | [`promotion_to_position_zero_moves_writes`] |
//! | On multi-write, it does not change who may write | [`promotion_to_position_zero_does_not_gate_writes_under_multi_write`] |
//! | A failover transiently advertises **two** writable locations | [`in_flight_failover_advertises_both_write_regions`] |
//! | The demoted region rejects writes while still advertised | [`demoted_region_rejects_writes_while_still_advertised`] |
//!
//! Time is virtualized with `tokio::time::pause()` (via `start_paused`) because
//! the production account-refresh interval is five minutes.

#[cfg(not(feature = "preview_dtx"))]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use azure_core::http::{Method, Request, StatusCode, Url};

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, EmulatorStore, InMemoryEmulatorHttpClient, ReplicationConfig,
    RequestObserver, SeedingPolicy, VirtualAccountConfig, VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, ItemReference, PartitionKey, SubStatusCode,
};
use azure_data_cosmos_driver::options::{DriverOptions, ExcludedRegions, OperationOptions, Region};
use azure_data_cosmos_driver::CosmosDriver;

use super::collect_response;
use super::host_recorder::HostRecorder;

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
const CENTRAL_URL: &str = "https://centralus.emulator.local";
const EAST_HOST: &str = "eastus.emulator.local";
const WEST_HOST: &str = "westus.emulator.local";
const CENTRAL_HOST: &str = "centralus.emulator.local";

/// Production refresh interval, kept in sync with
/// `LocationStateStore::BACKGROUND_REFRESH_INTERVAL` (300 s).
const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

/// Offlines West after the driver has selected it and emitted the request, but
/// before the emulator resolves the destination. This deterministically
/// exercises the selection/dispatch topology race.
#[derive(Debug, Default)]
struct OfflineWestOnFirstRequest {
    store: Mutex<Option<Arc<EmulatorStore>>>,
    armed: AtomicBool,
    fired: AtomicBool,
    hosts: Mutex<Vec<String>>,
}

impl OfflineWestOnFirstRequest {
    fn attach(&self, store: Arc<EmulatorStore>) {
        *self.store.lock().unwrap() = Some(store);
    }

    fn arm(&self) {
        self.hosts.lock().unwrap().clear();
        self.fired.store(false, Ordering::SeqCst);
        self.armed.store(true, Ordering::SeqCst);
    }

    fn hosts(&self) -> Vec<String> {
        self.hosts.lock().unwrap().clone()
    }
}

impl RequestObserver for OfflineWestOnFirstRequest {
    fn on_request(&self, request: &Request) {
        if !request.url().path().contains("/docs") {
            return;
        }
        let host = request.url().host_str().unwrap_or_default().to_string();
        self.hosts.lock().unwrap().push(host.clone());
        if host == WEST_HOST
            && self.armed.load(Ordering::SeqCst)
            && self
                .fired
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
        {
            self.store
                .lock()
                .unwrap()
                .as_ref()
                .expect("observer store attached")
                .set_region_offline("West US")
                .expect("race transition should offline West");
        }
    }
}

fn east() -> VirtualRegion {
    VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap())
}

fn west() -> VirtualRegion {
    VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap())
}

fn central() -> VirtualRegion {
    VirtualRegion::new("Central US", Url::parse(CENTRAL_URL).unwrap())
}

fn build_emulator(
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    observer: Arc<HostRecorder>,
) -> Arc<InMemoryEmulatorHttpClient> {
    build_emulator_with_consistency(regions, write_mode, ConsistencyLevel::Session, observer)
}

fn build_emulator_with_consistency(
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    consistency: ConsistencyLevel,
    observer: Arc<HostRecorder>,
) -> Arc<InMemoryEmulatorHttpClient> {
    build_emulator_with_replication(
        regions,
        write_mode,
        consistency,
        ReplicationConfig::immediate(),
        observer,
    )
}

fn build_emulator_with_replication(
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    consistency: ConsistencyLevel,
    replication: ReplicationConfig,
    observer: Arc<HostRecorder>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(regions)
        .unwrap()
        .with_write_mode(write_mode)
        .with_consistency(consistency)
        .with_replication_config(replication);

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

fn account_at(endpoint: &str) -> AccountReference {
    AccountReference::with_master_key(Url::parse(endpoint).unwrap(), "ZW11bGF0b3Ita2V5")
}

async fn build_driver(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    preferred: Vec<Region>,
) -> Arc<CosmosDriver> {
    build_driver_at(emulator, EAST_URL, preferred).await
}

async fn build_driver_at(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    account_endpoint: &str,
    preferred: Vec<Region>,
) -> Arc<CosmosDriver> {
    build_driver_at_with_options(
        emulator,
        account_endpoint,
        preferred,
        OperationOptions::default(),
    )
    .await
}

async fn build_driver_at_with_options(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    account_endpoint: &str,
    preferred: Vec<Region>,
    operation_options: OperationOptions,
) -> Arc<CosmosDriver> {
    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    runtime
        .create_driver(
            DriverOptions::builder(account_at(account_endpoint))
                .with_preferred_regions(preferred)
                .with_operation_options(operation_options)
                .build(),
        )
        .await
        .expect("driver should initialize")
}

async fn seed_item(driver: &CosmosDriver, item_id: &str) {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    let body = serde_json::json!({"id": item_id, "pk": "pk1", "value": 1}).to_string();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(body.into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("seeding write should succeed");
}

/// Reads a seeded item, returning the hosts the data-plane traffic landed on.
async fn read_and_capture_hosts(
    driver: &CosmosDriver,
    recorder: &Arc<HostRecorder>,
    item_id: &str,
) -> Vec<String> {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    recorder.clear();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .expect("read should succeed");
    recorder.data_plane_hosts()
}

/// Writes an item, returning the hosts the data-plane traffic landed on.
async fn write_and_capture_hosts(
    driver: &CosmosDriver,
    recorder: &Arc<HostRecorder>,
    item_id: &str,
) -> Vec<String> {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    recorder.clear();
    let body = serde_json::json!({"id": item_id, "pk": "pk1"}).to_string();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(body.into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("write should succeed");
    recorder.data_plane_hosts()
}

/// Lets the background account-refresh loop observe the new topology.
async fn advance_past_refresh() {
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
}

/// Reads the account payload the way a client would, straight off the emulator.
///
/// Takes the endpoint explicitly because an offlined region's endpoint does not
/// resolve — a test that offlines the region it reads through would fail at the
/// transport layer rather than observing the new topology.
async fn account_payload_from(
    emulator: &InMemoryEmulatorHttpClient,
    endpoint: &str,
) -> serde_json::Value {
    let req = Request::new(Url::parse(&format!("{endpoint}/")).unwrap(), Method::Get);
    let (status, _, body) = collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Ok, "account read should succeed");
    body
}

/// Reads the account payload through the East US endpoint.
async fn account_payload(emulator: &InMemoryEmulatorHttpClient) -> serde_json::Value {
    account_payload_from(emulator, EAST_URL).await
}

fn names(payload: &serde_json::Value, field: &str) -> Vec<String> {
    payload[field]
        .as_array()
        .unwrap_or_else(|| panic!("{field} should be an array"))
        .iter()
        .map(|loc| loc["name"].as_str().unwrap().to_string())
        .collect()
}

// --- Offline / online -------------------------------------------------------

/// An offlined region leaves **both** advertised lists.
///
/// Verified against a live multi-write account: `offlineRegion` dropped the
/// region from `readableLocations` and `writableLocations` simultaneously on
/// every endpoint. ARM keeps the region with `provisioningState: Offline`, but
/// that state is invisible on the data plane, so a client cannot distinguish it
/// from a removal by reading the payload alone.
#[tokio::test]
async fn offlined_region_is_dropped_from_both_location_lists() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Multi,
        recorder.clone(),
    );

    let before = account_payload(&emulator).await;
    assert_eq!(names(&before, "readableLocations").len(), 3);
    assert_eq!(names(&before, "writableLocations").len(), 3);

    emulator
        .store()
        .set_region_offline("Central US")
        .expect("offlining a non-write region should succeed");

    let after = account_payload(&emulator).await;
    assert!(
        !names(&after, "readableLocations").contains(&"Central US".to_string()),
        "an offlined region must not be advertised as readable; got {:?}",
        names(&after, "readableLocations")
    );
    assert!(
        !names(&after, "writableLocations").contains(&"Central US".to_string()),
        "an offlined region must not be advertised as writable; got {:?}",
        names(&after, "writableLocations")
    );
}

/// A request to an offlined region fails at the transport layer, not with
/// `403/1008`.
///
/// This is the distinction between offline and removal, and the reason offline
/// could not simply reuse `RegionStatus::Draining`. Verified against a live
/// account: `dig` for the offlined region's regional hostname returned nothing,
/// so a client opening a new connection fails to resolve the host. A *removed*
/// region, by contrast, keeps resolving and answers `403/1008`.
#[tokio::test]
async fn request_to_offlined_region_fails_at_the_transport_layer() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    emulator
        .store()
        .set_region_offline("West US")
        .expect("offlining a non-write region should succeed");

    let req = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll")).unwrap(),
        Method::Get,
    );
    let error = emulator
        .execute_request(&req)
        .await
        .expect_err("an offlined region's endpoint must not answer");

    let status = error.status();
    assert_eq!(
        status.sub_status(),
        Some(SubStatusCode::TRANSPORT_DNS_FAILED),
        "an offlined region must fail name resolution, not return 403/1008; got {error}"
    );
}

/// Bringing a region back online restores it to both lists.
///
/// Note this models the `onlineRegion` ARM operation, which on a live account is
/// gated behind a capability that is **off by default** — an attempt against a
/// normal account is rejected, and re-listing the region with an ordinary
/// topology update does not restore it either. The emulator models the
/// capability as present so recovery is testable.
#[tokio::test]
async fn onlining_a_region_restores_it() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let store = emulator.store();

    store.set_region_offline("West US").unwrap();
    store
        .set_region_online("West US")
        .expect("onlining should succeed");

    let payload = account_payload(&emulator).await;
    assert!(
        names(&payload, "readableLocations").contains(&"West US".to_string()),
        "an onlined region must be advertised again"
    );

    // The endpoint answers once more, rather than failing name resolution.
    let req = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll")).unwrap(),
        Method::Get,
    );
    let (status, _, _) = collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Ok);
}

/// Offlining the write region is permitted and performs a failover.
///
/// Unlike removal, which the emulator refuses for the write region, the service
/// accepts this: on the live single-write account the next region in priority
/// order was promoted and the offlined region was pushed to the highest priority
/// number, all without an error.
#[tokio::test]
async fn offlining_the_write_region_fails_over() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["East US".to_string()],
        "test setup: East US starts as the write region"
    );

    store
        .set_region_offline("East US")
        .expect("offlining the write region should fail over, not error");

    // Read through a region that is still online: the offlined one no longer
    // resolves, exactly as the live endpoint stops resolving.
    let payload = account_payload_from(&emulator, WEST_URL).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["West US".to_string()],
        "the next region in priority order must be promoted"
    );
    assert!(
        !names(&payload, "readableLocations").contains(&"East US".to_string()),
        "the offlined former write region must not be advertised"
    );
}

/// An offlined hub moves to the end of failover priority, so bringing it back
/// does not make it the next promotion candidate.
#[tokio::test]
async fn offline_failover_renumbers_priorities_before_the_next_failover() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.set_region_offline("East US").unwrap();
    assert_eq!(store.config().write_region_name(), "West US");

    store.set_region_online("East US").unwrap();
    store.set_region_offline("West US").unwrap();
    assert_eq!(
        store.config().write_region_name(),
        "Central US",
        "the restored former hub must remain behind the next surviving priority"
    );
}

/// Offlining the only remaining online region is refused.
///
/// Guard rather than observed behavior: an account with every region offline
/// could serve nothing and no public call could repair it.
#[tokio::test]
async fn offlining_the_last_online_region_is_refused() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.set_region_offline("West US").unwrap();
    let error = store
        .set_region_offline("East US")
        .expect_err("offlining the last online region must be refused");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
}

/// The driver stops routing to an offlined region once it refreshes topology,
/// and picks it up again when it comes back.
#[tokio::test(start_paused = true)]
async fn driver_stops_and_resumes_routing_across_offline_and_online() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    // East US owns writes, so prefer the satellites for reads.
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "offline-item").await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "offline-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == WEST_HOST),
        "test setup: reads should start on the top preferred region; observed {hosts:?}"
    );

    emulator.store().set_region_offline("West US").unwrap();
    advance_past_refresh().await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "offline-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == CENTRAL_HOST),
        "reads must move from the offlined top preference to the next preferred region; observed {hosts:?}"
    );

    emulator.store().set_region_online("West US").unwrap();
    advance_past_refresh().await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "offline-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == WEST_HOST),
        "reads must return to the top preferred region once it is online; observed {hosts:?}"
    );
}

/// Fresh drivers initialize correctly from both the offline and restored
/// account topology, without relying on any cache or background-refresh state
/// from a previous client instance.
#[tokio::test(start_paused = true)]
async fn cold_restart_routes_around_offline_region_then_returns_after_online() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let initial = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&initial, "cold-restart-item").await;

    emulator.store().set_region_offline("West US").unwrap();

    // A new runtime + driver has no prior topology cache. Initialization must
    // consume the already-offline account payload and choose Central directly.
    let cold_offline = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    let hosts = read_and_capture_hosts(&cold_offline, &recorder, "cold-restart-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == CENTRAL_HOST),
        "a cold driver must skip the absent preferred region and start on Central; observed {hosts:?}"
    );

    emulator.store().set_region_online("West US").unwrap();

    // A second cold driver must discover West immediately; no failback probe or
    // refresh inherited from the prior instance is available to help it.
    let cold_online = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    let hosts = read_and_capture_hosts(&cold_online, &recorder, "cold-restart-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == WEST_HOST),
        "a cold driver must immediately restore the top preference once advertised; observed {hosts:?}"
    );
}

// --- Failover priority ------------------------------------------------------

/// A priority change that does not touch position 0 produces no observable
/// change at all.
///
/// This is the single most surprising observation of the exercise, and it
/// contradicts the emulator spec's own prior claim that `failoverPriority`
/// "orders `readableLocations`". Swapping p1 and p2 on the live account changed
/// the ARM view while leaving `readableLocations` byte-identical — re-read on
/// fresh connections after a three-minute settle to rule out lag — and
/// interrupted no writes.
#[tokio::test]
async fn priority_change_below_position_zero_is_invisible() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);

    let before = account_payload(&emulator).await;

    emulator
        .store()
        .set_failover_priorities(&["East US", "Central US", "West US"])
        .expect("a complete priority assignment should be accepted");

    let after = account_payload(&emulator).await;
    assert_eq!(
        names(&before, "readableLocations"),
        names(&after, "readableLocations"),
        "reordering below position 0 must not change the advertised read order"
    );
    assert_eq!(
        names(&before, "writableLocations"),
        names(&after, "writableLocations"),
        "reordering below position 0 must not change write ownership"
    );
}

/// On a single-write account, promoting a region to position 0 **is** the manual
/// failover operation: it moves write ownership.
#[tokio::test]
async fn promotion_to_position_zero_moves_writes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);

    emulator
        .store()
        .set_failover_priorities(&["Central US", "East US", "West US"])
        .expect("promotion should succeed");

    let payload = account_payload(&emulator).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["Central US".to_string()],
        "promotion to position 0 must move write ownership"
    );
    assert_eq!(
        names(&payload, "readableLocations")[0],
        "Central US",
        "the write region is always advertised first"
    );
}

/// On a multi-write account the same promotion changes nothing about who may
/// write, because every advertised region is writable.
#[tokio::test]
async fn promotion_to_position_zero_does_not_gate_writes_under_multi_write() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Multi, recorder);

    emulator
        .store()
        .set_failover_priorities(&["Central US", "East US", "West US"])
        .expect("promotion should succeed");

    let payload = account_payload(&emulator).await;
    let writable = names(&payload, "writableLocations");
    assert_eq!(
        writable.len(),
        3,
        "every region stays writable under multi-write; got {writable:?}"
    );

    // Writes to a region that is not p0 still succeed.
    let mut req = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "mw-write", "pk": "pk1"}).to_string());
    req.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);
}

/// An incomplete priority assignment is refused, as the service refuses one.
#[tokio::test]
async fn incomplete_priority_assignment_is_refused() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);

    let error = emulator
        .store()
        .set_failover_priorities(&["East US", "West US"])
        .expect_err("a partial assignment must be refused");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    let duplicate = emulator
        .store()
        .set_failover_priorities(&["East US", "East US", "West US"])
        .expect_err("a duplicated region must be refused");
    assert_eq!(duplicate.status().status_code(), StatusCode::BadRequest);
}

/// A priority reorder must not renumber region IDs.
///
/// Session-token vector clocks embed region IDs, so renumbering would silently
/// invalidate tokens a client is still holding.
#[tokio::test]
async fn priority_reorder_preserves_region_ids() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();
    let config = store.config();

    let before: Vec<(String, u64)> = config
        .active_regions()
        .iter()
        .map(|r| (r.name().to_string(), r.region_id()))
        .collect();

    emulator
        .store()
        .set_failover_priorities(&["Central US", "West US", "East US"])
        .unwrap();

    let after: Vec<(String, u64)> = config
        .active_regions()
        .iter()
        .map(|r| (r.name().to_string(), r.region_id()))
        .collect();

    for (name, id) in &before {
        let found = after
            .iter()
            .find(|(other, _)| other == name)
            .unwrap_or_else(|| panic!("{name} should still be present"));
        assert_eq!(
            found.1, *id,
            "{name} must keep region ID {id} across a priority reorder"
        );
    }
}

/// Offlining the write region promotes by **failover priority**, not by
/// advertisement order.
///
/// The two differ once priorities have been reordered, and only priority is
/// correct: on the live account the region that took over was the one at the
/// next priority position.
#[tokio::test]
async fn offline_promotion_follows_priority_not_advertisement_order() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    // Advertisement order stays East, West, Central; priority now puts Central
    // ahead of West, so offlining the hub must promote Central.
    store
        .set_failover_priorities(&["East US", "Central US", "West US"])
        .unwrap();
    store.set_region_offline("East US").unwrap();

    let payload = account_payload_from(&emulator, WEST_URL).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["Central US".to_string()],
        "promotion must follow priority order, not the advertised list order"
    );
}

// --- In-flight failover -----------------------------------------------------

/// A single-write account transiently advertises **two** writable locations.
///
/// Verified against a live account and observed in 76 samples across all four
/// polled endpoints: during a manual failover `writableLocations` contained both
/// the outgoing and incoming write regions while `enableMultipleWriteLocations`
/// stayed `false`. The emulator previously could not represent this — it emitted
/// exactly one writable location under single-write — so a driver could never be
/// tested against a payload the service really produces.
#[tokio::test]
async fn in_flight_failover_advertises_both_write_regions() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .begin_failover("West US")
        .expect("beginning a failover should succeed");

    let payload = account_payload(&emulator).await;
    let writable = names(&payload, "writableLocations");
    assert_eq!(
        writable,
        vec!["West US".to_string(), "East US".to_string()],
        "an in-flight failover advertises the incoming region first, then the outgoing one"
    );
    assert_eq!(
        payload["enableMultipleWriteLocations"], false,
        "the account is still single-write during a failover"
    );

    store.complete_failover();

    let settled = account_payload(&emulator).await;
    assert_eq!(
        names(&settled, "writableLocations"),
        vec!["West US".to_string()],
        "completing the failover narrows the payload to the new write region"
    );
}

/// The demoted region rejects writes with `403/3` while the account read still
/// advertises it as writable.
///
/// This is the race the observation pinned down precisely: on the live account
/// the outgoing region began returning `403/3` roughly ten seconds *before* it
/// disappeared from `writableLocations`, so for that window the payload was
/// actively wrong. A client that trusts the payload without handling `403/3`
/// will fail.
#[tokio::test]
async fn demoted_region_rejects_writes_while_still_advertised() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    emulator
        .store()
        .begin_failover("West US")
        .expect("beginning a failover should succeed");

    // The payload still lists the outgoing region as writable...
    let payload = account_payload(&emulator).await;
    assert!(
        names(&payload, "writableLocations").contains(&"East US".to_string()),
        "test setup: the outgoing region is still advertised as writable"
    );

    // ...but it already refuses writes.
    let mut req = Request::new(
        Url::parse(&format!("{EAST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "racing", "pk": "pk1"}).to_string());
    req.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, headers, _) =
        collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(
        headers.get_optional_str(&super::SUBSTATUS),
        Some("3"),
        "the demoted region must reject writes with WriteForbidden even while advertised"
    );
}

/// The driver follows the write region across a completed failover.
#[tokio::test(start_paused = true)]
async fn writes_follow_the_new_write_region_after_failover() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&driver, "failover-seed").await;

    let hosts = write_and_capture_hosts(&driver, &recorder, "before-failover").await;
    assert!(
        hosts.iter().all(|h| h == EAST_HOST),
        "test setup: writes should start on the hub; observed {hosts:?}"
    );

    let store = emulator.store();
    store.begin_failover("West US").unwrap();
    store.complete_failover();
    advance_past_refresh().await;

    let hosts = write_and_capture_hosts(&driver, &recorder, "after-failover").await;
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "writes must follow the promoted region; observed {hosts:?}"
    );
}

/// Offlining the write region must not promote a **draining** region.
///
/// A draining region's endpoint already answers `403/1008`, so promoting it
/// would advertise a write region that rejects everything — and once it owns
/// writes, neither removal path can retire it.
#[tokio::test]
async fn offlining_the_write_region_will_not_promote_a_draining_region() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .begin_region_removal("West US")
        .expect("draining a non-write region should succeed");

    let error = store
        .set_region_offline("East US")
        .expect_err("promoting a draining region must be refused");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    // The account is untouched: East US still owns writes and still serves.
    let payload = account_payload(&emulator).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["East US".to_string()]
    );
}

/// Promoting a draining region to position 0 is refused, for the same reason.
#[tokio::test]
async fn priority_promotion_of_a_draining_region_is_refused() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.begin_region_removal("West US").unwrap();
    let error = store
        .set_failover_priorities(&["West US", "East US", "Central US"])
        .expect_err("promoting a draining region to p0 must be refused");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
}

/// A rejected priority change must not partially apply.
///
/// `priority_order` steers which region is promoted when the write region is
/// offlined, so a rejected call that still rewrote it would silently change a
/// later failover's outcome.
#[tokio::test]
async fn rejected_priority_change_leaves_priority_order_untouched() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    // Establish a known priority: Central ahead of West.
    store
        .set_failover_priorities(&["East US", "Central US", "West US"])
        .unwrap();

    // A rejected change that would have put West ahead of Central.
    store.set_region_offline("West US").unwrap();
    store
        .set_failover_priorities(&["West US", "Central US", "East US"])
        .expect_err("promoting an offline region must be refused");
    store.set_region_online("West US").unwrap();

    // If the rejected call had applied, offlining the hub would promote West.
    store.set_region_offline("East US").unwrap();
    let payload = account_payload_from(&emulator, WEST_URL).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["Central US".to_string()],
        "the rejected priority change must not have taken effect"
    );
}

/// A priority-driven write-region move ends any in-flight failover.
///
/// Otherwise the payload would advertise the outgoing region of an *unrelated*
/// failover alongside the newly promoted write region — two regions that were
/// never party to the same transition, which the service never emits.
#[tokio::test]
async fn priority_promotion_clears_an_in_flight_failover() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.begin_failover("West US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["West US".to_string(), "East US".to_string()],
        "test setup: a failover is in flight"
    );

    store
        .set_failover_priorities(&["Central US", "East US", "West US"])
        .unwrap();

    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["Central US".to_string()],
        "a priority-driven move must settle, not inherit the previous failover"
    );
}

/// The **leading edge** of a failover: both regions advertised, the outgoing one
/// still accepting writes.
///
/// This is a third write-region slot the service maintains
/// (`Topology.NextWriteRegion`, folded into `writableLocations` by
/// `DatabaseAccountHandler.GetLocationsFromTopology` alongside `WriteRegion` and
/// `PreviousWriteRegion`). The captured failover shows the window precisely:
/// both regions were advertised at 19:20:18, the outgoing region still returned
/// `201` at 19:20:23, and only at 19:20:24 did it start returning `403/3`.
///
/// Without this the emulator jumps straight to the trailing edge, so a client
/// that must tolerate "advertised as writable *and* actually writable, but about
/// to move" could never be tested.
#[tokio::test]
async fn announced_failover_advertises_both_but_keeps_writes_on_the_outgoing_region() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .announce_failover("West US")
        .expect("announcing a failover should succeed");

    let payload = account_payload(&emulator).await;
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["East US".to_string(), "West US".to_string()],
        "both the current and announced write regions are advertised"
    );
    assert_eq!(
        payload["enableMultipleWriteLocations"], false,
        "the account is still single-write during an announced failover"
    );

    // The outgoing region still owns writes -- this is what distinguishes the
    // announce phase from `begin_failover`.
    let mut req = Request::new(
        Url::parse(&format!("{EAST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "announced", "pk": "pk1"}).to_string());
    req.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Created,
        "the outgoing region still accepts writes until ownership actually moves"
    );
}

/// The three failover phases produce three distinct payloads, and the announced
/// region is never listed twice.
#[tokio::test]
async fn failover_phases_progress_from_announce_to_switch_to_settled() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.announce_failover("West US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["East US".to_string(), "West US".to_string()],
        "announce: current region first, incoming second"
    );

    store.begin_failover("West US").unwrap();
    let switched = names(&account_payload(&emulator).await, "writableLocations");
    assert_eq!(
        switched,
        vec!["West US".to_string(), "East US".to_string()],
        "switch: the new write region leads, the outgoing one trails"
    );
    assert_eq!(
        switched.len(),
        2,
        "the incoming region must not be listed twice once it owns writes"
    );

    store.complete_failover();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["West US".to_string()],
        "settled: only the new write region remains"
    );
}

/// Announcing a failover to an offline or draining region is refused, matching
/// the guards on the other write-ownership paths.
#[tokio::test]
async fn announcing_a_failover_to_an_unusable_region_is_refused() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.set_region_offline("West US").unwrap();
    let offline = store
        .announce_failover("West US")
        .expect_err("announcing to an offline region must be refused");
    assert_eq!(offline.status().status_code(), StatusCode::BadRequest);

    store.begin_region_removal("Central US").unwrap();
    let draining = store
        .announce_failover("Central US")
        .expect_err("announcing to a draining region must be refused");
    assert_eq!(draining.status().status_code(), StatusCode::BadRequest);
}

/// Failing over to an offline region is refused: it is not advertised at all, so
/// promoting it would produce an account with no reachable write region.
#[tokio::test]
async fn failover_to_an_offline_region_is_refused() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.set_region_offline("West US").unwrap();
    let error = store
        .begin_failover("West US")
        .expect_err("failing over to an offline region must be refused");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    let promotion = store
        .set_failover_priorities(&["West US", "East US", "Central US"])
        .expect_err("promoting an offline region to p0 must be refused");
    assert_eq!(promotion.status().status_code(), StatusCode::BadRequest);
}

// --- Service implementation parity -----------------------------------------

/// Strong consistency is a second, independent gate on multi-write routing.
///
/// The service's `DatabaseAccountHandler.GetDatabaseAccountAsync` emits
/// `enableMultipleWriteLocations: true` from account configuration but sets its
/// internal `allowMultipleWriteLocations` only when consistency is not Strong.
/// The resulting payload looks contradictory on purpose: the flag is true,
/// while `writableLocations` contains only the hub.
#[tokio::test]
async fn strong_consistency_gates_multi_write_locations_without_clearing_the_flag() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_consistency(
        vec![east(), west(), central()],
        WriteMode::Multi,
        ConsistencyLevel::Strong,
        recorder,
    );

    let payload = account_payload(&emulator).await;
    assert_eq!(payload["enableMultipleWriteLocations"], true);
    assert_eq!(
        names(&payload, "writableLocations"),
        vec!["East US".to_string()],
        "Strong consistency must keep writableLocations hub-only even when the multi-write flag is true"
    );
    assert_eq!(
        names(&payload, "readableLocations").len(),
        3,
        "Strong consistency does not hide readable regions"
    );
}

/// Strong + multi-write is hub-only in enforcement as well as advertisement.
#[tokio::test]
async fn strong_consistency_rejects_satellite_writes_under_multi_write_mode() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_consistency(
        vec![east(), west()],
        WriteMode::Multi,
        ConsistencyLevel::Strong,
        recorder,
    );

    let mut req = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "strong-west", "pk": "pk1"}).to_string());
    req.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );

    let (status, headers, _) =
        collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(headers.get_optional_str(&super::SUBSTATUS), Some("3"));
}

/// Failover transition slots cannot bypass Strong's hub-only gateway gate.
#[tokio::test]
async fn strong_multi_write_stays_hub_only_during_failover_transitions() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_consistency(
        vec![east(), west()],
        WriteMode::Multi,
        ConsistencyLevel::Strong,
        recorder,
    );
    let store = emulator.store();

    store.announce_failover("West US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["East US".to_string()],
        "NextWriteRegion must not bypass Strong's hub-only gate"
    );

    store.begin_failover("West US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["West US".to_string()],
        "PreviousWriteRegion must not bypass Strong's hub-only gate"
    );
}

/// A revoked satellite remains advertised writable but rejects writes.
///
/// This mirrors the split between the service layers: RoutingGateway constructs
/// account locations from topology without consulting
/// `WriteStatusRevokedSatelliteRegions`, while backend write enforcement checks
/// that set. The payload is knowingly stale in the same spirit as a demoted
/// write region during failover.
#[tokio::test]
async fn revoked_satellite_stays_advertised_but_rejects_writes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    emulator
        .store()
        .revoke_region_write("West US")
        .expect("revoking a satellite should succeed");

    let payload = account_payload(&emulator).await;
    assert!(
        names(&payload, "readableLocations").contains(&"West US".to_string()),
        "revocation must not remove the satellite from readable locations"
    );
    assert!(
        names(&payload, "writableLocations").contains(&"West US".to_string()),
        "RoutingGateway does not filter revoked satellites from writable locations"
    );

    let mut write = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "revoked", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, headers, _) =
        collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(headers.get_optional_str(&super::SUBSTATUS), Some("3"));

    let read = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll")).unwrap(),
        Method::Get,
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "write revocation must not revoke reads"
    );
}

/// Restoring local write status makes the satellite writable again.
#[tokio::test]
async fn restored_satellite_accepts_writes_again() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let store = emulator.store();
    store.revoke_region_write("West US").unwrap();
    store.restore_region_write("West US").unwrap();

    let mut req = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "restored", "pk": "pk1"}).to_string());
    req.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&req).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);
}

/// The hub cannot be placed in the satellite write-revocation set, and the
/// feature is meaningless on a single-write account.
#[tokio::test]
async fn write_revocation_rejects_the_hub_and_single_write_accounts() {
    let recorder = HostRecorder::new();
    let multi = build_emulator(vec![east(), west()], WriteMode::Multi, recorder.clone());
    let hub = multi
        .store()
        .revoke_region_write("East US")
        .expect_err("the topology invariant forbids revoking the hub");
    assert_eq!(hub.status().status_code(), StatusCode::BadRequest);

    let single = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let satellite = single
        .store()
        .revoke_region_write("West US")
        .expect_err("satellite write revocation requires multi-write mode");
    assert_eq!(satellite.status().status_code(), StatusCode::BadRequest);
}

/// A revoked satellite cannot be promoted to hub through any topology API.
#[tokio::test]
async fn revoked_satellite_cannot_be_promoted_to_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west(), central()], WriteMode::Multi, recorder);
    let store = emulator.store();
    store.revoke_region_write("West US").unwrap();

    assert!(store.set_write_region("West US").is_err());
    assert!(store
        .set_failover_priorities(&["West US", "East US", "Central US"])
        .is_err());
    assert!(store.announce_failover("West US").is_err());
    assert!(store.begin_failover("West US").is_err());
}

/// A region whose add is in progress is hidden from both client-visible lists
/// until buildout completes.
///
/// Cosmos Fabric enables `EnableSkipInProgressRegionInGetDatabaseAccount` by
/// default. RoutingGateway then removes any region with
/// `IsAddRegionInProgress` from both location sets. This policy differs from
/// `SeedingPolicy::Delayed`, which intentionally models the older/alternate
/// behavior where the unseeded region is already advertised.
#[tokio::test(start_paused = true)]
async fn in_progress_region_is_hidden_until_ready() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder);
    emulator
        .store()
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(60)),
        )
        .expect("starting a hidden region add should succeed");

    let during = account_payload(&emulator).await;
    assert_eq!(
        names(&during, "readableLocations"),
        vec!["East US".to_string()]
    );
    assert_eq!(
        names(&during, "writableLocations"),
        vec!["East US".to_string()]
    );

    let mut write = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "building", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, headers, _) =
        collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Forbidden,
        "a still-building region must not accept writes"
    );
    assert_eq!(headers.get_optional_str(&super::SUBSTATUS), Some("3"));
    assert!(
        emulator.store().set_write_region("West US").is_err(),
        "a still-building region cannot be promoted"
    );

    tokio::time::sleep(Duration::from_secs(61)).await;

    let ready = account_payload(&emulator).await;
    assert!(
        names(&ready, "readableLocations").contains(&"West US".to_string()),
        "the region must become visible after buildout completes"
    );
    assert!(
        names(&ready, "writableLocations").contains(&"West US".to_string()),
        "a ready region on a multi-write account must become writable"
    );

    let mut write = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "ready", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Created,
        "the region accepts writes once buildout completes"
    );
}

/// A stale timer from an older add incarnation cannot expose a newer re-add.
#[tokio::test(start_paused = true)]
async fn old_hidden_add_task_cannot_expose_a_readded_region() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder);
    let store = emulator.store();

    store
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(60)),
        )
        .unwrap();
    store.remove_region("West US").unwrap();
    store
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(120)),
        )
        .unwrap();

    // The first incarnation's timer has fired; the second has not.
    tokio::time::sleep(Duration::from_secs(61)).await;
    assert!(
        !names(&account_payload(&emulator).await, "readableLocations")
            .contains(&"West US".to_string()),
        "the old task must not expose the new incarnation"
    );

    tokio::time::sleep(Duration::from_secs(60)).await;
    assert!(
        names(&account_payload(&emulator).await, "readableLocations")
            .contains(&"West US".to_string()),
        "the current incarnation becomes visible on its own timer"
    );
}

/// Delayed catch-up resolves the current hub and merges, so removing the
/// original seed source cannot overwrite writes made during buildout.
#[tokio::test(start_paused = true)]
async fn hidden_add_uses_current_hub_without_losing_newer_writes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), central()], WriteMode::Multi, recorder);
    let store = emulator.store();
    store
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(60)),
        )
        .unwrap();

    store
        .set_write_region("Central US")
        .expect("move the hub away from the original source");
    store
        .remove_region("East US")
        .expect("the original source can now be removed");

    let mut write = Request::new(
        Url::parse(&format!("{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "during-buildout", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);

    tokio::time::sleep(Duration::from_secs(61)).await;

    let mut read = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/during-buildout"
        ))
        .unwrap(),
        Method::Get,
    );
    read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "catch-up must not erase a write replicated during buildout"
    );
}

/// Replications already queued before enrollment are replayed into the joining
/// region, including deletion tombstones.
#[tokio::test(start_paused = true)]
async fn hidden_add_replays_preexisting_delayed_mutations() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_replication(
        vec![east(), central()],
        WriteMode::Multi,
        ConsistencyLevel::Session,
        ReplicationConfig::fixed(Duration::from_secs(120)),
        recorder,
    );

    // Create in Central and let it reach East so both existing replicas start
    // with the item.
    let mut create = Request::new(
        Url::parse(&format!("{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    create.set_body(serde_json::json!({"id": "pending-delete", "pk": "pk1"}).to_string());
    create.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&create).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);
    tokio::time::sleep(Duration::from_secs(121)).await;

    // Delete in Central. Its replication to the East hub is now queued but has
    // not applied when West snapshots East for enrollment.
    let mut delete = Request::new(
        Url::parse(&format!(
            "{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs/pending-delete"
        ))
        .unwrap(),
        Method::Delete,
    );
    delete.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&delete).await.unwrap()).await;
    assert_eq!(status, StatusCode::NoContent);

    emulator
        .store()
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(60)),
        )
        .unwrap();
    tokio::time::sleep(Duration::from_secs(61)).await;

    let mut read = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/pending-delete"
        ))
        .unwrap(),
        Method::Get,
    );
    read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::NotFound,
        "the joining region must replay the queued delete before becoming visible"
    );
}

/// The legacy delayed policy remains distinct: it advertises the region before
/// catch-up, preserving existing tests for the alternate service behavior.
#[tokio::test(start_paused = true)]
async fn delayed_region_remains_advertised_during_buildout() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder);
    emulator
        .store()
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_secs(60)))
        .expect("starting a delayed region add should succeed");

    let during = account_payload(&emulator).await;
    assert!(
        names(&during, "readableLocations").contains(&"West US".to_string()),
        "Delayed intentionally advertises before catch-up"
    );
}

/// A real point-write handler must hold catch-up behind replication registration.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn delayed_catch_up_cannot_overwrite_handler_write_before_registration() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder);
    let store = emulator.store();

    let mut seed = Request::new(
        Url::parse(&format!("{EAST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    seed.set_body(serde_json::json!({"id": "catch-up-sentinel", "pk": "pk1"}).to_string());
    seed.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&seed).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);

    let (registration_tx, registration_rx) = tokio::sync::oneshot::channel();
    let registration_tx = Arc::new(Mutex::new(Some(registration_tx)));
    let release_registration = Arc::new((Mutex::new(false), Condvar::new()));
    let registration_tx_clone = Arc::clone(&registration_tx);
    let release_registration_clone = Arc::clone(&release_registration);
    store.set_before_replication_registration_hook_for_tests(Some(Arc::new(move || {
        if let Some(tx) = registration_tx_clone.lock().unwrap().take() {
            tx.send(()).expect("test must wait for registration hook");
            let (released, wake) = &*release_registration_clone;
            let mut released = released.lock().unwrap();
            while !*released {
                released = wake.wait(released).unwrap();
            }
        }
    })));
    drop(registration_tx);

    struct RegistrationRelease(Arc<(Mutex<bool>, Condvar)>);

    impl RegistrationRelease {
        fn release(&self) {
            let (released, wake) = &*self.0;
            *released.lock().unwrap() = true;
            wake.notify_all();
        }
    }

    impl Drop for RegistrationRelease {
        fn drop(&mut self) {
            self.release();
        }
    }

    let registration_release = RegistrationRelease(Arc::clone(&release_registration));
    let catch_up_started = std::time::Instant::now();
    store
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_secs(1)))
        .unwrap();

    let mut create = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    create.set_body(serde_json::json!({"id": "handler-race", "pk": "pk1"}).to_string());
    create.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );

    let emulator_clone = Arc::clone(&emulator);
    let write = tokio::spawn(async move {
        let response = emulator_clone.execute_request(&create).await.unwrap();
        collect_response(response).await.0
    });
    tokio::time::timeout(Duration::from_secs(5), registration_rx)
        .await
        .expect("handler must reach replication registration before timeout")
        .expect("handler must reach replication registration");
    assert!(
        catch_up_started.elapsed() < Duration::from_secs(1),
        "handler must commit before delayed catch-up starts"
    );

    tokio::time::sleep(Duration::from_millis(1_100)).await;
    registration_release.release();

    assert_eq!(write.await.unwrap(), StatusCode::Created);
    store.set_before_replication_registration_hook_for_tests(None);

    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let mut read = Request::new(
                Url::parse(&format!(
                    "{WEST_URL}/dbs/testdb/colls/testcoll/docs/catch-up-sentinel"
                ))
                .unwrap(),
                Method::Get,
            );
            read.headers_mut().insert(
                super::PARTITION_KEY.clone(),
                azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
            );
            let (status, _, _) =
                collect_response(emulator.execute_request(&read).await.unwrap()).await;
            if status == StatusCode::Ok {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("delayed catch-up must complete");

    let mut read = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/handler-race"
        ))
        .unwrap(),
        Method::Get,
    );
    read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "delayed catch-up must not erase a local write before registration"
    );
}

/// Independent handlers share the replication barrier through registration.
#[cfg(not(feature = "preview_dtx"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn replication_registration_preserves_concurrent_handler_writes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let store = emulator.store();

    let registrations = Arc::new(AtomicUsize::new(0));
    let (both_registered_tx, both_registered_rx) = tokio::sync::oneshot::channel();
    let both_registered_tx = Arc::new(Mutex::new(Some(both_registered_tx)));
    let release_registration = Arc::new((Mutex::new(false), Condvar::new()));
    let registrations_clone = Arc::clone(&registrations);
    let both_registered_tx_clone = Arc::clone(&both_registered_tx);
    let release_registration_clone = Arc::clone(&release_registration);
    store.set_before_replication_registration_hook_for_tests(Some(Arc::new(move || {
        if registrations_clone.fetch_add(1, Ordering::SeqCst) + 1 == 2 {
            if let Some(tx) = both_registered_tx_clone.lock().unwrap().take() {
                let _ = tx.send(());
            }
        }
        let (released, wake) = &*release_registration_clone;
        let mut released = released.lock().unwrap();
        while !*released {
            released = wake.wait(released).unwrap();
        }
    })));
    drop(both_registered_tx);

    struct RegistrationRelease(Arc<(Mutex<bool>, Condvar)>);

    impl RegistrationRelease {
        fn release(&self) {
            let (released, wake) = &*self.0;
            *released.lock().unwrap() = true;
            wake.notify_all();
        }
    }

    impl Drop for RegistrationRelease {
        fn drop(&mut self) {
            self.release();
        }
    }

    let registration_release = RegistrationRelease(release_registration);
    let write = |region_url: &str, id: &str| {
        let mut request = Request::new(
            Url::parse(&format!("{region_url}/dbs/testdb/colls/testcoll/docs")).unwrap(),
            Method::Post,
        );
        request.set_body(serde_json::json!({"id": id, "pk": id}).to_string());
        request.headers_mut().insert(
            super::PARTITION_KEY.clone(),
            azure_core::http::headers::HeaderValue::from(format!("[\"{id}\"]")),
        );
        request
    };

    let east_write = {
        let emulator = Arc::clone(&emulator);
        let request = write(EAST_URL, "east-concurrent");
        tokio::spawn(async move {
            collect_response(emulator.execute_request(&request).await.unwrap())
                .await
                .0
        })
    };
    let west_write = {
        let emulator = Arc::clone(&emulator);
        let request = write(WEST_URL, "west-concurrent");
        tokio::spawn(async move {
            collect_response(emulator.execute_request(&request).await.unwrap())
                .await
                .0
        })
    };

    tokio::time::timeout(Duration::from_secs(5), both_registered_rx)
        .await
        .expect("independent handlers must register replication concurrently")
        .expect("registration hook must remain installed");
    registration_release.release();

    assert_eq!(east_write.await.unwrap(), StatusCode::Created);
    assert_eq!(west_write.await.unwrap(), StatusCode::Created);
    assert_eq!(registrations.load(Ordering::SeqCst), 2);
    store.set_before_replication_registration_hook_for_tests(None);
}

/// A mutation already pending at enrollment stays out of a `Delayed` region
/// until its catch-up timer fires.
#[tokio::test(start_paused = true)]
async fn delayed_region_defers_preexisting_pending_replay_until_catch_up() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_replication(
        vec![east(), central()],
        WriteMode::Multi,
        ConsistencyLevel::Session,
        ReplicationConfig::fixed(Duration::from_secs(120)),
        recorder,
    );

    let mut create = Request::new(
        Url::parse(&format!("{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    create.set_body(serde_json::json!({"id": "delayed-pending", "pk": "pk1"}).to_string());
    create.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&create).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_secs(60)))
        .unwrap();

    let mut read = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/delayed-pending"
        ))
        .unwrap(),
        Method::Get,
    );
    read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::NotFound,
        "pending enrollment replay must not populate Delayed before its timer"
    );

    tokio::time::sleep(Duration::from_secs(61)).await;
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "pending enrollment replay must apply after delayed catch-up"
    );
}

/// A delete committed during delayed buildout supersedes a create that was
/// already pending at enrollment; catch-up must not resurrect the document.
#[tokio::test(start_paused = true)]
async fn delayed_region_journal_preserves_delete_after_pending_create() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_replication(
        vec![east(), central()],
        WriteMode::Multi,
        ConsistencyLevel::Session,
        ReplicationConfig::fixed(Duration::from_secs(120)),
        recorder,
    );

    let mut create = Request::new(
        Url::parse(&format!("{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    create.set_body(serde_json::json!({"id": "journal-delete", "pk": "pk1"}).to_string());
    create.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&create).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_secs(60)))
        .unwrap();

    let mut delete = Request::new(
        Url::parse(&format!(
            "{CENTRAL_URL}/dbs/testdb/colls/testcoll/docs/journal-delete"
        ))
        .unwrap(),
        Method::Delete,
    );
    delete.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&delete).await.unwrap()).await;
    assert_eq!(status, StatusCode::NoContent);

    tokio::time::sleep(Duration::from_secs(61)).await;
    let mut read = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/journal-delete"
        ))
        .unwrap(),
        Method::Get,
    );
    read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&read).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::NotFound,
        "the buildout journal must replay create then delete without resurrection"
    );
}

/// Offlining and onlining a failover participant must not resurrect an old
/// NextWriteRegion or PreviousWriteRegion slot.
#[tokio::test]
async fn offline_online_cancels_stale_failover_slots() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store.begin_failover("West US").unwrap();
    store.set_region_offline("East US").unwrap();
    store.set_region_online("East US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["West US".to_string()],
        "onlining must not resurrect PreviousWriteRegion"
    );

    store.announce_failover("East US").unwrap();
    store.set_region_offline("East US").unwrap();
    store.set_region_online("East US").unwrap();
    assert_eq!(
        names(&account_payload(&emulator).await, "writableLocations"),
        vec!["West US".to_string()],
        "onlining must not resurrect NextWriteRegion"
    );
}

// --- CosmosDriver end-to-end coverage --------------------------------------

/// The driver honors Strong's hub-only writable list even when the account flag
/// says multi-write and West is the first preferred region.
#[tokio::test(start_paused = true)]
async fn driver_routes_strong_multi_write_to_the_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator_with_consistency(
        vec![east(), west()],
        WriteMode::Multi,
        ConsistencyLevel::Strong,
        recorder.clone(),
    );
    // Bootstrap through West (not the expected destination) so the test cannot
    // pass via an implementation that always writes to the account endpoint.
    let driver = build_driver_at(&emulator, WEST_URL, vec![Region::WEST_US, Region::EAST_US]).await;

    let hosts = write_and_capture_hosts(&driver, &recorder, "strong-driver-write").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "Strong must route writes to the hub despite West being preferred; observed {hosts:?}"
    );
}

/// A revoked preferred satellite returns `403/3`; the driver refreshes and
/// retries the write against another writable region.
#[tokio::test(start_paused = true)]
async fn driver_recovers_from_revoked_preferred_satellite() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;

    emulator.store().revoke_region_write("West US").unwrap();
    let hosts = write_and_capture_hosts(&driver, &recorder, "revoked-driver-write").await;

    assert!(
        hosts.iter().any(|host| host == WEST_HOST),
        "the first attempt should exercise the revoked preferred satellite; observed {hosts:?}"
    );
    assert!(
        hosts.iter().any(|host| host == EAST_HOST),
        "the driver must recover by retrying the healthy hub; observed {hosts:?}"
    );
}

/// The driver ignores a hidden in-progress region through background refresh,
/// then adopts it after buildout completes and a later refresh observes it.
#[tokio::test(start_paused = true)]
async fn driver_adopts_hidden_region_only_after_it_is_ready() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "hidden-driver-item").await;

    emulator
        .store()
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(1_800)),
        )
        .unwrap();

    // Refresh while the add is still in progress: West remains absent.
    advance_past_refresh().await;
    let hosts = read_and_capture_hosts(&driver, &recorder, "hidden-driver-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "the driver must not route to a hidden region; observed {hosts:?}"
    );

    tokio::time::sleep(Duration::from_secs(1_201)).await;
    advance_past_refresh().await;
    let hosts = read_and_capture_hosts(&driver, &recorder, "hidden-driver-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == WEST_HOST),
        "the driver should adopt its top preferred region once ready; observed {hosts:?}"
    );
}

/// Fresh drivers initialized during both failover transition phases recover
/// from the stale writable entry advertised by the account payload.
#[tokio::test(start_paused = true)]
async fn cold_start_during_failover_transition_recovers_writes() {
    // NextWriteRegion phase: East still owns writes, but West is advertised.
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let initial = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&initial, "cold-transition-seed").await;
    emulator.store().announce_failover("West US").unwrap();

    let announced = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    let hosts = write_and_capture_hosts(&announced, &recorder, "cold-announced-write").await;
    assert!(
        hosts.iter().any(|host| host == WEST_HOST)
            && hosts.iter().any(|host| host == EAST_HOST),
        "cold announce-phase client must retry from advertised West to actual owner East; observed {hosts:?}"
    );

    // PreviousWriteRegion phase: West now owns writes, but East remains
    // advertised until the transition settles.
    emulator.store().begin_failover("West US").unwrap();
    let switched = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    let hosts = write_and_capture_hosts(&switched, &recorder, "cold-switched-write").await;
    assert!(
        hosts.iter().any(|host| host == EAST_HOST)
            && hosts.iter().any(|host| host == WEST_HOST),
        "cold switch-phase client must retry from stale East to actual owner West; observed {hosts:?}"
    );
}

/// A fresh driver sees the revoked satellite in account metadata, attempts it,
/// and recovers through `403/3` without relying on a warm client's marks.
#[tokio::test(start_paused = true)]
async fn cold_start_with_revoked_satellite_recovers_write() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder.clone());
    emulator.store().revoke_region_write("West US").unwrap();

    let cold = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    let hosts = write_and_capture_hosts(&cold, &recorder, "cold-revoked-write").await;
    assert!(
        hosts.iter().any(|host| host == WEST_HOST) && hosts.iter().any(|host| host == EAST_HOST),
        "cold revoked-region client must attempt West then recover East; observed {hosts:?}"
    );
}

/// A fresh driver initialized during hidden buildout starts on East and later
/// adopts West after readiness and its own background refresh.
#[tokio::test(start_paused = true)]
async fn cold_start_during_hidden_buildout_adopts_region_when_ready() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder.clone());
    let initial = build_driver(&emulator, vec![Region::EAST_US]).await;
    seed_item(&initial, "cold-hidden-item").await;
    emulator
        .store()
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(1_800)),
        )
        .unwrap();

    let cold = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    let hosts = read_and_capture_hosts(&cold, &recorder, "cold-hidden-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "cold client must ignore hidden West; observed {hosts:?}"
    );

    tokio::time::sleep(Duration::from_secs(1_801)).await;
    advance_past_refresh().await;
    let hosts = read_and_capture_hosts(&cold, &recorder, "cold-hidden-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == WEST_HOST),
        "cold client should adopt West after readiness; observed {hosts:?}"
    );
}

/// Multi-write reads and writes leave an offlined preferred region and return
/// after it is online and observed by background refresh.
#[tokio::test(start_paused = true)]
async fn multi_write_traffic_leaves_offline_region_and_returns_after_online() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Multi,
        recorder.clone(),
    );
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "multi-offline-item").await;

    emulator.store().set_region_offline("West US").unwrap();
    advance_past_refresh().await;
    let read_hosts = read_and_capture_hosts(&driver, &recorder, "multi-offline-item").await;
    let write_hosts = write_and_capture_hosts(&driver, &recorder, "multi-offline-write").await;
    assert!(
        !read_hosts.is_empty() && read_hosts.iter().all(|host| host == CENTRAL_HOST),
        "multi-write reads must move to Central; observed {read_hosts:?}"
    );
    assert!(
        !write_hosts.is_empty() && write_hosts.iter().all(|host| host == CENTRAL_HOST),
        "multi-write writes must move to Central; observed {write_hosts:?}"
    );

    emulator.store().set_region_online("West US").unwrap();
    advance_past_refresh().await;
    let read_hosts = read_and_capture_hosts(&driver, &recorder, "multi-offline-item").await;
    let write_hosts = write_and_capture_hosts(&driver, &recorder, "multi-online-write").await;
    assert!(
        !read_hosts.is_empty() && read_hosts.iter().all(|host| host == WEST_HOST),
        "multi-write reads must return to West; observed {read_hosts:?}"
    );
    assert!(
        !write_hosts.is_empty() && write_hosts.iter().all(|host| host == WEST_HOST),
        "multi-write writes must return to West; observed {write_hosts:?}"
    );
}

/// If West is offline and the next preference (Central) is excluded for this
/// operation, routing must skip both and use nonpreferred East.
#[tokio::test(start_paused = true)]
async fn offline_region_plus_exclusion_uses_nonpreferred_region() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "offline-excluded-item").await;
    emulator.store().set_region_offline("West US").unwrap();
    advance_past_refresh().await;

    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    let item = ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "offline-excluded-item",
    );
    recorder.clear();
    let mut options = OperationOptions::default();
    options.excluded_regions = Some(ExcludedRegions::from_iter([Region::CENTRAL_US]));
    driver
        .execute_operation(CosmosOperation::read_item(item), options)
        .await
        .expect("read must fall back to nonpreferred East");
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "routing must skip offline West and excluded Central; observed {hosts:?}"
    );
}

/// Warm and cold drivers with different cache histories both make progress
/// while West is offline, then independently converge back to West.
#[tokio::test(start_paused = true)]
async fn warm_and_cold_clients_with_divergent_caches_converge() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let mut warm_options = OperationOptions::default();
    warm_options.endpoint_unavailability_ttl = Some(Duration::from_millis(1));
    let warm = build_driver_at_with_options(
        &emulator,
        EAST_URL,
        vec![Region::WEST_US, Region::CENTRAL_US],
        warm_options,
    )
    .await;
    seed_item(&warm, "divergent-clients-item").await;
    emulator.store().set_region_offline("West US").unwrap();

    let cold = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    let cold_hosts = read_and_capture_hosts(&cold, &recorder, "divergent-clients-item").await;
    let warm_hosts = read_and_capture_hosts(&warm, &recorder, "divergent-clients-item").await;
    assert!(
        !cold_hosts.is_empty() && cold_hosts.iter().all(|host| host == CENTRAL_HOST),
        "cold client should initialize on Central; observed {cold_hosts:?}"
    );
    assert!(
        warm_hosts.first().map(String::as_str) == Some(WEST_HOST)
            && warm_hosts.iter().any(|host| host == CENTRAL_HOST),
        "warm stale client must attempt West then recover to Central; observed {warm_hosts:?}"
    );
    assert!(
        warm.is_endpoint_host_marked_unavailable_for_testing(WEST_HOST),
        "the failed West attempt must create an account-level unavailability mark"
    );

    emulator.store().set_region_online("West US").unwrap();
    advance_past_refresh().await;
    // The endpoint cooldown uses std::time::Instant, not Tokio time.
    std::thread::sleep(Duration::from_millis(5));
    warm.run_endpoint_probe_once_for_testing().await;
    assert!(
        !warm.is_endpoint_host_marked_unavailable_for_testing(WEST_HOST),
        "a successful probe must clear the warm client's West mark"
    );

    let cold_hosts = read_and_capture_hosts(&cold, &recorder, "divergent-clients-item").await;
    let warm_hosts = read_and_capture_hosts(&warm, &recorder, "divergent-clients-item").await;
    assert!(
        !cold_hosts.is_empty() && cold_hosts.iter().all(|host| host == WEST_HOST),
        "cold client should eventually fail back to West; observed {cold_hosts:?}"
    );
    assert!(
        !warm_hosts.is_empty() && warm_hosts.iter().all(|host| host == WEST_HOST),
        "warm client should eventually fail back to West independently; observed {warm_hosts:?}"
    );
}

/// A topology transition after endpoint selection but before emulator dispatch
/// produces a failed West attempt and a successful Central retry.
#[tokio::test(start_paused = true)]
async fn operation_racing_offline_transition_retries_next_region() {
    let observer = Arc::new(OfflineWestOnFirstRequest::default());
    let config = VirtualAccountConfig::new(vec![east(), west(), central()])
        .unwrap()
        .with_write_mode(WriteMode::Single)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(observer.clone()));
    let store = emulator.store();
    observer.attach(store.clone());
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
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "race-offline-item").await;

    observer.arm();
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "race-offline-item");
    driver
        .execute_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .expect("race must recover on Central");
    let hosts = observer.hosts();
    assert_eq!(
        hosts.first().map(String::as_str),
        Some(WEST_HOST),
        "endpoint selection should occur before the observer offlines West"
    );
    assert!(
        hosts.iter().any(|host| host == CENTRAL_HOST),
        "request must retry Central after West becomes offline; observed {hosts:?}"
    );
}

/// A warm multi-write driver leaves a revoked preferred satellite through PPCB
/// and routes there again after local write status is restored.
#[tokio::test(start_paused = true)]
async fn warm_client_routes_away_from_revoked_region_and_back_after_restore() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;

    emulator.store().revoke_region_write("West US").unwrap();
    let revoked_hosts = write_and_capture_hosts(&driver, &recorder, "warm-revoked-write").await;
    assert!(
        revoked_hosts.iter().any(|host| host == WEST_HOST)
            && revoked_hosts.iter().any(|host| host == EAST_HOST),
        "warm client must fail from revoked West to East; observed {revoked_hosts:?}"
    );

    for index in 0..3 {
        let still_revoked =
            write_and_capture_hosts(&driver, &recorder, &format!("warm-still-revoked-{index}"))
                .await;
        assert!(
            still_revoked.iter().any(|host| host == EAST_HOST),
            "every write must make progress through healthy East while West remains revoked; observed {still_revoked:?}"
        );
    }

    emulator.store().restore_region_write("West US").unwrap();
    advance_past_refresh().await;
    let restored_hosts = write_and_capture_hosts(&driver, &recorder, "warm-restored-write").await;
    assert!(
        !restored_hosts.is_empty() && restored_hosts.iter().all(|host| host == WEST_HOST),
        "warm client must fail back after write restoration; observed {restored_hosts:?}"
    );
}

/// When every explicitly preferred region is offline, the driver uses an
/// available nonpreferred account region.
#[tokio::test(start_paused = true)]
async fn exhausted_offline_preferences_use_available_nonpreferred_region() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "exhausted-preferences-item").await;
    emulator.store().set_region_offline("West US").unwrap();
    emulator.store().set_region_offline("Central US").unwrap();
    advance_past_refresh().await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "exhausted-preferences-item").await;
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "driver must use healthy nonpreferred East; observed {hosts:?}"
    );
}
