// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end coverage that a registered [`DiagnosticsHandler`] actually receives
//! a completed context through a **real** SDK operation — driven by the in-memory
//! emulator. This guards the completion seams (singleton success + failure, and
//! paginated success) against wiring regressions that still compile.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use azure_core::http::Context;
use azure_data_cosmos::diagnostics::{
    CosmosOperationContext, DiagnosticsContext, DiagnosticsHandler,
};
use azure_data_cosmos::options::Region;
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    FeedScope, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestDoc {
    id: String,
    pk: String,
    value: i64,
}

/// The operation-scope identity (`CosmosOperationContext`) a handler observed on
/// its most recent invocation.
#[derive(Clone, Debug, Default, PartialEq)]
struct ObservedOp {
    operation_name: Option<String>,
    database_name: Option<String>,
    container_name: Option<String>,
}

/// A [`DiagnosticsHandler`] that counts invocations, how many carried a failed
/// context, and the operation-scope identity of the latest invocation — so a test
/// can assert the chain fired on both success and failure *and* that the correct
/// operation/database/container identity (WS8) was propagated.
#[derive(Default)]
struct CountingHandler {
    total: AtomicUsize,
    failures: AtomicUsize,
    last_op: Mutex<Option<ObservedOp>>,
}

impl CountingHandler {
    fn total(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }

    fn failures(&self) -> usize {
        self.failures.load(Ordering::SeqCst)
    }

    /// The operation identity observed on the most recent invocation, or `None`
    /// when the handler was invoked without a `CosmosOperationContext`.
    fn last_op(&self) -> Option<ObservedOp> {
        self.last_op.lock().unwrap().clone()
    }
}

impl DiagnosticsHandler for CountingHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        self.total.fetch_add(1, Ordering::SeqCst);
        if diagnostics.is_failure() {
            self.failures.fetch_add(1, Ordering::SeqCst);
        }
        // Capture the SDK-supplied operation identity carried on the pipeline
        // context so a test can assert the WS8 `db.*` wiring (operation name,
        // database, container) actually reaches handlers. Store the observed
        // value verbatim (including `None`) so missing wiring is detectable.
        let observed = cx.value::<CosmosOperationContext>().map(|op| ObservedOp {
            operation_name: op.operation_name().map(str::to_owned),
            database_name: op.database_name().map(str::to_owned),
            container_name: op.container_name().map(str::to_owned),
        });
        *self.last_op.lock().unwrap() = observed;
    }
}

/// Builds an emulator-backed SDK client with `handler` registered and provisions
/// an empty `(database, container)` keyed on `/pk`.
async fn setup() -> (CosmosClient, Arc<CountingHandler>, String, String) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();

    let db = format!("diag-{}", &Uuid::new_v4().to_string()[..8]);
    let container = "items".to_string();
    store.create_database(&db);
    store.create_container(
        &db,
        &container,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );

    let handler = Arc::new(CountingHandler::default());
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .with_diagnostics_handler(handler.clone())
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();

    (client, handler, db, container)
}

#[tokio::test]
async fn handler_receives_singleton_success() {
    let (client, handler, db, container) = setup().await;
    let c = client
        .database_client(&db)
        .container_client(&container)
        .await
        .unwrap();

    let before = handler.total();
    c.create_item(
        "pkA",
        "doc-1",
        &TestDoc {
            id: "doc-1".into(),
            pk: "pkA".into(),
            value: 1,
        },
        None,
    )
    .await
    .unwrap();

    assert_eq!(
        handler.total(),
        before + 1,
        "a singleton success must dispatch exactly one completion callback"
    );
    assert_eq!(
        handler.last_op(),
        Some(ObservedOp {
            operation_name: Some("create_item".to_string()),
            database_name: Some(db.clone()),
            container_name: Some(container.clone()),
        }),
        "the create_item operation must propagate its db.* identity to handlers"
    );
}

#[tokio::test]
async fn handler_receives_singleton_failure() {
    let (client, handler, db, container) = setup().await;
    let c = client
        .database_client(&db)
        .container_client(&container)
        .await
        .unwrap();

    let before_total = handler.total();
    let before_failures = handler.failures();

    let result = c.read_item("pkMissing", "does-not-exist", None).await;
    assert!(result.is_err(), "reading a missing item must fail");

    assert_eq!(
        handler.total(),
        before_total + 1,
        "a singleton failure must dispatch exactly one completion callback"
    );
    assert_eq!(
        handler.failures(),
        before_failures + 1,
        "the failed operation must be dispatched exactly once with a failed context"
    );
    assert_eq!(
        handler.last_op().and_then(|op| op.operation_name),
        Some("read_item".to_string()),
        "the failed read_item must still propagate its operation identity"
    );
}

#[tokio::test]
async fn handler_receives_paginated_success() {
    let (client, handler, db, container) = setup().await;
    let c = client
        .database_client(&db)
        .container_client(&container)
        .await
        .unwrap();

    for i in 0..3 {
        let id = format!("q-{i}");
        c.create_item(
            "pkQ",
            &id,
            &TestDoc {
                id: id.clone(),
                pk: "pkQ".into(),
                value: i,
            },
            None,
        )
        .await
        .unwrap();
    }

    let before = handler.total();
    let items: Vec<TestDoc> = c
        .query_items(
            Query::from("SELECT * FROM c"),
            FeedScope::partition("pkQ"),
            None,
        )
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    assert_eq!(items.len(), 3);
    assert_eq!(
        handler.total(),
        before + 1,
        "a single query page must dispatch exactly one completion callback"
    );
    assert_eq!(
        handler.last_op(),
        Some(ObservedOp {
            operation_name: Some("query_items".to_string()),
            database_name: Some(db.clone()),
            container_name: Some(container.clone()),
        }),
        "the query_items operation must propagate its db.* identity to handlers"
    );
}

/// The paginated **failure** dispatch seam — a page fetch that errors — must fire
/// the handler exactly once with a failed context. This is a distinct completion
/// seam from the paginated-success and singleton paths and can regress
/// independently, so it gets its own emulator-driven coverage.
#[tokio::test]
async fn handler_receives_paginated_failure() {
    let (client, handler, db, container) = setup().await;
    let c = client
        .database_client(&db)
        .container_client(&container)
        .await
        .unwrap();

    let before_total = handler.total();
    let before_failures = handler.failures();

    // A syntactically invalid query is rejected by the emulator with a terminal
    // (non-retryable) 400 BadRequest, so the first page fetch errors instead of
    // returning a page — exercising the iterator's failure dispatch branch.
    let result: Result<Vec<TestDoc>, _> = c
        .query_items(
            Query::from("SELECT * FROM c WHERE"),
            FeedScope::partition("pkFail"),
            None,
        )
        .await
        .unwrap()
        .try_collect()
        .await;
    assert!(
        result.is_err(),
        "the invalid query must surface as a terminal page-fetch error"
    );

    assert_eq!(
        handler.total(),
        before_total + 1,
        "a failed query page must dispatch exactly one completion callback"
    );
    assert_eq!(
        handler.failures(),
        before_failures + 1,
        "the failed page fetch must be dispatched exactly once with a failed context"
    );
}
