// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Public SDK coverage for service-backed account-topology edge cases.
//!
//! These tests intentionally drive the full `CosmosClient -> ContainerClient ->
//! CosmosDriver -> InMemoryEmulatorHttpClient` stack. The driver crate has more
//! focused payload/enforcement tests; this module proves the public client
//! observes the intended routing and recovery behavior.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use azure_core::http::headers::HeaderName;
use azure_core::http::{Request, Url};
use azure_data_cosmos::{
    options::{ItemReadOptions, Region},
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, RequestObserver,
    SeedingPolicy, VirtualAccountConfig, VirtualRegion, WriteMode,
};
use serde::{Deserialize, Serialize};

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
const CENTRAL_URL: &str = "https://centralus.emulator.local";
const EAST_HOST: &str = "eastus.emulator.local";
const WEST_HOST: &str = "westus.emulator.local";
const CENTRAL_HOST: &str = "centralus.emulator.local";
const DATABASE: &str = "topology-parity-db";
const CONTAINER: &str = "topology-parity-coll";

#[derive(Debug, Serialize, Deserialize)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

#[derive(Debug, Default)]
struct DataPlaneHostRecorder {
    hosts: Mutex<Vec<String>>,
    session_tokens: Mutex<Vec<Option<String>>>,
}

impl DataPlaneHostRecorder {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn clear(&self) {
        self.hosts.lock().unwrap().clear();
        self.session_tokens.lock().unwrap().clear();
    }

    fn hosts(&self) -> Vec<String> {
        self.hosts.lock().unwrap().clone()
    }

    fn session_tokens(&self) -> Vec<Option<String>> {
        self.session_tokens.lock().unwrap().clone()
    }
}

impl RequestObserver for DataPlaneHostRecorder {
    fn on_request(&self, request: &Request) {
        if !request.url().path().contains("/docs") {
            return;
        }
        if let Some(host) = request.url().host_str() {
            self.hosts.lock().unwrap().push(host.to_string());
        }
        self.session_tokens.lock().unwrap().push(
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-session-token"))
                .map(str::to_string),
        );
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

async fn build_client(
    config: VirtualAccountConfig,
    routing_region: Region,
    recorder: Arc<DataPlaneHostRecorder>,
) -> (CosmosClient, Arc<InMemoryEmulatorHttpClient>) {
    build_client_at(config, EAST_URL, routing_region, recorder).await
}

async fn build_client_at(
    config: VirtualAccountConfig,
    account_endpoint: &str,
    routing_region: Region,
    recorder: Arc<DataPlaneHostRecorder>,
) -> (CosmosClient, Arc<InMemoryEmulatorHttpClient>) {
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(recorder));
    let store = emulator.store();
    store.create_database(DATABASE);
    store.create_container(
        DATABASE,
        CONTAINER,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2,
        }))
        .unwrap(),
    );

    let client = create_client(
        &emulator,
        account_endpoint,
        RoutingStrategy::ProximityTo(routing_region),
    )
    .await;
    (client, emulator)
}

async fn create_client(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    account_endpoint: &str,
    strategy: RoutingStrategy,
) -> CosmosClient {
    let runtime = CosmosRuntimeBuilder::from(emulator.runtime_builder())
        .build()
        .await
        .expect("runtime builds");
    let account = AccountReference::with_authentication_key(
        account_endpoint.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(runtime)
        .build(account, strategy)
        .await
        .expect("client builds");
    client
}

async fn container(client: &CosmosClient) -> azure_data_cosmos::ContainerClient {
    client
        .database_client(DATABASE)
        .container_client(CONTAINER, None)
        .await
        .expect("container resolves")
}

async fn create_item(client: &CosmosClient, id: &str) {
    container(client)
        .await
        .create_item(
            "pk1",
            id,
            &TestItem {
                id: id.to_string(),
                pk: "pk1".to_string(),
                value: 1,
            },
            None,
        )
        .await
        .expect("public SDK create succeeds");
}

async fn read_item(client: &CosmosClient, id: &str) {
    container(client)
        .await
        .read_item("pk1", id, None)
        .await
        .expect("public SDK read succeeds");
}

/// Strong keeps SDK writes on the hub even when the account's multi-write flag
/// is true and the caller asks for proximity to West.
#[tokio::test(start_paused = true)]
async fn strong_multi_write_routes_public_sdk_to_hub() {
    let recorder = DataPlaneHostRecorder::new();
    let config = VirtualAccountConfig::new(vec![east(), west()])
        .unwrap()
        .with_write_mode(WriteMode::Multi)
        .with_consistency(ConsistencyLevel::Strong)
        .with_replication_config(ReplicationConfig::immediate());
    // Bootstrap through West (not the expected destination) so an
    // implementation that blindly writes to the account endpoint would fail.
    let (client, _) = build_client_at(config, WEST_URL, Region::WEST_US, recorder.clone()).await;

    recorder.clear();
    create_item(&client, "strong-public").await;
    let hosts = recorder.hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == EAST_HOST),
        "Strong must keep public SDK writes on the hub; observed {hosts:?}"
    );
}

/// The public client recovers when its preferred multi-write satellite remains
/// advertised but rejects writes due to local write revocation.
#[tokio::test(start_paused = true)]
async fn revoked_satellite_public_sdk_retries_healthy_region() {
    let recorder = DataPlaneHostRecorder::new();
    let config = VirtualAccountConfig::new(vec![east(), west()])
        .unwrap()
        .with_write_mode(WriteMode::Multi)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let (client, emulator) = build_client(config, Region::WEST_US, recorder.clone()).await;
    emulator.store().revoke_region_write("West US").unwrap();

    recorder.clear();
    create_item(&client, "revoked-public").await;
    let hosts = recorder.hosts();
    assert!(
        hosts.iter().any(|host| host == WEST_HOST),
        "the test must exercise the revoked preferred satellite; observed {hosts:?}"
    );
    assert!(
        hosts.iter().any(|host| host == EAST_HOST),
        "the public SDK must retry a healthy region; observed {hosts:?}"
    );
}

/// The public client ignores a hidden buildout region until a background
/// account refresh observes it as ready, then routes to the preferred region.
#[tokio::test(start_paused = true)]
async fn hidden_region_public_sdk_adopts_only_when_ready() {
    let recorder = DataPlaneHostRecorder::new();
    let config = VirtualAccountConfig::new(vec![east()])
        .unwrap()
        .with_write_mode(WriteMode::Multi)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let (client, emulator) = build_client(config, Region::WEST_US, recorder.clone()).await;
    create_item(&client, "hidden-public").await;

    emulator
        .store()
        .add_region(
            west(),
            SeedingPolicy::HiddenUntilReady(Duration::from_secs(1_800)),
        )
        .unwrap();

    tokio::time::sleep(Duration::from_secs(600)).await;
    recorder.clear();
    read_item(&client, "hidden-public").await;
    let hidden_hosts = recorder.hosts();
    assert!(
        !hidden_hosts.is_empty() && hidden_hosts.iter().all(|host| host == EAST_HOST),
        "the public SDK must not route to a hidden region; observed {hidden_hosts:?}"
    );

    tokio::time::sleep(Duration::from_secs(1_801)).await;
    tokio::time::sleep(Duration::from_secs(600)).await;
    recorder.clear();
    read_item(&client, "hidden-public").await;
    let ready_hosts = recorder.hosts();
    assert!(
        !ready_hosts.is_empty() && ready_hosts.iter().all(|host| host == WEST_HOST),
        "the public SDK should adopt West after buildout; observed {ready_hosts:?}"
    );
}

/// New public clients initialize correctly while a preferred region is already
/// offline and after it returns. Each client has a fresh runtime/driver and
/// therefore cannot inherit topology, failback marks, or probes.
#[tokio::test(start_paused = true)]
async fn cold_restart_public_sdk_routes_offline_then_restored_preference() {
    let recorder = DataPlaneHostRecorder::new();
    let config = VirtualAccountConfig::new(vec![east(), west(), central()])
        .unwrap()
        .with_write_mode(WriteMode::Single)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let (initial, emulator) = build_client(config, Region::WEST_US, recorder.clone()).await;
    create_item(&initial, "cold-public").await;

    emulator.store().set_region_offline("West US").unwrap();
    let cold_offline = create_client(
        &emulator,
        EAST_URL,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::CENTRAL_US]),
    )
    .await;
    recorder.clear();
    read_item(&cold_offline, "cold-public").await;
    let hosts = recorder.hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == CENTRAL_HOST),
        "a cold public client must start on the next preferred region; observed {hosts:?}"
    );

    emulator.store().set_region_online("West US").unwrap();
    let cold_online = create_client(
        &emulator,
        EAST_URL,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::CENTRAL_US]),
    )
    .await;
    recorder.clear();
    read_item(&cold_online, "cold-public").await;
    let hosts = recorder.hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == WEST_HOST),
        "a cold public client must immediately restore the top preference; observed {hosts:?}"
    );
}

/// A session token issued before a topology change remains valid when supplied
/// explicitly to a completely fresh public client after the issuing region is
/// offline.
#[tokio::test(start_paused = true)]
async fn session_token_survives_offline_across_public_client_restart() {
    let recorder = DataPlaneHostRecorder::new();
    let config = VirtualAccountConfig::new(vec![east(), west(), central()])
        .unwrap()
        .with_write_mode(WriteMode::Single)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let (initial, emulator) = build_client(config, Region::WEST_US, recorder.clone()).await;
    create_item(&initial, "restart-session-token").await;

    recorder.clear();
    let response = container(&initial)
        .await
        .read_item("pk1", "restart-session-token", None)
        .await
        .expect("initial West read succeeds");
    let issuing_hosts = recorder.hosts();
    assert!(
        !issuing_hosts.is_empty() && issuing_hosts.iter().all(|host| host == WEST_HOST),
        "the token must be issued by pre-offline West; observed {issuing_hosts:?}"
    );
    let token = response
        .headers()
        .session_token()
        .expect("session read returns a token")
        .as_str()
        .to_string();

    emulator.store().set_region_offline("West US").unwrap();
    let restarted = create_client(
        &emulator,
        EAST_URL,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::CENTRAL_US]),
    )
    .await;
    recorder.clear();
    container(&restarted)
        .await
        .read_item(
            "pk1",
            "restart-session-token",
            Some(ItemReadOptions::default().with_session_token(token.clone())),
        )
        .await
        .expect("fresh client must accept the pre-offline session token on Central");
    let hosts = recorder.hosts();
    assert!(
        !hosts.is_empty() && hosts.iter().all(|host| host == CENTRAL_HOST),
        "fresh client must use Central with the pre-offline token; observed {hosts:?}"
    );
    let sent_tokens = recorder.session_tokens();
    assert!(
        !sent_tokens.is_empty()
            && sent_tokens
                .iter()
                .all(|sent| sent.as_deref() == Some(token.as_str())),
        "fresh client must transmit the exact pre-offline token; observed {sent_tokens:?}"
    );
}
