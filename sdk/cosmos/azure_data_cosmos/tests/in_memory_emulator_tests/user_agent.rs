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

/// Pre-provisions a database + container in the emulator store, builds a
/// `CosmosClient` (optionally with a `UserAgentSuffix`), and performs one
/// `create_item` + `read_item` round-trip so the observer captures both
/// control-plane and data-plane requests.
async fn perform_create_and_read(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    suffix: Option<UserAgentSuffix>,
) {
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
    assert!(
        !snapshots.is_empty(),
        "observer should have captured at least one request"
    );

    let with_suffix = snapshots
        .iter()
        .filter(|s| {
            s.user_agent
                .as_deref()
                .is_some_and(|ua| ua.contains(SUFFIX))
        })
        .count();

    assert!(
        with_suffix > 0,
        "expected at least one captured request to carry the user-agent suffix {SUFFIX:?}; \
         captured user-agents: {:?}",
        snapshots
            .iter()
            .map(|s| (s.method, s.url.as_str(), s.user_agent.as_deref()))
            .collect::<Vec<_>>(),
    );
}

/// Negative control: without
/// [`CosmosClientBuilder::with_user_agent_suffix`], no captured request
/// should carry the suffix. This ensures the positive test above is not
/// passing because the suffix string happens to appear in some unrelated
/// part of the default `User-Agent`.
#[tokio::test]
async fn no_user_agent_suffix_means_no_suffix_on_the_wire() {
    const SUFFIX: &str = "myapp-westus2";

    let observer = RecordingObserver::new();
    let emulator = build_emulator(observer.clone());

    perform_create_and_read(emulator, None).await;

    let snapshots = observer.snapshots();
    assert!(
        !snapshots.is_empty(),
        "observer should have captured at least one request"
    );

    for snap in &snapshots {
        if let Some(ua) = snap.user_agent.as_deref() {
            assert!(
                !ua.contains(SUFFIX),
                "request {:?} {} unexpectedly carried suffix {SUFFIX:?} in User-Agent {ua:?}",
                snap.method,
                snap.url,
            );
        }
    }
}
