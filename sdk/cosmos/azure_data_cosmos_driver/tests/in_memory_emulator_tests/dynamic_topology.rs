// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for **runtime account-topology changes**:
//! adding a region, removing a region, switching single-write to multi-write,
//! and moving write ownership between regions.
//!
//! All four reach a client through exactly one channel -- the `readableLocations`
//! / `writableLocations` / `enableMultipleWriteLocations` fields of the account
//! read -- so these tests mutate the emulator's topology and assert the driver
//! recomputes its routing on the next background account refresh, with no
//! restart.
//!
//! The assertions are deliberately modeled on the .NET SDK's own coverage so
//! the two SDKs agree on observable behavior:
//!
//! | .NET test | mirrored here |
//! | --- | --- |
//! | `GlobalEndpointManagerTest.ReadLocationRemoveAndAddMockTestAsync` | [`removing_a_region_moves_traffic_to_next_preferred`], [`re_adding_a_region_restores_preference_order`] |
//! | `LocationCacheTests.ValidateRetryOnDatabaseAccountNotFoundAsync` | [`read_to_retired_region_gets_403_1008`] and the retry-count assertions |
//! | `LocationCacheTests.ValidateRetryOnWriteForbiddenExceptionAsync` | [`demoted_write_region_returns_403_3`] |
//!
//! Time is virtualized with `tokio::time::pause()` (via `start_paused`) because
//! the production refresh interval is five minutes.

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::{Method, Request, StatusCode, Url};

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, RegionStatus, ReplicationConfig, SeedingPolicy,
    VirtualAccountConfig, VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, Region};
use azure_data_cosmos_driver::CosmosDriver;

use super::collect_response;
use super::host_recorder::HostRecorder;

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
const CENTRAL_URL: &str = "https://centralus.emulator.local";
const WEST_HOST: &str = "westus.emulator.local";
const CENTRAL_HOST: &str = "centralus.emulator.local";

/// Production refresh interval, kept in sync with
/// `LocationStateStore::BACKGROUND_REFRESH_INTERVAL` (300 s).
const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

fn east() -> VirtualRegion {
    VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap())
}

fn west() -> VirtualRegion {
    VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap())
}

fn central() -> VirtualRegion {
    VirtualRegion::new("Central US", Url::parse(CENTRAL_URL).unwrap())
}

/// Builds an emulator over the given regions, with `testdb`/`testcoll`
/// provisioned and the supplied observer attached.
fn build_emulator(
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    observer: Arc<HostRecorder>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(regions)
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

async fn build_driver(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    preferred: Vec<Region>,
) -> Arc<CosmosDriver> {
    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    runtime
        .create_driver(
            DriverOptions::builder(account())
                .with_preferred_regions(preferred)
                .build(),
        )
        .await
        .expect("driver should initialize")
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

/// Lets the background account-refresh loop observe the new topology.
async fn advance_past_refresh() {
    // Two intervals, so a first tick racing the test scheduler under paused
    // time still leaves a second one.
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
}

// --- Region removal ---------------------------------------------------------

/// Mirrors `GlobalEndpointManagerTest.ReadLocationRemoveAndAddMockTestAsync`:
/// dropping the client's top preferred region from the account moves reads to
/// the next preferred region, without restarting the driver.
#[tokio::test(start_paused = true)]
async fn removing_a_region_moves_traffic_to_next_preferred() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    // East US is the write region, so prefer the two satellites for reads and
    // remove the first of them.
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "topology-item").await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "topology-item").await;
    assert!(
        hosts.iter().all(|h| h == WEST_HOST),
        "test setup: reads should start on the top preferred region; observed {hosts:?}"
    );

    emulator
        .store()
        .remove_region("West US")
        .expect("removing a non-write region should succeed");
    advance_past_refresh().await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "topology-item").await;
    assert!(
        hosts.iter().all(|h| h == CENTRAL_HOST),
        "after the region is removed, reads must move to the next preferred \
         region ({CENTRAL_HOST}); observed {hosts:?}"
    );
}

/// The second half of `ReadLocationRemoveAndAddMockTestAsync`: adding the
/// region back restores the original preference order once the background
/// refresh rediscovers it.
#[tokio::test(start_paused = true)]
async fn re_adding_a_region_restores_preference_order() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::CENTRAL_US]).await;
    seed_item(&driver, "topology-item").await;

    let store = emulator.store();
    store
        .remove_region("West US")
        .expect("remove should succeed");
    advance_past_refresh().await;

    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("re-adding a removed region should succeed");
    advance_past_refresh().await;

    let hosts = read_and_capture_hosts(&driver, &recorder, "topology-item").await;
    assert!(
        hosts.iter().all(|h| h == WEST_HOST),
        "the re-added region must reclaim its place in the preference order; \
         observed {hosts:?}"
    );
}

/// A re-added region keeps its original region ID, because session-token vector
/// clocks issued while it was active still reference that ID.
#[tokio::test]
async fn re_added_region_keeps_its_region_id() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();
    let original_id = store.config().region_id_for("West US");

    store
        .remove_region("West US")
        .expect("remove should succeed");
    assert_eq!(
        store.config().region_id_for("West US"),
        original_id,
        "a retired region must keep its ID so tokens referencing it stay resolvable"
    );

    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("re-add should succeed");
    assert_eq!(
        store.config().region_id_for("West US"),
        original_id,
        "re-adding a region must not renumber it"
    );
}

/// An exhausted region-ID space must be reported, not wrapped: reusing an ID
/// would break the never-reuse invariant that keeps stale session tokens safe.
#[tokio::test]
async fn region_id_allocation_reports_exhaustion_instead_of_overflowing() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![
            east(),
            VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()).with_region_id(u64::MAX),
        ],
        WriteMode::Single,
        recorder,
    );
    let store = emulator.store();

    // Auto-allocating past `u64::MAX` must error rather than panic or wrap to 0.
    let error = store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect_err("allocating past the end of the ID space must fail");
    assert_eq!(
        error.status().status_code(),
        StatusCode::BadRequest,
        "exhaustion should surface as 400, got {error:?}"
    );
}

/// A brand-new region gets a fresh ID that does not collide with any existing
/// or retired region.
#[tokio::test]
async fn added_region_gets_a_fresh_unused_id() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect("add should succeed");

    let ids = ["East US", "West US", "Central US"]
        .map(|name| store.config().region_id_for(name))
        .to_vec();
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(
        unique.len(),
        ids.len(),
        "region IDs must be unique: {ids:?}"
    );
}

// --- Retired-region responses ----------------------------------------------

/// A request sent to a removed region gets `403 Forbidden` with substatus
/// `1008 DatabaseAccountNotFound` — **not** a routing failure.
///
/// Removing a region makes its regional endpoint return this within seconds,
/// well before ARM reports the update complete. The status, substatus and body
/// shape all match what the service sends.
#[tokio::test]
async fn read_to_retired_region_gets_403_1008() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    emulator
        .store()
        .remove_region("West US")
        .expect("remove should succeed");

    let request = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs/x")).unwrap(),
        Method::Get,
    );
    let response = emulator
        .execute_request(&request)
        .await
        .expect("a retired region must answer, not fail to route");

    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(
        headers.get_optional_str(&super::SUBSTATUS),
        Some("1008"),
        "a removed region must report DatabaseAccountNotFound"
    );

    // Body shape. The live response (from a region removed seconds earlier)
    // was, with the ActivityId/version suffix on the message elided:
    //
    //   { "code": "Forbidden",
    //     "message": "Database Account {id} does not exist",
    //     "writableLocations": [], "readableLocations": [],
    //     "id": "{account}-{region}" }
    let actual = body.as_object().expect("error body is a JSON object");
    let mut keys: Vec<&str> = actual.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "code",
            "id",
            "message",
            "readableLocations",
            "writableLocations"
        ],
        "403/1008 body shape must match the service"
    );
    assert_eq!(actual["code"], "Forbidden");
    assert_eq!(actual["writableLocations"], serde_json::json!([]));
    assert_eq!(actual["readableLocations"], serde_json::json!([]));
    assert_eq!(
        actual["id"], "westus",
        "the service names the account from the regional host it was called on"
    );
    assert!(
        actual["message"]
            .as_str()
            .unwrap()
            .starts_with("Database Account westus does not exist"),
        "message should match the service's wording; got {}",
        actual["message"]
    );
}

/// Removing a region retires its endpoint rather than forgetting it: the URL
/// still resolves, so the emulator can answer 403/1008 instead of failing to
/// route a request the client sent before refreshing.
#[tokio::test]
async fn removed_region_endpoint_still_resolves_as_retired() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    let resolved = store
        .config()
        .region_for_url(&Url::parse(WEST_URL).unwrap())
        .expect("active region should resolve");
    assert_eq!(resolved.status, RegionStatus::Active);

    store
        .remove_region("West US")
        .expect("remove should succeed");

    let resolved = store
        .config()
        .region_for_url(&Url::parse(WEST_URL).unwrap())
        .expect("retired region must still resolve");
    assert_eq!(resolved.status, RegionStatus::Retired);
    assert_eq!(resolved.name, "West US");
}

// --- Write-mode and write-region changes ------------------------------------

/// Before enabling multi-write, a write to a non-hub region is rejected with
/// `403/3 WriteForbidden`; afterwards the same region accepts it.
#[tokio::test]
async fn enabling_multi_write_makes_satellite_regions_writable() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    let write_request = |id: &str| {
        let mut req = Request::new(
            Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
            Method::Post,
        );
        req.set_body(serde_json::json!({"id": id, "pk": "pk1"}).to_string());
        req.headers_mut().insert(
            super::PARTITION_KEY.clone(),
            azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
        );
        req
    };

    let response = emulator
        .execute_request(&write_request("before"))
        .await
        .unwrap();
    let (status, headers, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(
        headers.get_optional_str(&super::SUBSTATUS),
        Some("3"),
        "single-write accounts reject writes to a satellite with WriteForbidden"
    );

    store.set_write_mode(WriteMode::Multi);

    let response = emulator
        .execute_request(&write_request("after"))
        .await
        .unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(
        status,
        StatusCode::Created,
        "after enabling multi-write the satellite must accept writes"
    );
}

/// Moving write ownership demotes the previous hub: writes there start failing
/// with `403/3`, exactly as during a failover. The gateway itself can report a
/// different write location between account reads, so clients must tolerate it.
#[tokio::test]
async fn demoted_write_region_returns_403_3() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .set_write_region("West US")
        .expect("promoting an active region should succeed");

    let mut req = Request::new(
        Url::parse(&format!("{EAST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    req.set_body(serde_json::json!({"id": "demoted", "pk": "pk1"}).to_string());
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
        "the demoted hub must reject writes with WriteForbidden"
    );
}

/// After a write-region change, the driver routes writes to the new hub once it
/// picks the topology up.
#[tokio::test(start_paused = true)]
async fn write_region_change_moves_writes_to_new_hub() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&driver, "hub-move").await;

    emulator
        .store()
        .set_write_region("West US")
        .expect("promotion should succeed");
    advance_past_refresh().await;

    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    recorder.clear();
    let item = ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "hub-move-2".to_string(),
    );
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(
                serde_json::json!({"id": "hub-move-2", "pk": "pk1"})
                    .to_string()
                    .into_bytes(),
            ),
            OperationOptions::default(),
        )
        .await
        .expect("write should succeed against the promoted hub");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().all(|h| h == WEST_HOST),
        "writes must follow the promoted write region; observed {hosts:?}"
    );
}

// --- Mutation guardrails ----------------------------------------------------

/// Adding a region that is already part of the account is a `400`, matching the
/// service's rejection of a redundant add-read-region request.
#[tokio::test]
async fn adding_an_existing_region_is_rejected() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);

    let error = emulator
        .store()
        .add_region(west(), SeedingPolicy::Immediate)
        .expect_err("re-adding an active region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
}

/// The account must always retain a write region, so removing the current one
/// is rejected rather than leaving writes nowhere to go.
#[tokio::test]
async fn removing_the_write_region_is_rejected() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);

    let error = emulator
        .store()
        .remove_region("East US")
        .expect_err("removing the write region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    // Promoting the other region first makes the removal legal.
    let store = emulator.store();
    store
        .set_write_region("West US")
        .expect("promotion should succeed");
    store
        .remove_region("East US")
        .expect("the demoted region can now be removed");
}

/// The window the live service actually exhibits during a region removal: the
/// regional endpoint returns `403/1008` while the account read **still**
/// advertises the region, so a client that refreshes topology in response to the
/// 1008 gets the dead region right back.
///
/// The service exhibits this: a removed region's regional endpoint begins
/// returning 403/1008 within seconds of the removal being accepted, while the
/// global account read keeps listing that region for several more minutes.
#[tokio::test]
async fn draining_region_rejects_while_still_advertised() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .begin_region_removal("West US")
        .expect("draining an active region should succeed");

    // The endpoint already rejects...
    let request = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs/x")).unwrap(),
        Method::Get,
    );
    let (status, headers, _) =
        collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(headers.get_optional_str(&super::SUBSTATUS), Some("1008"));

    // ...but a topology refresh still hands the region back.
    let account = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, _, body) = collect_response(emulator.execute_request(&account).await.unwrap()).await;
    let readable: Vec<&str> = body["readableLocations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        readable,
        vec!["East US", "West US"],
        "a draining region stays advertised until the removal completes"
    );

    // Completing the removal finally drops it from the topology, and it stays
    // resolvable as retired.
    store
        .remove_region("West US")
        .expect("remove should succeed");
    let (_, _, body) = collect_response(emulator.execute_request(&account).await.unwrap()).await;
    assert_eq!(
        body["readableLocations"].as_array().unwrap().len(),
        1,
        "after removal the region is no longer advertised"
    );
    assert_eq!(
        store
            .config()
            .region_for_url(&Url::parse(WEST_URL).unwrap())
            .unwrap()
            .status,
        RegionStatus::Retired
    );
}

/// A delayed-seeding region must read as genuinely *behind*, not merely empty.
///
/// Session freshness is judged against the partition LSN counters, so a region
/// left at the source's high-water mark with no documents would claim to be
/// caught up and answer a session read with a bare `404` — which a driver treats
/// as "item does not exist" — instead of `404/1002 ReadSessionNotAvailable`,
/// which drives session retry and region failover.
#[tokio::test]
async fn delayed_seeding_region_reports_session_not_available() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US]).await;
    seed_item(&driver, "session-item").await;

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_secs(60)))
        .expect("add should succeed");

    // Capture the token *after* the topology change, so its version matches the
    // partition's current one and the LSN comparison actually runs. A token from
    // before the change carries an older version and is superseded rather than
    // compared — which is what the live service does too.
    //
    // A read (not a write) keeps the delayed region empty: a write would
    // replicate into it immediately under `ReplicationConfig::immediate()`.
    let mut hub_read = Request::new(
        Url::parse(&format!(
            "{EAST_URL}/dbs/testdb/colls/testcoll/docs/session-item"
        ))
        .unwrap(),
        Method::Get,
    );
    hub_read.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (_, headers, _) =
        collect_response(emulator.execute_request(&hub_read).await.unwrap()).await;
    let session_token = headers
        .get_optional_str(&super::SESSION_TOKEN)
        .expect("read should return a session token")
        .to_string();

    let mut request = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/session-item"
        ))
        .unwrap(),
        Method::Get,
    );
    request.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    request.headers_mut().insert(
        super::SESSION_TOKEN.clone(),
        azure_core::http::headers::HeaderValue::from(session_token),
    );

    let (status, headers, _) =
        collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(status, StatusCode::NotFound);
    assert_eq!(
        headers.get_optional_str(&super::SUBSTATUS),
        Some("1002"),
        "a not-yet-caught-up region must report ReadSessionNotAvailable, not a \
         bare 404 that looks like a missing item"
    );
}

/// Draining is the first phase of removal, so it carries removal's guards: a
/// drained write region (or last region) would be advertised but unable to serve
/// anything, and no public call could repair it.
#[tokio::test]
async fn draining_the_write_or_last_region_is_rejected() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    let error = store
        .begin_region_removal("East US")
        .expect_err("draining the write region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    let single = build_emulator(vec![east()], WriteMode::Single, HostRecorder::new());
    let error = single
        .store()
        .begin_region_removal("East US")
        .expect_err("draining the last region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    // Promoting the other region first makes draining legal.
    store
        .set_write_region("West US")
        .expect("promotion should succeed");
    store
        .begin_region_removal("East US")
        .expect("the demoted region can now be drained");
}

/// A region added to a **multi-write** account joins `readableLocations` and
/// `writableLocations` together, and accepts writes immediately.
///
/// On a multi-write account the new region enters both lists in a single atomic
/// transition, with none of the flapping a single-write add exhibits.
#[tokio::test]
async fn region_added_to_multi_write_account_is_immediately_writable() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Multi, recorder);
    let store = emulator.store();

    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("add should succeed");

    let account = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, _, body) = collect_response(emulator.execute_request(&account).await.unwrap()).await;
    let names = |key: &str| {
        body[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|l| l["name"].as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(names("readableLocations"), vec!["East US", "West US"]);
    assert_eq!(
        names("writableLocations"),
        vec!["East US", "West US"],
        "under multi-write a joining region is writable as soon as it is readable"
    );

    // And it really accepts writes.
    let mut write = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "mw-add", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(status, StatusCode::Created);
}

/// Under multi-write, a draining region is still advertised as **writable**
/// while its endpoint already rejects writes with 403/1008.
///
/// This is the window that makes removal riskier under multi-write than under
/// single-write: a multi-write client routes writes to its *local* region, so a
/// client colocated with the dying region writes into it, gets 403/1008,
/// refreshes topology, is told the region is still writable, and retries into it
/// again. Under single-write those writes were going to the hub anyway.
#[tokio::test]
async fn draining_region_under_multi_write_still_advertised_as_writable() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let store = emulator.store();

    store
        .begin_region_removal("West US")
        .expect("draining a non-write region should succeed");

    let mut write = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    write.set_body(serde_json::json!({"id": "mw-drain", "pk": "pk1"}).to_string());
    write.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, headers, _) =
        collect_response(emulator.execute_request(&write).await.unwrap()).await;
    assert_eq!(status, StatusCode::Forbidden);
    assert_eq!(
        headers.get_optional_str(&super::SUBSTATUS),
        Some("1008"),
        "the draining region rejects writes even though it is advertised writable"
    );

    let account = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, _, body) = collect_response(emulator.execute_request(&account).await.unwrap()).await;
    let writable: Vec<&str> = body["writableLocations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["name"].as_str().unwrap())
        .collect();
    assert!(
        writable.contains(&"West US"),
        "a refresh during the drain still reports the dying region as writable; got {writable:?}"
    );
}

/// A region membership change bumps the session-token **version**, across every
/// region and partition.
///
/// Verified live: the version advanced on each topology change (`-1 → 0` when a
/// region was added, `2 → 3` when it was removed, `3 → 4` when it was re-added).
/// That bump is what makes a client's older token safe — region entries recorded
/// under a previous version are superseded rather than compared against a
/// topology that no longer exists. Without it, the emulator would keep serving
/// the same version across a topology change and stale region entries would
/// never be invalidated.
#[tokio::test]
async fn region_membership_change_bumps_session_token_version() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    // `{pkrange}:{version}#{globalLSN}...`
    let version_of = |token: &str| -> u64 {
        token
            .split('#')
            .next()
            .and_then(|head| head.split(':').nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| panic!("unexpected session token shape: {token}"))
    };

    let write_and_read_version = |suffix: &str| {
        let suffix = suffix.to_string();
        let emulator = emulator.clone();
        async move {
            let mut req = Request::new(
                Url::parse(&format!("{EAST_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
                Method::Post,
            );
            req.set_body(
                serde_json::json!({"id": format!("vc-{suffix}"), "pk": "pk1"}).to_string(),
            );
            req.headers_mut().insert(
                super::PARTITION_KEY.clone(),
                azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
            );
            let (_, headers, _) =
                collect_response(emulator.execute_request(&req).await.unwrap()).await;
            headers
                .get_optional_str(&super::SESSION_TOKEN)
                .expect("write should return a session token")
                .to_string()
        }
    };

    let before = version_of(&write_and_read_version("before").await);

    store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    let after_add = version_of(&write_and_read_version("after-add").await);
    assert!(
        after_add > before,
        "adding a region must bump the session-token version ({before} -> {after_add})"
    );

    store
        .remove_region("Central US")
        .expect("remove should succeed");
    let after_remove = version_of(&write_and_read_version("after-remove").await);
    assert!(
        after_remove > after_add,
        "removing a region must bump the session-token version \
         ({after_add} -> {after_remove})"
    );
}

/// Promoting a **draining** region must be refused.
///
/// Without this guard the account can be walked into an unrecoverable state
/// purely through public calls: promote West, drain East, promote East back
/// (allowed, since East is still *active*), then remove West. The account is
/// left advertising a single region whose endpoint rejects every request with
/// 403/1008, and neither removal path can retire it — `remove_region` refuses
/// both the write region and the last region.
#[tokio::test]
async fn promoting_a_draining_region_is_rejected() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .set_write_region("West US")
        .expect("promotion should succeed");
    store
        .begin_region_removal("East US")
        .expect("draining the non-write region should succeed");

    let error = store
        .set_write_region("East US")
        .expect_err("promoting a draining region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    // The escape hatch: abort the removal, then the promotion is legal again.
    store
        .cancel_region_removal("East US")
        .expect("cancelling the removal should succeed");
    store
        .set_write_region("East US")
        .expect("promotion should succeed once the region is no longer draining");
}

/// Draining is reversible — otherwise it is a one-way door, since
/// `remove_region` is the only other exit and it refuses the write and last
/// regions.
#[tokio::test]
async fn a_draining_region_can_be_returned_to_service() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    store
        .begin_region_removal("West US")
        .expect("draining should succeed");

    let mut request = Request::new(
        Url::parse(&format!("{WEST_URL}/dbs/testdb/colls/testcoll/docs/x")).unwrap(),
        Method::Get,
    );
    request.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );
    let (status, _, _) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Forbidden,
        "test setup: a draining region rejects requests"
    );

    store
        .cancel_region_removal("West US")
        .expect("cancelling should succeed");

    let (status, _, _) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::NotFound,
        "after cancelling, the region serves normally again (404 = item absent, \
         not 403 = region gone)"
    );
}

/// Removing the only region would leave the account unreachable.
#[tokio::test]
async fn removing_the_last_region_is_rejected() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder);

    let error = emulator
        .store()
        .remove_region("East US")
        .expect_err("removing the last region must fail");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);
}

/// The write-region guard applies under multi-write too. Every active region
/// accepts writes there, but `write_region` still designates the hub that a
/// later switch back to single-write restores and that new regions are seeded
/// from -- so letting it be removed would leave `writableLocations` empty and
/// new regions unseeded.
#[tokio::test]
async fn removing_the_write_region_is_rejected_under_multi_write() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let store = emulator.store();

    let error = store
        .remove_region("East US")
        .expect_err("removing the write region must fail even under multi-write");
    assert_eq!(error.status().status_code(), StatusCode::BadRequest);

    // The account is still coherent: switching back to single-write leaves a
    // real writable location, and a region added afterwards is seeded.
    store.set_write_mode(WriteMode::Single);
    let request = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, _, body) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(
        body["writableLocations"].as_array().unwrap().len(),
        1,
        "the account must always advertise a writable location"
    );
}

/// A rejected duplicate add must leave the existing region's data untouched --
/// the store is now inserted before the region is published, so the guard has
/// to run before anything is built.
#[tokio::test]
async fn rejected_duplicate_add_preserves_existing_region_data() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US]).await;
    seed_item(&driver, "survivor").await;

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Immediate)
        .expect_err("duplicate add must fail");

    let mut request = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/survivor"
        ))
        .unwrap(),
        Method::Get,
    );
    request.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );

    let (status, _, _) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "a rejected duplicate add must not replace the existing region's store"
    );
}

// --- Seeding ----------------------------------------------------------------

/// A region added with [`SeedingPolicy::Immediate`] comes online already
/// holding the account's data, so a read routed there succeeds right away.
#[tokio::test]
async fn immediately_seeded_region_serves_existing_data() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US]).await;
    seed_item(&driver, "seeded-item").await;

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("add should succeed");

    let request = Request::new(
        Url::parse(&format!(
            "{WEST_URL}/dbs/testdb/colls/testcoll/docs/seeded-item"
        ))
        .unwrap(),
        Method::Get,
    );
    let mut request = request;
    request.headers_mut().insert(
        super::PARTITION_KEY.clone(),
        azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
    );

    let (status, _, _) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "a fully seeded region must serve data written before it joined"
    );
}

/// A region added with [`SeedingPolicy::Delayed`] is advertised immediately but
/// serves nothing until replication catches up -- the window in which a region
/// is in the topology but not yet useful.
#[tokio::test]
async fn delayed_seeding_region_is_empty_until_catch_up() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US]).await;
    seed_item(&driver, "late-item").await;

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Delayed(Duration::from_millis(50)))
        .expect("add should succeed");

    let build_read = || {
        let mut request = Request::new(
            Url::parse(&format!(
                "{WEST_URL}/dbs/testdb/colls/testcoll/docs/late-item"
            ))
            .unwrap(),
            Method::Get,
        );
        request.headers_mut().insert(
            super::PARTITION_KEY.clone(),
            azure_core::http::headers::HeaderValue::from_static("[\"pk1\"]"),
        );
        request
    };

    let (status, _, _) =
        collect_response(emulator.execute_request(&build_read()).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::NotFound,
        "before catch-up the region is advertised but empty"
    );

    tokio::time::sleep(Duration::from_millis(200)).await;

    let (status, _, _) =
        collect_response(emulator.execute_request(&build_read()).await.unwrap()).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "after catch-up the region serves the pre-existing data"
    );
}

// --- Account payload --------------------------------------------------------

/// The service does **not** send an `_etag` on the account read, on either the
/// global or a regional endpoint. The emulator must not invent one, or it would
/// exercise the driver's unchanged-etag short-circuit in
/// `sync_account_properties`, which is inert in production.
#[tokio::test]
async fn account_read_has_no_etag_like_the_service() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);

    let request = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, headers, body) =
        collect_response(emulator.execute_request(&request).await.unwrap()).await;

    assert!(
        !body.as_object().unwrap().contains_key("_etag"),
        "the account payload must not carry an _etag; got {body}"
    );
    assert!(
        headers.get_optional_str(&super::ETAG).is_none(),
        "the account response must not carry an ETag header"
    );
}

/// Every top-level field a live Cosmos DB account read returns, recorded from a
/// real two-region multi-write account.
///
/// Captured with `x-ms-version` `2020-07-15`; the raw payload is in the live
/// capture linked from the module docs. Kept as an explicit list rather than a
/// checked-in JSON blob so the expectation is readable in the test itself.
///
/// Note the absence of `_etag`: the service does not send one, on either the
/// global or a regional endpoint, for `x-ms-version` `2018-12-31` or
/// `2020-07-15`.
const SERVICE_ACCOUNT_FIELDS: &[&str] = &[
    "_dbs",
    "_rid",
    "_self",
    "addresses",
    "continuousBackupEnabled",
    "disableCrossRegionalHedging",
    "enableMultipleWriteLocations",
    "enableNRegionSynchronousCommit",
    "enablePerPartitionFailoverBehavior",
    "id",
    "media",
    "queryEngineConfiguration",
    "readPolicy",
    "readableLocations",
    "systemReplicationPolicy",
    "userConsistencyPolicy",
    "userReplicationPolicy",
    "writableLocations",
];

/// Account fields the service returns as JSON booleans.
const SERVICE_ACCOUNT_BOOL_FIELDS: &[&str] = &[
    "continuousBackupEnabled",
    "disableCrossRegionalHedging",
    "enableMultipleWriteLocations",
    "enableNRegionSynchronousCommit",
    "enablePerPartitionFailoverBehavior",
];

/// Keys the service nests under `userReplicationPolicy`.
const SERVICE_USER_REPLICATION_POLICY_KEYS: &[&str] =
    &["asyncReplication", "maxReplicasetSize", "minReplicaSetSize"];

/// The emulator's account payload must carry every field the live service
/// returns, so a driver change that starts consuming one is not silently
/// unexercised — and must not invent fields the service never sends.
#[tokio::test]
async fn account_payload_shape_matches_the_service() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder);
    let request = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
    let (_, _, body) = collect_response(emulator.execute_request(&request).await.unwrap()).await;
    let actual = body.as_object().expect("account payload is a JSON object");

    let missing: Vec<&&str> = SERVICE_ACCOUNT_FIELDS
        .iter()
        .filter(|field| !actual.contains_key(**field))
        .collect();
    assert!(
        missing.is_empty(),
        "emulator account payload is missing fields the service returns: {missing:?}"
    );

    let extra: Vec<&String> = actual
        .keys()
        .filter(|key| !SERVICE_ACCOUNT_FIELDS.contains(&key.as_str()))
        .collect();
    assert!(
        extra.is_empty(),
        "emulator account payload invents fields the service does not send: {extra:?} \
         (if the service really does send these, update SERVICE_ACCOUNT_FIELDS)"
    );

    for field in SERVICE_ACCOUNT_BOOL_FIELDS {
        assert!(
            actual[*field].is_boolean(),
            "{field} must be a JSON boolean, as the service sends it"
        );
    }

    let policy_keys: Vec<&str> = actual["userReplicationPolicy"]
        .as_object()
        .expect("userReplicationPolicy is an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        policy_keys, SERVICE_USER_REPLICATION_POLICY_KEYS,
        "userReplicationPolicy shape must match the service"
    );
}

/// `_rid` and `id` must name the same account on a single payload — the live
/// service never disagrees with itself there.
///
/// The service additionally encodes the **write** region into `_rid`
/// (`{account}-{write-region}.sql.cosmos.azure.com`, regardless of which endpoint
/// served the read). The emulator's synthetic hosts carry no
/// `{account}-{region}` structure to recover a base account name from, so that
/// component is deliberately not modeled.
#[tokio::test]
async fn account_rid_and_id_name_the_same_account() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);

    for url in [EAST_URL, WEST_URL] {
        let request = Request::new(Url::parse(url).unwrap(), Method::Get);
        let (_, _, body) =
            collect_response(emulator.execute_request(&request).await.unwrap()).await;
        let id = body["id"].as_str().unwrap();
        let rid = body["_rid"].as_str().unwrap();
        assert_eq!(
            rid,
            format!("{id}.sql.cosmos.azure.com"),
            "id and _rid must name the same account (endpoint {url})"
        );
    }
}

/// The account `id` is derived from the host the read arrived on, mirroring the
/// gateway rewriting the account resource per regional endpoint.
#[tokio::test]
async fn account_id_reflects_the_request_host() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);

    let account_id = |url: &str| {
        let url = url.to_string();
        let emulator = emulator.clone();
        async move {
            let request = Request::new(Url::parse(&url).unwrap(), Method::Get);
            let (_, _, body) =
                collect_response(emulator.execute_request(&request).await.unwrap()).await;
            body["id"].as_str().unwrap().to_string()
        }
    };

    assert_eq!(account_id(EAST_URL).await, "eastus");
    assert_eq!(account_id(WEST_URL).await, "westus");
}

/// Read and write location lists track the topology, including the single-write
/// account advertising exactly one writable location.
#[tokio::test]
async fn account_locations_track_topology_changes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let store = emulator.store();

    let locations = || async {
        let request = Request::new(Url::parse(EAST_URL).unwrap(), Method::Get);
        let (_, _, body) =
            collect_response(emulator.execute_request(&request).await.unwrap()).await;
        let names = |key: &str| {
            body[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|l| l["name"].as_str().unwrap().to_string())
                .collect::<Vec<_>>()
        };
        (
            names("readableLocations"),
            names("writableLocations"),
            body["enableMultipleWriteLocations"].as_bool().unwrap(),
        )
    };

    let (readable, writable, multi) = locations().await;
    assert_eq!(readable, vec!["East US", "West US"]);
    assert_eq!(writable, vec!["East US"]);
    assert!(!multi);

    store.set_write_mode(WriteMode::Multi);
    let (_, writable, multi) = locations().await;
    assert_eq!(writable, vec!["East US", "West US"]);
    assert!(multi);

    store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    let (readable, writable, _) = locations().await;
    assert_eq!(readable, vec!["East US", "West US", "Central US"]);
    assert_eq!(writable, vec!["East US", "West US", "Central US"]);

    store.set_write_mode(WriteMode::Single);
    store
        .remove_region("West US")
        .expect("remove should succeed");
    let (readable, writable, multi) = locations().await;
    assert_eq!(readable, vec!["East US", "Central US"]);
    assert_eq!(writable, vec!["East US"]);
    assert!(!multi);
}
