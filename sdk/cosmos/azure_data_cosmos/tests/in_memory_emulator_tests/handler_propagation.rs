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
use azure_data_cosmos::options::{DiagnosticsVerbosity, Region};
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
    last_diagnostics: Mutex<Option<serde_json::Value>>,
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
        *self.last_diagnostics.lock().unwrap() = Some(
            serde_json::from_str(diagnostics.to_json_string(Some(DiagnosticsVerbosity::Detailed)))
                .unwrap(),
        );
        self.total.fetch_add(1, Ordering::SeqCst);
        if diagnostics.is_failure() {
            self.failures.fetch_add(1, Ordering::SeqCst);
        }
        let observed = cx.value::<CosmosOperationContext>().map(|op| ObservedOp {
            operation_name: op.operation_name().map(str::to_owned),
            database_name: op.database_name().map(str::to_owned),
            container_name: op.container_name().map(str::to_owned),
        });
        *self.last_op.lock().unwrap() = observed;
    }
}

#[cfg(feature = "fault_injection")]
#[tokio::test(start_paused = true)]
async fn change_feed_split_exhaustion_reaches_sdk_error_and_handler() {
    use azure_data_cosmos::options::ChangeFeedStartFrom;
    use azure_data_cosmos_driver::{
        error::status_codes,
        fault_injection::{
            FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
            FaultInjectionRuleBuilder, FaultOperationType,
        },
    };

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
        VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
        )])
        .unwrap(),
    ));
    emulator.store().create_database("split-diagnostics");
    emulator.store().create_container(
        "split-diagnostics",
        "items",
        serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))
            .unwrap(),
    );
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "sdk-split-exhaustion",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::PartitionIsGone)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ChangeFeedItem)
                .build(),
        )
        .build(),
    );
    let runtime =
        CosmosRuntimeBuilder::from(emulator.runtime_builder_with_fault_rules(vec![rule.clone()]))
            .build()
            .await
            .unwrap();
    let handler = Arc::new(CountingHandler::default());
    let client = CosmosClientBuilder::new()
        .with_runtime(runtime)
        .with_diagnostics_handler(handler.clone())
        .build(
            AccountReference::with_authentication_key(
                EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
                azure_core::credentials::Secret::new("dGVzdGtleQ=="),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await
        .unwrap();
    let container = client
        .database_client("split-diagnostics")
        .container_client("items", None)
        .await
        .unwrap();
    let ranges = container.read_feed_ranges(None).await.unwrap();
    let failures_before = handler.failures();
    let mut pages = Box::pin(
        container
            .query_change_feed::<TestDoc>(
                FeedScope::range(ranges[0].clone()),
                ChangeFeedStartFrom::Beginning,
                None,
            )
            .await
            .unwrap(),
    );
    let error = pages.try_next().await.unwrap_err();
    assert_eq!(error.status(), status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED);
    let diagnostics = error
        .diagnostics()
        .expect("SDK error must retain recovery context");
    assert_eq!(diagnostics.request_count(), 11);
    assert_eq!(rule.hit_count(), 11);
    assert_eq!(handler.failures(), failures_before + 1);
    let observed = handler.last_diagnostics.lock().unwrap();
    let observed = observed.as_ref().unwrap();
    assert_eq!(
        observed["status"],
        status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED.to_string()
    );
    assert_eq!(observed["request_count"], 11);
    assert_eq!(observed["topology_recovery"]["total_attempts"], 11);
}

#[cfg(feature = "fault_injection")]
#[tokio::test(start_paused = true)]
async fn handler_observes_wrapped_faults_with_original_attempts() {
    use azure_data_cosmos_driver::{
        error::status_codes,
        fault_injection::{
            FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
            FaultInjectionRuleBuilder, FaultOperationType,
        },
    };
    use std::error::Error as _;

    for (operation, fault, original_status, public_status) in [
        (
            FaultOperationType::ReadItem,
            FaultInjectionErrorType::ReadSessionNotAvailable,
            status_codes::READ_SESSION_NOT_AVAILABLE,
            status_codes::CLIENT_READ_SESSION_NOT_AVAILABLE,
        ),
        (
            FaultOperationType::CreateItem,
            FaultInjectionErrorType::WriteForbidden,
            status_codes::WRITE_FORBIDDEN,
            status_codes::CLIENT_WRITE_FORBIDDEN,
        ),
        (
            FaultOperationType::ReadItem,
            FaultInjectionErrorType::DatabaseAccountNotFound,
            status_codes::DATABASE_ACCOUNT_NOT_FOUND,
            status_codes::CLIENT_DATABASE_ACCOUNT_NOT_FOUND,
        ),
        (
            FaultOperationType::CreateItem,
            FaultInjectionErrorType::DatabaseAccountNotFound,
            status_codes::DATABASE_ACCOUNT_NOT_FOUND,
            status_codes::CLIENT_DATABASE_ACCOUNT_NOT_FOUND,
        ),
    ] {
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
            )])
            .unwrap()
            .with_consistency(ConsistencyLevel::Session),
        ));
        emulator.store().create_database("wrapped-db");
        emulator.store().create_container(
            "wrapped-db",
            "items",
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))
                .unwrap(),
        );
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "sdk-terminal",
                FaultInjectionResultBuilder::new().with_error(fault).build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(operation)
                    .build(),
            )
            .build(),
        );
        let runtime = CosmosRuntimeBuilder::from(
            emulator.runtime_builder_with_fault_rules(vec![rule.clone()]),
        )
        .build()
        .await
        .unwrap();
        let handler = Arc::new(CountingHandler::default());
        let client = CosmosClientBuilder::new()
            .with_runtime(runtime)
            .with_diagnostics_handler(handler.clone())
            .build(
                AccountReference::with_authentication_key(
                    EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
                    azure_core::credentials::Secret::new("dGVzdGtleQ=="),
                ),
                RoutingStrategy::ProximityTo(Region::EAST_US),
            )
            .await
            .unwrap();
        let container = client
            .database_client("wrapped-db")
            .container_client("items", None)
            .await
            .unwrap();
        let failures_before = handler.failures();
        let error = if operation == FaultOperationType::ReadItem {
            container.read_item("pk1", "doc", None).await.unwrap_err()
        } else {
            container
                .create_item(
                    "pk1",
                    "doc",
                    &TestDoc {
                        id: "doc".into(),
                        pk: "pk1".into(),
                        value: 1,
                    },
                    None,
                )
                .await
                .unwrap_err()
        };
        assert_eq!(error.status(), public_status);
        assert_eq!(
            error
                .source()
                .unwrap()
                .downcast_ref::<azure_data_cosmos::CosmosError>()
                .unwrap()
                .status(),
            original_status
        );
        assert_eq!(handler.failures(), failures_before + 1);
        let diagnostics = handler.last_diagnostics.lock().unwrap();
        let json = diagnostics.as_ref().unwrap();
        assert_eq!(json["status"], public_status.to_string());
        let attempts: Vec<_> = json["requests"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|request| request["pipeline_type"] == "data_plane")
            .collect();
        assert_eq!(attempts.len(), rule.hit_count() as usize);
        assert!(!attempts.is_empty());
        for attempt in attempts {
            assert_eq!(attempt["status"], original_status.to_string());
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
        .container_client(&container, None)
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
        .container_client(&container, None)
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
        .container_client(&container, None)
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
    let items: Vec<TestDoc> = Box::pin(c.query_items(
        Query::from("SELECT * FROM c"),
        FeedScope::partition("pkQ"),
        None,
    ))
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
        .container_client(&container, None)
        .await
        .unwrap();

    let before_total = handler.total();
    let before_failures = handler.failures();

    // A syntactically invalid query is rejected by the emulator with a terminal
    // (non-retryable) 400 BadRequest, so the first page fetch errors instead of
    // returning a page — exercising the iterator's failure dispatch branch.
    let result: Result<Vec<TestDoc>, _> = Box::pin(c.query_items(
        Query::from("SELECT * FROM c WHERE"),
        FeedScope::partition("pkFail"),
        None,
    ))
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
