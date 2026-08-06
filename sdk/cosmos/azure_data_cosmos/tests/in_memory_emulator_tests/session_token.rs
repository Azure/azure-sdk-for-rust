// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end verification of **session-token flow** under Session consistency.
//!
//! These tests drive the full driver pipeline against the in-memory emulator
//! and use a [`RequestObserver`] to inspect the **outgoing** `x-ms-session-token`
//! header on each request the transport actually sees. They prove two halves of
//! the session-token contract:
//!
//! 1. **Capture** — write responses carry a session token, and the driver's
//!    [`SessionContainer`] cache is updated with it (and advances as later
//!    writes arrive).
//! 2. **Resolve** — subsequent **read** requests carry the cached token on the
//!    wire when (and only when) Session consistency is effective.
//!
//! The negative controls (Eventual consistency, session-capturing disabled,
//! empty cache) confirm the driver does *not* attach a token when it should
//! not, and the precedence test confirms a user-supplied token wins over the
//! cache verbatim (no merge).
//!
//! Unlike [`super::driver_end_to_end`], these tests assert on the request wire
//! shape rather than only response headers, so they are emulator-only (the
//! observer hook is an emulator construct). The same behavior is exercised
//! against a real account by the dual-backend tests, which compare response
//! session tokens between backends.

use std::sync::{Arc, Mutex};

use azure_core::http::{headers::HeaderName, Method, Request, Url};
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, ContainerReference, CosmosOperation, ItemReference, PartitionKey},
    options::{DriverOptions, OperationOptionsBuilder, ReadConsistencyStrategy},
    CosmosDriver,
};
use serde::{Deserialize, Serialize};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const EMULATOR_KEY: &str = "dGVzdGtleQ==";

static SESSION_TOKEN_HEADER: HeaderName = HeaderName::from_static("x-ms-session-token");

#[derive(Debug, Serialize, Deserialize)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
}

/// Snapshot of a single request observed by the emulator, capturing the method,
/// URL, and outgoing `x-ms-session-token` header (if any).
#[derive(Clone, Debug)]
struct RequestSnapshot {
    method: Method,
    url: Url,
    session_token: Option<String>,
}

impl RequestSnapshot {
    /// Returns `true` when this request targets the item data-plane path
    /// `dbs/{db}/colls/{coll}/docs[/{id}]`.
    fn is_item_request(&self) -> bool {
        let mut segments = match self.url.path_segments() {
            Some(s) => s,
            None => return false,
        };
        segments.next() == Some("dbs")
            && segments.next().is_some()
            && segments.next() == Some("colls")
            && segments.next().is_some()
            && segments.next() == Some("docs")
    }

    /// Returns `true` for a point read (`GET`) of an item.
    fn is_item_read(&self) -> bool {
        self.method == Method::Get && self.is_item_request()
    }
}

/// [`RequestObserver`] that records every request the emulator sees so tests can
/// assert on the outgoing session token. Supports draining so a test can reset
/// the log between phases.
#[derive(Debug, Default)]
struct RecordingObserver {
    snapshots: Mutex<Vec<RequestSnapshot>>,
}

impl RecordingObserver {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Returns a clone of all recorded snapshots without clearing them.
    fn snapshots(&self) -> Vec<RequestSnapshot> {
        self.snapshots
            .lock()
            .expect("recording observer lock poisoned")
            .clone()
    }

    /// Drains and discards all recorded snapshots, resetting the log.
    fn clear(&self) {
        self.snapshots
            .lock()
            .expect("recording observer lock poisoned")
            .clear();
    }

    /// Returns the single item read recorded since the last [`Self::clear`],
    /// panicking if there is not exactly one.
    fn single_item_read(&self) -> RequestSnapshot {
        let reads: Vec<RequestSnapshot> = self
            .snapshots()
            .into_iter()
            .filter(RequestSnapshot::is_item_read)
            .collect();
        assert_eq!(
            reads.len(),
            1,
            "expected exactly one item read, observed: {reads:#?}"
        );
        reads.into_iter().next().unwrap()
    }
}

impl RequestObserver for RecordingObserver {
    fn on_request(&self, request: &Request) {
        let snapshot = RequestSnapshot {
            method: request.method(),
            url: request.url().clone(),
            session_token: request
                .headers()
                .get_optional_str(&SESSION_TOKEN_HEADER)
                .map(|s| s.to_owned()),
        };
        self.snapshots
            .lock()
            .expect("recording observer lock poisoned")
            .push(snapshot);
    }
}

/// Parses the global LSN out of a V2 session token of the form
/// `{pkrange}:{version}#{globalLsn}#{region}={localLsn}` (or the bare V1 form
/// `{pkrange}:{lsn}`).
pub(crate) fn global_lsn(token: &str) -> u64 {
    let after_pkrange = token.split_once(':').map(|(_, rest)| rest).unwrap_or(token);
    let mut parts = after_pkrange.split('#');
    let first = parts.next().unwrap_or_default();
    // V2: version#globalLsn#... → the global LSN is the second `#` segment.
    // V1: just the LSN → it is the first segment.
    match parts.next() {
        Some(global) => global.parse().unwrap_or(0),
        None => first.parse().unwrap_or(0),
    }
}

/// Test harness: an emulator wired to a recording observer plus a driver and a
/// resolved container, all under Session consistency.
struct Harness {
    driver: Arc<CosmosDriver>,
    observer: Arc<RecordingObserver>,
    container: ContainerReference,
}

impl Harness {
    async fn setup() -> Self {
        let observer = RecordingObserver::new();

        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
        )])
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);

        let emulator = Arc::new(
            InMemoryEmulatorHttpClient::new(config).with_request_observer(observer.clone()),
        );

        let db_name = "session_db";
        let container_name = "session_coll";
        let store = emulator.store();
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

        let runtime = emulator.runtime_builder().build().await.unwrap();
        let account = AccountReference::with_master_key(
            Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
            EMULATOR_KEY,
        );
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await
            .unwrap();

        let container = driver
            .resolve_container(db_name, container_name)
            .await
            .unwrap();

        Self {
            driver,
            observer,
            container,
        }
    }

    fn item_ref(&self, pk: &str, id: &str) -> ItemReference {
        ItemReference::from_name(
            &self.container,
            PartitionKey::from(pk.to_string()),
            id.to_string(),
        )
    }

    /// Creates an item and returns the session token from the response.
    async fn create(&self, pk: &str, id: &str, value: i64) -> Option<String> {
        let body = serde_json::to_vec(&TestItem {
            id: id.to_string(),
            pk: pk.to_string(),
            value,
        })
        .unwrap();
        let resp = self
            .driver
            .execute_singleton_operation(
                CosmosOperation::create_item(self.item_ref(pk, id)).with_body(body),
                OperationOptionsBuilder::new().build(),
            )
            .await
            .expect("create_item should succeed");
        resp.headers()
            .session_token
            .as_ref()
            .map(|t| t.as_str().to_string())
    }

    /// Replaces an item and returns the session token from the response.
    async fn replace(&self, pk: &str, id: &str, value: i64) -> Option<String> {
        let body = serde_json::to_vec(&TestItem {
            id: id.to_string(),
            pk: pk.to_string(),
            value,
        })
        .unwrap();
        let resp = self
            .driver
            .execute_singleton_operation(
                CosmosOperation::replace_item(self.item_ref(pk, id)).with_body(body),
                OperationOptionsBuilder::new().build(),
            )
            .await
            .expect("replace_item should succeed");
        resp.headers()
            .session_token
            .as_ref()
            .map(|t| t.as_str().to_string())
    }
}

/// A read that follows a create must carry the **cached** session token equal to
/// the create response's token, while the create write itself carries **no**
/// token (the cache was empty when it was issued).
#[tokio::test]
async fn read_after_create_sends_cached_session_token() {
    let h = Harness::setup().await;

    h.observer.clear();
    let create_token = h
        .create("pk1", "item-1", 1)
        .await
        .expect("create should return a session token");

    // The create write went out with an empty cache → no outgoing token.
    let create_writes: Vec<RequestSnapshot> = h
        .observer
        .snapshots()
        .into_iter()
        .filter(|s| s.is_item_request() && s.method == Method::Post)
        .collect();
    assert_eq!(create_writes.len(), 1, "expected exactly one create POST");
    assert_eq!(
        create_writes[0].session_token, None,
        "create issued against an empty cache must not carry a session token"
    );

    // Now read the item back; it must carry the token captured from the create.
    h.observer.clear();
    h.driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "item-1")),
            OperationOptionsBuilder::new().build(),
        )
        .await
        .expect("read_item should succeed");

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token.as_deref(),
        Some(create_token.as_str()),
        "read must carry the session token captured from the create response"
    );
}

/// As successive writes return higher tokens, the cache must advance so that a
/// later read carries the **latest** token (not an earlier one).
#[tokio::test]
async fn cache_advances_as_write_responses_arrive() {
    let h = Harness::setup().await;

    let create_token = h.create("pk1", "item-1", 1).await.expect("create token");
    let replace_token = h.replace("pk1", "item-1", 2).await.expect("replace token");

    assert!(
        global_lsn(&replace_token) >= global_lsn(&create_token),
        "replace token global LSN ({replace_token}) should be >= create token ({create_token})"
    );

    h.observer.clear();
    h.driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "item-1")),
            OperationOptionsBuilder::new().build(),
        )
        .await
        .expect("read_item should succeed");

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token.as_deref(),
        Some(replace_token.as_str()),
        "read must carry the latest cached token, after the replace advanced it"
    );
}

/// A read issued with `ReadConsistencyStrategy::Eventual` must **not** carry a
/// session token even though the cache holds one — session consistency is not
/// effective for that request.
#[tokio::test]
async fn eventual_read_omits_session_token() {
    let h = Harness::setup().await;

    h.create("pk1", "item-1", 1)
        .await
        .expect("create should populate cache");

    h.observer.clear();
    h.driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "item-1")),
            OperationOptionsBuilder::new()
                .with_read_consistency_strategy(ReadConsistencyStrategy::Eventual)
                .build(),
        )
        .await
        .expect("eventual read_item should succeed");

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token, None,
        "an Eventual read must not carry a session token"
    );
}

/// A read issued with session capturing disabled must **not** carry a session
/// token: disabling session management gates both capture and resolve.
#[tokio::test]
async fn session_capturing_disabled_omits_session_token() {
    let h = Harness::setup().await;

    h.create("pk1", "item-1", 1)
        .await
        .expect("create should populate cache");

    h.observer.clear();
    h.driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "item-1")),
            OperationOptionsBuilder::new()
                .with_session_capturing_disabled(true)
                .build(),
        )
        .await
        .expect("read_item with session capturing disabled should succeed");

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token, None,
        "a read with session capturing disabled must not carry a session token"
    );
}

/// A user-supplied session token on the operation must be sent **verbatim**,
/// taking precedence over the (newer) cached token with no merge.
#[tokio::test]
async fn user_supplied_session_token_takes_precedence() {
    let h = Harness::setup().await;

    // create → T1, replace → T2 (T2 has a strictly higher LSN; it is what the
    // cache holds). We then read with an explicit T1 and expect T1 on the wire.
    let create_token = h.create("pk1", "item-1", 1).await.expect("create token");
    let replace_token = h.replace("pk1", "item-1", 2).await.expect("replace token");
    assert!(
        global_lsn(&replace_token) > global_lsn(&create_token),
        "precondition: replace must advance the LSN ({create_token} -> {replace_token})"
    );

    h.observer.clear();
    h.driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "item-1"))
                .with_session_token(create_token.clone()),
            OperationOptionsBuilder::new().build(),
        )
        .await
        .expect("read_item should succeed");

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token.as_deref(),
        Some(create_token.as_str()),
        "user-supplied token must be sent verbatim, not the newer cached token"
    );
}

/// A read issued before any write — i.e. against an empty cache — must carry no
/// session token.
#[tokio::test]
async fn read_before_any_write_has_no_session_token() {
    let h = Harness::setup().await;

    h.observer.clear();
    // The item does not exist, so the read returns 404; we only assert on the
    // outgoing header, which must be absent because the cache is empty.
    let _ = h
        .driver
        .execute_singleton_operation(
            CosmosOperation::read_item(h.item_ref("pk1", "missing")),
            OperationOptionsBuilder::new()
                .with_max_session_retry_count(0)
                .build(),
        )
        .await;

    let read = h.observer.single_item_read();
    assert_eq!(
        read.session_token, None,
        "a read against an empty session cache must not carry a token"
    );
}
