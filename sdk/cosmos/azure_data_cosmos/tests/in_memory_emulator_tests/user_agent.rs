// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests verifying that a configured [`UserAgentSuffix`] reaches
//! the wire on data-plane requests emitted by the SDK pipeline.
//!
//! Regression coverage for [PR #4368](https://github.com/Azure/azure-sdk-for-rust/pull/4368),
//! which fixed a bug where `CosmosClientBuilder::with_user_agent_suffix`
//! recorded the suffix in the SDK options but never forwarded it onto the
//! driver runtime, so the `User-Agent` header that hit the wire never
//! contained it. The unit test added by that PR only verifies the runtime
//! receives the suffix; this test verifies the suffix is observable on the
//! actual request the transport sees.
//!
//! The in-memory emulator (introduced by
//! [PR #4315](https://github.com/Azure/azure-sdk-for-rust/pull/4315))
//! intercepts every request the SDK pipeline produces, and a
//! [`RequestObserver`] captures the headers so the assertions run against
//! the real wire format rather than the runtime configuration.

use std::sync::{Arc, Mutex};

use azure_core::http::{headers::USER_AGENT, Method, Request, Url};
use azure_data_cosmos::regions::Region;
use azure_data_cosmos::{
    CosmosAccountReference, CosmosClientBuilder, RoutingStrategy, UserAgentSuffix,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use serde::{Deserialize, Serialize};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const EMULATOR_KEY: &str = "dGVzdGtleQ==";

#[derive(Debug, Serialize, Deserialize)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

/// Snapshot of a single request observed by the in-memory emulator. Captures
/// only what these tests assert on (method, URL, `User-Agent`) to keep the
/// helper minimal.
#[derive(Clone, Debug)]
struct RequestSnapshot {
    method: Method,
    url: Url,
    user_agent: Option<String>,
}

/// Minimal [`RequestObserver`] that records every request the emulator sees,
/// for tests that need to assert on outgoing request shape.
#[derive(Debug, Default)]
struct RecordingObserver {
    snapshots: Mutex<Vec<RequestSnapshot>>,
}

impl RecordingObserver {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn snapshots(&self) -> Vec<RequestSnapshot> {
        self.snapshots
            .lock()
            .expect("recording observer lock poisoned")
            .clone()
    }
}

impl RequestObserver for RecordingObserver {
    fn on_request(&self, request: &Request) {
        let snapshot = RequestSnapshot {
            method: request.method(),
            url: request.url().clone(),
            user_agent: request
                .headers()
                .get_optional_str(&USER_AGENT)
                .map(|s| s.to_owned()),
        };
        self.snapshots
            .lock()
            .expect("recording observer lock poisoned")
            .push(snapshot);
    }
}

fn build_emulator(observer: Arc<RecordingObserver>) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(observer))
}

/// Pre-provisions a database + container in the emulator store and builds a
/// `CosmosClient` (optionally with a `UserAgentSuffix`).
///
/// Returns `(client, db_name, container_name)`.
async fn build_client_with_provisioned_container(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    suffix: Option<UserAgentSuffix>,
) -> (azure_data_cosmos::CosmosClient, &'static str, &'static str) {
    let store = emulator.store();
    let db_name = "ua_db";
    let container_name = "ua_coll";

    store.create_database(db_name);
    store.create_container(
        db_name,
        container_name,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2,
        }))
        .unwrap(),
    );

    let mut builder = CosmosClientBuilder::new()
        .with_driver_runtime_builder(emulator.runtime_builder())
        .with_emulator_http_client(emulator.clone());
    if let Some(s) = suffix {
        builder = builder.with_user_agent_suffix(s);
    }

    let account = CosmosAccountReference::with_master_key(
        EMULATOR_GATEWAY_URL.parse().unwrap(),
        azure_core::credentials::Secret::new(EMULATOR_KEY),
    );

    let client = builder
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .expect("client builds");

    (client, db_name, container_name)
}

/// Builds a client and performs one `create_item` + `read_item` round-trip
/// so the observer captures the full data-plane path.
async fn perform_create_and_read(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    suffix: Option<UserAgentSuffix>,
) {
    let (client, db_name, container_name) =
        build_client_with_provisioned_container(emulator, suffix).await;

    let container = client
        .database_client(db_name)
        .container_client(container_name)
        .await
        .expect("container client");

    let item = TestItem {
        id: "ua-1".into(),
        pk: "pk1".into(),
        value: 1,
    };

    container
        .create_item("pk1", "ua-1", &item, None)
        .await
        .expect("create_item");

    container
        .read_item::<TestItem>("pk1", "ua-1", None)
        .await
        .expect("read_item");
}

/// Builds a client and performs container + database metadata reads so the
/// observer captures requests on both `/dbs/{db}` and
/// `/dbs/{db}/colls/{coll}` paths.
async fn perform_metadata_reads(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    suffix: Option<UserAgentSuffix>,
) {
    let (client, db_name, container_name) =
        build_client_with_provisioned_container(emulator, suffix).await;

    let database = client.database_client(db_name);
    let container = database
        .container_client(container_name)
        .await
        .expect("container client");

    database.read(None).await.expect("database read");
    container.read(None).await.expect("container read");
}

/// Returns `true` for captured requests targeting Cosmos DB **data-plane**
/// item operations.
///
/// In the Cosmos REST contract, item operations route through the
/// `/dbs/{db}/colls/{coll}/docs[/{id}]` path; control-plane operations
/// (database/container CRUD, throughput, etc.) target other paths. Filtering
/// on this segment lets the assertion narrow to the operations the test
/// actually invokes (`create_item`, `read_item`) instead of accepting any
/// request the SDK pipeline emits — the latter would mask a regression where
/// only SDK-pipeline / control-plane requests carried the suffix while item
/// operations did not.
fn is_item_data_plane_request(snap: &RequestSnapshot) -> bool {
    let mut segments = match snap.url.path_segments() {
        Some(s) => s,
        None => return false,
    };
    // Expect `dbs / {db} / colls / {coll} / docs [/ {id}]`.
    segments.next() == Some("dbs")
        && segments.next().is_some()
        && segments.next() == Some("colls")
        && segments.next().is_some()
        && segments.next() == Some("docs")
}

/// Returns `true` for captured requests targeting Cosmos DB **container or
/// database metadata reads** — i.e. `GET /dbs/{db}` or
/// `GET /dbs/{db}/colls/{coll}`.
///
/// These paths cover [`DatabaseClient::read`] and [`ContainerClient::read`]
/// (and the implicit container metadata fetch that
/// [`DatabaseClient::container_client`] performs to populate partition-key
/// info). Item operations under `/docs` and child resources like
/// `/colls/{coll}/sprocs` are deliberately excluded so the assertion stays
/// focused on metadata-only operations.
fn is_metadata_read_request(snap: &RequestSnapshot) -> bool {
    if snap.method != Method::Get {
        return false;
    }
    let Some(segments) = snap.url.path_segments() else {
        return false;
    };
    let segments: Vec<&str> = segments.filter(|s| !s.is_empty()).collect();
    matches!(segments.as_slice(), ["dbs", _] | ["dbs", _, "colls", _])
}

/// Verifies that a configured [`UserAgentSuffix`] actually appears in the
/// `User-Agent` header on data-plane requests emitted by the SDK pipeline.
///
/// Regression coverage for PR #4368: prior to that fix the suffix was
/// stored on the SDK builder but never forwarded to the driver runtime, so
/// the header that reached the wire never contained it. Asserting against
/// the captured request (rather than just the runtime configuration)
/// guards against any future regression in the same wiring.
#[tokio::test]
async fn user_agent_suffix_appears_on_data_plane_requests() {
    const SUFFIX: &str = "myapp-westus2";

    let observer = RecordingObserver::new();
    let emulator = build_emulator(observer.clone());

    perform_create_and_read(emulator, Some(UserAgentSuffix::new(SUFFIX))).await;

    let snapshots = observer.snapshots();
    let data_plane: Vec<&RequestSnapshot> = snapshots
        .iter()
        .filter(|s| is_item_data_plane_request(s))
        .collect();

    // The test invoked `create_item` (POST /docs) and `read_item`
    // (GET /docs/{id}); both must reach the emulator, otherwise the
    // assertion below would be vacuously true.
    assert!(
        data_plane.len() >= 2,
        "expected at least one create_item POST and one read_item GET to reach the emulator; \
         captured requests: {:?}",
        snapshots
            .iter()
            .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
            .collect::<Vec<_>>(),
    );

    // Every data-plane item request must carry the suffix. Asserting on all
    // (rather than at least one) catches regressions where only some code
    // paths forward the suffix.
    let missing: Vec<_> = data_plane
        .iter()
        .filter(|s| {
            !s.user_agent
                .as_deref()
                .is_some_and(|ua| ua.contains(SUFFIX))
        })
        .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
        .collect();
    assert!(
        missing.is_empty(),
        "expected every data-plane item request to carry user-agent suffix {SUFFIX:?}; \
         requests missing the suffix: {missing:?}",
    );
}

/// Negative control: without
/// [`CosmosClientBuilder::with_user_agent_suffix`], no captured data-plane
/// request should carry the suffix. This ensures the positive test above is
/// not passing because the suffix string happens to appear in some
/// unrelated part of the default `User-Agent` produced by the SDK.
#[tokio::test]
async fn no_user_agent_suffix_means_no_suffix_on_the_wire() {
    const SUFFIX: &str = "myapp-westus2";

    let observer = RecordingObserver::new();
    let emulator = build_emulator(observer.clone());

    perform_create_and_read(emulator, None).await;

    let snapshots = observer.snapshots();
    let data_plane: Vec<&RequestSnapshot> = snapshots
        .iter()
        .filter(|s| is_item_data_plane_request(s))
        .collect();

    // Sanity-check that the test actually exercised the data plane. If
    // `create_item`/`read_item` never reach the emulator the assertion
    // below is vacuously satisfied and would not catch a regression.
    assert!(
        data_plane.len() >= 2,
        "expected at least one create_item POST and one read_item GET to reach the emulator; \
         captured requests: {:?}",
        snapshots
            .iter()
            .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
            .collect::<Vec<_>>(),
    );

    for snap in &data_plane {
        let ua = snap.user_agent.as_deref().unwrap_or_else(|| {
            panic!(
                "data-plane request {:?} {} reached the emulator without a User-Agent header",
                snap.method, snap.url,
            )
        });
        assert!(
            !ua.contains(SUFFIX),
            "data-plane request {:?} {} unexpectedly carried suffix {SUFFIX:?} \
             in User-Agent {ua:?}",
            snap.method,
            snap.url,
        );
    }
}

/// Verifies that the configured [`UserAgentSuffix`] also reaches **metadata**
/// requests (database / container `read`), not just data-plane item
/// operations. The PR #4368 wiring bug affected the driver runtime's
/// `User-Agent` plumbing, which sits below every request type the SDK
/// pipeline emits — metadata reads are therefore an independent
/// observation point that prevents a future regression where the suffix
/// reaches data-plane requests but is dropped on metadata requests (or
/// vice-versa).
#[tokio::test]
async fn user_agent_suffix_appears_on_metadata_requests() {
    const SUFFIX: &str = "myapp-westus2";

    let observer = RecordingObserver::new();
    let emulator = build_emulator(observer.clone());

    perform_metadata_reads(emulator, Some(UserAgentSuffix::new(SUFFIX))).await;

    let snapshots = observer.snapshots();
    let metadata: Vec<&RequestSnapshot> = snapshots
        .iter()
        .filter(|s| is_metadata_read_request(s))
        .collect();

    // The test invoked `database.read()` (GET /dbs/{db}) and
    // `container.read()` (GET /dbs/{db}/colls/{coll}); both must reach
    // the emulator, otherwise the assertion below would be vacuously
    // true.
    assert!(
        metadata.len() >= 2,
        "expected at least one database read and one container read to reach the emulator; \
         captured requests: {:?}",
        snapshots
            .iter()
            .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
            .collect::<Vec<_>>(),
    );

    let missing: Vec<_> = metadata
        .iter()
        .filter(|s| {
            !s.user_agent
                .as_deref()
                .is_some_and(|ua| ua.contains(SUFFIX))
        })
        .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
        .collect();
    assert!(
        missing.is_empty(),
        "expected every metadata request to carry user-agent suffix {SUFFIX:?}; \
         requests missing the suffix: {missing:?}",
    );
}
