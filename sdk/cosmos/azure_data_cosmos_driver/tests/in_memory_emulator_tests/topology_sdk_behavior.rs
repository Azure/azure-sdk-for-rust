// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! **SDK-behavior** tests for account-topology changes.
//!
//! `dynamic_topology.rs` establishes that the *emulator* reproduces what the
//! service does. This file asks the complementary question: given that
//! behavior, does the **driver** do the right thing? Every test here drives the
//! full `CosmosDriver::execute_operation` pipeline — endpoint resolution,
//! session routing, retry, failover, background refresh — and asserts on
//! driver-observable outcomes rather than on emulator internals.
//!
//! The topology changes are *real* (regions genuinely added and removed from
//! the emulated account) rather than fault-injected, so the driver sees the same
//! `403/1008`, `403/3` and shifting location lists a live account produces.
//!
//! Retry-count expectations are ported from the .NET SDK's
//! `LocationCacheTests.ValidateRetryOnDatabaseAccountNotFoundAsync`, which pins:
//!
//! ```text
//! expectedRetryCount = isReadRequest || enableMultipleWriteLocations ? 2 : 1;
//! ```
//!
//! i.e. reads and multi-write writes cross-region retry once on `403/1008`;
//! a write on a single-write account surfaces the `Forbidden` instead.

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, SeedingPolicy,
    VirtualAccountConfig, VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{DriverOptions, ExcludedRegions, OperationOptions, Region};
use azure_data_cosmos_driver::CosmosDriver;

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

fn east() -> VirtualRegion {
    VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap())
}

fn west() -> VirtualRegion {
    VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap())
}

fn central() -> VirtualRegion {
    VirtualRegion::new("Central US", Url::parse(CENTRAL_URL).unwrap())
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(EAST_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

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

async fn read_item(
    driver: &CosmosDriver,
    item_id: &str,
    options: OperationOptions,
) -> Result<(), azure_data_cosmos_driver::error::CosmosError> {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    driver
        .execute_operation(CosmosOperation::read_item(item), options)
        .await
        .map(|_| ())
}

async fn write_item(
    driver: &CosmosDriver,
    item_id: &str,
    options: OperationOptions,
) -> Result<(), azure_data_cosmos_driver::error::CosmosError> {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    let body = serde_json::json!({"id": item_id, "pk": "pk1", "value": 2}).to_string();
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(body.into_bytes()),
            options,
        )
        .await
        .map(|_| ())
}

/// Lets the background account-refresh loop observe a new topology.
async fn advance_past_refresh() {
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
}

// --- 403/1008 retry contract ------------------------------------------------

/// A **read** routed to a region that was removed while the driver held stale
/// topology must recover: `403/1008` triggers a topology refresh and a retry to
/// the next preferred region, and the caller sees success.
///
/// This is the .NET `expectedRetryCount = 2` case for reads, driven by a real
/// region removal rather than an injected fault.
#[tokio::test(start_paused = true)]
async fn read_recovers_when_its_preferred_region_is_removed() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    // Prefer West (a read region) so the removal below takes out the region the
    // driver is actively using.
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "read-recover").await;

    // Seeding is a write and goes to the hub, so scope the assertion to the read.
    recorder.clear();
    read_item(&driver, "read-recover", OperationOptions::default())
        .await
        .expect("test setup: read should succeed before the removal");
    assert!(
        recorder.data_plane_hosts().iter().all(|h| h == WEST_HOST),
        "test setup: reads should start on the preferred region; got {:?}",
        recorder.data_plane_hosts()
    );

    // Remove West *without* letting the driver refresh: its cached topology
    // still lists West, so the next read routes into the dead region.
    emulator
        .store()
        .remove_region("West US")
        .expect("remove should succeed");
    recorder.clear();

    read_item(&driver, "read-recover", OperationOptions::default())
        .await
        .expect("the read must recover from 403/1008 by retrying another region");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.first().map(String::as_str) == Some(WEST_HOST),
        "the first attempt should still target the stale preferred region; got {hosts:?}"
    );
    assert!(
        hosts.last().map(String::as_str) == Some(EAST_HOST),
        "the retry must land on the surviving region; got {hosts:?}"
    );
}

/// A **write** on a single-write account whose write region was removed
/// refreshes topology and retries into the *new* write region, rather than
/// surfacing the `Forbidden`.
///
/// **This is a deliberate divergence from the .NET SDK.**
/// `LocationCacheTests.ValidateRetryOnDatabaseAccountNotFoundAsync` pins
/// `expectedRetryCount = 1` for this case — .NET surfaces the error. This driver
/// instead treats `403/1008` as a topology-divergence signal for *every*
/// operation type including writes (see the comment in
/// `retry_evaluation::evaluate_http_outcome`, PR #4590): the region no longer
/// owns the account, so the request refreshes account properties and fails over.
///
/// The retry is safe precisely because the refresh happens first — the driver
/// learns the current write region before retrying, so the write lands in the
/// right place rather than being blindly replayed into a read-only region.
#[tokio::test(start_paused = true)]
async fn single_write_write_refreshes_and_retries_when_its_write_region_is_removed() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    // East is the hub. Promote West so the driver caches West as the write
    // region, then take West away underneath it.
    let store = emulator.store();
    store
        .set_write_region("West US")
        .expect("promotion should succeed");

    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "sw-seed").await;

    store
        .set_write_region("East US")
        .expect("promotion back should succeed");
    store
        .remove_region("West US")
        .expect("remove should succeed");
    recorder.clear();

    write_item(&driver, "sw-after", OperationOptions::default())
        .await
        .expect("the write must recover by refreshing topology and retrying the new hub");

    let hosts = recorder.data_plane_hosts();
    assert_eq!(
        hosts.first().map(String::as_str),
        Some(WEST_HOST),
        "the first attempt should target the stale cached write region; got {hosts:?}"
    );
    assert_eq!(
        hosts.last().map(String::as_str),
        Some(EAST_HOST),
        "after refreshing, the retry must land on the *current* write region — \
         not merely on some other region; got {hosts:?}"
    );
}

/// A **write** on a multi-write account *does* cross-region retry on
/// `403/1008`, because every region is a legitimate write target.
///
/// The `enableMultipleWriteLocations = true` half of the .NET contract.
#[tokio::test(start_paused = true)]
async fn multi_write_write_recovers_when_its_region_is_removed() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Multi, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "mw-seed").await;

    emulator
        .store()
        .remove_region("West US")
        .expect("remove should succeed");
    recorder.clear();

    write_item(&driver, "mw-after", OperationOptions::default())
        .await
        .expect("a multi-write write must recover by retrying another write region");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.last().map(String::as_str) == Some(EAST_HOST),
        "the retry must land on a surviving write region; got {hosts:?}"
    );
}

// --- Draining window --------------------------------------------------------

/// During the draining window the endpoint rejects with `403/1008` while the
/// account still advertises the region — so a topology refresh hands the dead
/// region straight back. The driver must still make progress.
///
/// This is the window measured live, where the regional endpoint failed minutes
/// before the account read stopped listing the region.
#[tokio::test(start_paused = true)]
async fn operations_make_progress_while_a_region_is_draining() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "drain-item").await;

    emulator
        .store()
        .begin_region_removal("West US")
        .expect("draining should succeed");

    // Refreshing does NOT help here: the region is still advertised.
    advance_past_refresh().await;
    recorder.clear();

    read_item(&driver, "drain-item", OperationOptions::default())
        .await
        .expect("reads must still succeed while a region is draining");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == EAST_HOST),
        "the read must reach the healthy region even though the account still \
         advertises the draining one; got {hosts:?}"
    );
}

// --- Background refresh adoption --------------------------------------------

/// A region added at runtime is adopted by a **running** driver on its next
/// background account refresh — no restart, no reconnect.
#[tokio::test(start_paused = true)]
async fn added_region_is_adopted_without_restart() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    // Prefer a region that does not exist yet: the driver must tolerate that,
    // then adopt it once it appears.
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "adopt-item").await;

    read_item(&driver, "adopt-item", OperationOptions::default())
        .await
        .expect("a preferred region that is not in the account must not break routing");

    let (_, readable) = driver
        .cached_account_regions_for_testing()
        .await
        .expect("account metadata should be cached");
    assert_eq!(
        readable.len(),
        1,
        "test setup: the driver should start with one region"
    );

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    advance_past_refresh().await;
    read_item(&driver, "adopt-item", OperationOptions::default())
        .await
        .expect("read after refresh should succeed");

    let (_, readable) = driver
        .cached_account_regions_for_testing()
        .await
        .expect("account metadata should be cached");
    assert_eq!(
        readable.len(),
        2,
        "the running driver must pick up the added region; saw {readable:?}"
    );

    recorder.clear();
    read_item(&driver, "adopt-item", OperationOptions::default())
        .await
        .expect("read should succeed");
    assert!(
        recorder.data_plane_hosts().iter().all(|h| h == WEST_HOST),
        "once present, the newly added region must take its place in the \
         preference order; got {:?}",
        recorder.data_plane_hosts()
    );
}

// --- Unavailability marks across topology changes ---------------------------

/// A region marked unavailable, then removed and re-added, keeps its mark
/// across the topology churn — `sync_account_properties` is a pure
/// carry-forward, so no account payload can drop a mark.
///
/// This is the driver-visible half of the "immortal mark" fix. The mark is
/// deliberately **kept** rather than pruned on removal: it is inert for routing
/// while the region is absent (endpoint selection only consults marks for
/// endpoints in the current preferred lists), and keeping it means a region that
/// returns to the account still has to pass a connectivity probe before taking
/// live traffic. The wasted-probe cost that motivated pruning is handled in
/// `probe_and_failback_unavailable_endpoints`, which skips endpoints the account
/// no longer advertises.
///
/// Scope: this test covers topology churn only. The probe cooldown is measured
/// with `std::time::Instant`, which `start_paused` does **not** advance, so no
/// endpoint becomes probe-due here however far the virtual clock moves — the
/// probe loop is deliberately out of the picture. That the mark is cleared *by a
/// probe* is proved separately by
/// `a_successful_probe_is_what_clears_an_unavailability_mark`.
#[tokio::test(start_paused = true)]
async fn unavailability_mark_is_probe_gated_across_remove_and_re_add() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "mark-item").await;

    assert!(
        driver.mark_region_endpoint_unavailable_for_testing(&Region::WEST_US),
        "test setup: West US should have an endpoint that can be marked"
    );

    // With West marked, reads avoid it even though it is the top preference.
    recorder.clear();
    read_item(&driver, "mark-item", OperationOptions::default())
        .await
        .expect("read should succeed on the unmarked region");
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == EAST_HOST),
        "test setup: a marked region must not take traffic; got {hosts:?}"
    );

    let store = emulator.store();
    store
        .remove_region("West US")
        .expect("remove should succeed");
    advance_past_refresh().await;
    read_item(&driver, "mark-item", OperationOptions::default())
        .await
        .expect("read should succeed after the removal");

    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("re-add should succeed");
    advance_past_refresh().await;
    read_item(&driver, "mark-item", OperationOptions::default())
        .await
        .expect("read should succeed after the re-add");

    // Decisive: the mark survived the round trip, so the returning region is
    // still held out of rotation pending a probe.
    assert!(
        driver.is_endpoint_host_marked_unavailable_for_testing(WEST_HOST),
        "the mark must survive remove/re-add so failback stays probe-gated — \
         a returning region must not silently take traffic again"
    );

    recorder.clear();
    read_item(&driver, "mark-item", OperationOptions::default())
        .await
        .expect("read should succeed");
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == EAST_HOST),
        "the still-marked region must not take traffic after re-add; got {hosts:?}"
    );
}

/// A topology flap must not disturb an unavailability mark.
///
/// The live gateway alternates between including and excluding a region for
/// minutes during a transition. `sync_account_properties` is a pure carry-forward
/// precisely so no payload — transient or not — can clear a mark; only a probe
/// can. This asserts the mark *and* its routing effect survive repeated churn.
///
/// As above, the probe loop cannot fire under `start_paused` (its cooldown uses
/// `std::time::Instant`), so this isolates the effect of the account payloads
/// themselves.
#[tokio::test(start_paused = true)]
async fn unavailability_mark_survives_a_topology_flap() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "flap-item").await;

    assert!(driver.mark_region_endpoint_unavailable_for_testing(&Region::WEST_US));

    let store = emulator.store();
    for iteration in 0..3 {
        store
            .remove_region("West US")
            .expect("remove should succeed");
        advance_past_refresh().await;
        read_item(&driver, "flap-item", OperationOptions::default())
            .await
            .expect("read should succeed mid-flap");

        store
            .add_region(west(), SeedingPolicy::Immediate)
            .expect("re-add should succeed");
        advance_past_refresh().await;

        recorder.clear();
        read_item(&driver, "flap-item", OperationOptions::default())
            .await
            .expect("read should succeed mid-flap");

        assert!(
            driver.is_endpoint_host_marked_unavailable_for_testing(WEST_HOST),
            "iteration {iteration}: a flapping region must keep its mark until a \
             probe clears it, not lose it to a transient absence"
        );
        let hosts = recorder.data_plane_hosts();
        assert!(
            !hosts.is_empty() && hosts.iter().all(|h| h == EAST_HOST),
            "iteration {iteration}: the marked region must still be excluded from \
             routing; got {hosts:?}"
        );
    }
}

/// A successful **probe** is what clears an unavailability mark — the other
/// half of "failback stays probe-gated".
///
/// The two tests above prove topology churn cannot clear a mark; this proves a
/// probe can, so a returning region is gated on connectivity rather than on
/// nothing at all.
///
/// Deliberately not `start_paused`: the probe cooldown is measured with
/// `std::time::Instant`, which virtual time does not advance, so a paused test
/// can never exercise this path. A zero `endpoint_unavailability_ttl` plus the
/// one-shot probe hook drives failback deterministically without waiting for the
/// 60-second background loop.
#[tokio::test]
async fn a_successful_probe_is_what_clears_an_unavailability_mark() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");

    // Short cooldown: long enough that the mark is still excluding traffic for
    // the first read, short enough that the endpoint becomes probe-due within
    // the test. The cooldown doubles as the routing-exclusion window, so it
    // cannot be zero.
    let mut operation_options = OperationOptions::default();
    operation_options.endpoint_unavailability_ttl = Some(Duration::from_millis(100));
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account())
                .with_preferred_regions(vec![Region::WEST_US, Region::EAST_US])
                .with_operation_options(operation_options)
                .build(),
        )
        .await
        .expect("driver should initialize");
    seed_item(&driver, "probe-item").await;

    assert!(driver.mark_region_endpoint_unavailable_for_testing(&Region::WEST_US));
    recorder.clear();
    read_item(&driver, "probe-item", OperationOptions::default())
        .await
        .expect("read should succeed while West is marked");
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == EAST_HOST),
        "test setup: a marked region must not take traffic; got {hosts:?}"
    );

    // Wall-clock wait: the cooldown is measured with `std::time::Instant`.
    tokio::time::sleep(Duration::from_millis(200)).await;
    driver.run_endpoint_probe_once_for_testing().await;
    assert!(
        !driver.is_endpoint_host_marked_unavailable_for_testing(WEST_HOST),
        "a successful probe must clear the mark -- otherwise failback is not \
         probe-gated, the region is simply stranded"
    );

    recorder.clear();
    read_item(&driver, "probe-item", OperationOptions::default())
        .await
        .expect("read should succeed after failback");
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|h| h == WEST_HOST),
        "after failback the restored top-preference region must take traffic again; \
         got {hosts:?}"
    );
}

// --- Write-region movement --------------------------------------------------

/// Writes follow a hub promotion, and the demoted hub stops receiving them.
#[tokio::test(start_paused = true)]
async fn writes_follow_the_promoted_hub_and_leave_the_demoted_one() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&driver, "hub-a").await;

    recorder.clear();
    write_item(&driver, "hub-b", OperationOptions::default())
        .await
        .expect("write should succeed on the original hub");
    assert!(
        recorder.data_plane_hosts().iter().all(|h| h == EAST_HOST),
        "test setup: writes should start on the original hub"
    );

    emulator
        .store()
        .set_write_region("West US")
        .expect("promotion should succeed");
    advance_past_refresh().await;

    recorder.clear();
    write_item(&driver, "hub-c", OperationOptions::default())
        .await
        .expect("write should succeed on the promoted hub");
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "writes must follow the promoted write region; got {hosts:?}"
    );
    assert!(
        hosts.last().map(String::as_str) == Some(WEST_HOST),
        "the successful write must land on the new hub; got {hosts:?}"
    );
}

/// Enabling multi-write lets writes stay in the caller's preferred region
/// instead of taking a cross-region hop to the hub.
#[tokio::test(start_paused = true)]
async fn enabling_multi_write_keeps_writes_local() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    // Prefer West, which is a read-only region while the account is single-write.
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "mw-local-a").await;

    recorder.clear();
    write_item(&driver, "mw-local-b", OperationOptions::default())
        .await
        .expect("write should succeed via the hub");
    assert!(
        recorder.data_plane_hosts().iter().any(|h| h == EAST_HOST),
        "test setup: a single-write account must send writes to the hub"
    );

    emulator.store().set_write_mode(WriteMode::Multi);
    advance_past_refresh().await;

    recorder.clear();
    write_item(&driver, "mw-local-c", OperationOptions::default())
        .await
        .expect("write should succeed locally under multi-write");
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().all(|h| h == WEST_HOST),
        "under multi-write the driver should write to its preferred region \
         rather than hopping to the hub; got {hosts:?}"
    );
}

// --- Interaction with caller options ----------------------------------------

/// `excluded_regions` keeps excluding a region after topology changes — a newly
/// added region must not silently become a routing target if the caller
/// excluded it.
#[tokio::test(start_paused = true)]
async fn excluded_regions_still_honored_after_a_region_is_added() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&driver, "excl-item").await;

    emulator
        .store()
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    advance_past_refresh().await;
    read_item(&driver, "excl-item", OperationOptions::default())
        .await
        .expect("warm-up read should succeed");

    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([Region::WEST_US]));

    recorder.clear();
    read_item(&driver, "excl-item", opts)
        .await
        .expect("read should succeed while excluding the newly added region");

    assert!(
        recorder.data_plane_hosts().iter().all(|h| h != WEST_HOST),
        "an excluded region must not be used even after it joins the account; got {:?}",
        recorder.data_plane_hosts()
    );
}

/// Session consistency keeps working across a topology change: a token obtained
/// before regions moved must not wedge subsequent reads.
///
/// The service makes this safe by bumping the token *version* on every
/// membership change, so an older token is superseded rather than compared
/// against a topology that no longer exists.
#[tokio::test(start_paused = true)]
async fn session_reads_keep_working_across_topology_changes() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder);
    let driver = build_driver(&emulator, vec![Region::EAST_US, Region::WEST_US]).await;
    seed_item(&driver, "sess-item").await;

    read_item(&driver, "sess-item", OperationOptions::default())
        .await
        .expect("test setup: read should succeed");

    let store = emulator.store();
    store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    advance_past_refresh().await;
    read_item(&driver, "sess-item", OperationOptions::default())
        .await
        .expect("session reads must survive a region being added");

    store
        .remove_region("West US")
        .expect("remove should succeed");
    advance_past_refresh().await;
    read_item(&driver, "sess-item", OperationOptions::default())
        .await
        .expect("session reads must survive a region being removed");

    // Writes still advance the session cleanly afterwards.
    write_item(&driver, "sess-item-2", OperationOptions::default())
        .await
        .expect("writes must still succeed after the topology settles");
    read_item(&driver, "sess-item-2", OperationOptions::default())
        .await
        .expect("read-your-write must hold after topology changes");
}

/// A region removed and re-added is usable again for reads — the driver must
/// not permanently blacklist it.
#[tokio::test(start_paused = true)]
async fn re_added_region_is_usable_for_reads_again() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east(), west()], WriteMode::Single, recorder.clone());
    let driver = build_driver(&emulator, vec![Region::WEST_US, Region::EAST_US]).await;
    seed_item(&driver, "readd-item").await;

    let store = emulator.store();
    store
        .remove_region("West US")
        .expect("remove should succeed");
    advance_past_refresh().await;
    read_item(&driver, "readd-item", OperationOptions::default())
        .await
        .expect("read should fall back to the surviving region");

    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("re-add should succeed");
    advance_past_refresh().await;

    recorder.clear();
    read_item(&driver, "readd-item", OperationOptions::default())
        .await
        .expect("read should succeed after the re-add");
    assert!(
        recorder.data_plane_hosts().iter().all(|h| h == WEST_HOST),
        "the re-added preferred region must be used again; got {:?}",
        recorder.data_plane_hosts()
    );
}

/// Removing every region except the hub leaves a working single-region account
/// rather than an unroutable one.
#[tokio::test(start_paused = true)]
async fn collapsing_to_a_single_region_keeps_the_account_usable() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(
        vec![east(), west(), central()],
        WriteMode::Single,
        recorder.clone(),
    );
    let driver = build_driver(&emulator, vec![Region::CENTRAL_US, Region::WEST_US]).await;
    seed_item(&driver, "collapse-item").await;

    let store = emulator.store();
    store
        .remove_region("Central US")
        .expect("remove should succeed");
    store
        .remove_region("West US")
        .expect("remove should succeed");
    advance_past_refresh().await;

    recorder.clear();
    read_item(&driver, "collapse-item", OperationOptions::default())
        .await
        .expect("reads must survive collapsing to one region");
    write_item(&driver, "collapse-item-2", OperationOptions::default())
        .await
        .expect("writes must survive collapsing to one region");

    assert!(
        recorder.data_plane_hosts().iter().all(|h| h == EAST_HOST),
        "with one region left everything must route there; got {:?}",
        recorder.data_plane_hosts()
    );
}

/// Growing from one region to three, then reading, must honor the caller's
/// preference order over the account's advertisement order.
#[tokio::test(start_paused = true)]
async fn preference_order_wins_over_advertisement_order_after_growth() {
    let recorder = HostRecorder::new();
    let emulator = build_emulator(vec![east()], WriteMode::Single, recorder.clone());
    // Prefer the region that will be added *last*.
    let driver = build_driver(&emulator, vec![Region::CENTRAL_US, Region::EAST_US]).await;
    seed_item(&driver, "grow-item").await;

    let store = emulator.store();
    store
        .add_region(west(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    store
        .add_region(central(), SeedingPolicy::Immediate)
        .expect("add should succeed");
    advance_past_refresh().await;
    read_item(&driver, "grow-item", OperationOptions::default())
        .await
        .expect("warm-up read should succeed");

    recorder.clear();
    read_item(&driver, "grow-item", OperationOptions::default())
        .await
        .expect("read should succeed");
    assert!(
        recorder
            .data_plane_hosts()
            .iter()
            .all(|h| h == CENTRAL_HOST),
        "the caller's first preferred region must win even though it was \
         advertised last; got {:?}",
        recorder.data_plane_hosts()
    );
}
