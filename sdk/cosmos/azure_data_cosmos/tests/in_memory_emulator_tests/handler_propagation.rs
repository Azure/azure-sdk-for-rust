// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end coverage that a registered [`DiagnosticsHandler`] actually receives
//! a completed context through a **real** SDK operation — driven by the in-memory
//! emulator. This guards the completion seams (singleton success + failure, and
//! paginated success) against wiring regressions that still compile.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use azure_core::http::Context;
use azure_data_cosmos::diagnostics::{DiagnosticsContext, DiagnosticsHandler};
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

/// A [`DiagnosticsHandler`] that counts invocations and how many carried a failed
/// context, so a test can assert the chain fired on both success and failure.
#[derive(Default)]
struct CountingHandler {
    total: AtomicUsize,
    failures: AtomicUsize,
}

impl CountingHandler {
    fn total(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }

    fn failures(&self) -> usize {
        self.failures.load(Ordering::SeqCst)
    }
}

impl DiagnosticsHandler for CountingHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, _cx: &Context<'_>) {
        self.total.fetch_add(1, Ordering::SeqCst);
        if diagnostics.is_failure() {
            self.failures.fetch_add(1, Ordering::SeqCst);
        }
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

    assert!(
        handler.total() > before,
        "the handler chain must fire on a singleton success"
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

    assert!(
        handler.total() > before_total,
        "the handler chain must fire on a singleton failure"
    );
    assert!(
        handler.failures() > before_failures,
        "the failed operation must be dispatched with a failed context"
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
    assert!(
        handler.total() > before,
        "the handler chain must fire for query pages"
    );
}
